//! Parsed CPU affinity sets.
//!
//! Grounded in `prun_lab.py::parse_cpuset` (486-498): comma-separated single
//! cores and inclusive ranges (`2-5`), with empty members, descending ranges
//! and non-numeric parts rejected as typed faults.

use std::collections::BTreeSet;

use crate::Fault;

/// A set of CPU core indices subject to Docker affinity.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CpuSet {
    /// Parsed core indices in ascending order.
    pub cores: BTreeSet<u32>,
}

impl CpuSet {
    /// Parse a cpuset expression (prun_lab.py:486-498).
    pub fn parse(text: &str) -> Result<Self, Fault> {
        let cores = text
            .split(',')
            .try_fold(BTreeSet::<u32>::new(), |mut collected, part| {
                match part.split_once('-') {
                    Some((low, high)) => {
                        let first = low.parse::<u32>().map_err(|_| Self::bad(part))?;
                        let last = high.parse::<u32>().map_err(|_| Self::bad(part))?;
                        match first <= last {
                            true => {
                                collected.extend(first..=last);
                                Ok(collected)
                            }
                            false => Err(Self::bad(part)),
                        }
                    }
                    None => {
                        collected.insert(part.parse::<u32>().map_err(|_| Self::bad(part))?);
                        Ok(collected)
                    }
                }
            })?;
        Ok(Self { cores })
    }

    /// Whether any core is shared with another set (prun_lab.py:549).
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.cores.intersection(&other.cores).next().is_some()
    }

    /// Union of two sets.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            cores: self.cores.union(&other.cores).copied().collect(),
        }
    }

    /// Whether the set has no cores.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cores.is_empty()
    }

    fn bad(part: &str) -> Fault {
        Fault::Malformed {
            domain: "cpuset",
            field: "affinity".to_owned(),
            detail: part.to_owned(),
        }
    }
}
