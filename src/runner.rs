use std::{collections::VecDeque, path::PathBuf};

use colored::Colorize;
use rustyline::{Config, error::ReadlineError};

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
    rl: rustyline::Editor<(), rustyline::history::FileHistory>,
}

impl Runner {
    pub async fn new(vault: PathBuf, commands: Vec<Commands>) -> Self {
        let start = std::time::Instant::now();
        let eidos = Eidos::read(vault.clone()).await;
        let duration = start.elapsed();
        let llm = LLM::new(&eidos);

        let config = Config::builder().edit_mode(rustyline::EditMode::Vi).build();
        let rl = rustyline::DefaultEditor::with_config(config).unwrap();

        if commands.is_empty() {
            println!(
                "{} {} {} {}",
                "Loaded".dimmed(),
                format!("{}", eidos.len().await).cyan(),
                "notes from".dimmed(),
                vault.display().to_string().cyan().underline(),
            );
            println!(
                "{} {:.2?} {}",
                "(read took".dimmed(),
                duration,
                ")".dimmed(),
            );
        }

        Self {
            instructions: commands.into(),
            eidos,
            llm,
            rl,
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
            let command = self.parse().await;
            self.instructions.push_back(command)
        }
    }

    async fn parse(&mut self) -> Commands {
        let readline = self.rl.readline(&format!("{} ", ">>".cyan().bold()));
        match readline {
            Ok(line) => {
                // self.rl.add_history_entry(line.as_str())?;

                let line = line.trim().to_string();
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                let cmd = parts[0];
                let arg = parts.get(1).map(|s| s.trim());

                match cmd {
                    ":list" | ":ls" => Commands::List,
                    ":read" | ":cat" => {
                        let title = arg.unwrap_or("").to_string();
                        match title.is_empty() {
                            true => {
                                println!("{}", "Usage: read <title>".red());
                                Commands::None
                            }
                            false => Commands::Display { title },
                        }
                    }
                    ":create" | ":new" | ":c" => {
                        let title = arg.unwrap_or("").to_string();
                        if title.is_empty() {
                            println!("{}", format!("Usage: {cmd} <title>").red());
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
                    ":help" | ":h" | ":?" => {
                        println!("{}", "Commands:".bold());
                        println!(
                            "  {}  {}  {}",
                            ":list".cyan(),
                            "(:ls)".dimmed(),
                            "List all notes".dimmed()
                        );
                        println!(
                            "  {}  {}  {}",
                            ":read <title>".cyan(),
                            "(:cat)".dimmed(),
                            "Read a note".dimmed()
                        );
                        println!(
                            "  {}  {}  {}",
                            ":create <title>".cyan(),
                            "(:new)".dimmed(),
                            "Create a note".dimmed()
                        );
                        println!(
                            "  {}  {}          {}",
                            ":help".cyan(),
                            "(:h/:?)".dimmed(),
                            "Show this help".dimmed()
                        );
                        println!(
                            "  {}  {}          {}",
                            ":quit".cyan(),
                            "(:q)".dimmed(),
                            "Exit".dimmed()
                        );
                        Commands::None
                    }
                    ":quit" | ":q" | ":exit" => Commands::Exit { verbose: true },
                    _ => Commands::Ask { prompt: line },
                }
            }
            Err(ReadlineError::Interrupted) => Commands::None,
            Err(ReadlineError::Eof) => Commands::Exit { verbose: true },
            Err(err) => {
                println!("Error: {:?}", err);
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
