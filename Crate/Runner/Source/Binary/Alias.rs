//! The four bin aliases that all point at the same crate root.
//!
//! Grounded in `Module/Run/Cargo.toml:1-15`: `PRun`, `Run`, `Inn` and
//! `InnKeeper` each declare `path = "Source/Library.rs"`; `:47` sets
//! `default-run = "Run"`. `autobins = false` (`:44`) makes the list exact.

/// One published binary name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Alias {
    /// `[[bin]] name = "PRun"`.
    PRun,
    /// `[[bin]] name = "Run"`; the default.
    Run,
    /// `[[bin]] name = "Inn"`.
    Inn,
    /// `[[bin]] name = "InnKeeper"`.
    InnKeeper,
}

impl Alias {
    /// All four aliases in manifest order (`Cargo.toml:1-15`).
    pub const ALL: [Self; 4] = [Self::PRun, Self::Run, Self::Inn, Self::InnKeeper];
    /// `default-run` (`Cargo.toml:47`).
    pub const DEFAULT: Self = Self::Run;
    /// Shared crate root (`Cargo.toml:3,7,11,15`).
    pub const PATH: &'static str = "Source/Library.rs";

    /// The literal bin name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PRun => "PRun",
            Self::Run => "Run",
            Self::Inn => "Inn",
            Self::InnKeeper => "InnKeeper",
        }
    }

    /// Whether this alias is `default-run`.
    #[must_use]
    pub const fn is_default(self) -> bool {
        matches!(self, Self::Run)
    }
}
