//! Disjoint CPU leases required by `--parallel-runners`.
//!
//! Grounded in `README.md:64-82` and `prun_lab.py:540-551`: every active
//! runner group must have its own `runner_cpusets` entry, the sets must parse,
//! and they must not overlap. The global `cpuset` is not sufficient here.

use std::collections::BTreeMap;

use crate::config::Invariant;
use crate::Fault;

use super::{CpuSet, Group};

/// CPU affinity assigned to one runner group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    /// Owning group.
    pub group: Group,
    /// Parsed cores reserved for that group.
    pub cpuset: CpuSet,
}

impl Lease {
    /// Assign disjoint leases to the given groups (prun_lab.py:540-551).
    pub fn assign(groups: &[Group], invariant: &Invariant) -> Result<Vec<Self>, Fault> {
        let dedicated: BTreeMap<&str, &str> = invariant
            .runner_cpusets
            .leases()
            .into_iter()
            .collect();
        groups
            .iter()
            .try_fold((Vec::new(), CpuSet::default()), |(mut leases, assigned), group| {
                let text = dedicated.get(group.tag()).copied().ok_or_else(|| Fault::Affinity {
                    detail: format!("missing runner_cpusets for {}", group.tag()),
                })?;
                let set = CpuSet::parse(text)?;
                match set.overlaps(&assigned) {
                    true => Err(Fault::Affinity {
                        detail: format!("overlapping runner_cpusets for {}", group.tag()),
                    }),
                    false => {
                        let merged = assigned.union(&set);
                        leases.push(Self {
                            group: *group,
                            cpuset: set,
                        });
                        Ok((leases, merged))
                    }
                }
            })
            .map(|(leases, _)| leases)
    }
}
