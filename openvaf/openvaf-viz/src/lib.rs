//! OpenVAF IR Visualization Library
//!
//! Provides visualization capabilities for OpenVAF's intermediate representations.

pub mod compiled_viz;
pub mod html;
pub mod json;
pub mod mir_viz;

// Re-export types from submodules
pub use html::HtmlOptions;
pub use json::JsonOptions;
