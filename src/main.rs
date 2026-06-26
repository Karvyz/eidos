mod cli;
mod eidos;
mod llm;
mod note;

use cli::{Commands, parse, validate_directory};
use colored::*;
use eidos::Eidos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse();

    if let Err(e) = validate_directory(&args.dir) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }

    let start = std::time::Instant::now();
    let eidos = Eidos::read(&args.dir).await;
    let duration = start.elapsed();

    match &args.command {
        Some(Commands::List) => cmd_list(&eidos).await,
        Some(Commands::Read { title }) => cmd_read(&eidos, title).await,
        Some(Commands::Create { title, content }) => {
            cmd_create(&eidos, title, content.as_deref()).await
        }
        Some(Commands::Ask { prompt }) => cmd_ask(&eidos, prompt).await,
        Some(Commands::Repl) | None => {
            println!(
                "{} {} {} {}",
                "Loaded".dimmed(),
                format!("{}", eidos.len().await).cyan(),
                "notes from".dimmed(),
                args.dir.display().to_string().cyan().underline(),
            );
            println!(
                "{} {:.2?} {}",
                "(read took".dimmed(),
                duration,
                ")".dimmed(),
            );
            cli::repl(&eidos).await;
        }
    }

    Ok(())
}

async fn cmd_list(eidos: &Eidos) {
    let notes = eidos.notes().await;
    if notes.is_empty() {
        println!("{}", "No notes found.".yellow());
        return;
    }
    for note in &notes {
        let preview = note.content().lines().next().unwrap_or("").trim();
        let preview = if preview.len() > 60 {
            format!("{}...", &preview[..60])
        } else {
            preview.to_string()
        };
        println!("  {}  {}", note.title().cyan().bold(), preview.dimmed());
    }
    println!("{}", format!("\n{} note(s) total", notes.len()).dimmed());
}

async fn cmd_read(eidos: &Eidos, title: &str) {
    match eidos.find_note(title).await {
        Some(note) => println!("{}", note.display()),
        None => {
            eprintln!(
                "{} {}",
                "Error:".red().bold(),
                format!("note '{}' not found", title).red()
            );
            std::process::exit(1);
        }
    }
}

async fn cmd_create(eidos: &Eidos, title: &str, content: Option<&[String]>) {
    let content = match content {
        Some(lines) => lines.join(" "),
        None => {
            println!(
                "{} {}",
                "Enter content".green(),
                "(Ctrl+D to finish):".dimmed()
            );
            let mut buf = String::new();
            for line in std::io::stdin().lines() {
                match line {
                    Ok(l) => {
                        buf.push_str(&l);
                        buf.push('\n');
                    }
                    Err(_) => break,
                }
            }
            buf.trim().to_string()
        }
    };
    eidos.create_note(title.to_string(), content).await;
    println!("{}", format!("Created note '{}'.", title).green());
}

async fn cmd_ask(eidos: &Eidos, prompt: &str) {
    let llm = crate::llm::LLM::new();
    let result = llm.create_note(prompt, eidos.clone()).await;
    println!("{}", result);
}
