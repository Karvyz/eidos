use std::path::{Path, PathBuf};

use tokio::fs;

#[derive(Clone)]
pub struct Note {
    title: String,
    content: String,
}

impl Note {
    pub async fn new(
        title: String,
        content: String,
        vault: &Path,
    ) -> Result<(Self, PathBuf), std::io::Error> {
        let path = vault.join(format!("{}.md", title));
        fs::write(&path, &content).await?;
        Ok((Self { title, content }, path))
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
        Self { title, content }
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
