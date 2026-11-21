//! Research curriculum as deterministic software facade.
//!
//! Lesson assignment, prerequisites, tests, grades, repetition, resource
//! allocation and release decisions are software; the model is never asked
//! which lesson should run next and self-reported scores never count
//! (`docs/27:1-40`).

#[path = "Challenge.rs"]
pub mod challenge;
#[path = "Assignment.rs"]
pub mod assignment;
#[path = "Outcome.rs"]
pub mod outcome;
#[path = "Judge.rs"]
pub mod judge;
