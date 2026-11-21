//! The twelve access-matrix roles.
//!
//! `policy/access-matrix.json:46-182` declares observer, operator, coordinator,
//! supervisor, llm-worker, builder, oracle, runner, evaluator, publisher, pki
//! and telemetry. The model role has no network, no control API and no role in
//! any authoritative transition (`policy/v61-roles.json:4-12`).

/// An access-matrix role.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Role {
    Observer,
    Operator,
    Coordinator,
    Supervisor,
    Worker,
    Builder,
    Oracle,
    Runner,
    Evaluator,
    Publisher,
    PKI,
    Telemetry,
}

impl Role {
    /// Wire spelling from the access matrix.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observer => "observer",
            Self::Operator => "operator",
            Self::Coordinator => "coordinator",
            Self::Supervisor => "supervisor",
            Self::Worker => "llm-worker",
            Self::Builder => "builder",
            Self::Oracle => "oracle",
            Self::Runner => "runner",
            Self::Evaluator => "evaluator",
            Self::Publisher => "publisher",
            Self::PKI => "pki",
            Self::Telemetry => "telemetry",
        }
    }

    /// Decode a wire role name.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "observer" => Some(Self::Observer),
            "operator" => Some(Self::Operator),
            "coordinator" => Some(Self::Coordinator),
            "supervisor" => Some(Self::Supervisor),
            "llm-worker" => Some(Self::Worker),
            "builder" => Some(Self::Builder),
            "oracle" => Some(Self::Oracle),
            "runner" => Some(Self::Runner),
            "evaluator" => Some(Self::Evaluator),
            "publisher" => Some(Self::Publisher),
            "pki" => Some(Self::PKI),
            "telemetry" => Some(Self::Telemetry),
            _ => None,
        }
    }

    /// Whether this role is the untrusted model.
    #[must_use]
    pub const fn is_model(self) -> bool {
        match self {
            Self::Worker => true,
            _ => false,
        }
    }
}
