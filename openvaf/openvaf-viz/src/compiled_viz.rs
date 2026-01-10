//! CompiledModule visualization utilities

use hir::CompilationDB;
use hir_lower::{CurrentKind, ParamKind, PlaceKind};
use lasso::Rodeo;
use sim_back::{CompiledModule, ModuleInfo, SimUnknownKind};
use std::fmt::Write;

/// Generate a text summary of the CompiledModule
pub fn generate_text_summary(
    db: &CompilationDB,
    module_info: &ModuleInfo,
    compiled: &CompiledModule,
    _literals: &Rodeo,
) -> String {
    let mut out = String::new();
    let module = module_info.module;

    writeln!(out, "Module: {}", module.name(db)).unwrap();
    writeln!(out, "========================================").unwrap();
    writeln!(out).unwrap();

    // Ports
    writeln!(out, "Ports:").unwrap();
    for port in module.ports(db) {
        writeln!(out, "  - {}", port.name(db)).unwrap();
    }
    writeln!(out).unwrap();

    // Internal nodes
    writeln!(out, "Internal Nodes:").unwrap();
    for node in module.internal_nodes(db) {
        writeln!(out, "  - {}", node.name(db)).unwrap();
    }
    writeln!(out).unwrap();

    // DAE System
    writeln!(out, "DAE System:").unwrap();
    writeln!(out, "  Unknowns: {}", compiled.dae_system.unknowns.len()).unwrap();
    writeln!(out, "  Resistive equations: {}", compiled.dae_system.num_resistive).unwrap();
    writeln!(out, "  Reactive equations: {}", compiled.dae_system.num_reactive).unwrap();
    writeln!(out, "  Jacobian entries: {}", compiled.dae_system.jacobian.len()).unwrap();
    writeln!(out, "  Noise sources: {}", compiled.dae_system.noise_sources.len()).unwrap();
    writeln!(out).unwrap();

    // Unknowns detail
    writeln!(out, "  Unknown variables:").unwrap();
    for kind in compiled.dae_system.unknowns.iter() {
        let desc = match kind {
            SimUnknownKind::KirchoffLaw(node) => {
                format!("KirchoffLaw({})", node.name(db))
            }
            SimUnknownKind::Current(curr) => match curr {
                CurrentKind::Branch(br) => format!("Current(branch {})", br.name(db)),
                CurrentKind::Unnamed { hi, lo } => {
                    if let Some(lo) = lo {
                        format!("Current(I({}, {}))", hi.name(db), lo.name(db))
                    } else {
                        format!("Current(I({}))", hi.name(db))
                    }
                }
                CurrentKind::Port(n) => format!("Current(port {})", n.name(db)),
            },
            SimUnknownKind::Implicit(eq) => format!("Implicit(eq{})", u32::from(*eq)),
        };
        writeln!(out, "    - {}", desc).unwrap();
    }
    writeln!(out).unwrap();

    // Functions summary
    writeln!(out, "Functions:").unwrap();
    writeln!(
        out,
        "  eval: {} blocks, {} values",
        compiled.eval.layout.num_blocks(),
        compiled.eval.dfg.num_values()
    )
    .unwrap();
    writeln!(
        out,
        "  init: {} blocks, {} values",
        compiled.init.func.layout.num_blocks(),
        compiled.init.func.dfg.num_values()
    )
    .unwrap();
    writeln!(
        out,
        "  model_param_setup: {} blocks, {} values",
        compiled.model_param_setup.layout.num_blocks(),
        compiled.model_param_setup.dfg.num_values()
    )
    .unwrap();
    writeln!(out).unwrap();

    // Parameters
    writeln!(out, "Parameters (HirInterner):").unwrap();
    for (kind, val) in compiled.intern.params.iter() {
        let desc = format_param_kind(kind, db);
        writeln!(out, "  {} -> v{}", desc, u32::from(*val)).unwrap();
    }
    writeln!(out).unwrap();

    // Outputs
    writeln!(out, "Outputs:").unwrap();
    for (kind, val) in compiled.intern.outputs.iter() {
        if let Some(v) = val.expand() {
            let desc = format_place_kind(kind, db);
            writeln!(out, "  {} -> v{}", desc, u32::from(v)).unwrap();
        }
    }
    writeln!(out).unwrap();

    // Callbacks
    writeln!(out, "Callbacks: {}", compiled.intern.callbacks.len()).unwrap();
    writeln!(out).unwrap();

    // Implicit equations
    writeln!(out, "Implicit Equations: {}", compiled.intern.implicit_equations.len()).unwrap();

    out
}

fn format_param_kind(kind: &ParamKind, db: &CompilationDB) -> String {
    match kind {
        ParamKind::Param(param) => format!("param({})", param.name(db)),
        ParamKind::ParamGiven { param } => format!("param_given({})", param.name(db)),
        ParamKind::Voltage { hi, lo } => {
            if let Some(lo) = lo {
                format!("V({}, {})", hi.name(db), lo.name(db))
            } else {
                format!("V({})", hi.name(db))
            }
        }
        ParamKind::Current(curr) => match curr {
            CurrentKind::Branch(br) => format!("I(branch {})", br.name(db)),
            CurrentKind::Unnamed { hi, lo } => {
                if let Some(lo) = lo {
                    format!("I({}, {})", hi.name(db), lo.name(db))
                } else {
                    format!("I({})", hi.name(db))
                }
            }
            CurrentKind::Port(n) => format!("I(port {})", n.name(db)),
        },
        ParamKind::Temperature => "$temperature".to_string(),
        ParamKind::Abstime => "$abstime".to_string(),
        ParamKind::HiddenState(var) => format!("hidden_state({})", var.name(db)),
        ParamKind::PortConnected { port } => format!("port_connected({})", port.name(db)),
        ParamKind::ParamSysFun(f) => format!("${:?}", f),
        _ => format!("{:?}", kind),
    }
}

fn format_place_kind(kind: &PlaceKind, db: &CompilationDB) -> String {
    match kind {
        PlaceKind::Var(var) => format!("var({})", var.name(db)),
        PlaceKind::FunctionReturn(func) => format!("return({})", func.name(db)),
        PlaceKind::FunctionArg(arg) => format!("arg({})", arg.name(db)),
        _ => format!("{:?}", kind),
    }
}
