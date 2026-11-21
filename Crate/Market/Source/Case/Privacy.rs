//! Privacy class of a research case.
//!
//! Grounded in `schemas/research-case.v5.schema.json:98-100`,
//! `sql/v5_migration.sql:16` and `README.md:53-56`: default contexts stay
//! private; sensitive material is decrypted only by authorized consumers; a
//! sanitized publisher is an explicit declassification boundary.

use crate::Warning;

/// Visibility class of a case and its derived artifacts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Privacy {
    /// Private to the named researchers (`schema:99`).
    Private,
    /// Scoped to one tenant (`schema:99`).
    Tenant,
    /// Eligible for a signed public release (`schema:99`).
    Public,
}

impl Privacy {
    /// All privacy classes.
    pub const ALL: [Self; 3] = [Self::Private, Self::Tenant, Self::Public];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "private" => Ok(Self::Private),
            "tenant" => Ok(Self::Tenant),
            "public" => Ok(Self::Public),
            other => Err(Warning::InvalidValue {
                field: "privacy",
                detail: format!("unknown class {other}"),
            }),
        }
    }

    /// The wire label of this class.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::Tenant => "tenant",
            Self::Public => "public",
        }
    }

    /// A public release requires an explicit declassification grant
    /// (`PUBLISHING.md:66-70`, `playbooks/research-market-week.yaml:56-62`).
    #[must_use]
    pub const fn requires_grant(self) -> crate::Permission {
        match self {
            Self::Public => crate::Permission::Allow,
            Self::Private | Self::Tenant => crate::Permission::Deny,
        }
    }
}
