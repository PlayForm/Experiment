//! Fail-closed revocation transition.
//!
//! Grounded in `security/src/storage.rs:221-228`: `revoke` validates the identity,
//! journals `IDENTITY_REVOKED` first and only then writes the effective marker,
//! so a partial revocation fails closed. `crypto.rs:116-118` refuses any identity
//! with a marker in `revoked/`. This type records the ordered transition; it
//! writes no journal and creates no marker.

/// The two ordered steps of a revocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Revoke {
    /// Journal the revocation intent before any state change (`storage.rs:225`).
    JournalIntent,
    /// Write the effective `revoked/<name>` marker (`storage.rs:226`).
    WriteMarker,
}

impl Revoke {
    /// The mandatory execution order.
    pub const ORDER: [Self; 2] = [Self::JournalIntent, Self::WriteMarker];

    /// Marker directory (`storage.rs:135`).
    pub const DIRECTORY: &'static str = "revoked";

    /// The audit event name (`storage.rs:225`).
    pub const EVENT: &'static str = "IDENTITY_REVOKED";

    /// Marker path for a name, as a typed path fragment.
    #[must_use]
    pub fn marker_path(name: &str) -> String {
        format!("{}/{name}", Self::DIRECTORY)
    }

    /// The index of this step in the mandatory order.
    #[must_use]
    pub const fn position(self) -> usize {
        match self {
            Self::JournalIntent => 0,
            Self::WriteMarker => 1,
        }
    }

    /// Whether a marker path is the one this step writes.
    #[must_use]
    pub fn writes(self, path: &str) -> bool {
        match self {
            Self::JournalIntent => false,
            Self::WriteMarker => path.starts_with(Self::DIRECTORY),
        }
    }
}
