//! Freehand stroke smoothing and velocity-based pressure modulation.
//!
//! Provides spline interpolation (Catmull-Rom) and dynamic pressure simulation
//! based on pointer velocity. Fast cursor movements yield thinner strokes,
//! while slow deliberate movements yield thicker strokes.

use crate::geometry::Point;

/// A raw timestamped pointer input sample in world coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InputPoint {
    /// World position coordinate.
    pub point: Point,
    /// Timestamp in milliseconds.
    pub timestamp_ms: f64,
}

impl InputPoint {
    /// Creates a new timestamped input point.
    #[must_use]
    pub const fn new(x: f32, y: f32, timestamp_ms: f64) -> Self {
        Self {
            point: Point::new(x, y),
            timestamp_ms,
        }
    }
}

/// Evaluates a 2D Catmull-Rom spline point between `p1` and `p2` at parameter `t` in `[0.0, 1.0]`.
#[must_use]
pub fn catmull_rom(p0: Point, p1: Point, p2: Point, p3: Point, t: f32) -> Point {
    let t2 = t * t;
    let t3 = t2 * t;

    let x = 0.5
        * ((2.0 * p1.x)
            + (-p0.x + p2.x) * t
            + (2.0 * p0.x - 5.0 * p1.x + 4.0 * p2.x - p3.x) * t2
            + (-p0.x + 3.0 * p1.x - 3.0 * p2.x + p3.x) * t3);

    let y = 0.5
        * ((2.0 * p1.y)
            + (-p0.y + p2.y) * t
            + (2.0 * p0.y - 5.0 * p1.y + 4.0 * p2.y - p3.y) * t2
            + (-p0.y + 3.0 * p1.y - 3.0 * p2.y + p3.y) * t3);

    Point::new(x, y)
}

/// Dynamic stroke smoother accumulating raw pointer inputs and producing smoothed strokes.
#[derive(Debug, Clone)]
pub struct StrokeSmoother {
    /// Raw input buffer.
    inputs: Vec<InputPoint>,
    /// Minimum simulated pressure factor.
    pub min_pressure: f32,
    /// Maximum simulated pressure factor.
    pub max_pressure: f32,
    /// Speed threshold (pixels/ms) corresponding to minimum pressure.
    pub max_velocity: f32,
}

impl Default for StrokeSmoother {
    fn default() -> Self {
        Self::new()
    }
}

impl StrokeSmoother {
    /// Creates a new smoother with default velocity and pressure parameters.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inputs: Vec::new(),
            min_pressure: 0.25,
            max_pressure: 1.0,
            max_velocity: 2.5,
        }
    }

    /// Adds a raw timestamped input point to the stroke.
    pub fn add_point(&mut self, point: InputPoint) {
        if let Some(last) = self.inputs.last() {
            // Ignore redundant duplicate samples at identical positions
            if (last.point.x - point.point.x).abs() < 1e-4
                && (last.point.y - point.point.y).abs() < 1e-4
            {
                return;
            }
        }
        self.inputs.push(point);
    }

    /// Returns the number of accumulated raw points.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    /// Returns `true` if no points have been accumulated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }

    /// Clears all accumulated input points.
    pub fn clear(&mut self) {
        self.inputs.clear();
    }

    /// Computes smoothed points and velocity-modulated pressures.
    #[must_use]
    pub fn finish(&self) -> (Vec<Point>, Vec<f32>) {
        smooth_stroke(
            &self.inputs,
            self.min_pressure,
            self.max_pressure,
            self.max_velocity,
        )
    }
}

/// Smooths a sequence of timestamped input points using velocity modulation and Catmull-Rom splines.
#[must_use]
pub fn smooth_stroke(
    inputs: &[InputPoint],
    min_pressure: f32,
    max_pressure: f32,
    max_velocity: f32,
) -> (Vec<Point>, Vec<f32>) {
    if inputs.is_empty() {
        return (Vec::new(), Vec::new());
    }
    if inputs.len() == 1 {
        return (vec![inputs[0].point], vec![max_pressure]);
    }

    // Step 1: Compute instantaneous velocities and smoothed pressures at each raw point
    let mut raw_pressures = Vec::with_capacity(inputs.len());
    raw_pressures.push(max_pressure);

    let mut current_pressure = max_pressure;
    for i in 1..inputs.len() {
        let p_prev = inputs[i - 1];
        let p_curr = inputs[i];

        let dist = p_curr.point.distance(p_prev.point);
        let dt = (p_curr.timestamp_ms - p_prev.timestamp_ms).max(1.0) as f32;
        let speed = dist / dt; // pixels per millisecond

        let velocity_ratio = (speed / max_velocity).clamp(0.0, 1.0);
        let target_pressure = max_pressure - velocity_ratio * (max_pressure - min_pressure);

        // Exponential moving average filter for smooth pressure transitions
        current_pressure += 0.4 * (target_pressure - current_pressure);
        raw_pressures.push(current_pressure);
    }

    if inputs.len() == 2 {
        return (vec![inputs[0].point, inputs[1].point], raw_pressures);
    }

    // Step 2: Interpolate along Catmull-Rom spline curves between control points
    let mut smoothed_points = Vec::with_capacity(inputs.len() * 3);
    let mut smoothed_pressures = Vec::with_capacity(inputs.len() * 3);

    for i in 0..(inputs.len() - 1) {
        let p0 = if i == 0 {
            inputs[0].point
        } else {
            inputs[i - 1].point
        };
        let p1 = inputs[i].point;
        let p2 = inputs[i + 1].point;
        let p3 = if i + 2 < inputs.len() {
            inputs[i + 2].point
        } else {
            p2
        };

        let press1 = raw_pressures[i];
        let press2 = raw_pressures[i + 1];

        let seg_dist = p1.distance(p2);
        // Subdivide segment if distance is large (step ~5px)
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let steps = (seg_dist / 5.0).ceil().clamp(1.0, 16.0) as usize;

        for step in 0..steps {
            let t = step as f32 / steps as f32;
            let pt = catmull_rom(p0, p1, p2, p3, t);
            let press = press1 + t * (press2 - press1);

            smoothed_points.push(pt);
            smoothed_pressures.push(press);
        }
    }

    // Include the final endpoint
    if let Some(last) = inputs.last() {
        smoothed_points.push(last.point);
        smoothed_pressures.push(*raw_pressures.last().unwrap_or(&max_pressure));
    }

    (smoothed_points, smoothed_pressures)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_and_single_point_stroke() {
        let smoother = StrokeSmoother::new();
        let (pts, press) = smoother.finish();
        assert!(pts.is_empty());
        assert!(press.is_empty());

        let mut smoother = StrokeSmoother::new();
        smoother.add_point(InputPoint::new(10.0, 20.0, 1000.0));
        let (pts, press) = smoother.finish();
        assert_eq!(pts.len(), 1);
        assert_eq!(press.len(), 1);
        assert_eq!(pts[0], Point::new(10.0, 20.0));
    }

    #[test]
    fn test_velocity_pressure_modulation() {
        let mut slow_smoother = StrokeSmoother::new();
        // Slow movement: 10 pixels over 500 ms (0.02 px/ms)
        slow_smoother.add_point(InputPoint::new(0.0, 0.0, 0.0));
        slow_smoother.add_point(InputPoint::new(10.0, 0.0, 500.0));
        let (_, slow_press) = slow_smoother.finish();

        let mut fast_smoother = StrokeSmoother::new();
        // Fast movement: 100 pixels over 10 ms (10.0 px/ms)
        fast_smoother.add_point(InputPoint::new(0.0, 0.0, 0.0));
        fast_smoother.add_point(InputPoint::new(100.0, 0.0, 10.0));
        let (_, fast_press) = fast_smoother.finish();

        // Fast stroke must produce lower pressure (thinner line) than slow stroke
        assert!(
            fast_press.last().unwrap() < slow_press.last().unwrap(),
            "fast pressure ({:?}) should be lower than slow pressure ({:?})",
            fast_press.last(),
            slow_press.last()
        );
    }

    #[test]
    fn test_catmull_rom_smoothing_interpolates_points() {
        let mut smoother = StrokeSmoother::new();
        // 4 points forming an L-shape
        smoother.add_point(InputPoint::new(0.0, 0.0, 0.0));
        smoother.add_point(InputPoint::new(50.0, 0.0, 50.0));
        smoother.add_point(InputPoint::new(100.0, 0.0, 100.0));
        smoother.add_point(InputPoint::new(100.0, 50.0, 150.0));

        let (points, pressures) = smoother.finish();
        assert!(
            points.len() > 4,
            "spline interpolation should add intermediate points"
        );
        assert_eq!(points.len(), pressures.len());
        assert_eq!(points.first().unwrap(), &Point::new(0.0, 0.0));
        assert_eq!(points.last().unwrap(), &Point::new(100.0, 50.0));
    }
}
