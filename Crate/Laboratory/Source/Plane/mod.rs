//! Plane and boundary topology facade.
//!
//! Declares only the five leaves. Plane 0 is the operator API, Plane 1 trusted
//! control, Plane 2 product evidence; Boundary X is an offline worker and is
//! deliberately not a fourth trusted plane (`README.md:7-20`,
//! `docs/01-topology-and-boundaries.md`).

#[path = "Public.rs"]
pub mod public;
#[path = "Control.rs"]
pub mod control;
#[path = "Product.rs"]
pub mod product;
#[path = "Boundary.rs"]
pub mod boundary;
#[path = "Worker.rs"]
pub mod worker;

pub use boundary::Boundary;
pub use control::Control;
pub use product::Product;
pub use public::Public;
pub use worker::Worker;
