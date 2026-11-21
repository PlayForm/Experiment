//! Opaque, scope-specific identifiers and fail-closed admission.
//!
//! Typed constraints come from `docs/03:8-18` and the byte-level reference in
//! `rust/lab-identifier/src/lib.rs`; namespace families from `docs/03:39-61`;
//! the collision and semantic-collision rule from `docs/01:60-67` and
//! `README.md:25-26`. Nothing here resolves a path or grants authority.

#[path = "Kind.rs"]
pub mod kind;
#[path = "Id.rs"]
pub mod id;
#[path = "Noncanonical.rs"]
pub mod noncanonical;
#[path = "Scope.rs"]
pub mod scope;
#[path = "Factory.rs"]
pub mod factory;
#[path = "Pseudonym.rs"]
pub mod pseudonym;
#[path = "Collision.rs"]
pub mod collision;
#[path = "Registry.rs"]
pub mod registry;
