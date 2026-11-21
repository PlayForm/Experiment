# Offline worker ABI, tools, correspondence and silent policy enforcement

## Single visible interface to untrusted LLM

The task is a plain pinned Rust repository snapshot at `/workspace/repo`, a
short task at `/workspace/task.txt`, allowed local reference texts at
`/workspace/references`, optional prior own feedback at
`/workspace/feedback.txt`, and scratch `/workspace/out`. No service hostnames,
DNS resolver, credentials, internal schema, dashboard, policy, worker identity
graph or privileged environment are mounted. The trusted supervisor maintains
the mapping between local task nonce and internal task IDs.

**Permitted tools**, if compiled into the pinned worker image: `rg`, file
reader, `rustfmt`, `cargo check`, `cargo test` on local untrusted inputs with
offline dependencies, and bounded `git diff` in a throwaway worktree.
`cargo test` output is merely local feedback; only the independent oracle runner
establishes authoritative correctness. Run source code and build scripts as
untrusted execution with appropriate container/VM policy, not on the trusted
host. Symlink escapes, procfs leaks, writable cargo home, compiler wrapper
hooks, inherited credentials, env var leaks and build.rs outbound/network
attempts must be mitigated.

**Submitted files**: `change.patch`, `claims.json`, `proposed-tests/` (not
trusted test mutations), `paper-draft.md` where assigned. A trusted supervisor
validates path, content/type/size, base source digest, scope and manifest; it
seals an immutable output receipt. Model-proposed command text is never
interpreted as coordinator control input.

## Correspondence record and provenance

For every visible turn, archive: `conversation_id`, `segment_id`, `turn_id`,
role/task-local ID, model family/build digest, prompt-template revision, bounded
context snapshot digest, input envelope encrypted CAS digest, output envelope
encrypted digest, tool-call names + encrypted args/result refs, timeouts,
reasoning-visible content if deliberately output, restart/summarization
provenance, effective view ID, supplied source hash, and supervisor-sealed
receipt. Do not claim to store hidden model reasoning or internal
weights/activation unless explicitly captured as a separate authorized research
artifact.

Model output is not an authorized agent-to-agent message. An exchange between
sibling research tracks is possible only after a trusted
publication/declassification transition; direct sockets and shared volumes are
prohibited.

## Quiet enforcement without deceptive results

The user requested that violations be invisible to the agent. Implement
**non-disclosing error boundaries**:

- Do not send an event saying `embargo violated`, `trade denied`,
  `access revoked`, `policy blocked` or details of sibling identities to an LLM.
- A local file that was never mounted is unavailable, exactly like any absent
  file. A forbidden local tool operation returns a fixed, non-identifying status
  chosen before deployment.
- A submitted task receives an opaque receipt that literally means **only**
  `submission_received`. Do not say `accepted`, `tests passed`,
  `benchmark completed` or `published` unless that fact is independently true
  and authorized to reveal.
- Failed submissions do not mutate trusted state. The supervisor writes a
  restricted denial event; the coordinator may safely terminate/quarantine or
  issue another independent task according to software policy.
- Keep response payload shape fixed for a class of failures; avoid disclosing
  which internal rule fired via detailed errors, varying diagnostic text,
  special status codes, or different file names. Exact timing
  indistinguishability is not generally achievable and must not be promised.
- Human operators see actual failure reasons, incident priority and policy
  decision hash in protected audit UI. Critical incidents are never silently
  ignored by the lab.

## Tool/worker failure classes

`local-compile-error`: may show useful compiler lines if allowed.
`local-test-fail`: can show local test output. `submission-invalid`: neutral
worker receipt and restricted audit. `policy-denied`: deny/quarantine without
detailed worker notification. `transport-failure`: supervisor may retry with
same idempotency token. `trusted-oracle-fail`: bounded semantic failure feedback
subject to disclosure policy. `runner-contaminated`: retried by scheduler, not
blamed on agent. `infrastructure-unavailable`: no fabricated benchmark result.

## LLM image is not the lab

A preinstalled local model/runtime is allowed; no remote API or egress. A
separate trusted supervisor owns task orchestration and reads outputs. If the
inference backend itself requires IPC, that interface is local and can only
relay the bounded task content; the model has no direct service authority. Avoid
untrusted plugin tools that can themselves access network or sensitive host
paths.
