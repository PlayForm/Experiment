//! The `build` subcommand.
//!
//! Grounded in `prun_lab.py:370-461`: snapshot, lock capture, recipe, builder
//! image and artifact, then runner setup and the build manifest. Only builds
//! are parallelised by `--jobs`.

use super::Jobs;

/// A request to build every planned variant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Build {
    /// Builder concurrency.
    pub jobs: Jobs,
}

impl Build {
    /// Ordered model stages of one variant build (prun_lab.py:370-407).
    pub const STAGES: [&'static str; 5] = [
        "snapshot",
        "lock",
        "recipe",
        "image",
        "artifact",
    ];
}
