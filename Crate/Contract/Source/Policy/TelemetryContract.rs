//! Dual telemetry contracts as versioned data.
//!
//! `policy/telemetry.json:1-152` declares a control pipeline (lab.control,
//! mTLS OTLP/gRPC, enabled by default, 21 spans, 15 metrics) and a product
//! pipeline (prun.sut, separate OTLP/gRPC, disabled by default, four modes,
//! trace-off authoritative). Deny attributes always override allow attributes.
//! No span, metric or OTLP message is emitted here.

/// One telemetry pipeline contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TelemetryContract {
    /// Service name.
    pub service_name: &'static str,
    /// Export mechanism.
    pub export: &'static str,
    /// Whether the pipeline is enabled by default.
    pub default_enabled: crate::Permission,
    /// Allowed attribute names.
    pub allowed: &'static [&'static str],
    /// Denied attribute names; these always win.
    pub denied: &'static [&'static str],
    /// Declared span names.
    pub spans: &'static [&'static str],
    /// Declared metric names.
    pub metrics: &'static [&'static str],
    /// Declared trace modes.
    pub modes: &'static [&'static str],
    /// Authoritative benchmark mode.
    pub authoritative_mode: &'static str,
    /// Cross-plane correlation attributes.
    pub correlation: &'static [&'static str],
}

impl TelemetryContract {
    /// The trusted control pipeline.
    #[must_use]
    pub const fn control() -> Self {
        Self {
            service_name: "lab.control",
            export: "otlp-grpc",
            default_enabled: crate::Permission::Allow,
            allowed: &[
                "lab.campaign_id", "lab.experiment_id", "lab.mutation_id", "lab.job_id",
                "lab.task_id", "lab.worker_role", "lab.model_cohort_hash", "lab.host_id",
                "lab.lease_id", "lab.fencing_token", "lab.event_id", "lab.state_from",
                "lab.state_to", "lab.result_code", "lab.build_recipe_digest",
                "lab.artifact_digest", "lab.queue_depth", "lab.event_seq",
                "messaging.system", "messaging.operation.type",
            ],
            denied: &[
                "prompt", "response", "source", "patch", "secret", "private_key",
                "raw_stdout", "raw_stderr", "command_argv", "raw_tool_args",
                "http.request.body",
            ],
            spans: &[
                "lab.command.validate", "lab.state.transition", "lab.queue.dispatch",
                "lab.worker.lease", "lab.supervisor.stage", "lab.inference.invoke",
                "lab.worker.output.seal", "lab.novelty.check", "lab.build.prepare",
                "lab.build.execute", "lab.oracle.verify", "lab.benchmark.reserve",
                "lab.benchmark.run", "lab.evaluation.evaluate", "lab.feedback.prepare",
                "lab.pki.issue", "lab.pki.revoke", "lab.paper.sanitize",
                "lab.publication.review", "lab.cas.commit", "lab.event.append",
            ],
            metrics: &[
                "lab.jobs.queue.depth", "lab.jobs.active", "lab.leases.held",
                "lab.bench_host.exclusive_occupancy", "lab.agent.task_duration",
                "lab.inference.wall_duration", "lab.build.wall_duration",
                "lab.oracle.pass_count", "lab.oracle.failure_count",
                "lab.benchmark.contamination_count", "lab.ledger.append_latency",
                "lab.pki.credentials.issued", "lab.pki.credentials.revoked",
                "lab.artifact.bytes_total", "lab.telemetry.dropped_spans",
            ],
            modes: &[],
            authoritative_mode: "",
            correlation: &[
                "lab.campaign_id", "lab.experiment_id", "lab.mutation_id",
                "sut.binary_digest", "sut.workload_digest",
            ],
        }
    }

    /// The isolated product diagnostic pipeline.
    #[must_use]
    pub const fn product() -> Self {
        Self {
            service_name: "prun.sut",
            export: "separate-otlp-grpc",
            default_enabled: crate::Permission::Deny,
            allowed: &[
                "lab.experiment_id", "lab.mutation_id", "lab.sample_id",
                "sut.binary_digest", "sut.workload_digest", "sut.trace_mode",
                "sut.stage", "sut.strategy_id", "sut.threads",
                "sut.host_cohort_hash", "sut.exit_code",
            ],
            denied: &[
                "child_command", "child_argv", "input_path", "user_home",
                "environment", "file_contents", "stdout", "stderr",
            ],
            spans: &[
                "sut.command.parse", "sut.heuristic.analyze", "sut.plan.select",
                "sut.walk.discover", "sut.glob.match", "sut.scheduler.enqueue",
                "sut.process.spawn", "sut.process.wait", "sut.output.forward",
                "sut.command.total",
            ],
            metrics: &[
                "sut.elapsed_ns", "sut.cpu_user_ns", "sut.cpu_system_ns",
                "sut.peak_rss_bytes", "sut.allocated_bytes", "sut.minor_faults",
                "sut.major_faults", "sut.files_visited", "sut.matches",
                "sut.children_spawned", "sut.errors",
            ],
            modes: &["off", "lite", "diagnostic", "profile"],
            authoritative_mode: "off",
            correlation: &[
                "lab.campaign_id", "lab.experiment_id", "lab.mutation_id",
                "sut.binary_digest", "sut.workload_digest",
            ],
        }
    }

    /// Decide whether an attribute may be exported: deny wins.
    #[must_use]
    pub fn admits(&self, attribute: &str) -> crate::Permission {
        match (
            self.denied.iter().any(|denied| *denied == attribute),
            self.allowed.iter().any(|allowed| *allowed == attribute),
        ) {
            (true, _) | (false, false) => crate::Permission::Deny,
            (false, true) => crate::Permission::Allow,
        }
    }

    /// Privacy invariants from `policy/telemetry.json:145-151`.
    #[must_use]
    pub const fn privacy_invariants() -> &'static [&'static str] {
        &[
            "no_raw_prompt_in_otel",
            "no_raw_source_in_otel",
            "no_dynamic_filepaths_in_otel",
            "trace_context_not_exported_to_llm",
            "metadata_identity_separation",
        ]
    }
}
