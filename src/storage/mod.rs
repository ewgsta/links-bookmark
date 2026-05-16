use std::fs;
use std::path::PathBuf;

use crate::errors::{AppError, Result};
use crate::models::Bookmark;

pub struct Store {
    path: PathBuf,
    bookmarks: Vec<Bookmark>,
    next_id: u64,
}

impl Store {
    pub fn load(path: PathBuf) -> Result<Self> {
        let bookmarks: Vec<Bookmark> = if path.exists() {
            let data = fs::read_to_string(&path)?;
            serde_json::from_str(&data)?
        } else {
            Vec::new()
        };

        let next_id = bookmarks.iter().map(|b| b.id).max().unwrap_or(0) + 1;

        Ok(Self {
            path,
            bookmarks,
            next_id,
        })
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(&self.bookmarks)?;
        fs::write(&self.path, data)?;
        Ok(())
    }

    pub fn add(&mut self, url: &str, title: &str, tags: Vec<String>) -> Result<Bookmark> {
        let bookmark = Bookmark {
            id: self.next_id,
            url: url.to_string(),
            title: title.to_string(),
            tags,
            created_at: chrono::Local::now().to_rfc3339(),
        };
        self.next_id += 1;
        self.bookmarks.push(bookmark.clone());
        self.save()?;
        Ok(bookmark)
    }

    pub fn list(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    pub fn search(&self, query: &str) -> Vec<&Bookmark> {
        let q = query.to_lowercase();
        self.bookmarks
            .iter()
            .filter(|bm| {
                bm.title.to_lowercase().contains(&q)
                    || bm.url.to_lowercase().contains(&q)
                    || bm.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn delete(&mut self, id: u64) -> Result<()> {
        let idx = self
            .bookmarks
            .iter()
            .position(|b| b.id == id)
            .ok_or(AppError::NotFound(id))?;
        self.bookmarks.remove(idx);
        self.save()?;
        Ok(())
    }
}
