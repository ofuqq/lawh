//! Variable-width freehand pen stroke rasterization.
//!
//! Generates closed outline polygons from smoothed point sequences with velocity-modulated
//! dynamic thickness and smooth rounded end caps, rasterizing ink strokes into [`tiny_skia::PixmapMut`].

use ruqa_core::element::ElementStyle;
use ruqa_core::geometry::Point;
use std::f32::consts::PI;
use tiny_skia::{Color, FillRule, Paint, Path, PathBuilder, PixmapMut, Transform};

/// Reusable scratch buffer for building stroke outline polygons without heap reallocations at 60 FPS.
#[derive(Debug, Default)]
pub struct StrokeOutlineBuilder {
    left_points: Vec<Point>,
    right_points: Vec<Point>,
    cleaned_points: Vec<(Point, f32)>,
}

impl StrokeOutlineBuilder {
    /// Creates a new outline builder with empty initial capacities.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            left_points: Vec::new(),
            right_points: Vec::new(),
            cleaned_points: Vec::new(),
        }
    }

    /// Generates a closed outline polygon [`Path`] enclosing the variable-thickness stroke.
    ///
    /// Reuses internal buffer capacities across frames to maintain 60 FPS without allocations.
    #[must_use]
    pub fn build_outline(
        &mut self,
        points: &[Point],
        pressures: &[f32],
        stroke_width: f32,
    ) -> Option<Path> {
        if points.is_empty() {
            return None;
        }

        self.cleaned_points.clear();
        self.left_points.clear();
        self.right_points.clear();

        // Step 1: Clean and deduplicate consecutive identical points
        let min_radius = 0.5f32;
        let half_base = (stroke_width * 0.5).max(min_radius);

        for (i, &pt) in points.iter().enumerate() {
            let pressure = pressures.get(i).copied().unwrap_or(1.0).clamp(0.05, 2.0);
            let radius = (half_base * pressure).max(min_radius);

            if let Some(&(last_pt, _)) = self.cleaned_points.last() {
                let dx = pt.x - last_pt.x;
                let dy = pt.y - last_pt.y;
                if dx.hypot(dy) < 1e-4 {
                    continue;
                }
            }
            self.cleaned_points.push((pt, radius));
        }

        let n = self.cleaned_points.len();
        if n == 0 {
            return None;
        }
        if n == 1 {
            let (pt, radius) = self.cleaned_points[0];
            return PathBuilder::from_circle(pt.x, pt.y, radius);
        }

        // Step 2: Compute unit tangents and left/right boundary points
        let mut tangents = Vec::with_capacity(n);
        for i in 0..n {
            let t = if i == 0 {
                unit_vector(self.cleaned_points[1].0 - self.cleaned_points[0].0)
            } else if i == n - 1 {
                unit_vector(self.cleaned_points[n - 1].0 - self.cleaned_points[n - 2].0)
            } else {
                let v1 = unit_vector(self.cleaned_points[i].0 - self.cleaned_points[i - 1].0);
                let v2 = unit_vector(self.cleaned_points[i + 1].0 - self.cleaned_points[i].0);
                let v_sum = Point::new(v1.x + v2.x, v1.y + v2.y);
                if v_sum.x.hypot(v_sum.y) > 1e-4 {
                    unit_vector(v_sum)
                } else {
                    v2
                }
            };
            tangents.push(t);
        }

        for (&(pt, radius), &t) in self.cleaned_points.iter().zip(&tangents) {
            // Normal perpendicular to tangent (rotated 90 degrees CCW)
            let normal = Point::new(-t.y, t.x);

            self.left_points.push(Point::new(
                pt.x + normal.x * radius,
                pt.y + normal.y * radius,
            ));
            self.right_points.push(Point::new(
                pt.x - normal.x * radius,
                pt.y - normal.y * radius,
            ));
        }

        // Step 3: Assemble closed path contour with smooth rounded end caps
        // 8 subdivision steps per semicircular cap
        let cap_steps = 8usize;
        let total_verbs = n * 2 + cap_steps * 2 + 2;
        let total_points = total_verbs;
        let mut pb = PathBuilder::with_capacity(total_verbs, total_points);

        let l0 = self.left_points[0];
        pb.move_to(l0.x, l0.y);

        // Forward along left edge
        for &lp in &self.left_points[1..] {
            pb.line_to(lp.x, lp.y);
        }

        // Rounded end cap at the stroke terminus
        let (end_center, end_radius) = self.cleaned_points[n - 1];
        let end_t = tangents[n - 1];
        let end_norm_angle = (-end_t.y).atan2(-end_t.x) + PI; // angle of left normal
        for step in 1..=cap_steps {
            #[allow(clippy::cast_precision_loss)]
            let angle = end_norm_angle - (PI * step as f32 / cap_steps as f32);
            let cx = end_center.x + end_radius * angle.cos();
            let cy = end_center.y + end_radius * angle.sin();
            pb.line_to(cx, cy);
        }

        // Backward along right edge
        for &rp in self.right_points.iter().rev() {
            pb.line_to(rp.x, rp.y);
        }

        // Rounded start cap at the stroke origin
        let (start_center, start_radius) = self.cleaned_points[0];
        let start_t = tangents[0];
        let start_right_angle = (start_t.y).atan2(start_t.x) - PI * 0.5; // angle of right normal
        for step in 1..=cap_steps {
            #[allow(clippy::cast_precision_loss)]
            let angle = start_right_angle - (PI * step as f32 / cap_steps as f32);
            let cx = start_center.x + start_radius * angle.cos();
            let cy = start_center.y + start_radius * angle.sin();
            pb.line_to(cx, cy);
        }

        pb.close();
        pb.finish()
    }
}

/// Helper function to normalize a 2D delta into a unit direction vector.
#[must_use]
fn unit_vector(delta: Point) -> Point {
    let len = delta.x.hypot(delta.y);
    if len > 1e-5 {
        Point::new(delta.x / len, delta.y / len)
    } else {
        Point::new(1.0, 0.0)
    }
}

/// Converts a sequence of points and dynamic pressures into a closed variable-width outline [`Path`].
#[must_use]
pub fn stroke_to_outline_polygon(
    points: &[Point],
    pressures: &[f32],
    stroke_width: f32,
) -> Option<Path> {
    let mut builder = StrokeOutlineBuilder::new();
    builder.build_outline(points, pressures, stroke_width)
}

/// Rasterizes a variable-width freehand stroke into a [`tiny_skia::PixmapMut`].
pub fn render_pen_stroke(
    points: &[Point],
    pressures: &[f32],
    style: &ElementStyle,
    pixmap: &mut PixmapMut<'_>,
    transform: Transform,
) {
    let Some(path) = stroke_to_outline_polygon(points, pressures, style.stroke_width) else {
        return;
    };

    let color = crate::shape_render::parse_hex_color(&style.stroke_color)
        .unwrap_or_else(|| Color::from_rgba8(224, 224, 224, 255));

    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    pixmap.fill_path(&path, &paint, FillRule::Winding, transform, None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_skia::Pixmap;

    #[test]
    fn test_empty_stroke_produces_none() {
        assert!(stroke_to_outline_polygon(&[], &[], 2.0).is_none());
    }

    #[test]
    fn test_single_point_stroke_produces_circle_dab() {
        let path = stroke_to_outline_polygon(&[Point::new(50.0, 50.0)], &[1.0], 4.0)
            .expect("single point stroke should yield path");
        let bounds = path.bounds();
        assert!((bounds.left() - 48.0).abs() < 1e-2);
        assert!((bounds.right() - 52.0).abs() < 1e-2);
    }

    #[test]
    fn test_two_point_horizontal_stroke_caps() {
        let pts = [Point::new(10.0, 20.0), Point::new(100.0, 20.0)];
        let press = [1.0, 1.0];
        let path = stroke_to_outline_polygon(&pts, &press, 6.0)
            .expect("valid stroke should yield outline path");

        let bounds = path.bounds();
        // Base half-width = 3.0. Capsule should extend from x = 10 - 3 = 7 to x = 100 + 3 = 103
        assert!((bounds.left() - 7.0).abs() < 0.5);
        assert!((bounds.right() - 103.0).abs() < 0.5);
        assert!((bounds.top() - 17.0).abs() < 0.5);
        assert!((bounds.bottom() - 23.0).abs() < 0.5);
    }

    #[test]
    fn test_variable_width_pressure_renders_to_pixmap() {
        let mut pixmap = Pixmap::new(200, 200).expect("pixmap creation succeeds");
        pixmap.fill(Color::TRANSPARENT);

        let points = [
            Point::new(20.0, 100.0),
            Point::new(80.0, 100.0),
            Point::new(140.0, 100.0),
        ];
        let pressures = [0.2, 1.0, 0.4];
        let style = ElementStyle {
            stroke_color: "#FF5500".into(),
            stroke_width: 8.0,
            ..Default::default()
        };

        render_pen_stroke(
            &points,
            &pressures,
            &style,
            &mut pixmap.as_mut(),
            Transform::identity(),
        );

        // Center pixel at (80, 100) should be colored
        let center_pixel = pixmap.pixel(80, 100).expect("pixel within bounds");
        assert_eq!(center_pixel.red(), 255);
        assert_eq!(center_pixel.green(), 85);
        assert_eq!(center_pixel.blue(), 0);
        assert_eq!(center_pixel.alpha(), 255);
    }
}
