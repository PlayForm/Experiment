//! Publication states (`docs/06-state-machines.md:66-76`).
//!
//! Draft -> private-reviewed -> sanitization-pending -> sanitized ->
//! internal-released -> public-released, with rejected branches. Public release
//! requires an owner policy decision; a paper never auto-promotes itself
//! (`docs/00-system-constitution.md:55`, `docs/17:96`).

/// Publication state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PublicationPhase {
    /// Private draft.
    Draft,
    /// Passed private review.
    PrivateReviewed,
    /// Awaiting sanitization.
    SanitizationPending,
    /// Sanitized and evidence-mapped.
    Sanitized,
    /// Released internally only.
    InternalReleased,
    /// Released publicly under owner policy.
    PublicReleased,
    /// Rejected by authority or sanitizer.
    Rejected,
}
