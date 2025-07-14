use eframe::egui;
use crate::features::Note;

pub struct MarkdownEditor {
    content: String,
    is_preview_mode: bool,
    cursor_position: usize,
}

impl MarkdownEditor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            is_preview_mode: false,
            cursor_position: 0,
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, note: &mut Note) {
        ui.horizontal(|ui| {
            ui.heading(&note.title);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.selectable_label(!self.is_preview_mode, "Edit").clicked() {
                    self.is_preview_mode = false;
                }
                if ui.selectable_label(self.is_preview_mode, "Preview").clicked() {
                    self.is_preview_mode = true;
                }
            });
        });
        
        ui.separator();
        
        if !self.is_preview_mode {
            self.show_toolbar(ui);
            ui.separator();
        }
        
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if self.is_preview_mode {
                    self.show_preview(ui, &note.content);
                } else {
                    self.show_editor(ui, &mut note.content);
                }
            });
    }
    
    fn show_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("B").clicked() {
                // Insert bold markdown
            }
            if ui.button("I").clicked() {
                // Insert italic markdown
            }
            if ui.button("H").clicked() {
                // Insert header markdown
            }
            ui.separator();
            if ui.button("🔗").clicked() {
                // Insert link markdown
            }
            if ui.button("📷").clicked() {
                // Insert image markdown
            }
            if ui.button("📝").clicked() {
                // Insert list markdown
            }
            ui.separator();
            if ui.button("```").clicked() {
                // Insert code block markdown
            }
            if ui.button("📊").clicked() {
                // Insert table markdown
            }
        });
    }
    
    fn show_editor(&mut self, ui: &mut egui::Ui, content: &mut String) {
        let text_edit = egui::TextEdit::multiline(content)
            .font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .desired_rows(20);
            
        ui.add(text_edit);
    }
    
    fn show_preview(&self, ui: &mut egui::Ui, content: &str) {
        // Simple markdown preview - in production would use pulldown-cmark
        let lines: Vec<&str> = content.lines().collect();
        
        for line in lines {
            if line.starts_with("# ") {
                ui.heading(&line[2..]);
            } else if line.starts_with("## ") {
                ui.add(egui::Label::new(&line[3..]).text_style(egui::TextStyle::Heading));
            } else if line.starts_with("### ") {
                ui.add(egui::Label::new(&line[4..]).text_style(egui::TextStyle::Heading));
            } else if line.starts_with("- ") || line.starts_with("* ") {
                ui.horizontal(|ui| {
                    ui.label("•");
                    ui.label(&line[2..]);
                });
            } else if line.trim().is_empty() {
                ui.separator();
            } else {
                ui.label(line);
            }
        }
    }
}