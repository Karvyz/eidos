use std::{path::PathBuf, sync::Arc};

use tokio::{fs, sync::Mutex};

use crate::note::Note;

#[derive(Clone)]
pub struct Eidos {
    notes: Arc<Mutex<Vec<Note>>>,
}

impl Eidos {
    pub async fn read(path: &PathBuf) -> Self {
        let mut notes = vec![];
        let paths = Self::collect_md_files(path).await.unwrap_or(vec![]);
        for path in &paths {
            notes.push(Note::read(path).await);
        }
        Self {
            notes: Arc::new(Mutex::new(notes)),
        }
    }

    /// Get a snapshot of all notes.
    pub async fn notes(&self) -> Vec<Note> {
        self.notes.lock().await.clone()
    }

    /// Find a note by title (case-insensitive prefix match).
    pub async fn find_note(&self, title: &str) -> Option<Note> {
        let lower = title.to_lowercase();
        let notes = self.notes.lock().await;
        notes
            .iter()
            .find(|n| n.title().to_lowercase() == lower)
            .or_else(|| {
                notes
                    .iter()
                    .find(|n| n.title().to_lowercase().starts_with(&lower))
            })
            .cloned()
    }

    pub async fn len(&self) -> usize {
        self.notes.lock().await.len()
    }

    pub async fn create_note(&self, title: String, content: String) {
        self.notes.lock().await.push(Note::new(title, content));
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
