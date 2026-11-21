//! Metadata-minimization techniques with an honest side-channel limit.
//!
//! `docs/02:36-39` lists bucketed lengths, delayed batch dispatch, scoped
//! rotating pseudonyms and independent network namespaces, and states they
//! cannot hide all correlations from the host. `docs/07:22-26` repeats the
//! limit. Each technique is a typed value; none is applied to traffic.

/// One declared traffic-minimization technique.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Traffic {
    /// Expose only bucketed message lengths.
    BucketedLength,
    /// Delay and batch dispatch to weaken timing correlation.
    DelayedBatch,
    /// Rotate pseudonyms per scope and epoch.
    RotatingPseudonym,
    /// Separate traffic into independent network namespaces.
    SeparateNamespace,
}

impl Traffic {
    /// All four techniques from `docs/02:38-39`.
    #[must_use]
    pub const fn all() -> [Self; 4] {
        [
            Self::BucketedLength,
            Self::DelayedBatch,
            Self::RotatingPseudonym,
            Self::SeparateNamespace,
        ]
    }

    /// These techniques cost throughput and still do not defeat a host
    /// administrator (`docs/07:25-26`).
    #[must_use]
    pub const fn absolute_secrecy() -> Verdict {
        Verdict::Deny
    }

    /// Stable label for an audit record.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::BucketedLength => "bucketed-length",
            Self::DelayedBatch => "delayed-batch",
            Self::RotatingPseudonym => "rotating-pseudonym",
            Self::SeparateNamespace => "separate-namespace",
        }
    }
}
