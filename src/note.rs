use std::path::PathBuf;

use tokio::fs;

pub struct Note {
    title: String,
    content: String,
    modified: bool,
}

impl Note {
    pub async fn read(path: &PathBuf) -> Self {
        let title = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let content = match fs::read_to_string(&path).await {
            Ok(content) => content,
            Err(e) => {
                println!("{e}");
                String::new()
            }
        };
        Self {
            title,
            content,
            modified: false,
        }
    }
}
