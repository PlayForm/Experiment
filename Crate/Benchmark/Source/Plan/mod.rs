//! Deterministic matrix expansion and variant planning.
//!
//! Grounded in `README.md:8-9,84-103`, `prun_lab.py::matrix` (220-241) and the
//! runner driver's workload loop (`prun_lab.py:122-124`). The controller
//! expands refs x builds x feature sets, then crosses each variant with the
//! workload axis at benchmark time; both views are typed here.

#[path = "Matrix.rs"]
pub mod matrix;
#[path = "Variant.rs"]
pub mod variant;
#[path = "Axis.rs"]
pub mod axis;
#[path = "Candidate.rs"]
pub mod candidate;
#[path = "Skip.rs"]
pub mod skip;

pub use axis::Axis;
pub use candidate::Candidate;
pub use matrix::Matrix;
pub use skip::Skip;
pub use variant::Variant;
