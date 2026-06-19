mod cli;
mod eidos;
mod llm;
mod note;

use crate::{eidos::Eidos, llm::LLM};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = cli::get_directory();

    if let Err(e) = cli::validate_directory(&dir_path) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    let start = std::time::Instant::now();
    let eidos = Eidos::read(&dir_path).await;
    let duration = start.elapsed();

    println!(
        "Found {} .md file(s) in '{}' (read took {:.2?}):",
        eidos.len().await,
        dir_path.display(),
        duration
    );

    let llm = LLM::new();
    llm.create_note(
        "Create a new note with a few lines of info of cats",
        eidos.clone(),
    )
    .await;
    eidos.print_all().await;

    Ok(())
}
