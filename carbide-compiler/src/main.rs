use anyhow::{Result};
use clap::Parser;

use crate::cli::Cli;

mod errors;
mod cli;


fn main() -> Result<()> {
    let cli = Cli::parse();

    Ok(())
}
