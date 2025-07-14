use rusqlite::{Connection, Result, Row};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::anyhow;
use crate::features::{Note, NoteLink, NoteVersion, Folder};

pub struct Database {
    conn: Connection,
    data_dir: PathBuf,
}

impl Database {
    pub async fn new() -> anyhow::Result<Self> {
        let data_dir = Self::get_data_directory()?;
        std::fs::create_dir_all(&data_dir)?;
        
        let db_path = data_dir.join("edison_note.db");
        let conn = Connection::open(&db_path)?;
        
        let mut db = Self { conn, data_dir };
        db.initialize_schema().await?;
        
        Ok(db)
    }
    
    fn get_data_directory() -> anyhow::Result<PathBuf> {
        if let Some(home_dir) = dirs::home_dir() {
            Ok(home_dir.join("EdisonNote"))
        } else {
            Err(anyhow!("Could not find home directory"))
        }
    }
    
    async fn initialize_schema(&mut self) -> anyhow::Result<()> {
        // Create notes table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                tags TEXT NOT NULL, -- JSON array
                created_at TEXT NOT NULL,
                modified_at TEXT NOT NULL,
                folder_id TEXT,
                is_favorite BOOLEAN NOT NULL DEFAULT 0,
                is_deleted BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )?;
        
        // Create note_links table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS note_links (
                id TEXT PRIMARY KEY,
                from_note_id TEXT NOT NULL,
                to_note_id TEXT NOT NULL,
                link_text TEXT NOT NULL,
                position INTEGER NOT NULL,
                FOREIGN KEY(from_note_id) REFERENCES notes(id),
                FOREIGN KEY(to_note_id) REFERENCES notes(id)
            )",
            [],
        )?;
        
        // Create note_versions table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS note_versions (
                id TEXT PRIMARY KEY,
                note_id TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                version_number INTEGER NOT NULL,
                FOREIGN KEY(note_id) REFERENCES notes(id)
            )",
            [],
        )?;
        
        // Create folders table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS folders (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                parent_id TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY(parent_id) REFERENCES folders(id)
            )",
            [],
        )?;
        
        // Create AI suggestions table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS ai_suggestions (
                id TEXT PRIMARY KEY,
                note_id TEXT NOT NULL,
                original_content TEXT NOT NULL,
                suggested_content TEXT NOT NULL,
                suggestion_type TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                applied BOOLEAN NOT NULL DEFAULT 0,
                FOREIGN KEY(note_id) REFERENCES notes(id)
            )",
            [],
        )?;
        
        // Create indexes for better performance
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_notes_modified_at ON notes(modified_at)",
            [],
        )?;
        
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_notes_tags ON notes(tags)",
            [],
        )?;
        
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_note_links_from ON note_links(from_note_id)",
            [],
        )?;
        
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_note_links_to ON note_links(to_note_id)",
            [],
        )?;
        
        Ok(())
    }
    
    pub fn save_note(&self, note: &Note) -> Result<()> {
        let tags_json = serde_json::to_string(&note.tags)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        self.conn.execute(
            "INSERT OR REPLACE INTO notes 
             (id, title, content, tags, created_at, modified_at, folder_id, is_favorite, is_deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                note.id,
                note.title,
                note.content,
                tags_json,
                note.created_at.to_rfc3339(),
                note.modified_at.to_rfc3339(),
                note.folder_id,
                note.is_favorite,
                note.is_deleted
            ],
        )?;
        
        // Also save as markdown file
        self.save_note_as_file(note)?;
        
        Ok(())
    }
    
    fn save_note_as_file(&self, note: &Note) -> Result<()> {
        let file_name = format!("{}.md", sanitize_filename(&note.title));
        let file_path = self.data_dir.join("notes").join(&file_name);
        
        // Create notes directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                    Some(e.to_string())
                ))?;
        }
        
        // Create markdown content with metadata
        let mut content = String::new();
        content.push_str(&format!("---\n"));
        content.push_str(&format!("id: {}\n", note.id));
        content.push_str(&format!("title: {}\n", note.title));
        content.push_str(&format!("created: {}\n", note.created_at.to_rfc3339()));
        content.push_str(&format!("modified: {}\n", note.modified_at.to_rfc3339()));
        if !note.tags.is_empty() {
            content.push_str(&format!("tags: [{}]\n", note.tags.join(", ")));
        }
        content.push_str(&format!("---\n\n"));
        content.push_str(&note.content);
        
        std::fs::write(&file_path, content)
            .map_err(|e| rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some(e.to_string())
            ))?;
        
        Ok(())
    }
    
    pub fn get_note(&self, note_id: &str) -> Result<Option<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, tags, created_at, modified_at, folder_id, is_favorite, is_deleted
             FROM notes WHERE id = ?1"
        )?;
        
        let note = stmt.query_row([note_id], |row| {
            self.row_to_note(row)
        }).optional()?;
        
        Ok(note)
    }
    
    pub fn get_all_notes(&self) -> Result<HashMap<String, Note>> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT id, title, content, tags, created_at, modified_at, folder_id, is_favorite, is_deleted
             FROM notes WHERE is_deleted = 0 ORDER BY modified_at DESC"
        )?;
        
        let notes = stmt.query_map([], |row| {
            self.row_to_note(row)
        })?;
        
        let mut result = HashMap::with_capacity(1000); // Pre-allocate for better performance
        for note in notes {
            let note = note?;
            result.insert(note.id.clone(), note);
        }
        
        Ok(result)
    }
    
    // New optimized method for listing notes with minimal data
    pub fn get_notes_list(&self) -> Result<Vec<(String, String, chrono::DateTime<chrono::Utc>)>> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT id, title, modified_at FROM notes WHERE is_deleted = 0 ORDER BY modified_at DESC LIMIT 100"
        )?;
        
        let notes = stmt.query_map([], |row| {
            let modified_at: String = row.get(2)?;
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                chrono::DateTime::parse_from_rfc3339(&modified_at)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(2, "modified_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&chrono::Utc),
            ))
        })?;
        
        let mut result = Vec::with_capacity(100);
        for note in notes {
            result.push(note?);
        }
        
        Ok(result)
    }
    
    pub fn delete_note(&self, note_id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM notes WHERE id = ?1", [note_id])?;
        self.conn.execute("DELETE FROM note_links WHERE from_note_id = ?1 OR to_note_id = ?1", [note_id])?;
        self.conn.execute("DELETE FROM note_versions WHERE note_id = ?1", [note_id])?;
        self.conn.execute("DELETE FROM ai_suggestions WHERE note_id = ?1", [note_id])?;
        Ok(())
    }
    
    pub fn create_note_version(&self, note: &Note) -> Result<()> {
        let version_id = uuid::Uuid::new_v4().to_string();
        
        // Get the next version number
        let version_number: i32 = self.conn.query_row(
            "SELECT COALESCE(MAX(version_number), 0) + 1 FROM note_versions WHERE note_id = ?1",
            [&note.id],
            |row| row.get(0)
        ).unwrap_or(1);
        
        self.conn.execute(
            "INSERT INTO note_versions (id, note_id, content, timestamp, version_number)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                version_id,
                note.id,
                note.content,
                note.modified_at.to_rfc3339(),
                version_number
            ],
        )?;
        
        Ok(())
    }
    
    pub fn get_note_versions(&self, note_id: &str) -> Result<Vec<NoteVersion>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, note_id, content, timestamp, version_number
             FROM note_versions WHERE note_id = ?1 ORDER BY version_number DESC"
        )?;
        
        let versions = stmt.query_map([note_id], |row| {
            Ok(NoteVersion {
                id: row.get(0)?,
                note_id: row.get(1)?,
                content: row.get(2)?,
                timestamp: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(3, "timestamp".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&chrono::Utc),
                version_number: row.get(4)?,
            })
        })?;
        
        let mut result = Vec::new();
        for version in versions {
            result.push(version?);
        }
        
        Ok(result)
    }
    
    pub fn update_note_links(&self, note_id: &str, target_note_ids: &[String]) -> Result<()> {
        // Delete existing links from this note
        self.conn.execute(
            "DELETE FROM note_links WHERE from_note_id = ?1",
            [note_id]
        )?;
        
        // Insert new links
        for (position, target_id) in target_note_ids.iter().enumerate() {
            let link_id = uuid::Uuid::new_v4().to_string();
            self.conn.execute(
                "INSERT INTO note_links (id, from_note_id, to_note_id, link_text, position)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![
                    link_id,
                    note_id,
                    target_id,
                    "", // We could extract the actual link text here
                    position as i32
                ],
            )?;
        }
        
        Ok(())
    }
    
    pub fn get_note_links(&self, note_id: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT to_note_id FROM note_links WHERE from_note_id = ?1 ORDER BY position"
        )?;
        
        let links = stmt.query_map([note_id], |row| {
            row.get(0)
        })?;
        
        let mut result = Vec::new();
        for link in links {
            result.push(link?);
        }
        
        Ok(result)
    }
    
    pub fn get_backlinks(&self, note_id: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT from_note_id FROM note_links WHERE to_note_id = ?1"
        )?;
        
        let backlinks = stmt.query_map([note_id], |row| {
            row.get(0)
        })?;
        
        let mut result = Vec::new();
        for backlink in backlinks {
            result.push(backlink?);
        }
        
        Ok(result)
    }
    
    pub fn save_folder(&self, folder: &Folder) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO folders (id, name, parent_id, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                folder.id,
                folder.name,
                folder.parent_id,
                folder.created_at.to_rfc3339()
            ],
        )?;
        
        Ok(())
    }
    
    pub fn get_all_folders(&self) -> Result<HashMap<String, Folder>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, parent_id, created_at FROM folders ORDER BY name"
        )?;
        
        let folders = stmt.query_map([], |row| {
            Ok(Folder {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .map_err(|e| rusqlite::Error::InvalidColumnType(3, "created_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&chrono::Utc),
            })
        })?;
        
        let mut result = HashMap::new();
        for folder in folders {
            let folder = folder?;
            result.insert(folder.id.clone(), folder);
        }
        
        Ok(result)
    }
    
    fn row_to_note(&self, row: &Row) -> Result<Note> {
        let tags_json: String = row.get(3)?;
        let tags: Vec<String> = serde_json::from_str(&tags_json)
            .map_err(|e| rusqlite::Error::InvalidColumnType(3, "tags".to_string(), rusqlite::types::Type::Text))?;
        
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tags,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                .map_err(|e| rusqlite::Error::InvalidColumnType(4, "created_at".to_string(), rusqlite::types::Type::Text))?
                .with_timezone(&chrono::Utc),
            modified_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .map_err(|e| rusqlite::Error::InvalidColumnType(5, "modified_at".to_string(), rusqlite::types::Type::Text))?
                .with_timezone(&chrono::Utc),
            folder_id: row.get(6)?,
            is_favorite: row.get(7)?,
            is_deleted: row.get(8)?,
        })
    }
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}