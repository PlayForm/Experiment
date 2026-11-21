//! The five disjoint mutation scopes of a fanout.
//! Grounded in `ARCHITECTURE.md:69-76` (allocate five disjoint mutation scopes
//! or hypothesis families; workers share no conversational history, rival
//! patches, sealed samples or leaderboard) and `experiment::FANOUT`.

/// One of the five disjoint areas a worker may mutate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Scope {
    /// Directory traversal and path selection.
    Walk,
    /// Command execution and process handling.
    Execute,
    /// Event recording and taxonomy.
    Event,
    /// Binary entry point and argument parsing.
    Binary,
    /// The behavioural contract surface.
    Contract,
}

impl Scope {
    /// The five scopes in fanout order.
    pub const ALL: [Self; 5] = [
        Self::Walk,
        Self::Execute,
        Self::Event,
        Self::Binary,
        Self::Contract,
    ];

    /// The five scopes as an owned vector.
    #[must_use]
    pub fn all() -> Vec<Self> {
        Self::ALL.to_vec()
    }

    /// Whether two scopes are disjoint; identical scopes collide.
    #[must_use]
    pub const fn disjoint_from(self, other: Self) -> bool {
        !self.eq(&other)
    }

    /// Short key for the task bundle.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Walk => "walk",
            Self::Execute => "execute",
            Self::Event => "event",
            Self::Binary => "binary",
            Self::Contract => "contract",
        }
    }
}
