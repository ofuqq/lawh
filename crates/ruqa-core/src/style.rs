//! Visual styling, stroke parameters, and fill patterns for whiteboard elements.

use serde::{Deserialize, Serialize};

/// Curated dark-theme color palette: Crisp Off-White, Coral, Yellow, Green, Cyan, Lavender.
pub const PALETTE_COLORS: [&str; 6] = [
    crate::theme::DARK_PEN_INK, // Crisp Off-White (#ECEFF4)
    "#FF6B6B",                  // Coral
    "#FFD166",                  // Yellow
    "#06D6A0",                  // Green
    "#38BDF8",                  // Cyan
    "#A78BFA",                  // Lavender
];

/// Standard stroke width presets for whiteboard elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StrokeWidthPreset {
    /// Thin outline (1.0 px).
    Thin,
    /// Standard medium outline (2.5 px).
    #[default]
    Medium,
    /// Bold emphasis outline (4.0 px).
    Bold,
}

impl StrokeWidthPreset {
    /// Array of all 3 stroke width presets.
    pub const ALL: [StrokeWidthPreset; 3] = [
        StrokeWidthPreset::Thin,
        StrokeWidthPreset::Medium,
        StrokeWidthPreset::Bold,
    ];

    /// Returns the stroke width in world pixels.
    #[must_use]
    pub fn width(self) -> f32 {
        match self {
            StrokeWidthPreset::Thin => 1.0,
            StrokeWidthPreset::Medium => 2.5,
            StrokeWidthPreset::Bold => 4.0,
        }
    }

    /// Selects the closest preset matching `width`.
    #[must_use]
    pub fn from_width(width: f32) -> Self {
        if width <= 1.75 {
            StrokeWidthPreset::Thin
        } else if width <= 3.25 {
            StrokeWidthPreset::Medium
        } else {
            StrokeWidthPreset::Bold
        }
    }
}

/// Roughness presets governing hand-drawn sketchiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoughnessPreset {
    /// Clean, crisp, straight geometric lines (roughness 0.0).
    Clean,
    /// Hand-drawn rough sketchy lines (roughness 1.2).
    #[default]
    Sketchy,
}

impl RoughnessPreset {
    /// Array of all roughness presets.
    pub const ALL: [RoughnessPreset; 2] = [RoughnessPreset::Clean, RoughnessPreset::Sketchy];

    /// Returns the Rough algorithm roughness multiplier factor.
    #[must_use]
    pub fn factor(self) -> f32 {
        match self {
            RoughnessPreset::Clean => 0.0,
            RoughnessPreset::Sketchy => 1.2,
        }
    }

    /// Selects the closest preset matching `roughness`.
    #[must_use]
    pub fn from_roughness(roughness: f32) -> Self {
        if roughness < 0.6 {
            RoughnessPreset::Clean
        } else {
            RoughnessPreset::Sketchy
        }
    }

    /// Returns the inverted toggle state.
    #[must_use]
    pub fn toggle(self) -> Self {
        match self {
            RoughnessPreset::Clean => RoughnessPreset::Sketchy,
            RoughnessPreset::Sketchy => RoughnessPreset::Clean,
        }
    }
}

/// Fill pattern style for closed whiteboard shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FillStyle {
    /// No fill (transparent interior).
    #[default]
    None,
    /// Solid single-color fill.
    Solid,
    /// Semi-transparent solid single-color fill (50% opacity).
    SemiTransparent,
    /// Hand-drawn parallel hachure lines.
    Hachure,
    /// Crossed hand-drawn hatch lines.
    CrossHatch,
    /// Stippled dot pattern fill.
    Dots,
}

impl FillStyle {
    /// Primary fill styles supported by Ruqa.
    pub const ALL: [FillStyle; 5] = [
        FillStyle::None,
        FillStyle::Solid,
        FillStyle::SemiTransparent,
        FillStyle::Hachure,
        FillStyle::CrossHatch,
    ];

    /// Returns `true` if the fill style is not [`FillStyle::None`].
    #[must_use]
    pub fn is_filled(self) -> bool {
        self != FillStyle::None
    }
}

/// Styling and roughness parameters for a whiteboard element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ElementStyle {
    /// Stroke outline color in hex format (e.g. `"#E0E0E0"`).
    pub stroke_color: String,
    /// Stroke outline width in world units (standard presets: 1.5, 3.0, 5.0).
    pub stroke_width: f32,
    /// Optional fill color in hex format (e.g. `"#1E88E5"`).
    pub fill_color: Option<String>,
    /// Fill pattern style.
    pub fill_style: FillStyle,
    /// Roughness factor (0.0 for crisp straight lines, 1.2 for hand-drawn, 2.5 for very sketchy).
    pub roughness: f32,
    /// Deterministic PRNG seed for reproducible rough lines across renders.
    pub seed: u64,
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self {
            stroke_color: crate::theme::DARK_PEN_INK.to_string(),
            stroke_width: 2.0,
            fill_color: None,
            fill_style: FillStyle::None,
            roughness: 1.2,
            seed: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_style_defaults() {
        let style = ElementStyle::default();
        assert_eq!(style.stroke_color, crate::theme::DARK_PEN_INK);
        assert_eq!(style.stroke_width, 2.0);
        assert!(style.fill_color.is_none());
        assert_eq!(style.fill_style, FillStyle::None);
        assert_eq!(style.roughness, 1.2);
        assert_eq!(style.seed, 0);
    }

    #[test]
    fn test_stroke_width_preset_roundtrip() {
        assert_eq!(StrokeWidthPreset::Thin.width(), 1.0);
        assert_eq!(StrokeWidthPreset::Medium.width(), 2.5);
        assert_eq!(StrokeWidthPreset::Bold.width(), 4.0);

        assert_eq!(StrokeWidthPreset::from_width(1.0), StrokeWidthPreset::Thin);
        assert_eq!(
            StrokeWidthPreset::from_width(2.5),
            StrokeWidthPreset::Medium
        );
        assert_eq!(StrokeWidthPreset::from_width(4.0), StrokeWidthPreset::Bold);
    }

    #[test]
    fn test_roughness_preset_toggle() {
        assert_eq!(RoughnessPreset::Clean.factor(), 0.0);
        assert_eq!(RoughnessPreset::Sketchy.factor(), 1.2);

        assert_eq!(RoughnessPreset::Clean.toggle(), RoughnessPreset::Sketchy);
        assert_eq!(RoughnessPreset::Sketchy.toggle(), RoughnessPreset::Clean);
    }

    #[test]
    fn test_fill_style_properties() {
        assert!(!FillStyle::None.is_filled());
        assert!(FillStyle::Solid.is_filled());
        assert!(FillStyle::SemiTransparent.is_filled());
        assert!(FillStyle::Hachure.is_filled());
        assert!(FillStyle::CrossHatch.is_filled());
        assert_eq!(FillStyle::ALL.len(), 5);
    }
}
