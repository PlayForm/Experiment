//! Repository-agnostic rule facade.
//!
//! Declares only the three required leaves. The laboratory applies to any
//! consenting repository through a signed adapter; transfer evidence is not
//! portable speedup (`docs/13-repository-agnostic.md`).

#[path = "Agnostic.rs"]
pub mod agnostic;
#[path = "Adapter.rs"]
pub mod adapter;
#[path = "Consent.rs"]
pub mod consent;

pub use adapter::Adapter;
pub use agnostic::Agnostic;
pub use consent::Consent;
