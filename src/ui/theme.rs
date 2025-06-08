//! Comprehensive design system for consistent UI styling

use eframe::egui;

/// Text emphasis levels for semantic meaning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEmphasis {
    /// Primary text (high contrast)
    Primary,
    /// Secondary text (medium contrast)
    Secondary,
    /// Tertiary text (low contrast)
    Tertiary,
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

/// Complete design system tokens
#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub colors: ColorTokens,
    pub typography: TypographyTokens,
    pub spacing: SpacingTokens,
    pub shadows: ShadowTokens,
    pub radius: RadiusTokens,
    pub animations: AnimationTokens,
}

/// Comprehensive color token system
#[derive(Debug, Clone)]
pub struct ColorTokens {
    // Brand colors
    pub primary: egui::Color32,
    pub primary_hover: egui::Color32,
    pub primary_disabled: egui::Color32,
    pub secondary: egui::Color32,
    pub secondary_hover: egui::Color32,

    // Surface colors
    pub surface: egui::Color32,
    pub surface_variant: egui::Color32,
    pub surface_container: egui::Color32,
    pub surface_container_high: egui::Color32,

    // Content colors
    pub on_surface: egui::Color32,
    pub on_surface_variant: egui::Color32,
    pub outline: egui::Color32,
    pub outline_variant: egui::Color32,

    // State colors
    pub success: egui::Color32,
    pub success_container: egui::Color32,
    pub warning: egui::Color32,
    pub warning_container: egui::Color32,
    pub error: egui::Color32,
    pub error_container: egui::Color32,
}

/// Typography system with semantic names
#[derive(Debug, Clone)]
pub struct TypographyTokens {
    pub display_large: FontToken,
    pub display_medium: FontToken,
    pub display_small: FontToken,
    pub headline_large: FontToken,
    pub headline_medium: FontToken,
    pub headline_small: FontToken,
    pub title_large: FontToken,
    pub title_medium: FontToken,
    pub title_small: FontToken,
    pub body_large: FontToken,
    pub body_medium: FontToken,
    pub body_small: FontToken,
    pub label_large: FontToken,
    pub label_medium: FontToken,
    pub label_small: FontToken,
}

#[derive(Debug, Clone)]
pub struct FontToken {
    pub size: f32,
    pub line_height: f32,
    pub weight: egui::FontFamily,
}

/// Spacing system based on 4px grid
#[derive(Debug, Clone)]
pub struct SpacingTokens {
    pub xs: f32,   // 4px
    pub sm: f32,   // 8px
    pub md: f32,   // 12px
    pub lg: f32,   // 16px
    pub xl: f32,   // 24px
    pub xxl: f32,  // 32px
    pub xxxl: f32, // 48px
}

/// Shadow system for elevation with Material Design levels
#[derive(Debug, Clone)]
pub struct ShadowTokens {
    pub none: egui::epaint::Shadow,
    pub sm: egui::epaint::Shadow,
    pub md: egui::epaint::Shadow,
    pub lg: egui::epaint::Shadow,
    pub xl: egui::epaint::Shadow,
}

/// Material Design elevation levels
#[derive(Debug, Clone, Copy)]
pub enum Elevation {
    /// No elevation (0dp)
    None,
    /// Level 1 elevation (1dp) - Cards at rest
    Level1,
    /// Level 2 elevation (3dp) - Raised buttons, search bars
    Level2,
    /// Level 3 elevation (6dp) - Snackbars
    Level3,
    /// Level 4 elevation (8dp) - Standard menus, cards on hover
    Level4,
    /// Level 5 elevation (12dp) - App bars, elevated buttons
    Level5,
}

impl Elevation {
    /// Gets the appropriate shadow for this elevation level
    pub fn shadow(self, dark_mode: bool) -> egui::epaint::Shadow {
        let base_alpha = if dark_mode { 80 } else { 25 };

        match self {
            Self::None => egui::epaint::Shadow::NONE,
            Self::Level1 => egui::epaint::Shadow {
                offset: [0, 1],
                blur: 3,
                spread: 0,
                color: egui::Color32::from_black_alpha(base_alpha),
            },
            Self::Level2 => egui::epaint::Shadow {
                offset: [0, 2],
                blur: 6,
                spread: 0,
                color: egui::Color32::from_black_alpha(base_alpha + 5),
            },
            Self::Level3 => egui::epaint::Shadow {
                offset: [0, 4],
                blur: 12,
                spread: 0,
                color: egui::Color32::from_black_alpha(base_alpha + 10),
            },
            Self::Level4 => egui::epaint::Shadow {
                offset: [0, 6],
                blur: 18,
                spread: 0,
                color: egui::Color32::from_black_alpha(base_alpha + 15),
            },
            Self::Level5 => egui::epaint::Shadow {
                offset: [0, 8],
                blur: 24,
                spread: 0,
                color: egui::Color32::from_black_alpha(base_alpha + 20),
            },
        }
    }
}

/// Border radius system
#[derive(Debug, Clone)]
pub struct RadiusTokens {
    pub none: egui::CornerRadius,
    pub xs: egui::CornerRadius,
    pub sm: egui::CornerRadius,
    pub md: egui::CornerRadius,
    pub lg: egui::CornerRadius,
    pub xl: egui::CornerRadius,
    pub full: egui::CornerRadius,
}

/// Animation timing and easing tokens
#[derive(Debug, Clone)]
pub struct AnimationTokens {
    pub duration_fast: f32,   // 150ms
    pub duration_normal: f32, // 300ms
    pub duration_slow: f32,   // 500ms
    pub easing_out: f32,      // Ease-out curve strength
    pub easing_in_out: f32,   // Ease-in-out curve strength
}

/// Core theme constants and utilities
#[derive(Debug)]
pub struct Theme;

impl Theme {
    // Layout constants
    pub const TOP_BAR_HEIGHT: f32 = 48.0;
    pub const SIDEBAR_MIN_WIDTH: f32 = 200.0;
    pub const SIDEBAR_MAX_WIDTH: f32 = 600.0;
    pub const SIDEBAR_DEFAULT_WIDTH: f32 = 350.0;

    // Legacy spacing constants (use DesignTokens.spacing for new code)
    pub const SPACING_XS: f32 = 4.0;
    pub const SPACING_SM: f32 = 8.0;
    pub const SPACING_MD: f32 = 12.0;
    pub const SPACING_LG: f32 = 16.0;
    pub const SPACING_XL: f32 = 24.0;
    pub const SPACING_XXL: f32 = 32.0;

    // Legacy radius constants (use DesignTokens.radius for new code)
    pub const RADIUS_SM: f32 = 4.0;
    pub const RADIUS_MD: f32 = 6.0;
    pub const RADIUS_LG: f32 = 8.0;
    pub const RADIUS_XL: f32 = 12.0;

    // Button constants
    /// Primary button height
    pub const PRIMARY_BUTTON_HEIGHT: f32 = 40.0;
    /// Primary button minimum width
    pub const PRIMARY_BUTTON_MIN_WIDTH: f32 = 160.0;
    /// Action bar padding
    pub const ACTION_BAR_PADDING: f32 = 16.0; // Increased from 12.0

    // Legacy color constants for compatibility
    pub const SUCCESS: egui::Color32 = egui::Color32::from_rgb(34, 197, 94);
    pub const WARNING: egui::Color32 = egui::Color32::from_rgb(251, 191, 36);
    pub const ERROR: egui::Color32 = egui::Color32::from_rgb(239, 68, 68);

    // UI layout constants for tree component compatibility
    pub const ROW_HEIGHT: f32 = 20.0;
    pub const INDENT_SIZE: f32 = 16.0;
    pub const ICON_SIZE: f32 = 14.0;

    // Icon constants (temporary - will be replaced with SVGs)
    /// Chevron icons
    pub const CHEVRON_RIGHT: &'static str = "▶";
    pub const CHEVRON_DOWN: &'static str = "▼";

    /// File/folder icons
    pub const ICON_FOLDER_CLOSED: &'static str = "📁";
    pub const ICON_FOLDER_OPEN: &'static str = "📂";
    pub const ICON_FILE: &'static str = "📄";
    pub const ICON_CODE: &'static str = "📝";
    pub const ICON_CONFIG: &'static str = "⚙️";
    pub const ICON_DOC: &'static str = "📚";

    /// Creates design tokens for the given theme mode
    pub fn design_tokens(dark_mode: bool) -> DesignTokens {
        DesignTokens {
            colors: Self::color_tokens(dark_mode),
            typography: Self::typography_tokens(),
            spacing: Self::spacing_tokens(),
            shadows: Self::shadow_tokens(dark_mode),
            radius: Self::radius_tokens(),
            animations: Self::animation_tokens(),
        }
    }

    /// Creates color tokens for the theme
    fn color_tokens(dark_mode: bool) -> ColorTokens {
        if dark_mode {
            ColorTokens {
                // Brand colors - warmer blue palette
                primary: egui::Color32::from_rgb(59, 130, 246), // Blue-500
                primary_hover: egui::Color32::from_rgb(37, 99, 235), // Blue-600
                primary_disabled: egui::Color32::from_rgb(59, 130, 246).gamma_multiply(0.4),
                secondary: egui::Color32::from_rgb(156, 163, 175), // Gray-400
                secondary_hover: egui::Color32::from_rgb(209, 213, 219), // Gray-300

                // Surface colors
                surface: egui::Color32::from_rgb(24, 24, 27), // Zinc-900
                surface_variant: egui::Color32::from_rgb(39, 39, 42), // Zinc-800
                surface_container: egui::Color32::from_rgb(63, 63, 70), // Zinc-700
                surface_container_high: egui::Color32::from_rgb(82, 82, 91), // Zinc-600

                // Content colors
                on_surface: egui::Color32::from_rgb(250, 250, 250), // Zinc-50
                on_surface_variant: egui::Color32::from_rgb(161, 161, 170), // Zinc-400
                outline: egui::Color32::from_rgb(82, 82, 91),       // Zinc-600
                outline_variant: egui::Color32::from_rgb(63, 63, 70), // Zinc-700

                // State colors
                success: egui::Color32::from_rgb(34, 197, 94), // Green-500
                success_container: egui::Color32::from_rgb(21, 128, 61), // Green-700
                warning: egui::Color32::from_rgb(251, 191, 36), // Amber-400
                warning_container: egui::Color32::from_rgb(180, 83, 9), // Amber-700
                error: egui::Color32::from_rgb(239, 68, 68),   // Red-500
                error_container: egui::Color32::from_rgb(153, 27, 27), // Red-800
            }
        } else {
            ColorTokens {
                // Brand colors
                primary: egui::Color32::from_rgb(37, 99, 235), // Blue-600
                primary_hover: egui::Color32::from_rgb(29, 78, 216), // Blue-700
                primary_disabled: egui::Color32::from_rgb(37, 99, 235).gamma_multiply(0.4),
                secondary: egui::Color32::from_rgb(107, 114, 128), // Gray-500
                secondary_hover: egui::Color32::from_rgb(75, 85, 99), // Gray-600

                // Surface colors
                surface: egui::Color32::from_rgb(255, 255, 255), // White
                surface_variant: egui::Color32::from_rgb(249, 250, 251), // Gray-50
                surface_container: egui::Color32::from_rgb(243, 244, 246), // Gray-100
                surface_container_high: egui::Color32::from_rgb(229, 231, 235), // Gray-200

                // Content colors
                on_surface: egui::Color32::from_rgb(17, 24, 39), // Gray-900
                on_surface_variant: egui::Color32::from_rgb(107, 114, 128), // Gray-500
                outline: egui::Color32::from_rgb(209, 213, 219), // Gray-300
                outline_variant: egui::Color32::from_rgb(229, 231, 235), // Gray-200

                // State colors
                success: egui::Color32::from_rgb(22, 163, 74), // Green-600
                success_container: egui::Color32::from_rgb(220, 252, 231), // Green-100
                warning: egui::Color32::from_rgb(217, 119, 6), // Amber-600
                warning_container: egui::Color32::from_rgb(254, 243, 199), // Amber-100
                error: egui::Color32::from_rgb(220, 38, 38),   // Red-600
                error_container: egui::Color32::from_rgb(254, 226, 226), // Red-100
            }
        }
    }

    /// Creates typography tokens
    const fn typography_tokens() -> TypographyTokens {
        TypographyTokens {
            display_large: FontToken {
                size: 57.0,
                line_height: 64.0,
                weight: egui::FontFamily::Proportional,
            },
            display_medium: FontToken {
                size: 45.0,
                line_height: 52.0,
                weight: egui::FontFamily::Proportional,
            },
            display_small: FontToken {
                size: 36.0,
                line_height: 44.0,
                weight: egui::FontFamily::Proportional,
            },
            headline_large: FontToken {
                size: 32.0,
                line_height: 40.0,
                weight: egui::FontFamily::Proportional,
            },
            headline_medium: FontToken {
                size: 28.0,
                line_height: 36.0,
                weight: egui::FontFamily::Proportional,
            },
            headline_small: FontToken {
                size: 24.0,
                line_height: 32.0,
                weight: egui::FontFamily::Proportional,
            },
            title_large: FontToken {
                size: 22.0,
                line_height: 28.0,
                weight: egui::FontFamily::Proportional,
            },
            title_medium: FontToken {
                size: 16.0,
                line_height: 24.0,
                weight: egui::FontFamily::Proportional,
            },
            title_small: FontToken {
                size: 14.0,
                line_height: 20.0,
                weight: egui::FontFamily::Proportional,
            },
            body_large: FontToken {
                size: 16.0,
                line_height: 24.0,
                weight: egui::FontFamily::Proportional,
            },
            body_medium: FontToken {
                size: 14.0,
                line_height: 20.0,
                weight: egui::FontFamily::Proportional,
            },
            body_small: FontToken {
                size: 12.0,
                line_height: 16.0,
                weight: egui::FontFamily::Proportional,
            },
            label_large: FontToken {
                size: 14.0,
                line_height: 20.0,
                weight: egui::FontFamily::Proportional,
            },
            label_medium: FontToken {
                size: 12.0,
                line_height: 16.0,
                weight: egui::FontFamily::Proportional,
            },
            label_small: FontToken {
                size: 11.0,
                line_height: 16.0,
                weight: egui::FontFamily::Proportional,
            },
        }
    }

    /// Creates spacing tokens
    const fn spacing_tokens() -> SpacingTokens {
        SpacingTokens {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            xxl: 32.0,
            xxxl: 48.0,
        }
    }

    /// Creates shadow tokens
    const fn shadow_tokens(dark_mode: bool) -> ShadowTokens {
        let shadow_color = if dark_mode {
            egui::Color32::from_black_alpha(60)
        } else {
            egui::Color32::from_black_alpha(25)
        };

        ShadowTokens {
            none: egui::epaint::Shadow::NONE,
            sm: egui::epaint::Shadow {
                offset: [0, 1],
                blur: 2,
                spread: 0,
                color: shadow_color,
            },
            md: egui::epaint::Shadow {
                offset: [0, 4],
                blur: 6,
                spread: 0, // Negative spread not supported, using 0
                color: shadow_color,
            },
            lg: egui::epaint::Shadow {
                offset: [0, 10],
                blur: 15,
                spread: 0, // Negative spread not supported, using 0
                color: shadow_color,
            },
            xl: egui::epaint::Shadow {
                offset: [0, 20],
                blur: 25,
                spread: 0, // Negative spread not supported, using 0
                color: shadow_color,
            },
        }
    }

    /// Creates radius tokens
    const fn radius_tokens() -> RadiusTokens {
        RadiusTokens {
            none: egui::CornerRadius::ZERO,
            xs: egui::CornerRadius::same(2),
            sm: egui::CornerRadius::same(4),
            md: egui::CornerRadius::same(6),
            lg: egui::CornerRadius::same(8),
            xl: egui::CornerRadius::same(12),
            full: egui::CornerRadius::same(255), // Max value for u8
        }
    }

    /// Creates animation tokens
    const fn animation_tokens() -> AnimationTokens {
        AnimationTokens {
            duration_fast: 0.15,  // 150ms
            duration_normal: 0.3, // 300ms
            duration_slow: 0.5,   // 500ms
            easing_out: 0.25,     // Cubic bezier control point
            easing_in_out: 0.5,   // Cubic bezier control point
        }
    }

    /// Apply enhanced theme with design tokens
    #[allow(clippy::cast_possible_truncation)]
    pub fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
        let tokens = Self::design_tokens(dark_mode);

        let mut style = egui::Style {
            text_styles: [
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(
                        tokens.typography.headline_small.size,
                        tokens.typography.headline_small.weight.clone(),
                    ),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(
                        tokens.typography.body_medium.size,
                        tokens.typography.body_medium.weight.clone(),
                    ),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(
                        tokens.typography.label_large.size,
                        tokens.typography.label_large.weight.clone(),
                    ),
                ),
                (
                    egui::TextStyle::Small,
                    egui::FontId::new(
                        tokens.typography.label_small.size,
                        tokens.typography.label_small.weight.clone(),
                    ),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(
                        tokens.typography.body_small.size,
                        egui::FontFamily::Monospace,
                    ),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
            ..Default::default()
        };

        // Configure spacing with design tokens
        style.spacing.item_spacing = egui::vec2(tokens.spacing.sm, tokens.spacing.sm);
        style.spacing.button_padding = egui::vec2(tokens.spacing.lg, tokens.spacing.sm);
        style.spacing.menu_margin = egui::Margin::same(tokens.spacing.sm as i8);
        style.spacing.indent = tokens.spacing.xl;

        // Configure visuals with design tokens
        let mut visuals = if dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        // Apply color tokens
        visuals.panel_fill = tokens.colors.surface_variant;
        visuals.window_fill = tokens.colors.surface;
        visuals.window_stroke = egui::Stroke::new(1.0, tokens.colors.outline_variant);

        // Widget colors
        visuals.widgets.noninteractive.bg_fill = tokens.colors.surface;
        visuals.widgets.noninteractive.weak_bg_fill = tokens.colors.surface_variant;
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, tokens.colors.on_surface);

        visuals.widgets.inactive.bg_fill = tokens.colors.surface_container;
        visuals.widgets.inactive.weak_bg_fill = tokens.colors.surface_variant;
        visuals.widgets.inactive.fg_stroke =
            egui::Stroke::new(1.0, tokens.colors.on_surface_variant);

        visuals.widgets.hovered.bg_fill = tokens.colors.surface_container_high;
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, tokens.colors.on_surface);

        visuals.widgets.active.bg_fill = tokens.colors.primary;
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

        visuals.selection.bg_fill = tokens.colors.primary.gamma_multiply(0.3);
        visuals.selection.stroke = egui::Stroke::new(1.0, tokens.colors.primary);
        visuals.hyperlink_color = tokens.colors.primary;

        // State colors
        visuals.warn_fg_color = tokens.colors.warning;
        visuals.error_fg_color = tokens.colors.error;

        // Apply radius tokens
        visuals.window_corner_radius = tokens.radius.lg;
        visuals.menu_corner_radius = tokens.radius.md;

        ctx.set_visuals(visuals);
        ctx.set_style(style);
    }

    /// Legacy compatibility methods
    pub fn text_color(dark_mode: bool, emphasis: TextEmphasis) -> egui::Color32 {
        let tokens = Self::design_tokens(dark_mode);
        match emphasis {
            TextEmphasis::Primary => tokens.colors.on_surface,
            TextEmphasis::Secondary => tokens.colors.on_surface_variant,
            TextEmphasis::Tertiary => tokens.colors.on_surface_variant.gamma_multiply(0.6),
        }
    }

    pub fn accent_color(dark_mode: bool) -> egui::Color32 {
        let tokens = Self::design_tokens(dark_mode);
        tokens.colors.primary
    }

    pub fn bg_color(dark_mode: bool, level: BgLevel) -> egui::Color32 {
        let tokens = Self::design_tokens(dark_mode);
        match level {
            BgLevel::Primary => tokens.colors.surface,
            BgLevel::Secondary => tokens.colors.surface_variant,
            BgLevel::Tertiary => tokens.colors.surface_container,
        }
    }
}
