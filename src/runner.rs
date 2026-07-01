use std::{
    collections::VecDeque,
    io::{self, Write},
    path::PathBuf,
};

use colored::Colorize;

use crate::{eidos::Eidos, llm::LLM};

pub enum Commands {
    Ask { prompt: String },
    List,
    Display { title: String },
    Create { title: String, content: String },
    Exit { verbose: bool },
    None,
}

pub struct Runner {
    instructions: VecDeque<Commands>,
    eidos: Eidos,
    llm: LLM,
}

impl Runner {
    pub async fn new(vault: PathBuf, commands: Vec<Commands>) -> Self {
        let eidos = Eidos::read(vault).await;
        let llm = LLM::new(&eidos);
        Self {
            instructions: commands.into(),
            eidos,
            llm,
        }
    }

    pub async fn run(&mut self) {
        loop {
            while let Some(instruction) = self.instructions.pop_front() {
                match instruction {
                    Commands::Ask { prompt } => self.ask(prompt).await,
                    Commands::List => self.list().await,
                    Commands::Display { title } => self.display(title).await,
                    Commands::Create { title, content } => self.create(title, content).await,
                    Commands::Exit { verbose } => {
                        verbose.then(|| println!("Goodbye"));
                        return;
                    }
                    Commands::None => (),
                }
            }
            self.instructions.push_back(self.parse().await)
        }
    }

    async fn parse(&self) -> Commands {
        print!("{} ", "eidos>".cyan().bold());
        io::stdout().flush().ok();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => return Commands::Exit { verbose: false }, // EOF or error → exit
            Ok(_) if line.trim().is_empty() => return Commands::None,
            Ok(_) => {}
        }

        let line = line.trim();
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        let cmd = parts[0];
        let arg = parts.get(1).map(|s| s.trim());

        match cmd {
            "list" | "ls" => Commands::List,
            "read" | "cat" => {
                let title = arg.unwrap_or("").to_string();
                match title.is_empty() {
                    true => {
                        println!("{}", "Usage: read <title>".red());
                        Commands::None
                    }
                    false => Commands::Display { title },
                }
            }
            "create" | "new" => {
                let title = arg.unwrap_or("").to_string();
                if title.is_empty() {
                    println!("{}", "Usage: create <title>".red());
                    return Commands::None;
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
                Commands::Create { title, content }
            }
            "ask" => {
                let prompt = arg.unwrap_or("").to_string();
                match prompt.is_empty() {
                    true => {
                        println!("{}", "Usage: ask <prompt>".red());
                        Commands::None
                    }
                    false => Commands::Ask { prompt },
                }
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
                Commands::None
            }
            "quit" | "q" | "exit" => Commands::Exit { verbose: true },
            _ => {
                println!(
                    "{}",
                    format!("Unknown command '{}'. Type 'help' for commands.", cmd).red()
                );
                Commands::None
            }
        }
    }

    async fn ask(&self, prompt: String) {
        let response = self.llm.ask(&prompt).await;
        println!("{}", response);
    }

    async fn list(&self) {
        let notes = self.eidos.notes().await;
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

    async fn display(&self, title: String) {
        let note = self.eidos.find_note(&title).await;
        match note {
            Some(n) => println!("{}", n.display()),
            None => println!("{}", format!("Note '{}' not found.", title).red()),
        }
    }

    async fn create(&self, title: String, content: String) {
        match self.eidos.create_note(title, content).await {
            Ok(path) => println!("Note created in {:?}", path),
            Err(e) => println!("{}", format!("Error creating note : {}", e).red()),
        }
    }
}
