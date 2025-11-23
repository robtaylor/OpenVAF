use inkwell::basic_block::BasicBlock;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue, PointerValue};
use mir::{Block, ControlFlowGraph, FuncRef, Function, Param, PhiNode, Value};
use typed_index_collections::TiVec;

use crate::callbacks::CallbackFun;
use crate::CodegenCx;

/// Memory location abstraction for GEP (GetElementPtr) chains
#[derive(Clone)]
pub struct MemLoc<'ctx> {
    pub ptr: PointerValue<'ctx>,
    pub ptr_ty: BasicTypeEnum<'ctx>,
    pub ty: BasicTypeEnum<'ctx>,
    pub indices: Vec<IntValue<'ctx>>,
}

impl<'ctx> MemLoc<'ctx> {
    pub fn struct_gep(
        ptr: PointerValue<'ctx>,
        ptr_ty: BasicTypeEnum<'ctx>,
        ty: BasicTypeEnum<'ctx>,
        idx: u32,
        cx: &CodegenCx<'_, 'ctx>,
    ) -> MemLoc<'ctx> {
        MemLoc {
            ptr,
            ptr_ty,
            ty,
            indices: vec![cx.const_unsigned_int(0), cx.const_unsigned_int(idx)],
        }
    }

    pub fn read(&self, builder: &inkwell::builder::Builder<'ctx>) -> BasicValueEnum<'ctx> {
        // Build GEP instruction
        let gep_ptr = if !self.indices.is_empty() {
            unsafe {
                builder.build_gep(
                    self.ptr_ty,
                    self.ptr,
                    &self.indices.iter().map(|i| (*i).into()).collect::<Vec<_>>(),
                    "gep",
                ).unwrap()
            }
        } else {
            self.ptr
        };

        // Load from the GEP result
        builder.build_load(self.ty, gep_ptr, "load").unwrap()
    }

    pub fn to_ptr(&self, builder: &inkwell::builder::Builder<'ctx>) -> PointerValue<'ctx> {
        if !self.indices.is_empty() {
            unsafe {
                builder.build_gep(
                    self.ptr_ty,
                    self.ptr,
                    &self.indices.iter().map(|i| (*i).into()).collect::<Vec<_>>(),
                    "gep",
                ).unwrap()
            }
        } else {
            self.ptr
        }
    }
}

impl<'ctx> From<MemLoc<'ctx>> for BuilderVal<'ctx> {
    fn from(loc: MemLoc<'ctx>) -> Self {
        BuilderVal::Load(Box::new(loc))
    }
}

/// Lazy value representation in the builder
#[derive(Clone)]
pub enum BuilderVal<'ctx> {
    Undef,
    Eager(BasicValueEnum<'ctx>),
    Load(Box<MemLoc<'ctx>>),
}

impl<'ctx> From<BasicValueEnum<'ctx>> for BuilderVal<'ctx> {
    fn from(val: BasicValueEnum<'ctx>) -> Self {
        BuilderVal::Eager(val)
    }
}

impl<'ctx> BuilderVal<'ctx> {
    pub fn get(&self, builder: &Builder<'_, '_, 'ctx>) -> BasicValueEnum<'ctx> {
        match self {
            BuilderVal::Undef => panic!("attempted to read undefined value"),
            BuilderVal::Eager(val) => *val,
            BuilderVal::Load(loc) => loc.read(&builder.inkwell_builder),
        }
    }

    pub fn get_ty(&self, builder: &Builder<'_, '_, 'ctx>) -> Option<BasicTypeEnum<'ctx>> {
        match self {
            BuilderVal::Undef => None,
            BuilderVal::Eager(val) => Some(val.get_type()),
            BuilderVal::Load(loc) => Some(loc.ty),
        }
    }
}

pub enum FastMathMode {
    Full,
    Partial,
    Disabled,
}

/// Main IR builder for code generation
#[must_use]
pub struct Builder<'a, 'cx, 'ctx> {
    pub inkwell_builder: inkwell::builder::Builder<'ctx>,
    pub cx: &'a CodegenCx<'cx, 'ctx>,
    pub func: &'a Function,
    pub blocks: TiVec<Block, Option<BasicBlock<'ctx>>>,
    pub values: TiVec<Value, BuilderVal<'ctx>>,
    pub params: TiVec<Param, BuilderVal<'ctx>>,
    pub callbacks: TiVec<FuncRef, Option<CallbackFun<'ctx>>>,
    pub prepend_pos: BasicBlock<'ctx>,
    pub unfinished_phis: Vec<(PhiNode, inkwell::values::PhiValue<'ctx>)>,
    pub fun: inkwell::values::FunctionValue<'ctx>,
    pub return_void: bool,
    pub ret_allocated: Option<PointerValue<'ctx>>,
    pub ret_alloc_type: Option<BasicTypeEnum<'ctx>>,
    pub ret_store_ptr: std::cell::Cell<Option<PointerValue<'ctx>>>,
}

impl<'a, 'cx, 'ctx> Builder<'a, 'cx, 'ctx> {
    pub fn new(
        cx: &'a CodegenCx<'cx, 'ctx>,
        mir_func: &'a Function,
        llfunc: inkwell::values::FunctionValue<'ctx>,
        ret_alloc_type: Option<BasicTypeEnum<'ctx>>,
        return_void: bool,
    ) -> Self {
        let entry = cx.context.append_basic_block(llfunc, "entry");
        let inkwell_builder = cx.context.create_builder();

        let mut blocks: TiVec<_, _> = vec![None; mir_func.layout.num_blocks()].into();
        for bb in mir_func.layout.blocks() {
            blocks[bb] = Some(cx.context.append_basic_block(llfunc, &format!("bb{}", bb.index())));
        }

        inkwell_builder.position_at_end(entry);

        // Allocate return value if not void
        let ret_allocated = ret_alloc_type.map(|ty| {
            inkwell_builder.build_alloca(ty, "ret").unwrap()
        });

        Builder {
            inkwell_builder,
            cx,
            func: mir_func,
            blocks,
            values: vec![BuilderVal::Undef; mir_func.dfg.num_values()].into(),
            params: Default::default(),
            callbacks: Default::default(),
            fun: llfunc,
            prepend_pos: entry,
            unfinished_phis: Vec::new(),
            return_void,
            ret_allocated,
            ret_alloc_type,
            ret_store_ptr: std::cell::Cell::new(None),
        }
    }

    // Stub methods - to be implemented
    pub fn alloca(&self, ty: BasicTypeEnum<'ctx>) -> PointerValue<'ctx> {
        // Save current position
        let current_block = self.inkwell_builder.get_insert_block().unwrap();

        // Move to prepend position
        self.inkwell_builder.position_at_end(self.prepend_pos);
        let alloca = self.inkwell_builder.build_alloca(ty, "alloca").unwrap();

        // Restore position
        self.inkwell_builder.position_at_end(current_block);
        alloca
    }

    pub fn switch_to_block(&self, block: Block) {
        if let Some(bb) = self.blocks[block] {
            self.inkwell_builder.position_at_end(bb);
        }
    }

    // Additional methods will panic until implemented
    pub fn build_call(&mut self, _func_ty: BasicTypeEnum<'ctx>, _func: inkwell::values::FunctionValue<'ctx>, _args: &[BasicValueEnum<'ctx>]) -> BasicValueEnum<'ctx> {
        panic!("Builder::build_call not yet implemented in stub")
    }

    pub fn build_add(&mut self, _lhs: BasicValueEnum<'ctx>, _rhs: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        panic!("Builder::build_add not yet implemented in stub")
    }

    pub fn build_ret(&mut self, _val: Option<BasicValueEnum<'ctx>>) {
        panic!("Builder::build_ret not yet implemented in stub")
    }

    // More methods to be added as needed...
}
