//! User-facing command modules. Each leaf is one typed value; every leaf is
//! declared here so no source file is orphaned on the case-sensitive volume.

#[path = "Command.rs"]
pub mod command;
#[path = "Option.rs"]
pub mod option;
