use core::ffi::c_uint;

use ahash::RandomState;
use hir::{CompilationDB, ParamSysFun, Parameter, Variable};
use hir_lower::{HirInterner, LimitState, ParamKind, PlaceKind};
use indexmap::IndexMap;
use inkwell::builder::Builder;
use inkwell::types::{ArrayType, BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use inkwell::IntPredicate;
use mir::{strip_optbarrier, Const, Function, Param, ValueDef, F_ZERO};
use mir_llvm::{CodegenCx, MemLoc};
use sim_back::dae::{self, MatrixEntryId, SimUnknown};
use sim_back::init::CacheSlot;
use stdx::packed_option::PackedOption;
use stdx::{impl_debug_display, impl_idx_from};
use typed_index_collections::TiVec;
use typed_indexmap::TiMap;

use crate::compilation_unit::{OsdiCompilationUnit, OsdiModule};
use crate::{bitfield, lltype, Offset};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum OsdiInstanceParam {
    Builtin(ParamSysFun),
    User(Parameter),
}

pub const NUM_CONST_FIELDS: u32 = 8;
pub const PARAM_GIVEN: u32 = 0;
pub const JACOBIAN_PTR_RESIST: u32 = 1;
pub const JACOBIAN_PTR_REACT: u32 = 2;
pub const NODE_MAPPING: u32 = 3;
pub const COLLAPSED: u32 = 4;
pub const TEMPERATURE: u32 = 5;
pub const CONNECTED: u32 = 6;
pub const STATE_IDX: u32 = 7;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum EvalOutput {
    Calculated(EvalOutputSlot),
    Const(Const, PackedOption<EvalOutputSlot>),
    Param(Param),
    Cache(CacheSlot),
}

impl EvalOutput {
    const NONE: EvalOutput = EvalOutput::Cache(CacheSlot(u32::MAX));

    fn new<'ll>(
        module: &OsdiModule<'_>,
        val: mir::Value,
        eval_outputs: &mut TiMap<EvalOutputSlot, mir::Value, BasicTypeEnum<'ll>>,
        requires_slot: bool,
        ty: BasicTypeEnum<'ll>,
    ) -> EvalOutput {
        match module.eval.dfg.value_def(val) {
            ValueDef::Result(_, _) => (),
            ValueDef::Param(param) => {
                // parameters are already stored in the model anyway so no need to create a slot
                if let Some((&kind, _)) = module.intern.params.get_index(param) {
                    if matches!(
                        kind,
                        ParamKind::Param { .. }
                            | ParamKind::ParamSysFun { .. }
                            | ParamKind::Temperature { .. }
                    ) {
                        return EvalOutput::Param(param);
                    }
                } else {
                    let slot = usize::from(param) - module.intern.params.len();
                    return EvalOutput::Cache(slot.into());
                }
            }
            ValueDef::Const(const_val) => {
                let slot = requires_slot.then(|| eval_outputs.insert_full(val, ty).0);
                return EvalOutput::Const(const_val, slot.into());
            }
            ValueDef::Invalid => unreachable!(),
        }

        EvalOutput::Calculated(eval_outputs.insert_full(val, ty).0)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct EvalOutputSlot(u32);
impl_idx_from!(EvalOutputSlot(u32));
impl_debug_display! {match EvalOutputSlot{EvalOutputSlot(id) => "out{id}";}}

#[derive(Clone, Copy, Debug)]
pub struct Residual {
    pub resist: PackedOption<EvalOutputSlot>,
    pub react: PackedOption<EvalOutputSlot>,
    pub resist_lim_rhs: PackedOption<EvalOutputSlot>,
    pub react_lim_rhs: PackedOption<EvalOutputSlot>,
}

impl Residual {
    pub fn new<'ll>(
        residual: &dae::Residual,
        slots: &mut TiMap<EvalOutputSlot, mir::Value, BasicTypeEnum<'ll>>,
        ty_real: BasicTypeEnum<'ll>,
        func: &Function,
    ) -> Residual {
        let mut get_slot = |mut val| {
            val = strip_optbarrier(func, val);
            if val == F_ZERO {
                None.into()
            } else {
                Some(slots.insert_full(val, ty_real).0).into()
            }
        };
        Residual {
            resist: get_slot(residual.resist),
            react: get_slot(residual.react),
            resist_lim_rhs: get_slot(residual.resist_lim_rhs),
            react_lim_rhs: get_slot(residual.react_lim_rhs),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct MatrixEntry {
    pub resist: Option<EvalOutput>,
    pub react: Option<EvalOutput>,
    pub react_off: PackedOption<Offset>,
}

impl MatrixEntry {
    pub fn new<'ll>(
        entry: &dae::MatrixEntry,
        module: &OsdiModule<'_>,
        slots: &mut TiMap<EvalOutputSlot, mir::Value, BasicTypeEnum<'ll>>,
        ty_real: BasicTypeEnum<'ll>,
        num_react: &mut u32,
    ) -> MatrixEntry {
        let mut get_output = |mut val| {
            val = strip_optbarrier(module.eval, val);
            if val == F_ZERO {
                None
            } else {
                Some(EvalOutput::new(module, val, slots, false, ty_real))
            }
        };
        let react_off = if entry.react == F_ZERO {
            None
        } else {
            *num_react += 1;
            Some(Offset(*num_react - 1))
        };
        MatrixEntry {
            resist: get_output(entry.resist),
            react: get_output(entry.react),
            react_off: react_off.into(),
        }
    }
}

#[derive(Debug)]
pub struct NoiseSource {
    pub factor: EvalOutput,
    /// content of values depend on kind of noise source
    pub args: [EvalOutput; 2],
}

impl NoiseSource {
    pub fn new<'ll>(
        source: &dae::NoiseSource,
        module: &OsdiModule<'_>,
        slots: &mut TiMap<EvalOutputSlot, mir::Value, BasicTypeEnum<'ll>>,
        ty_real: BasicTypeEnum<'ll>,
    ) -> NoiseSource {
        let mut get_output = |mut val| {
            val = strip_optbarrier(module.eval, val);
            EvalOutput::new(module, val, slots, false, ty_real)
        };
        let args = match source.kind {
            dae::NoiseSourceKind::WhiteNoise { pwr } => [get_output(pwr), EvalOutput::NONE],
            dae::NoiseSourceKind::FlickerNoise { pwr, exp } => [get_output(pwr), get_output(exp)],
            dae::NoiseSourceKind::NoiseTable { .. } => [EvalOutput::NONE; 2],
        };
        NoiseSource { args, factor: get_output(source.factor) }
    }

    pub fn eval_outputs(&self) -> [EvalOutput; 3] {
        [self.factor, self.args[0], self.args[1]]
    }
}

pub struct OsdiInstanceData<'ll> {
    /// llvm type for the instance data struct
    pub ty: StructType<'ll>,

    // llvm types for static (always present) instance data struct fields
    pub param_given: ArrayType<'ll>,
    pub jacobian_ptr: BasicTypeEnum<'ll>,
    pub jacobian_ptr_react: BasicTypeEnum<'ll>,
    pub node_mapping: BasicTypeEnum<'ll>,
    pub state_idx: BasicTypeEnum<'ll>,
    pub collapsed: BasicTypeEnum<'ll>,

    // llvm types for dynamic instance data struct fields
    pub params: IndexMap<OsdiInstanceParam, BasicTypeEnum<'ll>, RandomState>,
    pub eval_outputs: TiMap<EvalOutputSlot, mir::Value, BasicTypeEnum<'ll>>,
    pub cache_slots: TiVec<CacheSlot, BasicTypeEnum<'ll>>,

    pub residual: TiVec<SimUnknown, Residual>,
    pub noise: Vec<NoiseSource>,
    pub opvars: IndexMap<Variable, EvalOutput, RandomState>,
    pub jacobian: TiVec<MatrixEntryId, MatrixEntry>,
    pub bound_step: Option<EvalOutputSlot>,
}

impl<'ll> OsdiInstanceData<'ll> {
    pub fn new(db: &CompilationDB, module: &OsdiModule<'_>, cx: &CodegenCx<'_, 'll>) -> Self {
        let ty_f64 = cx.ty_double();
        let ty_u32 = cx.ty_int();

        let builtin_inst_params = ParamSysFun::iter().filter_map(|param| {
            let is_live = |intern: &HirInterner, func| {
                intern.is_param_live(func, &ParamKind::ParamSysFun(param))
            };
            let is_live = is_live(module.intern, module.eval)
                || is_live(&module.init.intern, &module.init.func);
            is_live.then_some((OsdiInstanceParam::Builtin(param), ty_f64))
        });
        let alias_inst_params = module
            .info
            .sys_fun_alias
            .keys()
            .map(|param| (OsdiInstanceParam::Builtin(*param), ty_f64));
        let user_inst_params = module.info.params.iter().filter_map(|(param, info)| {
            info.is_instance.then(|| (OsdiInstanceParam::User(*param), lltype(&param.ty(db), cx)))
        });
        let params: IndexMap<_, _, _> =
            builtin_inst_params.chain(alias_inst_params).chain(user_inst_params).collect();

        let mut eval_outputs = TiMap::default();
        let opvars = module
            .info
            .op_vars
            .keys()
            .map(|var| {
                let val = module.intern.outputs[&PlaceKind::Var(*var)].unwrap_unchecked();
                let ty = lltype(&var.ty(db), cx);
                let pos = EvalOutput::new(module, val, &mut eval_outputs, true, ty);
                (*var, pos)
            })
            .collect();
        let residual = module
            .dae_system
            .residual
            .iter()
            .map(|residual| Residual::new(residual, &mut eval_outputs, ty_f64, module.eval))
            .collect();
        let mut num_react = 0;
        let jacobian = module
            .dae_system
            .jacobian
            .iter()
            .map(|entry| MatrixEntry::new(entry, module, &mut eval_outputs, ty_f64, &mut num_react))
            .collect();
        let noise = module
            .dae_system
            .noise_sources
            .iter()
            .map(|source| NoiseSource::new(source, module, &mut eval_outputs, ty_f64))
            .collect();
        let bound_step = module.intern.outputs.get(&PlaceKind::BoundStep).and_then(|val| {
            let mut val = val.expand()?;
            val = strip_optbarrier(module.eval, val);
            let slot = eval_outputs.insert_full(val, ty_f64).0;
            Some(slot)
        });

        let param_given = bitfield::arr_ty(params.len() as u32, cx);
        let jacobian_ptr = cx.ty_array(cx.ty_ptr().into(), module.dae_system.jacobian.len() as u32);
        let jacobian_ptr_react = cx.ty_array(cx.ty_ptr().into(), num_react);
        let node_mapping = cx.ty_array(ty_u32.into(), module.dae_system.unknowns.len() as u32);
        let collapsed = cx.ty_array(cx.ty_c_bool().into(), module.node_collapse.num_pairs());
        let temperature = cx.ty_double();
        let connected_ports = cx.ty_int();

        let cache_slots: TiVec<_, _> =
            module.init.cache_slots.raw.values().map(|ty| lltype(ty, cx)).collect();

        let state_idx = cx.ty_array(cx.ty_int().into(), module.intern.lim_state.len() as u32);
        let static_fields: [BasicTypeEnum; NUM_CONST_FIELDS as usize] = [
            param_given.into(),
            jacobian_ptr.into(),
            jacobian_ptr_react.into(),
            node_mapping.into(),
            collapsed.into(),
            temperature.into(),
            connected_ports.into(),
            state_idx.into(),
        ];

        let fields: Vec<BasicTypeEnum> = static_fields
            .into_iter()
            .chain(params.values().copied())
            .chain(cache_slots.iter().copied())
            .chain(eval_outputs.raw.values().copied())
            .collect();

        let name = &module.sym;
        let name = format!("osdi_inst_data_{name}");
        let ty = cx.ty_struct(&name, &fields);

        OsdiInstanceData {
            ty,
            param_given,
            jacobian_ptr,
            jacobian_ptr_react,
            node_mapping,
            state_idx,
            collapsed,
            params,
            eval_outputs,
            cache_slots,
            residual,
            noise,
            opvars,
            jacobian,
            bound_step,
        }
    }

    pub unsafe fn store_bound_step(
        &self,
        ptr: PointerValue<'ll>,
        builder: &mir_llvm::Builder<'_, '_, 'll>,
    ) {
        if let Some(slot) = self.bound_step {
            self.store_eval_output_slot(slot, ptr, builder);
        }
    }

    pub fn bound_step_elem(&self) -> Option<u32> {
        let elem = self.eval_output_slot_elem(self.bound_step?);
        Some(elem)
    }

    pub unsafe fn param_ptr(
        &self,
        param: OsdiInstanceParam,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> Option<(PointerValue<'ll>, BasicTypeEnum<'ll>)> {
        let (pos, _, ty) = self.params.get_full(&param)?;
        let elem = NUM_CONST_FIELDS + pos as u32;
        let ptr = builder.build_struct_gep(self.ty, ptr, elem, "param_ptr").unwrap();
        Some((ptr, *ty))
    }

    pub unsafe fn nth_param_ptr(
        &self,
        pos: u32,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> (PointerValue<'ll>, BasicTypeEnum<'ll>) {
        let ty = self.params.get_index(pos as usize).unwrap().1;
        let elem = NUM_CONST_FIELDS + pos;
        let ptr = builder.build_struct_gep(self.ty, ptr, elem, "nth_param_ptr").unwrap();
        (ptr, *ty)
    }

    pub fn nth_param_loc(
        &self,
        cx: &CodegenCx<'_, 'll>,
        pos: u32,
        ptr: PointerValue<'ll>,
    ) -> MemLoc<'ll> {
        let ty = *self.params.get_index(pos as usize).unwrap().1;
        let elem = NUM_CONST_FIELDS + pos;
        MemLoc::struct_gep(ptr, self.ty.into(), ty, elem, cx)
    }

    pub fn param_loc(
        &self,
        cx: &CodegenCx<'_, 'll>,
        param: OsdiInstanceParam,
        ptr: PointerValue<'ll>,
    ) -> Option<MemLoc<'ll>> {
        let pos = self.params.get_index_of(&param)? as u32;
        let res = self.nth_param_loc(cx, pos, ptr);
        Some(res)
    }

    pub unsafe fn read_param(
        &self,
        param: OsdiInstanceParam,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> Option<BasicValueEnum<'ll>> {
        let (ptr, ty) = self.param_ptr(param, ptr, builder)?;
        let val = builder.build_load(ty, ptr, "read_param").unwrap();
        Some(val)
    }

    pub unsafe fn store_nth_param(
        &self,
        param_id: u32,
        ptr: PointerValue<'ll>,
        val: BasicValueEnum<'ll>,
        builder: &Builder<'ll>,
    ) {
        let (ptr, _) = self.nth_param_ptr(param_id, ptr, builder);
        builder.build_store(ptr, val).unwrap();
    }

    pub unsafe fn read_nth_param(
        &self,
        pos: u32,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> BasicValueEnum<'ll> {
        let (ptr, ty) = self.nth_param_ptr(pos, ptr, builder);
        builder.build_load(ty, ptr, "read_nth_param").unwrap()
    }

    // pub unsafe fn opvar_ptr(
    //     &self,
    //     var: VarId,
    //     ptr: &'ll llvm_sys::LLVMValue,
    //     llbuilder: &llvm_sys::LLVMBuilder,
    // ) -> Option<(&'ll llvm_sys::LLVMValue, &'ll llvm_sys::LLVMType)> {
    //     let (pos, _, ty) = self.opvars.get_full(&var)?;
    //     let elem = NUM_CONST_FIELDS + self.params.len() as u32 + pos as u32;
    //     let ptr = LLVMBuildStructGEP2(llbuilder, self.ty, ptr, elem, UNNAMED);
    //     Some((ptr, ty))
    // }

    pub fn residual_off(
        &self,
        node: SimUnknown,
        reactive: bool,
        target_data: &LLVMTargetDataRef,
    ) -> Option<u32> {
        let residual = &self.residual[node];
        let slot = if reactive { &residual.react } else { &residual.resist };
        let elem = NUM_CONST_FIELDS
            + self.params.len() as u32
            + self.cache_slots.len() as u32
            + u32::from(slot.expand()?);

        let off =
            unsafe { LLVMOffsetOfElement(*target_data, NonNull::from(self.ty).as_ptr(), elem) }
                as u32;
        Some(off)
    }

    pub fn lim_rhs_off(
        &self,
        node: SimUnknown,
        reactive: bool,
        target_data: &LLVMTargetDataRef,
    ) -> Option<u32> {
        let residual = &self.residual[node];
        let residual = if reactive { &residual.react_lim_rhs } else { &residual.resist_lim_rhs };
        let slot = residual.expand()?;
        let elem = self.eval_output_slot_elem(slot);
        let off =
            unsafe { LLVMOffsetOfElement(*target_data, NonNull::from(self.ty).as_ptr(), elem) }
                as u32;
        Some(off)
    }

    unsafe fn eval_output_slot_ptr(
        &self,
        builder: &Builder<'ll>,
        ptr: PointerValue<'ll>,
        slot: EvalOutputSlot,
    ) -> (PointerValue<'ll>, BasicTypeEnum<'ll>) {
        let elem = self.eval_output_slot_elem(slot);
        let ptr = builder.build_struct_gep(self.ty, ptr, elem, "eval_output_slot").unwrap();
        let ty = *self.eval_outputs.get_index(slot).unwrap().1;
        (ptr, ty)
    }

    fn eval_output_slot_elem(&self, slot: EvalOutputSlot) -> u32 {
        NUM_CONST_FIELDS
            + self.params.len() as u32
            + self.cache_slots.len() as u32
            + u32::from(slot)
    }

    unsafe fn load_eval_output_slot(
        &self,
        builder: &Builder<'ll>,
        ptr: PointerValue<'ll>,
        slot: EvalOutputSlot,
    ) -> BasicValueEnum<'ll> {
        let (ptr, ty) = self.eval_output_slot_ptr(builder, ptr, slot);
        builder.build_load(ty, ptr, "load_eval_output_slot").unwrap()
    }

    pub unsafe fn store_eval_output_slot(
        &self,
        slot: EvalOutputSlot,
        inst_ptr: PointerValue<'ll>,
        builder: &mir_llvm::Builder<'_, '_, 'll>,
    ) {
        let val = *self.eval_outputs.get_index(slot).unwrap().0;
        let val = builder.values[val].get(builder);
        let (ptr, _) = self.eval_output_slot_ptr(&builder.inkwell_builder, inst_ptr, slot);
        builder.inkwell_builder.build_store(ptr, val).unwrap();
    }

    pub unsafe fn store_eval_output(
        &self,
        output: EvalOutput,
        inst_ptr: PointerValue<'ll>,
        builder: &mir_llvm::Builder<'_, '_, 'll>,
    ) {
        if let EvalOutput::Calculated(slot) = output {
            self.store_eval_output_slot(slot, inst_ptr, builder)
        }
    }

    // pub unsafe fn param_given_pointer_and_mask(
    //     &self,
    //     cx: &CodegenCx<'_, 'll>,
    //     param: OsdiInstanceParam,
    //     ptr: &'ll llvm_sys::LLVMValue,
    //     llbuilder: &llvm_sys::LLVMBuilder,
    // ) -> Option<(&'ll llvm_sys::LLVMValue, &'ll llvm_sys::LLVMValue)> {
    //     let pos = self.params.get_index_of(&param)?;
    //     Some(self.nth_param_given_pointer_and_mask(cx, pos as u32, ptr, llbuilder))
    // }

    // pub unsafe fn nth_param_given_pointer_and_mask(
    //     &self,
    //     cx: &CodegenCx<'_, 'll>,
    //     pos: u32,
    //     ptr: &'ll llvm_sys::LLVMValue,
    //     llbuilder: &llvm_sys::LLVMBuilder,
    // ) -> (&'ll llvm_sys::LLVMValue, &'ll llvm_sys::LLVMValue) {
    //     let arr_ptr = LLVMBuildStructGEP2(llbuilder, self.ty, ptr, PARAM_GIVEN, UNNAMED);
    //     bitfield::word_ptr_and_mask(cx, pos, arr_ptr, self.param_given, llbuilder)
    // }

    pub unsafe fn is_nth_param_given(
        &self,
        cx: &CodegenCx<'_, 'll>,
        pos: u32,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> IntValue<'ll> {
        let arr_ptr = builder.build_struct_gep(self.ty, ptr, PARAM_GIVEN, "param_given").unwrap();
        bitfield::is_set(cx, pos, arr_ptr, self.param_given, builder)
    }

    pub unsafe fn is_param_given(
        &self,
        cx: &CodegenCx<'_, 'll>,
        param: OsdiInstanceParam,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> Option<IntValue<'ll>> {
        let pos = self.params.get_index_of(&param)?;
        let res = self.is_nth_param_given(cx, pos as u32, ptr, builder);
        Some(res)
    }

    pub unsafe fn set_nth_param_given(
        &self,
        cx: &CodegenCx<'_, 'll>,
        pos: u32,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) {
        let arr_ptr = builder.build_struct_gep(self.ty, ptr, PARAM_GIVEN, "param_given").unwrap();
        bitfield::set_bit(cx, pos, arr_ptr, self.param_given, builder)
    }

    // pub unsafe fn set_param_given(
    //     &self,
    //     cx: &CodegenCx<'_, 'll>,
    //     param: OsdiInstanceParam,
    //     ptr: &'ll llvm_sys::LLVMValue,
    //     llbuilder: &llvm_sys::LLVMBuilder,
    // ) -> bool {
    //     if let Some(pos) = self.params.get_index_of(&param) {
    //         self.set_nth_param_given(cx, pos as u32, ptr, llbuilder);
    //         true
    //     } else {
    //         false
    //     }
    // }
    pub unsafe fn read_node_off(
        &self,
        cx: &CodegenCx<'_, 'll>,
        node: SimUnknown,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> IntValue<'ll> {
        // First GEP for accessing the NODE_MAPPING field
        let ptr = builder.build_struct_gep(self.ty, ptr, NODE_MAPPING, "node_mapping").unwrap();

        // Apply GEP for node mapping array access
        let indices = [cx.const_int(0).into(), cx.const_unsigned_int(node.into()).into()];
        let ptr = builder.build_gep(self.node_mapping, ptr, &indices, "node_gep").unwrap();

        // Load the integer value from the final pointer
        builder.build_load(cx.ty_int(), ptr, "node_off").unwrap().into_int_value()
    }

    pub unsafe fn read_state_idx(
        &self,
        cx: &CodegenCx<'_, 'll>,
        idx: LimitState,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> IntValue<'ll> {
        // First GEP for accessing the STATE_IDX field
        let ptr = builder.build_struct_gep(self.ty, ptr, STATE_IDX, "state_idx").unwrap();

        // Apply GEP for state index array access
        let indices = [cx.const_int(0).into(), cx.const_unsigned_int(idx.into()).into()];
        let ptr = builder.build_gep(self.state_idx, ptr, &indices, "state_gep").unwrap();

        // Load the integer value from the final pointer
        builder.build_load(cx.ty_int(), ptr, "state_idx_val").unwrap().into_int_value()
    }

    pub unsafe fn read_node_voltage(
        &self,
        cx: &CodegenCx<'_, 'll>,
        node: SimUnknown,
        ptr: PointerValue<'ll>,
        prev_result: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> inkwell::values::FloatValue<'ll> {
        let off = self.read_node_off(cx, node, ptr, builder);
        let indices = [off.into()];
        let ptr = builder.build_gep(cx.ty_double(), prev_result, &indices, "node_voltage_gep").unwrap();
        builder.build_load(cx.ty_double(), ptr, "node_voltage").unwrap().into_float_value()
    }

    pub unsafe fn read_residual(
        &self,
        node: SimUnknown,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
        reactive: bool,
    ) -> Option<BasicValueEnum<'ll>> {
        let residual = &self.residual[node];
        let residual = if reactive { &residual.react } else { &residual.resist };
        let val = self.load_eval_output_slot(builder, ptr, residual.expand()?);
        Some(val)
    }

    pub unsafe fn store_lim_rhs(
        &self,
        node: SimUnknown,
        ptr: PointerValue<'ll>,
        builder: &mir_llvm::Builder<'_, '_, 'll>,
        reactive: bool,
    ) -> bool {
        let dst = &self.residual[node];
        let slot = if reactive { dst.react_lim_rhs } else { dst.resist_lim_rhs };
        if let Some(slot) = slot.expand() {
            self.store_eval_output_slot(slot, ptr, builder);
            true
        } else {
            false
        }
    }

    pub unsafe fn read_lim_rhs(
        &self,
        node: SimUnknown,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
        reactive: bool,
    ) -> Option<BasicValueEnum<'ll>> {
        let residual = &self.residual[node];
        let lim_rhs = if reactive { &residual.react_lim_rhs } else { &residual.resist_lim_rhs };
        let val = self.load_eval_output_slot(builder, ptr, lim_rhs.expand()?);
        Some(val)
    }

    pub unsafe fn store_residual(
        &self,
        node: SimUnknown,
        ptr: PointerValue<'ll>,
        builder: &mir_llvm::Builder<'_, '_, 'll>,
        reactive: bool,
    ) -> bool {
        let residual = &self.residual[node];
        let slot = if reactive { residual.react } else { residual.resist };
        if let Some(slot) = slot.expand() {
            self.store_eval_output_slot(slot, ptr, builder);
            true
        } else {
            false
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub unsafe fn store_contrib(
        &self,
        cx: &CodegenCx<'_, 'll>,
        node: SimUnknown,
        ptr: PointerValue<'ll>,
        dst: PointerValue<'ll>,
        contrib: inkwell::values::FloatValue<'ll>,
        builder: &Builder<'ll>,
        negate: bool,
    ) {
        let off = self.read_node_off(cx, node, ptr, builder);
        let indices = [off.into()];
        let dst = builder.build_gep(cx.ty_double(), dst, &indices, "contrib_gep").unwrap();
        let old = builder.build_load(cx.ty_double(), dst, "old_contrib").unwrap().into_float_value();

        let val = if negate {
            builder.build_float_sub(old, contrib, "negate_contrib").unwrap()
        } else {
            builder.build_float_add(old, contrib, "add_contrib").unwrap()
        };

        // TODO: Set fast math flags once inkwell exposes this API
        builder.build_store(dst, val).unwrap();
    }

    pub unsafe fn store_jacobian(
        &self,
        entry: MatrixEntryId,
        inst_ptr: PointerValue<'ll>,
        builder: &mir_llvm::Builder<'_, '_, 'll>,
        reactive: bool,
    ) {
        let entry = &self.jacobian[entry];
        let dst = if reactive { entry.react } else { entry.resist };
        if let Some(EvalOutput::Calculated(slot)) = dst {
            self.store_eval_output_slot(slot, inst_ptr, builder)
        }
    }

    // Adds Jacobian contribution to destination
    pub unsafe fn store_jacobian_contrib(
        &self,
        cx: &CodegenCx<'_, 'll>,
        entry: MatrixEntryId,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
        reactive: bool,
        has_offset: bool,
        offset: IntValue<'ll>,
        val: inkwell::values::FloatValue<'ll>,
    ) {
        // Field number within instance structure
        let field = if reactive { JACOBIAN_PTR_REACT } else { JACOBIAN_PTR_RESIST };

        // Get pointer to array of double* pointers
        let ptr = builder.build_struct_gep(self.ty, ptr, field, "jacobian_ptr_field").unwrap();
        let zero = cx.const_int(0);
        // Get entry index
        let entry = if reactive {
            // For reactive Jacobian get index of reactive entry
            self.jacobian[entry].react_off.unwrap_unchecked().into()
        } else {
            // For resistive Jacobian the u32 value within MatrixEntryId is the index
            entry.into()
        };
        // Convert to LLVM u32
        let entry = cx.const_unsigned_int(entry);
        // Prepare type of Jacobian entry pointers array
        let ty = if reactive { self.jacobian_ptr_react } else { self.jacobian_ptr };
        // Create pointer to array entry with index entry
        let indices = [zero.into(), entry.into()];
        let ptr = builder.build_gep(ty, ptr, &indices, "jacobian_entry").unwrap();

        // Load value from destination pointed to by ptr (get pointer to Jacobian entry destination)
        let mut dst = builder.build_load(cx.ty_ptr(), ptr, "jacobian_dst").unwrap().into_pointer_value();

        // Add offset to destination
        if has_offset {
            let indices = [offset.into()];
            dst = builder.build_gep(cx.ty_double(), dst, &indices, "offset_dst").unwrap();
        }
        // Load value from where the Jacobian entry should be added (pointed to by dst)
        let old = builder.build_load(cx.ty_double(), dst, "old_jacobian").unwrap().into_float_value();

        // Add value to old
        let val = builder.build_float_add(old, val, "jacobian_add").unwrap();
        // TODO: Set fast math flags once inkwell exposes this API
        // Store value where dst pointer points to
        builder.build_store(dst, val).unwrap();
    }

    // Writes Jacobian contribution to corresponding slot in array of doubles
    pub unsafe fn write_jacobian_contrib(
        &self,
        cx: &CodegenCx<'_, 'll>,
        entry: u32,
        ty: BasicTypeEnum<'ll>,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
        val: BasicValueEnum<'ll>,
    ) {
        let zero = cx.const_int(0);
        // Convert to LLVM u32
        let entry = cx.const_unsigned_int(entry);
        // Create pointer to array entry with index entry
        let indices = [zero.into(), entry.into()];
        let ptr = builder.build_gep(ty, ptr, &indices, "write_jacobian_entry").unwrap();

        // Store value where dst pointer points to
        builder.build_store(ptr, val).unwrap();
    }

    pub fn cache_slot_elem(&self, slot: CacheSlot) -> u32 {
        NUM_CONST_FIELDS + self.params.len() as u32 + u32::from(slot)
    }

    fn cache_slot_ptr(
        &self,
        builder: &Builder<'ll>,
        slot: CacheSlot,
        ptr: PointerValue<'ll>,
    ) -> (PointerValue<'ll>, BasicTypeEnum<'ll>) {
        let elem = self.cache_slot_elem(slot);
        let ptr = unsafe {
            builder.build_struct_gep(self.ty, ptr, elem, "cache_slot").unwrap()
        };
        let ty = self.cache_slots[slot];
        (ptr, ty)
    }

    pub unsafe fn load_cache_slot(
        &self,
        module: &OsdiModule,
        builder: &Builder<'ll>,
        slot: CacheSlot,
        ptr: PointerValue<'ll>,
    ) -> BasicValueEnum<'ll> {
        let (ptr, ty) = self.cache_slot_ptr(builder, slot, ptr);
        let mut val = builder.build_load(ty, ptr, "cache_slot_val").unwrap();

        if module.init.cache_slots[slot] == hir::Type::Bool {
            let int_val = val.into_int_value();
            let zero = ty.into_int_type().const_int(0, false);
            val = builder.build_int_compare(IntPredicate::NE, int_val, zero, "bool_cmp").unwrap().into();
        }

        val
    }

    pub unsafe fn store_cache_slot(
        &self,
        module: &OsdiModule,
        builder: &Builder<'ll>,
        slot: CacheSlot,
        ptr: PointerValue<'ll>,
        mut val: BasicValueEnum<'ll>,
    ) {
        let (ptr, ty) = self.cache_slot_ptr(builder, slot, ptr);
        if module.init.cache_slots[slot] == hir::Type::Bool {
            val = builder.build_int_cast(val.into_int_value(), ty.into_int_type(), "bool_cast").unwrap().into();
        }
        builder.build_store(ptr, val).unwrap();
    }
    pub unsafe fn store_is_collapsible(
        &self,
        cx: &CodegenCx<'_, 'll>,
        builder: &Builder<'ll>,
        ptr: PointerValue<'ll>,
        idx: IntValue<'ll>,
    ) {
        // Create GEP for the struct field
        let ptr = builder.build_struct_gep(self.ty, ptr, COLLAPSED, "collapsed").unwrap();

        // Apply GEP for collapsed field array access
        let indices = [cx.const_unsigned_int(0).into(), idx.into()];
        let ptr = builder.build_gep(self.collapsed, ptr, &indices, "collapsed_gep").unwrap();

        // Final store operation
        builder.build_store(ptr, cx.const_c_bool(true)).unwrap();
    }

    pub unsafe fn temperature_loc(
        &self,
        cx: &CodegenCx<'_, 'll>,
        ptr: PointerValue<'ll>,
    ) -> MemLoc<'ll> {
        MemLoc::struct_gep(ptr, self.ty.into(), cx.ty_double().into(), TEMPERATURE, cx)
    }

    pub unsafe fn store_temperature(
        &self,
        builder: &mut mir_llvm::Builder<'_, '_, 'll>,
        ptr: PointerValue<'ll>,
        val: BasicValueEnum<'ll>,
    ) {
        let ptr = builder.inkwell_builder.build_struct_gep(self.ty, ptr, TEMPERATURE, "temperature").unwrap();
        builder.inkwell_builder.build_store(ptr, val).unwrap();
    }

    pub unsafe fn load_connected_ports(
        &self,
        builder: &mut mir_llvm::Builder<'_, '_, 'll>,
        ptr: PointerValue<'ll>,
    ) -> IntValue<'ll> {
        let ptr = builder.inkwell_builder.build_struct_gep(self.ty, ptr, CONNECTED, "connected").unwrap();
        builder.inkwell_builder.build_load(builder.cx.ty_int(), ptr, "connected_ports").unwrap().into_int_value()
    }

    pub unsafe fn store_connected_ports(
        &self,
        builder: &mut mir_llvm::Builder<'_, '_, 'll>,
        ptr: PointerValue<'ll>,
        val: BasicValueEnum<'ll>,
    ) {
        let ptr = builder.inkwell_builder.build_struct_gep(self.ty, ptr, CONNECTED, "connected").unwrap();
        builder.inkwell_builder.build_store(ptr, val).unwrap();
    }
}

impl<'ll> OsdiCompilationUnit<'_, '_, 'll> {
    pub unsafe fn load_eval_output(
        &self,
        output: EvalOutput,
        inst_ptr: PointerValue<'ll>,
        model_ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> BasicValueEnum<'ll> {
        let OsdiCompilationUnit { inst_data, model_data, cx, module, .. } = self;
        let (ptr, ty) = match output {
            EvalOutput::Calculated(slot) => {
                inst_data.eval_output_slot_ptr(builder, inst_ptr, slot)
            }
            EvalOutput::Const(val, _) => {
                return cx.const_val(&val);
            }
            EvalOutput::Param(param) => {
                let intern = &module.intern;
                let (kind, _) = intern.params.get_index(param).unwrap();
                match *kind {
                    ParamKind::Param(param) => inst_data
                        .param_ptr(OsdiInstanceParam::User(param), inst_ptr, builder)
                        .unwrap_or_else(|| {
                            model_data.param_ptr(param, model_ptr, builder).unwrap()
                        }),
                    ParamKind::Temperature => {
                        let ptr = builder.build_struct_gep(inst_data.ty, inst_ptr, TEMPERATURE, "temperature").unwrap();
                        (ptr, cx.ty_double().into())
                    }
                    ParamKind::ParamSysFun(func) => inst_data
                        .param_ptr(OsdiInstanceParam::Builtin(func), inst_ptr, builder)
                        .unwrap(),

                    ParamKind::HiddenState(_) => todo!("hidden state"),

                    ParamKind::Voltage { .. }
                    | ParamKind::Current(_)
                    | ParamKind::PortConnected { .. }
                    | ParamKind::ParamGiven { .. }
                    | ParamKind::Abstime
                    | ParamKind::EnableIntegration
                    | ParamKind::EnableLim
                    | ParamKind::PrevState(_)
                    | ParamKind::NewState(_)
                    | ParamKind::ImplicitUnknown(_) => unreachable!(),
                }
            }
            EvalOutput::Cache(slot) => inst_data.cache_slot_ptr(builder, slot, inst_ptr),
        };

        builder.build_load(ty, ptr, "load_eval_output").unwrap()
    }

    pub unsafe fn load_jacobian_entry(
        &self,
        entry: MatrixEntryId,
        inst_ptr: PointerValue<'ll>,
        model_ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
        reactive: bool,
    ) -> Option<BasicValueEnum<'ll>> {
        let entry = &self.inst_data.jacobian[entry];
        let entry = if reactive { entry.react } else { entry.resist };
        let val = self.load_eval_output(entry?, inst_ptr, model_ptr, builder);
        Some(val)
    }

    pub unsafe fn nth_opvar_ptr(
        &self,
        pos: u32,
        inst_ptr: PointerValue<'ll>,
        model_ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> (PointerValue<'ll>, BasicTypeEnum<'ll>) {
        let OsdiCompilationUnit { inst_data, model_data, cx, module, .. } = self;
        match *inst_data.opvars.get_index(pos as usize).unwrap().1 {
            EvalOutput::Calculated(slot) => {
                inst_data.eval_output_slot_ptr(builder, inst_ptr, slot)
            }
            EvalOutput::Const(val, slot) => {
                let (ptr, ty) = inst_data.eval_output_slot_ptr(builder, inst_ptr, slot.unwrap());
                builder.build_store(ptr, cx.const_val(&val)).unwrap();
                (ptr, ty)
            }
            EvalOutput::Param(param) => {
                let intern = &module.intern;
                let (kind, _) = intern.params.get_index(param).unwrap();
                match *kind {
                    ParamKind::Param(param) => inst_data
                        .param_ptr(OsdiInstanceParam::User(param), inst_ptr, builder)
                        .unwrap_or_else(|| {
                            model_data.param_ptr(param, model_ptr, builder).unwrap()
                        }),
                    ParamKind::Temperature => {
                        let ptr = builder.build_struct_gep(inst_data.ty, inst_ptr, TEMPERATURE, "temperature").unwrap();
                        (ptr, cx.ty_double().into())
                    }
                    ParamKind::ParamSysFun(func) => inst_data
                        .param_ptr(OsdiInstanceParam::Builtin(func), inst_ptr, builder)
                        .unwrap(),

                    ParamKind::HiddenState(_) => todo!("hidden state"),

                    ParamKind::Voltage { .. }
                    | ParamKind::Current(_)
                    | ParamKind::PortConnected { .. }
                    | ParamKind::ParamGiven { .. }
                    | ParamKind::EnableIntegration { .. }
                    | ParamKind::Abstime
                    | ParamKind::EnableLim
                    | ParamKind::PrevState(_)
                    | ParamKind::NewState(_)
                    | ParamKind::ImplicitUnknown(_) => unreachable!(),
                }
            }
            EvalOutput::Cache(slot) => inst_data.cache_slot_ptr(builder, slot, inst_ptr),
        }
    }
}
