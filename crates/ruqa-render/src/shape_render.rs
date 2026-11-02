//! Hand-drawn rough shape rasterizer for geometric whiteboard elements.
//!
//! Generates organic, multi-pass vector paths from [`Element`] models using `roughr`
//! and rasterizes them into a [`tiny_skia::PixmapMut`] surface with deterministic
//! seed stability across frames.

use crate::rough_convert::{default_rough_stroke, drawable_to_paths};
use roughr::core::OptionsBuilder;
use roughr::generator::Generator;
use roughr::Point2D;
use ruqa_core::element::Element;
use ruqa_core::geometry::{Size, Viewport};
use ruqa_core::scene::Scene;
use tiny_skia::{Color, Paint, Path, PixmapMut, Transform};

/// Parses a hex color string (`"#RRGGBB"` or `"#RRGGBBAA"`) into a `tiny_skia::Color`.
///
/// Falls back to `None` if the hex string is malformed or invalid.
#[must_use]
pub fn parse_hex_color(hex: &str) -> Option<Color> {
    let s = hex.strip_prefix('#').unwrap_or(hex);
    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some(Color::from_rgba8(r, g, b, 255))
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            let a = u8::from_str_radix(&s[6..8], 16).ok()?;
            Some(Color::from_rgba8(r, g, b, a))
        }
        _ => None,
    }
}

/// Generates vector [`Path`]s for an [`Element`] using deterministic seed options.
///
/// The returned paths are completely stable across calls as long as the element's
/// geometry and PRNG seed remain unchanged.
#[must_use]
pub fn generate_element_paths(element: &Element) -> Vec<Path> {
    let generator = Generator::default();
    let style = element.style();

    let options = OptionsBuilder::default()
        .seed(style.seed)
        .roughness(style.roughness)
        .stroke_width(style.stroke_width)
        .build()
        .ok();

    match element {
        Element::Rectangle { bounds, .. } => {
            let drawable = generator.rectangle(
                bounds.min.x,
                bounds.min.y,
                bounds.width(),
                bounds.height(),
                &options,
            );
            drawable_to_paths(&drawable)
        }
        Element::Ellipse { bounds, .. } => {
            let center = bounds.center();
            let drawable = generator.ellipse(
                center.x,
                center.y,
                bounds.width(),
                bounds.height(),
                &options,
            );
            drawable_to_paths(&drawable)
        }
        Element::Diamond { bounds, .. } => {
            let center = bounds.center();
            let points = [
                Point2D::new(center.x, bounds.min.y),
                Point2D::new(bounds.max.x, center.y),
                Point2D::new(center.x, bounds.max.y),
                Point2D::new(bounds.min.x, center.y),
            ];
            let drawable = generator.polygon(&points, &options);
            drawable_to_paths(&drawable)
        }
        Element::Line { start, end, .. } => {
            let drawable = generator.line(start.x, start.y, end.x, end.y, &options);
            drawable_to_paths(&drawable)
        }
        Element::Arrow { start, end, .. } => {
            let mut paths = Vec::new();
            // Main shaft line
            let shaft = generator.line(start.x, start.y, end.x, end.y, &options);
            paths.extend(drawable_to_paths(&shaft));

            // Arrowhead barbs
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let length = dx.hypot(dy);
            if length > 1e-4 {
                let angle = dy.atan2(dx);
                let barb_len = (12.0 + style.stroke_width * 2.0).min(length * 0.5);
                let barb_angle = std::f32::consts::FRAC_PI_6; // 30 degrees in radians

                let a1 = angle - std::f32::consts::PI + barb_angle;
                let a2 = angle - std::f32::consts::PI - barb_angle;

                let p1 = (end.x + barb_len * a1.cos(), end.y + barb_len * a1.sin());
                let p2 = (end.x + barb_len * a2.cos(), end.y + barb_len * a2.sin());

                let barb1 = generator.line(end.x, end.y, p1.0, p1.1, &options);
                let barb2 = generator.line(end.x, end.y, p2.0, p2.1, &options);
                paths.extend(drawable_to_paths(&barb1));
                paths.extend(drawable_to_paths(&barb2));
            }
            paths
        }
        Element::Stroke {
            points,
            pressures,
            style,
            ..
        } => crate::pen_render::stroke_to_outline_polygon(points, pressures, style.stroke_width)
            .into_iter()
            .collect(),
    }
}

/// Renders a single whiteboard element to a pixmap surface with a 2D coordinate transform.
pub fn render_element(element: &Element, pixmap: &mut PixmapMut<'_>, transform: Transform) {
    match element {
        Element::Stroke {
            points,
            pressures,
            style,
            ..
        } => {
            crate::pen_render::render_pen_stroke(points, pressures, style, pixmap, transform);
        }
        _ => {
            let style = element.style();
            let color = parse_hex_color(&style.stroke_color)
                .unwrap_or_else(|| Color::from_rgba8(224, 224, 224, 255));

            let mut paint = Paint::default();
            paint.set_color(color);
            paint.anti_alias = true;

            let stroke = default_rough_stroke(style.stroke_width);
            let paths = generate_element_paths(element);

            for path in &paths {
                pixmap.stroke_path(path, &paint, &stroke, transform, None);
            }
        }
    }
}

/// Renders all elements in a scene graph to a pixmap surface.
pub fn render_scene(scene: &Scene, pixmap: &mut PixmapMut<'_>, transform: Transform) {
    for element in scene.elements() {
        render_element(element, pixmap, transform);
    }
}

/// Renders visible elements from a scene within the viewport camera view.
///
/// Applies coarse spatial culling to skip elements outside the viewport, and transforms
/// world coordinates into window screen pixels.
pub fn render_scene_viewport(
    scene: &Scene,
    viewport: &Viewport,
    window_size: Size,
    pixmap: &mut PixmapMut<'_>,
) {
    let visible_rect = viewport.visible_world_rect(window_size);
    let center = window_size.center();
    let zoom = viewport.zoom;

    let transform = Transform::from_row(
        zoom,
        0.0,
        0.0,
        zoom,
        center.x - viewport.pan.x * zoom,
        center.y - viewport.pan.y * zoom,
    );

    for element in scene.visible_elements(visible_rect) {
        render_element(element, pixmap, transform);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruqa_core::element::ElementStyle;
    use ruqa_core::geometry::Rect;
    use tiny_skia::Pixmap;

    #[test]
    fn test_parse_hex_color() {
        let white = parse_hex_color("#FFFFFF").unwrap();
        assert_eq!(white, Color::from_rgba8(255, 255, 255, 255));

        let cyan_no_hash = parse_hex_color("4DD0E1").unwrap();
        assert_eq!(cyan_no_hash, Color::from_rgba8(77, 208, 225, 255));

        let with_alpha = parse_hex_color("#12345680").unwrap();
        assert_eq!(with_alpha, Color::from_rgba8(0x12, 0x34, 0x56, 0x80));

        assert!(parse_hex_color("invalid").is_none());
        assert!(parse_hex_color("#12345").is_none());
    }

    #[test]
    fn test_stable_seed_produces_identical_renders() {
        let element = Element::Rectangle {
            id: 1,
            bounds: Rect::from_xywh(20.0, 20.0, 100.0, 60.0),
            style: ElementStyle {
                stroke_color: "#FFFFFF".into(),
                stroke_width: 2.0,
                roughness: 1.5,
                seed: 42_4242,
                ..Default::default()
            },
        };

        let mut p1 = Pixmap::new(200, 150).unwrap();
        let mut p2 = Pixmap::new(200, 150).unwrap();

        render_element(&element, &mut p1.as_mut(), Transform::identity());
        render_element(&element, &mut p2.as_mut(), Transform::identity());

        assert_eq!(
            p1.data(),
            p2.data(),
            "renders with identical seed must match pixel-for-pixel"
        );
    }

    #[test]
    fn test_differing_seeds_produce_different_paths() {
        let e1 = Element::Rectangle {
            id: 1,
            bounds: Rect::from_xywh(20.0, 20.0, 100.0, 60.0),
            style: ElementStyle {
                roughness: 1.5,
                seed: 100,
                ..Default::default()
            },
        };
        let e2 = Element::Rectangle {
            id: 1,
            bounds: Rect::from_xywh(20.0, 20.0, 100.0, 60.0),
            style: ElementStyle {
                roughness: 1.5,
                seed: 200,
                ..Default::default()
            },
        };

        let mut p1 = Pixmap::new(200, 150).unwrap();
        let mut p2 = Pixmap::new(200, 150).unwrap();

        render_element(&e1, &mut p1.as_mut(), Transform::identity());
        render_element(&e2, &mut p2.as_mut(), Transform::identity());

        assert_ne!(
            p1.data(),
            p2.data(),
            "renders with differing seeds should not be identical"
        );
    }
}
