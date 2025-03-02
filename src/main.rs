//! Zing - A fast, beautiful, cross-platform text editor written in Rust.
//!
//! Zing is designed to handle very large files with ease while maintaining
//! a sleek, modern interface.

mod buffer;
mod config;
mod file_io;
mod ui;

use eframe::{egui, NativeOptions};
use env_logger::Env;
use std::time::Instant;

/// Main entry point for the application.
fn main() -> Result<(), eframe::Error> {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    log::info!("Starting Zing");
    
    // Set up the native options
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([400.0, 300.0])
            .with_position([100.0, 100.0])
            .with_decorations(true)
            .with_transparent(false)
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    // Run the native application
    eframe::run_native(
        "Zing",
        options,
        Box::new(|cc| Box::new(ZingApp::new(cc))),
    )
}

/// The main application state.
struct ZingApp {
    /// The UI state
    ui_state: ui::ZingApp,
    /// Last update time for calculating delta time
    last_update: Instant,
}

impl ZingApp {
    /// Creates a new application instance.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set up the UI state
        let ui_state = ui::ZingApp::new(&cc.egui_ctx);
        
        Self {
            ui_state,
            last_update: Instant::now(),
        }
    }
    
    /// Handle keyboard shortcuts
    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        let modifiers = ctx.input(|i| i.modifiers);
        let cmd_or_ctrl = if cfg!(target_os = "macos") { modifiers.mac_cmd } else { modifiers.ctrl };
        
        // Save: Cmd+S or Ctrl+S
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::S)) && !modifiers.shift {
            ui::editor::save_file(&mut self.ui_state, false);
        }
        
        // Save As: Cmd+Shift+S or Ctrl+Shift+S
        if cmd_or_ctrl && modifiers.shift && ctx.input(|i| i.key_pressed(egui::Key::S)) {
            ui::editor::save_file(&mut self.ui_state, true);
        }
        
        // Open: Cmd+O or Ctrl+O
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::O)) {
            ui::editor::open_file(&mut self.ui_state);
        }
        
        // New Tab: Cmd+T or Ctrl+T
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::T)) {
            self.ui_state.tabs.new_tab();
        }
        
        // Close Tab: Cmd+W or Ctrl+W
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::W)) {
            // Check if this is the last tab and the user has already been warned
            if self.ui_state.tabs.tabs.len() <= 1 && self.ui_state.last_tab_close_warning {
                // User confirmed closing the last tab, quit the application
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            } else {
                // Normal tab closing behavior
                ui::editor::close_tab(&mut self.ui_state);
            }
        }
        
        // Print: Cmd+P or Ctrl+P
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::P)) {
            ui::editor::print_file(&mut self.ui_state);
        }
        
        // Undo: Cmd+Z or Ctrl+Z
        if cmd_or_ctrl && ctx.input(|i| i.key_pressed(egui::Key::Z)) && !modifiers.shift {
            ui::editor::undo(&mut self.ui_state);
        }
        
        // Redo: Cmd+Shift+Z or Ctrl+Shift+Z or Ctrl+Y
        if (cmd_or_ctrl && modifiers.shift && ctx.input(|i| i.key_pressed(egui::Key::Z))) ||
           (cfg!(not(target_os = "macos")) && modifiers.ctrl && ctx.input(|i| i.key_pressed(egui::Key::Y))) {
            ui::editor::redo(&mut self.ui_state);
        }
    }
    
    /// Sets up the native menu bar for macOS
    fn setup_menu_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Only show the menu bar on macOS
        #[cfg(target_os = "macos")]
        {
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                // Don't set the panel to invisible - we need it to be visible
                // ui.set_visible(false); // Hide the panel but still process the menu
                
                egui::menu::bar(ui, |ui| {
                    // File menu
                    egui::menu::menu_button(ui, "File", |ui| {
                        if ui.button("New Tab ⌘T").clicked() {
                            self.ui_state.tabs.new_tab();
                            ui.close_menu();
                        }
                        if ui.button("Open... ⌘O").clicked() {
                            ui::editor::open_file(&mut self.ui_state);
                            ui.close_menu();
                        }
                        
                        // Change the label based on whether it's the last tab
                        let close_label = if self.ui_state.tabs.tabs.len() <= 1 {
                            "Quit ⌘W"
                        } else {
                            "Close Tab ⌘W"
                        };
                        
                        if ui.button(close_label).clicked() {
                            if self.ui_state.tabs.tabs.len() <= 1 && self.ui_state.last_tab_close_warning {
                                // User confirmed closing the last tab, quit the application
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            } else {
                                // Normal tab closing behavior
                                ui::editor::close_tab(&mut self.ui_state);
                            }
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        if ui.button("Save ⌘S").clicked() {
                            ui::editor::save_file(&mut self.ui_state, false);
                            ui.close_menu();
                        }
                        if ui.button("Save As... Shift+⌘S").clicked() {
                            ui::editor::save_file(&mut self.ui_state, true);
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Print... ⌘P").clicked() {
                            ui::editor::print_file(&mut self.ui_state);
                            ui.close_menu();
                        }
                    });
                    
                    // Edit menu
                    egui::menu::menu_button(ui, "Edit", |ui| {
                        if ui.button("Undo ⌘Z").clicked() {
                            ui::editor::undo(&mut self.ui_state);
                            ui.close_menu();
                        }
                        if ui.button("Redo Shift+⌘Z").clicked() {
                            ui::editor::redo(&mut self.ui_state);
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Cut ⌘X").clicked() {
                            // Implement cut functionality
                            ui.close_menu();
                        }
                        if ui.button("Copy ⌘C").clicked() {
                            // Implement copy functionality
                            ui.close_menu();
                        }
                        if ui.button("Paste ⌘V").clicked() {
                            // Implement paste functionality
                            ui.close_menu();
                        }
                    });
                    
                    // View menu
                    egui::menu::menu_button(ui, "View", |ui| {
                        if ui.button(if self.ui_state.config.show_line_numbers { "Hide Line Numbers" } else { "Show Line Numbers" }).clicked() {
                            self.ui_state.config.toggle_line_numbers();
                            ui.close_menu();
                        }
                        if ui.button(if self.ui_state.config.word_wrap { "Disable Word Wrap" } else { "Enable Word Wrap" }).clicked() {
                            self.ui_state.config.toggle_word_wrap();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(if matches!(self.ui_state.config.theme, crate::config::Theme::Dark) { "Light Theme" } else { "Dark Theme" }).clicked() {
                            self.ui_state.toggle_theme(ctx);
                            ui.close_menu();
                        }
                    });
                    
                    // Format menu
                    egui::menu::menu_button(ui, "Format", |ui| {
                        if ui.button("Decrease Font Size").clicked() {
                            self.ui_state.config.decrease_font_size();
                            self.ui_state.config.apply_to_context(ctx);
                            ui.close_menu();
                        }
                        if ui.button("Increase Font Size").clicked() {
                            self.ui_state.config.increase_font_size();
                            self.ui_state.config.apply_to_context(ctx);
                            ui.close_menu();
                        }
                    });
                });
            });
        }
    }
}

impl eframe::App for ZingApp {
    /// Called each time the UI needs repainting.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;
        
        // Update status message timeout
        self.ui_state.update_status(delta_time);
        
        // Handle keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);
        
        // Set up the menu bar (macOS native menu)
        self.setup_menu_bar(ctx, frame);
        
        // Render the UI
        ui::ui(&mut self.ui_state, ctx);
    }
}

/// Loads the application icon.
fn load_icon() -> egui::IconData {
    // Load icon from the assets directory
    let (icon_rgba, icon_width, icon_height) = {
        let icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png");
        
        // Try to open the icon file
        match image::open(icon_path) {
            Ok(image) => {
                let image = image.into_rgba8();
                let (width, height) = image.dimensions();
                let rgba = image.into_raw();
                (rgba, width, height)
            },
            Err(err) => {
                // If the icon file doesn't exist or can't be opened, create a default icon
                log::warn!("Failed to load icon: {}", err);
                
                // Create a simple 32x32 icon with a gradient
                let width = 32;
                let height = 32;
                let mut rgba = Vec::with_capacity((width * height * 4) as usize);
                
                for y in 0..height {
                    for x in 0..width {
                        // Create a simple blue gradient
                        let r = 0;
                        let g = ((x as f32 / width as f32) * 100.0) as u8 + 50;
                        let b = ((y as f32 / height as f32) * 200.0) as u8 + 55;
                        let a = 255;
                        
                        rgba.push(r);
                        rgba.push(g);
                        rgba.push(b);
                        rgba.push(a);
                    }
                }
                
                (rgba, width, height)
            }
        }
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
