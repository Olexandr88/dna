use apibara_dna_starknet::{cli::Cli, error::StarknetError};
use apibara_observability::init_opentelemetry;
use clap::Parser;
use error_stack::{Result, ResultExt};
use mimalloc::MiMalloc;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<(), StarknetError> {
    let args = Cli::parse();
    run_with_args(args).await
}

async fn run_with_args(args: Cli) -> Result<(), StarknetError> {
    init_opentelemetry(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .change_context(StarknetError)
        .attach_printable("failed to initialize opentelemetry")?;

    let ct = CancellationToken::new();

    ctrlc::set_handler({
        let ct = ct.clone();
        move || {
            info!("SIGINT received");
            ct.cancel();
        }
    })
    .change_context(StarknetError)
    .attach_printable("failed to set SIGINT handler")?;

    args.run(ct).await
}
