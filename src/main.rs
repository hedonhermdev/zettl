use anyhow::Result;

mod cli;
mod commands;
mod config;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    cli::CLI::run().await
}
