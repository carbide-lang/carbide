use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

mod cli;
mod errors;

fn main() -> Result<()> {
    let cli = Cli::parse();

    Ok(())
}
