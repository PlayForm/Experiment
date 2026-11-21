//! Reversible, traceable, cohort-scoped promotion. Promotion is a typed
//! decision, never a git write or an executed change.
//! Grounded in `00-system-constitution.md:49-61` (release is reversible,
//! traceable, cohort-scoped; corrections supersede immutable artifacts) and
//! `06-memory-and-lineage.md:32-44` (rollback is a new authorized transition).

#[path = "Policy.rs"]
pub mod policy;
#[path = "Decision.rs"]
pub mod decision;
#[path = "Supersede.rs"]
pub mod supersede;
#[path = "Rollback.rs"]
pub mod rollback;
#[path = "Cohort.rs"]
pub mod cohort;
#[path = "Embargo.rs"]
pub mod embargo;
