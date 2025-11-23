use inkwell::module::Linkage;
use inkwell::types::{BasicTypeEnum, FunctionType};
use inkwell::values::{FunctionValue, GlobalValue, PointerValue};
use inkwell::AddressSpace;

use crate::CodegenCx;

impl<'a, 'ctx> CodegenCx<'a, 'ctx> {
    /// Declare a C ABI function.
    ///
    /// Only use this for foreign function ABIs and glue. For Rust functions use
    /// `declare_fn` instead.
    ///
    /// If there's a value with the same name already declared, the function will
    /// update the declaration and return existing Value instead.
    pub fn declare_ext_fn(&self, name: &str, fn_type: FunctionType<'ctx>) -> FunctionValue<'ctx> {
        if let Some(existing) = self.module.get_function(name) {
            return existing;
        }

        let func = self.module.add_function(name, fn_type, None);
        func.set_call_conventions(inkwell::llvm_sys::LLVMCallConv::LLVMCCallConv as u32);
        func
    }

    /// Declare a internal function.
    pub fn declare_int_fn(&self, name: &str, fn_type: FunctionType<'ctx>) -> FunctionValue<'ctx> {
        if let Some(existing) = self.module.get_function(name) {
            return existing;
        }

        let func = self.module.add_function(name, fn_type, Some(Linkage::Internal));
        func.set_call_conventions(inkwell::llvm_sys::LLVMCallConv::LLVMFastCallConv as u32);
        func.set_unnamed_addr(true);
        func
    }

    /// Declare a internal function with C calling convention.
    pub fn declare_int_c_fn(&self, name: &str, fn_type: FunctionType<'ctx>) -> FunctionValue<'ctx> {
        if let Some(existing) = self.module.get_function(name) {
            return existing;
        }

        let func = self.module.add_function(name, fn_type, Some(Linkage::Internal));
        func.set_call_conventions(inkwell::llvm_sys::LLVMCallConv::LLVMCCallConv as u32);
        func.set_unnamed_addr(true);
        func
    }

    /// Declare a global with an intention to define it.
    ///
    /// Use this function when you intend to define a global. This function will
    /// return `None` if the name already has a definition associated with it.
    pub fn define_global(&self, name: &str, ty: BasicTypeEnum<'ctx>) -> Option<GlobalValue<'ctx>> {
        if self.get_defined_value(name).is_some() {
            None
        } else {
            Some(self.module.add_global(ty, Some(AddressSpace::default()), name))
        }
    }

    /// Declare a private global
    ///
    /// Use this function when you intend to define a global without a name.
    pub fn define_private_global(&self, ty: BasicTypeEnum<'ctx>) -> GlobalValue<'ctx> {
        let global = self.module.add_global(ty, Some(AddressSpace::default()), "");
        global.set_linkage(Linkage::Private);
        global
    }

    /// Gets declared value by name.
    pub fn get_declared_value(&self, name: &str) -> Option<PointerValue<'ctx>> {
        self.module.get_global(name).map(|g| g.as_pointer_value())
    }

    /// Gets defined value by name.
    pub fn get_defined_value(&self, name: &str) -> Option<PointerValue<'ctx>> {
        if let Some(global) = self.module.get_global(name) {
            if global.get_initializer().is_some() || self.module.get_function(name).is_some() {
                return Some(global.as_pointer_value());
            }
        }
        self.module.get_function(name).map(|f| f.as_global_value().as_pointer_value())
    }
}
