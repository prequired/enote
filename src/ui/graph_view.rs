use eframe::egui;
use crate::features::NoteManager;
use std::collections::HashMap;

pub struct GraphView {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
    selected_node: Option<String>,
    zoom_level: f32,
    pan_offset: egui::Vec2,
}

#[derive(Clone)]
struct GraphNode {
    id: String,
    title: String,
    position: egui::Pos2,
    size: f32,
    color: egui::Color32,
}

#[derive(Clone)]
struct GraphEdge {
    from: String,
    to: String,
}

impl GraphView {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            selected_node: None,
            zoom_level: 1.0,
            pan_offset: egui::Vec2::ZERO,
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui, note_manager: &NoteManager) {
        ui.vertical(|ui| {
            // Graph controls
            ui.horizontal(|ui| {
                ui.heading("Graph View");
                
                ui.separator();
                
                if ui.button("🔄 Refresh").clicked() {
                    self.update_graph(note_manager);
                }
                
                if ui.button("🎯 Center").clicked() {
                    self.center_graph();
                }
                
                ui.separator();
                
                ui.label("Zoom:");
                ui.add(egui::Slider::new(&mut self.zoom_level, 0.1..=3.0).step_by(0.1));
                
                ui.separator();
                
                if ui.button("🔍 Focus Selected").clicked() {
                    self.focus_selected_node();
                }
            });
            
            ui.separator();
            
            // Graph canvas
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::click_and_drag(),
            );
            
            self.handle_graph_interaction(&response);
            self.draw_graph(&painter, response.rect);
        });
        
        // Update graph data if needed
        if self.nodes.is_empty() {
            self.update_graph(note_manager);
        }
    }
    
    fn update_graph(&mut self, note_manager: &NoteManager) {
        self.nodes.clear();
        self.edges.clear();
        
        let notes = note_manager.get_all_notes();
        let mut node_positions = HashMap::new();
        
        // Create nodes
        for (i, note) in notes.iter().enumerate() {
            let angle = (i as f32) * 2.0 * std::f32::consts::PI / (notes.len() as f32);
            let radius = 200.0;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            
            let node = GraphNode {
                id: note.id.clone(),
                title: note.title.clone(),
                position: egui::pos2(x, y),
                size: 20.0 + (note.content.len() as f32).sqrt() * 0.1,
                color: if note.tags.is_empty() {
                    egui::Color32::from_rgb(0, 120, 212) // Open Blue
                } else {
                    egui::Color32::from_rgb(0, 204, 106) // Community Green
                },
            };
            
            node_positions.insert(note.id.clone(), node.position);
            self.nodes.push(node);
        }
        
        // Create edges based on links
        for note in &notes {
            let links = note_manager.get_note_links(&note.id);
            for link in links {
                if node_positions.contains_key(&link) {
                    self.edges.push(GraphEdge {
                        from: note.id.clone(),
                        to: link,
                    });
                }
            }
        }
    }
    
    fn handle_graph_interaction(&mut self, response: &egui::Response) {
        // Handle panning
        if response.dragged() {
            self.pan_offset += response.drag_delta();
        }
        
        // Handle node selection
        if response.clicked() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                self.selected_node = self.find_node_at_position(pointer_pos);
            }
        }
        
        // Handle zooming with scroll
        if response.hovered() {
            let scroll_delta = response.ctx.input(|i| i.scroll_delta.y);
            if scroll_delta != 0.0 {
                self.zoom_level *= 1.0 + scroll_delta * 0.001;
                self.zoom_level = self.zoom_level.clamp(0.1, 3.0);
            }
        }
    }
    
    fn draw_graph(&self, painter: &egui::Painter, rect: egui::Rect) {
        let center = rect.center();
        
        // Draw edges first (so they appear behind nodes)
        for edge in &self.edges {
            if let (Some(from_node), Some(to_node)) = (
                self.nodes.iter().find(|n| n.id == edge.from),
                self.nodes.iter().find(|n| n.id == edge.to),
            ) {
                let from_pos = center + (from_node.position.to_vec2() + self.pan_offset) * self.zoom_level;
                let to_pos = center + (to_node.position.to_vec2() + self.pan_offset) * self.zoom_level;
                
                painter.line_segment(
                    [from_pos, to_pos],
                    egui::Stroke::new(1.0, egui::Color32::GRAY),
                );
            }
        }
        
        // Draw nodes
        for node in &self.nodes {
            let pos = center + (node.position.to_vec2() + self.pan_offset) * self.zoom_level;
            let radius = node.size * self.zoom_level;
            
            let color = if Some(&node.id) == self.selected_node.as_ref() {
                egui::Color32::from_rgb(255, 255, 0) // Yellow for selected
            } else {
                node.color
            };
            
            // Draw node circle
            painter.circle_filled(pos, radius, color);
            painter.circle_stroke(pos, radius, egui::Stroke::new(2.0, egui::Color32::WHITE));
            
            // Draw node label if zoomed in enough
            if self.zoom_level > 0.5 {
                let text_pos = pos + egui::vec2(0.0, radius + 10.0);
                painter.text(
                    text_pos,
                    egui::Align2::CENTER_TOP,
                    &node.title,
                    egui::FontId::default(),
                    egui::Color32::BLACK,
                );
            }
        }
        
        // Draw selection info
        if let Some(selected_id) = &self.selected_node {
            if let Some(node) = self.nodes.iter().find(|n| n.id == *selected_id) {
                let info_rect = egui::Rect::from_min_size(
                    rect.min + egui::vec2(10.0, 10.0),
                    egui::vec2(200.0, 60.0),
                );
                
                painter.rect_filled(info_rect, 5.0, egui::Color32::from_black_alpha(200));
                painter.rect_stroke(info_rect, 5.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
                
                painter.text(
                    info_rect.min + egui::vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    &node.title,
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
                
                painter.text(
                    info_rect.min + egui::vec2(10.0, 30.0),
                    egui::Align2::LEFT_TOP,
                    "Click to open note",
                    egui::FontId::default(),
                    egui::Color32::LIGHT_GRAY,
                );
            }
        }
    }
    
    fn find_node_at_position(&self, pos: egui::Pos2) -> Option<String> {
        // This is simplified - in practice you'd need to transform coordinates properly
        for node in &self.nodes {
            let distance = (node.position - pos).length();
            if distance < node.size {
                return Some(node.id.clone());
            }
        }
        None
    }
    
    fn center_graph(&mut self) {
        self.pan_offset = egui::Vec2::ZERO;
        self.zoom_level = 1.0;
    }
    
    fn focus_selected_node(&mut self) {
        if let Some(selected_id) = &self.selected_node {
            if let Some(node) = self.nodes.iter().find(|n| n.id == *selected_id) {
                self.pan_offset = -node.position.to_vec2();
                self.zoom_level = 1.5;
            }
        }
    }
}