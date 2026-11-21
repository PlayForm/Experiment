//! Whether an atomic hypothesis registered a concrete falsifier.
//!
//! Grounded in `README.md:44` and `AGENTS.md:9`: a claim without a falsifier
//! is not preregistered and cannot be promoted to fact.

/// The preregistration status of a mechanism hypothesis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Falsifier {
    /// A concrete falsifier was declared before outcomes were observed.
    Declared,
    /// No falsifier was declared; the claim is not preregistered.
    Absent,
}
