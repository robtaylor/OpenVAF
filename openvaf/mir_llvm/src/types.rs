use inkwell::context::Context;
use inkwell::types::{
    ArrayType, BasicType, BasicTypeEnum, FloatType, FunctionType, IntType,
    PointerType, StructType, VoidType,
};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue, PointerValue};
use inkwell::AddressSpace;

use mir::Const;

use crate::CodegenCx;

pub struct Types<'ctx> {
    pub double: FloatType<'ctx>,
    pub char: IntType<'ctx>,
    pub int: IntType<'ctx>,
    pub size: IntType<'ctx>,
    pub ptr: PointerType<'ctx>,
    pub fat_ptr: StructType<'ctx>,
    pub bool: IntType<'ctx>,
    pub void: VoidType<'ctx>,
    pub null_ptr_val: PointerValue<'ctx>,
}

impl<'ctx> Types<'ctx> {
    pub fn new(context: &'ctx Context, pointer_width: u32) -> Types<'ctx> {
        let char = context.i8_type();
        // Using opaque pointers (address space 0 is default/DATA address space)
        let ptr = context.ptr_type(AddressSpace::default());
        let size = context.custom_width_int_type(pointer_width);
        let double = context.f64_type();
        let i64_type = context.i64_type();

        let fat_ptr = context.struct_type(
            &[
                ptr.into(),
                i64_type.into(),
            ],
            false,
        );

        let bool_type = context.bool_type();
        let void = context.void_type();
        let null_ptr_val = ptr.const_null();

        Types {
            double,
            char,
            int: context.i32_type(),
            size,
            ptr,
            fat_ptr,
            bool: bool_type,
            void,
            null_ptr_val,
        }
    }
}

impl<'a, 'ctx> CodegenCx<'a, 'ctx> {
    #[inline(always)]
    pub fn ty_double(&self) -> FloatType<'ctx> {
        self.tys.double
    }

    #[inline(always)]
    pub fn ty_int(&self) -> IntType<'ctx> {
        self.tys.int
    }

    #[inline(always)]
    pub fn ty_char(&self) -> IntType<'ctx> {
        self.tys.char
    }

    #[inline(always)]
    pub fn ty_size(&self) -> IntType<'ctx> {
        self.tys.size
    }

    #[inline(always)]
    pub fn ty_bool(&self) -> IntType<'ctx> {
        self.tys.bool
    }

    #[inline(always)]
    pub fn ty_c_bool(&self) -> IntType<'ctx> {
        self.tys.char
    }

    #[inline(always)]
    pub fn ty_ptr(&self) -> PointerType<'ctx> {
        self.tys.ptr
    }

    #[inline(always)]
    pub fn ty_void(&self) -> VoidType<'ctx> {
        self.tys.void
    }

    #[inline(always)]
    pub fn ty_fat_ptr(&self) -> StructType<'ctx> {
        self.tys.fat_ptr
    }

    pub fn ty_aint(&self, bits: u32) -> IntType<'ctx> {
        self.context.custom_width_int_type(bits)
    }

    pub fn ty_struct(&self, name: &str, elements: &[BasicTypeEnum<'ctx>]) -> StructType<'ctx> {
        let struct_type = self.context.opaque_struct_type(name);
        struct_type.set_body(elements, false);
        struct_type
    }

    pub fn ty_func(&self, args: &[BasicTypeEnum<'ctx>], ret: BasicTypeEnum<'ctx>) -> FunctionType<'ctx> {
        let meta_args: Vec<_> = args.iter().map(|t| (*t).into()).collect();
        ret.fn_type(&meta_args, false)
    }

    pub fn ty_variadic_func(&self, args: &[BasicTypeEnum<'ctx>], ret: BasicTypeEnum<'ctx>) -> FunctionType<'ctx> {
        let meta_args: Vec<_> = args.iter().map(|t| (*t).into()).collect();
        ret.fn_type(&meta_args, true)
    }

    pub fn ty_array(&self, ty: BasicTypeEnum<'ctx>, len: u32) -> ArrayType<'ctx> {
        ty.array_type(len)
    }

    pub fn const_val(&self, val: &Const) -> BasicValueEnum<'ctx> {
        match *val {
            Const::Float(val) => self.const_real(val.into()).into(),
            Const::Int(val) => self.const_int(val).into(),
            Const::Bool(val) => self.const_bool(val).into(),
            Const::Str(val) => self.const_str(val).as_basic_value_enum(),
        }
    }

    /// # Safety
    /// indices must be valid and inbounds for the provided ptr
    /// The pointer must be a constant address
    pub unsafe fn const_gep(
        &self,
        elem_ty: BasicTypeEnum<'ctx>,
        ptr: PointerValue<'ctx>,
        indices: &[IntValue<'ctx>],
    ) -> PointerValue<'ctx> {
        ptr.const_gep(elem_ty, indices)
    }

    pub fn const_int(&self, val: i32) -> IntValue<'ctx> {
        self.ty_int().const_int(val as u64, true)
    }

    pub fn const_unsigned_int(&self, val: u32) -> IntValue<'ctx> {
        self.ty_int().const_int(val as u64, false)
    }

    pub fn const_isize(&self, val: isize) -> IntValue<'ctx> {
        self.ty_size().const_int(val as u64, true)
    }

    pub fn const_usize(&self, val: usize) -> IntValue<'ctx> {
        self.ty_size().const_int(val as u64, false)
    }

    pub fn const_bool(&self, val: bool) -> IntValue<'ctx> {
        self.ty_bool().const_int(val as u64, false)
    }

    pub fn const_c_bool(&self, val: bool) -> IntValue<'ctx> {
        self.ty_c_bool().const_int(val as u64, false)
    }

    pub fn const_u8(&self, val: u8) -> IntValue<'ctx> {
        self.ty_char().const_int(val as u64, false)
    }

    pub fn const_real(&self, val: f64) -> inkwell::values::FloatValue<'ctx> {
        self.ty_double().const_float(val)
    }

    pub fn const_arr(&self, elem_ty: BasicTypeEnum<'ctx>, vals: &[BasicValueEnum<'ctx>]) -> inkwell::values::ArrayValue<'ctx> {
        match elem_ty {
            BasicTypeEnum::ArrayType(t) => t.const_array(
                &vals.iter().map(|v| v.into_array_value()).collect::<Vec<_>>()
            ),
            BasicTypeEnum::FloatType(t) => t.const_array(
                &vals.iter().map(|v| v.into_float_value()).collect::<Vec<_>>()
            ),
            BasicTypeEnum::IntType(t) => t.const_array(
                &vals.iter().map(|v| v.into_int_value()).collect::<Vec<_>>()
            ),
            BasicTypeEnum::PointerType(t) => t.const_array(
                &vals.iter().map(|v| v.into_pointer_value()).collect::<Vec<_>>()
            ),
            BasicTypeEnum::StructType(t) => t.const_array(
                &vals.iter().map(|v| v.into_struct_value()).collect::<Vec<_>>()
            ),
            BasicTypeEnum::VectorType(t) => t.const_array(
                &vals.iter().map(|v| v.into_vector_value()).collect::<Vec<_>>()
            ),
        }
    }

    pub fn const_struct(&self, ty: StructType<'ctx>, vals: &[BasicValueEnum<'ctx>]) -> inkwell::values::StructValue<'ctx> {
        ty.const_named_struct(vals)
    }

    pub fn const_null_ptr(&self) -> PointerValue<'ctx> {
        self.tys.null_ptr_val
    }

    pub fn const_undef(&self, t: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx> {
        // get_undef doesn't exist in inkwell, use const_zero or type-specific methods
        match t {
            BasicTypeEnum::ArrayType(t) => t.const_zero().into(),
            BasicTypeEnum::FloatType(t) => t.const_zero().into(),
            BasicTypeEnum::IntType(t) => t.const_zero().into(),
            BasicTypeEnum::PointerType(t) => t.const_null().into(),
            BasicTypeEnum::StructType(t) => t.const_zero().into(),
            BasicTypeEnum::VectorType(t) => t.const_zero().into(),
        }
    }

    pub fn val_ty(&self, v: BasicValueEnum<'ctx>) -> BasicTypeEnum<'ctx> {
        v.get_type()
    }
}
