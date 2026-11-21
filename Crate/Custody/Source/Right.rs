//! Distinct rights over one held artifact.
//!
//! `CanHold` is not `CanRead`, `CanExecute`, `CanDerive`, `CanRedistribute`
//! or `CanTrainModelOn` (`docs/19:66-70`; `docs/26:73-74`). Storing encrypted
//! bytes grants none of the others.

/// One authority a holder may possess over an artifact.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Right {
    /// Store encrypted bytes; the default and narrowest right.
    Hold,
    /// Decrypt and view plaintext.
    Read,
    /// Execute code as a trusted builder or runner.
    Execute,
    /// Derive a new artifact from the contents.
    Derive,
    /// Redistribute the artifact across a repository boundary.
    Redistribute,
    /// Train or fine-tune a model on the contents.
    Train,
}

impl Right {
    /// The external capability name this right models.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Hold => "CanHold",
            Self::Read => "CanRead",
            Self::Execute => "CanExecute",
            Self::Derive => "CanDerive",
            Self::Redistribute => "CanRedistribute",
            Self::Train => "CanTrainModelOn",
        }
    }

    /// Rights that can never be granted to an offline worker.
    #[must_use]
    pub fn worker_forbidden(&self) -> bool {
        match self {
            Self::Hold => false,
            Self::Read | Self::Execute | Self::Derive | Self::Redistribute | Self::Train => true,
        }
    }
}
