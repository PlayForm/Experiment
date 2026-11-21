# In-memory database, sealed lineage and recoverable migrations

Arenas optimize allocation and locality; they are not confidential vaults or
durable databases. Use an in-memory read model with keyed indexes, guarded
lifetimes and snapshots, backed by **encrypted append-only evidence** and
transactional durable metadata. An option for an all-Rust embedded persistence
layer is `redb`, with encryption occurring in a separate application-controlled
layer; do not assume its default database file is encrypted.

Storage compartments:

- **Schema vault:** canonical private IR, versioned migrations and encrypted
  crosswalks; trusted schema factory and specific authorized auditors only.
- **Key vault:** key handles / wrapped keys; cryptographic operations behind a
  non-exporting broker, ideally hardware-protected.
- **Custody ledger:** encrypted event records, previous-hash commitments,
  signatures, nonces, authoritative timestamps and replay fencing.
- **Warm registry:** in-memory immutable snapshots of validated schemas,
  resolver handles and policy projections; no primary secrets outside authorized
  cryptographic processes.
- **Observation store:** redacted metrics and trace projections with separate
  retention; not authoritative evidence and not a way to reverse private
  mappings.
- **Offline worker scratch:** disposable, read/write scoped to task, with no
  privileged material and no network.

For a traceable genesis, publish/store a signed genesis _commitment_: hash of
the initial schema/policy manifests, issuance policy, initial event log head and
allowed migration path. It is safe as an audit anchor. It is not the encryption
seed.

Each migration is a signed transition
`(previous_state_digest, new_schema_digest, policy_digest, compiler_digest, migration_id, epoch)`
recorded to the journal. The controller performs write-ahead prepare, validates
invariants, atomically promotes, and records final activation. Rollback requires
a new authorized transition; it is not allowed to erase historical journal
entries.

On restart: verify journal signatures, sequence and previous-hash chain; reject
rollback/forks absent explicit recovery policy; restore a sealed snapshot and
replay the remaining committed records; then compile the warm in-memory
registry. A backup must contain recoverable encrypted records, keys escrowed
separately and rotation history. Provide specific tests for power loss after
prepare, before activation, and during key rotation.

A private observer can see operational health without plaintext of the schema
registry. A human authorized to restore or audit the system may necessarily have
access to more information than a standard observer; separation of duties and
audited break-glass workflows should be explicit.
