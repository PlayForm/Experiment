//! Structural validator over the typed contract catalog.
//!
//! The validator checks operation-ID uniqueness, resolvable schema references,
//! event-kind uniqueness, payload-schema resolution, example conformance and
//! service-route resolution. Every missing reference becomes a warning finding
//! and validation continues; a worker grant or duplicate identifier becomes a
//! fault finding. No validator is executed by this crate and no HTTP call is
//! made (`docs/26-acceptance-and-build-plan.md:60-80`).

use crate::validate::{Finding, Report};

/// A structural-only validator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Validator {
    known_schemas: &'static [&'static str],
}

impl Validator {
    /// Build a validator pinned to the full schema namespace.
    #[must_use]
    pub const fn of() -> Self {
        Self { known_schemas: crate::schema::Definition::ALL }
    }

    /// Reduce a schema reference to its stem, without panicking on odd input.
    fn stem(reference: &str) -> &str {
        reference
            .rsplit('/')
            .next()
            .unwrap_or(reference)
            .strip_suffix(".schema.json")
            .unwrap_or(reference)
    }

    /// Whether a schema reference resolves against the known namespace.
    fn resolves(&self, reference: &str) -> bool {
        let stem = Self::stem(reference);
        self.known_schemas.iter().any(|known| *known == stem)
    }

    /// Validate the whole catalog without mutating it.
    #[must_use]
    pub fn validate(&self, catalog: &crate::catalog::Catalog) -> Report {
        Report::new()
            .merge(self.operations(catalog))
            .merge(self.events(catalog))
            .merge(self.examples())
            .merge(self.routes(catalog))
            .with(Finding::info(
                "catalog",
                format!(
                    "checked {} operations, {} events, {} schemas",
                    catalog.api.len(),
                    catalog.events.len(),
                    self.known_schemas.len()
                ),
            ))
    }

    /// Operation catalog checks.
    fn operations(&self, catalog: &crate::catalog::Catalog) -> Report {
        let duplicates = catalog.api.duplicates().into_iter().map(|id| {
            Finding::fault("api", format!("duplicate operation ID {id}"))
        });
        let worker_grants = catalog
            .api
            .iter()
            .filter(|operation| operation.grants_worker() == crate::Permission::Allow)
            .map(|operation| {
                Finding::fault(
                    "api",
                    format!("operation {} grants a worker role", operation.id),
                )
            });
        let missing = catalog.api.iter().flat_map(|operation| {
            [operation.input, operation.output]
                .into_iter()
                .flatten()
                .filter(|reference| !self.resolves(reference))
                .map(|reference| {
                    Finding::warning(
                        "api",
                        format!(
                            "operation {} references missing schema {}",
                            operation.id,
                            reference
                        ),
                    )
                })
                .collect::<Vec<Finding>>()
        });
        Report::new().extend(duplicates.chain(worker_grants).chain(missing).collect())
    }

    /// Event catalog checks.
    fn events(&self, catalog: &crate::catalog::Catalog) -> Report {
        let duplicates = catalog
            .events
            .duplicates()
            .into_iter()
            .map(|kind| Finding::fault("event", format!("duplicate event kind {}", kind.as_str())));
        let missing = catalog
            .events
            .iter()
            .filter(|entry| !self.resolves(entry.schema))
            .map(|entry| {
                Finding::warning(
                    "event",
                    format!(
                        "event {} references missing schema {}",
                        entry.kind.as_str(),
                        entry.schema
                    ),
                )
            });
        let worker_visible = catalog
            .events
            .iter()
            .filter(|entry| entry.visible == crate::Permission::Allow)
            .map(|entry| {
                Finding::fault(
                    "event",
                    format!("event {} is worker-visible", entry.kind.as_str()),
                )
            });
        Report::new().extend(duplicates.chain(missing).chain(worker_visible).collect())
    }

    /// Example conformance checks: a missing schema is a warning.
    fn examples(&self) -> Report {
        let findings = crate::example::Conformance::all()
            .into_iter()
            .filter(|pair| !self.resolves(pair.schema))
            .map(|pair| {
                Finding::warning(
                    "example",
                    format!("example {} conforms to missing schema {}", pair.example, pair.schema),
                )
            })
            .chain(
                crate::example::Conformance::all()
                    .into_iter()
                    .filter(|pair| pair.matches() == crate::Permission::Deny)
                    .map(|pair| {
                        Finding::warning(
                            "example",
                            format!("example {} has non-matching schema stem {}", pair.example, pair.schema),
                        )
                    }),
            );
        Report::new().extend(findings.collect())
    }

    /// Internal service-route checks against the operation index.
    fn routes(&self, catalog: &crate::catalog::Catalog) -> Report {
        let findings = catalog.policy.services.iter().flat_map(|service| {
            service
                .paths
                .iter()
                .filter(|path| {
                    catalog
                        .api
                        .iter()
                        .find(|operation| operation.path.as_str() == **path)
                        .is_none()
                })
                .map(|path| {
                    Finding::warning(
                        "policy",
                        format!("service {} references missing route {}", service.name, path),
                    )
                })
                .collect::<Vec<Finding>>()
        });
        Report::new().extend(findings.collect())
    }
}
