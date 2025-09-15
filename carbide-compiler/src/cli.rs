use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
pub enum CliCommand {
    Build,
}

#[derive(Parser, Debug)]
#[command(about = "carbide-compiler", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub quiet: bool,
}
