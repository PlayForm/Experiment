//! Trusted fixtures drawn from a separately approved immutable trust root.
//!
//! Grounded in `CONTAINER_MATRIX.md:50-51` (fixtures and test definitions from
//! a separately approved immutable trust root; candidate binaries may not
//! redefine pass/fail criteria), `RESEARCH_GATE.md:92-93` (avoid leaking hidden
//! oracle fixtures via failure details; provide only minimal violation
//! categories to patchers) and `STORAGE.md:72-80` (sealed fixture metadata in
//! an encrypted namespace).

/// Which view of a fixture result is being rendered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum View {
    /// Full details, retained under trusted control.
    Trusted,
    /// Minimal violation category, the only detail a patcher may receive.
    Minimal,
}

/// Whether a fixture is held in the sealed namespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Seal {
    /// Sealed; hidden from mutators and generic APIs.
    Sealed,
    /// Open to authorized roles.
    Open,
}

/// One immutable trusted fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fixture {
    /// Stable fixture identifier.
    pub id: String,
    /// Digest of the fixture manifest.
    pub manifest_sha256: String,
    /// The frozen source commit that owns this fixture.
    pub trusted_commit: String,
    /// Whether this fixture belongs to the sealed holdout namespace.
    pub seal: Seal,
}

impl Fixture {
    /// Whether a patcher may read this fixture. Sealed fixtures are never
    /// exposed to mutators (`REQUIREMENTS_V3.md` AGT3-009).
    #[must_use]
    pub const fn readable_by_patcher(&self) -> bool {
        matches!(self.seal, Seal::Open)
    }

    /// Render a failure detail for the requested view. Sealed fixtures always
    /// collapse to a minimal category, never a hidden assertion or input.
    #[must_use]
    pub fn failure_detail(&self, view: View, detail: &str) -> String {
        match (view, self.seal) {
            (View::Trusted, _) => detail.to_string(),
            (View::Minimal, _) => "oracle_violation".to_string(),
        }
    }
}
