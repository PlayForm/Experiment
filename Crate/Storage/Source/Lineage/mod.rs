//! Experiment ancestry and correlation-index facade.
//!
//! Mutation ancestry is an explicit many-to-many relation with `causal` and
//! `merge` purposes (`sql/prerequisites/v2_lineage.sql:51-58`); claim links and
//! superseding temporal facts are further typed edges
//! (`sql/prerequisites/v5_migration.sql:82-97`, `sql/v61_migration.sql:343-355`).

#[path = "Purpose.rs"]
pub mod purpose;
#[path = "Parent.rs"]
pub mod parent;
#[path = "Child.rs"]
pub mod child;
#[path = "Relation.rs"]
pub mod relation;
#[path = "Edge.rs"]
pub mod edge;
#[path = "Direction.rs"]
pub mod direction;
#[path = "Query.rs"]
pub mod query;
#[path = "Ancestry.rs"]
pub mod ancestry;
