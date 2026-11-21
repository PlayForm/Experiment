//! The documented order of compression, encryption and hashing.
//!
//! Large files may be chunked into Merkle-addressed manifests with a verified
//! root; hashing order is part of the artifact's representation, not an
//! implementation detail (`docs/08-data-and-storage.md:33-38`).

/// One declared byte-to-digest representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Representation {
    /// Hash canonical plaintext inside an authorized enclave.
    PlaintextRoot,
    /// Encrypt independently, then hash ciphertext for transport.
    CiphertextRoot,
    /// Chunk into a Merkle tree and commit its verified root.
    MerkleRoot,
}

impl Representation {
    /// The documented operation order.
    #[must_use]
    pub fn order(&self) -> &'static str {
        match self {
            Self::PlaintextRoot => "hash canonical plaintext inside enclave, then encrypt",
            Self::CiphertextRoot => "encrypt independently, then hash ciphertext",
            Self::MerkleRoot => "chunk, hash leaves, commit verified root",
        }
    }

    /// Whether this crate may disclose a plaintext hash to an observer.
    /// Never: plaintext hashes of private data are dictionary-attackable.
    #[must_use]
    pub fn plaintext_disclosure(&self) -> crate::permission::Permission {
        crate::permission::Permission::Deny
    }
}
