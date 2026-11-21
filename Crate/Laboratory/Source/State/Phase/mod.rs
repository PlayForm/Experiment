//! Aggregate state vocabulary facade.
//!
//! Declares only the five lifecycle leaves drawn from the transition tables in
//! `docs/06-state-machines.md`. No behaviour lives here.

#[path = "CampaignPhase.rs"]
pub mod campaign_phase;
#[path = "MutationPhase.rs"]
pub mod mutation_phase;
#[path = "WorkerPhase.rs"]
pub mod worker_phase;
#[path = "BenchmarkPhase.rs"]
pub mod benchmark_phase;
#[path = "PublicationPhase.rs"]
pub mod publication_phase;

pub use benchmark_phase::BenchmarkPhase;
pub use campaign_phase::CampaignPhase;
pub use mutation_phase::MutationPhase;
pub use publication_phase::PublicationPhase;
pub use worker_phase::WorkerPhase;
