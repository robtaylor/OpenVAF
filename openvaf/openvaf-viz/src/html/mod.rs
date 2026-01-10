//! HTML generation for interactive visualization

mod assets;
mod template;

use hir::CompilationDB;
use lasso::Rodeo;
use sim_back::{CompiledModule, ModuleInfo};

/// Options for HTML output generation
pub struct HtmlOptions {
    pub include_cfg: bool,
    pub include_dataflow: bool,
    pub eval_only: bool,
    pub init_only: bool,
    pub model_param_only: bool,
}

/// Generate an interactive HTML visualization
pub fn generate_html(
    db: &CompilationDB,
    module_info: &ModuleInfo,
    compiled: &CompiledModule,
    literals: &Rodeo,
    options: &HtmlOptions,
) -> String {
    // Generate JSON data for the visualization
    let json_options = crate::json::JsonOptions {
        eval_only: options.eval_only,
        init_only: options.init_only,
        model_param_only: options.model_param_only,
    };
    let json_data = crate::json::generate_json(db, module_info, compiled, literals, &json_options);

    // Generate text summary
    let text_summary =
        crate::compiled_viz::generate_text_summary(db, module_info, compiled, literals);

    template::render_html(
        &module_info.module.name(db),
        &json_data,
        &text_summary,
        options.include_cfg,
    )
}
