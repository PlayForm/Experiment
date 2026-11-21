//! Cedar-like authorization kept separate from schema compilation.
//!
//! Mirrors `docs/05:34-37`, the capability formula in `docs/04:31-44`, the
//! cross-field checks of `docs/09:68-78`, and the `proof-bound-grant` schema.
//! The policy engine holds semantic entitlement IDs only (`docs/08:9`), emits
//! a decision and a digest, and never releases a key or returns a denial
//! reason to an untrusted worker (`docs/07:28-30`).

#[path = "Rule.rs"]
pub mod rule;
#[path = "Registry.rs"]
pub mod registry;
#[path = "Decision.rs"]
pub mod decision;
#[path = "Cedar.rs"]
pub mod cedar;
