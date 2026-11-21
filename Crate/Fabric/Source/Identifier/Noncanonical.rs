//! The factory rule applied when raw identifier bytes are not canonical.
//!
//! Grounded in `docs/03:5-18` and the reference validator: worker-supplied
//! bytes never change the authoritative parser.

/// How the identifier factory treats noncanonical input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Noncanonical {
    /// Reject noncanonical input rather than rewriting it.
    Reject,
    /// Rewrite noncanonical input into its canonical form.
    Rewrite,
}
