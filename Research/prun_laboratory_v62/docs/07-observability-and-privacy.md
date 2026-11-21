# Double observability with a private mapping boundary

Existing planes remain: Plane 1 collects laboratory event/queue/worker/timing
status; Plane 2 collects separately measured product profiling and benchmark
evidence. Plane -1 holds private semantic mappings and key-policy activity with
its own restricted audit pipeline. Plane 0 is a _projection_ that returns only
authorized public/operator views.

Default OTel fields are neutral: `trace_id`, `span_id`, `cohort_pseudonym`,
`operation_class`, `duration_ns`, `result_class`, `container_image_digest`,
`policy_epoch_bucket`, `attestation_digest`. Do not export repository paths,
prompt text, schema refs, feature names, private identities, raw errors,
certificate subjects or decrypted event types. Even an opaque stable ID can link
activities; rotate per tenant/view/epoch and guard the crosswalk.

The trusted private ledger records detailed reasons, explicit policy IDs,
provenance and correlated artifact references, encrypted at rest and signed.
Observers receive aggregate counts and access-controlled incident status without
private mappings. Disclosure of a private mapping must itself be logged and
policy checked.

Emitting an event of fixed size/timing is not enough to prevent activity
inference from container starts, CPU usage, disk I/O or routing patterns. If
these side channels are in scope, use controlled batch dispatch, padded
encrypted messages and dedicated hosts. There is no plausible absolute traffic
secrecy against a host administrator on an ordinary Docker machine.

No LLM worker sees any observability plane or the hidden mapping plane. It can
see its own compiler/test output and scoped feedback; denying this would prevent
useful code implementation. Never send internal policy-denial reasons to workers
and never fabricate success reports.
