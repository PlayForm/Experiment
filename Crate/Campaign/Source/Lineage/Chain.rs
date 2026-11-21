//! The minimal lineage proof chain as a typed sequence.
//!
//! Grounded in `REQUIREMENTS.md:752-772`: Campaign C → Hypothesis H → Mutation
//! M → Source S → Build B → Artifact A → Oracle O → Experiment E → Trial T →
//! Analysis X → Decision D → Next. The chain is linear per causal lane; forks
//! and joins live in `Ancestry`. A benchmark result never becomes the parent
//! of source code.

use crate::{lineage::Link, Warning};

/// One linear proof chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chain {
    pub links: Vec<Link>,
}

impl Chain {
    /// Build the design's proof chain.
    #[must_use]
    pub fn from_design() -> Self {
        Self {
            links: Link::ALL.to_vec(),
        }
    }

    /// Whether a link appears in the chain.
    #[must_use]
    pub fn contains(&self, link: Link) -> bool {
        self.links.contains(&link)
    }

    /// The immediate predecessor of a link, if any.
    #[must_use]
    pub fn predecessor(&self, link: Link) -> Option<Link> {
        self.links
            .iter()
            .position(|item| *item == link)
            .and_then(|position| position.checked_sub(1))
            .and_then(|position| self.links.get(position))
            .copied()
    }

    /// Warn on a missing link rather than assuming a complete proof.
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        Link::ALL
            .into_iter()
            .filter(|required| !self.contains(*required))
            .for_each(|missing| warnings.push(Warning::Unavailable {
                source: format!("lineage.chain.{}", missing.label()),
                substitute: "incomplete proof chain".to_owned(),
            }));
    }
}
