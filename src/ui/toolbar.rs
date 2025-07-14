use eframe::egui;
use crate::ui::AppView;

pub struct Toolbar {
    last_enhance_time: f64,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            last_enhance_time: 0.0,
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, current_view: &mut AppView, show_enhance_modal: &mut bool, show_collaboration_panel: &mut bool) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            
            // Left side - main actions
            if ui.button("📝 New").clicked() {
                // Create new note
            }
            
            if ui.button("💾 Save").clicked() {
                // Save current note
            }
            
            ui.separator();
            
            // View toggles
            ui.selectable_value(current_view, AppView::Editor, "📝 Editor");
            ui.selectable_value(current_view, AppView::GraphView, "🕸️ Graph");
            
            ui.separator();
            
            // Search
            if ui.button("🔍 Search").clicked() {
                // Open search
            }
            
            // Enhance button with animation
            let enhance_text = if self.should_pulse() {
                "✨ Enhance"
            } else {
                "✨ Enhance"
            };
            
            if ui.button(enhance_text).clicked() {
                *show_enhance_modal = true;
                self.last_enhance_time = ui.ctx().input(|i| i.time);
            }
            
            // Collaboration button
            let collab_text = if *show_collaboration_panel { "🤝 Leave" } else { "🤝 Collaborate" };
            if ui.button(collab_text).clicked() {
                *show_collaboration_panel = !*show_collaboration_panel;
            }
            
            ui.separator();
            
            // Right side actions
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🌙").clicked() {
                    // Toggle theme
                }
                
                if ui.button("☁️").clicked() {
                    // Sync
                }
                
                if ui.button("⚙️").clicked() {
                    // Settings
                    *current_view = AppView::Settings;
                }
                
                ui.separator();
                
                // Import/Export
                if ui.button("📤").clicked() {
                    // Export
                }
                
                if ui.button("📥").clicked() {
                    // Import
                }
            });
        });
    }
    
    fn should_pulse(&self) -> bool {
        // Simple pulse animation - would be more sophisticated in production
        false
    }
}