# Agent contract for local autonomous optimization

This is a **prompt/playbook design**, not an assertion that autonomous agent
coordination is already implemented.

## Mission card

Every agent assignment must include:

- `campaign_id`, `agent_id`, `task_id`, `attempt`, `parent_mutation_id`,
  `parent_commit`.
- `allowed_paths`, `allowed_tools`, `worktree_path`, `budget`, `deadline`,
  `cpu_allocation`.
- `hypothesis_id`, falsifiable hypothesis, targeted stage and expected direction
  of effect.
- `required_correctness_contract`, reference oracle version and untouched
  fixture IDs.
- `feature_combo_id`, build recipe, expected benchmark workloads and acceptance
  thresholds.
- `preexisting evidence`, including failed prior experiments in the same
  optimization lane.
- `security policy`, specifically prohibiting uncontrolled shell execution
  against personal data.

## Strict mutator instructions

1. Read the source path and affected unit's contract. Do not edit source until
   the intended causal mechanism is explicit.
2. Make exactly one conceptual implementation change; do not slip in a different
   optimization or a test change to pass a gate.
3. Avoid touching benchmark fixtures, baseline results, or approved semantic
   oracles. If an oracle needs updating, file a separate proposal requiring
   review.
4. Document any behavior whose equivalence cannot be established and mark
   candidate conditional/experimental.
5. Add targeted internal tests without weakening existing tests.
6. Run permitted format/lint/build/local tests only. Do not run or benchmark
   destructive user commands.
7. Commit the changes with a message referencing `mutation_id` and
   `hypothesis_id`.
8. Return a structured final artifact: `mutation_id`, commit SHA, patch digest,
   modified paths, risks, tests, reported limitations, and proposed next
   experiment.
9. Do not self-promote the candidate. Only coordinator-owned independent testing
   can mark a result as a win.

## Strict runner instructions

1. Confirm host capabilities, resource lease, immutable source/bin hashes,
   runner OCI digest, environment manifest, fixture hash and oracle approval.
2. Prepare the fixed runner and disposable fixtures before measurement.
3. Use the requested seeded paired order and keep all samples, including
   failures and contaminated trials.
4. Do not benchmark multiple candidate families on shared hardware
   simultaneously.
5. Do not change CPU frequency/IRQ settings without preapproved host policy and
   authorization.
6. On interference or thermal drift: quarantine, record reason, request
   rescheduling, do not edit data.
7. Persist samples first, then mark experiment complete via a coordinator
   transaction.

## Strict analyst instructions

1. Compare a candidate only to its protocol-compatible reference on matched
   hardware/fixture cohorts.
2. Include candidate planner overhead and executable startup.
3. Predeclare thresholds and statistical methods. Never tune stop rules after
   seeing data.
4. Preserve raw samples and negative evidence; explicitly mark low
   power/inconclusive results.
5. Require external validation before promotion; no training from sealed
   holdout.
6. Provide a machine-readable verdict with effect size, uncertainty, critical
   regressions, correctness result, and traceable evidence links.

## Recovery and escalation

- `missing_fixture`, `invalid_oracle`, `dirty_git_snapshot`, `incompatible_isa`,
  `sandbox_violation` => stop and report, never fabricate success.
- `transient_build_failure`, `temporary_runner_busy` => retry within budget,
  preserving attempts.
- `timing_environment_noisy` => quarantine data, reschedule with fresh baseline.
- `semantic_contract_unclear` => block mutation promotion pending contract
  characterization.
- `unexpected_large_gain` => verify child actually executed correct number of
  times, output and side effects; repeat on independent fixtures.
- `agent_unresponsive`, `disk_pressure`, `host_health_alarm` => checkpoint and
  pause campaign.
