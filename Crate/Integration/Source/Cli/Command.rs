//! The six user-facing commands of the composed `prun` binary.
//!
//! Grounded in `Research/prun_lab/README.md` Start (45-62): `init`, `plan`,
//! `all`, `bench`, `build`; and `Research/prun_laboratory_v6_1/docs/
//! 26-acceptance-and-build-plan.md` Recommended first vertical slice (34-58)
//! for the autonomous `loop`. `Unknown` is the typed degradation for a token
//! outside that vocabulary; it never aborts the process.

/// One parsed subcommand of the composed surface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Init,
    Plan,
    Build,
    Bench,
    All,
    Loop,
    Unknown { token: String },
}

impl Command {
    /// Parse the first non-flag token, degrading to `Unknown` rather than
    /// panicking on an empty or unrecognised argument vector.
    #[must_use]
    pub fn parse(words: &[String]) -> Self {
        words
            .iter()
            .find(|word| !word.starts_with('-'))
            .map_or(Self::Unknown { token: String::new() }, |token| {
                match token.as_str() {
                    "init" => Self::Init,
                    "plan" => Self::Plan,
                    "build" => Self::Build,
                    "bench" => Self::Bench,
                    "all" => Self::All,
                    "loop" => Self::Loop,
                    other => Self::Unknown {
                        token: other.to_owned(),
                    },
                }
            })
    }

    /// The Benchmark controller command this variant delegates to. `Loop` and
    /// `Unknown` have no v0 controller counterpart and stay unresolved here.
    #[must_use]
    pub fn controller(
        &self,
        option: &crate::cli::option::Option,
    ) -> std::option::Option<benchmark::controller::Command> {
        let jobs = option.controller.jobs;
        let parallel = option.controller.parallel;
        let global = option.controller.clone();
        let config = global.config.clone();
        match self {
            Self::Init => std::option::Option::Some(benchmark::controller::Command::Init {
                global,
                action: benchmark::controller::Init { config },
            }),
            Self::Plan => std::option::Option::Some(benchmark::controller::Command::Plan {
                global,
                action: benchmark::controller::Plan,
            }),
            Self::Build => std::option::Option::Some(benchmark::controller::Command::Build {
                global,
                action: benchmark::controller::Build { jobs },
            }),
            Self::Bench => std::option::Option::Some(benchmark::controller::Command::Bench {
                global,
                action: benchmark::controller::Bench { parallel },
            }),
            Self::All => std::option::Option::Some(benchmark::controller::Command::All {
                global,
                action: benchmark::controller::All {
                    build: benchmark::controller::Build { jobs },
                    bench: benchmark::controller::Bench { parallel },
                },
            }),
            Self::Loop | Self::Unknown { .. } => std::option::Option::None,
        }
    }
}
