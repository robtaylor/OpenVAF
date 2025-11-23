use std::path::Path;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple};
use inkwell::OptimizationLevel;
use lasso::Rodeo;
use target::spec::Target as TargetSpec;

mod builder;
mod context;
mod declarations;
mod intrinsics;
mod types;
mod callbacks;

#[cfg(test)]
mod tests;

pub use builder::{Builder, BuilderVal, MemLoc};
pub use callbacks::{BuiltCallbackFun, CallbackFun, InlineCallbackBuilder};
pub use context::CodegenCx;

// Re-export OptimizationLevel for compatibility with existing code
pub use inkwell::OptimizationLevel as LLVMCodeGenOptLevel;

pub struct LLVMBackend<'t> {
    target: &'t TargetSpec,
    target_cpu: String,
    features: String,
}

impl<'t> LLVMBackend<'t> {
    pub fn new(
        _cg_opts: &[String],
        target: &'t TargetSpec,
        mut target_cpu: String,
        target_features: &[String],
    ) -> LLVMBackend<'t> {
        // Initialize LLVM targets
        Target::initialize_native(&InitializationConfig::default())
            .expect("Failed to initialize native target");

        if target_cpu == "generic" {
            target_cpu = target.options.cpu.clone();
        }

        let mut features = vec![];
        if target_cpu == "native" {
            let native_cpu = TargetMachine::get_host_cpu_name();
            let native_features = TargetMachine::get_host_cpu_features();

            features.extend(native_features.to_string().split(',').map(String::from));
            target_cpu = native_cpu.to_string();
        }

        features.extend(target.options.features.split(',').filter(|v| !v.is_empty()).map(String::from));
        features.extend(target_features.iter().cloned());

        LLVMBackend { target, target_cpu, features: features.join(",") }
    }

    pub fn new_module(
        &self,
        name: &str,
        opt_lvl: OptimizationLevel,
    ) -> Result<ModuleLlvm, String> {
        ModuleLlvm::new(name, self.target, &self.target_cpu, &self.features, opt_lvl)
    }

    pub fn new_ctx<'a, 'ctx>(
        &'a self,
        literals: &'a Rodeo,
        module: &'ctx ModuleLlvm<'ctx>,
    ) -> CodegenCx<'a, 'ctx> {
        CodegenCx::new(literals, module, self.target)
    }

    pub fn target(&self) -> &'t TargetSpec {
        self.target
    }
}

pub struct ModuleLlvm<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    target_machine: TargetMachine,
    opt_lvl: OptimizationLevel,
}

impl<'ctx> ModuleLlvm<'ctx> {
    pub fn new(
        name: &str,
        target: &TargetSpec,
        target_cpu: &str,
        features: &str,
        opt_lvl: OptimizationLevel,
    ) -> Result<ModuleLlvm<'ctx>, String> {
        // We need a static context for the 'ctx lifetime
        // In a real scenario, this should be managed by the caller
        // For now, we'll use Box::leak to create a 'static reference
        let context = Box::leak(Box::new(Context::create()));

        let module = context.create_module(name);

        // Set target triple
        let triple = TargetTriple::create(&target.llvm_target);
        module.set_triple(&triple);

        // Create target machine first so we can get the data layout from it
        let target_obj = Target::from_triple(&triple)
            .map_err(|e| e.to_string())?;

        let target_machine = target_obj
            .create_target_machine(
                &triple,
                target_cpu,
                features,
                opt_lvl,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .ok_or_else(|| format!("Could not create target machine for {}", target.llvm_target))?;

        // Set data layout from target machine
        module.set_data_layout(&target_machine.get_target_data().get_data_layout());

        Ok(ModuleLlvm {
            context,
            module,
            target_machine,
            opt_lvl,
        })
    }

    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    pub fn module(&self) -> &'ctx Module<'ctx> {
        // Safety: We know the module has the same lifetime as context
        unsafe { std::mem::transmute(&self.module) }
    }

    pub fn to_str(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn optimize(&self) {
        // Note: PassManagerBuilder was removed in LLVM 18+
        // For now, we'll skip optimization in this stub
        // TODO: Implement new pass manager API when inkwell adds support
        // or use the command-line pass pipeline
        log::warn!("Optimization pass skipped - PassManagerBuilder not available in LLVM 18+");
    }

    pub fn verify_and_print(&self) -> bool {
        self.module.verify().is_ok()
    }

    pub fn verify(&self) -> Option<String> {
        self.module.verify().err().map(|e| e.to_string())
    }

    pub fn emit_object(&self, dst: &Path) -> Result<(), String> {
        self.target_machine
            .write_to_file(&self.module, inkwell::targets::FileType::Object, dst)
            .map_err(|e| e.to_string())
    }
}

// Helper functions for compatibility
pub fn get_host_cpu_name() -> String {
    TargetMachine::get_host_cpu_name().to_string()
}

pub fn get_host_cpu_features() -> String {
    TargetMachine::get_host_cpu_features().to_string()
}
