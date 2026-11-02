//! Visual styling, stroke parameters, and fill patterns for whiteboard elements.

use serde::{Deserialize, Serialize};

/// Fill pattern style for closed whiteboard shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FillStyle {
    /// No fill (transparent interior).
    #[default]
    None,
    /// Solid single-color fill.
    Solid,
    /// Hand-drawn parallel hachure lines.
    Hachure,
    /// Crossed hand-drawn hatch lines.
    CrossHatch,
    /// Stippled dot pattern fill.
    Dots,
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
            stroke_color: "#E0E0E0".to_string(),
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
        assert_eq!(style.stroke_color, "#E0E0E0");
        assert_eq!(style.stroke_width, 2.0);
        assert!(style.fill_color.is_none());
        assert_eq!(style.fill_style, FillStyle::None);
        assert_eq!(style.roughness, 1.2);
        assert_eq!(style.seed, 0);
    }
}
