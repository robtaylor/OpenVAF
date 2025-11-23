use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::builder::Builder;
use crate::CodegenCx;

pub trait InlineCallbackBuilder<'ctx> {
    fn build_inline(
        &self,
        builder: &Builder<'_, '_, 'ctx>,
        state: &[BasicValueEnum<'ctx>],
    ) -> BasicValueEnum<'ctx>;

    fn return_type(
        &self,
        builder: &Builder<'_, '_, 'ctx>,
        state: &[BasicValueEnum<'ctx>],
    ) -> BasicTypeEnum<'ctx>;
}

#[derive(Clone)]
pub struct BuiltCallbackFun<'ctx> {
    pub fun_ty: BasicTypeEnum<'ctx>,
    pub fun: FunctionValue<'ctx>,
    /// Some Callbacks need to read/modify some state (typically passed as pointers)
    /// outside of the arguments provided in Verilog-A.
    /// These arguments are always passed before any arguments specified in the CFG
    pub state: Vec<BasicValueEnum<'ctx>>,
    pub num_state: u32,
}

#[derive(Clone)]
pub enum CallbackFun<'ctx> {
    Inline {
        builder: Box<dyn InlineCallbackBuilder<'ctx>>,
        state: Vec<BasicValueEnum<'ctx>>,
    },
    Prebuilt(BuiltCallbackFun<'ctx>),
}

impl<'a, 'ctx> CodegenCx<'a, 'ctx> {
    pub fn const_callback(
        &self,
        args: &[BasicTypeEnum<'ctx>],
        val: BasicValueEnum<'ctx>,
    ) -> BuiltCallbackFun<'ctx> {
        let name = self.local_callback_name();
        let ret_ty = val.get_type();
        let fun_ty = self.ty_func(args, ret_ty);
        let fun = self.declare_int_fn(&name, fun_ty);

        let basic_block = self.context.append_basic_block(fun, "entry");
        let builder = self.context.create_builder();
        builder.position_at_end(basic_block);
        builder.build_return(Some(&val)).unwrap();

        BuiltCallbackFun {
            fun_ty: ret_ty,
            fun,
            state: Vec::new(),
            num_state: 0,
        }
    }

    pub fn trivial_callbacks(&self, args: &[BasicTypeEnum<'ctx>]) -> BuiltCallbackFun<'ctx> {
        let name = self.local_callback_name();
        let fun_ty = self.ty_func(args, self.ty_void().into());
        let fun = self.declare_int_fn(&name, fun_ty);

        let basic_block = self.context.append_basic_block(fun, "entry");
        let builder = self.context.create_builder();
        builder.position_at_end(basic_block);
        builder.build_return(None).unwrap();

        BuiltCallbackFun {
            fun_ty: self.ty_void().into(),
            fun,
            state: Vec::new(),
            num_state: 0,
        }
    }

    pub fn const_return(
        &self,
        args: &[BasicTypeEnum<'ctx>],
        idx: usize,
    ) -> BuiltCallbackFun<'ctx> {
        let name = self.local_callback_name();
        let fun_ty = self.ty_func(args, args[idx]);
        let fun = self.declare_int_fn(&name, fun_ty);

        let basic_block = self.context.append_basic_block(fun, "entry");
        let builder = self.context.create_builder();
        builder.position_at_end(basic_block);
        let param = fun.get_nth_param(idx as u32).unwrap();
        builder.build_return(Some(&param)).unwrap();

        BuiltCallbackFun {
            fun_ty: args[idx],
            fun,
            state: Vec::new(),
            num_state: 0,
        }
    }

    pub fn local_callback_name(&self) -> String {
        self.generate_local_symbol_name("cb")
    }
}
