//! Deterministic fixture specification and the `bench-signal` count contract.
//!
//! Grounded in `README.md:17-21,127-131`, `prun-bench.json:50-63`,
//! `prun_lab.py::FIXTURE_DRIVER` (147-161) and `RUNNER_DRIVER` (114-136).
//! Fixtures are described as data only; no files or volumes are created here.

#[path = "Fixture.rs"]
pub mod fixture;
#[path = "Signal.rs"]
pub mod signal;
#[path = "Preflight.rs"]
pub mod preflight;
#[path = "Volume.rs"]
pub mod volume;

pub use fixture::Fixture;
pub use preflight::Preflight;
pub use signal::Signal;
pub use volume::Volume;
