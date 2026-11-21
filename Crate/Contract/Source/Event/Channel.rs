//! The 15 signed event channels of the AsyncAPI contract.
//!
//! `api/events.asyncapi.yaml:9-101` declares 10 `lab.v6.*` channels and 5
//! `lab.v6.1.*` channels; every channel carries the single `AuditEvent`
//! message (`api/events.asyncapi.yaml:193-199`), and every subscription is a
//! trusted-software `receive` operation. No model is a subscriber.

/// One declared event channel.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Channel {
    Campaign,
    Source,
    Research,
    Worker,
    Mutation,
    Build,
    Oracle,
    Benchmark,
    Security,
    Operations,
    Custody,
    License,
    Feedback,
    Knowledge,
    Curriculum,
}

impl Channel {
    /// The AsyncAPI `address` value.
    #[must_use]
    pub const fn address(self) -> &'static str {
        match self {
            Self::Campaign => "lab.v6.campaign",
            Self::Source => "lab.v6.source",
            Self::Research => "lab.v6.research",
            Self::Worker => "lab.v6.worker",
            Self::Mutation => "lab.v6.mutation",
            Self::Build => "lab.v6.build",
            Self::Oracle => "lab.v6.oracle",
            Self::Benchmark => "lab.v6.benchmark",
            Self::Security => "lab.v6.security",
            Self::Operations => "lab.v6.operations",
            Self::Custody => "lab.v6.1.custody",
            Self::License => "lab.v6.1.license",
            Self::Feedback => "lab.v6.1.feedback",
            Self::Knowledge => "lab.v6.1.knowledge",
            Self::Curriculum => "lab.v6.1.curriculum",
        }
    }

    /// The contract revision that introduced the channel.
    #[must_use]
    pub const fn revision(self) -> &'static str {
        match self {
            Self::Campaign => "6",
            Self::Source => "6",
            Self::Research => "6",
            Self::Worker => "6",
            Self::Mutation => "6",
            Self::Build => "6",
            Self::Oracle => "6",
            Self::Benchmark => "6",
            Self::Security => "6",
            Self::Operations => "6",
            Self::Custody => "6.1",
            Self::License => "6.1",
            Self::Feedback => "6.1",
            Self::Knowledge => "6.1",
            Self::Curriculum => "6.1",
        }
    }

    /// Whether a model worker may subscribe.
    #[must_use]
    pub const fn model_access(self) -> crate::Permission {
        crate::Permission::Deny
    }
}
