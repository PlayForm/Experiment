//! One child endpoint of an ancestry edge.

/// A child mutation or claim identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Child {
    /// Child identifier.
    pub id: String,
}

impl Child {
    /// Declare a child endpoint.
    #[must_use]
    pub fn of(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}
