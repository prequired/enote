use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::storage::Database;
use crate::features::{Note, NoteManager, CollaborationManager, CollaborativeEdit, User};

mod editor;
mod sidebar;
mod toolbar;
mod settings;
mod graph_view;
mod accessibility;

pub use editor::MarkdownEditor;
pub use sidebar::{LeftSidebar, RightSidebar};
pub use toolbar::Toolbar;
pub use settings::SettingsPanel;
pub use graph_view::GraphView;
pub use accessibility::{AccessibilitySupport, FocusManager};

#[derive(Debug, Clone, PartialEq)]
pub enum AppView {
    Editor,
    GraphView,
    Settings,
}

#[derive(Debug, Clone)]
pub struct EnhanceModalState {
    pub is_open: bool,
    pub clarity_enabled: bool,
    pub structure_enabled: bool,
    pub tags_enabled: bool,
    pub summarize_enabled: bool,
    pub grammar_enabled: bool,
    pub preview_enabled: bool,
    pub original_content: String,
    pub preview_content: String,
    pub is_processing: bool,
    pub show_undo: bool,
    pub custom_prompt: String,
}

impl Default for EnhanceModalState {
    fn default() -> Self {
        Self {
            is_open: false,
            clarity_enabled: true,
            structure_enabled: true,
            tags_enabled: false,
            summarize_enabled: false,
            grammar_enabled: false,
            preview_enabled: false,
            original_content: String::new(),
            preview_content: String::new(),
            is_processing: false,
            show_undo: false,
            custom_prompt: String::new(),
        }
    }
}

pub struct EdisonNoteApp {
    db: Arc<Mutex<Database>>,
    note_manager: NoteManager,
    
    // UI State
    current_view: AppView,
    current_note_id: Option<String>,
    left_sidebar_open: bool,
    right_sidebar_open: bool,
    settings_open: bool,
    
    // UI Components
    editor: MarkdownEditor,
    left_sidebar: LeftSidebar,
    right_sidebar: RightSidebar,
    toolbar: Toolbar,
    settings_panel: SettingsPanel,
    graph_view: GraphView,
    
    // UI State
    search_query: String,
    show_enhance_modal: bool,
    word_count: usize,
    sync_status: String,
    
    // Enhanced AI Modal State
    enhance_modal_state: EnhanceModalState,
    
    // Collaboration State
    collaboration_manager: Option<CollaborationManager>,
    collaborative_users: Vec<User>,
    show_collaboration_panel: bool,
}

impl EdisonNoteApp {
    pub fn new(cc: &eframe::CreationContext<'_>, db: Arc<Mutex<Database>>) -> Self {
        // Load notes from database
        let note_manager = NoteManager::new(db.clone());
        
        Self {
            db: db.clone(),
            note_manager,
            
            current_view: AppView::Editor,
            current_note_id: None,
            left_sidebar_open: true,
            right_sidebar_open: true,
            settings_open: false,
            
            editor: MarkdownEditor::new(),
            left_sidebar: LeftSidebar::new(),
            right_sidebar: RightSidebar::new(),
            toolbar: Toolbar::new(),
            settings_panel: SettingsPanel::new(),
            graph_view: GraphView::new(),
            
            search_query: String::new(),
            show_enhance_modal: false,
            word_count: 0,
            sync_status: "Ready".to_string(),
            
            enhance_modal_state: EnhanceModalState::default(),
            
            collaboration_manager: None,
            collaborative_users: Vec::new(),
            show_collaboration_panel: false,
        }
    }
}

impl eframe::App for EdisonNoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        self.handle_shortcuts(ctx);
        
        // Top menu bar (native Tauri menus would be preferred but this works for demo)
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.show_menu_bar(ui);
        });
        
        // Toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.toolbar.show(ui, &mut self.current_view, &mut self.show_enhance_modal, &mut self.show_collaboration_panel);
        });
        
        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            self.show_status_bar(ui);
        });
        
        // Left sidebar
        if self.left_sidebar_open {
            egui::SidePanel::left("left_sidebar")
                .resizable(true)
                .default_width(250.0)
                .show(ctx, |ui| {
                    self.left_sidebar.show(ui, &mut self.note_manager, &mut self.current_note_id);
                });
        }
        
        // Right sidebar
        if self.right_sidebar_open {
            egui::SidePanel::right("right_sidebar")
                .resizable(true)
                .default_width(250.0)
                .show(ctx, |ui| {
                    if self.settings_open {
                        self.settings_panel.show(ui);
                    } else if self.show_collaboration_panel {
                        self.show_collaboration_panel(ui);
                    } else {
                        self.right_sidebar.show(ui, &self.note_manager, self.current_note_id.as_ref());
                    }
                });
        }
        
        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                AppView::Editor => {
                    if let Some(note_id) = &self.current_note_id {
                        if let Some(note) = self.note_manager.get_note(note_id) {
                            self.editor.show(ui, note);
                            self.word_count = note.content.split_whitespace().count();
                        }
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.heading("Welcome to Edison Note");
                            ui.label("Create a new note or select one from the sidebar to get started.");
                        });
                    }
                }
                AppView::GraphView => {
                    self.graph_view.show(ui, &self.note_manager);
                }
                AppView::Settings => {
                    self.settings_panel.show(ui);
                }
            }
        });
        
        // Show enhance modal if open
        if self.enhance_modal_state.is_open {
            self.show_enhanced_ai_modal(ctx);
        }
        
        // Request repaint for animations
        ctx.request_repaint();
    }
}

impl EdisonNoteApp {
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| i.clone());
        
        // Ctrl+N / Cmd+N - New note
        if input.modifiers.command && input.key_pressed(egui::Key::N) {
            self.note_manager.create_new_note();
        }
        
        // Ctrl+S / Cmd+S - Save note
        if input.modifiers.command && input.key_pressed(egui::Key::S) {
            if let Some(note_id) = &self.current_note_id {
                self.note_manager.save_note(note_id);
            }
        }
        
        // Ctrl+Shift+P / Cmd+Shift+P - Command palette (TODO: implement)
        if input.modifiers.command && input.modifiers.shift && input.key_pressed(egui::Key::P) {
            // TODO: Show command palette
        }
        
        // Ctrl+E / Cmd+E - Toggle enhance modal
        if input.modifiers.command && input.key_pressed(egui::Key::E) {
            self.enhance_modal_state.is_open = !self.enhance_modal_state.is_open;
            if self.enhance_modal_state.is_open {
                if let Some(note_id) = &self.current_note_id {
                    if let Some(note) = self.note_manager.get_note(note_id) {
                        self.enhance_modal_state.original_content = note.content.clone();
                    }
                }
            }
        }
    }
    
    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New Note").clicked() {
                    self.note_manager.create_new_note();
                    ui.close_menu();
                }
                if ui.button("Save").clicked() {
                    if let Some(note_id) = &self.current_note_id {
                        self.note_manager.save_note(note_id);
                    }
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Import").clicked() {
                    ui.close_menu();
                }
                if ui.button("Export").clicked() {
                    ui.close_menu();
                }
            });
            
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    ui.close_menu();
                }
                if ui.button("Redo").clicked() {
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Find").clicked() {
                    ui.close_menu();
                }
                if ui.button("Replace").clicked() {
                    ui.close_menu();
                }
            });
            
            ui.menu_button("View", |ui| {
                if ui.button("Toggle Left Sidebar").clicked() {
                    self.left_sidebar_open = !self.left_sidebar_open;
                    ui.close_menu();
                }
                if ui.button("Toggle Right Sidebar").clicked() {
                    self.right_sidebar_open = !self.right_sidebar_open;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Graph View").clicked() {
                    self.current_view = AppView::GraphView;
                    ui.close_menu();
                }
                if ui.button("Editor View").clicked() {
                    self.current_view = AppView::Editor;
                    ui.close_menu();
                }
            });
            
            ui.menu_button("Tools", |ui| {
                if ui.button("Enhance Note").clicked() {
                    self.enhance_modal_state.is_open = true;
                    if let Some(note_id) = &self.current_note_id {
                        if let Some(note) = self.note_manager.get_note(note_id) {
                            self.enhance_modal_state.original_content = note.content.clone();
                        }
                    }
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Settings").clicked() {
                    self.settings_open = true;
                    self.right_sidebar_open = true;
                    ui.close_menu();
                }
            });
            
            ui.menu_button("Help", |ui| {
                if ui.button("Documentation").clicked() {
                    ui.close_menu();
                }
                if ui.button("About Edison Note").clicked() {
                    ui.close_menu();
                }
            });
        });
    }
    
    fn show_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("Sync: {}", self.sync_status));
            ui.separator();
            ui.label(format!("Words: {}", self.word_count));
            ui.separator();
            ui.label("Ready");
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("⚙").clicked() {
                    self.settings_open = !self.settings_open;
                    self.right_sidebar_open = true;
                }
            });
        });
    }
    
    fn show_enhanced_ai_modal(&mut self, ctx: &egui::Context) {
        egui::Window::new("✨ AI Enhancement")
            .collapsible(false)
            .resizable(true)
            .default_width(600.0)
            .default_height(500.0)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("AI Enhancement");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("✕").clicked() {
                            self.enhance_modal_state.is_open = false;
                        }
                    });
                });
                ui.separator();
                
                // Enhancement options
                ui.horizontal(|ui| {
                    ui.label("Enhancement Types:");
                    ui.checkbox(&mut self.enhance_modal_state.clarity_enabled, "Clarity");
                    ui.checkbox(&mut self.enhance_modal_state.structure_enabled, "Structure");
                    ui.checkbox(&mut self.enhance_modal_state.tags_enabled, "Tags");
                });
                
                ui.horizontal(|ui| {
                    ui.add_space(120.0); // Align with label above
                    ui.checkbox(&mut self.enhance_modal_state.summarize_enabled, "Summarize");
                    ui.checkbox(&mut self.enhance_modal_state.grammar_enabled, "Grammar");
                });
                
                ui.separator();
                
                // Custom prompt
                ui.horizontal(|ui| {
                    ui.label("Custom Prompt:");
                    ui.add(egui::TextEdit::singleline(&mut self.enhance_modal_state.custom_prompt)
                        .hint_text("Optional: Add custom instructions..."));
                });
                
                ui.separator();
                
                // Preview toggle
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.enhance_modal_state.preview_enabled, "Real-time Preview");
                    if self.enhance_modal_state.preview_enabled {
                        ui.label("(Preview will update as you change options)");
                    }
                });
                
                // Content preview area
                if self.enhance_modal_state.preview_enabled || !self.enhance_modal_state.preview_content.is_empty() {
                    ui.separator();
                    
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Original:");
                                    ui.add(egui::TextEdit::multiline(&mut self.enhance_modal_state.original_content)
                                        .desired_rows(8)
                                        .interactive(false));
                                });
                                
                                ui.separator();
                                
                                ui.vertical(|ui| {
                                    ui.label("Enhanced:");
                                    if self.enhance_modal_state.is_processing {
                                        ui.spinner();
                                        ui.label("Processing...");
                                    } else {
                                        ui.add(egui::TextEdit::multiline(&mut self.enhance_modal_state.preview_content)
                                            .desired_rows(8)
                                            .interactive(false));
                                    }
                                });
                            });
                        });
                }
                
                ui.separator();
                
                // Action buttons
                ui.horizontal(|ui| {
                    let enhance_button = ui.add_enabled(
                        !self.enhance_modal_state.is_processing,
                        egui::Button::new(if self.enhance_modal_state.is_processing { "Processing..." } else { "✨ Enhance" })
                    );
                    
                    if enhance_button.clicked() {
                        self.enhance_modal_state.is_processing = true;
                        // TODO: Trigger AI enhancement
                        self.enhance_modal_state.preview_content = "Enhanced content will appear here...".to_string();
                        self.enhance_modal_state.is_processing = false;
                    }
                    
                    if ui.button("Apply").clicked() {
                        if let Some(note_id) = &self.current_note_id {
                            if let Some(mut note) = self.note_manager.get_note(note_id) {
                                note.update_content(self.enhance_modal_state.preview_content.clone());
                                self.note_manager.save_note(&note_id);
                                self.enhance_modal_state.show_undo = true;
                            }
                        }
                        self.enhance_modal_state.is_open = false;
                    }
                    
                    if self.enhance_modal_state.show_undo && ui.button("🔄 Undo").clicked() {
                        if let Some(note_id) = &self.current_note_id {
                            if let Some(mut note) = self.note_manager.get_note(note_id) {
                                note.update_content(self.enhance_modal_state.original_content.clone());
                                self.note_manager.save_note(&note_id);
                                self.enhance_modal_state.show_undo = false;
                            }
                        }
                    }
                    
                    if ui.button("Cancel").clicked() {
                        self.enhance_modal_state.is_open = false;
                        self.enhance_modal_state = EnhanceModalState::default();
                    }
                });
            });
    }
    
    // Collaboration methods
    fn start_collaboration(&mut self, note_id: &str) {
        let user_id = format!("user_{}", uuid::Uuid::new_v4().to_string()[..8]);
        let user_name = "Anonymous User".to_string(); // In production, get from user settings
        
        self.collaboration_manager = Some(CollaborationManager::new(
            "ws://localhost:8080".to_string(), // WebSocket server URL
            user_id,
            user_name,
        ));
        
        self.show_collaboration_panel = true;
        
        // In production, this would be async and handled properly
        // For now, we'll just show the collaboration UI
    }
    
    fn stop_collaboration(&mut self) {
        if let Some(ref mut manager) = self.collaboration_manager {
            if let Some(note_id) = &self.current_note_id {
                let _ = manager.leave_session(note_id);
            }
        }
        self.collaboration_manager = None;
        self.collaborative_users.clear();
        self.show_collaboration_panel = false;
    }
    
    fn show_collaboration_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("🤝 Collaboration");
            ui.separator();
            
            if let Some(ref manager) = self.collaboration_manager {
                if let Some(note_id) = &self.current_note_id {
                    if let Some(session) = manager.get_session_info(note_id) {
                        ui.label(format!("Active session for: {}", note_id));
                        ui.label(format!("Participants: {}", session.participants.len()));
                        
                        ui.separator();
                        ui.label("👥 Users:");
                        
                        for user in &session.participants {
                            ui.horizontal(|ui| {
                                // User color indicator
                                let color = egui::Color32::from_hex(&user.color).unwrap_or(egui::Color32::GRAY);
                                ui.colored_label(color, "●");
                                ui.label(&user.name);
                                
                                if let Some(cursor_pos) = user.cursor_position {
                                    ui.label(format!("@{}", cursor_pos));
                                }
                            });
                        }
                        
                        ui.separator();
                        
                        if ui.button("🚪 Leave Session").clicked() {
                            self.stop_collaboration();
                        }
                    }
                }
            } else {
                ui.label("No active collaboration session");
                
                if let Some(_note_id) = &self.current_note_id {
                    if ui.button("🚀 Start Collaborating").clicked() {
                        self.start_collaboration(_note_id);
                    }
                }
            }
        });
    }
}