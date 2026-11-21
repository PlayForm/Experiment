# Offline LLM task capsules, correspondence and prompts

The model is not a laboratory service. It is an untrusted computation invoked by
a trusted supervisor. It cannot initiate a network request, publish a paper,
allocate benchmark resources, send a service message, read the CA, query GPUI
dashboards or inspect OTel. A human may operate GPUI, but the LLM cannot.

## Offline worker mount contract

| Mount / channel                  | Access by model    | Content                                                   | Auth semantics                                      |
| -------------------------------- | ------------------ | --------------------------------------------------------- | --------------------------------------------------- |
| `/task/request.json`             | read-only          | Task-local purpose and output format; opaque task nonce   | No tokens/hostnames/CA info                         |
| `/task/source/`                  | read-only          | Exactly authorized source subset and approved fixtures    | Snapshot digest checked by supervisor               |
| `/task/references/`              | read-only          | Explicitly authorized, pre-fetched research/citations     | Offline; license/source manifest                    |
| `/task/feedback.json`            | read-only if retry | Scoped test failure codes and allowed diagnostic extracts | Never sealed confirmation inputs                    |
| `/scratch/`                      | writable           | Patch, claims, proposed tests, draft paper, local notes   | Output is untrusted until supervisor sealed receipt |
| Model process network            | **none**           | No DNS, Internet, internal service addresses              | No IP connectivity                                  |
| Host Docker socket / control API | **none**           | No privileged sockets, no secrets                         | Disabled and unmounted                              |

The _trusted supervisor_, not the model, maps opaque task local IDs to global
campaign IDs, holds mTLS credentials, gathers wall-clock/inference metadata and
seals output. If the worker calls a local file search or compiler tool, it
happens through a restricted local tool surface and is recorded as a
`tool-call`. A process executing model-generated code remains untrusted even if
offline.

## Minimum pre-prompt (in worker image, not transport config)

```
You are modifying only the supplied source subset to address the enclosed task.
Preserve existing observable behavior. You may submit a unified patch and
structured observations/tests. Do not assume any unprovided system exists.
You are not entitled to claim benchmark, correctness or release results.
If the source is insufficient, report unknown and propose a bounded next test.
Return only the required artifact names and documented local JSON format.
```

A role-specific text can ask the model to be a proponent, skeptic, test author,
or independent researcher, but the role prompt cannot confer authority. The
worker's _actual input contract_ is the minimal task/problem and local file
view, not internal OpenAPI/AsyncAPI schemas, certificate policies or service
architecture.

## Context and turn lifecycle

`conversation_id`: logical agent work thread; `segment_id`: one contiguous
invocation/restart; `turn_id`: visible prompt/model/tool message;
`context_snapshot_digest`: exact input context for that turn; `tool_call_id`:
local allowlisted invocation; `task_id`: only association needed by supervisor.
Visible turn text and local tool arguments/results are encrypted per access
policy in CAS. Turn metadata never grants authority.

Resuming the same logical worker conversation means loading an authorized
explicit context snapshot (or signed recap marked derived); it **does not** mean
giving the model network access or another researcher's private messages. Task
feedback describes failed tests and authorized excerpts only.

## Research role diversity and control

Separate proponent, skeptic, independent mechanism, replication/test designer,
paper drafter and integrator task _capsules_. The software coordinator selects
assignments according to signed policy and blinded cohorts. All agents use the
same worker ABI and may use the same locally provisioned LLM. Opinion diversity
is never considered statistical independence of evidence.

## No model-to-model transport

Researchers cannot chat directly across containers. To debate, each submits a
research claim, a trusted knowledge registry records it, and the deterministic
coordinator may later mount selected declassified findings into a different
offline task. Provenance, censorship policy, visibility, and causation stay in
software.

## Task artifacts accepted

- `patch`: Unified diff or machine-readable source patch plus claimed changed
  symbols and base tree ID. No special shell commands.
- `research-claim`: falsifiable mechanism, conditions, contrary evidence,
  confidence and source artifacts.
- `test-proposal`: new cases and invariants, never trusted oracle edits.
- `draft-paper`: claim-to-evidence narrative and citations, private until
  independent sanitization.
- `integration-proposal`: transfer hypothesis for new repository, never
  automatic deployment.

## Explicitly prohibited

Model-suggested shell commands are **text**, never accepted as scheduler
instructions. No user/owner credential, no CA private key, no production
repository write mount, no arbitrary code evaluation outside untrusted
build/test sandboxes, no raw holdout and no privileged terminal/SSH.
