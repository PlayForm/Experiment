//! Typed OpenPGP helper commands, never executed.
//!
//! Grounded in `security/src/main.rs:36-43` (the four feature-gated verbs and
//! their argument shapes) and `security/src/openpgp.rs:9-31` (each verb reads
//! paths and calls the optional helper). These variants model the command
//! surface only; no subprocess is spawned and no file is read.

use crate::permission::Permission;

/// One typed command accepted by the optional OpenPGP helper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    /// `pgp-encrypt` (`main.rs:37`, `openpgp.rs:9-14`).
    Encrypt {
        public_key: String,
        input: String,
        output: String,
    },
    /// `pgp-decrypt` (`main.rs:39`, `openpgp.rs:15-19`).
    Decrypt {
        secret_key: String,
        passphrase: String,
        input: String,
        output: String,
    },
    /// `pgp-sign` (`main.rs:41`, `openpgp.rs:21-25`).
    Sign {
        secret_key: String,
        passphrase: String,
        input: String,
        signature: String,
    },
    /// `pgp-verify` (`main.rs:43`, `openpgp.rs:27-31`).
    Verify {
        public_key: String,
        input: String,
        signature: String,
    },
}

impl Command {
    /// The CLI verb for this command.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Encrypt { .. } => "pgp-encrypt",
            Self::Decrypt { .. } => "pgp-decrypt",
            Self::Sign { .. } => "pgp-sign",
            Self::Verify { .. } => "pgp-verify",
        }
    }

    /// Whether the command needs secret-key material, expressed as a
    /// permission rather than a boolean field.
    #[must_use]
    pub const fn needs_secret_key(&self) -> Permission {
        match self {
            Self::Decrypt { .. } | Self::Sign { .. } => Permission::Allow,
            Self::Encrypt { .. } | Self::Verify { .. } => Permission::Deny,
        }
    }

    /// All four command names in declaration order.
    #[must_use]
    pub const fn all_names() -> [&'static str; 4] {
        ["pgp-encrypt", "pgp-decrypt", "pgp-sign", "pgp-verify"]
    }
}
