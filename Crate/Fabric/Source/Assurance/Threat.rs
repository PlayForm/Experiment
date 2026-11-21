//! Seven threat assumptions T1 through T7.
//!
//! Mirrors `docs/09:5-14`: compromised worker, compromised online service,
//! curious dashboard operator, privileged host owner, lost backups, malicious
//! dependency or build script, and an attacker who sees the protocol source.
//! Authorization must still hold when the design is known.

/// One threat assumption in the assurance plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Threat {
    /// T1: compromised LLM process or generated Rust code.
    CompromisedWorker,
    /// T2: compromised trusted online microservice.
    CompromisedService,
    /// T3: curious operator of an ordinary dashboard.
    CuriousOperator,
    /// T4: privileged host owner; Docker and RAM secrecy are insufficient.
    PrivilegedHost,
    /// T5: lost disk or backups.
    LostBackup,
    /// T6: malicious dependency or build script.
    MaliciousDependency,
    /// T7: attacker who sees protocol source.
    SourceVisible,
}

impl Threat {
    /// Stable plan label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::CompromisedWorker => "T1",
            Self::CompromisedService => "T2",
            Self::CuriousOperator => "T3",
            Self::PrivilegedHost => "T4",
            Self::LostBackup => "T5",
            Self::MaliciousDependency => "T6",
            Self::SourceVisible => "T7",
        }
    }

    /// Only the privileged-host threat needs separate hardware or a TEE under
    /// an explicitly narrower model (`docs/09:8-9`, `docs/04:58-60`).
    #[must_use]
    pub const fn needs_hardware_domain(&self) -> bool {
        matches!(self, Self::PrivilegedHost)
    }

    /// All seven assumptions are declared in the plan.
    #[must_use]
    pub const fn all() -> [Self; 7] {
        [
            Self::CompromisedWorker,
            Self::CompromisedService,
            Self::CuriousOperator,
            Self::PrivilegedHost,
            Self::LostBackup,
            Self::MaliciousDependency,
            Self::SourceVisible,
        ]
    }
}
