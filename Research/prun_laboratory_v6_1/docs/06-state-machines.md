# Deterministic state-machine specification

The exact state and transition vocabulary lives in `policy/state-machines.yaml`.
The model has no API to invoke these transitions. Every edge is triggered by one
of: an authenticated owner command, a signed trusted event, a timed software
rule, or a deterministic validator. Free-form natural language is never a state
transition input.

## Transitions and authority

### campaign

| From        | To          | Sole authorized guard / trigger    |
| ----------- | ----------- | ---------------------------------- |
| `draft`     | `validated` | `policy-service`                   |
| `validated` | `active`    | `coordinator-on-owner-command`     |
| `active`    | `paused`    | `coordinator-on-owner-command`     |
| `paused`    | `active`    | `coordinator-on-owner-command`     |
| `active`    | `draining`  | `coordinator-on-owner-command`     |
| `draining`  | `completed` | `coordinator-on-all-jobs-terminal` |
| `active`    | `failed`    | `coordinator-on-critical-fault`    |
| `paused`    | `cancelled` | `coordinator-on-owner-command`     |
| `active`    | `cancelled` | `coordinator-on-owner-command`     |

### mutation

| From            | To              | Sole authorized guard / trigger |
| --------------- | --------------- | ------------------------------- |
| `proposed`      | `admitted`      | `scope-validator`               |
| `proposed`      | `rejected`      | `scope-validator`               |
| `admitted`      | `built`         | `builder-attested`              |
| `built`         | `oracle_failed` | `oracle-attested`               |
| `built`         | `oracle_passed` | `oracle-attested`               |
| `oracle_passed` | `measured`      | `runner-attested`               |
| `measured`      | `evaluated`     | `evaluator-attested`            |
| `evaluated`     | `eligible`      | `policy-engine`                 |
| `evaluated`     | `rejected`      | `policy-engine`                 |
| `eligible`      | `promoted`      | `release-gate`                  |
| `oracle_failed` | `archived`      | `coordinator`                   |
| `rejected`      | `archived`      | `coordinator`                   |

### worker

| From          | To            | Sole authorized guard / trigger |
| ------------- | ------------- | ------------------------------- |
| `idle`        | `assigned`    | `scheduler-lease`               |
| `assigned`    | `running`     | `supervisor`                    |
| `running`     | `staging`     | `supervisor`                    |
| `staging`     | `idle`        | `supervisor-on-artifact-seal`   |
| `running`     | `failed`      | `supervisor`                    |
| `assigned`    | `failed`      | `supervisor`                    |
| `failed`      | `quarantined` | `host-verifier`                 |
| `quarantined` | `terminated`  | `operator`                      |

### benchmark

| From           | To             | Sole authorized guard / trigger |
| -------------- | -------------- | ------------------------------- |
| `queued`       | `leased`       | `scheduler-exclusive`           |
| `leased`       | `running`      | `runner-start`                  |
| `running`      | `sealed`       | `runner-evidence`               |
| `running`      | `contaminated` | `runner-integrity`              |
| `sealed`       | `evaluated`    | `evaluator-attestation`         |
| `contaminated` | `queued`       | `coordinator-retry-new-attempt` |

### publication

| From                   | To                     | Sole authorized guard / trigger         |
| ---------------------- | ---------------------- | --------------------------------------- |
| `draft`                | `private-reviewed`     | `review-authority`                      |
| `private-reviewed`     | `sanitization-pending` | `publisher`                             |
| `sanitization-pending` | `sanitized`            | `sanitizer`                             |
| `sanitized`            | `internal-released`    | `publication-authority`                 |
| `internal-released`    | `public-released`      | `publication-authority-on-owner-policy` |
| `draft`                | `rejected`             | `publication-authority`                 |
| `sanitization-pending` | `rejected`             | `sanitizer`                             |

## Universal transition transaction

1. Validate caller credential, assigned service role, request schema,
   campaign/policy scope, expiry and record signature.
2. Read current aggregate revision and last event hash in a transaction.
3. Validate transition `(from,to)` and guard from pinned policy and evidence
   IDs.
4. Check idempotency key; same key+same payload returns original result, same
   key+different payload is conflict.
5. Append immutable event, update state projection and outbox atomically;
   commit.
6. Consumers may fail/retry. They must never create a second effect from the
   same event/command ID.

## Lease rules

`lease_id + fencing_token + resource_id + owner_job_id` are binding. A timed-out
lease increments fencing token before another worker is allowed to produce
authoritative output. On restart, do not assume an uncommitted external action
never occurred; seal attempt IDs and treat irreconcilable outcome as
inconclusive/quarantined, not automatic success.

A held benchmark lease must be exclusive over the declared physical resource
topology, not simply a Docker `--cpuset-cpus` flag. A contaminated run returns a
distinct state and stays in the ledger.

## Feedback and loop closure

Feedback is prepared only after authoritative oracle/evaluation outcomes. A
worker's new patch becomes a new mutation/attempt, not an overwrite. The
campaign can continue without any LLM workers, while no new agent-originated
mutation is generated.
