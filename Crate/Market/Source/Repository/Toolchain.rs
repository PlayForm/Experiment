//! Pinned toolchain and builder image of an adapter.
//!
//! Grounded in `repository-adapter.v5.schema.json:59-79` (name, version,
//! `sha256:`-prefixed builder image digest) and `REPOSITORIES.md:19` (hermetic
//! build recipes, image digests, build platforms and compiler flag dimensions).
//! A toolchain is validated data; no image is pulled and no compiler is run.

use crate::{Digest, Warning};

/// The pinned build environment of one adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Toolchain {
    /// Toolchain name (`schema:62-63`).
    pub name: String,
    /// Pinned version (`schema:64-65`).
    pub version: String,
    /// Content-addressed builder image (`schema:73-76`).
    pub builder_image: Digest,
    /// Name of the language family this toolchain implements.
    pub language: super::Language,
}

impl Toolchain {
    /// Validate the two required names (`schema:61`).
    pub fn validate(&self) -> Result<(), Warning> {
        match (self.name.is_empty(), self.version.is_empty()) {
            (false, false) => Ok(()),
            (true, _) => Err(Warning::InvalidValue {
                field: "toolchain name",
                detail: "a toolchain name is required".to_owned(),
            }),
            (_, true) => Err(Warning::InvalidValue {
                field: "toolchain version",
                detail: "a pinned version is required".to_owned(),
            }),
        }
    }
}
