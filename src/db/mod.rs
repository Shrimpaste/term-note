use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::Connection;
use std::path::PathBuf;

use crate::models::Note;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::db_path()?;
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    fn db_path() -> Result<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "shrimpaste", "term-note") {
            Ok(proj_dirs.data_dir().join("notes.db"))
        } else {
            Ok(PathBuf::from("notes.db"))
        }
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                tags TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_note(&self, note: &Note) -> Result<()> {
        let tags_str = note.tags.join(",");
        self.conn.execute(
            "INSERT OR REPLACE INTO notes (id, title, content, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                &note.id,
                &note.title,
                &note.content,
                &tags_str,
                &note.created_at.to_rfc3339(),
                &note.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_all_notes(&self) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, tags, created_at, updated_at FROM notes ORDER BY updated_at DESC"
        )?;

        let notes = stmt.query_map([], |row| {
            let tags_str: String = row.get(3)?;
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tags: if tags_str.is_empty() {
                    Vec::new()
                } else {
                    tags_str.split(',').map(|s| s.to_string()).collect()
                },
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Local),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Local),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    pub fn delete_note(&self, id: &str) -> Result<bool> {
        let affected = self.conn.execute(
            "DELETE FROM notes WHERE id = ?1",
            [id],
        )?;
        Ok(affected > 0)
    }

    pub fn search(&self, query: &str) -> Result<Vec<Note>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, tags, created_at, updated_at
             FROM notes
             WHERE title LIKE ?1 OR content LIKE ?1 OR tags LIKE ?1
             ORDER BY updated_at DESC"
        )?;

        let notes = stmt.query_map([&pattern], |row| {
            let tags_str: String = row.get(3)?;
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tags: if tags_str.is_empty() {
                    Vec::new()
                } else {
                    tags_str.split(',').map(|s| s.to_string()).collect()
                },
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Local),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Local),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }
}