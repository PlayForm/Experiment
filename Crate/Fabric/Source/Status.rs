//! Honest evidence ladder for the assurance plan.
//!
//! The package demonstrates design review plus schema-level validation only.
//! Cryptographic, deployment and audit levels are named but not attained
//! (`docs/09:52-66`, `README.md:3-6`, `VALIDATION_REPORT.md:14-20`).

/// One level of the verification ladder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Evidence {
    /// Standards and contract relationships assessed; limits documented.
    DesignReviewed,
    /// Schema examples, required-field and negative cases pass.
    SchemaValidated,
    /// Rust unit/property/fuzz tests and static review passed.
    CodeVerified,
    /// Real egress, mount, TLS and host-threat tests passed.
    DeploymentVerified,
    /// Independent review, penetration and supply-chain checks completed.
    SecurityAudited,
}

impl Evidence {
    /// The two levels this package actually demonstrates.
    #[must_use]
    pub const fn attained(&self) -> crate::verdict::Verdict {
        match self {
            Self::DesignReviewed | Self::SchemaValidated => crate::verdict::Verdict::Allow,
            Self::CodeVerified | Self::DeploymentVerified | Self::SecurityAudited => {
                crate::verdict::Verdict::Deny
            }
        }
    }

    /// The stable name used in the validation report and README.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::DesignReviewed => "design-verified",
            Self::SchemaValidated => "schema-verified",
            Self::CodeVerified => "code-verified",
            Self::DeploymentVerified => "deployment-verified",
            Self::SecurityAudited => "security-audited",
        }
    }

    /// Whether a claim at this level may be presented to a third party.
    #[must_use]
    pub const fn claimable(&self) -> bool {
        matches!(self, Self::DesignReviewed | Self::SchemaValidated)
    }
}
