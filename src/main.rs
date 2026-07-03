mod cli;
mod target;
mod scanner;
mod packet;
mod output;
mod utils;
mod raw_socket;
mod fingerprint;
mod resume;
mod tcp_opt;
mod service;

use anyhow::Result;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
// initialized logging based on environment variables 
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

// parsing argument fr user
    let args = cli::parse();

//  run engine scan main asynchronously based on arguments
    let results = scanner::engine::run(args.clone()).await?;

// hm format and ekspor result scanning output
    output::output_results(&results, &args)?;
    Ok(())
}