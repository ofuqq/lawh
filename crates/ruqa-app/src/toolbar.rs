//! Toolbar interaction dispatch, hit-testing, and tool switching for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use ruqa_core::geometry::Point;
use ruqa_render::toolbar_render::ToolbarLayout;

impl WhiteboardApp {
    /// Returns the active toolbar layout geometry based on current logical window width.
    #[must_use]
    pub fn toolbar_layout(&self) -> ToolbarLayout {
        ToolbarLayout::new(self.logical_size().width)
    }

    /// Sets the active drawing tool and requests a redraw if it changed.
    pub fn set_active_tool(&mut self, tool: ruqa_core::tool::Tool) {
        if self.is_editing_text && self.active_tool != tool {
            self.commit_text_edit();
        }
        if self.active_tool != tool {
            self.active_tool = tool;
            self.request_redraw();
        }
    }

    /// Hit-tests a logical screen point against the toolbar.
    ///
    /// If a tool button is clicked, updates active tool, flags toolbar interaction,
    /// and returns `true`. If the toolbar background is clicked, flags toolbar interaction
    /// and returns `true`. Returns `false` if outside the toolbar bounds.
    pub fn handle_toolbar_pointer_down(&mut self, screen_pt: Point) -> bool {
        let layout = self.toolbar_layout();
        if let Some(tool) = layout.tool_at(screen_pt) {
            self.set_active_tool(tool);
            self.toolbar_pressed = true;
            true
        } else if layout.contains(screen_pt) {
            self.toolbar_pressed = true;
            true
        } else {
            false
        }
    }

    /// Handles pointer release for toolbar interaction, returning `true` if consumed.
    pub fn handle_toolbar_pointer_up(&mut self) -> bool {
        if self.toolbar_pressed {
            self.toolbar_pressed = false;
            true
        } else {
            false
        }
    }

    /// Updates the hovered tool button under the cursor and returns `true` if cursor is over toolbar.
    pub fn handle_toolbar_cursor_moved(&mut self, screen_pt: Point) -> bool {
        let layout = self.toolbar_layout();
        let new_hovered = layout.tool_at(screen_pt);
        if new_hovered != self.hovered_tool {
            self.hovered_tool = new_hovered;
            self.request_redraw();
        }
        layout.contains(screen_pt)
    }
}
