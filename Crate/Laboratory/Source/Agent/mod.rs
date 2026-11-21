//! Agent contract facade.
//!
//! Declares only the four required leaves. LLM output is a staged untrusted
//! artifact; trusted selection, evidence, scheduling, promotion and release
//! remain deterministic software (`docs/17-v61-constitution.md:8-11`).

#[path = "Contract.rs"]
pub mod contract;
#[path = "Proposal.rs"]
pub mod proposal;
#[path = "Artifact.rs"]
pub mod artifact;
#[path = "Authorization.rs"]
pub mod authorization;

pub use artifact::Artifact;
pub use authorization::Authorization;
pub use contract::Contract;
pub use proposal::Proposal;
