use egui::{Color32, FontFamily, Stroke, Style, Visuals};

/// Theme configuration and constants for the UI
#[derive(Debug)]
pub struct Theme;

impl Theme {
    /// Dark theme primary background color
    pub const DARK_BG_PRIMARY: Color32 = Color32::from_rgb(26, 26, 26);
    /// Dark theme secondary background color
    pub const DARK_BG_SECONDARY: Color32 = Color32::from_rgb(35, 35, 35);
    /// Dark theme tertiary background color
    pub const DARK_BG_TERTIARY: Color32 = Color32::from_rgb(45, 45, 45);
    /// Dark theme border color
    pub const DARK_BORDER: Color32 = Color32::from_rgb(58, 58, 58);
    /// Dark theme primary text color
    pub const DARK_TEXT_PRIMARY: Color32 = Color32::from_rgb(236, 236, 236);
    /// Dark theme secondary text color
    pub const DARK_TEXT_SECONDARY: Color32 = Color32::from_rgb(155, 155, 155);
    /// Dark theme accent color
    pub const DARK_ACCENT: Color32 = Color32::from_rgb(30, 144, 255);

    /// Light theme primary background color
    pub const LIGHT_BG_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255);
    /// Light theme secondary background color
    pub const LIGHT_BG_SECONDARY: Color32 = Color32::from_rgb(247, 247, 247);
    /// Light theme tertiary background color
    pub const LIGHT_BG_TERTIARY: Color32 = Color32::from_rgb(235, 235, 235);
    /// Light theme border color
    pub const LIGHT_BORDER: Color32 = Color32::from_rgb(227, 227, 227);
    /// Light theme primary text color
    pub const LIGHT_TEXT_PRIMARY: Color32 = Color32::from_rgb(32, 32, 32);
    /// Light theme secondary text color
    pub const LIGHT_TEXT_SECONDARY: Color32 = Color32::from_rgb(110, 110, 110);
    /// Light theme accent color
    pub const LIGHT_ACCENT: Color32 = Color32::from_rgb(0, 102, 204);

    /// Success status color
    pub const SUCCESS: Color32 = Color32::from_rgb(16, 185, 129);
    /// Warning status color
    pub const WARNING: Color32 = Color32::from_rgb(245, 158, 11);
    /// Error status color
    pub const ERROR: Color32 = Color32::from_rgb(239, 68, 68);
    /// Info status color
    pub const INFO: Color32 = Color32::from_rgb(59, 130, 246);

    /// Extra small spacing
    pub const SPACING_XS: f32 = 4.0;
    /// Small spacing
    pub const SPACING_SM: f32 = 8.0;
    /// Medium spacing
    pub const SPACING_MD: f32 = 16.0;
    /// Large spacing
    pub const SPACING_LG: f32 = 24.0;
    /// Extra large spacing
    pub const SPACING_XL: f32 = 32.0;

    /// Small corner radius
    pub const RADIUS_SM: f32 = 4.0;
    /// Medium corner radius
    pub const RADIUS_MD: f32 = 6.0;
    /// Large corner radius
    pub const RADIUS_LG: f32 = 8.0;

    /// Top bar height
    pub const TOP_BAR_HEIGHT: f32 = 48.0;
    /// Bottom bar height
    pub const BOTTOM_BAR_HEIGHT: f32 = 56.0;
    /// Default sidebar width
    pub const SIDEBAR_DEFAULT_WIDTH: f32 = 280.0;
    /// Minimum sidebar width
    pub const SIDEBAR_MIN_WIDTH: f32 = 200.0;
    /// Maximum sidebar width
    pub const SIDEBAR_MAX_WIDTH: f32 = 400.0;

    /// Standard button height
    pub const BUTTON_HEIGHT: f32 = 36.0;
    /// Horizontal button padding
    pub const BUTTON_PADDING_H: f32 = 16.0;
    /// Vertical button padding
    pub const BUTTON_PADDING_V: f32 = 8.0;

    /// Standard row height in lists
    pub const ROW_HEIGHT: f32 = 28.0;
    /// Tree indentation size
    pub const INDENT_SIZE: f32 = 20.0;
    /// Standard icon size
    pub const ICON_SIZE: f32 = 16.0;
    /// Spacing between icon and text
    pub const ICON_SPACING: f32 = 4.0;

    /// Apply theme to egui context
    #[allow(clippy::cast_possible_truncation)]
    pub fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
        let mut style = Style {
            text_styles: [
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(14.0, FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(13.0, FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(13.0, FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Small,
                    egui::FontId::new(11.0, FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(12.0, FontFamily::Monospace),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
            ..Default::default()
        };

        // Configure spacing
        style.spacing.item_spacing = egui::vec2(Self::SPACING_SM, Self::SPACING_SM);
        style.spacing.button_padding = egui::vec2(Self::BUTTON_PADDING_H, Self::BUTTON_PADDING_V);
        style.spacing.menu_margin = egui::Margin::same(Self::SPACING_SM as i8);
        style.spacing.indent = Self::INDENT_SIZE;

        // Configure visuals
        let mut visuals = if dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        // Customize colors
        if dark_mode {
            visuals.panel_fill = Self::DARK_BG_SECONDARY;
            visuals.window_fill = Self::DARK_BG_PRIMARY;
            visuals.window_stroke = Stroke::new(1.0, Self::DARK_BORDER);
            visuals.widgets.noninteractive.bg_fill = Self::DARK_BG_SECONDARY;
            visuals.widgets.noninteractive.weak_bg_fill = Self::DARK_BG_TERTIARY;
            visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Self::DARK_TEXT_PRIMARY);
            visuals.widgets.inactive.bg_fill = Self::DARK_BG_SECONDARY;
            visuals.widgets.inactive.weak_bg_fill = Self::DARK_BG_TERTIARY;
            visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Self::DARK_TEXT_SECONDARY);
            visuals.widgets.hovered.bg_fill = Self::DARK_BG_TERTIARY;
            visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Self::DARK_TEXT_PRIMARY);
            visuals.widgets.active.bg_fill = Self::DARK_ACCENT;
            visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
            visuals.selection.bg_fill = Self::DARK_ACCENT.linear_multiply(0.3);
            visuals.selection.stroke = Stroke::new(1.0, Self::DARK_ACCENT);
            visuals.hyperlink_color = Self::DARK_ACCENT;
            visuals.faint_bg_color = Self::DARK_BG_TERTIARY;
            visuals.extreme_bg_color = Self::DARK_BG_PRIMARY;
            visuals.code_bg_color = Self::DARK_BG_TERTIARY;
        } else {
            visuals.panel_fill = Self::LIGHT_BG_SECONDARY;
            visuals.window_fill = Self::LIGHT_BG_PRIMARY;
            visuals.window_stroke = Stroke::new(1.0, Self::LIGHT_BORDER);
            visuals.widgets.noninteractive.bg_fill = Self::LIGHT_BG_SECONDARY;
            visuals.widgets.noninteractive.weak_bg_fill = Self::LIGHT_BG_TERTIARY;
            visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Self::LIGHT_TEXT_PRIMARY);
            visuals.widgets.inactive.bg_fill = Self::LIGHT_BG_SECONDARY;
            visuals.widgets.inactive.weak_bg_fill = Self::LIGHT_BG_TERTIARY;
            visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Self::LIGHT_TEXT_SECONDARY);
            visuals.widgets.hovered.bg_fill = Self::LIGHT_BG_TERTIARY;
            visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Self::LIGHT_TEXT_PRIMARY);
            visuals.widgets.active.bg_fill = Self::LIGHT_ACCENT;
            visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
            visuals.selection.bg_fill = Self::LIGHT_ACCENT.linear_multiply(0.2);
            visuals.selection.stroke = Stroke::new(1.0, Self::LIGHT_ACCENT);
            visuals.hyperlink_color = Self::LIGHT_ACCENT;
            visuals.faint_bg_color = Self::LIGHT_BG_TERTIARY;
            visuals.extreme_bg_color = Self::LIGHT_BG_PRIMARY;
            visuals.code_bg_color = Self::LIGHT_BG_TERTIARY;
        }

        visuals.warn_fg_color = Self::WARNING;
        visuals.error_fg_color = Self::ERROR;

        // Set corner radius
        visuals.window_corner_radius = Self::RADIUS_LG.into();
        visuals.menu_corner_radius = Self::RADIUS_MD.into();

        ctx.set_visuals(visuals);
        ctx.set_style(style);
    }

    /// Get background color for given theme and level
    pub const fn bg_color(dark_mode: bool, level: BgLevel) -> Color32 {
        match (dark_mode, level) {
            (true, BgLevel::Primary) => Self::DARK_BG_PRIMARY,
            (true, BgLevel::Secondary) => Self::DARK_BG_SECONDARY,
            (true, BgLevel::Tertiary) => Self::DARK_BG_TERTIARY,
            (false, BgLevel::Primary) => Self::LIGHT_BG_PRIMARY,
            (false, BgLevel::Secondary) => Self::LIGHT_BG_SECONDARY,
            (false, BgLevel::Tertiary) => Self::LIGHT_BG_TERTIARY,
        }
    }

    /// Get text color for given theme and emphasis
    pub const fn text_color(dark_mode: bool, emphasis: TextEmphasis) -> Color32 {
        match (dark_mode, emphasis) {
            (true, TextEmphasis::Primary) => Self::DARK_TEXT_PRIMARY,
            (true, TextEmphasis::Secondary) => Self::DARK_TEXT_SECONDARY,
            (false, TextEmphasis::Primary) => Self::LIGHT_TEXT_PRIMARY,
            (false, TextEmphasis::Secondary) => Self::LIGHT_TEXT_SECONDARY,
        }
    }

    /// Get accent color for given theme
    pub const fn accent_color(dark_mode: bool) -> Color32 {
        if dark_mode {
            Self::DARK_ACCENT
        } else {
            Self::LIGHT_ACCENT
        }
    }

    /// Get border color for given theme
    pub const fn border_color(dark_mode: bool) -> Color32 {
        if dark_mode {
            Self::DARK_BORDER
        } else {
            Self::LIGHT_BORDER
        }
    }
}

/// Background color levels
#[derive(Clone, Copy, Debug)]
pub enum BgLevel {
    /// Primary background
    Primary,
    /// Secondary background
    Secondary,
    /// Tertiary background
    Tertiary,
}

/// Text emphasis levels
#[derive(Clone, Copy, Debug)]
pub enum TextEmphasis {
    /// Primary text
    Primary,
    /// Secondary text
    Secondary,
}
