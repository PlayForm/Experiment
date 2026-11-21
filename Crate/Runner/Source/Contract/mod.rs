//! The characterization contract the benchmark must preserve.
//!
//! Grounded in the SUT source files named on each `Invariant` and in the
//! card's five axes: match set, exclusion semantics, output/ordering, error
//! propagation and file mode. This is typed evidence vocabulary: it does not
//! measure, assert or grant any generated artifact authority.

#[path = "Behavior.rs"]
pub mod behavior;
#[path = "Invariant.rs"]
pub mod invariant;
#[path = "Observation.rs"]
pub mod observation;
