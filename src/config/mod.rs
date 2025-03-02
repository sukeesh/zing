//! Configuration module for Zing text editor.
//!
//! This module provides functionality for managing editor settings and themes.

use egui::{Color32, Stroke, Style, Visuals};
use std::sync::Arc;

/// Theme options for the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    /// Light theme with dark text on light background
    Light,
    /// Dark theme with light text on dark background
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

/// Editor configuration settings.
#[derive(Debug, Clone)]
pub struct EditorConfig {
    /// The current theme
    pub theme: Theme,
    /// Font size in points
    pub font_size: f32,
    /// Line spacing factor (1.0 = normal)
    pub line_spacing: f32,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
    /// Whether to wrap text
    pub word_wrap: bool,
    /// Tab size in spaces
    pub tab_size: usize,
    /// Whether to use spaces for tabs
    pub use_spaces: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            font_size: 14.0,
            line_spacing: 1.2,
            show_line_numbers: true,
            word_wrap: true,
            tab_size: 4,
            use_spaces: true,
        }
    }
}

impl EditorConfig {
    /// Creates a new editor configuration with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new editor configuration with the specified theme.
    pub fn with_theme(theme: Theme) -> Self {
        Self {
            theme,
            ..Self::default()
        }
    }

    /// Applies the configuration to the egui context.
    pub fn apply_to_context(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        match self.theme {
            Theme::Light => {
                style.visuals = Visuals::light();
                style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(245, 245, 245);
                style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(20, 20, 20));
            }
            Theme::Dark => {
                style.visuals = Visuals::dark();
                style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(30, 30, 30);
                style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(220, 220, 220));
            }
        }
        
        // Customize text styles
        let mut text_styles = style.text_styles.clone();
        for (_text_style, font_id) in text_styles.iter_mut() {
            font_id.size = self.font_size;
        }
        style.text_styles = text_styles;
        
        ctx.set_style(style);
    }

    /// Toggles between light and dark themes.
    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }

    /// Increases the font size.
    pub fn increase_font_size(&mut self) {
        self.font_size = (self.font_size + 1.0).min(32.0);
    }

    /// Decreases the font size.
    pub fn decrease_font_size(&mut self) {
        self.font_size = (self.font_size - 1.0).max(8.0);
    }

    /// Toggles line numbers.
    pub fn toggle_line_numbers(&mut self) {
        self.show_line_numbers = !self.show_line_numbers;
    }

    /// Toggles word wrap.
    pub fn toggle_word_wrap(&mut self) {
        self.word_wrap = !self.word_wrap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = EditorConfig::default();
        assert_eq!(config.theme, Theme::Dark);
        assert_eq!(config.font_size, 14.0);
        assert_eq!(config.line_spacing, 1.2);
        assert!(config.show_line_numbers);
        assert!(config.word_wrap);
        assert_eq!(config.tab_size, 4);
        assert!(config.use_spaces);
    }

    #[test]
    fn test_with_theme() {
        let config = EditorConfig::with_theme(Theme::Light);
        assert_eq!(config.theme, Theme::Light);
    }

    #[test]
    fn test_toggle_theme() {
        let mut config = EditorConfig::with_theme(Theme::Light);
        config.toggle_theme();
        assert_eq!(config.theme, Theme::Dark);
        config.toggle_theme();
        assert_eq!(config.theme, Theme::Light);
    }

    #[test]
    fn test_font_size_adjustments() {
        let mut config = EditorConfig::default();
        let original_size = config.font_size;
        
        config.increase_font_size();
        assert_eq!(config.font_size, original_size + 1.0);
        
        config.decrease_font_size();
        assert_eq!(config.font_size, original_size);
        
        // Test bounds
        for _ in 0..100 {
            config.increase_font_size();
        }
        assert_eq!(config.font_size, 32.0);
        
        for _ in 0..100 {
            config.decrease_font_size();
        }
        assert_eq!(config.font_size, 8.0);
    }
} 