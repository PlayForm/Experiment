//! Evidence hierarchy facade.
//!
//! Declares only the five leaves. The ordered rungs come from
//! `docs/00-system-constitution.md:49-55`; failure and inconclusive must survive
//! resolution (`docs/10-benchmark-mesh.md:56-62`).

#[path = "Rank.rs"]
pub mod rank;
#[path = "Observation.rs"]
pub mod observation;
#[path = "Stamp.rs"]
pub mod stamp;
#[path = "Verdict.rs"]
pub mod verdict;
#[path = "Hierarchy.rs"]
pub mod hierarchy;

pub use hierarchy::Hierarchy;
pub use observation::Observation;
pub use rank::Rank;
pub use stamp::Stamp;
pub use verdict::Verdict;
