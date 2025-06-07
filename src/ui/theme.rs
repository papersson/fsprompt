use egui::{Color32, FontFamily, Stroke, Style, Visuals};

pub struct Theme;

impl Theme {
    pub const DARK_BG_PRIMARY: Color32 = Color32::from_rgb(26, 26, 26);
    pub const DARK_BG_SECONDARY: Color32 = Color32::from_rgb(35, 35, 35);
    pub const DARK_BG_TERTIARY: Color32 = Color32::from_rgb(45, 45, 45);
    pub const DARK_BORDER: Color32 = Color32::from_rgb(58, 58, 58);
    pub const DARK_TEXT_PRIMARY: Color32 = Color32::from_rgb(236, 236, 236);
    pub const DARK_TEXT_SECONDARY: Color32 = Color32::from_rgb(155, 155, 155);
    pub const DARK_ACCENT: Color32 = Color32::from_rgb(30, 144, 255);

    pub const LIGHT_BG_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255);
    pub const LIGHT_BG_SECONDARY: Color32 = Color32::from_rgb(247, 247, 247);
    pub const LIGHT_BG_TERTIARY: Color32 = Color32::from_rgb(235, 235, 235);
    pub const LIGHT_BORDER: Color32 = Color32::from_rgb(227, 227, 227);
    pub const LIGHT_TEXT_PRIMARY: Color32 = Color32::from_rgb(32, 32, 32);
    pub const LIGHT_TEXT_SECONDARY: Color32 = Color32::from_rgb(110, 110, 110);
    pub const LIGHT_ACCENT: Color32 = Color32::from_rgb(0, 102, 204);

    pub const SUCCESS: Color32 = Color32::from_rgb(16, 185, 129);
    pub const WARNING: Color32 = Color32::from_rgb(245, 158, 11);
    pub const ERROR: Color32 = Color32::from_rgb(239, 68, 68);
    pub const INFO: Color32 = Color32::from_rgb(59, 130, 246);

    pub const SPACING_XS: f32 = 4.0;
    pub const SPACING_SM: f32 = 8.0;
    pub const SPACING_MD: f32 = 16.0;
    pub const SPACING_LG: f32 = 24.0;
    pub const SPACING_XL: f32 = 32.0;

    pub const RADIUS_SM: f32 = 4.0;
    pub const RADIUS_MD: f32 = 6.0;
    pub const RADIUS_LG: f32 = 8.0;

    pub const TOP_BAR_HEIGHT: f32 = 48.0;
    pub const BOTTOM_BAR_HEIGHT: f32 = 56.0;
    pub const SIDEBAR_DEFAULT_WIDTH: f32 = 280.0;
    pub const SIDEBAR_MIN_WIDTH: f32 = 200.0;
    pub const SIDEBAR_MAX_WIDTH: f32 = 400.0;

    pub const BUTTON_HEIGHT: f32 = 36.0;
    pub const BUTTON_PADDING_H: f32 = 16.0;
    pub const BUTTON_PADDING_V: f32 = 8.0;

    pub const ROW_HEIGHT: f32 = 28.0;
    pub const INDENT_SIZE: f32 = 20.0;
    pub const ICON_SIZE: f32 = 16.0;
    pub const ICON_SPACING: f32 = 4.0;

    pub fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
        let mut style = Style::default();

        // Configure text styles
        style.text_styles = [
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
        .collect();

        // Configure spacing
        style.spacing.item_spacing = egui::vec2(Self::SPACING_SM, Self::SPACING_SM);
        style.spacing.button_padding = egui::vec2(Self::BUTTON_PADDING_H, Self::BUTTON_PADDING_V);
        style.spacing.menu_margin = egui::Margin::symmetric(Self::SPACING_SM as i8, Self::SPACING_SM as i8);
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

    pub fn bg_color(dark_mode: bool, level: BgLevel) -> Color32 {
        match (dark_mode, level) {
            (true, BgLevel::Primary) => Self::DARK_BG_PRIMARY,
            (true, BgLevel::Secondary) => Self::DARK_BG_SECONDARY,
            (true, BgLevel::Tertiary) => Self::DARK_BG_TERTIARY,
            (false, BgLevel::Primary) => Self::LIGHT_BG_PRIMARY,
            (false, BgLevel::Secondary) => Self::LIGHT_BG_SECONDARY,
            (false, BgLevel::Tertiary) => Self::LIGHT_BG_TERTIARY,
        }
    }

    pub fn text_color(dark_mode: bool, emphasis: TextEmphasis) -> Color32 {
        match (dark_mode, emphasis) {
            (true, TextEmphasis::Primary) => Self::DARK_TEXT_PRIMARY,
            (true, TextEmphasis::Secondary) => Self::DARK_TEXT_SECONDARY,
            (false, TextEmphasis::Primary) => Self::LIGHT_TEXT_PRIMARY,
            (false, TextEmphasis::Secondary) => Self::LIGHT_TEXT_SECONDARY,
        }
    }

    pub fn accent_color(dark_mode: bool) -> Color32 {
        if dark_mode {
            Self::DARK_ACCENT
        } else {
            Self::LIGHT_ACCENT
        }
    }

    pub fn border_color(dark_mode: bool) -> Color32 {
        if dark_mode {
            Self::DARK_BORDER
        } else {
            Self::LIGHT_BORDER
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BgLevel {
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Clone, Copy, Debug)]
pub enum TextEmphasis {
    Primary,
    Secondary,
}
