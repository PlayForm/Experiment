//! Whether Cargo keeps default features, modelled as a two-variant enum
//! rather than a bare `bool` (house style).
//!
//! Grounded in `prun-bench.json:47` (`no_default_features`) and
//! `prun_lab.py:316-317`, which emits `--no-default-features` when set.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Default-feature policy for one feature set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefaultFeatures {
    Enabled,
    Disabled,
}

impl DefaultFeatures {
    /// The Cargo flag implied by this policy (prun_lab.py:316-317).
    #[must_use]
    pub const fn flag(self) -> Option<&'static str> {
        match self {
            Self::Enabled => None,
            Self::Disabled => Some("--no-default-features"),
        }
    }
}

impl Default for DefaultFeatures {
    fn default() -> Self {
        Self::Enabled
    }
}

impl<'de> Deserialize<'de> for DefaultFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        bool::deserialize(deserializer).map(|off| match off {
            true => Self::Disabled,
            false => Self::Enabled,
        })
    }
}

impl Serialize for DefaultFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => false.serialize(serializer),
            Self::Disabled => true.serialize(serializer),
        }
    }
}
