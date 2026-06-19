use crate::eidos::Eidos;

mod cli;
mod eidos;
mod llm;
mod note;

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
        eidos.len(),
        dir_path.display(),
        duration
    );

    println!("\nDone reading all .md files.");
    Ok(())
}
