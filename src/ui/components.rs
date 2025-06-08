//! Enhanced UI components with consistent styling

use crate::ui::{
    animations::SpinnerAnimation,
    icons::{IconManager, IconSize, IconType},
    theme::{DesignTokens, Elevation, Theme},
};
use eframe::egui;
use std::collections::HashMap;

/// Button variant types for consistent styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Primary filled button (main actions)
    Primary,
    /// Secondary outlined button (secondary actions)
    Secondary,
    /// Ghost button (subtle actions)
    Ghost,
    /// Danger button (destructive actions)
    Danger,
}

/// Button size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    /// Small button (28px height)
    Small,
    /// Medium button (36px height)
    Medium,
    /// Large button (44px height)
    Large,
}

impl ButtonSize {
    /// Returns the height of the button size
    pub const fn height(self) -> f32 {
        match self {
            Self::Small => 20.0,
            Self::Medium => 32.0,
            Self::Large => 40.0,
        }
    }

    /// Returns the padding for the button size
    pub const fn padding(self) -> f32 {
        match self {
            Self::Small => 4.0,
            Self::Medium => 8.0,
            Self::Large => 12.0,
        }
    }

    /// Returns the icon size for the button size
    pub const fn icon_size(self) -> IconSize {
        match self {
            Self::Small => IconSize::Small,
            Self::Medium => IconSize::Medium,
            Self::Large => IconSize::Large,
        }
    }
}

/// Simplified animation manager for spinners only
#[derive(Debug)]
pub struct AnimatedButtonManager {
    loading_spinners: HashMap<egui::Id, SpinnerAnimation>,
}

impl AnimatedButtonManager {
    /// Creates a new animated button manager
    pub fn new() -> Self {
        Self {
            loading_spinners: HashMap::new(),
        }
    }

    /// Gets or creates a loading spinner for a button
    fn get_or_create_spinner(&mut self, id: egui::Id) -> &mut SpinnerAnimation {
        self.loading_spinners.entry(id).or_insert_with(|| {
            SpinnerAnimation::new(2.0) // 2 rotations per second
        })
    }

    /// Cleans up old animations (spinners don't complete, so this is minimal)
    pub fn cleanup(&mut self) {
        // Keep all spinners as they don't complete
    }
}

impl Default for AnimatedButtonManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced button builder with icon support
#[derive(Debug)]
#[must_use]
pub struct Button {
    text: String,
    variant: ButtonVariant,
    size: ButtonSize,
    icon: Option<IconType>,
    icon_position: IconPosition,
    loading: bool,
    disabled: bool,
    min_width: Option<f32>,
    tooltip: Option<String>,
    id: Option<egui::Id>,
}

/// Position of the icon relative to text
#[derive(Debug, Clone, Copy)]
pub enum IconPosition {
    /// Icon on the left side of text
    Left,
    /// Icon on the right side of text
    Right,
    /// Icon only, no text
    Only,
}

impl Button {
    /// Creates a new button with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Medium,
            icon: None,
            icon_position: IconPosition::Left,
            loading: false,
            disabled: false,
            min_width: None,
            tooltip: None,
            id: None,
        }
    }

    /// Creates a primary button
    pub fn primary(text: impl Into<String>) -> Self {
        Self::new(text).variant(ButtonVariant::Primary)
    }

    /// Creates a ghost button
    pub fn ghost(text: impl Into<String>) -> Self {
        Self::new(text).variant(ButtonVariant::Ghost)
    }

    /// Creates an icon-only button
    pub const fn icon_only(icon: IconType) -> Self {
        Self {
            text: String::new(),
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Medium,
            icon: Some(icon),
            icon_position: IconPosition::Only,
            loading: false,
            disabled: false,
            min_width: None,
            tooltip: None,
            id: None,
        }
    }

    /// Sets the button variant
    pub const fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the button size
    pub const fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Adds an icon to the button
    pub const fn icon(mut self, icon: IconType) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets icon position
    pub const fn icon_position(mut self, position: IconPosition) -> Self {
        self.icon_position = position;
        self
    }

    /// Sets loading state
    pub const fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets disabled state
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets minimum width
    pub const fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Sets tooltip
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Sets a custom ID for animation tracking
    pub const fn id(mut self, id: egui::Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Shows the button and returns response
    pub fn show(self, ui: &mut egui::Ui, icon_manager: &mut IconManager) -> egui::Response {
        self.show_animated(ui, icon_manager, None)
    }

    /// Shows the button with animation support
    pub fn show_animated(
        self,
        ui: &mut egui::Ui,
        icon_manager: &mut IconManager,
        mut animation_manager: Option<&mut AnimatedButtonManager>,
    ) -> egui::Response {
        let tokens = Theme::design_tokens(ui.visuals().dark_mode);
        let enabled = !self.disabled && !self.loading;

        // Generate button ID for animation tracking
        let button_id = self.id.unwrap_or_else(|| ui.next_auto_id());

        // Calculate button dimensions
        let button_height = self.size.height();
        let button_padding = self.size.padding();
        let min_width = self.min_width.unwrap_or({
            if matches!(self.icon_position, IconPosition::Only) {
                button_height
            } else {
                80.0
            }
        });

        // Get colors based on variant and state
        let (_base_bg_color, text_color, _border_color) =
            self.get_colors(&tokens, enabled, ui.visuals().dark_mode);

        // Create button response with hover cursor
        let desired_size = egui::vec2(min_width, button_height);
        let (rect, mut response) = ui.allocate_at_least(desired_size, egui::Sense::click());

        // Add hover cursor for clickable feedback
        if enabled {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }

        // Handle disabled state
        if !enabled {
            response = response.on_disabled_hover_text(if self.loading {
                "Loading..."
            } else {
                "Disabled"
            });
        }

        // Add tooltip
        if let Some(tooltip_text) = &self.tooltip {
            response = response.on_hover_text(tooltip_text);
        }

        // Use egui's built-in interaction styling for consistent behavior
        let _visuals = ui.style().interact_selectable(&response, false);

        // Simple hover detection - no complex layer checking needed
        let is_hovered = response.hovered();

        // Enhanced visual feedback with animation and elevation
        let press_animation = ui.ctx().animate_value_with_time(
            button_id.with("press"),
            if response.is_pointer_button_down_on() {
                1.0
            } else {
                0.0
            },
            0.1,
        );

        let hover_animation = ui.ctx().animate_value_with_time(
            button_id.with("hover"),
            if is_hovered && enabled { 1.0 } else { 0.0 },
            tokens.animations.duration_fast,
        );

        // Calculate visual rect with smooth press animation
        let press_shrink = press_animation * 1.0;
        let visual_rect = rect.shrink(press_shrink);

        // Enhanced hover effect with smooth scaling
        let scale_factor = 1.0 + (hover_animation * 0.02); // Subtle 2% scale increase
        let scaled_rect = if scale_factor != 1.0 {
            let center = visual_rect.center();
            let scaled_size = visual_rect.size() * scale_factor;
            egui::Rect::from_center_size(center, scaled_size)
        } else {
            visual_rect
        };

        // Determine colors with smooth animation
        let target_bg_color = if !enabled {
            tokens.colors.surface_variant.gamma_multiply(0.6)
        } else {
            match self.variant {
                ButtonVariant::Primary => {
                    let base = tokens.colors.primary;
                    let hover = tokens.colors.primary_hover;
                    egui::Color32::from_rgb(
                        (base.r() as f32 + (hover.r() as f32 - base.r() as f32) * hover_animation)
                            as u8,
                        (base.g() as f32 + (hover.g() as f32 - base.g() as f32) * hover_animation)
                            as u8,
                        (base.b() as f32 + (hover.b() as f32 - base.b() as f32) * hover_animation)
                            as u8,
                    )
                }
                ButtonVariant::Secondary => {
                    let base = tokens.colors.surface_container;
                    let hover = tokens.colors.surface_container_high;
                    egui::Color32::from_rgb(
                        (base.r() as f32 + (hover.r() as f32 - base.r() as f32) * hover_animation)
                            as u8,
                        (base.g() as f32 + (hover.g() as f32 - base.g() as f32) * hover_animation)
                            as u8,
                        (base.b() as f32 + (hover.b() as f32 - base.b() as f32) * hover_animation)
                            as u8,
                    )
                }
                ButtonVariant::Danger => {
                    let base = tokens.colors.error;
                    let hover = tokens.colors.error_container;
                    egui::Color32::from_rgb(
                        (base.r() as f32 + (hover.r() as f32 - base.r() as f32) * hover_animation)
                            as u8,
                        (base.g() as f32 + (hover.g() as f32 - base.g() as f32) * hover_animation)
                            as u8,
                        (base.b() as f32 + (hover.b() as f32 - base.b() as f32) * hover_animation)
                            as u8,
                    )
                }
                ButtonVariant::Ghost => {
                    let alpha = (hover_animation * 40.0) as u8;
                    egui::Color32::from_gray(128).gamma_multiply(alpha as f32 / 255.0)
                }
            }
        };

        // Material Design elevation and shadows
        let elevation = if !enabled {
            Elevation::None
        } else if response.is_pointer_button_down_on() {
            Elevation::Level2
        } else if is_hovered {
            match self.variant {
                ButtonVariant::Primary => Elevation::Level3,
                _ => Elevation::Level2,
            }
        } else {
            match self.variant {
                ButtonVariant::Primary => Elevation::Level2,
                _ => Elevation::Level1,
            }
        };

        // Draw shadow with elevation
        let shadow = elevation.shadow(ui.visuals().dark_mode);
        if shadow != egui::epaint::Shadow::NONE {
            let shadow_rect =
                scaled_rect.translate([shadow.offset[0] as f32, shadow.offset[1] as f32].into());
            ui.painter()
                .rect_filled(shadow_rect, tokens.radius.md, shadow.color);
        }

        // Draw button background
        ui.painter()
            .rect_filled(scaled_rect, tokens.radius.md, target_bg_color);

        // Draw button content using painter API
        let content_rect = scaled_rect.shrink(button_padding);

        // Handle loading spinner animation
        if self.loading {
            if let Some(anim_manager) = &mut animation_manager {
                let spinner = anim_manager.get_or_create_spinner(button_id);
                let spinner_center = content_rect.center();
                let spinner_radius = self.size.icon_size().size() / 2.0;

                // Create a temporary UI just for the spinner (it needs UI context)
                let mut spinner_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(content_rect)
                        .sense(egui::Sense::hover()),
                );

                spinner.draw_arc(
                    &mut spinner_ui,
                    spinner_center,
                    spinner_radius,
                    text_color,
                    2.0,
                );

                // Request repaint for spinner animation
                ui.ctx().request_repaint();
            } else {
                // Fallback to static loading indicator using painter
                let spinner_pos = content_rect.center();
                let font_id =
                    egui::FontId::new(self.size.icon_size().size(), egui::FontFamily::Proportional);
                let galley = ui
                    .painter()
                    .layout_no_wrap("⏳".to_string(), font_id, text_color);
                let text_pos = spinner_pos - galley.size() / 2.0;
                ui.painter()
                    .add(egui::epaint::TextShape::new(text_pos, galley, text_color));
            }

            // Draw loading text if present
            if !self.text.is_empty() && !matches!(self.icon_position, IconPosition::Only) {
                let text_font = egui::FontId::new(
                    ui.style().text_styles[&egui::TextStyle::Button].size,
                    egui::FontFamily::Proportional,
                );
                let text_galley =
                    ui.painter()
                        .layout_no_wrap(self.text.clone(), text_font, text_color);
                let text_pos = content_rect.center()
                    + egui::vec2(self.size.icon_size().size() / 2.0 + 4.0, 0.0)
                    - egui::vec2(text_galley.size().x / 2.0, text_galley.size().y / 2.0);
                ui.painter().add(egui::epaint::TextShape::new(
                    text_pos,
                    text_galley,
                    text_color,
                ));
            }
        } else {
            // Calculate text color with smooth transitions
            let final_text_color = match self.variant {
                ButtonVariant::Primary => {
                    // Always white on primary buttons for contrast
                    egui::Color32::WHITE
                }
                _ => {
                    // Animate text brightness on hover for other button types
                    let base_color = text_color;
                    if enabled && hover_animation > 0.0 {
                        let brightness_multiplier = 1.0 + (hover_animation * 0.2);
                        egui::Color32::from_rgb(
                            (base_color.r() as f32 * brightness_multiplier).min(255.0) as u8,
                            (base_color.g() as f32 * brightness_multiplier).min(255.0) as u8,
                            (base_color.b() as f32 * brightness_multiplier).min(255.0) as u8,
                        )
                    } else {
                        base_color
                    }
                }
            };

            self.draw_content_with_painter(
                ui,
                content_rect,
                icon_manager,
                final_text_color,
                enabled,
            );
        }

        response
    }

    /// Gets colors for the button based on variant and state
    fn get_colors(
        &self,
        tokens: &DesignTokens,
        enabled: bool,
        _dark_mode: bool,
    ) -> (egui::Color32, egui::Color32, egui::Color32) {
        if !enabled {
            return (
                tokens.colors.surface_variant,
                tokens.colors.on_surface_variant.gamma_multiply(0.4),
                tokens.colors.outline_variant,
            );
        }

        match self.variant {
            ButtonVariant::Primary => (
                tokens.colors.primary,
                egui::Color32::WHITE,
                tokens.colors.primary,
            ),
            ButtonVariant::Secondary => (
                tokens.colors.surface_container,
                tokens.colors.on_surface,
                tokens.colors.outline,
            ),
            ButtonVariant::Ghost => (
                egui::Color32::TRANSPARENT,
                tokens.colors.on_surface,
                egui::Color32::TRANSPARENT,
            ),
            ButtonVariant::Danger => (
                tokens.colors.error,
                egui::Color32::WHITE,
                tokens.colors.error,
            ),
        }
    }

    /// Draws button content (icon + text) using painter API
    fn draw_content_with_painter(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        icon_manager: &mut IconManager,
        text_color: egui::Color32,
        _enabled: bool,
    ) {
        let icon_size = self.size.icon_size();
        let painter = ui.painter();

        // Get text font
        let text_font = egui::FontId::new(
            ui.style().text_styles[&egui::TextStyle::Button].size,
            egui::FontFamily::Proportional,
        );

        match self.icon_position {
            IconPosition::Only => {
                if let Some(icon) = self.icon {
                    // Draw icon centered
                    icon_manager.draw_icon_at(painter, rect.center(), icon, icon_size, text_color);
                }
            }
            IconPosition::Left => {
                let mut x_offset = rect.left();

                // Draw icon on the left
                if let Some(icon) = self.icon {
                    let icon_center =
                        egui::pos2(x_offset + icon_size.size() / 2.0, rect.center().y);
                    icon_manager.draw_icon_at(painter, icon_center, icon, icon_size, text_color);
                    x_offset += icon_size.size() + 6.0; // Add spacing
                }

                // Draw text
                if !self.text.is_empty() {
                    let text_galley =
                        painter.layout_no_wrap(self.text.clone(), text_font, text_color);
                    let text_pos =
                        egui::pos2(x_offset, rect.center().y - text_galley.size().y / 2.0);
                    painter.add(egui::epaint::TextShape::new(
                        text_pos,
                        text_galley,
                        text_color,
                    ));
                }
            }
            IconPosition::Right => {
                let mut x_offset = rect.left();

                // Calculate total width needed
                let text_galley = if !self.text.is_empty() {
                    Some(painter.layout_no_wrap(self.text.clone(), text_font.clone(), text_color))
                } else {
                    None
                };

                // Draw text first
                if let Some(galley) = &text_galley {
                    let text_pos = egui::pos2(x_offset, rect.center().y - galley.size().y / 2.0);
                    painter.add(egui::epaint::TextShape::new(
                        text_pos,
                        galley.clone(),
                        text_color,
                    ));
                    x_offset += galley.size().x + 6.0; // Add spacing
                }

                // Draw icon on the right
                if let Some(icon) = self.icon {
                    let icon_center =
                        egui::pos2(x_offset + icon_size.size() / 2.0, rect.center().y);
                    icon_manager.draw_icon_at(painter, icon_center, icon, icon_size, text_color);
                }
            }
        }
    }
}

/// Segmented control for exclusive selections
#[derive(Debug)]
#[must_use]
pub struct SegmentedControl<T: PartialEq + Clone> {
    options: Vec<(T, String, Option<IconType>)>,
    selected: T,
    size: ButtonSize,
}

impl<T: PartialEq + Clone> SegmentedControl<T> {
    /// Creates a new segmented control
    pub const fn new(selected: T) -> Self {
        Self {
            options: Vec::new(),
            selected,
            size: ButtonSize::Medium,
        }
    }

    /// Adds an option to the control
    pub fn option(mut self, value: T, label: impl Into<String>, icon: Option<IconType>) -> Self {
        self.options.push((value, label.into(), icon));
        self
    }

    /// Sets the size of the control
    pub const fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Shows the control and returns the selected value
    pub fn show(mut self, ui: &mut egui::Ui, _icon_manager: &mut IconManager) -> Option<T> {
        let tokens = Theme::design_tokens(ui.visuals().dark_mode);
        let mut changed = None;

        // Ensure vertical centering of the entire control
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            // Container with background
            let container_color = tokens.colors.surface_container;
            let container_rounding = tokens.radius.full;

            egui::Frame::new()
                .fill(container_color)
                .corner_radius(container_rounding)
                .inner_margin(egui::Margin::same(0)) // No margin for proper alignment
                .show(ui, |ui| {
                    for (i, (value, label, _icon)) in self.options.iter().enumerate() {
                        let is_selected = *value == self.selected;
                        let _is_last = i == self.options.len() - 1;

                        // Create segment button
                        let button_height = self.size.height(); // Use full button height
                        #[allow(clippy::cast_precision_loss)]
                        let response = ui.allocate_response(
                            egui::vec2((label.len() as f32).mul_add(8.0, 24.0), button_height),
                            egui::Sense::click(),
                        );
                        let rect = response.rect;

                        if response.clicked() {
                            self.selected = value.clone();
                            changed = Some(value.clone());
                        }

                        // Enhanced hover state with animation
                        let hover_amount = ui.ctx().animate_bool(
                            response.id.with("hover"),
                            response.hovered() && !is_selected,
                        );

                        // Draw segment with smooth transitions
                        let segment_color = if is_selected {
                            tokens.colors.primary
                        } else if hover_amount > 0.0 {
                            tokens
                                .colors
                                .surface_container_high
                                .gamma_multiply(hover_amount)
                        } else {
                            egui::Color32::TRANSPARENT
                        };

                        let text_color = if is_selected {
                            egui::Color32::WHITE
                        } else if hover_amount > 0.0 {
                            // Subtle text brightening on hover
                            let base = tokens.colors.on_surface;
                            let factor = 1.0 + (hover_amount * 0.2);
                            egui::Color32::from_rgb(
                                (base.r() as f32 * factor).min(255.0) as u8,
                                (base.g() as f32 * factor).min(255.0) as u8,
                                (base.b() as f32 * factor).min(255.0) as u8,
                            )
                        } else {
                            tokens.colors.on_surface
                        };

                        let segment_rounding = tokens.radius.md;

                        ui.painter()
                            .rect_filled(rect, segment_rounding, segment_color);

                        // Draw content - use painter directly for better control
                        let painter = ui.painter();
                        let font_id = egui::FontId::proportional(14.0);

                        // Calculate text galley first to properly center it
                        let text_galley =
                            painter.layout_no_wrap(label.clone(), font_id.clone(), text_color);

                        // Center the text vertically and horizontally
                        let text_pos = egui::pos2(
                            rect.center().x - text_galley.size().x / 2.0,
                            rect.center().y - text_galley.size().y / 2.0,
                        );

                        painter.galley(text_pos, text_galley, text_color);

                        if i < self.options.len() - 1 {
                            ui.add_space(1.0);
                        }
                    }
                });
        });

        changed
    }
}

// Progress bar animation infrastructure ready for future implementation

/// Enhanced progress bar with smooth animations
#[derive(Debug)]
#[must_use]
pub struct ProgressBar {
    progress: f32,
    height: f32,
    show_text: bool,
    color: Option<egui::Color32>,
    animate: bool,
}

impl ProgressBar {
    /// Creates a new progress bar
    pub const fn new(progress: f32) -> Self {
        Self {
            progress: if progress < 0.0 {
                0.0
            } else if progress > 1.0 {
                1.0
            } else {
                progress
            },
            height: 8.0,
            show_text: false,
            color: None,
            animate: true,
        }
    }

    /// Sets the height of the progress bar
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Shows percentage text
    pub const fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    /// Sets custom color
    pub const fn color(mut self, color: egui::Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Enables or disables animation
    pub const fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Shows the progress bar
    pub fn show(self, ui: &mut egui::Ui) {
        let tokens = Theme::design_tokens(ui.visuals().dark_mode);
        let available_width = ui.available_width();

        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(available_width, self.height),
            egui::Sense::hover(),
        );

        // Background
        ui.painter()
            .rect_filled(rect, tokens.radius.full, tokens.colors.surface_container);

        // Progress fill
        let progress_width = rect.width() * self.progress;
        let progress_rect =
            egui::Rect::from_min_size(rect.min, egui::vec2(progress_width, rect.height()));

        let fill_color = self.color.unwrap_or(tokens.colors.primary);
        ui.painter()
            .rect_filled(progress_rect, tokens.radius.full, fill_color);

        // Text overlay
        if self.show_text {
            let text = format!("{:.0}%", self.progress * 100.0);
            let text_color = tokens.colors.on_surface;
            #[allow(clippy::cast_precision_loss)]
            let text_pos = rect.center() - egui::vec2(text.len() as f32 * 3.0, 6.0);
            ui.painter().text(
                text_pos,
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::default(),
                text_color,
            );
        }
    }
}
