//! Private semantic IR, deterministic compilation and a separate public spec.
//!
//! Grounded in `docs/05:1-62`, `docs/01:16-36`, `docs/10:2`, the eight schemas
//! and their examples, and `README.md:56-69`. The compiler emits deterministic
//! tables and opaque handles; it never randomizes executable field layout and
//! never lets a generated artifact change authorization semantics.

#[path = "Ir.rs"]
pub mod ir;
#[path = "Definition.rs"]
pub mod definition;
#[path = "Compiler.rs"]
pub mod compiler;
#[path = "Resolver.rs"]
pub mod resolver;
#[path = "Version.rs"]
pub mod version;
#[path = "Private.rs"]
pub mod private;
#[path = "Public.rs"]
pub mod public;
