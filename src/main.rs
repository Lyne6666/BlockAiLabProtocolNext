// src/main.rs
/*
 * Main executable for BlockAiLabProtocolNext
 */

use clap::Parser;
use blockailabprotocolnext::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockAiLabProtocolNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
