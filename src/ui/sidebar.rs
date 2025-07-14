use eframe::egui;
use crate::features::{Note, NoteManager};

pub struct LeftSidebar {
    search_query: String,
    selected_folder: Option<String>,
    selected_tag: Option<String>,
}

impl LeftSidebar {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            selected_folder: None,
            selected_tag: None,
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, note_manager: &mut NoteManager, current_note_id: &mut Option<String>) {
        ui.vertical(|ui| {
            // Search bar
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("🔍 Search notes..."));
                if ui.button("⚙").clicked() {
                    // Search settings
                }
            });
            
            ui.separator();
            
            // Quick actions
            ui.horizontal(|ui| {
                if ui.button("📝 New Note").clicked() {
                    let new_note_id = note_manager.create_new_note();
                    *current_note_id = Some(new_note_id);
                }
                if ui.button("📁 New Folder").clicked() {
                    // Create new folder
                }
            });
            
            ui.separator();
            
            // Navigation sections
            egui::CollapsingHeader::new("📁 Folders")
                .default_open(true)
                .show(ui, |ui| {
                    self.show_folders(ui, note_manager);
                });
            
            egui::CollapsingHeader::new("📝 Recent Notes")
                .default_open(true)
                .show(ui, |ui| {
                    self.show_notes_list(ui, note_manager, current_note_id);
                });
            
            egui::CollapsingHeader::new("🏷️ Tags")
                .default_open(false)
                .show(ui, |ui| {
                    self.show_tags(ui, note_manager);
                });
            
            egui::CollapsingHeader::new("⭐ Favorites")
                .default_open(false)
                .show(ui, |ui| {
                    self.show_favorites(ui, note_manager, current_note_id);
                });
            
            egui::CollapsingHeader::new("🗑️ Trash")
                .default_open(false)
                .show(ui, |ui| {
                    self.show_trash(ui, note_manager);
                });
        });
    }
    
    fn show_folders(&mut self, ui: &mut egui::Ui, _note_manager: &NoteManager) {
        ui.selectable_label(self.selected_folder.as_deref() == Some("Work"), "📁 Work");
        ui.selectable_label(self.selected_folder.as_deref() == Some("Personal"), "📁 Personal");
        ui.selectable_label(self.selected_folder.as_deref() == Some("Projects"), "📁 Projects");
    }
    
    fn show_notes_list(&mut self, ui: &mut egui::Ui, note_manager: &NoteManager, current_note_id: &mut Option<String>) {
        let notes = note_manager.get_recent_notes(10);
        
        for note in notes {
            let is_selected = current_note_id.as_ref() == Some(&note.id);
            
            if ui.selectable_label(is_selected, &note.title).clicked() {
                *current_note_id = Some(note.id.clone());
            }
            
            // Context menu
            ui.add_space(2.0);
        }
        
        if notes.is_empty() {
            ui.weak("No notes yet. Create your first note!");
        }
    }
    
    fn show_tags(&mut self, ui: &mut egui::Ui, note_manager: &NoteManager) {
        let tags = note_manager.get_all_tags();
        
        for tag in tags {
            let is_selected = self.selected_tag.as_ref() == Some(&tag);
            
            if ui.selectable_label(is_selected, format!("#{}", tag)).clicked() {
                self.selected_tag = Some(tag);
            }
        }
    }
    
    fn show_favorites(&mut self, ui: &mut egui::Ui, note_manager: &NoteManager, current_note_id: &mut Option<String>) {
        let favorites = note_manager.get_favorite_notes();
        
        for note in favorites {
            let is_selected = current_note_id.as_ref() == Some(&note.id);
            
            if ui.selectable_label(is_selected, format!("⭐ {}", note.title)).clicked() {
                *current_note_id = Some(note.id.clone());
            }
        }
        
        if favorites.is_empty() {
            ui.weak("No favorites yet.");
        }
    }
    
    fn show_trash(&mut self, ui: &mut egui::Ui, note_manager: &NoteManager) {
        let trashed_notes = note_manager.get_trashed_notes();
        
        for note in trashed_notes {
            ui.horizontal(|ui| {
                ui.label(&note.title);
                if ui.small_button("🔄").clicked() {
                    note_manager.restore_note(&note.id);
                }
                if ui.small_button("🗑️").clicked() {
                    note_manager.permanently_delete_note(&note.id);
                }
            });
        }
        
        if trashed_notes.is_empty() {
            ui.weak("Trash is empty.");
        }
    }
}

pub struct RightSidebar {
    active_tab: RightSidebarTab,
}

#[derive(Debug, Clone, PartialEq)]
enum RightSidebarTab {
    Metadata,
    Backlinks,
    Outline,
    History,
}

impl RightSidebar {
    pub fn new() -> Self {
        Self {
            active_tab: RightSidebarTab::Metadata,
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, note_manager: &NoteManager, current_note_id: Option<&String>) {
        ui.vertical(|ui| {
            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, RightSidebarTab::Metadata, "📊");
                ui.selectable_value(&mut self.active_tab, RightSidebarTab::Backlinks, "🔗");
                ui.selectable_value(&mut self.active_tab, RightSidebarTab::Outline, "📋");
                ui.selectable_value(&mut self.active_tab, RightSidebarTab::History, "📜");
            });
            
            ui.separator();
            
            // Tab content
            match self.active_tab {
                RightSidebarTab::Metadata => self.show_metadata(ui, note_manager, current_note_id),
                RightSidebarTab::Backlinks => self.show_backlinks(ui, note_manager, current_note_id),
                RightSidebarTab::Outline => self.show_outline(ui, note_manager, current_note_id),
                RightSidebarTab::History => self.show_history(ui, note_manager, current_note_id),
            }
        });
    }
    
    fn show_metadata(&self, ui: &mut egui::Ui, note_manager: &NoteManager, current_note_id: Option<&String>) {
        if let Some(note_id) = current_note_id {
            if let Some(note) = note_manager.get_note(note_id) {
                ui.heading("Metadata");
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("Created:");
                    ui.label(note.created_at.format("%Y-%m-%d %H:%M").to_string());
                });
                
                ui.horizontal(|ui| {
                    ui.label("Modified:");
                    ui.label(note.modified_at.format("%Y-%m-%d %H:%M").to_string());
                });
                
                ui.horizontal(|ui| {
                    ui.label("Words:");
                    ui.label(note.content.split_whitespace().count().to_string());
                });
                
                ui.horizontal(|ui| {
                    ui.label("Characters:");
                    ui.label(note.content.chars().count().to_string());
                });
                
                ui.separator();
                
                ui.label("Tags:");
                for tag in &note.tags {
                    ui.horizontal(|ui| {
                        ui.label(format!("#{}", tag));
                        if ui.small_button("×").clicked() {
                            // Remove tag
                        }
                    });
                }
                
                ui.horizontal(|ui| {
                    let mut new_tag = String::new();
                    ui.add(egui::TextEdit::singleline(&mut new_tag).hint_text("Add tag..."));
                    if ui.button("+").clicked() && !new_tag.is_empty() {
                        // Add tag
                    }
                });
            }
        } else {
            ui.weak("No note selected");
        }
    }
    
    fn show_backlinks(&self, ui: &mut egui::Ui, note_manager: &NoteManager, current_note_id: Option<&String>) {
        ui.heading("Backlinks");
        ui.separator();
        
        if let Some(note_id) = current_note_id {
            let backlinks = note_manager.get_backlinks(note_id);
            
            for backlink in backlinks {
                if ui.link(&backlink.title).clicked() {
                    // Navigate to backlinked note
                }
            }
            
            if backlinks.is_empty() {
                ui.weak("No backlinks found");
            }
        } else {
            ui.weak("No note selected");
        }
    }
    
    fn show_outline(&self, ui: &mut egui::Ui, note_manager: &NoteManager, current_note_id: Option<&String>) {
        ui.heading("Outline");
        ui.separator();
        
        if let Some(note_id) = current_note_id {
            if let Some(note) = note_manager.get_note(note_id) {
                // Extract headings from markdown content
                for line in note.content.lines() {
                    if line.starts_with("# ") {
                        ui.link(&line[2..]);
                    } else if line.starts_with("## ") {
                        ui.horizontal(|ui| {
                            ui.add_space(16.0);
                            ui.link(&line[3..]);
                        });
                    } else if line.starts_with("### ") {
                        ui.horizontal(|ui| {
                            ui.add_space(32.0);
                            ui.link(&line[4..]);
                        });
                    }
                }
            }
        } else {
            ui.weak("No note selected");
        }
    }
    
    fn show_history(&self, ui: &mut egui::Ui, note_manager: &NoteManager, current_note_id: Option<&String>) {
        ui.heading("Version History");
        ui.separator();
        
        if let Some(note_id) = current_note_id {
            let versions = note_manager.get_note_versions(note_id);
            
            for version in versions {
                ui.horizontal(|ui| {
                    ui.label(version.timestamp.format("%m/%d %H:%M").to_string());
                    if ui.small_button("📖").clicked() {
                        // View this version
                    }
                    if ui.small_button("🔄").clicked() {
                        // Restore this version
                    }
                });
            }
            
            if versions.is_empty() {
                ui.weak("No version history");
            }
        } else {
            ui.weak("No note selected");
        }
    }
}