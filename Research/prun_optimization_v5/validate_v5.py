#!/usr/bin/env python3
"""Validates manifest examples, YAML and additive SQLite schema. Needs jsonschema and PyYAML."""

from pathlib import Path
import json, sqlite3
import yaml
from jsonschema import Draft202012Validator, FormatChecker

root = Path(__file__).resolve().parent
print("v5 structural validation")
for schema_file in sorted((root / "schemas").glob("*.schema.json")):
    schema = json.loads(schema_file.read_text())
    Draft202012Validator.check_schema(schema)
    example_file = root / "examples" / schema_file.name.replace(".schema.json", ".json")
    example = json.loads(example_file.read_text())
    Draft202012Validator(schema, format_checker=FormatChecker()).validate(example)
    print("PASS", schema_file.name, "+", example_file.name)
for p in [
    root / "playbooks/research-market-week.yaml",
    root / "deploy/compose.prototype.yaml",
]:
    assert isinstance(yaml.safe_load(p.read_text()), dict)
    print("PASS", p.relative_to(root))
conn = sqlite3.connect(":memory:")
conn.execute("PRAGMA foreign_keys=ON;")
base = root.parent / "prun_optimization_v2/sql/lineage.sql"
v3 = root.parent / "prun_optimization_v3/sql/v3_migration.sql"
for p in [base, v3, root / "sql/v5_migration.sql"]:
    conn.executescript(p.read_text())
    print("PASS", p.name)
# v5 tables exist after migrations
for t in [
    "research_cases",
    "researcher_assignments",
    "research_bids",
    "research_claims",
    "resource_transactions",
    "resource_reservations",
    "paper_manuscripts",
    "paper_claims",
    "paper_reviews",
    "publication_releases",
    "integration_cases",
]:
    assert conn.execute(
        "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?", (t,)
    ).fetchone(), t
print(
    "PASS 11 v5 database entities present, total tables:",
    conn.execute("SELECT count(*) FROM sqlite_master WHERE type='table'").fetchone()[0],
)
# Check expected immutability and idempotency constraints with fixture artifacts.
zero = "0" * 64
conn.execute(
    "INSERT INTO artifacts(sha256,kind,byte_size,relative_path,created_utc) VALUES(?,?,?,?,?)",
    (zero, "test", 0, "none", "2026-11-21T00:00:00Z"),
)
conn.execute(
    "INSERT INTO campaigns(id,schema_version,created_utc,manifest_sha256,state) VALUES(?,?,?,?,?)",
    ("c", 5, "2026-11-21T00:00:00Z", zero, "test"),
)
conn.execute(
    "INSERT INTO resource_treasuries(id,campaign_id,policy_sha256,created_utc) VALUES(?,?,?,?)",
    ("t", "c", zero, "2026-11-21T00:00:00Z"),
)
basevals = (
    "tx1",
    "t",
    None,
    None,
    "rq-1",
    "fund",
    100,
    0,
    0,
    0,
    0,
    0,
    0,
    zero,
    zero,
    "treasury",
    zero,
    "2026-11-21T00:00:00Z",
)
conn.execute(
    """INSERT INTO resource_transactions(transaction_id,treasury_id,assignment_id,bid_id,request_id,kind,input_tokens,output_tokens,cpu_core_seconds,benchmark_seconds,gpu_seconds,storage_byte_days,reference_requests,policy_sha256,evidence_sha256,issuer_identity,signature_sha256,created_utc) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)""",
    basevals,
)
try:
    conn.execute(
        "UPDATE resource_transactions SET input_tokens=999 WHERE transaction_id='tx1'"
    )
    raise AssertionError("immutable transaction wrongly permitted")
except sqlite3.IntegrityError:
    print("PASS immutable resource ledger trigger")
try:
    conn.execute(
        """INSERT INTO resource_transactions(transaction_id,treasury_id,request_id,kind,policy_sha256,evidence_sha256,issuer_identity,signature_sha256,created_utc) VALUES(?,?,?,?,?,?,?,?,?)""",
        (
            "tx2",
            "t",
            "rq-1",
            "fund",
            zero,
            zero,
            "treasury",
            zero,
            "2026-11-21T00:00:00Z",
        ),
    )
    raise AssertionError("duplicate request wrongly permitted")
except sqlite3.IntegrityError:
    print("PASS budget request idempotency uniqueness")
print(
    "Validation complete. Does NOT validate Rust compilation, Docker operation, cryptographic security or science."
)
