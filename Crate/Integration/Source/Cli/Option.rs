//! Global options for the composed binary, assembled from the typed models the
//! owner crates already define.
//!
//! Grounded in `Research/prun_lab/README.md` (60-62) for `--repo`, `--config`,
//! `--out`, `--jobs`, `--parallel-runners`, and `Module/Run/README.md` Usage
//! (104-122) for the SUT flags `-F/-P/-R/-E/-C`. The controller option object
//! is reused verbatim from `Crate/Benchmark`; the SUT argument is reused from
//! `Crate/Runner`. No flag semantics are reimplemented here.

/// Global controller options plus the resolved SUT contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Option {
    /// The v0 controller options (`Crate/Benchmark/Source/Controller/Option.rs`).
    pub controller: benchmark::controller::Option,
    /// The system-under-test argument set (`Crate/Runner/Source/Cli/Argument.rs`).
    /// Absent when a required SUT value could not be built; a caller logs a
    /// typed `AbsentBinary` warning instead of panicking.
    pub sut: std::option::Option<runner::cli::argument::Argument>,
}

impl Option {
    /// Defaults grounded in `prun_lab.py:648-660` and `README.md:112-122`.
    /// The SUT pieces are built through their validating constructors; an
    /// `Err` is captured as absence, never unwrapped.
    #[must_use]
    pub fn baseline() -> Self {
        let command = runner::cli::command::Command::build(vec!["/bin/true".to_owned()]).ok();
        let pattern = runner::cli::pattern::Pattern::build("bench.marker".to_owned()).ok();
        let sut = command.ok().and_then(|command| {
            pattern.ok().map(|pattern| {
                runner::cli::argument::Argument::from_defaults(command, pattern, '/')
            })
        });
        Self {
            controller: benchmark::controller::Option::baseline(),
            sut,
        }
    }
}
