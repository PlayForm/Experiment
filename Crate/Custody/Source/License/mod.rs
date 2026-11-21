//! Licensing and legal-review boundary facade.
//!
//! SPDX identifiers are not a legal conclusion: verbatim terms, EULA grants,
//! obligations, jurisdiction sensitivity and an owner review record all remain
//! distinct typed values (`docs/20:3-27`).

#[path = "Spdx.rs"]
pub mod spdx;
#[path = "Eula.rs"]
pub mod eula;
#[path = "Grant.rs"]
pub mod grant;
#[path = "Restriction.rs"]
pub mod restriction;
#[path = "Review.rs"]
pub mod review;
