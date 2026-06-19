use std::path::PathBuf;

use tokio::fs;

#[derive(Clone)]
pub struct Note {
    title: String,
    content: String,
    modified: bool,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            modified: true,
        }
    }

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

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn display(&self) -> String {
        let separator = "─".repeat(60);
        format!(
            "{}\n📄 {}:\n{}\n{}\n{}",
            separator, self.title, separator, self.content, separator,
        )
    }
}
