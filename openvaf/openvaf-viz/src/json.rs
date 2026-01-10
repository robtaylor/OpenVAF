//! JSON serialization for OpenVAF IR structures

use hir::CompilationDB;
use hir_lower::{CurrentKind, ParamKind};
use lasso::Rodeo;
use mir::{ControlFlowGraph, Function, InstructionData, ValueDef};
use serde::Serialize;
use sim_back::dae::{NoiseSourceKind, SimUnknown};
use sim_back::{CompiledModule, ModuleInfo, SimUnknownKind};

/// Options for JSON output generation
pub struct JsonOptions {
    pub eval_only: bool,
    pub init_only: bool,
    pub model_param_only: bool,
}

/// Generate JSON representation of the compiled module
pub fn generate_json(
    db: &CompilationDB,
    module_info: &ModuleInfo,
    compiled: &CompiledModule,
    literals: &Rodeo,
    options: &JsonOptions,
) -> String {
    let viz = CompiledModuleViz::new(db, module_info, compiled, literals, options);
    serde_json::to_string_pretty(&viz).unwrap()
}

#[derive(Serialize)]
pub struct CompiledModuleViz {
    pub module_name: String,
    pub ports: Vec<String>,
    pub internal_nodes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval: Option<FunctionViz>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<FunctionViz>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_param_setup: Option<FunctionViz>,
    pub dae_system: DaeSystemViz,
    pub parameters: Vec<ParameterViz>,
    pub outputs: Vec<OutputViz>,
}

impl CompiledModuleViz {
    pub fn new(
        db: &CompilationDB,
        module_info: &ModuleInfo,
        compiled: &CompiledModule,
        literals: &Rodeo,
        options: &JsonOptions,
    ) -> Self {
        let module = module_info.module;

        let ports: Vec<String> = module.ports(db).iter().map(|n| n.name(db).to_string()).collect();

        let internal_nodes: Vec<String> =
            module.internal_nodes(db).iter().map(|n| n.name(db).to_string()).collect();

        let eval = if !options.init_only && !options.model_param_only {
            Some(FunctionViz::new(&compiled.eval, literals, "eval"))
        } else {
            None
        };

        let init = if !options.eval_only && !options.model_param_only {
            Some(FunctionViz::new(&compiled.init.func, literals, "init"))
        } else {
            None
        };

        let model_param_setup = if !options.eval_only && !options.init_only {
            Some(FunctionViz::new(&compiled.model_param_setup, literals, "model_param_setup"))
        } else {
            None
        };

        let dae_system = DaeSystemViz::new(&compiled.dae_system, db, literals);

        let parameters = compiled
            .intern
            .params
            .iter()
            .map(|(kind, val)| ParameterViz::new(kind, *val, db))
            .collect();

        let outputs = compiled
            .intern
            .outputs
            .iter()
            .filter_map(|(kind, val)| {
                val.expand().map(|v| OutputViz {
                    kind: format!("{:?}", kind),
                    value: format!("v{}", u32::from(v)),
                })
            })
            .collect();

        CompiledModuleViz {
            module_name: module.name(db),
            ports,
            internal_nodes,
            eval,
            init,
            model_param_setup,
            dae_system,
            parameters,
            outputs,
        }
    }
}

#[derive(Serialize)]
pub struct FunctionViz {
    pub name: String,
    pub num_blocks: usize,
    pub num_instructions: usize,
    pub num_values: usize,
    pub blocks: Vec<BlockViz>,
}

impl FunctionViz {
    pub fn new(func: &Function, literals: &Rodeo, name: &str) -> Self {
        let cfg = ControlFlowGraph::with_function(func);

        let mut num_instructions = 0;
        let blocks: Vec<BlockViz> = func
            .layout
            .blocks()
            .map(|block| {
                let instructions: Vec<InstructionViz> = func
                    .layout
                    .block_insts(block)
                    .map(|inst| {
                        num_instructions += 1;
                        InstructionViz::new(func, inst, literals)
                    })
                    .collect();

                let predecessors: Vec<String> =
                    cfg.pred_iter(block).map(|b| format!("block{}", u32::from(b))).collect();

                let successors: Vec<String> =
                    cfg.succ_iter(block).map(|b| format!("block{}", u32::from(b))).collect();

                BlockViz {
                    id: format!("block{}", u32::from(block)),
                    instructions,
                    predecessors,
                    successors,
                }
            })
            .collect();

        let num_values = func.dfg.num_values();

        FunctionViz {
            name: name.to_string(),
            num_blocks: blocks.len(),
            num_instructions,
            num_values,
            blocks,
        }
    }
}

#[derive(Serialize)]
pub struct BlockViz {
    pub id: String,
    pub instructions: Vec<InstructionViz>,
    pub predecessors: Vec<String>,
    pub successors: Vec<String>,
}

#[derive(Serialize)]
pub struct InstructionViz {
    pub id: String,
    pub opcode: String,
    pub opcode_category: String,
    pub arguments: Vec<String>,
    pub results: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_targets: Option<BranchTargets>,
}

#[derive(Serialize)]
pub struct BranchTargets {
    pub then_block: String,
    pub else_block: String,
}

impl InstructionViz {
    pub fn new(func: &Function, inst: mir::Inst, literals: &Rodeo) -> Self {
        let data = &func.dfg.insts[inst];
        let opcode = data.opcode();

        let arguments: Vec<String> = if let InstructionData::PhiNode(phi) = data {
            func.dfg
                .phi_edges(phi)
                .map(|(block, val)| {
                    format!("block{}: {}", u32::from(block), format_value(func, val, literals))
                })
                .collect()
        } else {
            func.dfg.instr_args(inst).iter().map(|&v| format_value(func, v, literals)).collect()
        };

        let results: Vec<String> =
            func.dfg.inst_results(inst).iter().map(|&v| format!("v{}", u32::from(v))).collect();

        let branch_targets = match data {
            InstructionData::Branch { then_dst, else_dst, .. } => Some(BranchTargets {
                then_block: format!("block{}", u32::from(*then_dst)),
                else_block: format!("block{}", u32::from(*else_dst)),
            }),
            _ => None,
        };

        InstructionViz {
            id: format!("inst{}", u32::from(inst)),
            opcode: format!("{:?}", opcode),
            opcode_category: categorize_opcode(opcode),
            arguments,
            results,
            branch_targets,
        }
    }
}

fn format_value(func: &Function, val: mir::Value, literals: &Rodeo) -> String {
    match func.dfg.value_def(val) {
        ValueDef::Result(_, _) => format!("v{}", u32::from(val)),
        ValueDef::Param(param) => format!("param{}", u32::from(param)),
        ValueDef::Const(c) => match c {
            mir::Const::Float(f) => format!("{}", f64::from(f)),
            mir::Const::Int(i) => format!("{}", i),
            mir::Const::Str(s) => format!("\"{}\"", &literals[s]),
            mir::Const::Bool(b) => format!("{}", b),
        },
        ValueDef::Invalid => "invalid".to_string(),
    }
}

fn categorize_opcode(opcode: mir::Opcode) -> String {
    use mir::Opcode::*;
    match opcode {
        Iadd | Isub | Imul | Idiv | Irem | Ineg => "integer_arithmetic",
        Fadd | Fsub | Fmul | Fdiv | Fneg => "float_arithmetic",
        Sqrt | Exp | Ln | Log | Sin | Cos | Tan | Asin | Acos | Atan | Atan2 | Sinh | Cosh
        | Tanh | Asinh | Acosh | Atanh | Pow | Hypot | Floor | Ceil => "math",
        Ilt | Igt | Ile | Ige | Ieq | Ine | Flt | Fgt | Fle | Fge | Feq | Fne | Beq | Bne => {
            "comparison"
        }
        Iand | Ior | Ixor | Inot | Bnot | Seq => "logical",
        FIcast | IFcast | BIcast | IBcast | FBcast | BFcast => "cast",
        Br | Jmp | Exit => "control_flow",
        Phi => "phi",
        Call => "call",
        _ => "other",
    }
    .to_string()
}

#[derive(Serialize)]
pub struct DaeSystemViz {
    pub num_unknowns: usize,
    pub num_resistive: u32,
    pub num_reactive: u32,
    pub unknowns: Vec<UnknownViz>,
    pub jacobian_entries: usize,
    pub jacobian: Vec<JacobianEntryViz>,
    pub noise_sources: Vec<NoiseSourceViz>,
    pub topology: TopologyViz,
}

#[derive(Serialize)]
pub struct JacobianEntryViz {
    pub row: usize,
    pub col: usize,
    pub row_name: String,
    pub col_name: String,
    pub has_resist: bool,
    pub has_react: bool,
}

#[derive(Serialize)]
pub struct NoiseSourceViz {
    pub name: String,
    pub kind: String,
    pub hi: String,
    pub lo: Option<String>,
}

#[derive(Serialize)]
pub struct TopologyViz {
    pub nodes: Vec<TopologyNodeViz>,
    pub edges: Vec<TopologyEdgeViz>,
}

#[derive(Serialize)]
pub struct TopologyNodeViz {
    pub id: usize,
    pub name: String,
    pub kind: String, // "node", "current", "implicit"
}

#[derive(Serialize)]
pub struct TopologyEdgeViz {
    pub from: usize,
    pub to: usize,
    pub edge_type: String, // "jacobian_resist", "jacobian_react", "both"
}

impl DaeSystemViz {
    pub fn new(dae: &sim_back::dae::DaeSystem, db: &CompilationDB, literals: &Rodeo) -> Self {
        // Collect unknowns with their indices
        let unknowns: Vec<UnknownViz> = dae
            .unknowns
            .iter_enumerated()
            .map(|(idx, kind)| UnknownViz::new_with_index(kind, db, idx))
            .collect();

        // Build jacobian entries visualization
        let jacobian: Vec<JacobianEntryViz> = dae
            .jacobian
            .iter()
            .map(|entry| {
                let row_idx = u32::from(entry.row) as usize;
                let col_idx = u32::from(entry.col) as usize;
                let row_name = unknowns.get(row_idx).map(|u| u.name.clone()).unwrap_or_default();
                let col_name = unknowns.get(col_idx).map(|u| u.name.clone()).unwrap_or_default();

                // Check if values are the FALSE constant (used as placeholder for no value)
                let has_resist = u32::from(entry.resist) != 0;
                let has_react = u32::from(entry.react) != 0;

                JacobianEntryViz {
                    row: row_idx,
                    col: col_idx,
                    row_name,
                    col_name,
                    has_resist,
                    has_react,
                }
            })
            .collect();

        // Build noise sources visualization
        let noise_sources: Vec<NoiseSourceViz> = dae
            .noise_sources
            .iter()
            .map(|ns| {
                let hi_idx = u32::from(ns.hi) as usize;
                let hi_name = unknowns.get(hi_idx).map(|u| u.name.clone()).unwrap_or_default();
                let lo_name = ns.lo.map(|lo| {
                    let lo_idx = u32::from(lo) as usize;
                    unknowns.get(lo_idx).map(|u| u.name.clone()).unwrap_or_default()
                });

                let kind = match &ns.kind {
                    NoiseSourceKind::WhiteNoise { .. } => "white".to_string(),
                    NoiseSourceKind::FlickerNoise { .. } => "flicker".to_string(),
                    NoiseSourceKind::NoiseTable { log, .. } => {
                        if *log {
                            "table_log".to_string()
                        } else {
                            "table".to_string()
                        }
                    }
                };

                NoiseSourceViz {
                    name: literals.resolve(&ns.name).to_string(),
                    kind,
                    hi: hi_name,
                    lo: lo_name,
                }
            })
            .collect();

        // Build topology graph
        let topology = build_topology(&unknowns, &jacobian);

        DaeSystemViz {
            num_unknowns: unknowns.len(),
            num_resistive: dae.num_resistive,
            num_reactive: dae.num_reactive,
            unknowns,
            jacobian_entries: jacobian.len(),
            jacobian,
            noise_sources,
            topology,
        }
    }
}

fn build_topology(unknowns: &[UnknownViz], jacobian: &[JacobianEntryViz]) -> TopologyViz {
    // Create nodes from unknowns
    let nodes: Vec<TopologyNodeViz> = unknowns
        .iter()
        .map(|u| TopologyNodeViz { id: u.index, name: u.name.clone(), kind: u.kind.clone() })
        .collect();

    // Create edges from jacobian entries (dependencies between unknowns)
    let edges: Vec<TopologyEdgeViz> = jacobian
        .iter()
        .filter(|j| j.row != j.col) // Skip diagonal (self-dependencies)
        .map(|j| {
            let edge_type = match (j.has_resist, j.has_react) {
                (true, true) => "both",
                (true, false) => "jacobian_resist",
                (false, true) => "jacobian_react",
                (false, false) => "none",
            };
            TopologyEdgeViz {
                from: j.col, // Column is the variable being differentiated
                to: j.row,   // Row is the equation affected
                edge_type: edge_type.to_string(),
            }
        })
        .collect();

    TopologyViz { nodes, edges }
}

#[derive(Serialize)]
pub struct UnknownViz {
    pub index: usize,
    pub kind: String,
    pub name: String,
}

impl UnknownViz {
    pub fn new_with_index(kind: &SimUnknownKind, db: &CompilationDB, idx: SimUnknown) -> Self {
        let (kind_str, name) = match kind {
            SimUnknownKind::KirchoffLaw(node) => ("kirchoff_law", node.name(db).to_string()),
            SimUnknownKind::Current(curr) => match curr {
                CurrentKind::Branch(br) => ("branch_current", br.name(db)),
                CurrentKind::Unnamed { hi, lo } => {
                    let name = if let Some(lo) = lo {
                        format!("I({}, {})", hi.name(db), lo.name(db))
                    } else {
                        format!("I({})", hi.name(db))
                    };
                    ("unnamed_current", name)
                }
                CurrentKind::Port(n) => ("port_current", n.name(db).to_string()),
            },
            SimUnknownKind::Implicit(eq) => ("implicit", format!("implicit_{}", u32::from(*eq))),
        };

        UnknownViz { index: u32::from(idx) as usize, kind: kind_str.to_string(), name }
    }
}

#[derive(Serialize)]
pub struct ParameterViz {
    pub kind: String,
    pub name: String,
    pub value: String,
}

impl ParameterViz {
    pub fn new(kind: &ParamKind, val: mir::Value, db: &CompilationDB) -> Self {
        let (kind_str, name) = match kind {
            ParamKind::Param(param) => ("param", param.name(db)),
            ParamKind::ParamGiven { param } => {
                ("param_given", format!("${}_given", param.name(db)))
            }
            ParamKind::Voltage { hi, lo } => {
                let name = if let Some(lo) = lo {
                    format!("V({}, {})", hi.name(db), lo.name(db))
                } else {
                    format!("V({})", hi.name(db))
                };
                ("voltage", name)
            }
            ParamKind::Current(curr) => match curr {
                CurrentKind::Branch(br) => ("branch_current", br.name(db)),
                CurrentKind::Unnamed { hi, lo } => {
                    let name = if let Some(lo) = lo {
                        format!("I({}, {})", hi.name(db), lo.name(db))
                    } else {
                        format!("I({})", hi.name(db))
                    };
                    ("unnamed_current", name)
                }
                CurrentKind::Port(n) => ("port_current", n.name(db).to_string()),
            },
            ParamKind::Temperature => ("sim_state", "$temperature".to_string()),
            ParamKind::Abstime => ("sim_state", "$abstime".to_string()),
            ParamKind::HiddenState(var) => ("hidden_state", var.name(db).to_string()),
            ParamKind::PortConnected { port } => ("port_connected", port.name(db).to_string()),
            ParamKind::ParamSysFun(f) => ("param_sys_fun", format!("${:?}", f)),
            _ => ("other", format!("{:?}", kind)),
        };

        ParameterViz { kind: kind_str.to_string(), name, value: format!("v{}", u32::from(val)) }
    }
}

#[derive(Serialize)]
pub struct OutputViz {
    pub kind: String,
    pub value: String,
}
