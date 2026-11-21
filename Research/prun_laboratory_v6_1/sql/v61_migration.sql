-- v6.1 additive extension. Apply after v2 -> v3 -> v5 -> v6; no existing tables modified.
-- All authoritative transitions must also append a signed v6_event_journal event in one transaction.
PRAGMA foreign_keys = ON;

CREATE TABLE
	IF NOT EXISTS v61_repository_admissions (
		admission_id TEXT PRIMARY KEY,
		repository_id TEXT NOT NULL,
		tenant_id TEXT NOT NULL,
		commit_digest TEXT NOT NULL CHECK (length (commit_digest) = 64),
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64),
		license_review_digest TEXT NOT NULL CHECK (length (license_review_digest) = 64),
		admission_status TEXT NOT NULL CHECK (
			admission_status IN ('pending', 'admitted', 'quarantined', 'rejected')
		),
		record_digest TEXT NOT NULL CHECK (length (record_digest) = 64),
		event_id TEXT REFERENCES v6_event_journal (event_id),
		UNIQUE (
			repository_id,
			tenant_id,
			commit_digest,
			policy_digest
		)
	);

CREATE TABLE
	IF NOT EXISTS v61_artifact_policies (
		policy_id TEXT PRIMARY KEY,
		artifact_digest TEXT NOT NULL CHECK (length (artifact_digest) = 64),
		repository_id TEXT NOT NULL,
		tenant_id TEXT NOT NULL,
		classification TEXT NOT NULL CHECK (
			classification IN ('public', 'internal', 'restricted', 'sealed')
		),
		license_grant_id TEXT NOT NULL,
		embargo_set_digest TEXT NOT NULL CHECK (length (embargo_set_digest) = 64),
		permitted_purposes_digest TEXT NOT NULL CHECK (length (permitted_purposes_digest) = 64),
		policy_digest TEXT UNIQUE NOT NULL CHECK (length (policy_digest) = 64),
		effective_at TEXT NOT NULL,
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE INDEX IF NOT EXISTS v61_artifact_policy_lookup ON v61_artifact_policies (artifact_digest, tenant_id);

CREATE TABLE
	IF NOT EXISTS v61_holder_attestations (
		attestation_id TEXT PRIMARY KEY,
		holder_id TEXT NOT NULL,
		holder_role TEXT NOT NULL,
		tenant_id TEXT NOT NULL,
		environment_digest TEXT NOT NULL CHECK (length (environment_digest) = 64),
		capacity_bytes INTEGER NOT NULL CHECK (capacity_bytes >= 0),
		available_bytes INTEGER NOT NULL CHECK (
			available_bytes >= 0
			AND available_bytes <= capacity_bytes
		),
		encryption_required INTEGER NOT NULL CHECK (encryption_required IN (0, 1)),
		network_mode TEXT NOT NULL,
		capability_digest TEXT NOT NULL CHECK (length (capability_digest) = 64),
		expires_at TEXT NOT NULL,
		recorded_event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_license_grants (
		grant_id TEXT PRIMARY KEY,
		repository_id TEXT NOT NULL,
		spdx_expression TEXT NOT NULL,
		license_text_digest TEXT NOT NULL CHECK (length (license_text_digest) = 64),
		eula_digest TEXT NOT NULL CHECK (length (eula_digest) = 64),
		status TEXT NOT NULL CHECK (
			status IN (
				'unreviewed',
				'compatible',
				'restricted',
				'requires-review',
				'prohibited'
			)
		),
		model_training_allowed INTEGER NOT NULL CHECK (model_training_allowed IN (0, 1)),
		redistribution_allowed INTEGER NOT NULL CHECK (redistribution_allowed IN (0, 1)),
		record_digest TEXT UNIQUE NOT NULL CHECK (length (record_digest) = 64),
		reviewed_by TEXT NOT NULL,
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_embargo_rules (
		embargo_id TEXT PRIMARY KEY,
		repository_id TEXT NOT NULL,
		subject_digest TEXT NOT NULL CHECK (length (subject_digest) = 64),
		target_set_digest TEXT NOT NULL CHECK (length (target_set_digest) = 64),
		release_after TEXT NOT NULL,
		owner_signature_digest TEXT NOT NULL CHECK (length (owner_signature_digest) = 64),
		record_digest TEXT UNIQUE NOT NULL CHECK (length (record_digest) = 64),
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_trade_restrictions (
		restriction_id TEXT PRIMARY KEY,
		source_group TEXT NOT NULL,
		target_group TEXT NOT NULL,
		direction TEXT NOT NULL CHECK (direction IN ('a-to-b', 'b-to-a', 'both')),
		kind TEXT NOT NULL,
		effective_at TEXT NOT NULL,
		expires_at TEXT NOT NULL,
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64),
		event_id TEXT REFERENCES v6_event_journal (event_id),
		CHECK (source_group <> target_group)
	);

CREATE INDEX IF NOT EXISTS v61_trade_lookup ON v61_trade_restrictions (source_group, target_group, effective_at);

CREATE TABLE
	IF NOT EXISTS v61_custody_requests (
		request_id TEXT PRIMARY KEY,
		idempotency_key TEXT UNIQUE NOT NULL,
		artifact_digest TEXT NOT NULL CHECK (length (artifact_digest) = 64),
		source_holder TEXT NOT NULL,
		target_holder TEXT NOT NULL,
		purpose TEXT NOT NULL,
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64),
		requested_bytes INTEGER NOT NULL CHECK (requested_bytes >= 0),
		nonce TEXT UNIQUE NOT NULL,
		submitted_event_id TEXT REFERENCES v6_event_journal (event_id),
		CHECK (source_holder <> target_holder)
	);

CREATE TABLE
	IF NOT EXISTS v61_custody_decisions (
		decision_id TEXT PRIMARY KEY,
		request_id TEXT NOT NULL UNIQUE REFERENCES v61_custody_requests (request_id),
		decision TEXT NOT NULL CHECK (
			decision IN ('allow', 'deny', 'defer', 'quarantine')
		),
		internal_reason_code TEXT NOT NULL,
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64),
		holder_attestation_digest TEXT NOT NULL CHECK (length (holder_attestation_digest) = 64),
		issued_at TEXT NOT NULL,
		expires_at TEXT NOT NULL,
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_storage_leases (
		lease_id TEXT PRIMARY KEY,
		holder_id TEXT NOT NULL,
		artifact_digest TEXT NOT NULL CHECK (length (artifact_digest) = 64),
		byte_reservation INTEGER NOT NULL CHECK (byte_reservation >= 0),
		byte_actual INTEGER NOT NULL DEFAULT 0 CHECK (byte_actual >= 0),
		storage_class TEXT NOT NULL,
		state TEXT NOT NULL CHECK (
			state IN ('reserved', 'active', 'released', 'quarantined')
		),
		fence INTEGER NOT NULL CHECK (fence > 0),
		expires_at TEXT NOT NULL,
		allocation_event_id TEXT REFERENCES v6_event_journal (event_id),
		UNIQUE (holder_id, artifact_digest, fence)
	);

CREATE TABLE
	IF NOT EXISTS v61_custody_attempts (
		attempt_id TEXT PRIMARY KEY,
		request_id TEXT NOT NULL REFERENCES v61_custody_requests (request_id),
		decision_id TEXT NOT NULL REFERENCES v61_custody_decisions (decision_id),
		lease_id TEXT NOT NULL REFERENCES v61_storage_leases (lease_id),
		state TEXT NOT NULL CHECK (
			state IN (
				'reserved',
				'staged',
				'verified',
				'committed',
				'quarantined',
				'expired'
			)
		),
		fence INTEGER NOT NULL CHECK (fence > 0),
		stage_digest TEXT CHECK (
			stage_digest IS NULL
			OR length (stage_digest) = 64
		),
		expected_revision INTEGER NOT NULL CHECK (expected_revision >= 0),
		UNIQUE (request_id, decision_id, lease_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_custody_receipts (
		receipt_id TEXT PRIMARY KEY,
		request_id TEXT NOT NULL REFERENCES v61_custody_requests (request_id),
		attempt_id TEXT NOT NULL UNIQUE REFERENCES v61_custody_attempts (attempt_id),
		artifact_digest TEXT NOT NULL CHECK (length (artifact_digest) = 64),
		ciphertext_digest TEXT NOT NULL CHECK (length (ciphertext_digest) = 64),
		verified_digest TEXT NOT NULL CHECK (length (verified_digest) = 64),
		target_holder TEXT NOT NULL,
		holder_attestation_digest TEXT NOT NULL CHECK (length (holder_attestation_digest) = 64),
		committed_at TEXT NOT NULL,
		event_id TEXT REFERENCES v6_event_journal (event_id),
		UNIQUE (request_id, artifact_digest, target_holder)
	);

CREATE TABLE
	IF NOT EXISTS v61_holdings (
		holder_id TEXT NOT NULL,
		artifact_digest TEXT NOT NULL CHECK (length (artifact_digest) = 64),
		receipt_id TEXT NOT NULL REFERENCES v61_custody_receipts (receipt_id),
		use_rights_digest TEXT NOT NULL CHECK (length (use_rights_digest) = 64),
		key_scope TEXT NOT NULL,
		acquired_at TEXT NOT NULL,
		expires_at TEXT NOT NULL,
		released_at TEXT,
		PRIMARY KEY (holder_id, artifact_digest, receipt_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_research_tracks (
		track_id TEXT PRIMARY KEY,
		repository_id TEXT NOT NULL,
		role TEXT NOT NULL,
		view_id TEXT NOT NULL,
		trade_group TEXT NOT NULL,
		model_lineage_digest TEXT NOT NULL CHECK (length (model_lineage_digest) = 64),
		assignment_policy_digest TEXT NOT NULL CHECK (length (assignment_policy_digest) = 64),
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_research_views (
		view_id TEXT PRIMARY KEY,
		worker_assignment_id TEXT NOT NULL,
		repository_id TEXT NOT NULL,
		source_snapshot_digest TEXT NOT NULL CHECK (length (source_snapshot_digest) = 64),
		view_digest TEXT NOT NULL CHECK (length (view_digest) = 64),
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64),
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_task_context_plans (
		plan_id TEXT PRIMARY KEY,
		view_id TEXT NOT NULL REFERENCES v61_research_views (view_id),
		mount_digest TEXT NOT NULL CHECK (length (mount_digest) = 64),
		tools_digest TEXT NOT NULL CHECK (length (tools_digest) = 64),
		scratch_bytes INTEGER NOT NULL CHECK (scratch_bytes >= 0),
		network_mode TEXT NOT NULL CHECK (network_mode = 'none'),
		supervisor_attestation TEXT NOT NULL CHECK (length (supervisor_attestation) = 64)
	);

CREATE TABLE
	IF NOT EXISTS v61_experiment_intents (
		intent_id TEXT PRIMARY KEY,
		worker_assignment_id TEXT NOT NULL,
		proposal_digest TEXT NOT NULL CHECK (length (proposal_digest) = 64),
		source_digest TEXT NOT NULL CHECK (length (source_digest) = 64),
		worker_claim_digest TEXT NOT NULL CHECK (length (worker_claim_digest) = 64),
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_experiment_specs (
		spec_id TEXT PRIMARY KEY,
		intent_id TEXT NOT NULL REFERENCES v61_experiment_intents (intent_id),
		reference_digest TEXT NOT NULL CHECK (length (reference_digest) = 64),
		candidate_digest TEXT NOT NULL CHECK (length (candidate_digest) = 64),
		oracle_policy_digest TEXT NOT NULL CHECK (length (oracle_policy_digest) = 64),
		protocol_digest TEXT NOT NULL CHECK (length (protocol_digest) = 64),
		compiled_by TEXT NOT NULL CHECK (compiled_by = 'trusted-policy-engine'),
		event_id TEXT REFERENCES v6_event_journal (event_id),
		UNIQUE (intent_id, candidate_digest, protocol_digest)
	);

CREATE TABLE
	IF NOT EXISTS v61_candidate_batches (
		batch_id TEXT PRIMARY KEY,
		repository_id TEXT NOT NULL,
		baseline_digest TEXT NOT NULL CHECK (length (baseline_digest) = 64),
		protocol_digest TEXT NOT NULL CHECK (length (protocol_digest) = 64),
		variant_set_digest TEXT NOT NULL CHECK (length (variant_set_digest) = 64),
		signed_campaign_policy_digest TEXT NOT NULL CHECK (length (signed_campaign_policy_digest) = 64),
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_feedback_records (
		feedback_id TEXT PRIMARY KEY,
		track_id TEXT NOT NULL REFERENCES v61_research_tracks (track_id),
		spec_id TEXT NOT NULL REFERENCES v61_experiment_specs (spec_id),
		source_evidence_digest TEXT NOT NULL CHECK (length (source_evidence_digest) = 64),
		filtered_projection_digest TEXT NOT NULL CHECK (length (filtered_projection_digest) = 64),
		redaction_policy_digest TEXT NOT NULL CHECK (length (redaction_policy_digest) = 64),
		released_at TEXT NOT NULL,
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_knowledge_releases (
		release_id TEXT PRIMARY KEY,
		origin_repository TEXT NOT NULL,
		target_repository TEXT NOT NULL,
		source_evidence_set_digest TEXT NOT NULL CHECK (length (source_evidence_set_digest) = 64),
		sanitized_document_digest TEXT NOT NULL CHECK (length (sanitized_document_digest) = 64),
		licensing_decision_digest TEXT NOT NULL CHECK (length (licensing_decision_digest) = 64),
		state TEXT NOT NULL CHECK (
			state IN (
				'pending',
				'authorized',
				'denied',
				'published',
				'revoked'
			)
		),
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_transfer_compatibility (
		compatibility_id TEXT PRIMARY KEY,
		release_id TEXT NOT NULL REFERENCES v61_knowledge_releases (release_id),
		origin_repository TEXT NOT NULL,
		target_repository TEXT NOT NULL,
		verdict TEXT NOT NULL CHECK (
			verdict IN ('allow', 'deny', 'needs-human-review')
		),
		target_experiment_id TEXT,
		evaluated_at TEXT NOT NULL,
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64)
	);

CREATE TABLE
	IF NOT EXISTS v61_denial_audit (
		audit_id TEXT PRIMARY KEY,
		request_id TEXT NOT NULL,
		actor_id TEXT NOT NULL,
		action TEXT NOT NULL,
		reason_code TEXT NOT NULL,
		artifact_digest TEXT NOT NULL CHECK (length (artifact_digest) = 64),
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64),
		worker_notified INTEGER NOT NULL CHECK (worker_notified = 0),
		recorded_at TEXT NOT NULL,
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_temporal_facts (
		fact_id TEXT PRIMARY KEY,
		subject_id TEXT NOT NULL,
		event_time TEXT NOT NULL,
		observed_at TEXT NOT NULL,
		valid_from TEXT NOT NULL,
		valid_to TEXT,
		recorded_at TEXT NOT NULL,
		supersedes_fact_id TEXT REFERENCES v61_temporal_facts (fact_id),
		fact_digest TEXT NOT NULL CHECK (length (fact_digest) = 64),
		source_event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_neutral_receipts (
		receipt_id TEXT PRIMARY KEY,
		task_local_nonce TEXT UNIQUE NOT NULL,
		receipt_digest TEXT NOT NULL CHECK (length (receipt_digest) = 64),
		response_class TEXT NOT NULL CHECK (response_class = 'submission_received'),
		issued_at TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS v61_boundary_observations (
		observation_id TEXT PRIMARY KEY,
		worker_assignment_id TEXT NOT NULL,
		local_action TEXT NOT NULL,
		trusted_internal_cause TEXT NOT NULL,
		worker_feedback_class TEXT NOT NULL,
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64),
		logged_at TEXT NOT NULL,
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

-- Immutable evidence / authorization events; policy versions supersede rather than rewrite history.
CREATE TRIGGER IF NOT EXISTS v61_receipts_no_update BEFORE
UPDATE ON v61_custody_receipts BEGIN
SELECT
	RAISE (ABORT, 'immutable receipt');

END;

CREATE TRIGGER IF NOT EXISTS v61_receipts_no_delete BEFORE DELETE ON v61_custody_receipts BEGIN
SELECT
	RAISE (ABORT, 'immutable receipt');

END;

CREATE TRIGGER IF NOT EXISTS v61_denial_no_update BEFORE
UPDATE ON v61_denial_audit BEGIN
SELECT
	RAISE (ABORT, 'immutable denial audit');

END;

CREATE TRIGGER IF NOT EXISTS v61_denial_no_delete BEFORE DELETE ON v61_denial_audit BEGIN
SELECT
	RAISE (ABORT, 'immutable denial audit');

END;

CREATE TRIGGER IF NOT EXISTS v61_feedback_no_update BEFORE
UPDATE ON v61_feedback_records BEGIN
SELECT
	RAISE (ABORT, 'immutable feedback');

END;

CREATE TRIGGER IF NOT EXISTS v61_feedback_no_delete BEFORE DELETE ON v61_feedback_records BEGIN
SELECT
	RAISE (ABORT, 'immutable feedback');

END;

CREATE TRIGGER IF NOT EXISTS v61_facts_no_update BEFORE
UPDATE ON v61_temporal_facts BEGIN
SELECT
	RAISE (ABORT, 'immutable temporal fact');

END;

CREATE TRIGGER IF NOT EXISTS v61_facts_no_delete BEFORE DELETE ON v61_temporal_facts BEGIN
SELECT
	RAISE (ABORT, 'immutable temporal fact');

END;

CREATE TRIGGER IF NOT EXISTS v61_policies_no_update BEFORE
UPDATE ON v61_artifact_policies BEGIN
SELECT
	RAISE (ABORT, 'immutable policy');

END;

CREATE TRIGGER IF NOT EXISTS v61_policies_no_delete BEFORE DELETE ON v61_artifact_policies BEGIN
SELECT
	RAISE (ABORT, 'immutable policy');

END;

-- Defense-in-depth: a denied decision or wrong holder cannot be used to stage a custody attempt.
CREATE TRIGGER IF NOT EXISTS v61_attempt_requires_authorization BEFORE INSERT ON v61_custody_attempts BEGIN
SELECT
	CASE
		WHEN NOT EXISTS (
			SELECT
				1
			FROM
				v61_custody_requests r
				JOIN v61_custody_decisions d ON d.request_id = r.request_id
				JOIN v61_storage_leases l ON l.lease_id = NEW.lease_id
			WHERE
				r.request_id = NEW.request_id
				AND d.decision_id = NEW.decision_id
				AND d.decision = 'allow'
				AND d.policy_digest = r.policy_digest
				AND l.holder_id = r.target_holder
				AND l.artifact_digest = r.artifact_digest
				AND l.fence = NEW.fence
				AND l.state = 'reserved'
		) THEN RAISE (ABORT, 'custody attempt lacks authorization')
	END;

END;

CREATE TRIGGER IF NOT EXISTS v61_receipt_requires_verification BEFORE INSERT ON v61_custody_receipts BEGIN
SELECT
	CASE
		WHEN NOT EXISTS (
			SELECT
				1
			FROM
				v61_custody_attempts a
				JOIN v61_custody_requests r ON r.request_id = a.request_id
			WHERE
				a.attempt_id = NEW.attempt_id
				AND a.request_id = NEW.request_id
				AND a.state = 'verified'
				AND r.artifact_digest = NEW.artifact_digest
				AND r.target_holder = NEW.target_holder
		) THEN RAISE (ABORT, 'unverified custody receipt')
	END;

END;

-- Software-defined research curriculum, no agent-managed lesson/evaluation policy.
CREATE TABLE
	IF NOT EXISTS v61_curricula (
		curriculum_id TEXT PRIMARY KEY,
		repository_id TEXT NOT NULL,
		baseline_digest TEXT NOT NULL CHECK (length (baseline_digest) = 64),
		curriculum_policy_digest TEXT NOT NULL CHECK (length (curriculum_policy_digest) = 64),
		evaluation_policy_digest TEXT NOT NULL CHECK (length (evaluation_policy_digest) = 64),
		owner_signature_digest TEXT NOT NULL CHECK (length (owner_signature_digest) = 64),
		active_at TEXT NOT NULL,
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_study_assignments (
		study_assignment_id TEXT PRIMARY KEY,
		curriculum_id TEXT NOT NULL REFERENCES v61_curricula (curriculum_id),
		research_track_id TEXT NOT NULL,
		view_id TEXT NOT NULL,
		challenge_family TEXT NOT NULL,
		task_digest TEXT NOT NULL CHECK (length (task_digest) = 64),
		allocation_id TEXT NOT NULL,
		assignment_policy_digest TEXT NOT NULL CHECK (length (assignment_policy_digest) = 64),
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_study_outcomes (
		study_outcome_id TEXT PRIMARY KEY,
		study_assignment_id TEXT NOT NULL REFERENCES v61_study_assignments (study_assignment_id),
		proposal_digest TEXT NOT NULL CHECK (length (proposal_digest) = 64),
		oracle_result_digest TEXT NOT NULL CHECK (length (oracle_result_digest) = 64),
		benchmark_result_digest TEXT CHECK (
			benchmark_result_digest IS NULL
			OR length (benchmark_result_digest) = 64
		),
		verdict TEXT NOT NULL CHECK (
			verdict IN (
				'correct',
				'incorrect',
				'inconclusive',
				'not-applicable'
			)
		),
		evaluation_policy_digest TEXT NOT NULL CHECK (length (evaluation_policy_digest) = 64),
		recorded_at TEXT NOT NULL,
		event_id TEXT REFERENCES v6_event_journal (event_id)
	);

CREATE TABLE
	IF NOT EXISTS v61_physical_allocations (
		allocation_id TEXT PRIMARY KEY,
		research_track_id TEXT NOT NULL,
		holder_id TEXT NOT NULL,
		cpu_cores INTEGER NOT NULL CHECK (cpu_cores >= 0),
		gpu_devices INTEGER NOT NULL CHECK (gpu_devices >= 0),
		memory_bytes INTEGER NOT NULL CHECK (memory_bytes >= 0),
		storage_bytes INTEGER NOT NULL CHECK (storage_bytes >= 0),
		excluded_from_performance_scoring INTEGER NOT NULL CHECK (excluded_from_performance_scoring = 1),
		policy_digest TEXT NOT NULL CHECK (length (policy_digest) = 64),
		active_from TEXT NOT NULL,
		expires_at TEXT NOT NULL
	);

CREATE TRIGGER IF NOT EXISTS v61_study_outcome_no_update BEFORE
UPDATE ON v61_study_outcomes BEGIN
SELECT
	RAISE (ABORT, 'immutable study outcome');

END;

CREATE TRIGGER IF NOT EXISTS v61_study_outcome_no_delete BEFORE DELETE ON v61_study_outcomes BEGIN
SELECT
	RAISE (ABORT, 'immutable study outcome');

END;
