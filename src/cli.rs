use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

/// Eidos — a simple markdown note manager with LLM-powered creation.
#[derive(Parser)]
#[command(name = "eidos", version, about)]
#[command(arg_required_else_help = true)]
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
    /// Start an interactive REPL session
    Repl,
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

/// Run the interactive REPL loop.
pub async fn repl(eidos: &crate::Eidos) {
    use crate::llm::LLM;
    use colored::*;

    let llm = LLM::new(eidos);

    loop {
        print!("{} ", "eidos>".cyan().bold());
        use std::io::{self, Write};
        io::stdout().flush().ok();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => break, // EOF or error → exit
            Ok(_) if line.trim().is_empty() => continue,
            Ok(_) => {}
        }

        let line = line.trim();
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        let cmd = parts[0];
        let arg = parts.get(1).map(|s| s.trim());

        match cmd {
            "list" | "ls" => {
                let notes = eidos.notes().await;
                if notes.is_empty() {
                    println!("{}", "No notes found.".yellow());
                } else {
                    for note in &notes {
                        let title = note.title();
                        let preview = note.content().lines().next().unwrap_or("");
                        let preview = if preview.len() > 60 {
                            format!("{}...", &preview[..60])
                        } else {
                            preview.to_string()
                        };
                        println!("  {}  {}", title.cyan().bold(), preview.dimmed());
                    }
                    println!("{}", format!("\n{} note(s) total", notes.len()).dimmed());
                }
            }
            "read" | "cat" => {
                let title = arg.unwrap_or("");
                if title.is_empty() {
                    println!("{}", "Usage: read <title>".red());
                    continue;
                }
                let note = eidos.find_note(title).await;
                match note {
                    Some(n) => println!("{}", n.display()),
                    None => println!("{}", format!("Note '{}' not found.", title).red()),
                }
            }
            "create" | "new" => {
                let title = arg.unwrap_or("");
                if title.is_empty() {
                    println!("{}", "Usage: create <title>".red());
                    continue;
                }
                println!("{} (Ctrl+D to finish):", "Enter content".green());
                let mut content = String::new();
                use std::io::{self, BufRead};
                for line_result in io::stdin().lock().lines() {
                    match line_result {
                        Ok(l) => {
                            content.push_str(&l);
                            content.push('\n');
                        }
                        Err(_) => break,
                    }
                }
                let content = content.trim().to_string();
                eidos.create_note(title.to_string(), content).await;
                println!("{}", format!("Created note '{}'.", title).green());
            }
            "ask" => {
                let prompt = arg.unwrap_or("");
                if prompt.is_empty() {
                    println!("{}", "Usage: ask <prompt>".red());
                    continue;
                }
                println!("{}", "Thinking...".yellow());
                let result = llm.create_note(prompt).await;
                println!("{}", result.dimmed());
            }
            "help" | "h" | "?" => {
                println!("{}", "Commands:".bold());
                println!(
                    "  {}  {}  {}",
                    "list".cyan(),
                    "(ls)".dimmed(),
                    "List all notes".dimmed()
                );
                println!(
                    "  {}  {}  {}",
                    "read <title>".cyan(),
                    "(cat)".dimmed(),
                    "Read a note".dimmed()
                );
                println!(
                    "  {}  {}  {}",
                    "create <title>".cyan(),
                    "(new)".dimmed(),
                    "Create a note".dimmed()
                );
                println!(
                    "  {}  {}  {}",
                    "ask <prompt>".cyan(),
                    "".dimmed(),
                    "Ask the LLM to create notes".dimmed()
                );
                println!(
                    "  {}  {}          {}",
                    "help".cyan(),
                    "(h/?)".dimmed(),
                    "Show this help".dimmed()
                );
                println!(
                    "  {}  {}          {}",
                    "quit".cyan(),
                    "(q)".dimmed(),
                    "Exit".dimmed()
                );
            }
            "quit" | "q" | "exit" => {
                println!("{}", "Goodbye.".cyan());
                break;
            }
            _ => {
                println!(
                    "{}",
                    format!("Unknown command '{}'. Type 'help' for commands.", cmd).red()
                );
            }
        }
    }
}
