//! UI Theme configuration with modern styling

use iced::theme::Palette;
use iced::{color, Color, Theme};

/// Custom color palette for a modern dark theme
fn modern_dark_palette() -> Palette {
    Palette {
        background: color!(0x1a1b26),      // Deep dark blue
        text: color!(0xc0caf5),            // Soft white-blue
        primary: color!(0x7aa2f7),         // Vibrant blue
        success: color!(0x9ece6a),         // Green
        danger: color!(0xf7768e),          // Soft red
    }
}

/// Custom color palette for a modern light theme
fn modern_light_palette() -> Palette {
    Palette {
        background: color!(0xf5f5f5),      // Off-white
        text: color!(0x1a1b26),            // Dark text
        primary: color!(0x2563eb),         // Blue
        success: color!(0x16a34a),         // Green
        danger: color!(0xdc2626),          // Red
    }
}

/// Get the app theme based on preference
pub fn get_theme(preference: &str) -> Theme {
    match preference {
        "light" => Theme::custom(
            "Akkurate Light".to_string(),
            modern_light_palette(),
        ),
        _ => Theme::custom(
            "Akkurate Dark".to_string(),
            modern_dark_palette(),
        ),
    }
}

/// UI Colors for custom widgets
pub struct UiColors;

impl UiColors {
    pub fn accent() -> Color {
        color!(0x7aa2f7)
    }
    
    pub fn surface() -> Color {
        color!(0x24283b)
    }
    
    pub fn border() -> Color {
        color!(0x414868)
    }
    
    pub fn text_muted() -> Color {
        color!(0x565f89)
    }
}
