//! The sealed holdout, revealed only by a trusted confirmer.
//! Grounded in `RESEARCH_GATE.md:76-95` (sealed holdout evaluated only at fixed
//! checkpoints, results withheld until close or approved reveal) and in
//! `AGENT_CONTRACT.md:69-71` (independent confirmation uses a sealed holdout).

/// A sealed confirmation dataset with an explicit reveal gate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Holdout {
    /// Digest of the sealed fixture set.
    pub seal_digest: String,
    /// Number of campaigns left before the seal may be opened.
    pub sealed_campaigns: u32,
    /// Digest recorded when a sanctioned reveal happened; absent while sealed.
    pub revealed_at: Option<String>,
}

impl Holdout {
    /// Seal a holdout for a number of campaigns.
    #[must_use]
    pub fn seal(seal_digest: &str, sealed_campaigns: u32) -> Self {
        Self {
            seal_digest: seal_digest.to_owned(),
            sealed_campaigns,
            revealed_at: None,
        }
    }

    /// Whether a confirmer may open the seal at campaign close.
    #[must_use]
    pub fn may_reveal(&self, campaign_closed: bool, authorized: bool) -> bool {
        match (campaign_closed, authorized, self.sealed_campaigns) {
            (true, true, _) => true,
            _ => false,
        }
    }

    /// Record a sanctioned reveal; the seal is never silently reused.
    #[must_use]
    pub fn reveal(self, digest: &str) -> Self {
        Self {
            revealed_at: Some(digest.to_owned()),
            sealed_campaigns: self.sealed_campaigns.saturating_sub(1),
            ..self
        }
    }

    /// Whether the holdout is still sealed.
    #[must_use]
    pub const fn is_sealed(&self) -> bool {
        self.revealed_at.is_none()
    }
}
