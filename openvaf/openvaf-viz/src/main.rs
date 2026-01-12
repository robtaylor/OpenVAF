//! OpenVAF IR Visualization Tool
//!
//! Generates interactive HTML visualizations for OpenVAF's intermediate representations:
//! - MIR (Mid-level IR): Control flow graphs, instructions, data flow
//! - CompiledModule: DAE system, parameter mappings, jacobian structure

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use clap::Parser;

mod compiled_viz;
mod html;
mod json;
mod mir_viz;

pub use crate::html::HtmlOptions;
pub use crate::json::JsonOptions;

use basedb::diagnostics::ConsoleSink;
use hir::{CompilationDB, CompilationOpts};
use lasso::Rodeo;
use paths::AbsPathBuf;
use sim_back::{collect_modules, CompiledModule};

/// Interactive visualization tool for OpenVAF intermediate representations
#[derive(Parser, Debug)]
#[command(name = "openvaf-viz")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input Verilog-A file
    #[arg(required = true)]
    input: Utf8PathBuf,

    /// Module to visualize (default: first module)
    #[arg(short, long)]
    module: Option<String>,

    /// Output file (default: <module>.html)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output format: html, json, dot
    #[arg(short, long, default_value = "html")]
    format: OutputFormat,

    /// Only show eval function
    #[arg(long)]
    eval_only: bool,

    /// Only show init function
    #[arg(long)]
    init_only: bool,

    /// Only show model_param_setup function
    #[arg(long)]
    model_param_only: bool,

    /// Skip CFG graph in output
    #[arg(long)]
    no_cfg: bool,

    /// Skip data flow edges in output
    #[arg(long)]
    no_dataflow: bool,

    /// Include unoptimized MIR
    #[arg(long)]
    dump_unopt: bool,

    /// Include directories for preprocessing
    #[arg(short = 'I', long = "include")]
    include: Vec<PathBuf>,

    /// Preprocessor defines
    #[arg(short = 'D', long = "define")]
    defines: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum OutputFormat {
    Html,
    Json,
    Dot,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Resolve input path
    let input = args
        .input
        .canonicalize_utf8()
        .with_context(|| format!("failed to resolve {}", args.input))?;
    let input = AbsPathBuf::assert(input.into_std_path_buf());

    // Convert include paths
    let include: Vec<AbsPathBuf> =
        args.include.iter().filter_map(|p| p.canonicalize().ok().map(AbsPathBuf::assert)).collect();

    // Create compilation database
    let db = CompilationDB::new_fs(input.clone(), &include, &args.defines, &[], &CompilationOpts::default())?;

    // Collect modules
    let modules = collect_modules(&db, false, &mut ConsoleSink::new(&db))
        .context("failed to collect modules - compilation errors present")?;

    if modules.is_empty() {
        anyhow::bail!("no modules found in {}", args.input);
    }

    // Find the requested module or use the first one
    let module_info = if let Some(ref name) = args.module {
        modules.iter().find(|m| m.module.name(&db) == *name).with_context(|| {
            let available: Vec<_> = modules.iter().map(|m| m.module.name(&db)).collect();
            format!("module '{}' not found. Available modules: {:?}", name, available)
        })?
    } else {
        &modules[0]
    };

    let module_name = module_info.module.name(&db);
    eprintln!("Visualizing module: {}", module_name);

    // Compile the module to get MIR
    let mut literals = Rodeo::new();
    let compiled = CompiledModule::new(&db, module_info, &mut literals, args.dump_unopt, true);

    // Determine output file
    let output_path = args.output.unwrap_or_else(|| {
        let ext = match args.format {
            OutputFormat::Html => "html",
            OutputFormat::Json => "json",
            OutputFormat::Dot => "dot",
        };
        PathBuf::from(format!("{}.{}", module_name, ext))
    });

    // Generate output
    let output = match args.format {
        OutputFormat::Html => html::generate_html(
            &db,
            module_info,
            &compiled,
            &literals,
            &HtmlOptions {
                include_cfg: !args.no_cfg,
                include_dataflow: !args.no_dataflow,
                eval_only: args.eval_only,
                init_only: args.init_only,
                model_param_only: args.model_param_only,
            },
        ),
        OutputFormat::Json => json::generate_json(
            &db,
            module_info,
            &compiled,
            &literals,
            &JsonOptions {
                eval_only: args.eval_only,
                init_only: args.init_only,
                model_param_only: args.model_param_only,
            },
        ),
        OutputFormat::Dot => mir_viz::generate_dot(
            &compiled.eval,
            &mir::ControlFlowGraph::with_function(&compiled.eval),
            &module_name,
        ),
    };

    // Write output
    let mut file = fs::File::create(&output_path)
        .with_context(|| format!("failed to create output file: {:?}", output_path))?;
    file.write_all(output.as_bytes())
        .with_context(|| format!("failed to write to {:?}", output_path))?;

    eprintln!("Output written to: {:?}", output_path);

    Ok(())
}
