//! Status bar component for Zing text editor.

use egui::{Color32, Ui, Stroke, Rect, Pos2, FontId, Rounding, Vec2, Sense};

use crate::ui::ZingApp;
use crate::config::Theme;

/// Status bar component.
#[derive(Debug)]
pub struct StatusBar;

/// Renders the status bar UI.
pub fn ui(app: &mut ZingApp, ui: &mut Ui) {
    let is_dark = matches!(app.config.theme, Theme::Dark);
    
    // Get the status bar height - extremely compact
    let height = 10.0;
    
    // Calculate the status bar rect
    let rect = ui.available_rect_before_wrap();
    let status_rect = Rect::from_min_size(
        rect.min,
        Vec2::new(rect.width(), height),
    );
    
    // Draw the status bar background
    let bg_color = if is_dark {
        Color32::from_rgb(30, 30, 40)
    } else {
        Color32::from_rgb(230, 230, 235)
    };
    
    ui.painter().rect_filled(
        status_rect,
        0.0,
        bg_color,
    );
    
    // Draw the status message if there is one
    if let Some((message, _)) = &app.status_message {
        let text_color = if is_dark {
            Color32::from_rgb(200, 200, 200)
        } else {
            Color32::from_rgb(60, 60, 60)
        };
        
        ui.painter().text(
            status_rect.min + Vec2::new(4.0, height / 2.0),
            egui::Align2::LEFT_CENTER,
            message,
            FontId::proportional(10.0), // Smaller font
            text_color,
        );
    }
    
    // Draw the cursor position on the right side
    let text_color = if is_dark {
        Color32::from_rgb(220, 220, 220)  // Much brighter for dark mode
    } else {
        Color32::from_rgb(30, 30, 30)     // Much darker for light mode
    };
    
    let cursor_text = format!("Ln {}, Col {}", app.cursor_line + 1, app.cursor_column + 1);
    let font_id = FontId::proportional(11.0); // Slightly larger font
    let galley = ui.painter().layout_no_wrap(
        cursor_text.clone(),
        font_id.clone(),
        text_color,
    );
    
    // Add a subtle background behind the line/column text for better visibility
    let text_rect = Rect::from_min_size(
        status_rect.right_center() - Vec2::new(8.0 + galley.size().x, galley.size().y / 2.0),
        Vec2::new(galley.size().x + 8.0, galley.size().y)
    );
    
    let bg_highlight = if is_dark {
        Color32::from_rgb(40, 40, 55)  // Slightly lighter than background for dark mode
    } else {
        Color32::from_rgb(210, 210, 220)  // Slightly darker than background for light mode
    };
    
    ui.painter().rect_filled(
        text_rect,
        3.0,  // Rounded corners
        bg_highlight,
    );
    
    ui.painter().text(
        status_rect.right_center() - Vec2::new(4.0 + galley.size().x, 0.0),
        egui::Align2::RIGHT_CENTER,
        cursor_text,
        font_id,
        text_color,
    );
    
    // Allocate the space for the status bar
    ui.allocate_rect(status_rect, Sense::hover());
} 