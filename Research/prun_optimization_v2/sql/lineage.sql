-- Illustrative SQLite 3 lineage schema, not a complete deployed migration.
PRAGMA foreign_keys = ON;

PRAGMA journal_mode = WAL;

CREATE TABLE
	IF NOT EXISTS campaigns (
		id TEXT PRIMARY KEY,
		schema_version INTEGER NOT NULL CHECK (schema_version >= 1),
		created_utc TEXT NOT NULL,
		manifest_sha256 TEXT NOT NULL CHECK (length (manifest_sha256) = 64),
		state TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS artifacts (
		sha256 TEXT PRIMARY KEY CHECK (length (sha256) = 64),
		kind TEXT NOT NULL,
		byte_size INTEGER NOT NULL CHECK (byte_size >= 0),
		relative_path TEXT NOT NULL,
		created_utc TEXT NOT NULL,
		metadata_json TEXT NOT NULL DEFAULT '{}'
	);

CREATE TABLE
	IF NOT EXISTS hypotheses (
		id TEXT PRIMARY KEY,
		campaign_id TEXT NOT NULL REFERENCES campaigns (id),
		stage TEXT NOT NULL,
		text TEXT NOT NULL,
		parent_analysis_id TEXT,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS mutations (
		id TEXT PRIMARY KEY,
		campaign_id TEXT NOT NULL REFERENCES campaigns (id),
		hypothesis_id TEXT REFERENCES hypotheses (id),
		parent_mutation_id TEXT REFERENCES mutations (id),
		kind TEXT NOT NULL,
		changed_stage TEXT NOT NULL,
		source_commit TEXT NOT NULL,
		parent_commit TEXT NOT NULL,
		patch_sha256 TEXT REFERENCES artifacts (sha256),
		author_agent_id TEXT,
		state TEXT NOT NULL,
		created_utc TEXT NOT NULL
	);

-- Merge/integration ancestry is an explicit many-to-many relation.
CREATE TABLE
	IF NOT EXISTS mutation_parents (
		child_mutation_id TEXT NOT NULL REFERENCES mutations (id),
		parent_mutation_id TEXT NOT NULL REFERENCES mutations (id),
		purpose TEXT NOT NULL CHECK (purpose IN ('causal', 'merge')),
		PRIMARY KEY (child_mutation_id, parent_mutation_id)
	);

CREATE TABLE
	IF NOT EXISTS builds (
		id TEXT PRIMARY KEY,
		mutation_id TEXT NOT NULL REFERENCES mutations (id),
		recipe_sha256 TEXT NOT NULL,
		source_commit TEXT NOT NULL,
		binary_sha256 TEXT REFERENCES artifacts (sha256),
		builder_image_digest TEXT NOT NULL,
		target_triple TEXT NOT NULL,
		target_cpu TEXT NOT NULL,
		status TEXT NOT NULL,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS workloads (
		id TEXT PRIMARY KEY,
		fixture_sha256 TEXT NOT NULL,
		command_spec_sha256 TEXT NOT NULL,
		oracle_version TEXT NOT NULL,
		split TEXT NOT NULL CHECK (
			split IN (
				'training',
				'development',
				'holdout',
				'sealed_holdout'
			)
		),
		metadata_json TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS environments (
		id TEXT PRIMARY KEY,
		hardware_fingerprint_sha256 TEXT NOT NULL,
		os_kernel TEXT NOT NULL,
		runner_oci_digest TEXT NOT NULL,
		cpu_allocation_json TEXT NOT NULL,
		container_settings_sha256 TEXT NOT NULL,
		metadata_json TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS experiments (
		id TEXT PRIMARY KEY,
		campaign_id TEXT NOT NULL REFERENCES campaigns (id),
		build_id TEXT NOT NULL REFERENCES builds (id),
		workload_id TEXT NOT NULL REFERENCES workloads (id),
		environment_id TEXT NOT NULL REFERENCES environments (id),
		protocol_sha256 TEXT NOT NULL,
		configured_candidate_id TEXT NOT NULL,
		status TEXT NOT NULL,
		created_utc TEXT NOT NULL,
		UNIQUE (
			build_id,
			workload_id,
			environment_id,
			protocol_sha256
		)
	);

CREATE TABLE
	IF NOT EXISTS oracle_results (
		id TEXT PRIMARY KEY,
		experiment_id TEXT NOT NULL REFERENCES experiments (id),
		oracle_version TEXT NOT NULL,
		verdict TEXT NOT NULL CHECK (verdict IN ('pass', 'fail', 'inconclusive')),
		evidence_sha256 TEXT REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS trials (
		id TEXT PRIMARY KEY,
		experiment_id TEXT NOT NULL REFERENCES experiments (id),
		reference_experiment_id TEXT REFERENCES experiments (id),
		order_seed INTEGER NOT NULL,
		paired_block_index INTEGER NOT NULL,
		raw_sample_sha256 TEXT REFERENCES artifacts (sha256),
		status TEXT NOT NULL,
		invalid_reason TEXT,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS analyses (
		id TEXT PRIMARY KEY,
		experiment_id TEXT NOT NULL REFERENCES experiments (id),
		baseline_experiment_id TEXT NOT NULL REFERENCES experiments (id),
		oracle_result_id TEXT NOT NULL REFERENCES oracle_results (id),
		analysis_method_version TEXT NOT NULL,
		output_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		verdict TEXT NOT NULL CHECK (
			verdict IN ('win', 'loss', 'tie', 'inconclusive', 'invalid')
		),
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS decisions (
		id TEXT PRIMARY KEY,
		analysis_id TEXT NOT NULL REFERENCES analyses (id),
		action TEXT NOT NULL CHECK (
			action IN (
				'reject',
				'retry',
				'confirm',
				'eligible',
				'review_required',
				'promote',
				'revert'
			)
		),
		reason TEXT NOT NULL,
		actor TEXT NOT NULL,
		created_utc TEXT NOT NULL
	);

CREATE TABLE
	IF NOT EXISTS events (
		sequence INTEGER PRIMARY KEY AUTOINCREMENT,
		event_id TEXT NOT NULL UNIQUE,
		campaign_id TEXT REFERENCES campaigns (id),
		entity_kind TEXT NOT NULL,
		entity_id TEXT NOT NULL,
		event_kind TEXT NOT NULL,
		event_schema_version INTEGER NOT NULL,
		payload_sha256 TEXT NOT NULL REFERENCES artifacts (sha256),
		created_utc TEXT NOT NULL
	);

CREATE INDEX IF NOT EXISTS idx_mutations_parent ON mutations (parent_mutation_id);

CREATE INDEX IF NOT EXISTS idx_mutations_campaign ON mutations (campaign_id);

CREATE INDEX IF NOT EXISTS idx_experiments_workload ON experiments (workload_id);

CREATE INDEX IF NOT EXISTS idx_trials_experiment ON trials (experiment_id);

CREATE INDEX IF NOT EXISTS idx_events_campaign ON events (campaign_id, sequence);

-- Enforce immutability of event history: append only, no UPDATE/DELETE.
CREATE TRIGGER IF NOT EXISTS deny_events_update BEFORE
UPDATE ON events BEGIN
SELECT
	RAISE (ABORT, 'immutable evidence event');

END;

CREATE TRIGGER IF NOT EXISTS deny_events_delete BEFORE DELETE ON events BEGIN
SELECT
	RAISE (ABORT, 'immutable evidence event');

END;
