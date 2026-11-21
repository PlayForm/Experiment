//! Typed loop faults. Every failure degrades to a warning or a typed fault;
//! none panics and none becomes a fabricated measurement or policy decision.

/// Non-panicking fault classes for the loop crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fault {
    /// A required trusted source snapshot is absent or unreadable.
    MissingSnapshot { path: String },
    /// A config or capsule document could not be parsed.
    Malformed { domain: String, field: String },
    /// A trusted service (oracle, runner, queue) is unreachable.
    Unreachable { service: String },
    /// Two canonical identities collide unexpectedly.
    Collision { key: String },
    /// The playbook budget cannot satisfy a request.
    Budget { reason: String },
}

impl Fault {
    /// Operator-only diagnostic line; never worker-visible.
    #[must_use]
    pub fn diagnostic(&self) -> String {
        match self {
            Self::MissingSnapshot { path } => format!("missing trusted snapshot: {path}"),
            Self::Malformed { domain, field } => format!("malformed {domain}.{field}"),
            Self::Unreachable { service } => format!("trusted service unreachable: {service}"),
            Self::Collision { key } => format!("identity collision: {key}"),
            Self::Budget { reason } => format!("budget refused: {reason}"),
        }
    }
}
