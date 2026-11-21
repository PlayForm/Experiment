//! The two HTTP verbs the v6.1 contract declares.
//!
//! `HTTP_INDEX.md:9-128` lists only GET and POST. There is deliberately no
//! generic `execute`, PUT, PATCH or DELETE verb: owner writes are two
//! predeclared POSTs validated against pinned policy
//! (`docs/03-external-api.md:23-26`, `docs/04-internal-api.md:23-24`).

use crate::Fault;

/// A declared HTTP method.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Method {
    /// A read-only projection request.
    Get,
    /// A signed, idempotent command or report submission.
    Post,
}

impl Method {
    /// Wire spelling of the method.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
        }
    }

    /// Decode a wire method, degrading to a typed fault on anything else.
    #[must_use]
    pub fn parse(value: &str) -> Result<Self, Fault> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            other => Err(Fault::Shape {
                domain: "http method",
                detail: format!("{other} is not a declared verb"),
            }),
        }
    }

    /// Whether this verb may carry a mutation, as a typed permission.
    #[must_use]
    pub const fn may_mutate(self) -> crate::Permission {
        match self {
            Self::Get => crate::Permission::Deny,
            Self::Post => crate::Permission::Allow,
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Self::Get
    }
}
