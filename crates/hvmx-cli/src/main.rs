// ==============================================================================
// HVMX - High-order Virtual Machine eXtreme
// ==============================================================================
// File: main.rs
// Location: src/main.rs
// Purpose: HVMX CLI
// Author: scoobiii
// Date: 2025-12-28
// License: MIT OR Apache-2.0
// ==============================================================================


use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hvmx")]
#[command(about = "HVMX - High-order Virtual Machine eXtreme")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show version
    Version,
    /// Show project info
    Info,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Version => {
            println!("HVMX v1.0.0");
        }
        Commands::Info => {
            println!("HVMX - High-order Virtual Machine eXtreme");
            println!("Author: scoobiii");
            println!("License: MIT OR Apache-2.0");
        }
    }
}
