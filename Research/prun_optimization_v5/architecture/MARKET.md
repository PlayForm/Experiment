# v5 Research Market — Scarcity, Bids, Opinions, and Reward Design

## 1. Resource model

Capital is a **non-transferable, non-cash research allocation entitlement**
backed by hard budgets. There is no unbacked agent-to-agent money creation. Each
campaign has an owner-funded resource vector:

`(inference_input_tokens, inference_output_tokens, cpu_core_seconds, exclusive_benchmark_seconds, gpu_seconds, storage_byte_days, outbound_reference_requests, reviewer_seconds, false-discovery/holdout disclosures)`.

The last dimension is a scientific access quota, not money: exposure to sealed
holdouts is restricted by experimental policy, never purchasable. Disallowed
actions cannot be bought with extra credits.

Book physical resources in their native units. Optionally create _shadow prices_
or a planning denomination called `research_credit`, but no single exchange rate
perfectly substitutes for an exclusive benchmark CPU or rights to read a private
test. Use fixed-point integers (not binary floating-point) for accounting and
cost caps; track a versioned price schedule.

## 2. The research opportunity marketplace

The coordinator publishes tasks such as 'explain slow traversal on NVMe',
'replicate 4% gain on ARM', 'find counterexample to scheduler hypothesis',
'audit negative paper for duplication'. Offers specify budget ceiling, scope,
deliverables, expected evidence grade, time limit, confidentiality class and
reward rules.

A prospective agent submits a signed bid with:

- `requested_resources` as an upper bound per dimension;
- explicit probability forecasts (e.g. chance that candidate passes oracle,
  chance that speedup >= 3% on development data) frozen before the outcome;
- a falsifiable mechanism, novelty fingerprint, estimated information gain and
  planned reference queries;
- a confidential commitment to its method and a declared viewpoint
  (proponent/skeptic/replication/etc.);
- a unique parent case and signed budget request ID.

**Two-step assignment:** first reserve guaranteed minimum capacity for
contrarian/replication tracks and underrepresented approaches; second rank
eligible bids for remaining capacity using calibrated expected value,
information gain and constraints, never a raw LLM confidence claim alone.
Enforce global, per-project, per-agent, per-model-family and per-topic caps.
Require cold-start exploration so agents do not starve novel hypotheses by
historical reputation.

## 3. Prices and scoring

The allocation objective is _expected useful knowledge per bounded cost_ subject
to mandatory fairness and correctness constraints, rather than maximizing patch
count.

Illustrative ranking (not a guarantee of incentive compatibility):

`priority = [0.40 × prior-calibrated information gain + 0.25 × expected user value + 0.15 × novelty + 0.10 × replication urgency + 0.10 × diversity credit] / [1 + normalized scarce resource cost]`

The weights are example configuration only. Do not reward novelty assessed by
the same model evaluating itself. Use deterministic duplicate fingerprints and a
reviewed similarity index. Re-score on a separate policy version and analyze
resource allocation as an experiment.

For precommitted binary outcome predictions, a **Brier score** is available:
`Brier(p, y) = (p-y)^2`, with 0 best. Use calibrated prediction quality as _one_
input to future admission priority, not as a moral or scientific status label.
The score cannot reward a researcher for corrupting the evaluation; measured
outcomes come from separate signed authorities. See Gneiting and Raftery (2007):
https://stat.uw.edu/research/tech-reports/strictly-proper-scoring-rules-prediction-and-estimation.

## 4. Ledger semantics

The treasury emits immutable **authorization, reservation, usage, settlement,
refund, expiration and manual adjustment** records, each with a causal
transaction ID, signed actor and rationale. The actual resource consumption is
measured by trusted metering, not reported by the agent. A reservation decreases
available capacity atomically. Settlement consumes actual resources up to its
reservation and returns unused capacity; overage is rejected or escalated. No
agent can transfer credits to another agent, mint credits, lend resources,
forgive debts or buy benchmark verdicts.

A researcher can have a personal **experiment budget envelope** while the
campaign owns the actual treasury balance. Researchers may express subjective
priorities with bids but do not acquire unrestricted spending authority.

## 5. Avoiding Goodhart loops and pseudo-independence

- Award credit for useful falsifications, robust null results, uncertainty
  reduction, reproducible failure cases, verified references and preventing
  redundant experiments.
- Do not pay per generated paper, LOC changed, tokens emitted, number of
  experiments, or benchmark speedup alone.
- Preregister threshold/metric before trial; freeze the primary comparison and a
  split of fixtures; limit adaptive peeks at holdouts.
- Use paired randomized timing, repeated confirmation and separate host/case
  replication. Many hyperfine iterations on the same fixed machine are not
  independent machine-level replications.
- Same base LLM with different system prompts is not independent sampling of
  models: record model lineage and covariance groups. Use different
  seed/prompt/context; alternate model family when available and independent
  deterministic evidence verification regardless.
- Run a **protected dissent budget** so cheap majority consensus cannot silence
  a critical skeptic.
- Use a non-discretionary minimum budget for negative results and replication;
  cap repeated almost-identical proposals.
- No agents can vote on their own paper, edit evaluator policies for their own
  experiments, or see peer draft conclusions in the blind phase.

## 6. Auction style and limitations

Initially use a deterministic, constrained **priority allocator** (not call it a
truthful auction). An internal sealed-bid, multi-dimensional resource mechanism
is not automatically strategyproof; if real economic stakes develop, mechanism
design and independent audit become necessary. Optional virtual prices make
costs legible, but hard physical admission limits remain authoritative.

## 7. Epochs and stopping rules

Allocate in epochs (e.g. 1 hour per research round, 24 hours per campaign
generation). Budget expires at epoch boundary unless explicitly rolled over
under policy. Stop a search when expected value of information drops below
marginal cost, correctness fails repeatedly, candidate novelty is exhausted, or
contamination/holdout leakage makes evidence invalid. Resources return to the
treasury; no recursive self-spawning without signed admission.

## 8. Evaluation KPIs

Useful confirmed insight / token; confirmed improvement / benchmark host-hour;
reduction in redundant trials; calibration / Brier by model cohort; independent
replication rate; publication correction rate; null result coverage; portfolio
diversity; false promotion rate; median time to verified answer; value of
transfer validated on _new_ repositories. Publish failures alongside wins.
