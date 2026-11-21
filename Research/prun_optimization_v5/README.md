# PlayForm/Run v5 — Federated Research Economy

**Status:** versioned engineering specification, validated schemas/SQL, and an
**uncompiled** Rust reference auction/credit prototype. This is **not** an
operational autonomous research service. Builds on v1 Docker benchmark starter,
v2 provenance schema, v3 experiment/agent architecture, and the in-progress v4
trust prototype.

## Where to start

1. [architecture/SYSTEM.md](architecture/SYSTEM.md) — full research graph,
   ownership, roles, privacy and evidence boundaries.
2. [architecture/MARKET.md](architecture/MARKET.md) — monetary metaphor, credit
   model, allocations, auctions, incentives, risk and gaming.
3. [architecture/PUBLISHING.md](architecture/PUBLISHING.md) — claim -> notebook
   -> evidence -> sanitized paper -> knowledge transfer.
4. [architecture/AGENTS.md](architecture/AGENTS.md) — independent researcher
   roles, differing opinions, context isolation and prompts.
5. [architecture/SECURITY.md](architecture/SECURITY.md) — bridge to v4, blind
   relay, end-to-end encryption, authorized publication and confidentiality
   limits.
6. [architecture/REPOSITORIES.md](architecture/REPOSITORIES.md) — adapter
   contract for non-Rust repositories, federated hosts, operations.
7. [architecture/TELEMETRY.md](architecture/TELEMETRY.md) — three observability
   planes, GPUI research market cockpit, lineage.
8. [architecture/OPERATIONS.md](architecture/OPERATIONS.md) — week-long loop,
   failure recovery, acceptance criteria and roadmap.
9. [schemas/](schemas/) and [examples/](examples/) — validated research-case,
   proposal-bid, report and repository adapter contracts.
10. [sql/v5_migration.sql](sql/v5_migration.sql) — additive, validated against
    v2 and v3 SQLite migrations.
11. [playbooks/research-market-week.yaml](playbooks/research-market-week.yaml) —
    agent-economy campaign policy.
12. [rust/market-prototype/](rust/market-prototype/) — deterministic Rust
    _illustrative_ allocation CLI and tests. Toolchain unavailable in this
    environment; compilation not claimed.
13. [graphs/research-economy.mmd](graphs/research-economy.mmd) — Mermaid graph.

## Foundational distinctions

- Independent **opinions** belong to agents and can conflict. Measured **facts**
  belong to signed, independently produced evidence. Neither funding nor peer
  votes can substitute for correctness, replication or benchmark confirmation.
- Agents cannot promote themselves or alter an oracle, benchmark definition,
  budget ledger, private holdout, or publication stamp.
- A paper's status is explicit: **hypothesis / internal note / verified lab
  report / independently replicated / sanitized public preprint / externally
  peer reviewed**. A research agent's paper is never automatically a
  peer-reviewed publication.
- Tokens/compute/storage/benchmark leases form a **multi-resource,
  non-transferable research-credit economy**, not a cryptocurrency, unrestricted
  wallet, or pay-to-win deployment system.
- Default research contexts remain private from sibling agents. Sensitive
  material is decrypted only by authorized consumers. A sanitized knowledge
  publisher is an explicit declassification boundary. Visibility into timing,
  routing and the underlying infrastructure remains possible.
- A complete cryptographic audit proves attribution and integrity of submitted
  bytes, **not** the truth of an assertion or absence of prompt leakage.
- A single role-configurable Rust **agent image** is portable, but trustworthy
  operation still requires separated identity, coordinator, oracle, runner and
  publication principals. One-container developer appliance != one trust
  boundary.

## Suggested integration order

v2+v3 event store -> v4 identities/mTLS/HPKE -> v5 research cases & signed
claims -> budgets -> blind research assignment -> paper pipeline -> integration
bridge -> distributed federation and more repositories.

## Validation

Run `python validate_v5.py` from this folder. It loads all examples against
corresponding Draft 2020-12 JSON schemas, parses YAML, and applies the earlier
v2+v3 migrations followed by v5 in SQLite. This checks design artifacts only.
