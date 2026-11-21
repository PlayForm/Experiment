//! The single model-visible input manifest at `/workspace/task.json`.
//!
//! Mirrors `schemas/v61-offline-task.schema.json` and
//! `examples/v61-offline-task.example.json`: no endpoint, no internal ID, no
//! credential, no embargo information can appear in this value.

use super::schema::Schema;
use crate::warning::Warning;
use crate::worker::local_tool::LocalTool;

/// The complete worker-visible input value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input {
    /// `local_nonce`, 1-64 URL-safe characters.
    pub local_nonce: String,
    /// `problem`, the bounded task text.
    pub problem: String,
    /// `repository_path`, pinned to `/workspace/repo`.
    pub repository_path: String,
    /// `references_path`, pinned to `/workspace/references`.
    pub references_path: String,
    /// `feedback_path`, present only for this track's own feedback.
    pub feedback_path: Option<String>,
    /// `allowed_tools` from the pinned six-value enum.
    pub allowed_tools: Vec<LocalTool>,
    /// `output_directory`, pinned to `/workspace/out`.
    pub output_directory: String,
    /// `expected_artifacts` from the pinned four-value enum.
    pub expected_artifacts: Vec<String>,
}

impl Input {
    /// Construct an input with all paths pinned by trusted software.
    #[must_use]
    pub fn of(nonce: &str, problem: &str, tools: Vec<LocalTool>, artifacts: Vec<String>) -> Self {
        Self {
            local_nonce: nonce.to_owned(),
            problem: problem.to_owned(),
            repository_path: crate::REPOSITORY_PATH.to_owned(),
            references_path: crate::REFERENCES_PATH.to_owned(),
            feedback_path: None,
            allowed_tools: tools,
            output_directory: crate::OUTPUT_DIRECTORY.to_owned(),
            expected_artifacts: artifacts,
        }
    }

    /// Attach the optional own-feedback path; absence reveals no policy reason.
    #[must_use]
    pub fn feedback(mut self) -> Self {
        self.feedback_path = Some(crate::FEEDBACK_PATH.to_owned());
        self
    }

    /// Whether the value satisfies the pinned input schema structurally.
    #[must_use]
    pub fn validate(&self) -> Vec<Warning> {
        let nonce_ok = !self.local_nonce.is_empty()
            && self.local_nonce.len() <= 64
            && self.local_nonce.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
            });
        let problem_ok = !self.problem.is_empty() && self.problem.len() <= 12_000;
        let paths_ok = self.repository_path == crate::REPOSITORY_PATH
            && self.references_path == crate::REFERENCES_PATH
            && self.output_directory == crate::OUTPUT_DIRECTORY
            && self
                .feedback_path
                .as_deref()
                .is_none_or(|path| path == crate::FEEDBACK_PATH);
        let artifacts_ok = self.expected_artifacts.iter().all(|name| {
            matches!(
                name.as_str(),
                "change.patch" | "claims.json" | "proposed-tests" | "paper-draft.md"
            )
        });
        let known = self
            .allowed_tools
            .iter()
            .all(|tool| LocalTool::parse(tool.label()) == Some(*tool));
        let findings = [
            (
                "offline task input",
                nonce_ok && problem_ok && paths_ok && artifacts_ok,
            ),
            ("allowed_tools", known),
        ]
        .into_iter()
        .filter_map(|(domain, valid)| {
            (!valid).then(|| Warning::Malformed {
                domain,
                detail: "a field is outside the pinned schema".to_owned(),
            })
        })
        .collect();
        findings
    }

    /// The inbound schema that validates this value.
    #[must_use]
    pub fn schema(&self) -> Schema {
        Schema::OfflineTask
    }
}
