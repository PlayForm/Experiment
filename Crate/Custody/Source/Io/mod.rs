//! The two and only two worker-visible JSON schemas facade.
//!
//! No model-to-model envelope exists. Everything else is an ordinary local
//! file whose absence reveals no embargo or internal policy reason
//! (`docs/28:1-38`).

#[path = "Input.rs"]
pub mod input;
#[path = "Output.rs"]
pub mod output;
#[path = "Schema.rs"]
pub mod schema;
#[path = "File.rs"]
pub mod file;
