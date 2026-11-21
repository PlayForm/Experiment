# Embargoes, EULAs, licenses, trade and segregation

## Signed supply-chain policy

Repository ingress produces a `v61-repository-admission` record with pinned
commit, origin, verified owner policy, toolchain/dependency closure, classified
files, license review and required sandbox profile. Only owner-admitted source
can be used as a task snapshot. Automated scanning may flag license terms and
obligations but **cannot determine contractual rights in all cases**. Missing,
ambiguous, incompatible or jurisdiction-sensitive terms become
`needs-human-review` or `prohibited` and fail closed for that use.

- Store verbatim governing terms and version/digest in the protected vault;
  expose only the permitted task summary to a model.
- Record SPDX license expressions including `AND`, `OR`, `WITH` where
  applicable; preserve source-level LICENSE/NOTICE texts and component-level
  rights, not only a repo root label.
- EULA obligations and grants are distinct from an SPDX identifier. A private
  EULA may constrain modification, model-training use, redistribution,
  publication, transfer or jurisdiction.
- Disallow copying GPL-derived material into incompatible proprietary targets
  under default conservative policies; exception decisions require a separately
  approved license-review record. This is a compliance workflow, not a
  legal-opinion engine.
- Preserve source and derivative attribution and upstream provenance in release
  artifacts and knowledge publications; a sanitized summary cannot be assumed
  rights-free.

## Embargo as policy-object, not a hidden LLM prompt

`v61-embargo-rule` binds artifact/repo/track, recipient groups, allowed
metadata, clock validity, prohibited purposes and owner authorization. The
trusted broker consults it before context compilation, archive access, intra-lab
research dissemination, paper publication and repository transfer. Agents do not
see it. Revocation/changes use policy versions and event-time checks; they never
retroactively grant a prior transfer.

## Trade/conflict graph

The default is _no lateral trade_. A `v61-trade-restriction` can establish A→B,
B→A or mutual blocks for `anti-collusion`, `holdout`, `license`, `cross-tenant`,
or `conflict-of-interest` reasons. Graph edges are enforced even for a seemingly
harmless citation because existence metadata may leak a sibling's hypothesis.
Approved releases move only via the declassification publisher, never between
agents directly.

- Limit a group's dominance over future assignments using signed **research
  assignment constraints**, not LLM rankings or bids; this is fairness/diversity
  policy and is not part of runtime performance scoring.
- Model lineage fingerprints flag correlated model families; they cannot prove
  independent thinking.
- Researchers may disagree in private and publish contradictory signed claims;
  the lab preserves both and reports objective measurements separately.
- A public paper's right to be cited is not automatically a right to
  redistribute implementation code, model weights, or private datasets.

## Enforcement patterns

| Condition                         | Trusted response                                 | Agent-visible response                                           |
| --------------------------------- | ------------------------------------------------ | ---------------------------------------------------------------- |
| Expired license grant             | Deny transfer, audit, quarantine if staged       | No policy reason; normal local absence if source wasn't provided |
| Embargo blocks paper              | Keep draft private, record denied release        | No indication to another agent                                   |
| Peer disallowed by trade graph    | Don't construct shared context, audit attempt    | No indication of peer identity                                   |
| Recipient can't store binary      | Do not stage, reschedule eligible holder         | No storage topology message                                      |
| Unknown source rights             | Require owner/license review, stop dissemination | Task simply excludes source                                      |
| Attempt to inspect sealed holdout | Deny and alert owner in restricted audit         | Generic local tool error or no mount                             |

A system that suppresses all errors internally would be unsafe; _worker-blind
enforcement_ means confidential details are absent from worker feedback, while
operational status and incident response remain fully observable to authorized
humans.
