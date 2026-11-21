//! One typed HTTP operation from the canonical index.
//!
//! Every row of `api/HTTP_INDEX.md:9-128` becomes an `Operation` with its
//! operation ID, method, path template, plane, domain, input and output schema
//! references, allowed service roles and pinned policy revision. The operation
//! is data: it is never mounted, dispatched or authorized at runtime here
//! (`docs/26-acceptance-and-build-plan.md:60-65`).

use crate::api::{Method, Path, Plane};

/// One declared operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Operation {
    /// Stable operation ID from the OpenAPI map.
    pub id: &'static str,
    /// HTTP verb.
    pub method: Method,
    /// Path template.
    pub path: Path,
    /// Authenticated plane.
    pub plane: Plane,
    /// Inventory domain tag.
    pub domain: &'static str,
    /// Request body schema, if the verb carries one.
    pub input: Option<&'static str>,
    /// Success response schema, if the operation returns a body.
    pub output: Option<&'static str>,
    /// Allowed roles from `x-lab-roles`.
    pub roles: &'static [&'static str],
    /// Pinned policy revision (`6` or `6.1`).
    pub revision: &'static str,
}

impl Operation {
    /// Build one catalog entry from trusted research data.
    #[must_use]
    pub const fn of(
        id: &'static str,
        method: Method,
        path: &'static str,
        plane: Plane,
        domain: &'static str,
        input: Option<&'static str>,
        output: Option<&'static str>,
        roles: &'static [&'static str],
        revision: &'static str,
    ) -> Self {
        Self {
            id,
            method,
            path: Path::of(path),
            plane,
            domain,
            input,
            output,
            roles,
            revision,
        }
    }

    /// Whether this operation is one of the two owner writes
    /// (`docs/03-external-api.md:23-26`).
    #[must_use]
    pub fn is_owner_write(&self) -> bool {
        matches!(self.id, "submitOwnerCommand" | "validatePolicyCandidate")
    }

    /// Whether a worker role appears in the authorization list.
    #[must_use]
    pub fn grants_worker(&self) -> crate::Permission {
        match self.roles.iter().any(|role| *role == "llm-worker" || *role == "worker") {
            true => crate::Permission::Allow,
            false => crate::Permission::Deny,
        }
    }
}
