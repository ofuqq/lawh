//! Screen layout and geometry calculations for the quit confirmation dialog.

use ruqa_core::geometry::{Point, Rect};
use tiny_skia::Color;

/// Action triggered by interacting with the quit confirmation prompt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuitDialogAction {
    /// Save active changes to disk and exit.
    Save,
    /// Discard unsaved changes and exit immediately.
    DontSave,
    /// Cancel quitting and return to editing the whiteboard.
    Cancel,
}

/// Theme and color palette for the quit confirmation dialog.
#[derive(Debug, Clone, PartialEq)]
pub struct QuitDialogTheme {
    /// Fullscreen backdrop dimming color.
    pub backdrop_color: Color,
    /// Modal container background fill color.
    pub dialog_background: Color,
    /// Modal container border stroke color.
    pub dialog_border: Color,
    /// Primary Save button background color.
    pub save_background: Color,
    /// Primary Save button hovered background color.
    pub save_hovered: Color,
    /// Danger / Don't Save button background color.
    pub dont_save_background: Color,
    /// Danger / Don't Save button hovered background color.
    pub dont_save_hovered: Color,
    /// Neutral Cancel button background color.
    pub cancel_background: Color,
    /// Neutral Cancel button hovered background color.
    pub cancel_hovered: Color,
}

impl Default for QuitDialogTheme {
    fn default() -> Self {
        Self {
            backdrop_color: Color::from_rgba8(0, 0, 0, 160),
            dialog_background: Color::from_rgba8(28, 28, 30, 252),
            dialog_border: Color::from_rgba8(60, 60, 65, 255),
            save_background: Color::from_rgba8(56, 189, 248, 255), // Cyan #38BDF8
            save_hovered: Color::from_rgba8(14, 165, 233, 255),
            dont_save_background: Color::from_rgba8(239, 68, 68, 255), // Coral #EF4444
            dont_save_hovered: Color::from_rgba8(220, 38, 38, 255),
            cancel_background: Color::from_rgba8(48, 48, 54, 255),
            cancel_hovered: Color::from_rgba8(65, 65, 72, 255),
        }
    }
}

/// Precomputed layout bounds for the quit confirmation dialog and its buttons.
#[derive(Debug, Clone, PartialEq)]
pub struct QuitDialogLayout {
    /// Screen bounds enclosing the entire modal dialog.
    pub dialog_bounds: Rect,
    /// Screen bounds for the "Save" action button.
    pub save_button_bounds: Rect,
    /// Screen bounds for the "Don't Save" action button.
    pub dont_save_button_bounds: Rect,
    /// Screen bounds for the "Cancel" action button.
    pub cancel_button_bounds: Rect,
}

impl QuitDialogLayout {
    /// Computes the centered quit confirmation dialog layout.
    #[must_use]
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let dialog_width = 440.0f32.min(window_width - 32.0);
        let dialog_height = 190.0f32.min(window_height - 32.0);

        let left = (window_width - dialog_width) * 0.5;
        let top = (window_height - dialog_height) * 0.5;
        let dialog_bounds = Rect::from_xywh(left, top, dialog_width, dialog_height);

        let button_height = 36.0;
        let button_spacing = 10.0;
        let button_y = top + dialog_height - button_height - 20.0;

        // 3 buttons: [Cancel] (90px), [Don't Save] (110px), [Save] (100px)
        let cancel_w = 90.0;
        let dont_save_w = 110.0;
        let save_w = 100.0;

        let total_btn_w = cancel_w + dont_save_w + save_w + (button_spacing * 2.0);
        let start_x = left + dialog_width - total_btn_w - 20.0;

        let cancel_button_bounds = Rect::from_xywh(start_x, button_y, cancel_w, button_height);
        let dont_save_button_bounds = Rect::from_xywh(
            start_x + cancel_w + button_spacing,
            button_y,
            dont_save_w,
            button_height,
        );
        let save_button_bounds = Rect::from_xywh(
            start_x + cancel_w + dont_save_w + (button_spacing * 2.0),
            button_y,
            save_w,
            button_height,
        );

        Self {
            dialog_bounds,
            save_button_bounds,
            dont_save_button_bounds,
            cancel_button_bounds,
        }
    }

    /// Hit-tests a screen point against the dialog action buttons.
    #[must_use]
    pub fn hit_test(&self, point: Point) -> Option<QuitDialogAction> {
        if self.save_button_bounds.contains(point) {
            Some(QuitDialogAction::Save)
        } else if self.dont_save_button_bounds.contains(point) {
            Some(QuitDialogAction::DontSave)
        } else if self.cancel_button_bounds.contains(point) {
            Some(QuitDialogAction::Cancel)
        } else {
            None
        }
    }

    /// Returns `true` if `point` is contained inside the dialog container bounds.
    #[must_use]
    pub fn contains(&self, point: Point) -> bool {
        self.dialog_bounds.contains(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit_dialog_layout_geometry_and_hit_testing() {
        let layout = QuitDialogLayout::new(1280.0, 800.0);
        assert_eq!(layout.dialog_bounds.width(), 440.0);
        assert_eq!(layout.dialog_bounds.height(), 190.0);

        // Center point of each button
        let save_pt = layout.save_button_bounds.center();
        let dont_save_pt = layout.dont_save_button_bounds.center();
        let cancel_pt = layout.cancel_button_bounds.center();

        assert_eq!(layout.hit_test(save_pt), Some(QuitDialogAction::Save));
        assert_eq!(
            layout.hit_test(dont_save_pt),
            Some(QuitDialogAction::DontSave)
        );
        assert_eq!(layout.hit_test(cancel_pt), Some(QuitDialogAction::Cancel));

        // Outside dialog
        assert_eq!(layout.hit_test(Point::new(10.0, 10.0)), None);
        assert!(!layout.contains(Point::new(10.0, 10.0)));
        assert!(layout.contains(layout.dialog_bounds.center()));
    }
}
