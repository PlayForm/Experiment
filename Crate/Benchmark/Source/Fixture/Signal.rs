//! The `bench-signal` helper, modelled as the command it installs.
//!
//! Grounded in `prun_lab.py:102` (the runner Dockerfile writes a shell script
//! that touches `/checks/$(basename "$PWD")`) and `RUNNER_DRIVER:114-126`
//! (the `/checks` tmpfs is cleared before each validation run).

/// A correctness signal installed inside the runner, not executed here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signal {
    /// Directory into which each matched directory drops its marker.
    pub checks: String,
    /// Shell body of the installed helper.
    pub body: String,
}

impl Signal {
    /// The shared checks directory declared as a tmpfs (prun_lab.py:579).
    pub const DIRECTORY: &'static str = "/checks";

    /// The canonical helper installed in both runner images (prun_lab.py:102).
    #[must_use]
    pub fn canonical() -> Self {
        Self {
            checks: Self::DIRECTORY.to_owned(),
            body: "#!/bin/sh\nset -eu\ntouch \"/checks/$(basename \"$PWD\")\"\n".to_owned(),
        }
    }
}
