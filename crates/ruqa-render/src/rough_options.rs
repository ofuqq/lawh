//! Helper module for translating [`ElementStyle`] models into [`roughr::core::Options`].

use crate::shape_render::parse_hex_color;
use roughr::core::{
    Drawable, FillStyle as RoughFillStyle, LineCap, LineJoin, Options, OptionsBuilder,
};
use roughr::generator::Generator;
use roughr::Point2D;
use roughr::Srgba;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::style::FillStyle;
use tiny_skia::Color;

/// Constructs [`roughr::core::Options`] from a whiteboard [`ElementStyle`].
///
/// Sets stroke roughness, seed, width, and maps fill patterns (solid, semi-transparent,
/// hachure, and cross-hatch) to the underlying Rough algorithm parameters.
#[must_use]
pub fn build_rough_options(style: &ElementStyle) -> Options {
    let mut builder = OptionsBuilder::default();
    builder.seed(style.seed);
    builder.roughness(style.roughness);
    builder.stroke_width(style.stroke_width);

    // Hand-drawn look: round caps and joins, light-gray fallback for bad colors.
    let stroke_color = parse_hex_color(&style.stroke_color)
        .unwrap_or_else(|| Color::from_rgba8(224, 224, 224, 255));
    builder.stroke(Srgba::new(
        stroke_color.red(),
        stroke_color.green(),
        stroke_color.blue(),
        stroke_color.alpha(),
    ));
    builder.line_cap(LineCap::Round);
    builder.line_join(LineJoin::Round);

    // Hachure gap and fill weight scale organically with stroke width
    let gap = (style.stroke_width * 4.0).max(4.0);
    let weight = (style.stroke_width * 0.6).max(1.0);
    builder.hachure_gap(gap);
    builder.fill_weight(weight);
    builder.hachure_angle(-41.0);

    if let Some(fill_color) = resolved_fill_color(style) {
        let srgba = Srgba::new(
            fill_color.red(),
            fill_color.green(),
            fill_color.blue(),
            fill_color.alpha(),
        );
        builder.fill(srgba);

        let rough_fill_style = match style.fill_style {
            FillStyle::None => None,
            FillStyle::Solid | FillStyle::SemiTransparent => Some(RoughFillStyle::Solid),
            FillStyle::Hachure => Some(RoughFillStyle::Hachure),
            FillStyle::CrossHatch => Some(RoughFillStyle::CrossHatch),
            FillStyle::Dots => Some(RoughFillStyle::Dots),
        };

        if let Some(fs) = rough_fill_style {
            builder.fill_style(fs);
        }
    }

    builder.build().unwrap_or_default()
}

/// Resolves the effective [`tiny_skia::Color`] for an element's fill, applying opacity adjustments.
#[must_use]
pub fn resolved_fill_color(style: &ElementStyle) -> Option<Color> {
    if style.fill_style == FillStyle::None {
        return None;
    }

    let hex = style.fill_color.as_deref()?;
    let base = parse_hex_color(hex)?;

    if style.fill_style == FillStyle::SemiTransparent {
        Color::from_rgba(base.red(), base.green(), base.blue(), base.alpha() * 0.5)
    } else {
        Some(base)
    }
}

/// Constructs [`roughr::core::Drawable`] representation(s) of an [`Element`].
#[must_use]
pub fn generate_element_drawables(
    element: &Element,
    options: &roughr::core::Options,
) -> Vec<Drawable<f32>> {
    let generator = Generator::default();
    let opt = Some(options.clone());

    match element {
        Element::Rectangle { bounds, .. } => {
            vec![generator.rectangle(
                bounds.min.x,
                bounds.min.y,
                bounds.width(),
                bounds.height(),
                &opt,
            )]
        }
        Element::Ellipse { bounds, .. } => {
            let center = bounds.center();
            vec![generator.ellipse(center.x, center.y, bounds.width(), bounds.height(), &opt)]
        }
        Element::Diamond { bounds, .. } => {
            let center = bounds.center();
            let points = [
                Point2D::new(center.x, bounds.min.y),
                Point2D::new(bounds.max.x, center.y),
                Point2D::new(center.x, bounds.max.y),
                Point2D::new(bounds.min.x, center.y),
            ];
            vec![generator.polygon(&points, &opt)]
        }
        Element::Line { start, end, .. } => {
            vec![generator.line(start.x, start.y, end.x, end.y, &opt)]
        }
        Element::Arrow {
            start, end, style, ..
        } => {
            let mut drawables = vec![generator.line(start.x, start.y, end.x, end.y, &opt)];
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let length = dx.hypot(dy);
            if length > 1e-4 {
                let angle = dy.atan2(dx);
                let barb_len = (12.0 + style.stroke_width * 2.0).min(length * 0.5);
                let barb_angle = std::f32::consts::FRAC_PI_6;
                let a1 = angle - std::f32::consts::PI + barb_angle;
                let a2 = angle - std::f32::consts::PI - barb_angle;

                let p1 = (end.x + barb_len * a1.cos(), end.y + barb_len * a1.sin());
                let p2 = (end.x + barb_len * a2.cos(), end.y + barb_len * a2.sin());

                drawables.push(generator.line(end.x, end.y, p1.0, p1.1, &opt));
                drawables.push(generator.line(end.x, end.y, p2.0, p2.1, &opt));
            }
            drawables
        }
        Element::Stroke { .. } | Element::Text { .. } => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_without_fill() {
        let style = ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: None,
            fill_style: FillStyle::None,
            roughness: 1.2,
            seed: 99,
        };
        let options = build_rough_options(&style);
        assert_eq!(options.seed, Some(99));
        assert_eq!(options.roughness, Some(1.2));
        assert_eq!(options.stroke_width, Some(2.0));
        assert!(options.fill.is_none());
    }

    #[test]
    fn test_options_with_hachure_fill() {
        let style = ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#FF6B6B".to_string()),
            fill_style: FillStyle::Hachure,
            roughness: 1.5,
            seed: 42,
        };
        let options = build_rough_options(&style);
        assert!(options.fill.is_some());
        assert_eq!(options.fill_style, Some(RoughFillStyle::Hachure));
        assert_eq!(options.roughness, Some(1.5));
    }

    #[test]
    fn test_options_with_cross_hatch_fill() {
        let style = ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#38BDF8".to_string()),
            fill_style: FillStyle::CrossHatch,
            roughness: 1.0,
            seed: 777,
        };
        let options = build_rough_options(&style);
        assert!(options.fill.is_some());
        assert_eq!(options.fill_style, Some(RoughFillStyle::CrossHatch));
    }

    #[test]
    fn test_semi_transparent_fill_opacity() {
        let style = ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#FFFFFF".to_string()), // Full white
            fill_style: FillStyle::SemiTransparent,
            roughness: 0.0,
            seed: 1,
        };
        let color = resolved_fill_color(&style).expect("color resolved");
        assert!((color.alpha() - 0.5).abs() < 1e-3);
    }
}
