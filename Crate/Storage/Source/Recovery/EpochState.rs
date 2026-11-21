//! The relation of a presented epoch to the current one.

/// How an authority token's epoch compares with the live epoch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EpochState {
    /// The presented epoch is the live one.
    Current,
    /// The presented epoch is older and its authority is fenced.
    Stale,
    /// The presented epoch is newer than this projection knows.
    Superseded,
}

impl EpochState {
    /// The declared state name.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Superseded => "superseded",
        }
    }
}
