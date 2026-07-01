use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

/// Eidos — a simple markdown note manager with LLM-powered creation.
#[derive(Parser)]
#[command(name = "eidos", version, about)]
pub struct Cli {
    /// Directory containing .md notes (overrides config vault_path)
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all notes in the directory
    List,
    /// Read a note by its title (filename without .md)
    Read {
        /// Title of the note to read
        title: String,
    },
    /// Create a new note (content from second arg or stdin)
    Create {
        /// Title of the new note (no extension)
        title: String,
        /// Optional content body; reads from stdin if omitted
        content: Option<Vec<String>>,
    },
    /// Ask the LLM to create notes from a prompt
    Ask {
        /// Prompt describing what note(s) to create
        prompt: String,
    },
}

/// Validate that a path exists and is a directory.
pub fn validate_directory(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("path '{}' does not exist", path.display()));
    }
    if !path.is_dir() {
        return Err(format!("'{}' is not a directory", path.display()));
    }
    Ok(())
}

/// Parse CLI arguments.
pub fn parse() -> Cli {
    Cli::parse()
}
