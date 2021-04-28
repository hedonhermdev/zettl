use anyhow::Result;

mod cli;
mod commands;
mod config;
mod utils;

fn main() -> Result<()> {
    cli::CLI::run()
}
