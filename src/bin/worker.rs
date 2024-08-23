use crypto_helper::diff::{DiffTask, JsonCodec};
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_web::{performance_layer, MakeWebConsoleWriter};
use yew_agent::Registrable;

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new());
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .with(EnvFilter::from_default_env())
        .init();

    DiffTask::registrar().encoding::<JsonCodec>().register();
}
