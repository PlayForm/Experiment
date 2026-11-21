//! The typed subcommand enum and its dispatch.
//!
//! Grounded in `README.md:45-78` and `prun_lab.py:661-689`: `init`, `plan`,
//! `build`, `bench`, `all` and `report`, with global options applied to each.
//! `report` is kept as a variant because the controller defines it at
//! `prun_lab.py:664-665,675-677`.

use std::path::PathBuf;

use super::{All, Bench, Build, Dispatch, Init, Option, Plan};

/// One parsed controller invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Init { global: Option, action: Init },
    Plan { global: Option, action: Plan },
    Build { global: Option, action: Build },
    Bench { global: Option, action: Bench },
    All { global: Option, action: All },
    Report { global: Option, session: PathBuf },
}

impl Command {
    /// Reduce the command to a model-level next step without executing it.
    #[must_use]
    pub fn dispatch(&self) -> Dispatch {
        match self {
            Self::Init { action, .. } => Dispatch::WriteDefaultConfig {
                path: action.config.clone(),
            },
            Self::Plan { .. } => Dispatch::PrintPlan,
            Self::Build { action, .. } => Dispatch::Build {
                jobs: action.jobs,
            },
            Self::Bench { action, .. } => Dispatch::Bench {
                parallel: action.parallel,
            },
            Self::All { action, .. } => Dispatch::BuildThenBench {
                jobs: action.build.jobs,
                parallel: action.bench.parallel,
            },
            Self::Report { session, .. } => Dispatch::Report {
                session: session.clone(),
            },
        }
    }

    /// The global options attached to this command.
    #[must_use]
    pub const fn global(&self) -> &Option {
        match self {
            Self::Init { global, .. }
            | Self::Plan { global, .. }
            | Self::Build { global, .. }
            | Self::Bench { global, .. }
            | Self::All { global, .. }
            | Self::Report { global, .. } => global,
        }
    }
}
