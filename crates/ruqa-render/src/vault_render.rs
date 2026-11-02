//! Visual rasterization of the Study Vault drawer sidebar into a [`tiny_skia::Pixmap`].

pub use crate::vault_layout::{VaultAction, VaultItemRow, VaultLayout, VAULT_DEPTH_INDENT};
use std::path::Path;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, PixmapMut, Stroke, Transform};

/// Color styling for the Study Vault drawer sidebar.
#[derive(Debug, Clone, PartialEq)]
pub struct VaultTheme {
    /// Sidebar background fill color.
    pub background: Color,
    /// Right border divider stroke color.
    pub border_color: Color,
    /// Header title text and icon color.
    pub header_icon_color: Color,
    /// Subdued text/icon color for idle rows.
    pub text_dim: Color,
    /// Highlight background color for hovered rows.
    pub hover_bg: Color,
    /// Background color for the currently active open board.
    pub active_bg: Color,
    /// Vibrant accent strip color for active board (Cyan #38BDF8).
    pub active_accent: Color,
    /// Amber tint for folder icons (#FFD166).
    pub folder_color: Color,
    /// Muted slate for board document icons (#94A3B8).
    pub board_color: Color,
}

impl Default for VaultTheme {
    fn default() -> Self {
        Self {
            background: Color::from_rgba8(20, 20, 24, 252),
            border_color: Color::from_rgba8(45, 45, 52, 255),
            header_icon_color: Color::from_rgba8(220, 225, 235, 255),
            text_dim: Color::from_rgba8(140, 145, 155, 255),
            hover_bg: Color::from_rgba8(35, 35, 42, 255),
            active_bg: Color::from_rgba8(32, 44, 60, 255),
            active_accent: Color::from_rgba8(56, 189, 248, 255),
            folder_color: Color::from_rgba8(255, 209, 102, 255),
            board_color: Color::from_rgba8(148, 163, 184, 255),
        }
    }
}

/// Renders the complete Study Vault drawer sidebar into `pixmap`.
pub fn render_vault_sidebar(
    pixmap: &mut PixmapMut<'_>,
    layout: &VaultLayout,
    active_path: Option<&Path>,
    hovered_path: Option<&Path>,
    theme: &VaultTheme,
) {
    let t = Transform::identity();

    // 1. Drawer background
    let mut bg_paint = Paint::default();
    bg_paint.set_color(theme.background);
    if let Some(r) = tiny_skia::Rect::from_xywh(
        layout.drawer_bounds.min.x,
        layout.drawer_bounds.min.y,
        layout.drawer_bounds.width(),
        layout.drawer_bounds.height(),
    ) {
        pixmap.fill_rect(r, &bg_paint, t, None);
    }

    // 2. Right border divider
    let mut border_paint = Paint::default();
    border_paint.set_color(theme.border_color);
    border_paint.anti_alias = true;
    let border_stroke = Stroke {
        width: 1.0,
        ..Default::default()
    };
    let right_x = layout.drawer_bounds.max.x;
    let mut border_pb = PathBuilder::new();
    border_pb.move_to(right_x, 0.0);
    border_pb.line_to(right_x, layout.drawer_bounds.height());
    if let Some(path) = border_pb.finish() {
        pixmap.stroke_path(&path, &border_paint, &border_stroke, t, None);
    }

    // 3. Header bottom separator
    let mut sep_pb = PathBuilder::new();
    sep_pb.move_to(0.0, layout.header_bounds.max.y);
    sep_pb.line_to(right_x, layout.header_bounds.max.y);
    if let Some(sep_path) = sep_pb.finish() {
        pixmap.stroke_path(&sep_path, &border_paint, &border_stroke, t, None);
    }

    // 4. Header action buttons (Close chevron `<` and New Board `+`)
    render_header_buttons(pixmap, layout, theme, t);

    // 5. Entry rows
    for row in &layout.rows {
        let is_active = active_path.is_some_and(|p| p == row.item.path);
        let is_hovered = hovered_path.is_some_and(|p| p == row.item.path);

        // Row background
        if is_active {
            let mut active_paint = Paint::default();
            active_paint.set_color(theme.active_bg);
            if let Some(r) = tiny_skia::Rect::from_xywh(
                row.row_bounds.min.x,
                row.row_bounds.min.y,
                row.row_bounds.width(),
                row.row_bounds.height(),
            ) {
                pixmap.fill_rect(r, &active_paint, t, None);
            }

            // Left accent strip
            let mut accent_paint = Paint::default();
            accent_paint.set_color(theme.active_accent);
            if let Some(accent_r) =
                tiny_skia::Rect::from_xywh(0.0, row.row_bounds.min.y, 3.0, row.row_bounds.height())
            {
                pixmap.fill_rect(accent_r, &accent_paint, t, None);
            }
        } else if is_hovered {
            let mut hover_paint = Paint::default();
            hover_paint.set_color(theme.hover_bg);
            if let Some(r) = tiny_skia::Rect::from_xywh(
                row.row_bounds.min.x,
                row.row_bounds.min.y,
                row.row_bounds.width(),
                row.row_bounds.height(),
            ) {
                pixmap.fill_rect(r, &hover_paint, t, None);
            }
        }

        #[allow(clippy::cast_precision_loss)]
        let indent_x = 12.0 + (row.item.depth as f32) * VAULT_DEPTH_INDENT;
        let mid_y = row.row_bounds.center().y;

        if row.item.is_folder {
            render_folder_row(pixmap, row, indent_x, mid_y, theme, t);
        } else {
            render_board_row(pixmap, indent_x, mid_y, theme, is_active, t);
        }
    }
}

fn render_header_buttons(
    pixmap: &mut PixmapMut<'_>,
    layout: &VaultLayout,
    theme: &VaultTheme,
    t: Transform,
) {
    let mut stroke_paint = Paint::default();
    stroke_paint.set_color(theme.header_icon_color);
    stroke_paint.anti_alias = true;
    let icon_stroke = Stroke {
        width: 1.5,
        ..Default::default()
    };

    // Close button chevron: `<`
    let cb = layout.close_button_bounds;
    let cx = cb.center().x;
    let cy = cb.center().y;
    let mut close_pb = PathBuilder::new();
    close_pb.move_to(cx + 3.0, cy - 6.0);
    close_pb.line_to(cx - 3.0, cy);
    close_pb.line_to(cx + 3.0, cy + 6.0);
    if let Some(close_path) = close_pb.finish() {
        pixmap.stroke_path(&close_path, &stroke_paint, &icon_stroke, t, None);
    }

    // New Board button: `+`
    let nb = layout.new_board_bounds;
    let nx = nb.center().x;
    let ny = nb.center().y;
    let mut plus_pb = PathBuilder::new();
    plus_pb.move_to(nx - 6.0, ny);
    plus_pb.line_to(nx + 6.0, ny);
    plus_pb.move_to(nx, ny - 6.0);
    plus_pb.line_to(nx, ny + 6.0);
    if let Some(plus_path) = plus_pb.finish() {
        pixmap.stroke_path(&plus_path, &stroke_paint, &icon_stroke, t, None);
    }
}

fn render_folder_row(
    pixmap: &mut PixmapMut<'_>,
    row: &VaultItemRow,
    indent_x: f32,
    mid_y: f32,
    theme: &VaultTheme,
    t: Transform,
) {
    let mut chevron_paint = Paint::default();
    chevron_paint.set_color(theme.text_dim);
    chevron_paint.anti_alias = true;

    // Expand/collapse chevron
    let mut ch_pb = PathBuilder::new();
    if row.item.is_expanded {
        // Down-pointing triangle ▼
        ch_pb.move_to(indent_x, mid_y - 3.0);
        ch_pb.line_to(indent_x + 8.0, mid_y - 3.0);
        ch_pb.line_to(indent_x + 4.0, mid_y + 3.0);
    } else {
        // Right-pointing triangle ▶
        ch_pb.move_to(indent_x + 2.0, mid_y - 4.0);
        ch_pb.line_to(indent_x + 8.0, mid_y);
        ch_pb.line_to(indent_x + 2.0, mid_y + 4.0);
    }
    ch_pb.close();
    if let Some(ch_path) = ch_pb.finish() {
        pixmap.fill_path(&ch_path, &chevron_paint, FillRule::Winding, t, None);
    }

    // Folder icon
    let fx = indent_x + 14.0;
    let fy = mid_y - 7.0;
    let mut folder_pb = PathBuilder::new();
    folder_pb.move_to(fx, fy);
    folder_pb.line_to(fx + 5.0, fy);
    folder_pb.line_to(fx + 7.0, fy + 2.0);
    folder_pb.line_to(fx + 14.0, fy + 2.0);
    folder_pb.line_to(fx + 14.0, fy + 13.0);
    folder_pb.line_to(fx, fy + 13.0);
    folder_pb.close();
    if let Some(folder_path) = folder_pb.finish() {
        let mut f_paint = Paint::default();
        f_paint.set_color(theme.folder_color);
        f_paint.anti_alias = true;
        let stroke = Stroke {
            width: 1.2,
            ..Default::default()
        };
        pixmap.stroke_path(&folder_path, &f_paint, &stroke, t, None);
    }
}

fn render_board_row(
    pixmap: &mut PixmapMut<'_>,
    indent_x: f32,
    mid_y: f32,
    theme: &VaultTheme,
    is_active: bool,
    t: Transform,
) {
    let bx = indent_x + 6.0;
    let by = mid_y - 7.0;

    let mut doc_pb = PathBuilder::new();
    doc_pb.move_to(bx, by);
    doc_pb.line_to(bx + 7.0, by);
    doc_pb.line_to(bx + 11.0, by + 4.0);
    doc_pb.line_to(bx + 11.0, by + 14.0);
    doc_pb.line_to(bx, by + 14.0);
    doc_pb.close();

    if let Some(doc_path) = doc_pb.finish() {
        let mut d_paint = Paint::default();
        d_paint.set_color(if is_active {
            theme.active_accent
        } else {
            theme.board_color
        });
        d_paint.anti_alias = true;
        let stroke = Stroke {
            width: 1.2,
            ..Default::default()
        };
        pixmap.stroke_path(&doc_path, &d_paint, &stroke, t, None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruqa_core::vault::FlatVaultItem;
    use std::path::PathBuf;
    use tiny_skia::Pixmap;

    #[test]
    fn test_render_vault_sidebar_produces_pixels() {
        let mut pixmap = Pixmap::new(800, 600).unwrap();
        pixmap.fill(Color::TRANSPARENT);

        let entries = vec![
            FlatVaultItem {
                path: PathBuf::from("/vault/Algorithms"),
                name: "Algorithms".into(),
                is_folder: true,
                depth: 0,
                is_expanded: true,
            },
            FlatVaultItem {
                path: PathBuf::from("/vault/Algorithms/Sort.ruqa"),
                name: "Sort".into(),
                is_folder: false,
                depth: 1,
                is_expanded: false,
            },
        ];

        let layout = VaultLayout::new(600.0, &entries);
        let theme = VaultTheme::default();

        render_vault_sidebar(
            &mut pixmap.as_mut(),
            &layout,
            Some(Path::new("/vault/Algorithms/Sort.ruqa")),
            None,
            &theme,
        );

        let non_transparent = pixmap
            .pixels()
            .iter()
            .filter(|p| p.red() > 0 || p.green() > 0 || p.blue() > 0)
            .count();

        assert!(
            non_transparent > 1000,
            "Vault sidebar must rasterize drawer background and items"
        );
    }
}
