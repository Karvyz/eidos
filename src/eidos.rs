use std::path::PathBuf;

use tokio::fs;

use crate::note::Note;

pub struct Eidos {
    notes: Vec<Note>,
}

impl Eidos {
    pub async fn read(path: &PathBuf) -> Self {
        let mut notes = vec![];
        let paths = Self::collect_md_files(path).await.unwrap_or(vec![]);
        for path in &paths {
            notes.push(Note::read(path).await);
        }
        Self { notes }
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    async fn collect_md_files(dir: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut md_files = Vec::new();
        let mut dir_handle = fs::read_dir(dir).await?;

        while let Some(entry) = dir_handle.next_entry().await? {
            let path = entry.path();
            if path.extension().map(|ext| ext == "md").unwrap_or(false) {
                md_files.push(path);
            }
        }

        Ok(md_files)
    }
}
