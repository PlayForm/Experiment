//! Direction of a denied trade edge.
//!
//! A signed restriction may establish A to B, B to A, or both
//! (`docs/20:40-42`; `examples/v61-trade-restriction`, `direction: both`).

/// Which traversals a trade edge blocks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// Only source to target is blocked.
    Forward,
    /// Only target to source is blocked.
    Reverse,
    /// Neither direction may cross.
    Both,
}

impl Direction {
    /// Whether a traversal that runs against the stored edge is denied.
    /// `reversed` is computed by the edge owner, never by a worker.
    #[must_use]
    pub fn denies(&self, reversed: crate::switch::Switch) -> bool {
        match (self, reversed) {
            (Self::Both, _) | (Self::Forward, crate::switch::Switch::Off) => true,
            (Self::Reverse, crate::switch::Switch::On) => true,
            (Self::Forward, crate::switch::Switch::On)
            | (Self::Reverse, crate::switch::Switch::Off) => false,
        }
    }

    /// The wire spelling used by the pinned example.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Forward => "forward",
            Self::Reverse => "reverse",
            Self::Both => "both",
        }
    }
}
