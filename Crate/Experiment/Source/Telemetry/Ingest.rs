//! OTLP ingest and self-observation of the telemetry pipeline.
//!
//! Grounded in `telemetry/otel-collector-optional.yaml:4-40` (separate control
//! and product receivers on distinct ports, memory limiter, batch processors),
//! `TELEMETRY.md:115-141` (Rust axum/tonic receiver makes the Go Collector
//! optional) and `REQUIREMENTS_V3.md` TEL-009/TEL-011/TEL-012. Exporter
//! failures and dropped spans are recorded; missing spans never invalidate raw
//! timing facts.

/// Configuration of a Rust OTLP ingest service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ingest {
    /// Control-plane gRPC port (`otel-collector-optional.yaml:6-8`).
    pub control_grpc: u16,
    /// Control-plane HTTP port (`:9-10`).
    pub control_http: u16,
    /// Product-plane gRPC port (`:11-13`).
    pub product_grpc: u16,
    /// Product-plane HTTP port (`:14-16`).
    pub product_http: u16,
    /// Memory limiter ceiling in MiB (`:18-21`).
    pub limit_mib: u32,
    /// Number of spans dropped by this ingest service so far.
    pub dropped_spans: u64,
}

impl Ingest {
    /// The reference configuration from the optional Collector yaml.
    #[must_use]
    pub fn reference() -> Self {
        Self {
            control_grpc: 4317,
            control_http: 4318,
            product_grpc: 14317,
            product_http: 14318,
            limit_mib: 512,
            dropped_spans: 0,
        }
    }

    /// Record dropped spans without panicking or fabricating timing evidence.
    pub fn note_drop(&mut self, count: u64) {
        self.dropped_spans = self.dropped_spans.saturating_add(count);
    }

    /// Whether control and product endpoints are kept distinct
    /// (`REQUIREMENTS_V3.md` TEL-001).
    #[must_use]
    pub const fn planes_separated(&self) -> bool {
        self.control_grpc != self.product_grpc && self.control_http != self.product_http
    }
}
