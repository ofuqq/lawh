//! Orientation grid rasterization for infinite canvas background.

use ruqa_core::geometry::{Point, Size, Viewport};
use tiny_skia::PixmapMut;

/// Visual rendering style for the canvas orientation grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GridStyle {
    /// Subtle dots rendered at grid coordinate intersections (default).
    #[default]
    Dots,
    /// Subtle graph lines rendered along grid coordinates.
    GraphLines,
    /// Grid display disabled.
    None,
}

/// Configuration parameters for the canvas orientation grid.
#[derive(Debug, Clone, PartialEq)]
pub struct GridConfig {
    /// Base grid interval in canvas world units (default: 25.0).
    pub spacing: f32,
    /// Visual representation (dots, lines, or none).
    pub style: GridStyle,
    /// Grid color formatted as `(R, G, B, A)` bytes.
    pub color: (u8, u8, u8, u8),
    /// Dot or line stroke dimension in screen pixels.
    pub size: usize,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            spacing: 25.0,
            style: GridStyle::Dots,
            // Subtle dark-gray dots (#2D2D2D) on matte dark canvas (#121212)
            color: (45, 45, 45, 255),
            size: 1,
        }
    }
}

impl GridConfig {
    /// Returns default grid configuration for light mode (#E1E1E1 on #FFFFFF).
    #[must_use]
    pub fn light_mode() -> Self {
        Self {
            spacing: 25.0,
            style: GridStyle::Dots,
            color: (225, 225, 225, 255),
            size: 1,
        }
    }

    /// Returns default grid configuration for dark mode (#2D2D2D on #121212).
    #[must_use]
    pub fn dark_mode() -> Self {
        Self::default()
    }
}

/// Renders the canvas orientation grid into `pixmap` based on current viewport camera.
pub fn render_grid(
    pixmap: &mut PixmapMut<'_>,
    viewport: &Viewport,
    window_size: Size,
    config: &GridConfig,
) {
    render_grid_scaled(pixmap, viewport, window_size, 1.0, config);
}

/// Renders the canvas orientation grid into `pixmap` with HiDPI scale factor support.
pub fn render_grid_scaled(
    pixmap: &mut PixmapMut<'_>,
    viewport: &Viewport,
    window_size: Size,
    scale_factor: f32,
    config: &GridConfig,
) {
    if config.style == GridStyle::None {
        return;
    }

    let zoom = viewport.zoom;
    let base_spacing = config.spacing.max(5.0);

    // Adaptive step multiplier to maintain readable screen-space density
    let mut step = base_spacing;
    while step * zoom < 14.0 {
        step *= 2.0;
    }
    while step * zoom > 90.0 {
        step /= 2.0;
    }

    let visible = viewport.visible_world_rect(window_size);
    let start_x = (visible.min.x / step).floor() * step;
    let end_x = (visible.max.x / step).ceil() * step;
    let start_y = (visible.min.y / step).floor() * step;
    let end_y = (visible.max.y / step).ceil() * step;

    let width = pixmap.width() as usize;
    let height = pixmap.height() as usize;
    let data = pixmap.data_mut();
    let (r, g, b, a) = config.color;

    match config.style {
        GridStyle::Dots => {
            #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
            let dot_size = ((config.size as f32 * scale_factor).round() as isize).max(1);
            let mut y = start_y;
            while y <= end_y {
                let mut x = start_x;
                while x <= end_x {
                    let screen_pt = viewport.world_to_screen(Point::new(x, y), window_size);
                    #[allow(clippy::cast_possible_truncation)]
                    let sx = (screen_pt.x * scale_factor).round() as isize;
                    #[allow(clippy::cast_possible_truncation)]
                    let sy = (screen_pt.y * scale_factor).round() as isize;

                    for dy in 0..dot_size {
                        for dx in 0..dot_size {
                            let px = sx + dx;
                            let py = sy + dy;
                            if px >= 0 && (px as usize) < width && py >= 0 && (py as usize) < height
                            {
                                #[allow(clippy::cast_sign_loss)]
                                let offset = ((py as usize) * width + (px as usize)) * 4;
                                data[offset] = r;
                                data[offset + 1] = g;
                                data[offset + 2] = b;
                                data[offset + 3] = a;
                            }
                        }
                    }
                    x += step;
                }
                y += step;
            }
        }
        GridStyle::GraphLines => {
            // Vertical graph lines
            #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
            let line_width = ((config.size as f32 * scale_factor).round() as isize).max(1);
            let mut x = start_x;
            while x <= end_x {
                let screen_pt = viewport.world_to_screen(Point::new(x, 0.0), window_size);
                #[allow(clippy::cast_possible_truncation)]
                let sx = (screen_pt.x * scale_factor).round() as isize;
                for lw in 0..line_width {
                    let px = sx + lw;
                    if px >= 0 && (px as usize) < width {
                        for py in 0..height {
                            let offset = (py * width + (px as usize)) * 4;
                            data[offset] = r;
                            data[offset + 1] = g;
                            data[offset + 2] = b;
                            data[offset + 3] = a;
                        }
                    }
                }
                x += step;
            }

            // Horizontal graph lines
            #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
            let line_width = ((config.size as f32 * scale_factor).round() as isize).max(1);
            let mut y = start_y;
            while y <= end_y {
                let screen_pt = viewport.world_to_screen(Point::new(0.0, y), window_size);
                #[allow(clippy::cast_possible_truncation)]
                let sy = (screen_pt.y * scale_factor).round() as isize;
                for lw in 0..line_width {
                    let py = sy + lw;
                    if py >= 0 && (py as usize) < height {
                        let row_start = (py as usize) * width * 4;
                        for px in 0..width {
                            let offset = row_start + px * 4;
                            data[offset] = r;
                            data[offset + 1] = g;
                            data[offset + 2] = b;
                            data[offset + 3] = a;
                        }
                    }
                }
                y += step;
            }
        }
        GridStyle::None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_skia::{Color, Pixmap};

    #[test]
    fn test_grid_config_defaults() {
        let default_config = GridConfig::default();
        assert_eq!(default_config.style, GridStyle::Dots);
        assert_eq!(default_config.spacing, 25.0);
        assert_eq!(default_config.color, (45, 45, 45, 255));
    }

    #[test]
    fn test_render_dots_grid_produces_pixels() {
        let mut pixmap = Pixmap::new(200, 200).expect("pixmap creation succeeds");
        pixmap.fill(Color::BLACK);

        let viewport = Viewport::default();
        let window_size = Size::new(200.0, 200.0);
        let config = GridConfig::default();

        render_grid(&mut pixmap.as_mut(), &viewport, window_size, &config);

        // Count pixels that were set to grid color (45, 45, 45)
        let dot_pixels = pixmap
            .data()
            .chunks_exact(4)
            .filter(|chunk| chunk[0] == 45 && chunk[1] == 45 && chunk[2] == 45)
            .count();

        assert!(dot_pixels > 0, "dot grid must render visible pixels");
    }

    #[test]
    fn test_render_graph_lines_grid() {
        let mut pixmap = Pixmap::new(200, 200).expect("pixmap creation succeeds");
        pixmap.fill(Color::BLACK);

        let viewport = Viewport::default();
        let window_size = Size::new(200.0, 200.0);
        let config = GridConfig {
            style: GridStyle::GraphLines,
            color: (60, 60, 60, 255),
            ..GridConfig::default()
        };

        render_grid(&mut pixmap.as_mut(), &viewport, window_size, &config);

        let line_pixels = pixmap
            .data()
            .chunks_exact(4)
            .filter(|chunk| chunk[0] == 60)
            .count();

        assert!(line_pixels > 200, "graph lines must span multiple lines");
    }

    #[test]
    fn test_render_grid_none_is_noop() {
        let mut pixmap = Pixmap::new(100, 100).expect("pixmap creation succeeds");
        pixmap.fill(Color::TRANSPARENT);

        let viewport = Viewport::default();
        let window_size = Size::new(100.0, 100.0);
        let config = GridConfig {
            style: GridStyle::None,
            ..GridConfig::default()
        };

        render_grid(&mut pixmap.as_mut(), &viewport, window_size, &config);

        let non_transparent = pixmap.data().iter().filter(|&&b| b != 0).count();
        assert_eq!(
            non_transparent, 0,
            "disabled grid must not alter any pixels"
        );
    }
}
