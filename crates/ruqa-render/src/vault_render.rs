//! Visual rasterization of the Study Vault drawer sidebar into a [`tiny_skia::Pixmap`].

use crate::text_render::render_ui_text;
use crate::vault_icons::{render_board_row, render_folder_row, render_header_buttons};
pub use crate::vault_layout::{
    VaultAction, VaultItemRow, VaultLayout, VAULT_DEPTH_INDENT, VAULT_DRAWER_WIDTH,
    VAULT_HEADER_HEIGHT,
};
use std::path::Path;
use tiny_skia::{Color, Paint, PathBuilder, PixmapMut, Stroke, Transform};

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
            background: Color::from_rgba8(20, 20, 24, 255),
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
    render_vault_sidebar_scaled(pixmap, layout, active_path, hovered_path, 1.0, theme);
}

/// Renders the complete Study Vault drawer sidebar into `pixmap` with HiDPI scale factor support.
pub fn render_vault_sidebar_scaled(
    pixmap: &mut PixmapMut<'_>,
    layout: &VaultLayout,
    active_path: Option<&Path>,
    hovered_path: Option<&Path>,
    scale_factor: f64,
    theme: &VaultTheme,
) {
    #[allow(clippy::cast_possible_truncation)]
    let t = Transform::from_scale(scale_factor as f32, scale_factor as f32);

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

    // 4. Header title and action buttons
    render_ui_text(
        pixmap,
        "Study Vault",
        16.0,
        14.0,
        15.0,
        theme.header_icon_color,
        t,
        Some(150.0),
    );
    render_header_buttons(pixmap, layout, theme, t);

    if layout.rows.is_empty() {
        render_ui_text(
            pixmap,
            "No boards in vault",
            16.0,
            VAULT_HEADER_HEIGHT + 16.0,
            13.0,
            theme.text_dim,
            t,
            Some(VAULT_DRAWER_WIDTH - 32.0),
        );
    }

    // 5. Entry rows
    for row in &layout.rows {
        let is_active = active_path.is_some_and(|p| p == row.item.path);
        let is_hovered = hovered_path.is_some_and(|p| p == row.item.path);

        let row_bg = if is_active {
            Some(theme.active_bg)
        } else if is_hovered {
            Some(theme.hover_bg)
        } else {
            None
        };
        if let Some(bg_col) = row_bg {
            let mut p = Paint::default();
            p.set_color(bg_col);
            if let Some(r) = tiny_skia::Rect::from_xywh(
                row.row_bounds.min.x,
                row.row_bounds.min.y,
                row.row_bounds.width(),
                row.row_bounds.height(),
            ) {
                pixmap.fill_rect(r, &p, t, None);
            }
        }
        if is_active {
            let mut p = Paint::default();
            p.set_color(theme.active_accent);
            if let Some(r) =
                tiny_skia::Rect::from_xywh(0.0, row.row_bounds.min.y, 3.0, row.row_bounds.height())
            {
                pixmap.fill_rect(r, &p, t, None);
            }
        }

        #[allow(clippy::cast_precision_loss)]
        let indent_x = 12.0 + (row.item.depth as f32) * VAULT_DEPTH_INDENT;
        let mid_y = row.row_bounds.center().y;

        if row.item.is_folder {
            render_folder_row(pixmap, row, indent_x, mid_y, theme, t);
        } else {
            render_board_row(pixmap, row, indent_x, mid_y, theme, is_active, t);
        }
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
