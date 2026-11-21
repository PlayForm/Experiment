//! Operator-only diagnostics and the single neutral worker view.
//! Grounded in `22-hidden-worker-and-correspondence.md:47-69` (non-disclosing
//! error boundaries: a worker gets a fixed opaque receipt, never the reason).

/// A degradation that is logged for the operator, never sent to a worker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Warning {
    /// Operator-only message.
    pub operator: String,
}

impl Warning {
    /// Construct a warning from a plain message.
    #[must_use]
    pub fn of(message: &str) -> Self {
        Self {
            operator: message.to_owned(),
        }
    }

    /// The one neutral string a worker may observe.
    #[must_use]
    pub const fn worker_status(&self) -> &'static str {
        "submission_received"
    }

    /// A warning is never worker-safe evidence.
    #[must_use]
    pub const fn worker_safe(&self) -> bool {
        false
    }
}
