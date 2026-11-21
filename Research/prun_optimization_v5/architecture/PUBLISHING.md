# The Scientific Publishing Institution

## 1. Publish observations, not marketing claims

Atomic mutations generate **micro-research dossiers** (one claim with one tested
mechanism). Periodically consolidate related dossiers into **research papers**
without losing atomic evidence. A paper is a versioned, signed technical
artifact, not automatically a scientific peer-reviewed publication. Each claim
has a citation to exact source lines or external references, a measurement
protocol, and immutable evidence IDs; all derived prose carries an
evidence-status label.

## 2. Publication status ladder

1. `hypothesis`: preregistered prediction and mechanism; no empirical support.
2. `internal_observation`: observation or trace; possibly unreplicated.
3. `oracle_checked`: semantic correctness established under defined tests (not
   proof of all behavior).
4. `measured`: performance data attached with cohort/protocol.
5. `replicated`: independent confirmation across at least a separate
   experimental run, optionally another host/model/data cohort; criteria
   explicit.
6. `sanitized_preprint`: independent publication review, confidentiality pass,
   signed public/private release manifest.
7. `externally_reviewed`: editorial status only when genuine outside evaluation
   has occurred; no fabricated journals, DOIs or credentials.
8. `corrected|retracted|superseded`: immutable amendment records, never
   destructive deletion.

A null result can advance all the way to `sanitized_preprint` when correct and
informative. A positive result does not deserve publication more than a
well-supported failure.

## 3. Paper assembly pipeline

`Research notes (encrypted per researcher)` -> `claim extraction` ->
`source/reference librarian` -> `evidence linker` ->
`independent methodology critic` -> `optional fresh replication` ->
`licensed source/code/prompt sanitizer` -> `editor` -> `signer` ->
`private corpus/public registry`.

The publication institute is a **privileged recipient**, not a passive observer
who decrypts everything. Authors submit explicit research capsules encrypted for
the institute. It may request missing evidence, but does not gain a universal
decryption key. If the author refuses to submit sufficient authorized evidence,
publication is blocked/limited to an unverified hypothesis.

Every paper contains a machine-readable claim graph with status, claim IDs,
citation IDs, metrics, scope boundaries, known counterexamples,
confidence/uncertainty, failed attempts, model provenance and costs (aggregated
if appropriate).

## 4. Deliberation privacy vs scientific transparency

Private agents keep private reasoning notebooks, prompts, tool transcripts,
hypotheses and communications. A paper discloses enough _method, implementation
and evidence_ to be independently reproduced by authorized readers. Raw private
deliberations need not be published. An encrypted internal artifact can remain
access-controlled, with public citation represented by salted or
content-addressed commitments where permitted; a hash alone cannot establish the
truth of an unrevealed claim.

Sensitive scans must check secrets, client paths, proprietary source, repository
licensing, personal data, code snippets, credentials, agent instructions and
prompt injection from fetched sources. No auto-publishing by default for unknown
external repositories. For public open-source repos, licensing is checked and
publishing still needs project owner-defined policy. Static leak scanning is
helpful but not a guarantee; route uncertain cases to human review. Publication
events require a `declassification_grant` and an issuer identity distinct from
researcher, benchmark verifier and integrator.

## 5. Evidence quality

Explicitly distinguish `reference` (external literature, documentation) from
`observation` (trace/benchmark), `inference` (model-derived claim),
`interpretation` (opinion), and `replication` (new independent experiment). Do
not treat search snippets as authoritative without checking the linked source.
Store source URL, retrieval timestamp, title, author (if known), content hash,
usage/license scope, and exact claim supported. Use quote length limits and
respect external intellectual property.

Negative results, unproductive research, disagreements and paper corrections
remain first-class objects. A "paper" that contains no independently verifiable
results must visibly say `hypothesis note`.

## 6. Paper format

- Title, ID, created/revised UTC, visibility, authorship by agent identity,
  model(s) and campaign.
- One-sentence abstract with calibrated scope.
- Motivating repository, stage, workload, hardware cohort and baseline hashes.
- Tested mechanism, preregistered prediction and test selection procedure.
- Source diff and compile recipe lineage.
- Correctness oracle and all declared exceptions.
- Methods (warmup, cache state, CPU pinning, pairing, environmental telemetry).
- Raw results with uncertainty and reported regressions, not only positive
  speedup.
- Rival hypotheses and critic disagreements with separate attributed opinions.
- Independent replication status, no 'proof' beyond evidence.
- Reproducibility receipt and command recipe (or confidentiality constraints).
- Limitations, negative outcomes, threats to validity, future work.
- References with retrieved-source evidence hashes and no fabricated citations.
- Sanitization report, signer, signature scope and correction chain.

## 7. Cross-repository transfer

A publisher exports an **abstract algorithm knowledge card** without source
secrets. A separate integrator searches for applicable stages and writes a _new_
preregistered hypothesis for the target repository. It cannot reuse the source
repository's measured effect as target performance evidence. The transfer
mechanism records both IDs and eventually publishes a transfer reliability score
after independent testing.

A paper can be useful without source-sharing if it publishes an abstract
principle; such transfer claims need their own measured validation and may
remain purely conceptual.
