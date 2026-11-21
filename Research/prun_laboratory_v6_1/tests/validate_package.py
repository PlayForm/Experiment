#!/usr/bin/env python3
"""Contract-only validator. No Docker or CA security claims are made here."""

from pathlib import Path
import json, sqlite3, sys, re, zipfile
from urllib.parse import urljoin
import yaml
import jsonschema
from referencing import Registry, Resource

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT.parents[0]
errors = []


def check(cond, msg):
    if not cond:
        errors.append(msg)


def loadj(p):
    return json.loads(p.read_text())


def validate_all():
    docs = {}
    for path in (ROOT / "schemas").glob("*.schema.json"):
        try:
            data = loadj(path)
            jsonschema.Draft202012Validator.check_schema(data)
            docs[data["$id"]] = data
        except Exception as exc:
            errors.append(f"schema {path.name}: {exc}")
    registry = Registry().with_resources(
        [(u, Resource.from_contents(d)) for u, d in docs.items()]
    )
    total = 0
    for path in (ROOT / "examples").glob("*.example.json"):
        name = path.name.removesuffix(".example.json")
        schema = ROOT / "schemas" / f"{name}.schema.json"
        if not schema.exists():
            errors.append(f"example {path.name}: missing schema")
            continue
        try:
            obj = loadj(path)
            d = loadj(schema)
            jsonschema.Draft202012Validator(
                d, registry=registry, format_checker=jsonschema.FormatChecker()
            ).validate(obj)
            total += 1
        except Exception as exc:
            errors.append(f"example {path.name}: {type(exc).__name__}: {exc}")
    print(f"schema files={len(docs)} examples validated={total}")
    # Check all local direct refs (including those in OpenAPI)
    for path in (ROOT / "schemas").glob("*.schema.json"):
        for ref in re.findall(r'"\$ref"\s*:\s*"([^"#]+)"', path.read_text()):
            if "://" not in ref and not (path.parent / ref).resolve().exists():
                errors.append(f"unresolved schema ref {path.name} -> {ref}")
    for name in ("public", "internal"):
        path = ROOT / "api" / f"{name}.openapi.yaml"
        doc = yaml.safe_load(path.read_text())
        check(doc.get("openapi", "").startswith("3.1"), f"{name} API not 3.1")
        ops = []
        for url, methods in doc.get("paths", {}).items():
            for verb, method in methods.items():
                if verb not in ("get", "post", "put", "patch", "delete"):
                    continue
                ops.append((verb, url, method))
                check(
                    bool(method.get("operationId")), f"missing operationId {verb} {url}"
                )
                check(
                    "x-lab-roles" in method
                    and "llm-worker" not in method.get("x-lab-roles", []),
                    f"worker API entitlement {url}",
                )
                check("responses" in method, f"no response {url}")
                if name == "internal":
                    check(
                        "requestBody" in method and "post" == verb,
                        f"no typed request {verb} {url}",
                    )
        ids = [o[2]["operationId"] for o in ops]
        check(len(ids) == len(set(ids)), f"duplicate operationIds {name}")
        for ref in re.findall(r"\$ref:\s*([^\s]+)", path.read_text()):
            if not ref.startswith("#") and not (path.parent / ref).resolve().exists():
                errors.append(f"bad API ref {name}:{ref}")
        print(f"{name} HTTP operations={len(ops)}")
    bus = yaml.safe_load((ROOT / "api/events.asyncapi.yaml").read_text())
    check(len(bus.get("channels", {})) >= 14, "missing v61 event channels")
    event_doc = loadj(ROOT / "events/catalog.json")
    keys = [e["event_kind"] for e in event_doc["events"]]
    check(len(keys) == len(set(keys)), "duplicate event kinds")
    for e in event_doc["events"]:
        check(
            bool(e.get("producer")) and bool(e.get("consumer")),
            f"event lacks principal {e}",
        )
        check(
            (ROOT / "events" / e["payload_schema"]).resolve().exists(),
            f"event missing schema {e}",
        )
    print("event kinds=", len(keys), "channels=", len(bus.get("channels", {})))
    roles = loadj(ROOT / "policy/v61-roles.json")
    check(
        roles["model_trust_domain"]["network_routes"] == [], "model can reach network"
    )
    check(
        roles["model_trust_domain"]["authorized_transitions"] == [],
        "model authorized transition",
    )
    states = yaml.safe_load((ROOT / "policy/v61-state-machines.yaml").read_text())
    for grp, edges in states["transitions"].items():
        all_states = states["states"][grp]
        for src, dst, role in edges:
            check(
                src in all_states and dst in all_states and bool(role),
                f"bad state edge {grp}:{src}:{dst}",
            )
    # All five migrations from prior known exact paths in runtime.
    lineage_paths = [
        ROOT / "sql/prerequisites/v2_lineage.sql",
        ROOT / "sql/prerequisites/v3_migration.sql",
        ROOT / "sql/prerequisites/v5_migration.sql",
        ROOT / "sql/v6_migration.sql",
        ROOT / "sql/v61_migration.sql",
    ]
    if not all(x.exists() for x in lineage_paths):
        # Standalone portable tests allow a v6.1 SQL check with FKs disabled only for syntax.
        errors.append(
            "integration migration prerequisites v2/v3/v5 missing from runtime; use complete package"
        )
        return
    c = sqlite3.connect(":memory:")
    for path in lineage_paths:
        try:
            c.executescript(path.read_text())
        except Exception as e:
            errors.append(f"SQL migration failure {path.name}: {e}")
    cnt = c.execute("SELECT COUNT(*) FROM sqlite_master WHERE type='table'").fetchone()[
        0
    ]
    print("SQL all versions combined tables=", cnt)
    # Denied decision cannot become a verified custody transfer.
    h = "0" * 64
    c.execute(
        "INSERT INTO v61_custody_requests(request_id,idempotency_key,artifact_digest,source_holder,target_holder,purpose,policy_digest,requested_bytes,nonce) VALUES(?,?,?,?,?,?,?,?,?)",
        ("r-1", "idem-1", h, "vault", "worker", "research", h, 1024, "nonce-1"),
    )
    c.execute(
        "INSERT INTO v61_custody_decisions(decision_id,request_id,decision,internal_reason_code,policy_digest,holder_attestation_digest,issued_at,expires_at) VALUES(?,?,?,?,?,?,?,?)",
        (
            "d-1",
            "r-1",
            "deny",
            "embargo",
            h,
            h,
            "2026-11-21T00:00:00Z",
            "2026-11-22T00:00:00Z",
        ),
    )
    c.execute(
        "INSERT INTO v61_storage_leases(lease_id,holder_id,artifact_digest,byte_reservation,storage_class,state,fence,expires_at) VALUES(?,?,?,?,?,?,?,?)",
        (
            "l-1",
            "worker",
            h,
            1024,
            "ephemeral-scratch",
            "reserved",
            1,
            "2026-11-22T00:00:00Z",
        ),
    )
    try:
        c.execute(
            "INSERT INTO v61_custody_attempts(attempt_id,request_id,decision_id,lease_id,state,fence,expected_revision) VALUES(?,?,?,?,?,?,?)",
            ("a-1", "r-1", "d-1", "l-1", "reserved", 1, 0),
        )
        errors.append("DENIED custody attempt wrongly succeeded")
    except sqlite3.IntegrityError:
        pass
    c.execute(
        "INSERT INTO v61_custody_requests(request_id,idempotency_key,artifact_digest,source_holder,target_holder,purpose,policy_digest,requested_bytes,nonce) VALUES(?,?,?,?,?,?,?,?,?)",
        ("r-2", "idem-2", h, "vault", "worker", "research", h, 1024, "nonce-2"),
    )
    c.execute(
        "INSERT INTO v61_custody_decisions(decision_id,request_id,decision,internal_reason_code,policy_digest,holder_attestation_digest,issued_at,expires_at) VALUES(?,?,?,?,?,?,?,?)",
        (
            "d-2",
            "r-2",
            "allow",
            "permitted",
            h,
            h,
            "2026-11-21T00:00:00Z",
            "2026-11-22T00:00:00Z",
        ),
    )
    c.execute(
        "INSERT INTO v61_custody_attempts(attempt_id,request_id,decision_id,lease_id,state,fence,expected_revision) VALUES(?,?,?,?,?,?,?)",
        ("a-2", "r-2", "d-2", "l-1", "reserved", 1, 0),
    )
    try:
        c.execute(
            "INSERT INTO v61_custody_receipts(receipt_id,request_id,attempt_id,artifact_digest,ciphertext_digest,verified_digest,target_holder,holder_attestation_digest,committed_at) VALUES(?,?,?,?,?,?,?,?,?)",
            ("receipt", "r-2", "a-2", h, h, h, "worker", h, "2026-11-21T00:00:00Z"),
        )
        errors.append("UNVERIFIED custody receipt wrongly succeeded")
    except sqlite3.IntegrityError:
        pass
    c.execute("UPDATE v61_custody_attempts SET state='verified' WHERE attempt_id='a-2'")
    c.execute(
        "INSERT INTO v61_custody_receipts(receipt_id,request_id,attempt_id,artifact_digest,ciphertext_digest,verified_digest,target_holder,holder_attestation_digest,committed_at) VALUES(?,?,?,?,?,?,?,?,?)",
        ("receipt", "r-2", "a-2", h, h, h, "worker", h, "2026-11-21T00:00:00Z"),
    )
    for op in (
        "UPDATE v61_custody_receipts SET committed_at='2020' WHERE receipt_id='receipt'",
        "DELETE FROM v61_custody_receipts WHERE receipt_id='receipt'",
    ):
        try:
            c.execute(op)
            errors.append("immutable receipt operation allowed")
        except sqlite3.IntegrityError:
            pass
    # Same idempotency key must not create a second request.
    try:
        c.execute(
            "INSERT INTO v61_custody_requests(request_id,idempotency_key,artifact_digest,source_holder,target_holder,purpose,policy_digest,requested_bytes,nonce) VALUES(?,?,?,?,?,?,?,?,?)",
            ("r-3", "idem-2", h, "vault", "worker", "research", h, 1024, "nonce-3"),
        )
        errors.append("duplicate idempotency key allowed")
    except sqlite3.IntegrityError:
        pass
    print(
        "negative SQL tests=denial, unverified-receipt, receipt-immutability, duplicate-idempotency"
    )


if __name__ == "__main__":
    validate_all()
    if errors:
        for e in errors:
            print("FAIL:", e, file=sys.stderr)
        print("FAIL", len(errors), "issues")
        sys.exit(1)
    print("PASS: v6.1 schema + API + event + SQL contract checks")
