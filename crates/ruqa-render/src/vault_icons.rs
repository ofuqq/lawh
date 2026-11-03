//! Vector icon drawing and row item rasterization for the Study Vault sidebar.

use crate::text_render::render_ui_text;
use crate::vault_layout::{VaultItemRow, VaultLayout, VAULT_DRAWER_WIDTH};
use crate::vault_render::VaultTheme;
use tiny_skia::{FillRule, Paint, PathBuilder, PixmapMut, Stroke, Transform};

/// Renders the close (`<`) and new board (`+`) header action buttons.
pub fn render_header_buttons(
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

/// Renders a single folder entry row including chevron, folder icon, and text label.
pub fn render_folder_row(
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

    // Typography label
    let text_x = indent_x + 32.0;
    let text_y = row.row_bounds.min.y + 7.0;
    let max_w = (VAULT_DRAWER_WIDTH - text_x - 8.0).max(20.0);
    render_ui_text(
        pixmap,
        &row.item.name,
        text_x,
        text_y,
        13.0,
        theme.header_icon_color,
        t,
        Some(max_w),
    );
}

/// Renders a single board document entry row including document icon and text label.
pub fn render_board_row(
    pixmap: &mut PixmapMut<'_>,
    row: &VaultItemRow,
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

    // Typography label
    let text_x = indent_x + 22.0;
    let text_y = row.row_bounds.min.y + 7.0;
    let max_w = (VAULT_DRAWER_WIDTH - text_x - 8.0).max(20.0);
    render_ui_text(
        pixmap,
        &row.item.name,
        text_x,
        text_y,
        13.0,
        if is_active {
            theme.active_accent
        } else {
            theme.text_dim
        },
        t,
        Some(max_w),
    );
}
