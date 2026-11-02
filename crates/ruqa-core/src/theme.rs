//! Default dark theme palette, color constants, and light/dark color inversion algorithms.

use crate::element::Element;
use crate::scene::Scene;
use crate::style::ElementStyle;
use serde::{Deserialize, Serialize};

/// Matte dark canvas background color (`#121212`).
pub const DARK_BACKGROUND_COLOR: &str = "#121212";

/// Crisp light canvas background color (`#FFFFFF`).
pub const LIGHT_BACKGROUND_COLOR: &str = "#FFFFFF";

/// Default crisp off-white pen ink color for dark canvas (`#ECEFF4`).
pub const DARK_PEN_INK: &str = "#ECEFF4";

/// Default dark charcoal ink color for light canvas (`#1E1E1E`).
pub const LIGHT_PEN_INK: &str = "#1E1E1E";

/// Canvas display and rendering theme mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    /// High-contrast matte dark theme (default).
    #[default]
    Dark,
    /// High-contrast clean light theme.
    Light,
}

impl ThemeMode {
    /// Returns the canvas background hex color for this theme.
    #[must_use]
    pub const fn background_color(self) -> &'static str {
        match self {
            Self::Dark => DARK_BACKGROUND_COLOR,
            Self::Light => LIGHT_BACKGROUND_COLOR,
        }
    }

    /// Returns the default stroke ink hex color for this theme.
    #[must_use]
    pub const fn default_ink_color(self) -> &'static str {
        match self {
            Self::Dark => DARK_PEN_INK,
            Self::Light => LIGHT_PEN_INK,
        }
    }

    /// Toggles between dark and light theme modes.
    #[must_use]
    pub const fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}

/// Parses a 3, 4, 6, or 8-digit hexadecimal color string into `(R, G, B, A)` bytes.
#[must_use]
pub fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8, u8)> {
    let clean = hex.trim().strip_prefix('#').unwrap_or(hex.trim());
    match clean.len() {
        3 => {
            let r = u8::from_str_radix(&clean[0..1], 16).ok()?;
            let g = u8::from_str_radix(&clean[1..2], 16).ok()?;
            let b = u8::from_str_radix(&clean[2..3], 16).ok()?;
            Some((r * 17, g * 17, b * 17, 255))
        }
        4 => {
            let r = u8::from_str_radix(&clean[0..1], 16).ok()?;
            let g = u8::from_str_radix(&clean[1..2], 16).ok()?;
            let b = u8::from_str_radix(&clean[2..3], 16).ok()?;
            let a = u8::from_str_radix(&clean[3..4], 16).ok()?;
            Some((r * 17, g * 17, b * 17, a * 17))
        }
        6 => {
            let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
            let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
            let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
            Some((r, g, b, 255))
        }
        8 => {
            let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
            let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
            let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
            let a = u8::from_str_radix(&clean[6..8], 16).ok()?;
            Some((r, g, b, a))
        }
        _ => None,
    }
}

/// Computes the normalized perceptual luminance of an RGB color in `[0.0, 1.0]`.
#[must_use]
pub fn color_luminance(r: u8, g: u8, b: u8) -> f32 {
    let rf = f32::from(r) / 255.0;
    let gf = f32::from(g) / 255.0;
    let bf = f32::from(b) / 255.0;
    0.299 * rf + 0.587 * gf + 0.114 * bf
}

/// Computes the normalized HSV saturation of an RGB color in `[0.0, 1.0]`.
#[must_use]
pub fn color_saturation(r: u8, g: u8, b: u8) -> f32 {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    if max == 0 {
        0.0
    } else {
        f32::from(max - min) / f32::from(max)
    }
}

/// Inverts a color hex string for light/dark theme transitions or export.
///
/// Near-monochrome colors (low saturation, e.g. off-whites, grays, charcoals) invert
/// between light ink and dark ink, while chromatic accent colors (e.g. coral, yellow,
/// green, cyan, lavender) preserve their hue and saturation.
#[must_use]
pub fn invert_color(hex: &str) -> String {
    let Some((r, g, b, a)) = parse_hex_color(hex) else {
        return hex.to_string();
    };

    let sat = color_saturation(r, g, b);
    let lum = color_luminance(r, g, b);

    // If color has chromatic saturation (>= 0.18), preserve accent color identity
    if sat >= 0.18 {
        return hex.to_string();
    }

    // High-luminance monochrome (off-white, white, light gray) -> dark ink
    if lum >= 0.65 {
        if a == 255 {
            LIGHT_PEN_INK.to_string()
        } else {
            format!("{LIGHT_PEN_INK}{a:02X}")
        }
    } else if lum <= 0.35 {
        // Low-luminance monochrome (charcoal, matte dark, black) -> light ink
        if a == 255 {
            DARK_PEN_INK.to_string()
        } else {
            format!("{DARK_PEN_INK}{a:02X}")
        }
    } else {
        // Mid-gray: mirror luminance across midpoint 0.5
        let inv_r = 255 - r;
        let inv_g = 255 - g;
        let inv_b = 255 - b;
        if a == 255 {
            format!("#{inv_r:02X}{inv_g:02X}{inv_b:02X}")
        } else {
            format!("#{inv_r:02X}{inv_g:02X}{inv_b:02X}{a:02X}")
        }
    }
}

/// Inverts the stroke and optional fill color in an [`ElementStyle`].
pub fn invert_element_style(style: &mut ElementStyle) {
    style.stroke_color = invert_color(&style.stroke_color);
    if let Some(fill) = &style.fill_color {
        style.fill_color = Some(invert_color(fill));
    }
}

/// Inverts the style colors of a whiteboard element.
pub fn invert_element(element: &mut Element) {
    invert_element_style(element.style_mut());
}

/// Inverts all element colors across a scene document.
pub fn invert_scene(scene: &mut Scene) {
    for element in scene.elements_mut() {
        invert_element(element);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Rect;

    #[test]
    fn test_parse_hex_color_variants() {
        assert_eq!(parse_hex_color("#FFF"), Some((255, 255, 255, 255)));
        assert_eq!(parse_hex_color("121212"), Some((18, 18, 18, 255)));
        assert_eq!(parse_hex_color("#ECEFF4"), Some((236, 239, 244, 255)));
        assert_eq!(parse_hex_color("#1E1E1EFF"), Some((30, 30, 30, 255)));
        assert_eq!(parse_hex_color("invalid"), None);
    }

    #[test]
    fn test_theme_mode_properties_and_toggle() {
        let dark = ThemeMode::Dark;
        assert_eq!(dark.background_color(), DARK_BACKGROUND_COLOR);
        assert_eq!(dark.default_ink_color(), DARK_PEN_INK);

        let light = dark.toggle();
        assert_eq!(light, ThemeMode::Light);
        assert_eq!(light.background_color(), LIGHT_BACKGROUND_COLOR);
        assert_eq!(light.default_ink_color(), LIGHT_PEN_INK);

        assert_eq!(light.toggle(), ThemeMode::Dark);
    }

    #[test]
    fn test_invert_monochrome_strokes() {
        // Off-white inverts to dark ink
        assert_eq!(invert_color(DARK_PEN_INK), LIGHT_PEN_INK);
        assert_eq!(invert_color("#FFFFFF"), LIGHT_PEN_INK);
        assert_eq!(invert_color("#ECECEC"), LIGHT_PEN_INK);

        // Dark ink inverts to off-white
        assert_eq!(invert_color(LIGHT_PEN_INK), DARK_PEN_INK);
        assert_eq!(invert_color(DARK_BACKGROUND_COLOR), DARK_PEN_INK);
        assert_eq!(invert_color("#000000"), DARK_PEN_INK);
    }

    #[test]
    fn test_invert_color_preserves_accents() {
        // Curated dark-theme accent swatches must retain color identities
        assert_eq!(invert_color("#FF6B6B"), "#FF6B6B"); // Coral
        assert_eq!(invert_color("#FFD166"), "#FFD166"); // Yellow
        assert_eq!(invert_color("#06D6A0"), "#06D6A0"); // Green
        assert_eq!(invert_color("#38BDF8"), "#38BDF8"); // Cyan
        assert_eq!(invert_color("#A78BFA"), "#A78BFA"); // Lavender
    }

    #[test]
    fn test_invert_element_and_scene() {
        let mut scene = Scene::new();
        scene.push(Element::Rectangle {
            id: 1,
            bounds: Rect::from_xywh(0.0, 0.0, 50.0, 50.0),
            style: ElementStyle {
                stroke_color: DARK_PEN_INK.to_string(),
                fill_color: Some("#FF6B6B".to_string()),
                ..ElementStyle::default()
            },
        });

        invert_scene(&mut scene);

        let elem = scene.get(1).expect("element exists");
        assert_eq!(elem.style().stroke_color, LIGHT_PEN_INK);
        assert_eq!(elem.style().fill_color.as_deref(), Some("#FF6B6B"));

        // Roundtrip inversion restores original off-white ink
        invert_scene(&mut scene);
        let restored = scene.get(1).expect("element exists");
        assert_eq!(restored.style().stroke_color, DARK_PEN_INK);
        assert_eq!(restored.style().fill_color.as_deref(), Some("#FF6B6B"));
    }
}
