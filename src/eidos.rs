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

    pub async fn len(&self) -> usize {
        self.notes.lock().await.len()
    }

    pub async fn create_note(&self, title: String, content: String) {
        self.notes.lock().await.push(Note::new(title, content));
    }

    pub async fn print_all(&self) {
        let notes = self.notes.lock().await;
        if notes.is_empty() {
            println!("No notes found.");
            return;
        }
        for note in notes.iter() {
            println!("{}", note.display());
        }
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
