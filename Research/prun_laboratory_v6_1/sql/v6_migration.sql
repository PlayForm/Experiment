-- Laboratory v6 additive protocol + correspondence index. Apply after v2 -> v3 -> v5.
PRAGMA foreign_keys = ON;

CREATE TABLE
	IF NOT EXISTS v6_event_journal (
		global_seq INTEGER PRIMARY KEY AUTOINCREMENT,
		event_id TEXT UNIQUE NOT NULL,
		event_kind TEXT NOT NULL,
		aggregate_id TEXT NOT NULL,
		aggregate_seq INTEGER NOT NULL CHECK (aggregate_seq > 0),
		recorded_utc TEXT NOT NULL,
		actor_id TEXT NOT NULL,
		credential_id TEXT NOT NULL,
		correlation_id TEXT NOT NULL,
		causation_event_id TEXT,
		idempotency_key TEXT UNIQUE NOT NULL,
		payload_sha256 TEXT NOT NULL CHECK (length (payload_sha256) = 64),
		previous_hash TEXT NOT NULL CHECK (length (previous_hash) = 64),
		event_hash TEXT UNIQUE NOT NULL CHECK (length (event_hash) = 64),
		signature_artifact_sha256 TEXT NOT NULL CHECK (length (signature_artifact_sha256) = 64),
		privacy_class TEXT NOT NULL CHECK (
			privacy_class IN (
				'public-meta',
				'internal-meta',
				'restricted',
				'sealed'
			)
		),
		UNIQUE (aggregate_id, aggregate_seq)
	);

CREATE INDEX IF NOT EXISTS v6_event_correlation_idx ON v6_event_journal (correlation_id, global_seq);

CREATE INDEX IF NOT EXISTS v6_event_kind_idx ON v6_event_journal (event_kind, global_seq);

CREATE TABLE
	IF NOT EXISTS v6_command_inbox (
		command_id TEXT PRIMARY KEY,
		idempotency_key TEXT NOT NULL UNIQUE,
		operator_identity TEXT NOT NULL,
		command_kind TEXT NOT NULL,
		target_id TEXT NOT NULL,
		expected_revision INTEGER NOT NULL CHECK (expected_revision >= 0),
		payload_sha256 TEXT NOT NULL CHECK (length (payload_sha256) = 64),
		state TEXT NOT NULL CHECK (
			state IN ('received', 'validated', 'rejected', 'committed')
		),
		receipt_event_id TEXT REFERENCES v6_event_journal (event_id),
		submitted_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS v6_projection_outbox (
		event_id TEXT PRIMARY KEY REFERENCES v6_event_journal (event_id),
		delivery_seq INTEGER NOT NULL DEFAULT 0 CHECK (delivery_seq >= 0),
		delivery_state TEXT NOT NULL DEFAULT 'pending' CHECK (
			delivery_state IN ('pending', 'delivered', 'quarantined')
		),
		last_delivery_utc TEXT
	);

CREATE TABLE
	IF NOT EXISTS v6_record_index (
		record_kind TEXT NOT NULL,
		record_id TEXT NOT NULL,
		schema_version INTEGER NOT NULL CHECK (schema_version = 6),
		json_sha256 TEXT NOT NULL CHECK (length (json_sha256) = 64),
		privacy_class TEXT NOT NULL,
		subject_id TEXT NOT NULL,
		event_id TEXT NOT NULL REFERENCES v6_event_journal (event_id),
		PRIMARY KEY (record_kind, record_id)
	);

CREATE TABLE
	IF NOT EXISTS v6_artifact_grants (
		grant_id TEXT PRIMARY KEY,
		artifact_sha256 TEXT NOT NULL CHECK (length (artifact_sha256) = 64),
		recipient_identity TEXT NOT NULL,
		purpose TEXT NOT NULL,
		grant_policy_sha256 TEXT NOT NULL CHECK (length (grant_policy_sha256) = 64),
		expires_utc TEXT NOT NULL,
		revoked_event_id TEXT REFERENCES v6_event_journal (event_id),
		UNIQUE (artifact_sha256, recipient_identity, purpose)
	);

CREATE TABLE
	IF NOT EXISTS v6_correspondence (
		turn_id TEXT PRIMARY KEY,
		conversation_id TEXT NOT NULL,
		task_id TEXT NOT NULL,
		segment_id TEXT NOT NULL,
		turn_seq INTEGER NOT NULL CHECK (turn_seq >= 0),
		model_cohort_digest TEXT NOT NULL CHECK (length (model_cohort_digest) = 64),
		context_digest TEXT NOT NULL CHECK (length (context_digest) = 64),
		ciphertext_digest TEXT NOT NULL CHECK (length (ciphertext_digest) = 64),
		recipient_key_id TEXT NOT NULL,
		privacy_class TEXT NOT NULL CHECK (privacy_class IN ('restricted', 'sealed')),
		committed_event_id TEXT NOT NULL REFERENCES v6_event_journal (event_id),
		UNIQUE (conversation_id, turn_seq)
	);

CREATE INDEX IF NOT EXISTS v6_correspondence_task ON v6_correspondence (task_id, turn_seq);

CREATE TABLE
	IF NOT EXISTS v6_tool_calls (
		tool_call_id TEXT PRIMARY KEY,
		turn_id TEXT NOT NULL REFERENCES v6_correspondence (turn_id),
		allowed_tool_name TEXT NOT NULL,
		arguments_ciphertext_digest TEXT NOT NULL CHECK (length (arguments_ciphertext_digest) = 64),
		result_ciphertext_digest TEXT NOT NULL CHECK (length (result_ciphertext_digest) = 64),
		policy_check_digest TEXT NOT NULL CHECK (length (policy_check_digest) = 64),
		receipt_event_id TEXT NOT NULL REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v6_task_mailboxes (
		assignment_id TEXT PRIMARY KEY,
		task_id TEXT NOT NULL UNIQUE,
		supervisor_id TEXT NOT NULL,
		bundle_digest TEXT NOT NULL CHECK (length (bundle_digest) = 64),
		task_nonce TEXT NOT NULL UNIQUE,
		lease_id TEXT NOT NULL,
		fencing_token INTEGER NOT NULL CHECK (fencing_token >= 1),
		state TEXT NOT NULL CHECK (
			state IN (
				'staged',
				'mounted',
				'running',
				'sealed',
				'expired',
				'quarantined'
			)
		),
		output_digest TEXT CHECK (
			output_digest IS NULL
			OR length (output_digest) = 64
		)
	);

CREATE TABLE
	IF NOT EXISTS v6_service_credentials (
		serial TEXT PRIMARY KEY,
		subject_id TEXT NOT NULL,
		issuer_id TEXT NOT NULL,
		kind TEXT NOT NULL,
		public_key_fingerprint TEXT NOT NULL CHECK (length (public_key_fingerprint) = 64),
		scope_digest TEXT NOT NULL CHECK (length (scope_digest) = 64),
		expires_utc TEXT NOT NULL,
		status TEXT NOT NULL CHECK (status IN ('active', 'revoked', 'expired'))
	);

CREATE TABLE
	IF NOT EXISTS v6_credential_events (
		seq INTEGER PRIMARY KEY AUTOINCREMENT,
		serial TEXT NOT NULL REFERENCES v6_service_credentials (serial),
		action TEXT NOT NULL CHECK (
			action IN (
				'issued',
				'renewed',
				'revoked',
				'expired',
				'denied'
			)
		),
		event_id TEXT NOT NULL UNIQUE REFERENCES v6_event_journal (event_id),
		occurred_utc TEXT NOT NULL,
		reason_code TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS v6_benchmark_leases (
		lease_id TEXT PRIMARY KEY,
		host_id TEXT NOT NULL,
		experiment_id TEXT NOT NULL,
		cpuset TEXT NOT NULL,
		fencing_token INTEGER NOT NULL CHECK (fencing_token > 0),
		state TEXT NOT NULL CHECK (
			state IN ('held', 'released', 'revoked', 'expired')
		),
		acquired_utc TEXT NOT NULL,
		expires_utc TEXT NOT NULL,
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64)
	);

CREATE TABLE
	IF NOT EXISTS v6_telemetry_refs (
		telemetry_id TEXT PRIMARY KEY,
		plane TEXT NOT NULL CHECK (plane IN ('control', 'product')),
		experiment_id TEXT,
		task_id TEXT,
		event_id TEXT REFERENCES v6_event_journal (event_id),
		trace_id TEXT,
		span_id TEXT,
		archive_sha256 TEXT NOT NULL CHECK (length (archive_sha256) = 64),
		capture_mode TEXT NOT NULL,
		recorded_utc TEXT NOT NULL
	);

CREATE INDEX IF NOT EXISTS v6_telemetry_lookup ON v6_telemetry_refs (plane, experiment_id, recorded_utc);

CREATE TABLE
	IF NOT EXISTS v6_confirmation_access (
		access_id TEXT PRIMARY KEY,
		subject_id TEXT NOT NULL,
		partition_id TEXT NOT NULL,
		capability_digest TEXT NOT NULL CHECK (length (capability_digest) = 64),
		requester_identity TEXT NOT NULL,
		authorized INTEGER NOT NULL CHECK (authorized IN (0, 1)),
		reason_code TEXT NOT NULL,
		event_id TEXT NOT NULL REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v6_service_outbox (
		delivery_id TEXT PRIMARY KEY,
		event_id TEXT NOT NULL REFERENCES v6_event_journal (event_id),
		subscriber_identity TEXT NOT NULL,
		delivered INTEGER NOT NULL DEFAULT 0 CHECK (delivered IN (0, 1)),
		attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
		last_try_utc TEXT,
		UNIQUE (event_id, subscriber_identity)
	);

-- Immutable canonical journal and correspondence; append-only can be enforced in a transaction plus trigger.
CREATE TRIGGER IF NOT EXISTS v6_journal_no_update BEFORE
UPDATE ON v6_event_journal BEGIN
SELECT
	RAISE (ABORT, 'immutable journal');

END;

CREATE TRIGGER IF NOT EXISTS v6_journal_no_delete BEFORE DELETE ON v6_event_journal BEGIN
SELECT
	RAISE (ABORT, 'immutable journal');

END;

CREATE TRIGGER IF NOT EXISTS v6_correspondence_no_update BEFORE
UPDATE ON v6_correspondence BEGIN
SELECT
	RAISE (ABORT, 'immutable correspondence');

END;

CREATE TRIGGER IF NOT EXISTS v6_correspondence_no_delete BEFORE DELETE ON v6_correspondence BEGIN
SELECT
	RAISE (ABORT, 'immutable correspondence');

END;

CREATE TRIGGER IF NOT EXISTS v6_tool_calls_no_update BEFORE
UPDATE ON v6_tool_calls BEGIN
SELECT
	RAISE (ABORT, 'immutable tool-call receipt');

END;

CREATE TRIGGER IF NOT EXISTS v6_tool_calls_no_delete BEFORE DELETE ON v6_tool_calls BEGIN
SELECT
	RAISE (ABORT, 'immutable tool-call receipt');

END;
