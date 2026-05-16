use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;

use crate::errors::{AppError, Result};
use crate::models::Bookmark;

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(path: PathBuf) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&path)?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS bookmarks (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                url        TEXT NOT NULL,
                title      TEXT,
                tags       TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS bookmarks_fts USING fts5(
                url,
                title,
                tags,
                content='bookmarks',
                content_rowid='id'
            );

            -- Triggers to keep FTS in sync
            CREATE TRIGGER IF NOT EXISTS bookmarks_ai AFTER INSERT ON bookmarks BEGIN
                INSERT INTO bookmarks_fts(rowid, url, title, tags)
                VALUES (new.id, new.url, new.title, new.tags);
            END;

            CREATE TRIGGER IF NOT EXISTS bookmarks_ad AFTER DELETE ON bookmarks BEGIN
                INSERT INTO bookmarks_fts(bookmarks_fts, rowid, url, title, tags)
                VALUES ('delete', old.id, old.url, old.title, old.tags);
            END;

            CREATE TRIGGER IF NOT EXISTS bookmarks_au AFTER UPDATE ON bookmarks BEGIN
                INSERT INTO bookmarks_fts(bookmarks_fts, rowid, url, title, tags)
                VALUES ('delete', old.id, old.url, old.title, old.tags);
                INSERT INTO bookmarks_fts(rowid, url, title, tags)
                VALUES (new.id, new.url, new.title, new.tags);
            END;
            ",
        )?;
        Ok(())
    }

    pub fn add(&self, url: &str, title: Option<&str>, tags: Vec<String>) -> Result<Bookmark> {
        let now = chrono::Local::now().to_rfc3339();
        let tags_str = tags.join(",");

        self.conn.execute(
            "INSERT INTO bookmarks (url, title, tags, created_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![url, title, tags_str, now],
        )?;

        let id = self.conn.last_insert_rowid();

        Ok(Bookmark {
            id,
            url: url.to_string(),
            title: title.map(|s| s.to_string()),
            tags,
            created_at: now,
        })
    }

    pub fn list(&self) -> Result<Vec<Bookmark>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, url, title, tags, created_at FROM bookmarks ORDER BY id")?;

        let rows = stmt.query_map([], |row| {
            Ok(Bookmark {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                tags: parse_tags(&row.get::<_, String>(3)?),
                created_at: row.get(4)?,
            })
        })?;

        let mut bookmarks = Vec::new();
        for row in rows {
            bookmarks.push(row?);
        }
        Ok(bookmarks)
    }

    pub fn search_all(&self, query: &str) -> Result<Vec<Bookmark>> {
        let fts_query = format!("{}*", query);
        let mut stmt = self.conn.prepare(
            "SELECT b.id, b.url, b.title, b.tags, b.created_at
             FROM bookmarks b
             JOIN bookmarks_fts f ON b.id = f.rowid
             WHERE bookmarks_fts MATCH ?1
             ORDER BY rank",
        )?;

        let rows = stmt.query_map([&fts_query], |row| {
            Ok(Bookmark {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                tags: parse_tags(&row.get::<_, String>(3)?),
                created_at: row.get(4)?,
            })
        })?;

        let mut bookmarks = Vec::new();
        for row in rows {
            bookmarks.push(row?);
        }
        Ok(bookmarks)
    }

    pub fn search_by_tag(&self, tag: &str) -> Result<Vec<Bookmark>> {
        let fts_query = format!("tags:{tag}*");
        self.fts_search(&fts_query)
    }

    pub fn search_by_url(&self, url: &str) -> Result<Vec<Bookmark>> {
        let fts_query = format!("url:{url}*");
        self.fts_search(&fts_query)
    }

    pub fn search_by_title(&self, title: &str) -> Result<Vec<Bookmark>> {
        let fts_query = format!("title:{title}*");
        self.fts_search(&fts_query)
    }

    fn fts_search(&self, fts_query: &str) -> Result<Vec<Bookmark>> {
        let mut stmt = self.conn.prepare(
            "SELECT b.id, b.url, b.title, b.tags, b.created_at
             FROM bookmarks b
             JOIN bookmarks_fts f ON b.id = f.rowid
             WHERE bookmarks_fts MATCH ?1
             ORDER BY rank",
        )?;

        let rows = stmt.query_map([fts_query], |row| {
            Ok(Bookmark {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                tags: parse_tags(&row.get::<_, String>(3)?),
                created_at: row.get(4)?,
            })
        })?;

        let mut bookmarks = Vec::new();
        for row in rows {
            bookmarks.push(row?);
        }
        Ok(bookmarks)
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        let affected = self
            .conn
            .execute("DELETE FROM bookmarks WHERE id = ?1", [id])?;

        if affected == 0 {
            return Err(AppError::NotFound(id));
        }
        Ok(())
    }
}

fn parse_tags(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
