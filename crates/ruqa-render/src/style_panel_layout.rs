//! Layout geometry and hit-testing for the contextual styling HUD panel.

use ruqa_core::geometry::{Point, Rect};
use ruqa_core::style::{RoughnessPreset, StrokeWidthPreset, PALETTE_COLORS};

/// Interaction action triggered by clicking inside the contextual style panel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StyleAction {
    /// Changes the active stroke color to the given hex code.
    SetColor(&'static str),
    /// Changes the active stroke width preset.
    SetStrokeWidth(StrokeWidthPreset),
    /// Changes the active roughness preset.
    SetRoughness(RoughnessPreset),
}

/// Layout geometry for the floating contextual styling HUD panel in logical coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StylePanelLayout {
    /// Bounding rectangle of the panel container.
    pub bounds: Rect,
}

impl Default for StylePanelLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl StylePanelLayout {
    /// Default top-left X coordinate of the panel.
    pub const PANEL_X: f32 = 16.0;
    /// Default top-left Y coordinate of the panel (below top toolbar).
    pub const PANEL_Y: f32 = 68.0;
    /// Width of the panel container.
    pub const PANEL_WIDTH: f32 = 204.0;
    /// Height of the panel container.
    pub const PANEL_HEIGHT: f32 = 138.0;
    /// Corner radius of the panel container.
    pub const CORNER_RADIUS: f32 = 8.0;

    /// Creates the standard contextual styling panel layout.
    #[must_use]
    pub fn new() -> Self {
        Self {
            bounds: Rect::from_xywh(
                Self::PANEL_X,
                Self::PANEL_Y,
                Self::PANEL_WIDTH,
                Self::PANEL_HEIGHT,
            ),
        }
    }

    /// Returns the logical screen rectangle for the color swatch at `index`.
    #[must_use]
    pub fn color_swatch_rect(&self, index: usize) -> Rect {
        #[allow(clippy::cast_precision_loss)]
        let x = self.bounds.min.x + 16.0 + index as f32 * 29.0;
        let y = self.bounds.min.y + 14.0;
        Rect::from_xywh(x, y, 22.0, 22.0)
    }

    /// Returns the logical screen rectangle for the stroke width button at `index`.
    #[must_use]
    pub fn width_button_rect(&self, index: usize) -> Rect {
        #[allow(clippy::cast_precision_loss)]
        let x = self.bounds.min.x + 16.0 + index as f32 * 59.0;
        let y = self.bounds.min.y + 48.0;
        Rect::from_xywh(x, y, 54.0, 30.0)
    }

    /// Returns the logical screen rectangle for the roughness button at `index` (0: Clean, 1: Sketchy).
    #[must_use]
    pub fn roughness_button_rect(&self, index: usize) -> Rect {
        #[allow(clippy::cast_precision_loss)]
        let x = self.bounds.min.x + 16.0 + index as f32 * 89.0;
        let y = self.bounds.min.y + 90.0;
        Rect::from_xywh(x, y, 83.0, 30.0)
    }

    /// Hit-tests a logical screen point against panel controls, returning the clicked [`StyleAction`] if any.
    #[must_use]
    pub fn action_at(&self, point: Point) -> Option<StyleAction> {
        if !self.bounds.contains(point) {
            return None;
        }

        // 1. Color swatches
        for (i, &hex) in PALETTE_COLORS.iter().enumerate() {
            if self.color_swatch_rect(i).contains(point) {
                return Some(StyleAction::SetColor(hex));
            }
        }

        // 2. Stroke width buttons
        for (i, &preset) in StrokeWidthPreset::ALL.iter().enumerate() {
            if self.width_button_rect(i).contains(point) {
                return Some(StyleAction::SetStrokeWidth(preset));
            }
        }

        // 3. Roughness buttons
        for (i, &preset) in RoughnessPreset::ALL.iter().enumerate() {
            if self.roughness_button_rect(i).contains(point) {
                return Some(StyleAction::SetRoughness(preset));
            }
        }

        None
    }

    /// Returns `true` if the point falls inside the style panel container bounds.
    #[must_use]
    pub fn contains(&self, point: Point) -> bool {
        self.bounds.contains(point)
    }
}
