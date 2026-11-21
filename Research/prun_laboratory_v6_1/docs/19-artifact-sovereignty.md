# Artifact sovereignty, custody, capacity and storage

## Taxonomy

Artifacts include: `source-snapshot`, `patch`, `test-proposal`,
`trusted-oracle`, `fixture`, `holdout`, `build-script`, `binary`,
`compiler-diagnostic`, `benchmark-sample`, `profile`, `research-opinion`,
`private-correspondence`, `prompt`, `paper-draft`, `sanitized-paper`,
`model-checkpoint`, `license-text`, `policy`, `credential-public`,
`credential-private`. Each artifact is identified by ciphertext digest, optional
securely verified plaintext digest, media type, origin identity,
repository/tenant, license/EULA references, confidentiality class, retention and
legal hold, use restrictions and immutable provenance.

**Do not confuse an artifact's digest with its decryption capability.** A CAS
index can be globally content addressed without authorizing plaintext
disclosure. Use tenant/label scoped envelope encryption keys and view-specific
grants. Where deduplication could reveal sensitive existence information, use
per-tenant namespaces/blinded indexing instead of exposing a global digest
lookup.

## Transfer protocol — two-phase custody (not a distributed magical atomic commit)

1. `Inventory`: target supervisor attests environment, supported media, accepted
   classification, available capacity, encrypted-at-rest capability, storage
   class, key scope, and expiry. Report has a signed freshness deadline; do not
   believe an LLM's capacity claim.
2. `Preflight`: custody broker evaluates
   `(principal, action, artifact, purpose, context)`, SPDX/EULA review, embargo
   graph, repository/tenant boundary, retention, recipient tool support, valid
   policy epoch, current grants, and network/storage restrictions.
3. `Reserve`: atomically reserve bytes/lease and bind a one-use transfer nonce,
   source digest, target holder, purpose, policy digest, cap report and fencing
   token. If denied, trusted audit is appended; no worker sees the decision
   rationale.
4. `Stage`: trusted source encrypts content for the approved target/temporary
   key and puts it in quarantine. An LLM cannot initiate or address this
   delivery. No source tree or paper is staged before authorization.
5. `Verify`: trusted receiving supervisor checks size, cipher/plaintext hash in
   appropriate trust domain, media type, lease and recipient identity,
   malware/content policy as applicable, expiry, license obligations and
   unexpected links, escapes or symlinks. Never unpack archives with unchecked
   paths or symlinks.
6. `Commit`: append custody receipt and inventory update atomically in the
   trusted database. Only now mount an approved read-only view. On crash or
   timeout, fence and quarantine staged bytes; reconcile with the journal rather
   than invent success.
7. `Release`: end task, revoke key, unmount, encrypt/archive or crypto-shred
   according to retention/legal-hold rules. Revocation cannot erase knowledge
   already consumed by a worker, nor guarantee deletion from backups or
   side-channel copies.

## Holder qualification and denied combinations

| Holder                | May hold                                                              | Must never hold                                                                           |
| --------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Offline patcher       | Approved source view, task, local feedback and scratch                | CA keys, trusted holdout, peer paper draft, runner raw results unless expressly projected |
| Offline researcher    | Source/reference extracts, own private context, approved public paper | Another researcher's private dialogue, embargoed research, cross-tenant unreviewed source |
| Builder               | Build input & approved toolchain/dependency closure                   | Oracle signing key, private agent transcripts                                             |
| Oracle                | Reference fixtures, candidate binaries, controlled test data          | Model-generated promotion authority                                                       |
| Runner                | Frozen binaries, fixtures, signed protocol & hardware lease           | Agent execution/credential material                                                       |
| Correspondence vault  | Encrypted agent turns and tool transcripts                            | Unencrypted content in default OTel projections                                           |
| Publication custodian | Signed claims, approved review extracts, disclosure policy            | Raw private contents absent explicit authorization                                        |
| Cold archive          | Encrypted artifacts, policy metadata, legal-hold receipts             | Uncontrolled decryption grants                                                            |

`CanHold` is not the same as `CanRead`, `CanExecute`, `CanDerive`,
`CanRedistribute`, or `CanTrainModelOn`. A holder may store encrypted bytes
while having no decryption entitlement. Toolchain capability must match artifact
type; e.g. a GPU-checkpoint vault is not equivalent to a read-only source
snapshot mount.

## Storage tiers

- `ephemeral-scratch`: task-scoped writable quota, no cross-task persistence,
  restricted file operations, no network; remove after sealing.
- `encrypted-cas`: immutable evidentiary blobs with tenant-scoped encryption and
  custody indexes.
- `offline-bundle`: read-only approved source/fixture/context view mounted into
  a worker.
- `sealed-holdout`: separate key and runner role; no feedback exports of
  underlying samples.
- `cold-archive`: signed, encrypted, backed-up records with retention and legal
  hold.
- `diagnostic-trace`: content-minimized OTel with TTL and no prompt or source
  payload.

## Storage failure and pressure

The scheduler must reject or defer staging when the receiving holder cannot
reserve sufficient free capacity, cannot decrypt safely, lacks file format
tools, violates data residency, or cannot satisfy retention. The failure may be
retried on another holder only after fresh policy evaluation. Never silently
truncate an artifact and call transfer complete.
