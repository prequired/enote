use eframe::egui;

pub struct SettingsPanel {
    active_tab: SettingsTab,
    font_size: f32,
    theme: String,
    sync_enabled: bool,
    ai_enabled: bool,
    ai_model: String,
    ai_sensitivity: AiSensitivity,
    ai_style: AiStyle,
    enhance_clarity: bool,
    enhance_structure: bool,
    enhance_tags: bool,
    enhance_summarization: bool,
    enhance_grammar: bool,
    // Dynamic theming
    custom_accent_color: [f32; 3], // RGB values
    custom_background_color: [f32; 3],
    use_custom_colors: bool,
    custom_ai_prompt: String,
}

#[derive(Debug, Clone, PartialEq)]
enum SettingsTab {
    General,
    Appearance,
    Sync,
    Plugins,
    Enhance,
    Advanced,
    Privacy,
    Shortcuts,
}

#[derive(Debug, Clone, PartialEq)]
enum AiSensitivity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq)]
enum AiStyle {
    Formal,
    Casual,
    Technical,
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {
            active_tab: SettingsTab::General,
            font_size: 16.0,
            theme: "light".to_string(),
            sync_enabled: false,
            ai_enabled: true,
            ai_model: "local".to_string(),
            ai_sensitivity: AiSensitivity::Medium,
            ai_style: AiStyle::Casual,
            enhance_clarity: true,
            enhance_structure: true,
            enhance_tags: false,
            enhance_summarization: false,
            enhance_grammar: false,
            // Dynamic theming defaults
            custom_accent_color: [0.0, 0.8, 0.416], // Community Green #00CC6A
            custom_background_color: [1.0, 1.0, 1.0], // Clean White
            use_custom_colors: false,
            custom_ai_prompt: String::new(),
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Settings");
            ui.separator();
            
            // Tab bar
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut self.active_tab, SettingsTab::General, "General");
                ui.selectable_value(&mut self.active_tab, SettingsTab::Appearance, "Appearance");
                ui.selectable_value(&mut self.active_tab, SettingsTab::Sync, "Sync");
                ui.selectable_value(&mut self.active_tab, SettingsTab::Plugins, "Plugins");
                ui.selectable_value(&mut self.active_tab, SettingsTab::Enhance, "Enhance");
                ui.selectable_value(&mut self.active_tab, SettingsTab::Advanced, "Advanced");
                ui.selectable_value(&mut self.active_tab, SettingsTab::Privacy, "Privacy");
                ui.selectable_value(&mut self.active_tab, SettingsTab::Shortcuts, "Shortcuts");
            });
            
            ui.separator();
            
            // Tab content
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.active_tab {
                    SettingsTab::General => self.show_general_settings(ui),
                    SettingsTab::Appearance => self.show_appearance_settings(ui),
                    SettingsTab::Sync => self.show_sync_settings(ui),
                    SettingsTab::Plugins => self.show_plugin_settings(ui),
                    SettingsTab::Enhance => self.show_enhance_settings(ui),
                    SettingsTab::Advanced => self.show_advanced_settings(ui),
                    SettingsTab::Privacy => self.show_privacy_settings(ui),
                    SettingsTab::Shortcuts => self.show_shortcuts_settings(ui),
                }
            });
        });
    }
    
    fn show_general_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("General Settings");
        
        ui.group(|ui| {
            ui.label("Default note location:");
            ui.horizontal(|ui| {
                ui.label("~/EdisonNote");
                if ui.button("📁 Change").clicked() {
                    // Open folder picker
                }
            });
        });
        
        ui.group(|ui| {
            ui.checkbox(&mut true, "Auto-save notes");
            ui.checkbox(&mut true, "Show word count in status bar");
            ui.checkbox(&mut false, "Start with last opened note");
        });
        
        ui.group(|ui| {
            ui.label("Default new note template:");
            egui::ComboBox::from_label("")
                .selected_text("Blank")
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut "blank", "blank", "Blank");
                    ui.selectable_value(&mut "meeting", "blank", "Meeting Notes");
                    ui.selectable_value(&mut "daily", "blank", "Daily Journal");
                });
        });
    }
    
    fn show_appearance_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Appearance Settings");
        
        ui.group(|ui| {
            ui.label("Theme:");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.theme, "light".to_string(), "Light");
                ui.selectable_value(&mut self.theme, "dark".to_string(), "Dark");
                ui.selectable_value(&mut self.theme, "auto".to_string(), "Auto");
            });
        });
        
        // Dynamic color customization
        ui.group(|ui| {
            ui.label("Custom Colors:");
            ui.checkbox(&mut self.use_custom_colors, "Use custom color scheme");
            
            if self.use_custom_colors {
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("Accent Color:");
                    ui.color_edit_button_rgb(&mut self.custom_accent_color);
                    if ui.small_button("Reset").clicked() {
                        self.custom_accent_color = [0.0, 0.8, 0.416]; // Community Green
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Background:");
                    ui.color_edit_button_rgb(&mut self.custom_background_color);
                    if ui.small_button("Reset").clicked() {
                        self.custom_background_color = [1.0, 1.0, 1.0]; // Clean White
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.button("Apply Theme").clicked() {
                        self.apply_custom_theme(ui.ctx());
                    }
                    if ui.button("Reset to Default").clicked() {
                        self.reset_to_default_theme(ui.ctx());
                    }
                });
                
                // Live preview
                ui.separator();
                ui.label("Preview:");
                let accent_color = egui::Color32::from_rgb(
                    (self.custom_accent_color[0] * 255.0) as u8,
                    (self.custom_accent_color[1] * 255.0) as u8,
                    (self.custom_accent_color[2] * 255.0) as u8,
                );
                let bg_color = egui::Color32::from_rgb(
                    (self.custom_background_color[0] * 255.0) as u8,
                    (self.custom_background_color[1] * 255.0) as u8,
                    (self.custom_background_color[2] * 255.0) as u8,
                );
                
                ui.horizontal(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(100.0, 30.0),
                        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            let button = egui::Button::new("Sample Button").fill(accent_color);
                            ui.add(button);
                        },
                    );
                    ui.allocate_ui_with_layout(
                        egui::vec2(100.0, 30.0),
                        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            ui.painter().rect_filled(
                                ui.available_rect_before_wrap(),
                                egui::Rounding::same(4.0),
                                bg_color,
                            );
                            ui.label("Background");
                        },
                    );
                });
            }
        });
        
        ui.group(|ui| {
            ui.label("Font family:");
            egui::ComboBox::from_label("")
                .selected_text("Inter")
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut "inter", "inter", "Inter");
                    ui.selectable_value(&mut "roboto", "inter", "Roboto");
                    ui.selectable_value(&mut "arial", "inter", "Arial");
                });
            
            ui.add(egui::Slider::new(&mut self.font_size, 12.0..=24.0).text("Font size"));
        });
        
        ui.group(|ui| {
            ui.checkbox(&mut true, "Show line numbers in editor");
            ui.checkbox(&mut true, "Highlight current line");
            ui.checkbox(&mut false, "Enable high contrast mode");
            ui.checkbox(&mut true, "Smooth animations (150ms)");
        });
    }
    
    fn show_sync_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Sync Settings");
        
        ui.group(|ui| {
            ui.checkbox(&mut self.sync_enabled, "Enable cloud sync");
            
            if self.sync_enabled {
                ui.separator();
                ui.label("Sync provider:");
                egui::ComboBox::from_label("")
                    .selected_text("Edison Cloud")
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut "edison", "edison", "Edison Cloud");
                        ui.selectable_value(&mut "google", "edison", "Google Drive");
                        ui.selectable_value(&mut "dropbox", "edison", "Dropbox");
                    });
                
                ui.horizontal(|ui| {
                    if ui.button("🔗 Connect Account").clicked() {
                        // Connect sync account
                    }
                    ui.label("Status: Not connected");
                });
                
                ui.checkbox(&mut true, "Encrypt notes before sync");
                ui.checkbox(&mut false, "Sync images and attachments");
            }
        });
        
        if self.sync_enabled {
            ui.group(|ui| {
                ui.label("Last sync: Never");
                if ui.button("🔄 Sync Now").clicked() {
                    // Force sync
                }
            });
        }
    }
    
    fn show_plugin_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Plugin Settings");
        
        ui.group(|ui| {
            ui.label("Installed Plugins:");
            
            // Example plugins
            ui.horizontal(|ui| {
                ui.checkbox(&mut true, "Calendar Plugin");
                if ui.button("⚙️").clicked() {
                    // Configure plugin
                }
            });
            
            ui.horizontal(|ui| {
                ui.checkbox(&mut false, "LaTeX Plugin");
                if ui.button("⚙️").clicked() {
                    // Configure plugin
                }
            });
            
            ui.horizontal(|ui| {
                ui.checkbox(&mut true, "Table Plugin");
                if ui.button("⚙️").clicked() {
                    // Configure plugin
                }
            });
        });
        
        ui.group(|ui| {
            if ui.button("📦 Browse Plugin Store").clicked() {
                // Open plugin store
            }
            if ui.button("📁 Load Local Plugin").clicked() {
                // Load plugin from file
            }
        });
    }
    
    fn show_enhance_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("AI Enhancement Settings");
        
        ui.group(|ui| {
            ui.checkbox(&mut self.ai_enabled, "Enable AI features");
            
            if self.ai_enabled {
                ui.separator();
                
                ui.label("AI Model:");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.ai_model, "local".to_string(), "Local (Private)");
                    ui.selectable_value(&mut self.ai_model, "cloud".to_string(), "Cloud (Premium)");
                });
                
                ui.label("Sensitivity:");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.ai_sensitivity, AiSensitivity::Low, "Low");
                    ui.selectable_value(&mut self.ai_sensitivity, AiSensitivity::Medium, "Medium");
                    ui.selectable_value(&mut self.ai_sensitivity, AiSensitivity::High, "High");
                });
                
                ui.label("Writing Style:");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.ai_style, AiStyle::Formal, "Formal");
                    ui.selectable_value(&mut self.ai_style, AiStyle::Casual, "Casual");
                    ui.selectable_value(&mut self.ai_style, AiStyle::Technical, "Technical");
                });
            }
        });
        
        if self.ai_enabled {
            ui.group(|ui| {
                ui.label("Enhancement Types:");
                ui.checkbox(&mut self.enhance_clarity, "Improve Clarity");
                ui.checkbox(&mut self.enhance_structure, "Add Structure");
                ui.checkbox(&mut self.enhance_tags, "Suggest Tags");
                ui.checkbox(&mut self.enhance_summarization, "Create Summaries");
                ui.checkbox(&mut self.enhance_grammar, "Fix Grammar");
            });
            
            ui.group(|ui| {
                ui.label("Custom AI Prompt:");
                ui.add(
                    egui::TextEdit::multiline(&mut self.custom_ai_prompt)
                        .hint_text("Enter custom instructions for AI enhancement (e.g., 'Format as bullet points', 'Use technical language', etc.)")
                        .desired_rows(3)
                );
                
                ui.horizontal(|ui| {
                    if ui.button("Save Prompt").clicked() {
                        // Save custom prompt to settings
                    }
                    if ui.button("Reset").clicked() {
                        self.custom_ai_prompt.clear();
                    }
                    if ui.button("Load Template").clicked() {
                        // Show template options
                    }
                });
                
                ui.separator();
                ui.label("Template Examples:");
                ui.horizontal_wrapped(|ui| {
                    if ui.small_button("Meeting Notes").clicked() {
                        self.custom_ai_prompt = "Format as meeting notes with action items and decisions".to_string();
                    }
                    if ui.small_button("Technical Doc").clicked() {
                        self.custom_ai_prompt = "Use technical language with clear sections and code examples".to_string();
                    }
                    if ui.small_button("Journal").clicked() {
                        self.custom_ai_prompt = "Format as a personal journal entry with reflective tone".to_string();
                    }
                    if ui.small_button("Task List").clicked() {
                        self.custom_ai_prompt = "Convert to actionable task list with priorities".to_string();
                    }
                });
            });
        }
    }
    
    fn show_advanced_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Advanced Settings");
        
        ui.group(|ui| {
            ui.label("Performance:");
            ui.checkbox(&mut true, "Enable hardware acceleration");
            ui.checkbox(&mut false, "Preload notes for faster access");
            
            ui.add(egui::Slider::new(&mut 1000, 100..=5000).text("Note cache size"));
        });
        
        ui.group(|ui| {
            ui.label("Developer Options:");
            ui.checkbox(&mut false, "Enable debug logging");
            ui.checkbox(&mut false, "Show performance metrics");
            
            if ui.button("🔧 Open Developer Console").clicked() {
                // Open developer tools
            }
        });
    }
    
    fn show_privacy_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Privacy Settings");
        
        ui.group(|ui| {
            ui.label("Data Collection:");
            ui.checkbox(&mut false, "Send anonymous usage statistics");
            ui.checkbox(&mut false, "Send crash reports");
            
            ui.separator();
            ui.label("All data is stored locally by default. Edison Note respects your privacy.");
        });
        
        ui.group(|ui| {
            ui.label("Local Data:");
            ui.horizontal(|ui| {
                ui.label("Storage location: ~/EdisonNote");
                if ui.button("📁 Open").clicked() {
                    // Open data folder
                }
            });
            
            if ui.button("🗑️ Clear All Data").clicked() {
                // Clear all local data (with confirmation)
            }
        });
    }
    
    fn show_shortcuts_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Keyboard Shortcuts");
        
        ui.group(|ui| {
            ui.label("File Operations:");
            self.show_shortcut(ui, "New Note", "Ctrl+N");
            self.show_shortcut(ui, "Save Note", "Ctrl+S");
            self.show_shortcut(ui, "Open Note", "Ctrl+O");
        });
        
        ui.group(|ui| {
            ui.label("Edit Operations:");
            self.show_shortcut(ui, "Find", "Ctrl+F");
            self.show_shortcut(ui, "Replace", "Ctrl+H");
            self.show_shortcut(ui, "Command Palette", "Ctrl+Shift+P");
        });
        
        ui.group(|ui| {
            ui.label("View Operations:");
            self.show_shortcut(ui, "Toggle Left Sidebar", "Ctrl+\\");
            self.show_shortcut(ui, "Toggle Right Sidebar", "Ctrl+Shift+\\");
            self.show_shortcut(ui, "Graph View", "Ctrl+G");
        });
        
        ui.group(|ui| {
            ui.label("AI Operations:");
            self.show_shortcut(ui, "Enhance Note", "Ctrl+E");
        });
    }
    
    fn show_shortcut(&self, ui: &mut egui::Ui, action: &str, shortcut: &str) {
        ui.horizontal(|ui| {
            ui.label(action);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(shortcut);
                if ui.small_button("✏️").clicked() {
                    // Edit shortcut
                }
            });
        });
    }
    
    // Theme application methods
    fn apply_custom_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        // Apply custom colors
        let accent_color = egui::Color32::from_rgb(
            (self.custom_accent_color[0] * 255.0) as u8,
            (self.custom_accent_color[1] * 255.0) as u8,
            (self.custom_accent_color[2] * 255.0) as u8,
        );
        let bg_color = egui::Color32::from_rgb(
            (self.custom_background_color[0] * 255.0) as u8,
            (self.custom_background_color[1] * 255.0) as u8,
            (self.custom_background_color[2] * 255.0) as u8,
        );
        
        // Update button colors
        style.visuals.widgets.inactive.bg_fill = accent_color;
        style.visuals.widgets.hovered.bg_fill = accent_color.gamma_multiply(1.2);
        style.visuals.widgets.active.bg_fill = accent_color.gamma_multiply(0.8);
        
        // Update panel colors
        style.visuals.panel_fill = bg_color;
        style.visuals.window_fill = bg_color;
        
        // Update selection colors
        style.visuals.selection.bg_fill = accent_color.gamma_multiply(0.3);
        style.visuals.selection.stroke.color = accent_color;
        
        ctx.set_style(style);
    }
    
    fn reset_to_default_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        // Reset to Edison Note defaults
        let open_blue = egui::Color32::from_rgb(0, 120, 212);    // #0078D4
        let community_green = egui::Color32::from_rgb(0, 204, 106); // #00CC6A
        let clean_white = egui::Color32::from_rgb(255, 255, 255);   // #FFFFFF
        
        style.visuals.widgets.inactive.bg_fill = open_blue;
        style.visuals.widgets.hovered.bg_fill = community_green;
        style.visuals.widgets.active.bg_fill = community_green;
        style.visuals.panel_fill = clean_white;
        style.visuals.window_fill = clean_white;
        
        ctx.set_style(style);
    }
    
    // Accessor methods for other components
    pub fn get_custom_ai_prompt(&self) -> &str {
        &self.custom_ai_prompt
    }
    
    pub fn get_custom_colors_enabled(&self) -> bool {
        self.use_custom_colors
    }
    
    pub fn get_accent_color(&self) -> [f32; 3] {
        self.custom_accent_color
    }
    
    pub fn get_background_color(&self) -> [f32; 3] {
        self.custom_background_color
    }
}