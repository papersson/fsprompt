//! SVG icon management system for consistent iconography

use eframe::egui;

/// Icon types used throughout the application
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum IconType {
    // Navigation icons
    /// Closed folder icon
    Folder,
    /// Open folder icon
    FolderOpen,
    /// File icon
    File,
    /// Right chevron arrow
    ChevronRight,
    /// Down chevron arrow
    ChevronDown,

    // Action icons
    /// Settings gear icon
    Settings,
    /// Generate/rocket icon
    Generate,
    /// Copy to clipboard icon
    Copy,
    /// Save to disk icon
    Save,
    /// Close/X icon
    Close,
    /// Refresh/reload icon
    Refresh,

    // File type icons
    /// Code file icon
    Code,
    /// Configuration file icon
    Config,
    /// Document/text file icon
    Document,
    /// Image file icon
    Image,
    /// Archive/zip file icon
    Archive,

    // Status icons
    /// Success checkmark icon
    Success,
    /// Warning triangle icon
    Warning,
    /// Error X icon
    Error,
    /// Information icon
    Info,

    // UI icons
    /// Theme/palette icon
    Theme,
    /// Search magnifying glass icon
    Search,
    /// Filter dropdown icon
    Filter,
}

/// Icon size variants
#[derive(Debug, Clone, Copy)]
pub enum IconSize {
    /// Small icon (16px)
    Small,
    /// Medium icon (20px)
    Medium,
    /// Large icon (24px)
    Large,
    /// Extra large icon (32px)
    XLarge,
}

impl IconSize {
    /// Returns the size in pixels for this icon size
    pub const fn size(self) -> f32 {
        match self {
            Self::Small => 14.0,
            Self::Medium => 18.0,
            Self::Large => 22.0,
            Self::XLarge => 30.0,
        }
    }
}

/// Icon manager with caching and SVG support
#[derive(Debug, Default)]
pub struct IconManager {
    // For now, we'll use emoji fallbacks until SVG system is fully implemented
    _placeholder: (),
}

impl IconManager {
    /// Creates a new icon manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets an icon (simplified for now to use emoji fallbacks)
    pub const fn get_icon(
        &self,
        _icon_type: IconType,
        _size: IconSize,
        _ctx: &egui::Context,
    ) -> Option<String> {
        // For now, always return None to use emoji fallbacks
        None
    }

    /// Shows an icon with optional tint color
    pub fn show_icon(
        &mut self,
        ui: &mut egui::Ui,
        icon_type: IconType,
        size: IconSize,
        tint: Option<egui::Color32>,
    ) {
        // For now, always use emoji fallbacks
        let emoji = Self::fallback_emoji(icon_type);
        let mut text = egui::RichText::new(emoji).size(size.size());

        if let Some(color) = tint {
            text = text.color(color);
        }

        ui.label(text);
    }

    /// Draws an icon at a specific position using the painter API
    pub fn draw_icon_at(
        &mut self,
        painter: &egui::Painter,
        pos: egui::Pos2,
        icon_type: IconType,
        size: IconSize,
        tint: egui::Color32,
    ) {
        // For now, use emoji fallbacks drawn as text
        let emoji = Self::fallback_emoji(icon_type);
        let text_size = size.size();

        let font_id = egui::FontId::new(text_size, egui::FontFamily::Proportional);

        // Create a galley for the emoji text
        let galley = painter.layout_no_wrap(emoji.to_string(), font_id.clone(), tint);

        // Center the icon at the given position
        let text_pos = pos - galley.size() / 2.0;

        // Draw the text
        painter.add(egui::epaint::TextShape::new(text_pos, galley, tint));
    }

    /// Shows an icon button with hover effects
    pub fn icon_button(
        &mut self,
        ui: &mut egui::Ui,
        icon_type: IconType,
        size: IconSize,
        tooltip: Option<&str>,
    ) -> egui::Response {
        let button_size = egui::vec2(size.size() + 8.0, size.size() + 8.0);
        let (rect, response) = ui.allocate_exact_size(button_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let hover_color = if response.hovered() {
                ui.visuals().widgets.hovered.fg_stroke.color
            } else {
                ui.visuals().widgets.inactive.fg_stroke.color
            };

            let icon_rect =
                egui::Rect::from_center_size(rect.center(), egui::vec2(size.size(), size.size()));
            let mut icon_ui = ui.new_child(egui::UiBuilder::new().max_rect(icon_rect).layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
            ));

            let emoji = Self::fallback_emoji(icon_type);
            icon_ui.label(
                egui::RichText::new(emoji)
                    .size(size.size())
                    .color(hover_color),
            );

            if response.hovered() {
                ui.painter().rect_filled(
                    rect,
                    egui::CornerRadius::same(4),
                    ui.visuals().widgets.hovered.weak_bg_fill,
                );
            }
        }

        if let Some(tooltip_text) = tooltip {
            response.on_hover_text(tooltip_text)
        } else {
            response
        }
    }

    /// Fallback emoji for when SVG icons aren't available
    const fn fallback_emoji(icon_type: IconType) -> &'static str {
        match icon_type {
            IconType::Folder => "📁",
            IconType::FolderOpen => "📂",
            IconType::File => "📄",
            IconType::ChevronRight => "▶",
            IconType::ChevronDown => "▼",
            IconType::Settings | IconType::Config => "⚙",
            IconType::Generate => "🚀",
            IconType::Copy => "📋",
            IconType::Save => "💾",
            IconType::Close => "×",
            IconType::Refresh => "🔄",
            IconType::Code => "📝",
            IconType::Document => "📚",
            IconType::Image => "🖼",
            IconType::Archive => "📦",
            IconType::Success => "✓",
            IconType::Warning => "⚠",
            IconType::Error => "✕",
            IconType::Info => "ℹ",
            IconType::Theme => "🎨",
            IconType::Search => "🔍",
            IconType::Filter => "🔽",
        }
    }
}

/// Convenience function to show an icon
pub fn show_icon(
    ui: &mut egui::Ui,
    icon_manager: &mut IconManager,
    icon_type: IconType,
    size: IconSize,
    tint: Option<egui::Color32>,
) {
    icon_manager.show_icon(ui, icon_type, size, tint);
}

/// Convenience function for icon buttons
pub fn icon_button(
    ui: &mut egui::Ui,
    icon_manager: &mut IconManager,
    icon_type: IconType,
    size: IconSize,
    tooltip: Option<&str>,
) -> egui::Response {
    icon_manager.icon_button(ui, icon_type, size, tooltip)
}
