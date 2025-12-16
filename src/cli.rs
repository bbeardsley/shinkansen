use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// CLI arguments structure
#[derive(Parser, Debug)]
#[command(name = "shinkansen")]
#[command(version = VERSION)]
#[command(about = DESCRIPTION, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Input files or directories to process (defaults to stdin if not specified)
    #[arg(value_name = "INPUT")]
    pub inputs: Vec<String>,

    /// Recursively process directories
    #[arg(short, long)]
    pub recursive: bool,

    /// Output file or directory (use '-' for stdout)
    #[arg(short, long, value_name = "OUTPUT")]
    pub output: Option<String>,

    /// Template variables as key=value pairs
    /// Supports escaping special characters: \\ (backslash), \, (comma), \= (equals)
    /// Multiple variables can be specified in one flag separated by commas: -D "a=1,b=2"
    #[arg(short = 'D', long = "define", value_name = "KEY=VALUE")]
    pub variables: Vec<String>,

    /// Configuration file (JSON, YAML, or TOML) containing template variables
    #[arg(short, long, value_name = "CONFIG")]
    pub config: Option<PathBuf>,

    /// Load specific environment variables (comma-separated)
    #[arg(long, value_name = "VARS")]
    pub env: Option<String>,
}

/// Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completion scripts
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}
