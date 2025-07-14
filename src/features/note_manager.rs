use super::{Note, NoteLink, NoteVersion, Folder};
use crate::storage::Database;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use anyhow::Result;

pub struct NoteManager {
    db: Arc<Mutex<Database>>,
    notes_cache: HashMap<String, Note>,
    folders_cache: HashMap<String, Folder>,
}

impl NoteManager {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        let mut manager = Self {
            db,
            notes_cache: HashMap::new(),
            folders_cache: HashMap::new(),
        };
        
        // Load initial data
        if let Err(e) = manager.refresh_cache() {
            log::error!("Failed to refresh cache: {}", e);
        }
        
        manager
    }
    
    pub fn refresh_cache(&mut self) -> Result<()> {
        if let Ok(db) = self.db.lock() {
            self.notes_cache = db.get_all_notes()?;
            self.folders_cache = db.get_all_folders()?;
        }
        Ok(())
    }
    
    pub fn create_new_note(&mut self) -> String {
        let title = format!("Untitled Note {}", chrono::Utc::now().format("%Y-%m-%d %H:%M"));
        let mut note = Note::new(title);
        
        // Save to database
        if let Ok(db) = self.db.lock() {
            if let Err(e) = db.save_note(&note) {
                log::error!("Failed to save new note: {}", e);
                return note.id;
            }
        }
        
        let note_id = note.id.clone();
        self.notes_cache.insert(note_id.clone(), note);
        note_id
    }
    
    pub fn get_note(&self, note_id: &str) -> Option<&Note> {
        self.notes_cache.get(note_id)
    }
    
    pub fn get_note_mut(&mut self, note_id: &str) -> Option<&mut Note> {
        self.notes_cache.get_mut(note_id)
    }
    
    pub fn save_note(&mut self, note_id: &str) -> Result<()> {
        if let Some(note) = self.notes_cache.get(note_id) {
            if let Ok(db) = self.db.lock() {
                db.save_note(note)?;
                db.create_note_version(note)?;
            }
        }
        Ok(())
    }
    
    pub fn delete_note(&mut self, note_id: &str) -> Result<()> {
        if let Some(note) = self.notes_cache.get_mut(note_id) {
            note.move_to_trash();
            self.save_note(note_id)?;
        }
        Ok(())
    }
    
    pub fn restore_note(&mut self, note_id: &str) -> Result<()> {
        if let Some(note) = self.notes_cache.get_mut(note_id) {
            note.restore_from_trash();
            self.save_note(note_id)?;
        }
        Ok(())
    }
    
    pub fn permanently_delete_note(&mut self, note_id: &str) -> Result<()> {
        if let Ok(db) = self.db.lock() {
            db.delete_note(note_id)?;
        }
        self.notes_cache.remove(note_id);
        Ok(())
    }
    
    pub fn get_all_notes(&self) -> Vec<&Note> {
        self.notes_cache
            .values()
            .filter(|note| !note.is_deleted)
            .collect()
    }
    
    pub fn get_recent_notes(&self, limit: usize) -> Vec<&Note> {
        let mut notes: Vec<&Note> = self.notes_cache
            .values()
            .filter(|note| !note.is_deleted)
            .collect();
        
        notes.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        notes.into_iter().take(limit).collect()
    }
    
    pub fn get_favorite_notes(&self) -> Vec<&Note> {
        self.notes_cache
            .values()
            .filter(|note| !note.is_deleted && note.is_favorite)
            .collect()
    }
    
    pub fn get_trashed_notes(&self) -> Vec<&Note> {
        self.notes_cache
            .values()
            .filter(|note| note.is_deleted)
            .collect()
    }
    
    pub fn get_notes_by_tag(&self, tag: &str) -> Vec<&Note> {
        self.notes_cache
            .values()
            .filter(|note| !note.is_deleted && note.tags.contains(&tag.to_string()))
            .collect()
    }
    
    pub fn get_notes_by_folder(&self, folder_id: &str) -> Vec<&Note> {
        self.notes_cache
            .values()
            .filter(|note| {
                !note.is_deleted && note.folder_id.as_ref() == Some(&folder_id.to_string())
            })
            .collect()
    }
    
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut tags = std::collections::HashSet::new();
        for note in self.notes_cache.values() {
            if !note.is_deleted {
                for tag in &note.tags {
                    tags.insert(tag.clone());
                }
            }
        }
        let mut tags: Vec<String> = tags.into_iter().collect();
        tags.sort();
        tags
    }
    
    pub fn search_notes(&self, query: &str) -> Vec<&Note> {
        let query = query.to_lowercase();
        self.notes_cache
            .values()
            .filter(|note| {
                !note.is_deleted && (
                    note.title.to_lowercase().contains(&query) ||
                    note.content.to_lowercase().contains(&query) ||
                    note.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
                )
            })
            .collect()
    }
    
    pub fn get_backlinks(&self, note_id: &str) -> Vec<&Note> {
        if let Ok(db) = self.db.lock() {
            if let Ok(backlink_ids) = db.get_backlinks(note_id) {
                return backlink_ids
                    .iter()
                    .filter_map(|id| self.notes_cache.get(id))
                    .collect();
            }
        }
        Vec::new()
    }
    
    pub fn get_note_links(&self, note_id: &str) -> Vec<String> {
        if let Ok(db) = self.db.lock() {
            if let Ok(links) = db.get_note_links(note_id) {
                return links;
            }
        }
        Vec::new()
    }
    
    pub fn get_note_versions(&self, note_id: &str) -> Vec<NoteVersion> {
        if let Ok(db) = self.db.lock() {
            if let Ok(versions) = db.get_note_versions(note_id) {
                return versions;
            }
        }
        Vec::new()
    }
    
    pub fn create_folder(&mut self, name: String, parent_id: Option<String>) -> String {
        let folder = Folder::new(name, parent_id);
        let folder_id = folder.id.clone();
        
        if let Ok(db) = self.db.lock() {
            if let Err(e) = db.save_folder(&folder) {
                log::error!("Failed to save folder: {}", e);
                return folder_id;
            }
        }
        
        self.folders_cache.insert(folder_id.clone(), folder);
        folder_id
    }
    
    pub fn get_folders(&self) -> Vec<&Folder> {
        self.folders_cache.values().collect()
    }
    
    pub fn update_note_links(&mut self, note_id: &str) -> Result<()> {
        if let Some(note) = self.notes_cache.get(note_id) {
            let links = self.extract_wiki_links(&note.content);
            
            if let Ok(db) = self.db.lock() {
                db.update_note_links(note_id, &links)?;
            }
        }
        Ok(())
    }
    
    fn extract_wiki_links(&self, content: &str) -> Vec<String> {
        let mut links = Vec::new();
        let mut chars = content.chars().peekable();
        let mut current_pos = 0;
        
        while let Some(ch) = chars.next() {
            if ch == '[' {
                if chars.peek() == Some(&'[') {
                    chars.next(); // consume second '['
                    current_pos += 2;
                    
                    let mut link_text = String::new();
                    let mut found_closing = false;
                    
                    while let Some(ch) = chars.next() {
                        current_pos += 1;
                        if ch == ']' {
                            if chars.peek() == Some(&']') {
                                chars.next(); // consume second ']'
                                current_pos += 1;
                                found_closing = true;
                                break;
                            } else {
                                link_text.push(ch);
                            }
                        } else {
                            link_text.push(ch);
                        }
                    }
                    
                    if found_closing && !link_text.trim().is_empty() {
                        // Try to find note by title
                        if let Some(target_note) = self.find_note_by_title(&link_text.trim()) {
                            links.push(target_note.id.clone());
                        }
                    }
                }
            }
            current_pos += 1;
        }
        
        links
    }
    
    fn find_note_by_title(&self, title: &str) -> Option<&Note> {
        self.notes_cache
            .values()
            .find(|note| !note.is_deleted && note.title.to_lowercase() == title.to_lowercase())
    }
}