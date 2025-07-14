use eframe::egui;
use std::sync::{Arc, Mutex};
use anyhow::Result;

mod ui;
mod features;
mod ai;
mod storage;
mod utils;

use ui::EdisonNoteApp;
use storage::Database;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct AppConfig {
    font_size: f32,
    theme: String,
    sync_enabled: bool,
    ai_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            theme: "light".to_string(),
            sync_enabled: false,
            ai_enabled: true,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    // Initialize database
    let db = Arc::new(Mutex::new(Database::new().await?));
    
    // Set up native options for egui
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        min_window_size: Some(egui::vec2(800.0, 600.0)),
        icon_data: load_icon(),
        ..Default::default()
    };

    // Run the egui app
    eframe::run_native(
        "Edison Note",
        options,
        Box::new(|cc| {
            // Customize egui style for Edison Note brand
            setup_custom_style(&cc.egui_ctx);
            Box::new(EdisonNoteApp::new(cc, db))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run app: {}", e))?;

    Ok(())
}

fn setup_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Edison Note color scheme
    let open_blue = egui::Color32::from_rgb(0, 120, 212);    // #0078D4
    let community_green = egui::Color32::from_rgb(0, 204, 106); // #00CC6A
    let clean_white = egui::Color32::from_rgb(255, 255, 255);   // #FFFFFF
    let slate_gray = egui::Color32::from_rgb(74, 74, 74);       // #4A4A4A
    
    // Button styling
    style.visuals.widgets.inactive.bg_fill = open_blue;
    style.visuals.widgets.inactive.fg_stroke.color = clean_white;
    style.visuals.widgets.hovered.bg_fill = community_green;
    style.visuals.widgets.active.bg_fill = community_green;
    
    // Panel styling
    style.visuals.panel_fill = clean_white;
    style.visuals.window_fill = clean_white;
    
    // Text styling
    style.visuals.text_color = slate_gray;
    
    // Spacing (8px grid system)
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    style.spacing.menu_margin = egui::vec2(8.0, 8.0);
    
    // Rounded corners
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);
    
    // Enhanced animations with 150ms easing
    style.animation_time = 0.15; // 150ms for smoother animations
    style.explanation_tooltip_delay = 0.5;
    
    ctx.set_style(style);
}

fn load_icon() -> Option<eframe::IconData> {
    // Create a simple icon data - in production this would load from icons/
    let icon_bytes = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory(icon_bytes).ok()?;
    let image = image.to_rgba8();
    let (width, height) = image.dimensions();
    
    Some(eframe::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}