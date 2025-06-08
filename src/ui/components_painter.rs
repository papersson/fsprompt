//! Optimized button implementation using egui's painter API directly
//! This module demonstrates best practices for drawing buttons without child UI elements

use crate::ui::{
    animations::SpinnerAnimation,
    icons::{IconManager, IconSize, IconType},
    theme::{DesignTokens, Elevation, Theme},
};
use eframe::egui::{self, epaint, Align2, Color32, FontId, Pos2, Rect, Sense, Vec2};

/// Optimized button drawing using painter API
pub struct PainterButton {
    text: String,
    variant: super::ButtonVariant,
    size: super::ButtonSize,
    icon: Option<IconType>,
    icon_position: super::IconPosition,
    loading: bool,
    disabled: bool,
    min_width: Option<f32>,
    tooltip: Option<String>,
}

impl PainterButton {
    /// Draw button using painter API for better performance
    pub fn draw(
        self,
        ui: &mut egui::Ui,
        icon_manager: &mut IconManager,
    ) -> egui::Response {
        let tokens = Theme::design_tokens(ui.visuals().dark_mode);
        let enabled = !self.disabled && !self.loading;
        
        // Calculate button dimensions
        let button_height = self.size.height();
        let button_padding = self.size.padding();
        let min_width = self.min_width.unwrap_or({
            if matches!(self.icon_position, super::IconPosition::Only) {
                button_height
            } else {
                80.0
            }
        });
        
        // Allocate space
        let desired_size = egui::vec2(min_width, button_height);
        let (rect, mut response) = ui.allocate_at_least(desired_size, Sense::click());
        
        // Early return if not visible
        if !ui.is_rect_visible(rect) {
            return response;
        }
        
        // Add interaction feedback
        if enabled {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        
        // Add tooltip
        if let Some(tooltip_text) = &self.tooltip {
            response = response.on_hover_text(tooltip_text);
        }
        
        // Get painter
        let painter = ui.painter();
        
        // Calculate hover state
        let is_hovered = response.hovered() || ui.ctx().input(|i| {
            if let Some(pointer_pos) = i.pointer.interact_pos() {
                rect.contains(pointer_pos) && !i.pointer.any_down()
            } else {
                false
            }
        });
        
        // Get colors
        let (bg_color, text_color) = self.get_colors(&tokens, enabled, is_hovered, response.is_pointer_button_down_on());
        
        // Draw background
        painter.rect_filled(rect, tokens.radius.md, bg_color);
        
        // Draw shadow for elevation
        if enabled {
            let elevation = if response.is_pointer_button_down_on() {
                Elevation::Level2
            } else if is_hovered {
                match self.variant {
                    super::ButtonVariant::Primary => Elevation::Level3,
                    _ => Elevation::Level2,
                }
            } else {
                match self.variant {
                    super::ButtonVariant::Primary => Elevation::Level2,
                    _ => Elevation::Level1,
                }
            };
            
            let shadow = elevation.shadow(ui.visuals().dark_mode);
            if shadow != egui::epaint::Shadow::NONE {
                let shadow_rect = rect.translate([shadow.offset[0] as f32, shadow.offset[1] as f32].into());
                painter.rect_filled(shadow_rect, tokens.radius.md, shadow.color);
            }
        }
        
        // Draw content
        let content_rect = rect.shrink(button_padding);
        
        if self.loading {
            // Draw spinner
            self.draw_loading_spinner(painter, content_rect, text_color);
        } else {
            // Draw icon and text
            self.draw_content(painter, content_rect, text_color, &tokens, icon_manager);
        }
        
        response
    }
    
    /// Draw button content using painter API
    fn draw_content(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        text_color: Color32,
        tokens: &DesignTokens,
        icon_manager: &IconManager,
    ) {
        let font_id = FontId::proportional(tokens.typography.body_medium);
        
        // Get icon emoji if available
        let icon_emoji = self.icon.map(|icon_type| {
            icon_manager.get_icon(icon_type, self.size.icon_size(), painter.ctx())
                .unwrap_or_else(|| IconManager::fallback_emoji(icon_type).to_string())
        });
        
        match self.icon_position {
            super::IconPosition::Only => {
                // Center icon only
                if let Some(emoji) = icon_emoji {
                    let icon_galley = painter.layout_no_wrap(
                        emoji,
                        FontId::proportional(self.size.icon_size().size()),
                        text_color
                    );
                    
                    let icon_pos = rect.center() - icon_galley.size() / 2.0;
                    painter.add(epaint::TextShape::new(
                        icon_pos.to_pos2(),
                        icon_galley,
                        text_color
                    ));
                }
            }
            super::IconPosition::Left => {
                self.draw_icon_and_text(painter, rect, text_color, font_id, icon_emoji.as_deref(), true);
            }
            super::IconPosition::Right => {
                self.draw_icon_and_text(painter, rect, text_color, font_id, icon_emoji.as_deref(), false);
            }
        }
    }
    
    /// Draw icon and text with proper alignment
    fn draw_icon_and_text(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        text_color: Color32,
        font_id: FontId,
        icon_emoji: Option<&str>,
        icon_left: bool,
    ) {
        let spacing = 6.0;
        
        // Layout text
        let text_galley = if !self.text.is_empty() {
            Some(painter.layout_no_wrap(
                self.text.clone(),
                font_id,
                text_color
            ))
        } else {
            None
        };
        
        // Layout icon
        let icon_galley = icon_emoji.map(|emoji| {
            painter.layout_no_wrap(
                emoji.to_string(),
                FontId::proportional(self.size.icon_size().size()),
                text_color
            )
        });
        
        // Calculate total width
        let icon_width = icon_galley.as_ref().map_or(0.0, |g| g.size().x);
        let text_width = text_galley.as_ref().map_or(0.0, |g| g.size().x);
        let total_width = icon_width + text_width + if icon_width > 0.0 && text_width > 0.0 { spacing } else { 0.0 };
        
        // Center content horizontally
        let start_x = rect.center().x - total_width / 2.0;
        
        if icon_left {
            // Draw icon first
            let mut x = start_x;
            if let Some(galley) = icon_galley {
                let pos = Pos2::new(x, rect.center().y - galley.size().y / 2.0);
                painter.add(epaint::TextShape::new(pos, galley, text_color));
                x += icon_width + spacing;
            }
            
            // Draw text
            if let Some(galley) = text_galley {
                let pos = Pos2::new(x, rect.center().y - galley.size().y / 2.0);
                painter.add(epaint::TextShape::new(pos, galley, text_color));
            }
        } else {
            // Draw text first
            let mut x = start_x;
            if let Some(galley) = text_galley {
                let pos = Pos2::new(x, rect.center().y - galley.size().y / 2.0);
                painter.add(epaint::TextShape::new(pos, galley, text_color));
                x += text_width + spacing;
            }
            
            // Draw icon
            if let Some(galley) = icon_galley {
                let pos = Pos2::new(x, rect.center().y - galley.size().y / 2.0);
                painter.add(epaint::TextShape::new(pos, galley, text_color));
            }
        }
    }
    
    /// Draw loading spinner
    fn draw_loading_spinner(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        color: Color32,
    ) {
        // Simple spinner animation
        let time = painter.ctx().input(|i| i.time);
        let radius = self.size.icon_size().size() / 2.0;
        let center = rect.center();
        
        // Draw spinning arc
        let start_angle = (time * 2.0) as f32;
        let sweep = std::f32::consts::PI * 1.5;
        
        painter.add(epaint::Shape::Path(epaint::PathShape {
            points: arc_points(center, radius, start_angle, sweep, 32),
            closed: false,
            fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(2.0, color),
        }));
        
        // Request repaint for animation
        painter.ctx().request_repaint();
    }
    
    /// Get colors based on state
    fn get_colors(
        &self,
        tokens: &DesignTokens,
        enabled: bool,
        hovered: bool,
        pressed: bool,
    ) -> (Color32, Color32) {
        if !enabled {
            return (
                tokens.colors.surface_variant.gamma_multiply(0.6),
                tokens.colors.on_surface_variant.gamma_multiply(0.4)
            );
        }
        
        match self.variant {
            super::ButtonVariant::Primary => {
                let bg = if pressed {
                    tokens.colors.primary.gamma_multiply(0.8)
                } else if hovered {
                    tokens.colors.primary_hover
                } else {
                    tokens.colors.primary
                };
                (bg, Color32::WHITE)
            }
            super::ButtonVariant::Secondary => {
                let bg = if pressed {
                    tokens.colors.surface_container.gamma_multiply(0.9)
                } else if hovered {
                    tokens.colors.surface_container_high
                } else {
                    tokens.colors.surface_container
                };
                (bg, tokens.colors.on_surface)
            }
            super::ButtonVariant::Ghost => {
                let bg = if pressed {
                    tokens.colors.surface_variant.gamma_multiply(0.2)
                } else if hovered {
                    tokens.colors.surface_variant.gamma_multiply(0.1)
                } else {
                    Color32::TRANSPARENT
                };
                (bg, tokens.colors.on_surface)
            }
            super::ButtonVariant::Danger => {
                let bg = if pressed {
                    tokens.colors.error.gamma_multiply(0.8)
                } else if hovered {
                    tokens.colors.error_container
                } else {
                    tokens.colors.error
                };
                (bg, Color32::WHITE)
            }
        }
    }
}

/// Generate points for an arc
fn arc_points(center: Pos2, radius: f32, start_angle: f32, sweep: f32, segments: usize) -> Vec<Pos2> {
    (0..=segments)
        .map(|i| {
            let t = i as f32 / segments as f32;
            let angle = start_angle + sweep * t;
            Pos2::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            )
        })
        .collect()
}

/// Extension trait to make button creation easier
pub trait ButtonExt {
    /// Convert to painter button for optimized drawing
    fn to_painter(self) -> PainterButton;
}

impl ButtonExt for super::Button {
    fn to_painter(self) -> PainterButton {
        PainterButton {
            text: self.text,
            variant: self.variant,
            size: self.size,
            icon: self.icon,
            icon_position: self.icon_position,
            loading: self.loading,
            disabled: self.disabled,
            min_width: self.min_width,
            tooltip: self.tooltip,
        }
    }
}

/// Optimized icon button using painter API
pub fn draw_icon_button(
    ui: &mut egui::Ui,
    icon_type: IconType,
    size: IconSize,
    tooltip: Option<&str>,
) -> egui::Response {
    let button_size = egui::vec2(size.size() + 8.0, size.size() + 8.0);
    let (rect, mut response) = ui.allocate_exact_size(button_size, Sense::click());
    
    if !ui.is_rect_visible(rect) {
        return response;
    }
    
    if let Some(tooltip_text) = tooltip {
        response = response.on_hover_text(tooltip_text);
    }
    
    let painter = ui.painter();
    let is_hovered = response.hovered();
    let is_pressed = response.is_pointer_button_down_on();
    
    // Draw hover background
    if is_hovered || is_pressed {
        let bg_color = if is_pressed {
            ui.visuals().widgets.active.weak_bg_fill
        } else {
            ui.visuals().widgets.hovered.weak_bg_fill
        };
        painter.rect_filled(rect, 4.0, bg_color);
    }
    
    // Draw icon
    let icon_color = if is_hovered {
        ui.visuals().widgets.hovered.text_color()
    } else {
        ui.visuals().widgets.inactive.text_color()
    };
    
    let emoji = IconManager::fallback_emoji(icon_type);
    let galley = painter.layout_no_wrap(
        emoji.to_string(),
        FontId::proportional(size.size()),
        icon_color
    );
    
    let icon_pos = rect.center() - galley.size() / 2.0;
    painter.add(epaint::TextShape::new(
        icon_pos.to_pos2(),
        galley,
        icon_color
    ));
    
    response
}

/// Helper trait for Vec2 to Pos2 conversion
trait Vec2Ext {
    fn to_pos2(self) -> Pos2;
}

impl Vec2Ext for Vec2 {
    fn to_pos2(self) -> Pos2 {
        Pos2::new(self.x, self.y)
    }
}