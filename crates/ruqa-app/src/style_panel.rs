//! Contextual style panel state machine, hit-testing, and property updates for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use ruqa_core::geometry::Point;
use ruqa_core::history::Command;
use ruqa_render::style_panel_layout::{StyleAction, StylePanelLayout};

impl WhiteboardApp {
    /// Returns the layout of the contextual styling HUD panel.
    #[must_use]
    pub fn style_panel_layout(&self) -> StylePanelLayout {
        StylePanelLayout::new()
    }

    /// Hit-tests a logical screen point against the contextual style panel.
    ///
    /// If an action button is clicked, applies the style change (to selected elements as
    /// an undoable command if elements are selected, or to default creation styles if none are selected)
    /// and returns `true`. If clicked inside the panel container, returns `true` to isolate clicks.
    pub fn handle_style_panel_pointer_down(&mut self, screen_pt: Point) -> bool {
        if !self.show_style_panel {
            return false;
        }

        let layout = self.style_panel_layout();
        if let Some(action) = layout.action_at(screen_pt) {
            self.style_panel_pressed = true;
            self.apply_style_action(action);
            true
        } else if layout.contains(screen_pt) {
            self.style_panel_pressed = true;
            true
        } else {
            false
        }
    }

    /// Handles pointer release for style panel interaction, returning `true` if consumed.
    pub fn handle_style_panel_pointer_up(&mut self) -> bool {
        if self.style_panel_pressed {
            self.style_panel_pressed = false;
            true
        } else {
            false
        }
    }

    /// Applies a [`StyleAction`] to selected elements (as an undoable [`Command`]) or updates default creation style.
    pub fn apply_style_action(&mut self, action: StyleAction) {
        match action {
            StyleAction::SetColor(hex) => {
                self.current_style.stroke_color = hex.to_string();
                self.apply_style_to_selection(|style| {
                    style.stroke_color = hex.to_string();
                });
            }
            StyleAction::SetStrokeWidth(preset) => {
                self.current_style.stroke_width = preset.width();
                self.apply_style_to_selection(|style| {
                    style.stroke_width = preset.width();
                });
            }
            StyleAction::SetRoughness(preset) => {
                self.current_style.roughness = preset.factor();
                self.apply_style_to_selection(|style| {
                    style.roughness = preset.factor();
                });
            }
        }
        self.request_redraw();
    }

    /// Helper to apply style mutations to selected elements and record an undoable [`Command::ModifyElements`].
    fn apply_style_to_selection<F>(&mut self, mut mutate: F)
    where
        F: FnMut(&mut ruqa_core::style::ElementStyle),
    {
        if self.selection.is_empty() {
            return;
        }

        let before: Vec<_> = self
            .selection
            .selected_ids()
            .iter()
            .filter_map(|&id| self.scene.get(id).cloned())
            .collect();

        if before.is_empty() {
            return;
        }

        let mut after = Vec::with_capacity(before.len());
        for elem in &before {
            if let Some(target) = self.scene.get_mut(elem.id()) {
                mutate(target.style_mut());
                after.push(target.clone());
            }
        }

        self.history.push(Command::ModifyElements { before, after });
        self.mark_dirty();
    }

    /// Synchronizes the active styling properties from the currently selected element, if any.
    pub fn sync_style_from_selection(&mut self) {
        if let Some(&id) = self.selection.selected_ids().iter().next() {
            if let Some(elem) = self.scene.get(id) {
                self.current_style = elem.style().clone();
            }
        }
    }
}
