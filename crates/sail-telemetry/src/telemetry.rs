use std::borrow::Cow;
use std::env;
use std::io::Write;

use fastrace::collector::{Config, Reporter, SpanRecord};
use fastrace::prelude::*;
use fastrace_opentelemetry::OpenTelemetryReporter;
use opentelemetry::InstrumentationScope;
use opentelemetry_otlp::{Protocol, WithExportConfig, OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT};
use opentelemetry_sdk::Resource;

use crate::error::TelemetryResult;

pub fn init_telemetry() -> TelemetryResult<()> {
    let use_collector = match env::var("SAIL_OPENTELEMETRY_COLLECTOR") {
        Ok(val) => !val.is_empty(),
        Err(_) => false,
    };
    init_tracer(use_collector)?;
    init_logger(use_collector)?;
    Ok(())
}

pub fn init_tracer(use_collector: bool) -> TelemetryResult<()> {
    let reporter_config = Config::default();
    if use_collector {
        let host = env::var("SAIL_OPENTELEMETRY_COLLECTOR_SERVICE_HOST")?;
        let port = env::var("SAIL_OPENTELEMETRY_COLLECTOR_SERVICE_PORT_OTLP_GRPC")?;
        let url = format!("http://{host}:{port}");
        let reporter = OpenTelemetryReporter::new(
            opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint(url)
                .with_protocol(Protocol::Grpc)
                .with_timeout(OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT)
                .build()?,
            Cow::Owned(Resource::builder().with_service_name("sail_server").build()),
            InstrumentationScope::builder("sail")
                .with_version(env!("CARGO_PKG_VERSION"))
                .build(),
        );
        fastrace::set_reporter(reporter, Config::default());
    } else {
        fastrace::set_reporter(DummyReporter, reporter_config)
    };
    Ok(())
}

pub fn init_logger(use_collector: bool) -> TelemetryResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(move |buf, record| {
            if use_collector {
                let event = Event::new(record.level().as_str()).with_properties(|| {
                    [("message", record.args().to_string())]
                });
                LocalSpan::add_event(event);
            }
            let level = record.level();
            let target = record.target();
            let style = buf.default_level_style(level);
            let timestamp = buf.timestamp();
            let args = record.args();
            if let Some(span_context) = SpanContext::current_local_parent() {
                let trace_id = span_context.trace_id.0;
                let span_id = span_context.span_id.0;
                writeln!(buf, "[{timestamp} {style}{level}{style:#} {target} trace: {trace_id} span: {span_id}] {args}")
            } else {
                writeln!(buf, "[{timestamp} {style}{level}{style:#} {target}] {args}")
            }
        })
        .init();
    Ok(())
}

// We use DummyReporter instead of ConsoleReporter to avoid duplicate span output to the console.
// Although using ConsoleReporter and suppressing span output in the logger is possible, it would
// result in a different format.
// Disabling fastrace when logging only to the console is another option, but retaining trace and
// span IDs in the logs is useful.
pub struct DummyReporter;

impl Reporter for DummyReporter {
    fn report(&mut self, _spans: Vec<SpanRecord>) {}
}
