pub mod cli;
pub mod config;
pub mod error;
pub mod output;
pub mod platform;
pub mod processing;
pub mod rendering;
pub mod variables;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::{Cli, Commands};
use std::io;

fn main() {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        match command {
            Commands::Completions { shell } => {
                let mut cmd = Cli::command();
                let name = cmd.get_name().to_string();
                generate(shell, &mut cmd, name, &mut io::stdout());
                return;
            }
        }
    }

    // Validate input/output combinations
    if let Err(err) = processing::validate_args(&cli) {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    // Collect template variables with proper precedence
    let variables = match variables::collect_variables(&cli) {
        Ok(vars) => vars,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Process inputs
    if let Err(err) = processing::process_inputs(&cli, &variables) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
