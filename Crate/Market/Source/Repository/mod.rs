//! The repository-adapter contract for arbitrary source repositories.
//!
//! Grounded in `architecture/REPOSITORIES.md:1-95` (versioned declarative
//! manifest, capability manifest, hermetic build recipes, oracle contract,
//! deterministic workloads, metric direction, feature switches, privacy/reuse
//! policy, onboarding sequence and model-provider abstraction),
//! `schemas/repository-adapter.v5.schema.json:6-163`,
//! `examples/repository-adapter.v5.json:2-29` and `OPERATIONS.md:38`
//! (`unknown_repo_adapter`). An adapter here is validated data; no repository
//! is cloned, no command is run and no artifact is extracted.

#[path = "Adapter.rs"]
pub mod adapter;
#[path = "Language.rs"]
pub mod language;
#[path = "Runner.rs"]
pub mod runner;
#[path = "Artifact.rs"]
pub mod artifact;
#[path = "Consent.rs"]
pub mod consent;
#[path = "Visibility.rs"]
pub mod visibility;
#[path = "License.rs"]
pub mod license;
#[path = "Network.rs"]
pub mod network;
#[path = "Release.rs"]
pub mod release;
#[path = "Toolchain.rs"]
pub mod toolchain;
#[path = "Commands.rs"]
pub mod commands;
#[path = "Outputs.rs"]
pub mod outputs;
#[path = "Stage.rs"]
pub mod stage;
#[path = "Direction.rs"]
pub mod direction;

pub use adapter::Adapter;
pub use artifact::Artifact;
pub use commands::Commands;
pub use consent::Consent;
pub use direction::Direction;
pub use language::Language;
pub use license::License;
pub use network::Network;
pub use outputs::Outputs;
pub use release::Release;
pub use runner::Runner;
pub use stage::Stage;
pub use toolchain::Toolchain;
pub use visibility::Visibility;
