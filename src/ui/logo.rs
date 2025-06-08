//! fsPrompt logo component with branding and animations

use crate::ui::theme::{DesignTokens, Theme};
use eframe::egui;

/// Logo component with branding and animations
#[derive(Debug)]
pub struct Logo {
    size: f32,
    show_text: bool,
    animate_on_hover: bool,
    clickable: bool,
}

impl Logo {
    /// Creates a new logo component
    pub const fn new() -> Self {
        Self {
            size: 32.0,
            show_text: true,
            animate_on_hover: true,
            clickable: false,
        }
    }

    /// Creates a compact logo (icon only)
    pub const fn compact() -> Self {
        Self {
            size: 24.0,
            show_text: false,
            animate_on_hover: true,
            clickable: false,
        }
    }

    /// Creates a large header logo
    pub const fn header() -> Self {
        Self {
            size: 32.0,
            show_text: true,
            animate_on_hover: true,
            clickable: false,
        }
    }

    /// Sets the logo size
    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Sets whether to show the text
    pub const fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    /// Sets whether to animate on hover
    pub const fn animate_on_hover(mut self, animate: bool) -> Self {
        self.animate_on_hover = animate;
        self
    }

    /// Sets whether the logo is clickable
    pub const fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Shows the logo and returns response
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        self.show_animated(ui, None)
    }

    /// Shows the logo with animation support
    pub fn show_animated(
        self,
        ui: &mut egui::Ui,
        _animation_manager: Option<&mut ()>, // Placeholder for compatibility
    ) -> egui::Response {
        let tokens = Theme::design_tokens(ui.visuals().dark_mode);

        // Calculate layout
        // For horizontal layout, text doesn't add to height
        let total_height = self.size;
        let total_width = if self.show_text {
            self.size + 120.0 // Space for "fsPrompt" text
        } else {
            self.size
        };

        // Allocate space
        let sense = if self.clickable {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        };

        let response = ui.allocate_response(egui::vec2(total_width, total_height), sense);
        let rect = response.rect;

        // Use egui's built-in animation system for smooth transitions
        let (scale, rotation) = if self.animate_on_hover {
            // Animate scale using egui's responsive animation
            let scale = ui.ctx().animate_value_with_time(
                response.id.with("logo_scale"),
                if response.hovered() { 1.1 } else { 1.0 },
                tokens.animations.duration_fast,
            );

            // Animate rotation using egui's responsive animation
            let rotation = ui.ctx().animate_value_with_time(
                response.id.with("logo_rotation"),
                if response.hovered() { 0.1 } else { 0.0 },
                tokens.animations.duration_normal,
            );

            (scale, rotation)
        } else {
            (1.0, 0.0)
        };

        // Draw the logo
        self.draw_logo(ui, rect, scale, rotation, &tokens);

        response
    }

    /// Draws the logo graphics
    fn draw_logo(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        scale: f32,
        rotation: f32,
        tokens: &DesignTokens,
    ) {
        let painter = ui.painter();

        // Calculate positions
        let logo_center =
            egui::Pos2::new(rect.left() + self.size / 2.0, rect.top() + self.size / 2.0);

        let scaled_size = self.size * scale;

        // Apply rotation transform
        let cos_r = rotation.cos();
        let sin_r = rotation.sin();

        // Helper function to rotate a point around the center
        let rotate_point = |point: egui::Pos2| -> egui::Pos2 {
            let dx = point.x - logo_center.x;
            let dy = point.y - logo_center.y;
            egui::Pos2::new(
                logo_center.x + dx * cos_r - dy * sin_r,
                logo_center.y + dx * sin_r + dy * cos_r,
            )
        };

        // Draw folder body (main rectangle)
        let folder_width = scaled_size * 0.7;
        let folder_height = scaled_size * 0.5;
        let folder_rect =
            egui::Rect::from_center_size(logo_center, egui::vec2(folder_width, folder_height));

        // Rotate folder corners
        let folder_corners = [
            rotate_point(folder_rect.left_top()),
            rotate_point(folder_rect.right_top()),
            rotate_point(folder_rect.right_bottom()),
            rotate_point(folder_rect.left_bottom()),
        ];

        // Draw folder background
        let folder_color = if ui.visuals().dark_mode {
            tokens.colors.primary.gamma_multiply(0.8)
        } else {
            tokens.colors.primary
        };

        painter.add(egui::epaint::Shape::convex_polygon(
            folder_corners.to_vec(),
            folder_color,
            egui::Stroke::NONE,
        ));

        // Draw folder tab (small rectangle on top)
        let tab_width = folder_width * 0.3;
        let tab_height = folder_height * 0.2;
        let tab_center = egui::Pos2::new(
            logo_center.x - folder_width * 0.2,
            logo_center.y - folder_height * 0.6,
        );
        let tab_rect = egui::Rect::from_center_size(tab_center, egui::vec2(tab_width, tab_height));

        let tab_corners = [
            rotate_point(tab_rect.left_top()),
            rotate_point(tab_rect.right_top()),
            rotate_point(tab_rect.right_bottom()),
            rotate_point(tab_rect.left_bottom()),
        ];

        painter.add(egui::epaint::Shape::convex_polygon(
            tab_corners.to_vec(),
            folder_color,
            egui::Stroke::NONE,
        ));

        // Draw code brackets { } over the folder
        let bracket_size = scaled_size * 0.15;
        let bracket_color = tokens.colors.surface;
        let bracket_stroke = egui::Stroke::new(2.0 * scale, bracket_color);

        // Left bracket {
        let left_bracket_x = logo_center.x - folder_width * 0.15;
        let bracket_y_top = logo_center.y - bracket_size * 0.8;
        let bracket_y_bottom = logo_center.y + bracket_size * 0.8;
        let bracket_y_mid = logo_center.y;

        let left_bracket_points = [
            rotate_point(egui::Pos2::new(
                left_bracket_x + bracket_size * 0.3,
                bracket_y_top,
            )),
            rotate_point(egui::Pos2::new(left_bracket_x, bracket_y_top)),
            rotate_point(egui::Pos2::new(
                left_bracket_x - bracket_size * 0.2,
                bracket_y_mid,
            )),
            rotate_point(egui::Pos2::new(left_bracket_x, bracket_y_bottom)),
            rotate_point(egui::Pos2::new(
                left_bracket_x + bracket_size * 0.3,
                bracket_y_bottom,
            )),
        ];

        painter.add(egui::epaint::Shape::line(
            left_bracket_points.to_vec(),
            bracket_stroke,
        ));

        // Right bracket }
        let right_bracket_x = logo_center.x + folder_width * 0.15;

        let right_bracket_points = [
            rotate_point(egui::Pos2::new(
                right_bracket_x - bracket_size * 0.3,
                bracket_y_top,
            )),
            rotate_point(egui::Pos2::new(right_bracket_x, bracket_y_top)),
            rotate_point(egui::Pos2::new(
                right_bracket_x + bracket_size * 0.2,
                bracket_y_mid,
            )),
            rotate_point(egui::Pos2::new(right_bracket_x, bracket_y_bottom)),
            rotate_point(egui::Pos2::new(
                right_bracket_x - bracket_size * 0.3,
                bracket_y_bottom,
            )),
        ];

        painter.add(egui::epaint::Shape::line(
            right_bracket_points.to_vec(),
            bracket_stroke,
        ));

        // Draw text if enabled
        if self.show_text {
            // Use original position for text (not affected by scale/rotation)
            let text_pos = egui::Pos2::new(
                rect.left() + self.size + 8.0,
                rect.center().y, // Center text vertically with the icon
            );

            painter.text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                "fsPrompt",
                egui::FontId::new(
                    tokens.typography.title_medium.size,
                    egui::FontFamily::Proportional,
                ),
                tokens.colors.on_surface,
            );
        }
    }
}

impl Default for Logo {
    fn default() -> Self {
        Self::new()
    }
}
