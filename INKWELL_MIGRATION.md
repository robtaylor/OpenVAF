# Inkwell Migration Plan

Migration from `llvm-sys` (raw LLVM-C bindings) to `inkwell` (safe Rust LLVM wrapper).

**Branch:** `migrate-to-inkwell`
**Target LLVM Version:** 19+
**Strategy:** Big-bang migration with idiomatic inkwell adoption
**Started:** 2025-11-23

## Overview

This migration replaces raw LLVM-C FFI calls with inkwell's type-safe Rust API throughout the OpenVAF codebase. The main affected modules are `mir_llvm` (LLVM IR generation) and `osdi` (OSDI library compilation).

## Progress Tracking

### ✅ Phase 1: Core Infrastructure (COMPLETED)

**Commits:**
- `27e212b` - Replace llvm-sys with inkwell dependency
- `739b850` - WIP: Migrate core mir_llvm modules to inkwell

**Files Migrated:**

1. **openvaf/mir_llvm/Cargo.toml** ✅
   - Replaced `llvm-sys = "181.1.1"` with `inkwell = { version = "0.5", features = ["llvm19-0", "target-all"] }`

2. **openvaf/mir_llvm/src/types.rs** ✅ (286→220 lines)
   - Migrated from `&'ll LLVMType` to inkwell's `IntType<'ctx>`, `FloatType<'ctx>`, etc.
   - Replaced custom lifetime `'ll` with inkwell's `'ctx` convention
   - Used inkwell's `BasicTypeEnum` for type polymorphism
   - Updated all const value creation methods

3. **openvaf/mir_llvm/src/context.rs** ✅ (152→104 lines)
   - Changed `CodegenCx` to use `&'ctx Module<'ctx>` and `&'ctx Context`
   - Migrated bitcode parsing to `MemoryBuffer` and `Module::parse_bitcode_from_buffer`
   - Updated module linking to use `Module::link_in_module`
   - Removed raw LLVM pointer manipulation

4. **openvaf/mir_llvm/src/declarations.rs** ✅ (308→86 lines)
   - Simplified using inkwell's `Module::add_function` and `Module::add_global`
   - Updated calling conventions and linkage using inkwell enums
   - Removed manual pointer casting

5. **openvaf/mir_llvm/src/intrinsics.rs** ✅ (106→88 lines)
   - Updated to use `BasicTypeEnum` and `FunctionValue`
   - Maintained same intrinsic declarations (LLVM math, C stdlib)

6. **openvaf/mir_llvm/src/lib.rs** ✅ (530→185 lines)
   - Removed custom `LLVMString` wrapper (inkwell handles this internally)
   - Rewrote `ModuleLlvm` to use inkwell's `Context`, `Module`, `TargetMachine`
   - Simplified target initialization with `Target::initialize_native`
   - Updated optimization passes to use `PassManager` and `PassManagerBuilder`
   - Removed diagnostic handler (can be added back if needed)
   - Exported `OptimizationLevel as LLVMCodeGenOptLevel` for compatibility

**Key Changes:**
- Lifetime renamed: `'ll` → `'ctx` (inkwell convention)
- Removed ~700 lines of unsafe code
- Type-safe API throughout
- Better error handling with Result types

### 🚧 Phase 2: IR Builder (IN PROGRESS)

**Files Remaining:**

1. **openvaf/mir_llvm/src/builder.rs** 🔄 (~1,189 lines)
   - **Status:** Not started
   - **Complexity:** Very High - This is the largest and most complex file
   - **Key Components:**
     - `MemLoc<'ll>` - Memory location abstraction for GEP chains
       - Need to update to use inkwell's `PointerValue` and GEP methods
     - `BuilderVal<'ll>` - Lazy value loading enum
       - Update to use inkwell value types
     - `Builder<'a, 'cx, 'll>` - Main IR builder
       - Migrate to use `inkwell::builder::Builder`
       - Update all build_* methods (add, sub, mul, call, etc.)
       - Control flow: branches, switches, phi nodes
       - Memory operations: load, store, GEP, struct_gep
   - **Challenges:**
     - Complex lifetime management
     - Integration with MIR (mid-level IR)
     - Phi node handling
     - Switch statement construction

2. **openvaf/mir_llvm/src/callbacks.rs** 🔄 (117 lines)
   - **Status:** Not started
   - **Complexity:** Medium
   - **Key Components:**
     - `InlineCallbackBuilder` trait
     - `BuiltCallbackFun` struct
     - `CallbackFun` enum
   - **Changes Needed:**
     - Update all `&'ll llvm_sys::LLVMValue` to inkwell types
     - Migrate basic block and builder creation
     - Update function creation in callbacks

### 🔲 Phase 3: OSDI Module (~5,924 lines)

**Files to Migrate:**

1. **openvaf/osdi/Cargo.toml** 🔲
   - Update dependency from `llvm-sys = "181.1.1"` to inkwell

2. **openvaf/osdi/src/lib.rs** 🔲 (486 lines)
   - LLVM initialization
   - Compilation orchestration
   - Module management

3. **openvaf/osdi/src/compilation_unit.rs** 🔲 (596 lines)
   - Compilation unit management
   - LLVM module handling

4. **openvaf/osdi/src/inst_data.rs** 🔲 (1,299 lines)
   - Instance data structure generation
   - Largest OSDI file

5. **openvaf/osdi/src/model_data.rs** 🔲 (298 lines)
   - Model data structures

6. **openvaf/osdi/src/metadata.rs** 🔲 (552 lines)
   - OSDI metadata generation
   - Uses LLVM constants and globals extensively

7. **openvaf/osdi/src/access.rs** 🔲 (484 lines)
   - Parameter access functions

8. **openvaf/osdi/src/setup.rs** 🔲 (575 lines)
   - Model/instance setup functions

9. **openvaf/osdi/src/eval.rs** 🔲 (613 lines)
   - Evaluation functions

10. **openvaf/osdi/src/load.rs** 🔲 (554 lines)
    - Jacobian loading

11. **openvaf/osdi/src/bitfield.rs** 🔲 (167 lines)
    - Bitfield operations

### 🔲 Phase 4: Dependent Modules

1. **openvaf/openvaf/src/** 🔲
   - Main compiler binary
   - Uses `LLVMCodeGenOptLevel` (now aliased to `OptimizationLevel`)

2. **verilogae/verilogae/src/** 🔲
   - Python bindings
   - Exports `LLVMCodeGenOptLevel`

### 🔲 Phase 5: Testing & Validation

1. **Fix Compilation Errors** 🔲
   - Resolve all type mismatches
   - Fix lifetime issues
   - Update method calls

2. **Integration Tests** 🔲
   - Run `openvaf/osdi/tests/data_tests.rs`
   - Run `openvaf/openvaf/tests/integration.rs`
   - Compare test results with baseline

3. **IR Comparison** 🔲
   - Generate LLVM IR from sample Verilog-A files
   - Compare old vs new generated IR
   - Ensure functional equivalence

4. **Performance Validation** 🔲
   - Benchmark compilation times
   - Check generated code performance
   - Profile if regressions found

### 🔲 Phase 6: Cleanup & Documentation

1. **Code Cleanup** 🔲
   - Run `cargo fmt`
   - Run `cargo clippy` and fix warnings
   - Remove any remaining dead code

2. **Documentation** 🔲
   - Update README with new LLVM version
   - Document inkwell usage patterns
   - Update build instructions

3. **Pull Request** 🔲
   - Create PR against `master`
   - Write comprehensive PR description
   - Request review

## Known Issues & Challenges

### Lifetime Management
- inkwell uses `'ctx` lifetime tied to `Context`
- Original code used `'ll` tied to module
- Need careful lifetime management in builder.rs

### Context Ownership
- Current implementation uses `Box::leak` in `ModuleLlvm::new`
- This is a workaround for lifetime issues
- May need to reconsider ownership model

### Builder Complexity
- builder.rs is highly complex with many LLVM operations
- Integrates deeply with MIR (mid-level IR)
- Will require careful porting to maintain semantics

### Type Conversions
- inkwell uses type-safe enums (`BasicTypeEnum`, `IntValue`, etc.)
- Original code used raw pointers (`&'ll LLVMType`, `&'ll LLVMValue`)
- Many match statements needed for conversions

## Migration Guidelines

### Do's
✅ Use inkwell's type-safe APIs (`IntValue`, `FunctionValue`, etc.)
✅ Leverage inkwell's builder methods instead of raw FFI
✅ Use inkwell's `Result` types for error handling
✅ Follow inkwell's `'ctx` lifetime convention
✅ Commit frequently with clear messages

### Don'ts
❌ Mix llvm-sys and inkwell in the same module
❌ Use unsafe unless absolutely necessary
❌ Skip testing after major changes
❌ Change behavior - aim for equivalent IR generation

## References

- **Inkwell Documentation:** https://thedan64.github.io/inkwell/
- **Inkwell Repository:** https://github.com/TheDan64/inkwell
- **LLVM 19 Release Notes:** https://llvm.org/docs/ReleaseNotes.html
- **Original Discussion:** Branch created from commit `4745d1f`

## Next Steps

**Immediate Priority:**
1. Migrate `builder.rs` - This is the critical path
2. Migrate `callbacks.rs`
3. Get mir_llvm compiling
4. Then tackle OSDI module

**Estimated Effort:**
- builder.rs: 8-12 hours
- callbacks.rs: 2-3 hours
- OSDI module: 12-16 hours
- Testing: 4-6 hours
- **Total: 26-37 hours** over multiple sessions

## Session Notes

### Session 1 (2025-11-23)
- Created migration branch
- Updated dependencies
- Migrated core infrastructure (types, context, declarations, intrinsics, lib)
- Reduced mir_llvm from 2,684 to ~1,984 lines (excluding builder/callbacks)
- Identified builder.rs as main remaining challenge
- Created this migration plan document

---

**Last Updated:** 2025-11-23
**Next Session:** Continue with builder.rs migration
