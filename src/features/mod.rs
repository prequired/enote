use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::storage::Database;

pub mod note_manager;
pub mod linking;
pub mod search;
pub mod collaboration;

pub use note_manager::NoteManager;
pub use linking::LinkProcessor;
pub use search::SearchEngine;
pub use collaboration::{CollaborationManager, CollaborativeEdit, CollaborationSession, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub folder_id: Option<String>,
    pub is_favorite: bool,
    pub is_deleted: bool,
}

impl Note {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content: String::new(),
            tags: Vec::new(),
            created_at: now,
            modified_at: now,
            folder_id: None,
            is_favorite: false,
            is_deleted: false,
        }
    }
    
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.modified_at = Utc::now();
    }
    
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.modified_at = Utc::now();
        }
    }
    
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            self.modified_at = Utc::now();
        }
    }
    
    pub fn toggle_favorite(&mut self) {
        self.is_favorite = !self.is_favorite;
        self.modified_at = Utc::now();
    }
    
    pub fn move_to_trash(&mut self) {
        self.is_deleted = true;
        self.modified_at = Utc::now();
    }
    
    pub fn restore_from_trash(&mut self) {
        self.is_deleted = false;
        self.modified_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteLink {
    pub from_note_id: String,
    pub to_note_id: String,
    pub link_text: String,
    pub position: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteVersion {
    pub id: String,
    pub note_id: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub version_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Folder {
    pub fn new(name: String, parent_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            parent_id,
            created_at: Utc::now(),
        }
    }
}