//! Toast notification system for user feedback

use eframe::egui;
use std::time::{Duration, Instant};

use crate::ui::Theme;

/// Toast notification variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastVariant {
    /// Success message (green)
    Success,
    /// Warning message (yellow)
    Warning,
    /// Error message (red)
    Error,
}

impl ToastVariant {
    /// Gets the color for this variant
    fn color(&self) -> egui::Color32 {
        match self {
            Self::Success => Theme::SUCCESS,
            Self::Warning => Theme::WARNING,
            Self::Error => Theme::ERROR,
        }
    }

    /// Gets the icon for this variant
    fn icon(&self) -> &'static str {
        match self {
            Self::Success => "✓",
            Self::Warning => "⚠",
            Self::Error => "✕",
        }
    }

    /// Gets the auto-dismiss duration
    fn dismiss_duration(&self) -> Duration {
        match self {
            Self::Success => Duration::from_secs(2),
            Self::Warning => Duration::from_secs(3),
            Self::Error => Duration::from_secs(4),
        }
    }
}

/// A single toast notification
#[derive(Debug, Clone)]
pub struct Toast {
    /// The message to display
    pub message: String,
    /// The variant (determines color and duration)
    pub variant: ToastVariant,
    /// When the toast was created
    pub created_at: Instant,
}

impl Toast {
    /// Creates a new success toast
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            variant: ToastVariant::Success,
            created_at: Instant::now(),
        }
    }

    /// Creates a new warning toast
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            variant: ToastVariant::Warning,
            created_at: Instant::now(),
        }
    }

    /// Creates a new error toast
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            variant: ToastVariant::Error,
            created_at: Instant::now(),
        }
    }

    /// Checks if the toast should be dismissed
    pub fn should_dismiss(&self) -> bool {
        self.created_at.elapsed() >= self.variant.dismiss_duration()
    }

    /// Gets the remaining time as a fraction (0.0 to 1.0)
    pub fn remaining_fraction(&self) -> f32 {
        let elapsed = self.created_at.elapsed();
        let total = self.variant.dismiss_duration();

        if elapsed >= total {
            0.0
        } else {
            1.0 - (elapsed.as_secs_f32() / total.as_secs_f32())
        }
    }
}

/// Toast notification manager
#[derive(Debug, Default)]
pub struct ToastManager {
    /// Current toast (only one at a time per spec)
    current_toast: Option<Toast>,
}

impl ToastManager {
    /// Creates a new toast manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Shows a new toast notification
    pub fn show(&mut self, toast: Toast) {
        self.current_toast = Some(toast);
    }

    /// Shows a success toast
    pub fn success(&mut self, message: impl Into<String>) {
        self.show(Toast::success(message));
    }

    /// Shows a warning toast
    pub fn warning(&mut self, message: impl Into<String>) {
        self.show(Toast::warning(message));
    }

    /// Shows an error toast
    pub fn error(&mut self, message: impl Into<String>) {
        self.show(Toast::error(message));
    }

    /// Shows an info toast (uses warning variant)
    pub fn info(&mut self, message: impl Into<String>) {
        self.show(Toast::warning(message));
    }

    /// Updates the toast state (removes expired toasts)
    pub fn update(&mut self) {
        if let Some(toast) = &self.current_toast {
            if toast.should_dismiss() {
                self.current_toast = None;
            }
        }
    }

    /// Renders the toast UI
    pub fn show_ui(&mut self, ctx: &egui::Context) {
        // Update state first
        self.update();

        if self.current_toast.is_some() {
            let mut should_close = false;

            // Clone values we need in the closure
            let toast = self.current_toast.as_ref().unwrap();
            let variant_color = toast.variant.color();
            let variant_icon = toast.variant.icon();
            let message = toast.message.clone();
            let remaining_fraction = toast.remaining_fraction();

            // Position at top-right corner
            egui::Area::new(egui::Id::new("toast_area"))
                .anchor(
                    egui::Align2::RIGHT_TOP,
                    egui::vec2(-Theme::SPACING_MD, Theme::SPACING_MD),
                )
                .interactable(false)
                .show(ctx, |ui| {
                    // Container with shadow
                    egui::Frame::new()
                        .fill(ui.style().visuals.panel_fill)
                        .shadow(egui::epaint::Shadow {
                            offset: [0, 2],
                            blur: 8,
                            spread: 0,
                            color: egui::Color32::from_black_alpha(40),
                        })
                        .rounding(Theme::RADIUS_LG)
                        .inner_margin(Theme::SPACING_MD)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                // Icon
                                ui.colored_label(variant_color, variant_icon);

                                // Message
                                ui.label(&message);

                                // Close button
                                if ui.small_button("×").clicked() {
                                    should_close = true;
                                }
                            });

                            // Progress bar
                            let progress_height = 3.0;
                            let progress_rect = egui::Rect::from_min_size(
                                ui.cursor().min,
                                egui::vec2(
                                    ui.available_width() * remaining_fraction,
                                    progress_height,
                                ),
                            );
                            ui.painter().rect_filled(
                                progress_rect,
                                egui::CornerRadius::ZERO,
                                variant_color.gamma_multiply(0.3),
                            );

                            // Add space for progress bar
                            ui.add_space(progress_height);
                        });
                });

            if should_close {
                self.current_toast = None;
            }

            // Request repaint for animation
            ctx.request_repaint();
        }
    }
}
