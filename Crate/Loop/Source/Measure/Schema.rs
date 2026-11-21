//! The measurement protocol schema, sourced from the v0 controller.
//! Grounded in `RESEARCH_GATE.md:53-68` (measurement protocol digest is part of
//! the experiment key) and `AGENT_CONTRACT.md:63-75` (predeclare thresholds
//! and methods, never tune stop rules after seeing data).

/// The immutable protocol descriptor attached to raw samples.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schema {
    /// v0 configuration schema version.
    pub config_version: u32,
    /// Pinned Hyperfine version.
    pub hyperfine: String,
    /// Predeclared protocol digest.
    pub protocol: String,
}

impl Schema {
    /// Build a schema from the pinned v0 versions.
    #[must_use]
    pub fn of(protocol: &str) -> Self {
        Self {
            config_version: benchmark::CONFIG_VERSION,
            hyperfine: benchmark::HYPERFINE_VERSION.to_owned(),
            protocol: protocol.to_owned(),
        }
    }

    /// Whether the schema matches the supported configuration version.
    #[must_use]
    pub const fn is_current(&self) -> bool {
        self.config_version == benchmark::CONFIG_VERSION
    }

    /// Whether a protocol digest has been declared.
    #[must_use]
    pub fn is_declared(&self) -> bool {
        !self.protocol.is_empty()
    }
}
