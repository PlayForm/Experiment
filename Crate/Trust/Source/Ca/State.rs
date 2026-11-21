//! The on-disk CA state layout created by `init`.
//!
//! Grounded in `security/src/storage.rs:124-162`: `init` refuses to reuse an
//! existing directory, creates `offline`, `public`, `audit`, `identities`,
//! `registry` and `revoked`, writes the authority public key, the TLS root
//! certificate and an empty issuance journal, then journals `ROOT_INITIALIZED`.
//! This type records the layout; it creates no directory and writes no file.

/// The typed directory layout of a CA state root.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    /// Offline secret custody (`storage.rs:128`).
    pub offline: &'static str,
    /// Public trust anchors (`storage.rs:131`).
    pub public: &'static str,
    /// Signed hash-chained journals (`storage.rs:132`).
    pub audit: &'static str,
    /// Per-identity secret directories (`storage.rs:133`).
    pub identities: &'static str,
    /// Attested identity records (`storage.rs:134`).
    pub registry: &'static str,
    /// Revocation markers (`storage.rs:135`).
    pub revoked: &'static str,
}

impl State {
    /// Construct the prototype's directory layout.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            offline: "offline",
            public: "public",
            audit: "audit",
            identities: "identities",
            registry: "registry",
            revoked: "revoked",
        }
    }

    /// All directories `init` creates, in creation order (`storage.rs:128-135`).
    #[must_use]
    pub const fn directories() -> [&'static str; 6] {
        ["offline", "public", "audit", "identities", "registry", "revoked"]
    }

    /// Whether a path is one of the declared CA directories.
    #[must_use]
    pub fn contains(&self, path: &str) -> bool {
        Self::directories().iter().any(|directory| *directory == path)
    }
}
