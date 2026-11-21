//! Container responsibilities and pinning policy.
//!
//! Grounded in `campaign.v2.json:46-54`, the schema at
//! `schemas/campaign.v2.schema.json:230-271` and `REQUIREMENTS.md:627-671`
//! (builder, runner-release, runner-diagnostic, runner-training, agent-worker
//! and coordinator; REQ-DOC-01..10). A container here is a typed image name
//! and mount policy, never a running container.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// How a binary or fixture is mounted into the runner
/// (`campaign.v2.json:52-53`, `schemas/campaign.v2.schema.json:252-262`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mount {
    /// The only legal binary mount mode.
    ReadOnly,
}

/// Fixture mount policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FixtureMount {
    /// Read-only except declared disposable copies (`REQ-DOC-03`).
    #[serde(rename = "read_only_by_default")]
    ReadOnlyDefault,
    /// Writable copies are disposable per trial (`REQ-WLD-01`).
    #[serde(rename = "disposable_writable_copies")]
    Disposable,
}

/// The container images and mount policy of a campaign.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub builder: String,
    #[serde(rename = "runner_release")]
    pub runner_release: String,
    #[serde(rename = "runner_diagnostic")]
    pub runner_diagnostic: String,
    #[serde(rename = "training_runner", default)]
    pub training_runner: String,
    #[serde(rename = "pin_image_digests_for_campaign", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub pin_digests: Permission,
    #[serde(rename = "binary_mount_mode", default)]
    pub binary_mount: Mount,
    #[serde(rename = "fixture_mount_mode", default)]
    pub fixture_mount: FixtureMount,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            builder: "rust:slim-bookworm".to_owned(),
            runner_release: "debian:bookworm-slim".to_owned(),
            runner_diagnostic: "debian:bookworm-slim".to_owned(),
            training_runner: "debian:bookworm-slim".to_owned(),
            pin_digests: Permission::Allow,
            binary_mount: Mount::ReadOnly,
            fixture_mount: FixtureMount::ReadOnlyDefault,
        }
    }
}

impl Container {
    /// Recognised keys (`schemas/campaign.v2.schema.json:264-269`).
    pub const KEYS: [&'static str; 7] = [
        "builder",
        "runner_release",
        "runner_diagnostic",
        "training_runner",
        "pin_image_digests_for_campaign",
        "binary_mount_mode",
        "fixture_mount_mode",
    ];

    /// `latest` and implicit network upgrades are forbidden during a campaign
    /// (`REQUIREMENTS.md:648-651`). The model reports the state, never pulls.
    #[must_use]
    pub fn digests_are_pinned(&self) -> Permission {
        self.pin_digests
    }
}
