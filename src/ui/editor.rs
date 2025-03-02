//! Editor view component for Zing text editor.

use egui::{Color32, FontId, TextEdit, Ui, Vec2, Rounding, Stroke, TextStyle};
use egui::text::{LayoutJob, TextFormat};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex, Once};
use std::path::PathBuf;

use crate::config::Theme;
use crate::ui::ZingApp;

// Global channel for file operations
static INIT: Once = Once::new();
static mut FILE_OP_SENDER: Option<Sender<FileOperation>> = None;
static mut FILE_OP_RECEIVER: Option<Receiver<FileOperation>> = None;

// File operation types
enum FileOperation {
    OpenComplete(Option<crate::buffer::TextBuffer>),
    SaveComplete(Option<PathBuf>, bool),
    ResetDialogFlag,
}

/// Editor view component.
#[derive(Debug)]
pub struct EditorView;

/// Renders the editor UI.
pub fn ui(app: &mut ZingApp, ui: &mut Ui) {
    // Initialize the channel if not already done
    INIT.call_once(|| {
        let (sender, receiver) = mpsc::channel();
        unsafe {
            FILE_OP_SENDER = Some(sender);
            FILE_OP_RECEIVER = Some(receiver);
        }
    });
    
    // Check for file operation results
    unsafe {
        if let Some(receiver) = &FILE_OP_RECEIVER {
            while let Ok(op) = receiver.try_recv() {
                match op {
                    FileOperation::OpenComplete(Some(buffer)) => {
                        app.set_buffer(buffer);
                        app.set_status("File opened successfully".to_string(), 3.0);
                    },
                    FileOperation::SaveComplete(Some(path), _) => {
                        // Update the tab title and file path in the main app
                        if let Some(tab) = app.tabs.tabs.get_mut(app.tabs.active_tab) {
                            tab.title = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                            tab.file_path = Some(path.clone());
                            tab.is_modified = false;
                        }
                        app.set_status(format!("File saved: {}", path.display()), 3.0);
                    },
                    FileOperation::SaveComplete(None, true) => {
                        app.set_status("Save cancelled".to_string(), 3.0);
                    },
                    FileOperation::SaveComplete(None, false) => {
                        app.set_status("Failed to save file".to_string(), 5.0);
                    },
                    FileOperation::ResetDialogFlag => {
                        app.file_dialog_open = false;
                    },
                    _ => {}
                }
            }
        }
    }

    let buffer = app.buffer();
    let mut buffer_lock = buffer.lock().unwrap();
    
    // Get the content as a string
    let mut content_str = buffer_lock.content.to_string();
    
    // Store the font size for the layouter
    let font_size = app.config.font_size;
    
    // Get available size
    let available_size = ui.available_size();
    
    // Set up editor styling based on theme
    let is_dark = matches!(app.config.theme, Theme::Dark);
    
    // Define modern color scheme with extreme contrast
    let (bg_color, text_color) = if is_dark {
        (
            Color32::from_rgb(10, 10, 15),     // Dark background
            Color32::from_rgb(255, 255, 255)   // Pure white text
        )
    } else {
        (
            Color32::from_rgb(252, 252, 255),  // Light background
            Color32::from_rgb(0, 0, 0)         // Pure black text
        )
    };
    
    // Set the background color for the entire UI
    ui.style_mut().visuals.panel_fill = bg_color;
    ui.style_mut().visuals.window_fill = bg_color;
    ui.style_mut().visuals.faint_bg_color = bg_color;
    ui.style_mut().visuals.extreme_bg_color = bg_color;
    ui.style_mut().visuals.code_bg_color = bg_color;
    
    // Set text colors with maximum contrast using fg_stroke
    ui.style_mut().visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text_color);
    ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
    ui.style_mut().visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, text_color);
    ui.style_mut().visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, text_color);
    
    // Remove all strokes and borders
    ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;
    ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
    ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    
    // Set consistent background fills
    ui.style_mut().visuals.widgets.noninteractive.bg_fill = bg_color;
    ui.style_mut().visuals.widgets.inactive.bg_fill = bg_color;
    ui.style_mut().visuals.widgets.active.bg_fill = bg_color;
    ui.style_mut().visuals.widgets.hovered.bg_fill = bg_color;
    
    // Set text and selection colors
    ui.style_mut().visuals.widgets.noninteractive.fg_stroke.color = text_color;
    ui.style_mut().visuals.widgets.inactive.fg_stroke.color = text_color;
    ui.style_mut().visuals.widgets.active.fg_stroke.color = text_color;
    ui.style_mut().visuals.widgets.hovered.fg_stroke.color = text_color;
    ui.style_mut().visuals.selection.bg_fill = Color32::from_rgba_premultiplied(100, 100, 255, 100);
    ui.style_mut().visuals.selection.stroke = Stroke::NONE;
    
    // Customize scrolling behavior
    ui.style_mut().visuals.clip_rect_margin = 0.0;
    ui.style_mut().spacing.item_spacing = Vec2::splat(0.0);
    ui.style_mut().spacing.window_margin = egui::Margin::same(0.0);
    
    // Create a scrollable area for the editor content
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show(ui, |ui| {
            // Override text color settings directly
            ui.visuals_mut().override_text_color = Some(text_color);
            
            // Add padding at the top to ensure first line is visible
            ui.add_space(20.0);
            
            // Create a text edit widget with explicit styling
            let text_edit = TextEdit::multiline(&mut content_str)
                .font(FontId::monospace(font_size))
                .desired_width(f32::INFINITY)
                .desired_rows(50)  // Set a large number of visible rows to encourage scrolling
                .interactive(true) // Ensure it's interactive
                .text_color(text_color)
                .frame(false); // Remove frame to maximize space

            // Show the text edit widget
            let output = text_edit.show(ui);
            let response = output.response;

            // Update cursor position using TextEdit's output
            if let Some(cursor_range) = output.cursor_range {
                app.cursor_pos = cursor_range.primary.ccursor.index;
            }
            
            // Add some space at the bottom
            ui.add_space(100.0);
            
            // Handle text changes and cursor position
            if response.changed() {
                // If the text changed, update the buffer content
                buffer_lock.update_content(&content_str).unwrap();
                
                // Mark the current tab as modified
                if let Some(tab) = app.tabs.tabs.get_mut(app.tabs.active_tab) {
                    tab.is_modified = true;
                }
            }
        });

    // Get cursor position for status bar
    let cursor_info = if let Ok((line, col)) = buffer_lock.char_to_line_col(app.cursor_pos) {
        Some((line, col))
    } else {
        None
    };
    
    // Update the app with cursor position info for the status bar
    if let Some((line, col)) = cursor_info {
        app.cursor_line = line;
        app.cursor_column = col;
    }
    
    // Release the lock before calling functions that might need it
    drop(buffer_lock);
    
    // Handle keyboard shortcuts
    if ui.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
        // Ctrl+S: Save
        save_file(app, false);
    } else if ui.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift) {
        // Ctrl+Shift+S: Save As
        save_file(app, true);
    } else if ui.input(|i| i.key_pressed(egui::Key::O) && i.modifiers.ctrl) {
        // Ctrl+O: Open
        open_file(app);
    } else if ui.input(|i| i.key_pressed(egui::Key::P) && i.modifiers.ctrl) {
        // Ctrl+P: Print
        print_file(app);
    } else if ui.input(|i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && !i.modifiers.shift) {
        // Ctrl+Z: Undo
        undo(app);
    } else if (ui.input(|i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && i.modifiers.shift) ||
              ui.input(|i| i.key_pressed(egui::Key::Y) && i.modifiers.ctrl)) {
        // Ctrl+Shift+Z or Ctrl+Y: Redo
        redo(app);
    }
}

/// Opens a file using a file dialog.
pub fn open_file(app: &mut ZingApp) {
    if app.file_dialog_open {
        return;
    }
    
    app.file_dialog_open = true;
    let tabs = Arc::new(Mutex::new(app.tabs.clone()));
    
    // Use a background thread for file dialog to avoid blocking the UI
    std::thread::spawn({
        let sender = unsafe { FILE_OP_SENDER.clone().unwrap() };
        let tabs = Arc::clone(&tabs);
        
        move || {
            if let Some(path) = crate::file_io::open_file_dialog() {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                
                runtime.block_on(async {
                    match crate::file_io::load_file(&path).await {
                        Ok(new_buffer) => {
                            // Create a new tab for the opened file
                            sender.send(FileOperation::OpenComplete(Some(new_buffer))).ok();
                            // Update the tab information
                            let mut tabs = tabs.lock().unwrap();
                            tabs.tabs.push(crate::ui::tabs::Tab::new(
                                path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                                Some(path)
                            ));
                            tabs.active_tab = tabs.tabs.len() - 1;
                        }
                        Err(err) => {
                            log::error!("Failed to load file: {}", err);
                            sender.send(FileOperation::OpenComplete(None)).ok();
                        }
                    }
                });
            }
            
            // Reset the file dialog flag
            sender.send(FileOperation::ResetDialogFlag).ok();
        }
    });
}

/// Saves the current buffer to a file.
pub fn save_file(app: &mut ZingApp, save_as: bool) {
    if app.file_dialog_open {
        return;
    }
    
    let buffer = app.buffer();
    let buffer_lock = buffer.lock().unwrap();
    
    // If save_as is true or the buffer has no associated file path, show a save dialog
    let need_path = save_as || buffer_lock.file_path.is_none();
    
    // Get the current file path if it exists
    let current_path = buffer_lock.file_path.clone();
    
    // Release the lock
    drop(buffer_lock);
    
    if need_path {
        app.file_dialog_open = true;
        let tabs = Arc::new(Mutex::new(app.tabs.clone()));
        let active_tab = app.tabs.active_tab;
        
        // Use a background thread for file dialog to avoid blocking the UI
        std::thread::spawn({
            let buffer = app.buffer();
            let sender = unsafe { FILE_OP_SENDER.clone().unwrap() };
            let tabs = Arc::clone(&tabs);
            
            move || {
                if let Some(path) = crate::file_io::save_file_dialog() {
                    let runtime = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap();
                    
                    runtime.block_on(async {
                        let mut buffer_lock = buffer.lock().unwrap();
                        match buffer_lock.save_to(&path).await {
                            Ok(_) => {
                                log::info!("File saved successfully: {}", path.display());
                                // Update the tab information
                                let mut tabs = tabs.lock().unwrap();
                                if let Some(tab) = tabs.tabs.get_mut(active_tab) {
                                    tab.title = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                    tab.file_path = Some(path.clone());
                                    tab.is_modified = false;
                                }
                                sender.send(FileOperation::SaveComplete(Some(path), true)).ok();
                            }
                            Err(err) => {
                                log::error!("Failed to save file: {}", err);
                                sender.send(FileOperation::SaveComplete(None, false)).ok();
                            }
                        }
                    });
                } else {
                    // User cancelled the save dialog
                    sender.send(FileOperation::SaveComplete(None, true)).ok();
                }
                
                // Reset the file dialog flag
                sender.send(FileOperation::ResetDialogFlag).ok();
            }
        });
    } else if let Some(path) = current_path {
        // Save to the existing file path
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        
        let buffer = app.buffer();
        let path_clone = path.clone();
        let tabs = Arc::new(Mutex::new(app.tabs.clone()));
        let active_tab = app.tabs.active_tab;
        
        runtime.block_on(async {
            let mut buffer_lock = buffer.lock().unwrap();
            match buffer_lock.save().await {
                Ok(_) => {
                    log::info!("File saved successfully: {}", path_clone.display());
                    // Update the tab information
                    let mut tabs = tabs.lock().unwrap();
                    if let Some(tab) = tabs.tabs.get_mut(active_tab) {
                        tab.is_modified = false;
                    }
                    app.set_status(format!("Saved file: {}", path_clone.display()), 3.0);
                }
                Err(err) => {
                    log::error!("Failed to save file: {}", err);
                    app.set_status(format!("Failed to save file: {}", err), 5.0);
                }
            }
        });
    }
}

/// Prints the current buffer.
pub fn print_file(app: &mut ZingApp) {
    let buffer = app.buffer();
    let buffer_lock = buffer.lock().unwrap();
    
    match crate::file_io::print_buffer(&buffer_lock) {
        Ok(_) => {
            app.set_status("File opened with default application for printing.".to_string(), 3.0);
        }
        Err(err) => {
            app.set_status(format!("Failed to print: {}", err), 5.0);
            log::error!("Failed to print: {}", err);
        }
    }
}

/// Creates a new empty tab.
pub fn new_tab(app: &mut ZingApp) {
    // Create a new tab with an empty buffer
    let tab = crate::ui::tabs::Tab::new("Untitled".to_string(), None);
    app.tabs.tabs.push(tab);
    app.tabs.active_tab = app.tabs.tabs.len() - 1;
    app.cursor_pos = 0;
    app.set_status("Created new tab".to_string(), 2.0);
}

/// Performs an undo operation on the current buffer.
pub fn undo(app: &mut ZingApp) {
    let buffer = app.buffer();
    let mut buffer_lock = buffer.lock().unwrap();
    
    match buffer_lock.undo() {
        Ok(_) => {
            app.set_status("Undo successful".to_string(), 2.0);
        }
        Err(err) => {
            app.set_status(format!("Failed to undo: {}", err), 3.0);
            log::error!("Failed to undo: {}", err);
        }
    }
}

/// Performs a redo operation on the current buffer.
pub fn redo(app: &mut ZingApp) {
    let buffer = app.buffer();
    let mut buffer_lock = buffer.lock().unwrap();
    
    match buffer_lock.redo() {
        Ok(_) => {
            app.set_status("Redo successful".to_string(), 2.0);
        }
        Err(err) => {
            app.set_status(format!("Failed to redo: {}", err), 3.0);
            log::error!("Failed to redo: {}", err);
        }
    }
}

/// Closes the current tab.
pub fn close_tab(app: &mut ZingApp) {
    // Check if this is the last tab
    if app.tabs.tabs.len() <= 1 {
        // This is the last tab, warn the user
        app.set_status("Warning: Closing the last tab will quit the application. Press Cmd+W again to confirm.".to_string(), 5.0);
        
        // Set a flag in the app to track that the user has been warned
        app.last_tab_close_warning = true;
        return;
    }
    
    // If we previously showed a warning for the last tab but now have multiple tabs,
    // reset the warning flag
    if app.last_tab_close_warning && app.tabs.tabs.len() > 1 {
        app.last_tab_close_warning = false;
    }
    
    // Try to close the current tab
    if app.tabs.close_tab() {
        app.set_status("Tab closed".to_string(), 2.0);
    }
} 