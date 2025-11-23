use ahash::RandomState;
use hir::{CompilationDB, Parameter};
use indexmap::IndexMap;
use inkwell::builder::Builder;
use inkwell::types::{ArrayType, BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue};
use mir_llvm::{CodegenCx, MemLoc};

use crate::compilation_unit::OsdiModule;
use crate::inst_data::{OsdiInstanceData, OsdiInstanceParam};
use crate::{bitfield, lltype};

const NUM_CONST_FIELDS: u32 = 1;

pub struct OsdiModelData<'ll> {
    pub param_given: ArrayType<'ll>,
    pub params: IndexMap<Parameter, BasicTypeEnum<'ll>, RandomState>,
    pub ty: StructType<'ll>,
}

impl<'ll> OsdiModelData<'ll> {
    pub fn new(
        db: &CompilationDB,
        cgunit: &OsdiModule<'_>,
        cx: &CodegenCx<'_, 'll>,
        inst_data: &OsdiInstanceData<'ll>,
    ) -> Self {
        let inst_params = &inst_data.params;
        let params: IndexMap<_, _, _> = cgunit
            .info
            .params
            .keys()
            .filter_map(|param| {
                if inst_params.contains_key(&OsdiInstanceParam::User(*param)) {
                    None
                } else {
                    Some((*param, lltype(&param.ty(db), cx)))
                }
            })
            .collect();

        let param_given = bitfield::arr_ty((inst_params.len() + params.len()) as u32, cx);

        let mut fields: Vec<BasicTypeEnum> = vec![param_given.into()];
        fields.extend(params.values().copied());
        fields.extend(inst_params.values().map(|t| *t));

        let name = &cgunit.sym;
        let name = format!("osdi_model_data_{name}");
        let ty = cx.ty_struct(&name, &fields);

        OsdiModelData { param_given, params, ty }
    }

    pub fn nth_param_loc(
        &self,
        cx: &CodegenCx<'_, 'll>,
        pos: u32,
        ptr: PointerValue<'ll>,
    ) -> MemLoc<'ll> {
        let ty = *self.params.get_index(pos as usize).unwrap().1;
        let elem = NUM_CONST_FIELDS + pos;
        let indices = vec![cx.const_unsigned_int(0), cx.const_unsigned_int(elem)];
        MemLoc { ptr, ptr_ty: self.ty.into(), ty, indices }
    }

    pub fn param_loc(
        &self,
        cx: &CodegenCx<'_, 'll>,
        param: Parameter,
        ptr: PointerValue<'ll>,
    ) -> Option<MemLoc<'ll>> {
        let pos = self.params.get_index_of(&param)? as u32;
        let res = self.nth_param_loc(cx, pos, ptr);
        Some(res)
    }

    pub unsafe fn param_ptr(
        &self,
        param: Parameter,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> Option<(PointerValue<'ll>, BasicTypeEnum<'ll>)> {
        let (pos, _, ty) = self.params.get_full(&param)?;
        let elem = NUM_CONST_FIELDS + pos as u32;
        let ptr = builder.build_struct_gep(self.ty, ptr, elem, "param_ptr").unwrap();
        Some((ptr, *ty))
    }

    // build code for getting the pointer to the storage of pos-th parameter within ptr
    pub unsafe fn nth_param_ptr(
        &self,
        pos: u32,
        // unwrap() returns a tuple holding parameter and type, .1 selects type (ref to ref)
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> (PointerValue<'ll>, BasicTypeEnum<'ll>) {
        let ty = self.params.get_index(pos as usize).unwrap().1;
        // index of element, skip NUM_CONST_FIELDS
        let elem = NUM_CONST_FIELDS + pos;
        // retrieve pointer to parameter storage within model data structure pointed to by ptr
        let ptr = builder.build_struct_gep(self.ty, ptr, elem, "nth_param_ptr").unwrap();
        (ptr, *ty)
    }

    pub unsafe fn nth_inst_param_ptr(
        &self,
        inst_data: &OsdiInstanceData<'ll>,
        pos: u32,
        // get the type, but this time from inst_data because pos is the instance parameter index
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> (PointerValue<'ll>, BasicTypeEnum<'ll>) {
        let ty = inst_data.params.get_index(pos as usize).unwrap().1;
        // index of element, skip NUM_CONST_FIELDS, then skip model parameter fields
        let elem = NUM_CONST_FIELDS + self.params.len() as u32 + pos;
        // retrieve pointer to parameter storage within model data structure pointed to by ptr
        let ptr = builder.build_struct_gep(self.ty, ptr, elem, "nth_inst_param_ptr").unwrap();
        (ptr, *ty)
    }

    // pub unsafe fn read_param(
    //     &self,
    //     param: ParamId,
    //     ptr: &'ll llvm_sys::LLVMValue,
    //     llbuilder: &llvm_sys::LLVMBuilder,
    // ) -> Option<&'ll llvm_sys::LLVMValue> {
    //     let (ptr, ty) = self.param_ptr(param, ptr, llbuilder)?;
    //     let val = LLVMBuildLoad2(llbuilder, ty, ptr, UNNAMED);
    //     Some(val)
    // }

    pub unsafe fn store_nth_param(
        &self,
        param: u32,
        ptr: PointerValue<'ll>,
        val: BasicValueEnum<'ll>,
        builder: &Builder<'ll>,
    ) {
        let (ptr, _) = self.nth_param_ptr(param, ptr, builder);
        builder.build_store(ptr, val).unwrap();
    }

    // pub unsafe fn read_nth_param(
    //     &self,
    //     param: u32,
    //     ptr: PointerValue<'ll>,
    //     builder: &Builder<'ll>,
    // ) -> BasicValueEnum<'ll> {
    //     let (ptr, ty) = self.nth_param_ptr(param, ptr, builder);
    //     builder.build_load(ty, ptr, "read_nth_param").unwrap()
    // }

    pub unsafe fn read_nth_inst_param(
        &self,
        inst_data: &OsdiInstanceData<'ll>,
        param: u32,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> BasicValueEnum<'ll> {
        let (ptr, ty) = self.nth_inst_param_ptr(inst_data, param, ptr, builder);
        builder.build_load(ty, ptr, "read_nth_inst_param").unwrap()
    }

    pub unsafe fn is_nth_param_given(
        &self,
        cx: &CodegenCx<'_, 'll>,
        pos: u32,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> inkwell::values::IntValue<'ll> {
        let arr_ptr = builder.build_struct_gep(self.ty, ptr, 0, "param_given_arr").unwrap();
        bitfield::is_set(cx, pos, arr_ptr, self.param_given, builder)
    }

    pub unsafe fn is_nth_inst_param_given(
        &self,
        cx: &CodegenCx<'_, 'll>,
        pos: u32,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> inkwell::values::IntValue<'ll> {
        let arr_ptr = builder.build_struct_gep(self.ty, ptr, 0, "param_given_arr").unwrap();
        bitfield::is_set(cx, pos + self.params.len() as u32, arr_ptr, self.param_given, builder)
    }

    pub unsafe fn is_inst_param_given(
        &self,
        inst_data: &OsdiInstanceData<'ll>,
        cx: &CodegenCx<'_, 'll>,
        param: OsdiInstanceParam,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> inkwell::values::IntValue<'ll> {
        let pos = inst_data.params.get_index_of(&param).unwrap();
        self.is_nth_inst_param_given(cx, pos as u32, ptr, builder)
    }
    pub unsafe fn is_param_given(
        &self,
        cx: &CodegenCx<'_, 'll>,
        param: Parameter,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) -> Option<inkwell::values::IntValue<'ll>> {
        let pos = self.params.get_index_of(&param)?;
        let res = self.is_nth_param_given(cx, pos as u32, ptr, builder);
        Some(res)
    }

    pub unsafe fn set_nth_inst_param_given(
        &self,
        cx: &CodegenCx<'_, 'll>,
        pos: u32,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) {
        let arr_ptr = builder.build_struct_gep(self.ty, ptr, 0, "param_given_arr").unwrap();
        bitfield::set_bit(cx, pos + self.params.len() as u32, arr_ptr, self.param_given, builder)
    }
    pub unsafe fn set_nth_param_given(
        &self,
        cx: &CodegenCx<'_, 'll>,
        pos: u32,
        ptr: PointerValue<'ll>,
        builder: &Builder<'ll>,
    ) {
        let arr_ptr = builder.build_struct_gep(self.ty, ptr, 0, "param_given_arr").unwrap();
        bitfield::set_bit(cx, pos, arr_ptr, self.param_given, builder)
    }

    // pub unsafe fn set_param_given(
    //     &self,
    //     cx: &CodegenCx<'_, 'll>,
    //     param: ParamId,
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
}
