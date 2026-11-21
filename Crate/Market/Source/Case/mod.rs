//! Pre-registered research cases, one per atomic mutation.
//!
//! Grounded in `SYSTEM.md:7-11,104-116` (one atomic mutation receives one
//! pre-registered case, may branch into independent opinion streams, and never
//! edits a prior claim in place), `schemas/research-case.v5.schema.json:6-152`,
//! `examples/research-case.v5.json:2-25` and `sql/v5_migration.sql:7-20`. A case
//! is registered data with validation; nothing is opened, assigned or published.

#[path = "ResearchCase.rs"]
pub mod research_case;
#[path = "Falsifier.rs"]
pub mod falsifier;
#[path = "Hypothesis.rs"]
pub mod hypothesis;
#[path = "Prediction.rs"]
pub mod prediction;
#[path = "Question.rs"]
pub mod question;
#[path = "State.rs"]
pub mod state;
#[path = "Stance.rs"]
pub mod stance;
#[path = "Privacy.rs"]
pub mod privacy;
#[path = "Holdout.rs"]
pub mod holdout;

pub use holdout::Holdout;
pub use falsifier::Falsifier;
pub use hypothesis::Hypothesis;
pub use prediction::Prediction;
pub use privacy::Privacy;
pub use question::Question;
pub use research_case::ResearchCase;
pub use state::State;
pub use stance::Stance;
