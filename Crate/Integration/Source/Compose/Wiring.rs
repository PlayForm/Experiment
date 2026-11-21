//! Wires one invocation across all eleven crates without duplicating any of
//! their logic.
//!
//! Grounded in `Research/prun_laboratory_v6_1/docs/26-acceptance-and-build-plan.md`
//! (34-58, 82-89): the composed surface is a trusted, deterministic director
//! over typed models. An LLM-supplied command is a staged proposal; selection,
//! evidence and release stay with the models called below. Every failure path
//! (missing config, malformed config, absent binary, unknown token) becomes a
//! typed warning instead of a panic.

use crate::cli::command::Command;
use crate::compose::registry::Registry;
use crate::warning::failure::Failure;

/// The typed composition of one invocation: the chain, the degradations and
/// the lines each delegated model produced.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Wiring {
    /// The parsed subcommand.
    pub command: Command,
    /// All eleven crates in their root-manifest order.
    pub chain: [Registry; 11],
    /// Non-fatal degradations collected while composing.
    pub warnings: Vec<Failure>,
    /// Rendered output assembled from delegated results and declared facts.
    pub output: Vec<String>,
}

impl Wiring {
    /// Compose the command surface by calling each owner crate's typed API.
    #[must_use]
    pub fn compose(
        command: &Command,
        option: &crate::cli::option::Option,
        config: std::option::Option<&str>,
    ) -> Self {
        let chain = Registry::ALL;
        let mut warnings: Vec<Failure> = Vec::new();
        let mut output: Vec<String> = Vec::new();

        // Missing or unreadable config degrades; it never aborts the binary.
        match config {
            None => warnings.push(Failure::MissingConfig {
                path: option.controller.config.to_string_lossy().into_owned(),
            }),
            Some(text) => match benchmark::config::Manifest::decode(text) {
                Ok(_) => {}
                Err(_) => warnings.push(Failure::MalformedConfig {
                    path: option.controller.config.to_string_lossy().into_owned(),
                }),
            },
        }

        // A missing SUT argument is an absent-binary warning, not a panic.
        match &option.sut {
            None => warnings.push(Failure::AbsentBinary {
                name: runner::PACKAGE.to_owned(),
            }),
            Some(_) => {}
        }

        let decoded = config.and_then(|text| benchmark::config::Manifest::decode(text).ok());

        match command {
            Command::Unknown { token } => {
                warnings.push(Failure::UnknownCommand {
                    token: token.clone(),
                });
            }
            Command::Init => match benchmark::controller::Init::guard(config.is_some()) {
                Ok(()) => output.push("init: default config is present".to_owned()),
                Err(fault) => warnings.push(Failure::from(fault)),
            },
            Command::Plan => match decoded {
                Some((manifest, notes)) => match crate::dispatch::plan::Plan::of(&manifest) {
                    Ok(plan) => {
                        output.extend(plan.lines);
                        output.push(plan.footer);
                    }
                    Err(failure) => warnings.push(failure),
                },
                None => {}
            },
            Command::Build => output.extend(
                benchmark::controller::Build::STAGES
                    .iter()
                    .map(|stage| format!("build: {stage}")),
            ),
            Command::Bench => match crate::dispatch::bench::Bench::of(&[], &[], option.controller.parallel) {
                crate::dispatch::bench::Bench::Ready { dispatch } => {
                    output.push(format!("bench: {dispatch:?}"));
                }
                crate::dispatch::bench::Bench::Stale { line } => {
                    warnings.push(Failure::Benchmark { line });
                }
            },
            Command::All => output.extend(
                benchmark::controller::All::PHASES
                    .iter()
                    .map(|phase| format!("all: {phase}")),
            ),
            Command::Loop => {
                let decision = crate::dispatch::loop_::Loop::of(
                    experiment::evaluate::verdict::Verdict::Inconclusive,
                );
                output.push(format!(
                    "loop: {}h, promotable={}, negative={}",
                    decision.hours,
                    decision.promotable(),
                    decision.negative()
                ));
            }
        }

        // Declared facts from every crate: typed constants, not re-derived
        // rules. Each line proves the composition reaches that crate's model.
        output.extend([
            format!("benchmark schema {}", benchmark::CONFIG_VERSION),
            format!("sut {}", runner::VERSION),
            format!("campaign hours {}", campaign::WEEK_HOURS),
            format!("fanout {}", experiment::FANOUT),
            format!("trust protocol {}", trust::PROTOCOL_VERSION),
            format!("market schema {}", market::SCHEMA_VERSION),
            format!("planes {}", laboratory::TRUSTED_PLANES),
            format!("events {}", contract::TOTAL_EVENTS),
            format!("migrations {}", storage::CHAIN_ORDER.len()),
            format!("digest {}", custody::DIGEST_LENGTH),
            format!("fabric {}", fabric::PROTOCOL_REVISION),
        ]);

        Self {
            command: command.clone(),
            chain,
            warnings,
            output,
        }
    }
}
