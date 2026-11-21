//! Honestly declared leakage classes that remain after minimization.
//!
//! `docs/03:46-48` notes an artifact digest can reveal content equality;
//! `docs/02:36-39` and `docs/07:22-26` accept residual traffic correlation;
//! `docs/07:18-20` requires that disclosing a private mapping be logged and
//! policy checked. Nothing here measures a real channel.

/// One residual information-leakage class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Leakage {
    /// Equal digests reveal content equality within a controlled namespace.
    ContentEquality,
    /// A stable opaque token can link activities across views.
    Correlation,
    /// Container starts, CPU, disk I/O and routing expose activity.
    SideChannel,
    /// A private mapping was disclosed to an authorized viewer.
    MappingDisclosure,
    /// No residual channel was identified for this projection.
    Absent,
}

impl Leakage {
    /// Whether the class must be disclosed in an assurance finding.
    #[must_use]
    pub const fn acknowledged(&self) -> bool {
        !matches!(self, Self::Absent)
    }

    /// No configuration of these techniques yields absolute secrecy against
    /// a privileged host owner (`docs/07:26`, `docs/09:8-9`).
    #[must_use]
    pub const fn absolute_secrecy() -> Verdict {
        Verdict::Deny
    }

    /// Stable audit label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::ContentEquality => "content-equality",
            Self::Correlation => "correlation",
            Self::SideChannel => "side-channel",
            Self::MappingDisclosure => "mapping-disclosure",
            Self::Absent => "absent",
        }
    }
}
