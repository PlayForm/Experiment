-- Apply AFTER v2/sql/lineage.sql and v3/sql/v3_migration.sql.
-- Research economy schema: authoritative immutable history backed by prior artifacts/events.
-- NOTE: Full production signatures, transaction fencing, ledger balance enforcement,
-- authorization and request idempotency belong in a tested Rust transaction layer.
PRAGMA foreign_keys = ON;

CREATE TABLE
	IF NOT EXISTS research_cases (
		id TEXT PRIMARY KEY,
		campaign_id TEXT NOT NULL REFERENCES campaigns (id),
		parent_mutation_id TEXT REFERENCES mutations (id),
		repository_id TEXT NOT NULL,
		question_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		scope_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		workload_id TEXT REFERENCES workloads (id),
		privacy_class TEXT NOT NULL CHECK (privacy_class IN ('private', 'tenant', 'public')),
		preregistration_sha256 TEXT REFERENCES artifacts (sha256),
		status TEXT NOT NULL,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS researcher_assignments (
		id TEXT PRIMARY KEY,
		case_id TEXT NOT NULL REFERENCES research_cases (id),
		work_order_id TEXT REFERENCES work_orders (id),
		actor_identity TEXT NOT NULL,
		role TEXT NOT NULL CHECK (
			role IN (
				'proponent',
				'skeptic',
				'alternative',
				'replicator',
				'methodologist',
				'librarian',
				'editor',
				'integrator',
				'market_analyst'
			)
		),
		model_cohort TEXT NOT NULL,
		privacy_scope_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		blinded_phase INTEGER NOT NULL CHECK (blinded_phase IN (0, 1)),
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS research_bids (
		id TEXT PRIMARY KEY,
		assignment_id TEXT NOT NULL REFERENCES researcher_assignments (id),
		case_id TEXT NOT NULL REFERENCES research_cases (id),
		bid_payload_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		requested_resource_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		prediction_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		signature_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL,
		UNIQUE (assignment_id, bid_payload_sha256)
	);

CREATE TABLE
	IF NOT EXISTS research_claims (
		id TEXT PRIMARY KEY,
		case_id TEXT NOT NULL REFERENCES research_cases (id),
		assignment_id TEXT NOT NULL REFERENCES researcher_assignments (id),
		claim_type TEXT NOT NULL CHECK (
			claim_type IN (
				'opinion',
				'hypothesis',
				'external_reference',
				'observation',
				'interpretation',
				'replication'
			)
		),
		claim_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		context_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		signature_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		supersedes_claim_id TEXT REFERENCES research_claims (id),
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS claim_links (
		from_claim_id TEXT NOT NULL REFERENCES research_claims (id),
		to_claim_id TEXT NOT NULL REFERENCES research_claims (id),
		relation TEXT NOT NULL CHECK (
			relation IN (
				'supports',
				'contradicts',
				'refines',
				'replicates',
				'cites'
			)
		),
		created_utc TEXT NOT NULL,
		PRIMARY KEY (from_claim_id, to_claim_id, relation)
	);

CREATE TABLE
	IF NOT EXISTS research_references (
		id TEXT PRIMARY KEY,
		claim_id TEXT NOT NULL REFERENCES research_claims (id),
		uri TEXT NOT NULL,
		retrieved_utc TEXT NOT NULL,
		source_snapshot_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		citation_scope_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		license_status TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS resource_treasuries (
		id TEXT PRIMARY KEY,
		campaign_id TEXT NOT NULL REFERENCES campaigns (id),
		policy_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL
	);

-- Seven native resource dimensions. Stored independently, not fungible token balances.
CREATE TABLE
	IF NOT EXISTS resource_transactions (
		seq INTEGER PRIMARY KEY AUTOINCREMENT,
		transaction_id TEXT NOT NULL UNIQUE,
		treasury_id TEXT NOT NULL REFERENCES resource_treasuries (id),
		assignment_id TEXT REFERENCES researcher_assignments (id),
		bid_id TEXT REFERENCES research_bids (id),
		request_id TEXT NOT NULL UNIQUE,
		kind TEXT NOT NULL CHECK (
			kind IN (
				'fund',
				'reserve',
				'consume',
				'release',
				'expire',
				'adjust'
			)
		),
		input_tokens INTEGER NOT NULL DEFAULT 0 CHECK (input_tokens >= 0),
		output_tokens INTEGER NOT NULL DEFAULT 0 CHECK (output_tokens >= 0),
		cpu_core_seconds INTEGER NOT NULL DEFAULT 0 CHECK (cpu_core_seconds >= 0),
		benchmark_seconds INTEGER NOT NULL DEFAULT 0 CHECK (benchmark_seconds >= 0),
		gpu_seconds INTEGER NOT NULL DEFAULT 0 CHECK (gpu_seconds >= 0),
		storage_byte_days INTEGER NOT NULL DEFAULT 0 CHECK (storage_byte_days >= 0),
		reference_requests INTEGER NOT NULL DEFAULT 0 CHECK (reference_requests >= 0),
		policy_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		evidence_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		issuer_identity TEXT NOT NULL,
		signature_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL
	);

-- Reservation projections can be updated transactionally; immutable transaction ledger wins.
CREATE TABLE
	IF NOT EXISTS resource_reservations (
		id TEXT PRIMARY KEY,
		treasury_id TEXT NOT NULL REFERENCES resource_treasuries (id),
		bid_id TEXT NOT NULL REFERENCES research_bids (id),
		initial_transaction_id TEXT NOT NULL UNIQUE REFERENCES resource_transactions (transaction_id),
		current_state TEXT NOT NULL CHECK (
			current_state IN (
				'reserved',
				'partially_consumed',
				'settled',
				'released',
				'expired'
			)
		),
		fencing_token INTEGER NOT NULL DEFAULT 1 CHECK (fencing_token > 0),
		expires_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS paper_manuscripts (
		id TEXT PRIMARY KEY,
		repository_id TEXT NOT NULL,
		manuscript_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		methods_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		source_snapshot_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		status TEXT NOT NULL CHECK (
			status IN (
				'hypothesis_note',
				'internal_observation',
				'measured_lab_report',
				'replicated_report',
				'sanitized_preprint',
				'externally_reviewed',
				'corrected',
				'retracted'
			)
		),
		supersedes_id TEXT REFERENCES paper_manuscripts (id),
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS paper_claims (
		manuscript_id TEXT NOT NULL REFERENCES paper_manuscripts (id),
		claim_id TEXT NOT NULL REFERENCES research_claims (id),
		evidence_sha256 TEXT REFERENCES artifacts (sha256),
		strength TEXT NOT NULL,
		PRIMARY KEY (manuscript_id, claim_id)
	);

CREATE TABLE
	IF NOT EXISTS paper_reviews (
		id TEXT PRIMARY KEY,
		manuscript_id TEXT NOT NULL REFERENCES paper_manuscripts (id),
		reviewer_assignment_id TEXT NOT NULL REFERENCES researcher_assignments (id),
		review_kind TEXT NOT NULL CHECK (
			review_kind IN (
				'methodology',
				'replication',
				'citations',
				'sanitization',
				'release_policy'
			)
		),
		outcome TEXT NOT NULL CHECK (
			outcome IN ('pass', 'fail', 'revise', 'inconclusive')
		),
		review_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		signature_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL,
		UNIQUE (
			manuscript_id,
			reviewer_assignment_id,
			review_kind
		)
	);

CREATE TABLE
	IF NOT EXISTS publication_releases (
		id TEXT PRIMARY KEY,
		manuscript_id TEXT NOT NULL REFERENCES paper_manuscripts (id),
		visibility TEXT NOT NULL CHECK (visibility IN ('tenant', 'public')),
		declassification_grant_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		release_manifest_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		issuer_identity TEXT NOT NULL,
		signature_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS integration_cases (
		id TEXT PRIMARY KEY,
		source_release_id TEXT NOT NULL REFERENCES publication_releases (id),
		destination_repo_id TEXT NOT NULL,
		destination_research_case_id TEXT NOT NULL REFERENCES research_cases (id),
		integrator_assignment_id TEXT NOT NULL REFERENCES researcher_assignments (id),
		proposed_transfer_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL,
		UNIQUE (source_release_id, destination_research_case_id)
	);

CREATE INDEX IF NOT EXISTS idx_case_repo ON research_cases (repository_id, campaign_id);

CREATE INDEX IF NOT EXISTS idx_assignment_case ON researcher_assignments (case_id, role);

CREATE INDEX IF NOT EXISTS idx_claim_case ON research_claims (case_id, claim_type);

CREATE INDEX IF NOT EXISTS idx_treasury_transactions ON resource_transactions (treasury_id, seq);

CREATE INDEX IF NOT EXISTS idx_papers_repo ON paper_manuscripts (repository_id, status);

CREATE INDEX IF NOT EXISTS idx_release_paper ON publication_releases (manuscript_id, visibility);

-- Immutable facts: do not edit/delete; record a new event/compensating transaction.
CREATE TRIGGER IF NOT EXISTS deny_res_tx_update BEFORE
UPDATE ON resource_transactions BEGIN
SELECT
	RAISE (ABORT, 'immutable resource transaction');

END;

CREATE TRIGGER IF NOT EXISTS deny_res_tx_delete BEFORE DELETE ON resource_transactions BEGIN
SELECT
	RAISE (ABORT, 'immutable resource transaction');

END;

CREATE TRIGGER IF NOT EXISTS deny_claim_update BEFORE
UPDATE ON research_claims BEGIN
SELECT
	RAISE (ABORT, 'immutable research claim');

END;

CREATE TRIGGER IF NOT EXISTS deny_claim_delete BEFORE DELETE ON research_claims BEGIN
SELECT
	RAISE (ABORT, 'immutable research claim');

END;

CREATE TRIGGER IF NOT EXISTS deny_release_update BEFORE
UPDATE ON publication_releases BEGIN
SELECT
	RAISE (ABORT, 'immutable publication release');

END;

CREATE TRIGGER IF NOT EXISTS deny_release_delete BEFORE DELETE ON publication_releases BEGIN
SELECT
	RAISE (ABORT, 'immutable publication release');

END;

CREATE TRIGGER IF NOT EXISTS deny_review_update BEFORE
UPDATE ON paper_reviews BEGIN
SELECT
	RAISE (ABORT, 'immutable review');

END;

CREATE TRIGGER IF NOT EXISTS deny_review_delete BEFORE DELETE ON paper_reviews BEGIN
SELECT
	RAISE (ABORT, 'immutable review');

END;

CREATE TRIGGER IF NOT EXISTS deny_bid_update BEFORE
UPDATE ON research_bids BEGIN
SELECT
	RAISE (ABORT, 'immutable bid');

END;

CREATE TRIGGER IF NOT EXISTS deny_bid_delete BEFORE DELETE ON research_bids BEGIN
SELECT
	RAISE (ABORT, 'immutable bid');

END;

CREATE TRIGGER IF NOT EXISTS deny_manuscript_update BEFORE
UPDATE ON paper_manuscripts BEGIN
SELECT
	RAISE (ABORT, 'immutable manuscript');

END;

CREATE TRIGGER IF NOT EXISTS deny_manuscript_delete BEFORE DELETE ON paper_manuscripts BEGIN
SELECT
	RAISE (ABORT, 'immutable manuscript');

END;
