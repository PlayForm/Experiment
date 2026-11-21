//! Typed example registry facade.
//!
//! Declares the example descriptor, family and conformance leaves. Each
//! `examples/*.example.json` is paired with the schema stem it conforms to;
//! conformance is a structural name check, never an execution of an example
//! against a validator (`docs/26-acceptance-and-build-plan.md:60-63`).

#[path = "Family.rs"]
pub mod family;
#[path = "Conformance.rs"]
pub mod conformance;
#[path = "Example.rs"]
pub mod example;

pub use conformance::Conformance;
pub use example::Example;
pub use family::Family;
