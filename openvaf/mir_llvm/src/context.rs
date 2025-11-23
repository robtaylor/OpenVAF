use std::cell::{Cell, RefCell};

use ahash::AHashMap;
use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValue, FunctionValue, PointerValue};
use lasso::{Rodeo, Spur};
use target::spec::Target;

use crate::types::Types;

pub struct CodegenCx<'a, 'ctx> {
    pub module: &'ctx Module<'ctx>,
    pub context: &'ctx Context,
    pub target: &'a Target,
    pub literals: &'a Rodeo,
    str_lit_cache: RefCell<AHashMap<Spur, PointerValue<'ctx>>>,
    pub(crate) intrinsics: RefCell<AHashMap<&'static str, (BasicTypeEnum<'ctx>, FunctionValue<'ctx>)>>,
    pub(crate) local_gen_sym_counter: Cell<u32>,
    pub(crate) tys: Types<'ctx>,
}

impl<'a, 'ctx> CodegenCx<'a, 'ctx> {
    pub(crate) fn new(
        literals: &'a Rodeo,
        llvm_module: &'ctx crate::ModuleLlvm<'ctx>,
        target: &'a Target,
    ) -> CodegenCx<'a, 'ctx> {
        CodegenCx {
            module: llvm_module.module(),
            context: llvm_module.context(),
            str_lit_cache: RefCell::new(AHashMap::with_capacity(literals.len())),
            literals,
            intrinsics: RefCell::new(AHashMap::new()),
            local_gen_sym_counter: Cell::new(0),
            target,
            tys: Types::new(llvm_module.context(), target.pointer_width),
        }
    }

    pub fn get_func_by_name(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    pub fn include_bitcode(&self, bitcode: &[u8]) {
        let sym = self.generate_local_symbol_name("bitcode_buffer");
        let buffer = MemoryBuffer::create_from_memory_range(bitcode, &sym);
        let parsed_module = Module::parse_bitcode_from_buffer(&buffer, self.context)
            .expect("failed to parse bitcode");

        self.module.link_in_module(parsed_module)
            .expect("failed to link parsed bitcode");
    }

    pub fn to_str(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn const_str_uninterned(&self, lit: &str) -> PointerValue<'ctx> {
        let lit = self.literals.get(lit).unwrap();
        self.const_str(lit)
    }

    pub fn const_str(&self, lit: Spur) -> PointerValue<'ctx> {
        if let Some(val) = self.str_lit_cache.borrow().get(&lit) {
            return *val;
        }

        let val = self.literals.resolve(&lit);
        let const_string = self.context.const_string(val.as_bytes(), false);

        let sym = self.generate_local_symbol_name("str");
        let global = self
            .define_global(&sym, const_string.get_type().into())
            .unwrap_or_else(|| unreachable!("symbol {} already defined", sym));

        global.set_initializer(&const_string);
        global.set_constant(true);
        global.set_linkage(inkwell::module::Linkage::Internal);

        let ptr = global.as_pointer_value();
        self.str_lit_cache.borrow_mut().insert(lit, ptr);
        ptr
    }
}

impl CodegenCx<'_, '_> {
    /// Generates a new symbol name with the given prefix. This symbol name must
    /// only be used for definitions with `internal` or `private` linkage.
    pub fn generate_local_symbol_name(&self, prefix: &str) -> String {
        let idx = self.local_gen_sym_counter.get();
        self.local_gen_sym_counter.set(idx + 1);
        // Include a '.' character, so there can be no accidental conflicts with
        // user defined names
        let mut name = String::with_capacity(prefix.len() + 6);
        name.push_str(prefix);
        name.push('.');
        base_n::push_str(idx as u128, base_n::ALPHANUMERIC_ONLY, &mut name);
        name
    }

    /// Export a global value with the given name, type, and value
    pub fn export_val(
        &self,
        name: &str,
        ty: BasicTypeEnum,
        value: impl BasicValue,
        constant: bool,
    ) {
        let global = self.define_global(name, ty)
            .unwrap_or_else(|| panic!("symbol {} already defined", name));

        global.set_initializer(&value.as_basic_value_enum());
        global.set_constant(constant);
        global.set_linkage(inkwell::module::Linkage::External);
        global.set_dll_storage_class(inkwell::DLLStorageClass::Export);
    }

    /// Export an array of values as a global constant
    pub fn export_array(
        &self,
        name: &str,
        elem_ty: BasicTypeEnum,
        values: &[impl BasicValue],
        constant: bool,
        _external: bool, // Currently unused, kept for API compatibility
    ) {
        let vals: Vec<_> = values.iter().map(|v| v.as_basic_value_enum()).collect();
        let arr = self.const_arr(elem_ty, &vals);
        let arr_ty = arr.get_type();

        let global = self.define_global(name, arr_ty.into())
            .unwrap_or_else(|| panic!("symbol {} already defined", name));

        global.set_initializer(&arr);
        global.set_constant(constant);
        global.set_linkage(inkwell::module::Linkage::External);
        global.set_dll_storage_class(inkwell::DLLStorageClass::Export);
    }
}
