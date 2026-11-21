//! One signed denial edge in the trade/conflict graph.
//!
//! Mirrors `examples/v61-trade-restriction.example.json`. Edges are enforced
//! even for an apparently harmless citation, because existence metadata can
//! leak a sibling's hypothesis (`docs/20:40-45`).

use crate::direction::Direction;
use crate::permission::Permission;
use crate::switch::Switch;

/// A signed restriction between two research groups.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edge {
    /// `restriction_id` of the signed edge.
    pub restriction_id: String,
    /// `source_group_id` named by the restriction.
    pub source_group_id: String,
    /// `target_group_id` named by the restriction.
    pub target_group_id: String,
    /// `direction` of the block.
    pub direction: Direction,
    /// `allowed_metadata` that may be published in aggregate.
    pub allowed_metadata: Vec<String>,
    /// `policy_digest` binding the edge to a policy epoch.
    pub policy_digest: String,
    /// `effective_at` canonical UTC instant.
    pub effective_at: String,
    /// `expires_at` canonical UTC instant.
    pub expires_at: String,
    /// Whether the graph edge is owner-signed.
    pub signed: Switch,
}

impl Edge {
    /// Whether this edge denies a traversal from one group to another.
    #[must_use]
    pub fn denies(&self, from: &str, to: &str) -> bool {
        let forward = from == self.source_group_id && to == self.target_group_id;
        let reverse = from == self.target_group_id && to == self.source_group_id;
        let reversed = match (forward, reverse) {
            (true, false) => Switch::Off,
            (false, true) => Switch::On,
            (false, false) | (true, true) => Switch::Off,
        };
        self.direction.denies(reversed) && self.signed == Switch::On
    }

    /// Whether an edge is admissible in the current policy epoch.
    #[must_use]
    pub fn valid(&self, at: &str) -> Permission {
        Permission::filled(&self.restriction_id)
            .and(Permission::digest(&self.policy_digest))
            .and(self.signed.permit())
            .and(Permission::of(self.effective_at <= at.to_owned() && at <= self.expires_at.as_str()))
    }

    /// Whether a metadata class is explicitly allowed in aggregate.
    #[must_use]
    pub fn metadata_allowed(&self, name: &str) -> Permission {
        Permission::of(self.allowed_metadata.iter().any(|item| item == name))
    }
}
