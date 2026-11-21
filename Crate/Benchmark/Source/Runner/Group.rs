//! Release and diagnostic runner groups.
//!
//! Grounded in `prun_lab.py:236` (release profile -> `release`, everything
//! else -> `diagnostic`), `:341-367` (two runner images), and
//! `README.md:121-123` (comparisons stay sequential inside each group;
//! release and debug results are not like-for-like).

use serde::{Deserialize, Serialize};

use crate::config::Profile;

/// The two isolated benchmark environments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Group {
    Release,
    Diagnostic,
}

impl Group {
    /// Deterministic group order (prun_lab.py:347).
    pub const ALL: [Self; 2] = [Self::Release, Self::Diagnostic];

    /// Group tag used in image targets and result filenames.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Diagnostic => "diagnostic",
        }
    }
}

impl From<Profile> for Group {
    fn from(profile: Profile) -> Self {
        match profile {
            Profile::Release => Self::Release,
            Profile::Dev => Self::Diagnostic,
        }
    }
}
