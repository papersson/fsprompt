//! Animation utilities for smooth UI transitions

use eframe::egui;
use std::time::Instant;

/// Animation state for smooth transitions
#[derive(Debug, Clone)]
pub struct AnimationState {
    start_time: Instant,
    duration: f32,
    start_value: f32,
    target_value: f32,
    current_value: f32,
    easing: EasingFunction,
}

/// Easing functions for animations
#[derive(Debug, Clone, Copy)]
pub enum EasingFunction {
    Linear,
    EaseOut,
    EaseInOut,
    Bounce,
}

impl AnimationState {
    /// Creates a new animation
    pub fn new(start_value: f32, target_value: f32, duration: f32, easing: EasingFunction) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
            start_value,
            target_value,
            current_value: start_value,
            easing,
        }
    }

    /// Updates the animation and returns current value
    pub fn update(&mut self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let progress = (elapsed / self.duration).min(1.0);

        let eased_progress = match self.easing {
            EasingFunction::Linear => progress,
            EasingFunction::EaseOut => 1.0 - (1.0 - progress).powi(3),
            EasingFunction::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - (-2.0 * progress + 2.0).powi(2) / 2.0
                }
            }
            EasingFunction::Bounce => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
        };

        self.current_value =
            self.start_value + (self.target_value - self.start_value) * eased_progress;
        self.current_value
    }

    /// Returns true if animation is complete
    pub fn is_complete(&self) -> bool {
        self.start_time.elapsed().as_secs_f32() >= self.duration
    }

    /// Gets the current value without updating
    pub const fn current_value(&self) -> f32 {
        self.current_value
    }

    /// Sets a new target value, creating a smooth transition
    pub fn set_target(&mut self, new_target: f32, duration: f32) {
        self.start_time = Instant::now();
        self.start_value = self.current_value;
        self.target_value = new_target;
        self.duration = duration;
    }
}

/// Color animation for smooth color transitions
#[derive(Debug, Clone)]
pub struct ColorAnimation {
    start_time: Instant,
    duration: f32,
    start_color: egui::Color32,
    target_color: egui::Color32,
    current_color: egui::Color32,
    easing: EasingFunction,
}

impl ColorAnimation {
    /// Creates a new color animation
    pub fn new(
        start_color: egui::Color32,
        target_color: egui::Color32,
        duration: f32,
        easing: EasingFunction,
    ) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
            start_color,
            target_color,
            current_color: start_color,
            easing,
        }
    }

    /// Updates the animation and returns current color
    pub fn update(&mut self) -> egui::Color32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let progress = (elapsed / self.duration).min(1.0);

        let eased_progress = match self.easing {
            EasingFunction::Linear => progress,
            EasingFunction::EaseOut => 1.0 - (1.0 - progress).powi(3),
            EasingFunction::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - (-2.0 * progress + 2.0).powi(2) / 2.0
                }
            }
            EasingFunction::Bounce => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
        };

        // Interpolate RGBA components
        let start_r = f32::from(self.start_color.r());
        let start_g = f32::from(self.start_color.g());
        let start_b = f32::from(self.start_color.b());
        let start_a = f32::from(self.start_color.a());

        let target_r = f32::from(self.target_color.r());
        let target_g = f32::from(self.target_color.g());
        let target_b = f32::from(self.target_color.b());
        let target_a = f32::from(self.target_color.a());

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let current_r = (start_r + (target_r - start_r) * eased_progress) as u8;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let current_g = (start_g + (target_g - start_g) * eased_progress) as u8;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let current_b = (start_b + (target_b - start_b) * eased_progress) as u8;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let current_a = (start_a + (target_a - start_a) * eased_progress) as u8;

        self.current_color =
            egui::Color32::from_rgba_unmultiplied(current_r, current_g, current_b, current_a);
        self.current_color
    }

    /// Returns true if animation is complete
    pub fn is_complete(&self) -> bool {
        self.start_time.elapsed().as_secs_f32() >= self.duration
    }

    /// Gets the current color without updating
    pub const fn current_color(&self) -> egui::Color32 {
        self.current_color
    }

    /// Sets a new target color, creating a smooth transition
    pub fn set_target(&mut self, new_target: egui::Color32, duration: f32) {
        self.start_time = Instant::now();
        self.start_color = self.current_color;
        self.target_color = new_target;
        self.duration = duration;
    }
}

/// Loading spinner animation
#[derive(Debug)]
pub struct SpinnerAnimation {
    start_time: Instant,
    speed: f32, // Rotations per second
}

impl SpinnerAnimation {
    /// Creates a new spinner animation
    pub fn new(speed: f32) -> Self {
        Self {
            start_time: Instant::now(),
            speed,
        }
    }

    /// Gets the current rotation angle in radians
    pub fn rotation(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        (elapsed * self.speed * 2.0 * std::f32::consts::PI) % (2.0 * std::f32::consts::PI)
    }

    /// Draws a spinning circle
    pub fn draw_circle(
        &self,
        ui: &mut egui::Ui,
        center: egui::Pos2,
        radius: f32,
        color: egui::Color32,
    ) {
        let rotation = self.rotation();
        let num_segments = 8;

        for i in 0..num_segments {
            let angle = rotation + (i as f32 / num_segments as f32) * 2.0 * std::f32::consts::PI;
            let opacity = (1.0 - (i as f32 / num_segments as f32)) * 255.0;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let segment_color = egui::Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                opacity as u8,
            );

            let x = center.x + angle.cos() * radius * 0.8;
            let y = center.y + angle.sin() * radius * 0.8;
            let pos = egui::Pos2::new(x, y);

            ui.painter()
                .circle_filled(pos, radius * 0.15, segment_color);
        }
    }

    /// Draws a spinning arc
    pub fn draw_arc(
        &self,
        ui: &mut egui::Ui,
        center: egui::Pos2,
        radius: f32,
        color: egui::Color32,
        stroke_width: f32,
    ) {
        let rotation = self.rotation();
        let arc_length = std::f32::consts::PI * 1.5; // 3/4 circle

        // Draw the arc using multiple small segments
        let num_segments = 32;
        let segment_angle = arc_length / num_segments as f32;

        for i in 0..num_segments {
            let start_angle = rotation + i as f32 * segment_angle;
            let end_angle = rotation + (i + 1) as f32 * segment_angle;

            let start_x = center.x + start_angle.cos() * radius;
            let start_y = center.y + start_angle.sin() * radius;
            let end_x = center.x + end_angle.cos() * radius;
            let end_y = center.y + end_angle.sin() * radius;

            let opacity = (1.0 - (i as f32 / num_segments as f32)) * 255.0;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let segment_color = egui::Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                opacity as u8,
            );

            ui.painter().line_segment(
                [
                    egui::Pos2::new(start_x, start_y),
                    egui::Pos2::new(end_x, end_y),
                ],
                egui::Stroke::new(stroke_width, segment_color),
            );
        }
    }
}
