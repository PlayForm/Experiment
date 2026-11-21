//! The identity and role that hold a lease.

use super::role::Role;

/// A named holder with a declared workload role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Owner {
    /// Holder identifier.
    pub id: String,
    /// Workload role.
    pub role: Role,
}

impl Owner {
    /// Declare an owner.
    #[must_use]
    pub fn of(id: &str, role: Role) -> Self {
        Self {
            id: id.to_owned(),
            role,
        }
    }

    /// Report an unnamed holder without panicking.
    #[must_use]
    pub fn validate(&self) -> Vec<crate::warning::Warning> {
        self.id.is_empty()
            .then(|| crate::warning::Warning::Malformed {
                domain: "lease owner",
                detail: "a lease holder must be named".to_owned(),
            })
            .into_iter()
            .collect()
    }
}
