//! Typed schema facade for the whole additive chain.
//!
//! Declares only leaves; the tree is the API. Every table named in the five
//! migration files is represented by a value built in `Catalog`.

#[path = "Type.rs"]
pub mod scalar;
#[path = "Nullability.rs"]
pub mod nullability;
#[path = "Check.rs"]
pub mod check;
#[path = "Column.rs"]
pub mod column;
#[path = "ForeignKey.rs"]
pub mod foreign_key;
#[path = "Constraint.rs"]
pub mod constraint;
#[path = "Index.rs"]
pub mod index;
#[path = "Mutability.rs"]
pub mod mutability;
#[path = "Table.rs"]
pub mod table;
#[path = "Catalog.rs"]
pub mod catalog;
