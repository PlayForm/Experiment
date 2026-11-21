//! The frozen question of a pre-registered case.
//!
//! Grounded in `schemas/research-case.v5.schema.json:74-76` (`minLength: 10`),
//! `examples/research-case.v5.json:11` and `SYSTEM.md:8-11` (one atomic
//! algorithmic change, measurable predictions, separately traceable findings).
//! A question is validated text, never an instruction to a model.

/// The immutable question a case registers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Question {
    /// The question text.
    pub text: String,
}

impl Question {
    /// Minimum question length from the schema (`schema:76`).
    pub const MIN_LENGTH: usize = 10;

    /// Validate the question without mutating it.
    pub fn validate(&self) -> Result<(), crate::Warning> {
        match self.text.len() >= Self::MIN_LENGTH {
            true => Ok(()),
            false => Err(crate::Warning::InvalidValue {
                field: "question",
                detail: format!("expected at least {} bytes", Self::MIN_LENGTH),
            }),
        }
    }

    /// Whether this question is suitable for preregistration.
    #[must_use]
    pub fn is_frozen(&self) -> bool {
        self.validate().is_ok()
    }
}
