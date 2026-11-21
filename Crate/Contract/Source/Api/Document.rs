//! Document descriptors for the three canonical API contracts.
//!
//! Grounded in `api/public.openapi.yaml:1-31` (OpenAPI 3.1.1, mTLS only),
//! `api/internal.openapi.yaml:1-31` and `api/events.asyncapi.yaml:1-8,193-199`
//! (AsyncAPI 3.1.0, one `AuditEvent` message). These are metadata values; no
//! document is parsed from disk and no schema is fetched over HTTP.

/// Metadata for one machine-readable API document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Document {
    /// Document title from `info.title`.
    pub title: &'static str,
    /// Document version from `info.version`.
    pub version: &'static str,
    /// Specification name and version.
    pub spec: &'static str,
    /// Declared security requirement.
    pub security: &'static str,
    /// Default or declared content type.
    pub content_type: &'static str,
}

impl Document {
    /// The operator-facing OpenAPI document.
    #[must_use]
    pub const fn external() -> Self {
        Self {
            title: "Rust Laboratory External API",
            version: "6.1.0",
            spec: "openapi 3.1.1",
            security: "mutualTLS",
            content_type: "application/json",
        }
    }

    /// The trusted internal OpenAPI document.
    #[must_use]
    pub const fn internal() -> Self {
        Self {
            title: "Rust Laboratory Internal API",
            version: "6.1.0",
            spec: "openapi 3.1.1",
            security: "mutualTLS",
            content_type: "application/json",
        }
    }

    /// The signed event bus AsyncAPI document.
    #[must_use]
    pub const fn events() -> Self {
        Self {
            title: "Rust Lab Signed Event Bus",
            version: "6.1.0",
            spec: "asyncapi 3.1.0",
            security: "authenticated-subscriber-only",
            content_type: "application/json",
        }
    }

    /// Whether this document is a request/response API.
    #[must_use]
    pub const fn is_http(self) -> crate::Permission {
        match self.spec.as_bytes().first() {
            Some(b'o') => crate::Permission::Allow,
            _ => crate::Permission::Deny,
        }
    }
}
