//! Schema family facade.
//!
//! Declares the descriptor, reference, draft, family and field leaves plus the
//! `Definition` catalog. Only structural checks are modelled; no JSON Schema
//! engine dependency is introduced beyond the `serde` already in the workspace
//! (`docs/26-acceptance-and-build-plan.md:60-63`).

#[path = "Draft.rs"]
pub mod draft;
#[path = "Family.rs"]
pub mod family;
#[path = "Reference.rs"]
pub mod reference;
#[path = "Constraint.rs"]
pub mod constraint;
#[path = "Field.rs"]
pub mod field;
#[path = "Definition.rs"]
pub mod definition;

pub use constraint::Constraint;
pub use definition::Definition;
pub use draft::Draft;
pub use family::Family;
pub use field::Field;
pub use reference::Reference;
