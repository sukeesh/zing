//! Toolbar component for Zing text editor.

use egui::{Color32, RichText, Ui, Vec2, Stroke, Rounding};

use crate::ui::ZingApp;
use crate::ui::editor;
use crate::config::Theme;

/// Toolbar component.
#[derive(Debug)]
pub struct Toolbar;

/// Draw a custom icon in the UI
fn draw_icon(ui: &mut Ui, icon_type: &str, is_dark: bool, word_wrap: bool) {
    let rect = ui.available_rect_before_wrap().shrink(8.0);
    let center = rect.center();
    let stroke_width = 1.5;
    let text_color = ui.visuals().widgets.inactive.fg_stroke.color;
    let stroke = Stroke::new(stroke_width, text_color);
    
    match icon_type {
        "open" => {
            // Folder icon
            let folder_top = center.y - 4.0;
            let folder_left = center.x - 6.0;
            let folder_right = center.x + 6.0;
            let folder_bottom = center.y + 4.0;
            
            // Folder base
            ui.painter().rect_stroke(
                egui::Rect::from_min_max(
                    egui::pos2(folder_left, folder_top),
                    egui::pos2(folder_right, folder_bottom)
                ),
                Rounding::same(1.0),
                stroke
            );
            
            // Folder tab
            ui.painter().line_segment(
                [
                    egui::pos2(folder_left + 2.0, folder_top),
                    egui::pos2(folder_left + 4.0, folder_top - 2.0)
                ],
                stroke
            );
            ui.painter().line_segment(
                [
                    egui::pos2(folder_left + 4.0, folder_top - 2.0),
                    egui::pos2(folder_left + 7.0, folder_top - 2.0)
                ],
                stroke
            );
            ui.painter().line_segment(
                [
                    egui::pos2(folder_left + 7.0, folder_top - 2.0),
                    egui::pos2(folder_left + 7.0, folder_top)
                ],
                stroke
            );
        },
        "save" => {
            // Save icon (floppy disk)
            let size = 10.0;
            let left = center.x - size/2.0;
            let top = center.y - size/2.0;
            
            // Outer square
            ui.painter().rect_stroke(
                egui::Rect::from_min_max(
                    egui::pos2(left, top),
                    egui::pos2(left + size, top + size)
                ),
                Rounding::same(1.0),
                stroke
            );
            
            // Inner square (disk label)
            ui.painter().rect_stroke(
                egui::Rect::from_min_max(
                    egui::pos2(left + size*0.25, top + size*0.25),
                    egui::pos2(left + size*0.75, top + size*0.5)
                ),
                Rounding::ZERO,
                stroke
            );
        },
        "save_as" => {
            // Save As icon (floppy with plus)
            let size = 10.0;
            let left = center.x - size/2.0;
            let top = center.y - size/2.0;
            
            // Outer square
            ui.painter().rect_stroke(
                egui::Rect::from_min_max(
                    egui::pos2(left, top),
                    egui::pos2(left + size, top + size)
                ),
                Rounding::same(1.0),
                stroke
            );
            
            // Plus sign
            ui.painter().line_segment(
                [
                    egui::pos2(left + size*0.5, top + size*0.25),
                    egui::pos2(left + size*0.5, top + size*0.75)
                ],
                stroke
            );
            ui.painter().line_segment(
                [
                    egui::pos2(left + size*0.25, top + size*0.5),
                    egui::pos2(left + size*0.75, top + size*0.5)
                ],
                stroke
            );
        },
        "print" => {
            // Printer icon
            let width = 12.0;
            let height = 10.0;
            let left = center.x - width/2.0;
            let top = center.y - height/2.0;
            
            // Printer body
            ui.painter().rect_stroke(
                egui::Rect::from_min_max(
                    egui::pos2(left, top + 2.0),
                    egui::pos2(left + width, top + height - 2.0)
                ),
                Rounding::same(1.0),
                stroke
            );
            
            // Paper
            ui.painter().rect_stroke(
                egui::Rect::from_min_max(
                    egui::pos2(left + 2.0, top),
                    egui::pos2(left + width - 2.0, top + 2.0)
                ),
                Rounding::ZERO,
                stroke
            );
            
            // Output paper
            ui.painter().rect_stroke(
                egui::Rect::from_min_max(
                    egui::pos2(left + 2.0, top + height - 2.0),
                    egui::pos2(left + width - 2.0, top + height)
                ),
                Rounding::ZERO,
                stroke
            );
        },
        "theme" => {
            // Moon/sun icon
            let radius = 5.0;
            
            if is_dark {
                // Sun icon for dark mode
                ui.painter().circle_stroke(center, radius, stroke);
                
                // Sun rays
                let ray_length = 2.0;
                for i in 0..8 {
                    let angle = i as f32 * std::f32::consts::PI / 4.0;
                    let start_x = center.x + (radius + 1.0) * angle.cos();
                    let start_y = center.y + (radius + 1.0) * angle.sin();
                    let end_x = center.x + (radius + ray_length + 1.0) * angle.cos();
                    let end_y = center.y + (radius + ray_length + 1.0) * angle.sin();
                    
                    ui.painter().line_segment(
                        [egui::pos2(start_x, start_y), egui::pos2(end_x, end_y)],
                        stroke
                    );
                }
            } else {
                // Moon icon for light mode
                ui.painter().circle_stroke(center, radius, stroke);
                ui.painter().circle_filled(
                    egui::pos2(center.x + 2.0, center.y - 2.0),
                    radius - 1.0,
                    ui.style().visuals.widgets.inactive.bg_fill
                );
            }
        },
        "line_numbers" => {
            // Line numbers icon
            let width = 10.0;
            let height = 10.0;
            let left = center.x - width/2.0;
            let top = center.y - height/2.0;
            
            // Document outline
            ui.painter().rect_stroke(
                egui::Rect::from_min_max(
                    egui::pos2(left, top),
                    egui::pos2(left + width, top + height)
                ),
                Rounding::same(1.0),
                stroke
            );
            
            // Line number column
            ui.painter().line_segment(
                [
                    egui::pos2(left + 3.0, top),
                    egui::pos2(left + 3.0, top + height)
                ],
                stroke
            );
            
            // Text lines
            for i in 0..3 {
                let y = top + 2.5 + i as f32 * 2.5;
                ui.painter().line_segment(
                    [
                        egui::pos2(left + 4.0, y),
                        egui::pos2(left + width - 1.0, y)
                    ],
                    Stroke::new(0.7, text_color)
                );
            }
        },
        "word_wrap" => {
            // Word wrap icon
            let width = 10.0;
            let height = 10.0;
            let left = center.x - width/2.0;
            let top = center.y - height/2.0;
            
            // Text lines
            for i in 0..2 {
                let y = top + 3.0 + i as f32 * 4.0;
                ui.painter().line_segment(
                    [
                        egui::pos2(left, y),
                        egui::pos2(left + width, y)
                    ],
                    stroke
                );
            }
            
            // Wrap arrow
            if word_wrap {
                let arrow_y = top + 7.0;
                ui.painter().line_segment(
                    [
                        egui::pos2(left, arrow_y),
                        egui::pos2(left + width - 3.0, arrow_y)
                    ],
                    stroke
                );
                
                // Arrow head
                ui.painter().line_segment(
                    [
                        egui::pos2(left + width - 3.0, arrow_y),
                        egui::pos2(left + width - 5.0, arrow_y - 2.0)
                    ],
                    stroke
                );
                ui.painter().line_segment(
                    [
                        egui::pos2(left + width - 3.0, arrow_y),
                        egui::pos2(left + width - 5.0, arrow_y + 2.0)
                    ],
                    stroke
                );
            } else {
                // No wrap arrow
                ui.painter().line_segment(
                    [
                        egui::pos2(left, top + 7.0),
                        egui::pos2(left + width, top + 7.0)
                    ],
                    stroke
                );
                
                // Arrow head
                ui.painter().line_segment(
                    [
                        egui::pos2(left + width, top + 7.0),
                        egui::pos2(left + width - 2.0, top + 5.0)
                    ],
                    stroke
                );
                ui.painter().line_segment(
                    [
                        egui::pos2(left + width, top + 7.0),
                        egui::pos2(left + width - 2.0, top + 9.0)
                    ],
                    stroke
                );
            }
        },
        _ => {}
    }
}

/// Renders the toolbar UI.
pub fn ui(app: &mut ZingApp, ui: &mut Ui) {
    // Set up toolbar styling
    let is_dark = matches!(app.config.theme, Theme::Dark);
    let accent_color = if is_dark { Color32::from_rgb(75, 135, 220) } else { Color32::from_rgb(59, 130, 246) };
    let hover_color = if is_dark { Color32::from_rgb(45, 55, 72) } else { Color32::from_rgb(226, 232, 240) };
    let divider_color = if is_dark { Color32::from_gray(45) } else { Color32::from_gray(220) };
    
    // Style the toolbar background
    let toolbar_height = 36.0;
    let toolbar_rect = ui.max_rect();
    ui.painter().rect_filled(
        egui::Rect::from_min_size(
            toolbar_rect.min,
            egui::vec2(toolbar_rect.width(), toolbar_height)
        ),
        Rounding::ZERO,
        if is_dark { Color32::from_rgb(30, 30, 30) } else { Color32::from_rgb(245, 245, 245) }
    );
    
    // Configure button styling
    ui.style_mut().visuals.button_frame = true;
    ui.style_mut().spacing.button_padding = Vec2::new(10.0, 6.0);
    ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    ui.style_mut().visuals.widgets.hovered.bg_fill = hover_color;
    ui.style_mut().visuals.widgets.active.bg_fill = accent_color.linear_multiply(0.8);
    ui.style_mut().visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    
    // Create a horizontal layout with some padding
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.set_height(36.0);
        
        // File operations buttons with modern icons
        let button_size = Vec2::new(32.0, 28.0);
        let text_color = ui.visuals().widgets.inactive.fg_stroke.color;
        let word_wrap = app.config.word_wrap;
        
        // Draw the icons directly on the buttons
        let draw_button = |ui: &mut Ui, icon_type: &str, tooltip: &str| -> bool {
            let btn = ui.add_sized(button_size, egui::Button::new(" ")).on_hover_text(tooltip);
            
            // Draw the icon on the button
            let rect = btn.rect;
            let painter = ui.painter();
            let center = rect.center();
            let stroke_width = 1.5;
            let stroke = Stroke::new(stroke_width, text_color);
            
            match icon_type {
                "open" => {
                    // Folder icon
                    let folder_top = center.y - 4.0;
                    let folder_left = center.x - 6.0;
                    let folder_right = center.x + 6.0;
                    let folder_bottom = center.y + 4.0;
                    
                    // Folder base
                    painter.rect_stroke(
                        egui::Rect::from_min_max(
                            egui::pos2(folder_left, folder_top),
                            egui::pos2(folder_right, folder_bottom)
                        ),
                        Rounding::same(1.0),
                        stroke
                    );
                    
                    // Folder tab
                    painter.line_segment(
                        [
                            egui::pos2(folder_left + 2.0, folder_top),
                            egui::pos2(folder_left + 4.0, folder_top - 2.0)
                        ],
                        stroke
                    );
                    painter.line_segment(
                        [
                            egui::pos2(folder_left + 4.0, folder_top - 2.0),
                            egui::pos2(folder_left + 7.0, folder_top - 2.0)
                        ],
                        stroke
                    );
                    painter.line_segment(
                        [
                            egui::pos2(folder_left + 7.0, folder_top - 2.0),
                            egui::pos2(folder_left + 7.0, folder_top)
                        ],
                        stroke
                    );
                },
                "save" => {
                    // Save icon (floppy disk)
                    let size = 10.0;
                    let left = center.x - size/2.0;
                    let top = center.y - size/2.0;
                    
                    // Outer square
                    painter.rect_stroke(
                        egui::Rect::from_min_max(
                            egui::pos2(left, top),
                            egui::pos2(left + size, top + size)
                        ),
                        Rounding::same(1.0),
                        stroke
                    );
                    
                    // Inner square (disk label)
                    painter.rect_stroke(
                        egui::Rect::from_min_max(
                            egui::pos2(left + size*0.25, top + size*0.25),
                            egui::pos2(left + size*0.75, top + size*0.5)
                        ),
                        Rounding::ZERO,
                        stroke
                    );
                },
                "save_as" => {
                    // Save As icon (floppy with plus)
                    let size = 10.0;
                    let left = center.x - size/2.0;
                    let top = center.y - size/2.0;
                    
                    // Outer square
                    painter.rect_stroke(
                        egui::Rect::from_min_max(
                            egui::pos2(left, top),
                            egui::pos2(left + size, top + size)
                        ),
                        Rounding::same(1.0),
                        stroke
                    );
                    
                    // Plus sign
                    painter.line_segment(
                        [
                            egui::pos2(left + size*0.5, top + size*0.25),
                            egui::pos2(left + size*0.5, top + size*0.75)
                        ],
                        stroke
                    );
                    painter.line_segment(
                        [
                            egui::pos2(left + size*0.25, top + size*0.5),
                            egui::pos2(left + size*0.75, top + size*0.5)
                        ],
                        stroke
                    );
                },
                "print" => {
                    // Printer icon
                    let width = 12.0;
                    let height = 10.0;
                    let left = center.x - width/2.0;
                    let top = center.y - height/2.0;
                    
                    // Printer body
                    painter.rect_stroke(
                        egui::Rect::from_min_max(
                            egui::pos2(left, top + 2.0),
                            egui::pos2(left + width, top + height - 2.0)
                        ),
                        Rounding::same(1.0),
                        stroke
                    );
                    
                    // Paper
                    painter.rect_stroke(
                        egui::Rect::from_min_max(
                            egui::pos2(left + 2.0, top),
                            egui::pos2(left + width - 2.0, top + 2.0)
                        ),
                        Rounding::ZERO,
                        stroke
                    );
                    
                    // Output paper
                    painter.rect_stroke(
                        egui::Rect::from_min_max(
                            egui::pos2(left + 2.0, top + height - 2.0),
                            egui::pos2(left + width - 2.0, top + height)
                        ),
                        Rounding::ZERO,
                        stroke
                    );
                },
                "theme" => {
                    // Moon/sun icon
                    let radius = 5.0;
                    
                    if is_dark {
                        // Sun icon for dark mode
                        painter.circle_stroke(center, radius, stroke);
                        
                        // Sun rays
                        let ray_length = 2.0;
                        for i in 0..8 {
                            let angle = i as f32 * std::f32::consts::PI / 4.0;
                            let start_x = center.x + (radius + 1.0) * angle.cos();
                            let start_y = center.y + (radius + 1.0) * angle.sin();
                            let end_x = center.x + (radius + ray_length + 1.0) * angle.cos();
                            let end_y = center.y + (radius + ray_length + 1.0) * angle.sin();
                            
                            painter.line_segment(
                                [egui::pos2(start_x, start_y), egui::pos2(end_x, end_y)],
                                stroke
                            );
                        }
                    } else {
                        // Moon icon for light mode
                        painter.circle_stroke(center, radius, stroke);
                        painter.circle_filled(
                            egui::pos2(center.x + 2.0, center.y - 2.0),
                            radius - 1.0,
                            ui.style().visuals.widgets.inactive.bg_fill
                        );
                    }
                },
                "line_numbers" => {
                    // Line numbers icon
                    let width = 10.0;
                    let height = 10.0;
                    let left = center.x - width/2.0;
                    let top = center.y - height/2.0;
                    
                    // Document outline
                    painter.rect_stroke(
                        egui::Rect::from_min_max(
                            egui::pos2(left, top),
                            egui::pos2(left + width, top + height)
                        ),
                        Rounding::same(1.0),
                        stroke
                    );
                    
                    // Line number column
                    painter.line_segment(
                        [
                            egui::pos2(left + 3.0, top),
                            egui::pos2(left + 3.0, top + height)
                        ],
                        stroke
                    );
                    
                    // Text lines
                    for i in 0..3 {
                        let y = top + 2.5 + i as f32 * 2.5;
                        painter.line_segment(
                            [
                                egui::pos2(left + 4.0, y),
                                egui::pos2(left + width - 1.0, y)
                            ],
                            Stroke::new(0.7, text_color)
                        );
                    }
                },
                "word_wrap" => {
                    // Word wrap icon
                    let width = 10.0;
                    let height = 10.0;
                    let left = center.x - width/2.0;
                    let top = center.y - height/2.0;
                    
                    // Text lines
                    for i in 0..2 {
                        let y = top + 3.0 + i as f32 * 4.0;
                        painter.line_segment(
                            [
                                egui::pos2(left, y),
                                egui::pos2(left + width, y)
                            ],
                            stroke
                        );
                    }
                    
                    // Wrap arrow
                    if word_wrap {
                        let arrow_y = top + 7.0;
                        painter.line_segment(
                            [
                                egui::pos2(left, arrow_y),
                                egui::pos2(left + width - 3.0, arrow_y)
                            ],
                            stroke
                        );
                        
                        // Arrow head
                        painter.line_segment(
                            [
                                egui::pos2(left + width - 3.0, arrow_y),
                                egui::pos2(left + width - 5.0, arrow_y - 2.0)
                            ],
                            stroke
                        );
                        painter.line_segment(
                            [
                                egui::pos2(left + width - 3.0, arrow_y),
                                egui::pos2(left + width - 5.0, arrow_y + 2.0)
                            ],
                            stroke
                        );
                    } else {
                        // No wrap arrow
                        painter.line_segment(
                            [
                                egui::pos2(left, top + 7.0),
                                egui::pos2(left + width, top + 7.0)
                            ],
                            stroke
                        );
                        
                        // Arrow head
                        painter.line_segment(
                            [
                                egui::pos2(left + width, top + 7.0),
                                egui::pos2(left + width - 2.0, top + 5.0)
                            ],
                            stroke
                        );
                        painter.line_segment(
                            [
                                egui::pos2(left + width, top + 7.0),
                                egui::pos2(left + width - 2.0, top + 9.0)
                            ],
                            stroke
                        );
                    }
                },
                _ => {}
            }
            
            btn.clicked()
        };
        
        // Open file button
        if draw_button(ui, "open", "Open File (Ctrl+O)") {
            editor::open_file(app);
        }
        
        // Save button
        if draw_button(ui, "save", "Save (Ctrl+S)") {
            editor::save_file(app, false);
        }
        
        // Save As button
        if draw_button(ui, "save_as", "Save As (Ctrl+Shift+S)") {
            editor::save_file(app, true);
        }
        
        // Print button
        if draw_button(ui, "print", "Print (Ctrl+P)") {
            editor::print_file(app);
        }
        
        ui.add_space(8.0);
        // Draw vertical divider
        let divider_rect = ui.available_rect_before_wrap();
        ui.painter().line_segment(
            [
                egui::pos2(divider_rect.min.x, divider_rect.min.y + 8.0),
                egui::pos2(divider_rect.min.x, divider_rect.max.y - 8.0)
            ],
            Stroke::new(1.0, divider_color)
        );
        ui.add_space(8.0);
        
        // Theme toggle button
        if draw_button(ui, "theme", if is_dark { "Switch to Light Mode" } else { "Switch to Dark Mode" }) {
            app.toggle_theme(ui.ctx());
        }
        
        ui.add_space(8.0);
        // Draw vertical divider
        let divider_rect = ui.available_rect_before_wrap();
        ui.painter().line_segment(
            [
                egui::pos2(divider_rect.min.x, divider_rect.min.y + 8.0),
                egui::pos2(divider_rect.min.x, divider_rect.max.y - 8.0)
            ],
            Stroke::new(1.0, divider_color)
        );
        ui.add_space(8.0);
        
        // Font size controls with modern styling
        let text_color = ui.visuals().widgets.inactive.fg_stroke.color;
        
        if ui.add_sized(Vec2::new(28.0, 28.0), 
                        egui::Button::new(RichText::new("A-").size(14.0).color(text_color)))
            .on_hover_text("Decrease Font Size")
            .clicked() 
        {
            app.config.decrease_font_size();
            app.config.apply_to_context(ui.ctx());
        }
        
        ui.label(RichText::new(format!("{:.0}", app.config.font_size)).size(14.0));
        
        if ui.add_sized(Vec2::new(28.0, 28.0), 
                        egui::Button::new(RichText::new("A+").size(14.0).color(text_color)))
            .on_hover_text("Increase Font Size")
            .clicked() 
        {
            app.config.increase_font_size();
            app.config.apply_to_context(ui.ctx());
        }
        
        ui.add_space(8.0);
        // Draw vertical divider
        let divider_rect = ui.available_rect_before_wrap();
        ui.painter().line_segment(
            [
                egui::pos2(divider_rect.min.x, divider_rect.min.y + 8.0),
                egui::pos2(divider_rect.min.x, divider_rect.max.y - 8.0)
            ],
            Stroke::new(1.0, divider_color)
        );
        ui.add_space(8.0);
        
        // Line numbers button
        if draw_button(ui, "line_numbers", if app.config.show_line_numbers { "Hide Line Numbers" } else { "Show Line Numbers" }) {
            app.config.toggle_line_numbers();
        }
        
        // Word wrap button
        if draw_button(ui, "word_wrap", if app.config.word_wrap { "Disable Word Wrap" } else { "Enable Word Wrap" }) {
            app.config.toggle_word_wrap();
        }
    });
} 