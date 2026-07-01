mod cli;
mod config;
mod eidos;
mod llm;
mod note;
mod runner;

use cli::{parse, validate_directory};
use colored::*;

use crate::{config::Config, runner::Runner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse();

    // Load config from XDG path, then resolve vault directory:
    // CLI --dir overrides config vault_path, which itself defaults to "."
    let cfg = Config::load();
    let vault = match args.dir.or_else(|| cfg.and_then(|c| c.vault_path.clone())) {
        Some(path) => path,
        None => {
            eprintln!("{}", "Vault not configured".red().bold());
            return Ok(());
        }
    };

    if let Err(e) = validate_directory(&vault) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }

    let commands = cli_to_runner(args.command);

    let mut runner = Runner::new(vault.clone(), commands).await;
    runner.run().await;

    Ok(())
}

fn cli_to_runner(cli_command: Option<cli::Commands>) -> Vec<runner::Commands> {
    let mut commands = vec![];
    if let Some(cli_command) = cli_command {
        commands.push(match cli_command {
            cli::Commands::List => runner::Commands::List,
            cli::Commands::Read { title } => runner::Commands::Display { title },
            cli::Commands::Create { title, content } => runner::Commands::Create {
                title,
                content: content.map_or(String::new(), |s| s.concat()),
            },
            cli::Commands::Ask { prompt } => runner::Commands::Ask { prompt },
        });
        commands.push(runner::Commands::Exit { verbose: false });
    }
    commands
}
