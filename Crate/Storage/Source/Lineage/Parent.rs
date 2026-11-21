//! One parent endpoint of an ancestry edge.

use super::purpose::Purpose;

/// A parent identifier and the purpose of its edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parent {
    /// Parent mutation or claim identifier.
    pub id: String,
    /// Purpose of the relationship.
    pub purpose: Purpose,
}

impl Parent {
    /// Declare a parent endpoint.
    #[must_use]
    pub fn of(id: &str, purpose: Purpose) -> Self {
        Self {
            id: id.to_owned(),
            purpose,
        }
    }
}
