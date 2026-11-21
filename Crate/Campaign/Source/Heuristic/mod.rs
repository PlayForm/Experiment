//! The heuristic planner feature family as typed modes and measurements.
//!
//! Grounded in `REQUIREMENTS.md:194-278` (REQ-HEU-01..16), `:832-858` (the
//! runtime planner cost model), `campaign.v2.json:125-137` and the schema's
//! heuristics block (`schemas/campaign.v2.schema.json:406-470`). The planner
//! is a design contract: modes, analyses, plans and overhead fields are data;
//! no rule is executed and no command is probed.

#[path = "Heuristic.rs"]
pub mod root;
#[path = "Mode.rs"]
pub mod mode;
#[path = "Static.rs"]
pub mod static_mode;
#[path = "Shadow.rs"]
pub mod shadow;
#[path = "Reference.rs"]
pub mod reference;
#[path = "Fallback.rs"]
pub mod fallback;
#[path = "Overhead.rs"]
pub mod overhead;
#[path = "Analysis.rs"]
pub mod analysis;
#[path = "Plan.rs"]
pub mod plan;

pub use analysis::Analysis;
pub use fallback::Fallback;
pub use mode::Mode;
pub use overhead::Overhead;
pub use plan::Plan;
pub use reference::Reference;
pub use shadow::Shadow;
pub use static_mode::Static;
pub use root::Heuristic;
