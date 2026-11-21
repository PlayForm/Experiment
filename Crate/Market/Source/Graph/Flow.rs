//! The two observability feeds into the research cockpit.
//!
//! Grounded in `graphs/research-economy.mmd:24-26` and
//! `architecture/TELEMETRY.md:5-16`: control/research telemetry and isolated
//! product measurement are separate planes; both feed the GPUI observatory, but
//! neither carries private opinions or plaintext prompts.

use super::Node;

/// One observability flow into the cockpit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Flow {
    /// Control and research planes (`mmd:24`, `TELEMETRY.md:5-13`).
    ControlResearch,
    /// Isolated product measurement plane (`mmd:25`, `TELEMETRY.md:14-16`).
    ProductMeasurement,
}

impl Flow {
    /// Both observability flows.
    pub const ALL: [Self; 2] = [Self::ControlResearch, Self::ProductMeasurement];

    /// The graph node that emits this flow.
    #[must_use]
    pub const fn source(self) -> Node {
        match self {
            Self::ControlResearch => Node::ObservabilityControl,
            Self::ProductMeasurement => Node::ObservabilityProduct,
        }
    }

    /// Both flows terminate at the GPUI observatory (`mmd:26-27`).
    #[must_use]
    pub const fn target() -> Node {
        Node::Observatory
    }

    /// No flow may leak a private opinion into telemetry
    /// (`TELEMETRY.md:7,13`).
    #[must_use]
    pub const fn may_carry_private_opinion() -> crate::Permission {
        crate::Permission::Deny
    }
}
