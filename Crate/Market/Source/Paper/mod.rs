//! The paper pipeline with an explicit, non-auto-promoting status ladder.
//!
//! Grounded in `PUBLISHING.md:13-28` (eight status rungs, including corrective
//! amendments), `:34-40` (assembly pipeline), `:54-70` (deliberation privacy,
//! sensitive scans, declassification grant), `:86-116` (paper format and
//! transfer), `schemas/paper.v5.schema.json:6-151`,
//! `examples/paper.v5.json:2-25`, `sql/v5_migration.sql:171-228` (manuscripts,
//! claims, reviews) and `README.md:46-49` (a research paper is never
//! automatically a peer-reviewed publication). No paper is assembled or
//! promoted here.

#[path = "Paper.rs"]
pub mod paper;
#[path = "Notebook.rs"]
pub mod notebook;
#[path = "Status.rs"]
pub mod status;
#[path = "Transition.rs"]
pub mod transition;
#[path = "Sanitization.rs"]
pub mod sanitization;
#[path = "Link.rs"]
pub mod link;
#[path = "Review.rs"]
pub mod review;

pub use link::Link;
pub use notebook::Notebook;
pub use paper::Paper;
pub use review::Review;
pub use sanitization::Sanitization;
pub use status::Status;
pub use transition::Transition;
