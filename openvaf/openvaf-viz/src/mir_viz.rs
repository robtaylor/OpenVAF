//! MIR visualization utilities

use mir::{Block, ControlFlowGraph, Function, InstructionData};
use std::fmt::Write;

/// Generate DOT format output for the control flow graph
pub fn generate_dot(func: &Function, cfg: &ControlFlowGraph, name: &str) -> String {
    let mut out = String::new();

    writeln!(out, "digraph {} {{", escape_dot_id(name)).unwrap();
    writeln!(out, "    rankdir=TB;").unwrap();
    writeln!(out, "    node [shape=box, fontname=\"monospace\", fontsize=10];").unwrap();
    writeln!(out, "    edge [fontname=\"monospace\", fontsize=9];").unwrap();
    writeln!(out).unwrap();

    // Nodes (basic blocks)
    for block in func.layout.blocks() {
        let label = format_block_label(func, block);
        let color = block_color(func, cfg, block);

        writeln!(
            out,
            "    block{} [label=\"{}\", style=filled, fillcolor=\"{}\"];",
            u32::from(block),
            escape_dot_label(&label),
            color
        )
        .unwrap();
    }

    writeln!(out).unwrap();

    // Edges
    for block in func.layout.blocks() {
        if let Some(last_inst) = func.layout.last_inst(block) {
            match &func.dfg.insts[last_inst] {
                InstructionData::Jump { destination } => {
                    writeln!(
                        out,
                        "    block{} -> block{};",
                        u32::from(block),
                        u32::from(*destination)
                    )
                    .unwrap();
                }
                InstructionData::Branch { then_dst, else_dst, loop_entry, .. } => {
                    let then_style = if *loop_entry { ", style=bold, color=blue" } else { "" };
                    writeln!(
                        out,
                        "    block{} -> block{} [label=\"T\"{then_style}];",
                        u32::from(block),
                        u32::from(*then_dst),
                    )
                    .unwrap();
                    writeln!(
                        out,
                        "    block{} -> block{} [label=\"F\"];",
                        u32::from(block),
                        u32::from(*else_dst)
                    )
                    .unwrap();
                }
                _ => {}
            }
        }
    }

    writeln!(out, "}}").unwrap();
    out
}

fn format_block_label(func: &Function, block: Block) -> String {
    let mut label = format!("block{}", u32::from(block));

    let inst_count = func.layout.block_insts(block).count();
    label.push_str(&format!("\\n({} insts)", inst_count));

    // Show first and last instruction
    let mut insts: Vec<_> = func.layout.block_insts(block).collect();
    if !insts.is_empty() {
        let first = insts[0];
        let first_opcode = func.dfg.insts[first].opcode();
        label.push_str(&format!("\\n{:?}", first_opcode));

        if insts.len() > 1 {
            if insts.len() > 2 {
                label.push_str("\\n...");
            }
            let last = insts.pop().unwrap();
            let last_opcode = func.dfg.insts[last].opcode();
            label.push_str(&format!("\\n{:?}", last_opcode));
        }
    }

    label
}

fn block_color(func: &Function, cfg: &ControlFlowGraph, block: Block) -> &'static str {
    // Entry block
    if func.layout.entry_block() == Some(block) {
        return "#90EE90"; // Light green
    }

    // Exit block (no successors)
    if cfg.succ_iter(block).next().is_none() {
        return "#FFB6C1"; // Light pink
    }

    // Loop header (has back edge)
    let block_idx = u32::from(block);
    for pred in cfg.pred_iter(block) {
        if u32::from(pred) > block_idx {
            return "#FFFACD"; // Light yellow
        }
    }

    "#FFFFFF" // White
}

fn escape_dot_id(s: &str) -> String {
    s.chars().map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' }).collect()
}

fn escape_dot_label(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

/// Generate enhanced DOT with instruction details
#[allow(dead_code)]
pub fn generate_detailed_dot(func: &Function, cfg: &ControlFlowGraph, name: &str) -> String {
    let mut out = String::new();

    writeln!(out, "digraph {} {{", escape_dot_id(name)).unwrap();
    writeln!(out, "    rankdir=TB;").unwrap();
    writeln!(out, "    node [shape=none, fontname=\"monospace\", fontsize=9];").unwrap();
    writeln!(out, "    edge [fontname=\"monospace\", fontsize=8];").unwrap();
    writeln!(out).unwrap();

    // Nodes with HTML-like labels for instruction tables
    for block in func.layout.blocks() {
        let color = block_color(func, cfg, block);
        let label = format_block_html_label(func, block);

        writeln!(
            out,
            "    block{} [label=<{}>];",
            u32::from(block),
            label.replace("{color}", color)
        )
        .unwrap();
    }

    writeln!(out).unwrap();

    // Edges
    for block in func.layout.blocks() {
        if let Some(last_inst) = func.layout.last_inst(block) {
            match &func.dfg.insts[last_inst] {
                InstructionData::Jump { destination } => {
                    writeln!(
                        out,
                        "    block{} -> block{};",
                        u32::from(block),
                        u32::from(*destination)
                    )
                    .unwrap();
                }
                InstructionData::Branch { then_dst, else_dst, .. } => {
                    writeln!(
                        out,
                        "    block{} -> block{} [label=\"T\", color=green];",
                        u32::from(block),
                        u32::from(*then_dst)
                    )
                    .unwrap();
                    writeln!(
                        out,
                        "    block{} -> block{} [label=\"F\", color=red];",
                        u32::from(block),
                        u32::from(*else_dst)
                    )
                    .unwrap();
                }
                _ => {}
            }
        }
    }

    writeln!(out, "}}").unwrap();
    out
}

#[allow(dead_code)]
fn format_block_html_label(func: &Function, block: Block) -> String {
    let mut html = String::new();
    html.push_str("<TABLE BORDER=\"1\" CELLBORDER=\"0\" CELLSPACING=\"0\" BGCOLOR=\"{color}\">");
    html.push_str(&format!("<TR><TD COLSPAN=\"3\"><B>block{}</B></TD></TR>", u32::from(block)));

    for inst in func.layout.block_insts(block) {
        let data = &func.dfg.insts[inst];
        let opcode = data.opcode();

        let results: Vec<_> =
            func.dfg.inst_results(inst).iter().map(|v| format!("v{}", u32::from(*v))).collect();

        let args: Vec<_> = if let InstructionData::PhiNode(phi) = data {
            func.dfg
                .phi_edges(phi)
                .map(|(b, v)| format!("b{}:v{}", u32::from(b), u32::from(v)))
                .collect()
        } else {
            func.dfg.instr_args(inst).iter().map(|v| format!("v{}", u32::from(*v))).collect()
        };

        let result_str =
            if results.is_empty() { String::new() } else { format!("{} = ", results.join(", ")) };

        html.push_str(&format!(
            "<TR><TD ALIGN=\"RIGHT\">{}</TD><TD>{:?}</TD><TD ALIGN=\"LEFT\">{}</TD></TR>",
            escape_html(&result_str),
            opcode,
            escape_html(&args.join(", "))
        ));
    }

    html.push_str("</TABLE>");
    html
}

#[allow(dead_code)]
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
