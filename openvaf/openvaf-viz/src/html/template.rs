//! HTML template rendering

use super::assets::{CSS, JS};
use serde_json;

/// Render the complete HTML document
pub fn render_html(
    module_name: &str,
    json_data: &str,
    text_summary: &str,
    _include_cfg: bool,
) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>OpenVAF Viz - {module_name}</title>
    <style>
{css}
    </style>
    <!-- D3.js and dagre for graph visualization -->
    <script src="https://d3js.org/d3.v7.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/dagre@0.8.5/dist/dagre.min.js"></script>
</head>
<body>
    <div class="container">
        <div class="sidebar" id="sidebar">
            <!-- Populated by JavaScript -->
        </div>
        <div class="main-content">
            <div class="header">
                <h1>OpenVAF IR Visualization: {module_name}</h1>
            </div>
            <div class="search-box">
                <input type="text" id="search-input" class="search-input"
                       placeholder="Search instructions (e.g., fadd, v123, block0)...">
            </div>
            <div class="tabs" id="tabs">
                <!-- Populated by JavaScript -->
            </div>
            <div class="panel" id="panel">
                <!-- Populated by JavaScript -->
            </div>
        </div>
    </div>

    <script>
        // Module data embedded as JSON
        window.MODULE_DATA = {json_data};
        window.TEXT_SUMMARY = {text_summary_json};
    </script>
    <script>
{js}
    </script>
</body>
</html>"#,
        module_name = escape_html(module_name),
        css = CSS,
        js = JS,
        json_data = json_data,
        text_summary_json =
            serde_json::to_string(text_summary).unwrap_or_else(|_| "\"\"".to_string()),
    )
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
