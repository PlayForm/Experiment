//! A declared HTTP path template.
//!
//! Paths are opaque contract strings, never filesystem paths. A `{id}` segment
//! is a typed parameter; the API surface never accepts a shell command, a
//! prompt string or a model-delivered attestation object
//! (`docs/23-interface-inventory.md:75-88`).

use crate::Fault;

/// A validated path template descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Path {
    template: &'static str,
}

impl Path {
    /// Build a path template from a trusted catalog entry.
    #[must_use]
    pub const fn of(template: &'static str) -> Self {
        Self { template }
    }

    /// Validate an external path spelling without allocating it.
    ///
    /// A fault is returned rather than a panic so a malformed caller path can
    /// be refused with a normalized `api-error` (`docs/03-external-api.md:20-22`).
    #[must_use]
    pub fn validate(value: &str) -> Result<(), Fault> {
        let rooted = value.starts_with('/');
        let plain = !value.contains("://") && !value.contains('?') && !value.contains('#');
        match (rooted, plain) {
            (true, true) => Ok(()),
            _ => Err(Fault::Shape {
                domain: "http path",
                detail: format!("{value} is not a rooted, query-free template"),
            }),
        }
    }

    /// Borrow the template text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.template
    }

    /// Extract the single `{name}` parameter, if this template has one.
    #[must_use]
    pub fn parameter(&self) -> Option<&'static str> {
        self.template
            .split_once('{')
            .and_then(|(_, rest)| rest.split_once('}'))
            .map(|(name, _)| name)
    }
}
