//! The `plan` subcommand renderer.
//!
//! Grounded in `prun_lab.py:679-683`: one line per variant
//! (`id`, `git_ref`, `profile`) followed by a `builds x workloads` total.

use crate::plan::Variant;

/// A request to list the Cartesian matrix.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Plan;

impl Plan {
    /// One printed plan line (prun_lab.py:681).
    #[must_use]
    pub fn line(variant: &Variant) -> String {
        format!(
            "{:<50} {:<20} {}",
            variant.id,
            variant.reference.git_ref,
            variant.profile.cargo_name()
        )
    }

    /// Summary line (prun_lab.py:682).
    #[must_use]
    pub fn footer(builds: usize, workloads: usize) -> String {
        format!("Total {builds} builds x {workloads} workloads")
    }
}
