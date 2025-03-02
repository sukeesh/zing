use egui::{Color32, Rect, Response, Sense, Stroke, Ui, Vec2};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::config::Theme;
use crate::buffer::TextBuffer;

#[derive(Debug, Clone)]
pub struct Tab {
    pub title: String,
    pub file_path: Option<PathBuf>,
    pub is_modified: bool,
    pub buffer: Arc<Mutex<TextBuffer>>,
}

impl Tab {
    pub fn new(title: String, file_path: Option<PathBuf>) -> Self {
        Self {
            title,
            file_path,
            is_modified: false,
            buffer: Arc::new(Mutex::new(TextBuffer::new())),
        }
    }

    pub fn with_buffer(title: String, file_path: Option<PathBuf>, buffer: TextBuffer) -> Self {
        Self {
            title,
            file_path,
            is_modified: false,
            buffer: Arc::new(Mutex::new(buffer)),
        }
    }

    pub fn display_name(&self) -> String {
        if let Some(path) = &self.file_path {
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Untitled")
                .to_string()
        } else {
            "Untitled".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TabsView {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

impl TabsView {
    pub fn new() -> Self {
        let mut tabs = Vec::new();
        tabs.push(Tab::new("Untitled".to_string(), None));
        Self {
            tabs,
            active_tab: 0,
        }
    }

    pub fn active_buffer(&self) -> Arc<Mutex<TextBuffer>> {
        self.tabs[self.active_tab].buffer.clone()
    }
    
    /// Creates a new tab and makes it active
    pub fn new_tab(&mut self) {
        // Create a new tab
        self.tabs.push(Tab::new("Untitled".to_string(), None));
        self.active_tab = self.tabs.len() - 1;
    }

    /// Closes the current active tab
    /// Returns true if successful, false if it was the last tab
    pub fn close_tab(&mut self) -> bool {
        // Don't close the last tab
        if self.tabs.len() <= 1 {
            return false;
        }
        
        // Remove the active tab
        self.tabs.remove(self.active_tab);
        
        // Adjust the active tab index if needed
        if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        }
        
        true
    }

    pub fn ui(&mut self, ui: &mut Ui, theme: Theme) -> Response {
        let is_dark = matches!(theme, Theme::Dark);
        
        // Colors for the tabs
        let (bg_color, active_bg_color, hover_bg_color, text_color, active_text_color) = if is_dark {
            (
                Color32::from_rgb(18, 18, 24),      // Darker background
                Color32::from_rgb(35, 35, 48),      // Slightly lighter for active tab
                Color32::from_rgb(28, 28, 38),      // Hover state
                Color32::from_rgb(160, 160, 180),   // Muted text for inactive tabs
                Color32::from_rgb(230, 230, 250),   // Brighter text for active tab
            )
        } else {
            (
                Color32::from_rgb(235, 235, 240),   // Light gray background
                Color32::from_rgb(250, 250, 255),   // Almost white for active tab
                Color32::from_rgb(242, 242, 247),   // Hover state
                Color32::from_rgb(120, 120, 130),   // Muted text for inactive tabs
                Color32::from_rgb(40, 40, 60),      // Dark text for active tab
            )
        };

        // Set up the tabs panel
        let panel_rect = Rect::from_min_size(
            ui.min_rect().min,
            Vec2::new(140.0, ui.available_height()),  // Increased width from 100.0 to 140.0
        );

        // Draw the tabs background
        ui.painter().rect_filled(
            panel_rect,
            0.0,
            bg_color,
        );

        let mut clicked_tab = None;
        let tab_height = 32.0;  // Slightly shorter tabs
        let tab_padding = Vec2::new(8.0, 0.0);  // Less horizontal padding
        
        // Separator color - very subtle
        let separator_color = if is_dark {
            Color32::from_rgba_premultiplied(255, 255, 255, 10)  // Almost invisible white
        } else {
            Color32::from_rgba_premultiplied(0, 0, 0, 10)  // Almost invisible black
        };

        // Add top padding before the first tab
        let top_padding = 8.0;

        // Draw each tab
        for (index, tab) in self.tabs.iter().enumerate() {
            let is_active = index == self.active_tab;
            let tab_rect = Rect::from_min_size(
                panel_rect.min + Vec2::new(0.0, top_padding + index as f32 * tab_height),
                Vec2::new(panel_rect.width(), tab_height),
            );

            // Handle interactions
            let response = ui.allocate_rect(tab_rect, Sense::click());
            let is_hovered = response.hovered();

            // Background color based on state
            let bg_color = if is_active {
                active_bg_color
            } else if is_hovered {
                hover_bg_color
            } else {
                bg_color
            };

            // Draw tab background with subtle rounded corners on the right side
            if is_active || is_hovered {
                let rounding = egui::Rounding {
                    ne: 4.0,
                    se: 4.0,
                    ..Default::default()
                };
                
                ui.painter().rect_filled(
                    tab_rect,
                    rounding,
                    bg_color,
                );
            } else {
                ui.painter().rect_filled(
                    tab_rect,
                    0.0,
                    bg_color,
                );
            }

            // Active tab indicator - make it more stylish
            if is_active {
                let indicator_color = if is_dark {
                    Color32::from_rgb(86, 156, 255)  // Brighter blue for dark mode
                } else {
                    Color32::from_rgb(0, 120, 215)   // Standard blue for light mode
                };
                
                // Draw a thicker, rounded indicator
                ui.painter().rect_filled(
                    Rect::from_min_size(
                        tab_rect.min,
                        Vec2::new(3.0, tab_height),
                    ),
                    egui::Rounding {
                        ne: 2.0,
                        se: 2.0,
                        ..Default::default()
                    },
                    indicator_color,
                );
                
                // Draw a subtle highlight at the top of the active tab
                ui.painter().rect_filled(
                    Rect::from_min_size(
                        tab_rect.min,
                        Vec2::new(panel_rect.width(), 1.0),
                    ),
                    0.0,
                    indicator_color.linear_multiply(0.7),
                );
            }

            // Draw tab title with icon
            let text_color = if is_active { active_text_color } else { text_color };
            let mut text = tab.display_name();
            
            // Truncate long filenames to fit in the tab
            if text.len() > 10 {
                text = format!("{}...", &text[0..7]);
            }
            
            // Add file icon and modified indicator using simple text characters instead of emojis
            let icon = if tab.is_modified {
                "● "  // Filled circle for modified
            } else {
                "○ "  // Empty circle for unmodified
            };
            
            let display_text = format!("{}{}", icon, text);

            ui.painter().text(
                tab_rect.min + tab_padding + Vec2::new(4.0, tab_height/2.0),
                egui::Align2::LEFT_CENTER,
                display_text,
                egui::FontId::proportional(12.0),  // Slightly smaller font
                text_color,
            );

            if response.clicked() {
                clicked_tab = Some(index);
            }
            
            // Draw separator line AFTER the tab content (not through it)
            // Only draw separator if not the last tab
            if index < self.tabs.len() - 1 {
                let separator_y = tab_rect.min.y + tab_height;
                ui.painter().line_segment(
                    [
                        egui::pos2(tab_rect.min.x + 8.0, separator_y),
                        egui::pos2(tab_rect.max.x - 8.0, separator_y)
                    ],
                    egui::Stroke::new(1.0, separator_color)
                );
            }
        }

        // Handle tab click
        if let Some(index) = clicked_tab {
            self.active_tab = index;
        }
        
        // Add a "New Tab" button at the bottom of the panel
        let new_tab_rect = Rect::from_min_size(
            panel_rect.min + Vec2::new(0.0, panel_rect.height() - 30.0),
            Vec2::new(panel_rect.width(), 30.0),
        );
        
        let new_tab_response = ui.allocate_rect(new_tab_rect, Sense::click());
        let is_new_tab_hovered = new_tab_response.hovered();
        
        // Draw button background
        let new_tab_bg = if is_new_tab_hovered {
            if is_dark {
                Color32::from_rgb(40, 40, 55)
            } else {
                Color32::from_rgb(225, 225, 235)
            }
        } else {
            bg_color
        };
        
        ui.painter().rect_filled(
            new_tab_rect,
            egui::Rounding::same(2.0),
            new_tab_bg,
        );
        
        // Draw a subtle border
        if is_new_tab_hovered {
            let border_color = if is_dark {
                Color32::from_rgba_premultiplied(255, 255, 255, 20)
            } else {
                Color32::from_rgba_premultiplied(0, 0, 0, 20)
            };
            
            ui.painter().rect_stroke(
                new_tab_rect,
                egui::Rounding::same(2.0),
                egui::Stroke::new(1.0, border_color)
            );
        }
        
        // Draw plus icon and text
        let new_tab_text_color = if is_dark {
            Color32::from_rgb(160, 160, 180)
        } else {
            Color32::from_rgb(100, 100, 120)
        };
        
        ui.painter().text(
            new_tab_rect.center(),
            egui::Align2::CENTER_CENTER,
            "+ New",
            egui::FontId::proportional(11.0),
            new_tab_text_color,
        );
        
        // Handle new tab button click
        if new_tab_response.clicked() {
            // Create a new tab
            self.tabs.push(Tab::new("Untitled".to_string(), None));
            self.active_tab = self.tabs.len() - 1;
        }

        // Return the overall response
        ui.allocate_rect(panel_rect, Sense::hover())
    }
} 