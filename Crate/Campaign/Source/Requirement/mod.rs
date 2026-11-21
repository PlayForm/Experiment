//! The 22 numbered requirement sections as typed data.
//!
//! Grounded in all 1,149 lines of `REQUIREMENTS.md`: 17 design laws, the
//! REQ-* families, the eight progression gates (`:929-972`), the twelve
//! research questions (`:1001-1032`) and the acceptance scenarios
//! (`:974-999`). A requirement here is a typed value with a verification
//! kind; no test is run and no gate is enforced by this crate.

#[path = "Requirement.rs"]
pub mod requirement;
#[path = "Id.rs"]
pub mod id;
#[path = "Section.rs"]
pub mod section;
#[path = "Category.rs"]
pub mod category;
#[path = "AcceptanceGate.rs"]
pub mod gate;
#[path = "ResearchQuestion.rs"]
pub mod question;

pub use category::Category;
pub use gate::Gate;
pub use id::ID;
pub use question::Question;
pub use requirement::Requirement;
pub use section::Section;
