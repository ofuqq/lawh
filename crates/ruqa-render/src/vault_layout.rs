//! Geometric layout and screen hit-testing for the Study Vault drawer sidebar.

use ruqa_core::geometry::{Point, Rect};
use ruqa_core::vault::FlatVaultItem;
use std::path::PathBuf;

/// Default drawer sidebar width in logical pixels.
pub const VAULT_DRAWER_WIDTH: f32 = 250.0;

/// Standard height of the vault header in logical pixels.
pub const VAULT_HEADER_HEIGHT: f32 = 48.0;

/// Height of each entry row in the vault tree.
pub const VAULT_ROW_HEIGHT: f32 = 32.0;

/// Horizontal indentation step per folder depth level.
pub const VAULT_DEPTH_INDENT: f32 = 14.0;

/// User interactions triggered by clicking inside the Study Vault drawer sidebar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultAction {
    /// Toggle expand/collapse state for a subfolder.
    ToggleFolder(PathBuf),
    /// Open a whiteboard document board.
    OpenBoard(PathBuf),
    /// Collapse/hide the vault drawer sidebar.
    CollapseSidebar,
    /// Create a new empty board in the vault root.
    NewBoard,
}

/// Computed screen bounds for a single entry row in the vault sidebar.
#[derive(Debug, Clone, PartialEq)]
pub struct VaultItemRow {
    /// The corresponding vault item record.
    pub item: FlatVaultItem,
    /// Overall clickable row bounds.
    pub row_bounds: Rect,
    /// Clickable folder expand/collapse chevron bounds (if folder).
    pub chevron_bounds: Option<Rect>,
}

/// Screen layout geometry for the complete Study Vault drawer sidebar.
#[derive(Debug, Clone, PartialEq)]
pub struct VaultLayout {
    /// Total drawer boundary rectangle.
    pub drawer_bounds: Rect,
    /// Top header bar bounds.
    pub header_bounds: Rect,
    /// Close/collapse button bounds.
    pub close_button_bounds: Rect,
    /// Quick "New Board" button bounds.
    pub new_board_bounds: Rect,
    /// Ordered row layout bounds for each visible entry.
    pub rows: Vec<VaultItemRow>,
}

impl VaultLayout {
    /// Computes the drawer layout for given window height and visible vault entries.
    #[must_use]
    pub fn new(window_height: f32, entries: &[FlatVaultItem]) -> Self {
        let drawer_bounds = Rect::from_xywh(0.0, 0.0, VAULT_DRAWER_WIDTH, window_height);
        let header_bounds = Rect::from_xywh(0.0, 0.0, VAULT_DRAWER_WIDTH, VAULT_HEADER_HEIGHT);

        let button_size = 28.0;
        let button_y = (VAULT_HEADER_HEIGHT - button_size) * 0.5;

        // Close button on far right of header
        let close_button_bounds = Rect::from_xywh(
            VAULT_DRAWER_WIDTH - button_size - 10.0,
            button_y,
            button_size,
            button_size,
        );

        // "New Board" button next to close button
        let new_board_bounds = Rect::from_xywh(
            VAULT_DRAWER_WIDTH - (button_size * 2.0) - 16.0,
            button_y,
            button_size,
            button_size,
        );

        let mut rows = Vec::with_capacity(entries.len());
        let mut current_y = VAULT_HEADER_HEIGHT + 6.0;

        for entry in entries {
            if current_y + VAULT_ROW_HEIGHT > window_height {
                break; // Stop laying out rows if beyond viewport bottom
            }

            let row_bounds = Rect::from_xywh(0.0, current_y, VAULT_DRAWER_WIDTH, VAULT_ROW_HEIGHT);

            #[allow(clippy::cast_precision_loss)]
            let indent_x = 12.0 + (entry.depth as f32) * VAULT_DEPTH_INDENT;

            let chevron_bounds = if entry.is_folder {
                Some(Rect::from_xywh(
                    indent_x,
                    current_y + (VAULT_ROW_HEIGHT - 16.0) * 0.5,
                    16.0,
                    16.0,
                ))
            } else {
                None
            };

            rows.push(VaultItemRow {
                item: entry.clone(),
                row_bounds,
                chevron_bounds,
            });

            current_y += VAULT_ROW_HEIGHT;
        }

        Self {
            drawer_bounds,
            header_bounds,
            close_button_bounds,
            new_board_bounds,
            rows,
        }
    }

    /// Hit-tests a screen point against interactive drawer sidebar controls.
    #[must_use]
    pub fn action_at(&self, screen_pt: Point) -> Option<VaultAction> {
        if !self.drawer_bounds.contains(screen_pt) {
            return None;
        }

        if self.close_button_bounds.contains(screen_pt) {
            return Some(VaultAction::CollapseSidebar);
        }

        if self.new_board_bounds.contains(screen_pt) {
            return Some(VaultAction::NewBoard);
        }

        for row in &self.rows {
            if row.row_bounds.contains(screen_pt) {
                if row.item.is_folder {
                    return Some(VaultAction::ToggleFolder(row.item.path.clone()));
                }
                return Some(VaultAction::OpenBoard(row.item.path.clone()));
            }
        }

        None
    }

    /// Returns `true` if the screen point lies anywhere within the drawer boundaries.
    #[must_use]
    pub fn contains(&self, screen_pt: Point) -> bool {
        self.drawer_bounds.contains(screen_pt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_layout_hit_testing() {
        let entries = vec![
            FlatVaultItem {
                path: PathBuf::from("/vault/Algorithms"),
                name: "Algorithms".into(),
                is_folder: true,
                depth: 0,
                is_expanded: false,
            },
            FlatVaultItem {
                path: PathBuf::from("/vault/Lecture1.ruqa"),
                name: "Lecture1".into(),
                is_folder: false,
                depth: 0,
                is_expanded: false,
            },
        ];

        let layout = VaultLayout::new(600.0, &entries);

        assert_eq!(layout.drawer_bounds.width(), VAULT_DRAWER_WIDTH);
        assert_eq!(layout.drawer_bounds.height(), 600.0);

        // Clicking close button
        let close_pt = layout.close_button_bounds.center();
        assert_eq!(
            layout.action_at(close_pt),
            Some(VaultAction::CollapseSidebar)
        );

        // Clicking new board button
        let new_pt = layout.new_board_bounds.center();
        assert_eq!(layout.action_at(new_pt), Some(VaultAction::NewBoard));

        // Clicking folder row
        let folder_pt = layout.rows[0].row_bounds.center();
        assert_eq!(
            layout.action_at(folder_pt),
            Some(VaultAction::ToggleFolder(PathBuf::from(
                "/vault/Algorithms"
            )))
        );

        // Clicking board file row
        let board_pt = layout.rows[1].row_bounds.center();
        assert_eq!(
            layout.action_at(board_pt),
            Some(VaultAction::OpenBoard(PathBuf::from(
                "/vault/Lecture1.ruqa"
            )))
        );

        // Outside drawer
        assert_eq!(layout.action_at(Point::new(300.0, 100.0)), None);
    }
}
