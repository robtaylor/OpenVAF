use inkwell::types::BasicTypeEnum;
use inkwell::values::FunctionValue;

use crate::CodegenCx;

impl<'a, 'ctx> CodegenCx<'a, 'ctx> {
    pub fn intrinsic(&self, name: &'static str) -> Option<(BasicTypeEnum<'ctx>, FunctionValue<'ctx>)> {
        if let Some(res) = self.intrinsics.borrow().get(name) {
            return Some(*res);
        }

        macro_rules! ifn {
            ($name:expr, fn($($arg:expr),* ;...) -> $ret:expr) => (
                if name == $name {
                    return Some(self.insert_intrinsic($name, &[$($arg.into()),*], $ret.into(), true));
                }
            );
            ($name:expr, fn($($arg:expr),*) -> $ret:expr) => (
                if name == $name {
                    return Some(self.insert_intrinsic($name, &[$($arg.into()),*], $ret.into(), false));
                }
            );
        }

        let t_bool = self.ty_bool();
        let t_i32 = self.ty_int();
        let t_isize = self.ty_size();
        let t_f64 = self.ty_double();
        let t_str = self.ty_ptr();

        ifn!("llvm.pow.f64", fn(t_f64, t_f64) -> t_f64);
        ifn!("llvm.sqrt.f64", fn(t_f64) -> t_f64);
        ifn!("llvm.sin.f64", fn(t_f64) -> t_f64);
        ifn!("llvm.cos.f64", fn(t_f64) -> t_f64);
        ifn!("llvm.exp.f64", fn(t_f64) -> t_f64);
        ifn!("llvm.log.f64", fn(t_f64) -> t_f64);
        ifn!("llvm.log10.f64", fn(t_f64) -> t_f64);
        ifn!("llvm.log2.f64", fn(t_f64) -> t_f64);
        ifn!("llvm.floor.f64", fn(t_f64) -> t_f64);
        ifn!("llvm.ctlz", fn(t_i32, t_bool) -> t_i32);

        // not technically intrinsics but part of the C standard library
        ifn!("tan", fn(t_f64) -> t_f64);
        ifn!("acos", fn(t_f64) -> t_f64);
        ifn!("asin", fn(t_f64) -> t_f64);
        ifn!("atan", fn(t_f64) -> t_f64);
        ifn!("atan2", fn(t_f64, t_f64) -> t_f64);
        ifn!("sqrt", fn(t_f64) -> t_f64);
        ifn!("cosh", fn(t_f64) -> t_f64);
        ifn!("sinh", fn(t_f64) -> t_f64);
        ifn!("tanh", fn(t_f64) -> t_f64);
        ifn!("acosh", fn(t_f64) -> t_f64);
        ifn!("asinh", fn(t_f64) -> t_f64);
        ifn!("atanh", fn(t_f64) -> t_f64);

        if name == "hypot" {
            let name = if self.target.options.is_like_windows { "_hypot" } else { "hypot" };
            return Some(self.insert_intrinsic(name, &[t_f64.into()], t_f64.into(), false));
        }

        ifn!("strcmp", fn(t_str, t_str) -> t_i32);
        ifn!("llvm.lround.i32.f64", fn(t_f64) -> t_i32);

        if name == "snprintf" {
            return Some(self.insert_intrinsic("snprintf", &[t_str.into(), t_isize.into(), t_str.into()], t_i32.into(), true));
        }

        None
    }

    fn insert_intrinsic(
        &self,
        name: &'static str,
        args: &[BasicTypeEnum<'ctx>],
        ret: BasicTypeEnum<'ctx>,
        variadic: bool,
    ) -> (BasicTypeEnum<'ctx>, FunctionValue<'ctx>) {
        let fn_ty = if variadic {
            self.ty_variadic_func(args, ret)
        } else {
            self.ty_func(args, ret)
        };
        let f = self.get_func_by_name(name).unwrap_or_else(|| self.declare_ext_fn(name, fn_ty));
        self.intrinsics.borrow_mut().insert(name, (ret, f));
        (ret, f)
    }
}
