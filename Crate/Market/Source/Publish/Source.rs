//! The provenance class of a source submitted to the confidential scan.
//!
//! Grounded in `PUBLISHING.md:65-66`: an unknown external repository requires
//! human review, whereas a known source may pass an empty finding set.

/// Whether a submitted source is already known to the publisher.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    /// A source whose provenance the publisher already knows.
    Known,
    /// An external repository that has not been classified.
    External,
}
