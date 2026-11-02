//! Screen geometry, hit-testing, and layout calculations for the Recent Files / Welcome HUD modal.

use ruqa_core::geometry::{Point, Rect};
use ruqa_core::recent_files::RecentFileEntry;
use std::path::PathBuf;

/// Action triggered by clicking inside the Recent Files HUD overlay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecentFilesAction {
    /// Open the selected recent document file path.
    OpenFile(PathBuf),
    /// Close / dismiss the HUD overlay without opening a file.
    Dismiss,
    /// Create a new blank board and dismiss the HUD.
    NewBoard,
}

/// Precomputed layout bounds for the Recent Files HUD overlay.
#[derive(Debug, Clone, PartialEq)]
pub struct RecentFilesLayout {
    /// Screen bounds enclosing the entire modal dialog.
    pub dialog_bounds: Rect,
    /// Screen bounds for the close ("X") button.
    pub close_button_bounds: Rect,
    /// Screen bounds for the "New Board" button.
    pub new_board_bounds: Rect,
    /// Bounding boxes for each listed recent file item card.
    pub item_bounds: Vec<(PathBuf, Rect)>,
}

impl RecentFilesLayout {
    /// Computes the modal dialog layout centered within the given window dimensions.
    #[must_use]
    pub fn new(window_width: f32, window_height: f32, entries: &[RecentFileEntry]) -> Self {
        let dialog_width = 540.0f32.min(window_width - 40.0);
        let max_items = entries.len().min(6);
        let item_height = 44.0;
        let header_height = 80.0;
        let footer_height = 60.0;
        let list_height = (max_items as f32) * (item_height + 6.0);
        let dialog_height = header_height + list_height + footer_height;

        let left = (window_width - dialog_width) * 0.5;
        let top = (window_height - dialog_height) * 0.5;
        let dialog_bounds = Rect::from_xywh(left, top, dialog_width, dialog_height);

        let close_size = 28.0;
        let close_button_bounds = Rect::from_xywh(
            left + dialog_width - close_size - 16.0,
            top + 16.0,
            close_size,
            close_size,
        );

        let new_board_width = 130.0;
        let new_board_height = 36.0;
        let new_board_bounds = Rect::from_xywh(
            left + 24.0,
            top + dialog_height - footer_height + 12.0,
            new_board_width,
            new_board_height,
        );

        let mut item_bounds = Vec::with_capacity(max_items);
        let item_start_y = top + header_height;
        let item_padding_x = 24.0;
        let item_width = dialog_width - (item_padding_x * 2.0);

        for (idx, entry) in entries.iter().take(max_items).enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let item_y = item_start_y + (idx as f32) * (item_height + 6.0);
            let bounds = Rect::from_xywh(left + item_padding_x, item_y, item_width, item_height);
            item_bounds.push((entry.path.clone(), bounds));
        }

        Self {
            dialog_bounds,
            close_button_bounds,
            new_board_bounds,
            item_bounds,
        }
    }

    /// Hit-tests a screen point against the modal dialog and interactive child controls.
    #[must_use]
    pub fn action_at(&self, screen_pt: Point) -> Option<RecentFilesAction> {
        if self.close_button_bounds.contains(screen_pt) {
            return Some(RecentFilesAction::Dismiss);
        }

        if self.new_board_bounds.contains(screen_pt) {
            return Some(RecentFilesAction::NewBoard);
        }

        for (path, bounds) in &self.item_bounds {
            if bounds.contains(screen_pt) {
                return Some(RecentFilesAction::OpenFile(path.clone()));
            }
        }

        // If clicked outside dialog bounds, dismiss
        if !self.dialog_bounds.contains(screen_pt) {
            return Some(RecentFilesAction::Dismiss);
        }

        None
    }

    /// Returns `true` if the screen point lies anywhere within the modal dialog boundaries.
    #[must_use]
    pub fn contains(&self, screen_pt: Point) -> bool {
        self.dialog_bounds.contains(screen_pt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_files_layout_centering_and_hit_testing() {
        let entries = vec![
            RecentFileEntry::new(PathBuf::from("/vault/Algorithms.ruqa")),
            RecentFileEntry::new(PathBuf::from("/vault/Database_Systems.ruqa")),
        ];

        let layout = RecentFilesLayout::new(1280.0, 800.0, &entries);

        // Dialog should be horizontally and vertically centered
        assert!((layout.dialog_bounds.center().x - 640.0).abs() < 1.0);
        assert!((layout.dialog_bounds.center().y - 400.0).abs() < 50.0);

        // Clicking first item returns OpenFile
        let item1_center = layout.item_bounds[0].1.center();
        assert_eq!(
            layout.action_at(item1_center),
            Some(RecentFilesAction::OpenFile(PathBuf::from(
                "/vault/Algorithms.ruqa"
            )))
        );

        // Clicking close button returns Dismiss
        let close_center = layout.close_button_bounds.center();
        assert_eq!(
            layout.action_at(close_center),
            Some(RecentFilesAction::Dismiss)
        );

        // Clicking outside dialog returns Dismiss
        assert_eq!(
            layout.action_at(Point::new(10.0, 10.0)),
            Some(RecentFilesAction::Dismiss)
        );
    }
}
