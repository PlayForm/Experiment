# The only LLM-visible wire protocol: no wire at all

**This protocol consists of local files in a container that has no network.**
The agent does not know the controller endpoints, signed event catalog,
telemetry plane names, CA, DNS, research exchange, embargo graph, recipient
list, or internal archive IDs.

Model-visible input is `/workspace/task.json`, validated against
`schemas/v61-offline-task.schema.json`. A simple repository exists read-only at
`/workspace/repo`. Optional local approved references and feedback are ordinary
files; absence of a file does not reveal an embargo or internal policy reason.
Allowed Rust tooling is local and its outputs have no authority outside the
worker.

Output is `/workspace/out/submission.json`, validated against
`schemas/v61-offline-submission.schema.json`, referring only to a fixed set of
local artifacts under `/workspace/out`. The trusted supervisor reads it after
process completion, canonicalizes paths, rejects symlinks/hardlinks and paths
escaping scratch, checks quota, checks base source hash and scope, stores
ciphertext in the correspondence vault and records an ingestion event. Worker
receives the same neutral acknowledgment class regardless of the reason a
proposal may later be rejected by trusted software.

The task problem can ask the model to inspect code, run a local test, propose a
patch, or articulate a hypothesis. It never asks the model to control the lab,
assign lessons, pick confirmation tests, approve transfers, allocate a budget or
publish artifacts. `claim_of_success` is explicitly not authoritative.

No model-to-model envelope exists. All proposed collaboration is staged as
artifacts that can be selectively released through the trusted custody and
publication bridge at a later time. For a task with no entitlement to a peer's
document, that document is not mounted and its digest/filename is not leaked.

A forbidden local action cannot always be perfectly indistinguishable from an
unavailable file or missing tool, particularly through timing/side channels.
Enforce deny-by-default and avoid revealing policy-specific messages; never
forge a verification pass to hide a failure.
