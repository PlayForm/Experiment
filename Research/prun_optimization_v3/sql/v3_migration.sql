-- V3 ADDITIVE DRAFT migration. Apply AFTER v2 sql/lineage.sql in a fresh local SQLite DB.
-- All identities point to immutable artifact SHA256s; production requires transactional
-- appends, replay/outbox, and hash/attestation validation in the Rust application.
PRAGMA foreign_keys = ON;

CREATE TABLE
	IF NOT EXISTS generations (
		id TEXT PRIMARY KEY,
		campaign_id TEXT NOT NULL REFERENCES campaigns (id),
		frozen_baseline_build_id TEXT NOT NULL REFERENCES builds (id),
		generation_spec_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		status TEXT NOT NULL,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS work_orders (
		id TEXT PRIMARY KEY,
		campaign_id TEXT NOT NULL REFERENCES campaigns (id),
		generation_id TEXT REFERENCES generations (id),
		role TEXT NOT NULL,
		scope_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		task_spec_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		idempotency_key TEXT NOT NULL UNIQUE,
		state TEXT NOT NULL,
		lease_owner TEXT,
		lease_deadline_utc TEXT,
		attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS agent_sessions (
		id TEXT PRIMARY KEY,
		work_order_id TEXT NOT NULL REFERENCES work_orders (id),
		role_policy_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		prompt_template_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		provider_model_id TEXT NOT NULL,
		parameters_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		started_utc TEXT NOT NULL,
		ended_utc TEXT
	);

CREATE TABLE
	IF NOT EXISTS context_snapshots (
		id TEXT PRIMARY KEY,
		agent_session_id TEXT NOT NULL REFERENCES agent_sessions (id),
		allowed_view_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		resolved_context_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		compaction_sha256 TEXT REFERENCES artifacts (sha256),
		turn_first INTEGER NOT NULL,
		turn_last INTEGER NOT NULL,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS agent_turns (
		id TEXT PRIMARY KEY,
		agent_session_id TEXT NOT NULL REFERENCES agent_sessions (id),
		context_snapshot_id TEXT REFERENCES context_snapshots (id),
		turn_seq INTEGER NOT NULL,
		kind TEXT NOT NULL CHECK (
			kind IN (
				'system',
				'developer',
				'user',
				'tool_call',
				'tool_result',
				'assistant',
				'feedback'
			)
		),
		content_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		privacy_class TEXT NOT NULL,
		model_metrics_sha256 TEXT REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL,
		UNIQUE (agent_session_id, turn_seq)
	);

CREATE TABLE
	IF NOT EXISTS proposals (
		id TEXT PRIMARY KEY,
		work_order_id TEXT NOT NULL REFERENCES work_orders (id),
		mutation_id TEXT REFERENCES mutations (id),
		parent_source_commit TEXT NOT NULL,
		patch_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		proposal_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		mechanism_key TEXT NOT NULL,
		changed_symbols_sha256 TEXT REFERENCES artifacts (sha256),
		status TEXT NOT NULL,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS novelty_verdicts (
		id TEXT PRIMARY KEY,
		proposal_id TEXT NOT NULL REFERENCES proposals (id),
		gate_version TEXT NOT NULL,
		exact_identity_sha256 TEXT NOT NULL,
		similarity_report_sha256 TEXT REFERENCES artifacts (sha256),
		verdict TEXT NOT NULL CHECK (
			verdict IN (
				'NOVEL',
				'EXACT_CACHE_HIT',
				'NEAR_DUPLICATE',
				'CONFLICT',
				'INSUFFICIENT_PROOF',
				'INVALID'
			)
		),
		related_proposal_id TEXT REFERENCES proposals (id),
		reason_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS feedback_capsules (
		id TEXT PRIMARY KEY,
		work_order_id TEXT NOT NULL REFERENCES work_orders (id),
		proposal_id TEXT REFERENCES proposals (id),
		source_analysis_id TEXT REFERENCES analyses (id),
		view_policy_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		content_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		redaction_passed INTEGER NOT NULL CHECK (redaction_passed IN (0, 1)),
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS attestations (
		id TEXT PRIMARY KEY,
		subject_kind TEXT NOT NULL,
		subject_id TEXT NOT NULL,
		stamp_kind TEXT NOT NULL CHECK (
			stamp_kind IN (
				'scope_checked',
				'novelty_checked',
				'oracle_passed',
				'bench_integrity_passed',
				'analysis_confirmed',
				'integration_eligible',
				'release_approved'
			)
		),
		policy_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		evidence_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		issuer TEXT NOT NULL,
		signature TEXT NOT NULL,
		valid_cohort TEXT NOT NULL,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS telemetry_references (
		id TEXT PRIMARY KEY,
		domain TEXT NOT NULL CHECK (domain IN ('control', 'product', 'measurement')),
		event_id TEXT REFERENCES events (event_id),
		experiment_id TEXT REFERENCES experiments (id),
		trial_id TEXT REFERENCES trials (id),
		trace_id TEXT,
		span_id TEXT,
		payload_sha256 TEXT REFERENCES artifacts (sha256),
		instrumentation_mode TEXT NOT NULL,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS dispatch_outbox (
		id TEXT PRIMARY KEY,
		event_id TEXT NOT NULL UNIQUE REFERENCES events (event_id),
		work_order_id TEXT NOT NULL REFERENCES work_orders (id),
		operation TEXT NOT NULL,
		payload_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		accepted_utc TEXT NOT NULL,
		delivered_utc TEXT
	);

CREATE INDEX IF NOT EXISTS idx_orders_campaign ON work_orders (campaign_id, generation_id, state);

CREATE INDEX IF NOT EXISTS idx_proposals_patch ON proposals (patch_sha256, parent_source_commit);

CREATE INDEX IF NOT EXISTS idx_proposals_mechanism ON proposals (mechanism_key);

CREATE INDEX IF NOT EXISTS idx_novelty_proposal ON novelty_verdicts (proposal_id, created_utc);

CREATE INDEX IF NOT EXISTS idx_attest_subject ON attestations (subject_kind, subject_id, stamp_kind);

CREATE INDEX IF NOT EXISTS idx_telemetry_trial ON telemetry_references (trial_id, domain);

CREATE INDEX IF NOT EXISTS idx_turns_session ON agent_turns (agent_session_id, turn_seq);

-- These audit tables append only; stateful projection/outbox rows are allowed to change.
CREATE TRIGGER IF NOT EXISTS deny_attestation_update BEFORE
UPDATE ON attestations BEGIN
SELECT
	RAISE (ABORT, 'immutable attestation');

END;

CREATE TRIGGER IF NOT EXISTS deny_attestation_delete BEFORE DELETE ON attestations BEGIN
SELECT
	RAISE (ABORT, 'immutable attestation');

END;

CREATE TRIGGER IF NOT EXISTS deny_turns_update BEFORE
UPDATE ON agent_turns BEGIN
SELECT
	RAISE (ABORT, 'immutable agent turn');

END;

CREATE TRIGGER IF NOT EXISTS deny_turns_delete BEFORE DELETE ON agent_turns BEGIN
SELECT
	RAISE (ABORT, 'immutable agent turn');

END;

CREATE TRIGGER IF NOT EXISTS deny_feedback_update BEFORE
UPDATE ON feedback_capsules BEGIN
SELECT
	RAISE (ABORT, 'immutable feedback');

END;

CREATE TRIGGER IF NOT EXISTS deny_feedback_delete BEFORE DELETE ON feedback_capsules BEGIN
SELECT
	RAISE (ABORT, 'immutable feedback');

END;
