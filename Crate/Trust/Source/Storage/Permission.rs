//! Unix file-mode permissions used by the prototype CA.
//!
//! Grounded in `security/src/storage.rs:42-63` (secret files open with mode
//! 0o600 and `create_new`; public files use `create_new`), `storage.rs:128-135`
//! (the offline directory is 0o700), `storage.rs:172-175` (identity directories
//! are 0o700) and `storage.rs:286-291` (the staged TLS key is 0o600). A wrong
//! mode degrades to a warning; no file is opened here.

use crate::warning::Warning;

/// A file class and its required mode in the prototype.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Permission {
    /// A secret file: 0o600 (`storage.rs:49`).
    SecretFile,
    /// The offline CA directory: 0o700 (`storage.rs:130`).
    OfflineDirectory,
    /// A per-identity directory: 0o700 (`storage.rs:175`).
    IdentityDirectory,
    /// A public trust file: default mode with `create_new` (`storage.rs:55-63`).
    PublicFile,
}

impl Permission {
    /// All file classes.
    pub const ALL: [Self; 4] = [
        Self::SecretFile,
        Self::OfflineDirectory,
        Self::IdentityDirectory,
        Self::PublicFile,
    ];

    /// Required mode for this class, where one is declared.
    #[must_use]
    pub const fn mode(self) -> u32 {
        match self {
            Self::SecretFile => 0o600,
            Self::OfflineDirectory | Self::IdentityDirectory => 0o700,
            Self::PublicFile => 0o644,
        }
    }

    /// Look up a class by mode. A mode the model does not know is reported as
    /// `None` so the caller can warn instead of panicking.
    #[must_use]
    pub const fn from_mode(mode: u32) -> Option<Self> {
        match mode {
            0o600 => Some(Self::SecretFile),
            0o700 => Some(Self::OfflineDirectory),
            0o644 => Some(Self::PublicFile),
            _ => None,
        }
    }

    /// Check an observed mode against the declared mode for a class.
    pub fn check(self, observed: u32) -> Result<(), Warning> {
        match observed == self.mode() {
            true => Ok(()),
            false => Err(Warning::Malformed {
                domain: "file-mode",
                detail: format!("{:o}", observed),
            }),
        }
    }

    /// Writes never overwrite: the prototype uses `create_new`
    /// (`storage.rs:47,59`), so an existing path is a collision.
    #[must_use]
    pub const fn may_overwrite() -> crate::permission::Permission {
        crate::permission::Permission::Deny
    }
}
