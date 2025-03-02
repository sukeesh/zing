//! UI module for Zing text editor.
//!
//! This module provides the user interface components for the editor.

pub mod editor;
pub mod statusbar;
pub mod toolbar;
pub mod tabs;

use editor::EditorView;
use toolbar::Toolbar;
use statusbar::StatusBar;
use tabs::TabsView;

use egui::{Context, Ui, Vec2, Rounding, Color32, Stroke};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::buffer::TextBuffer;
use crate::config::{EditorConfig, Theme};

/// Main application state.
#[derive(Debug)]
pub struct ZingApp {
    /// Editor configuration
    pub config: EditorConfig,
    /// Cursor position (character index)
    pub cursor_pos: usize,
    /// Cursor line position (0-indexed)
    pub cursor_line: usize,
    /// Cursor column position (0-indexed)
    pub cursor_column: usize,
    /// Whether a file dialog is open
    pub file_dialog_open: bool,
    /// Status message to display
    pub status_message: Option<(String, Instant)>,
    /// Status message timeout in seconds
    status_timeout: f32,
    /// Tabs view
    pub tabs: TabsView,
    /// Flag to track if the user has been warned about closing the last tab
    pub last_tab_close_warning: bool,
}

impl ZingApp {
    /// Creates a new application instance.
    pub fn new(ctx: &Context) -> Self {
        let config = EditorConfig::default();
        config.apply_to_context(ctx);
        
        Self {
            config,
            cursor_pos: 0,
            cursor_line: 0,
            cursor_column: 0,
            file_dialog_open: false,
            status_message: None,
            status_timeout: 5.0,
            tabs: TabsView::new(),
            last_tab_close_warning: false,
        }
    }
    
    /// Sets the current buffer.
    pub fn set_buffer(&mut self, buffer: TextBuffer) {
        let title = buffer.file_path.as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        
        let tab = tabs::Tab::with_buffer(title, buffer.file_path.clone(), buffer);
        self.tabs.tabs.push(tab);
        self.tabs.active_tab = self.tabs.tabs.len() - 1;
        self.cursor_pos = 0;
    }
    
    /// Gets a reference to the current buffer.
    pub fn buffer(&self) -> Arc<Mutex<TextBuffer>> {
        self.tabs.active_buffer()
    }
    
    /// Sets a status message to display.
    pub fn set_status(&mut self, message: String, duration: f32) {
        self.status_message = Some((message, Instant::now() + std::time::Duration::from_secs_f32(duration)));
    }
    
    /// Updates the status message timeout.
    pub fn update_status(&mut self, delta_time: f32) {
        if let Some((_, expiration)) = &self.status_message {
            if Instant::now() >= *expiration {
                self.status_message = None;
            }
        }
    }
    
    /// Toggles the theme.
    pub fn toggle_theme(&mut self, ctx: &Context) {
        self.config.toggle_theme();
        self.config.apply_to_context(ctx);
    }
}

/// The main application UI.
pub fn ui(app: &mut ZingApp, ctx: &Context) {
    // Set up the main panel with proper styling
    let is_dark = matches!(app.config.theme, Theme::Dark);
    let bg_color = if is_dark {
        Color32::from_rgb(18, 18, 24)
    } else {
        Color32::from_rgb(248, 248, 252)
    };
    
    // Configure the central panel
    egui::CentralPanel::default()
        .frame(egui::Frame::default()
            .fill(bg_color)
            .inner_margin(0.0)
            .outer_margin(0.0))
        .show(ctx, |ui| {
            // Get the total available size
            let total_size = ui.available_size();
            
            // Define fixed heights for status bar and toolbar (if needed)
            let statusbar_height = 24.0;
            let toolbar_height = 24.0;
            let tabs_width = 140.0;
            
            // Determine if we need to show the toolbar (only on non-macOS platforms)
            #[cfg(not(target_os = "macos"))]
            let show_toolbar = true;
            
            #[cfg(target_os = "macos")]
            let show_toolbar = false;
            
            // Calculate the editor size
            let editor_width = total_size.x - tabs_width - 1.0; // 1px for separator
            let editor_height = if show_toolbar {
                total_size.y - toolbar_height - statusbar_height - 2.0 // 2px for separators
            } else {
                total_size.y - statusbar_height - 1.0 // 1px for separator
            };
            
            // Create a horizontal layout for the entire UI
            ui.horizontal(|ui| {
                // Left side: Tabs panel
                ui.vertical(|ui| {
                    ui.set_min_width(tabs_width);
                    ui.set_max_width(tabs_width);
                    ui.set_min_height(total_size.y);
                    app.tabs.ui(ui, app.config.theme);
                });
                
                // Vertical separator
                ui.add(egui::Separator::default().vertical().spacing(1.0));
                
                // Right side: Main content
                ui.vertical(|ui| {
                    // Top: Toolbar (only on non-macOS platforms)
                    if show_toolbar {
                        ui.allocate_ui_with_layout(
                            Vec2::new(editor_width, toolbar_height),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| { toolbar::ui(app, ui); }
                        );
                        
                        // Horizontal separator
                        ui.add(egui::Separator::default().horizontal().spacing(1.0));
                    }
                    
                    // Middle: Editor (takes most space)
                    let editor_rect = ui.allocate_rect(
                        egui::Rect::from_min_size(
                            ui.cursor().min,
                            Vec2::new(editor_width, editor_height)
                        ),
                        egui::Sense::hover()
                    );
                    
                    // Create a child UI for the editor with the allocated rectangle
                    let mut child_ui = ui.child_ui(editor_rect.rect, egui::Layout::default());
                    editor::ui(app, &mut child_ui);
                    
                    // Horizontal separator
                    ui.add(egui::Separator::default().horizontal().spacing(1.0));
                    
                    // Bottom: Status bar
                    ui.allocate_ui_with_layout(
                        Vec2::new(editor_width, statusbar_height),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| { statusbar::ui(app, ui); }
                    );
                });
            });
        });
} 