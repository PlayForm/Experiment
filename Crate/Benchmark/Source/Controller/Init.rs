//! The `init` subcommand.
//!
//! Grounded in `README.md:50` and `prun_lab.py:669-674`: it writes the default
//! config once and refuses to overwrite an existing one.

use std::path::PathBuf;

use crate::Fault;

/// A request to create the default config document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Init {
    /// Target config path.
    pub config: PathBuf,
}

impl Init {
    /// Guard against clobbering an existing config (prun_lab.py:670-671).
    pub fn guard(exists: bool) -> Result<(), Fault> {
        exists
            .then(|| Fault::Malformed {
                domain: "init",
                field: "config".to_owned(),
                detail: "config already exists".to_owned(),
            })
            .map_or(Ok(()), Err)
    }
}
