//! Clipboard and duplication operations for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use ruqa_core::element::Element;
use ruqa_core::geometry::Point;
use ruqa_core::history::Command;

/// Default offset in world units applied when duplicating elements.
pub const DUPLICATE_OFFSET: Point = Point::new(20.0, 20.0);

impl WhiteboardApp {
    /// Duplicates all currently selected elements with an offset, assigns fresh IDs and seeds,
    /// pushes an undoable command, and updates selection to the new elements.
    ///
    /// Returns `true` if any elements were duplicated.
    pub fn duplicate_selected(&mut self) -> bool {
        if self.selection.is_empty() {
            return false;
        }

        let mut selected = Vec::new();
        for elem in self.scene.elements() {
            if self.selection.selected_ids().contains(&elem.id()) {
                selected.push(elem.clone());
            }
        }

        if selected.is_empty() {
            return false;
        }

        let mut duplicates = Vec::with_capacity(selected.len());
        for elem in &selected {
            let new_id = self.scene.next_element_id();
            let new_seed = elem
                .style()
                .seed
                .wrapping_add(new_id)
                .wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let dup = elem.duplicate_with(new_id, new_seed, DUPLICATE_OFFSET);
            self.scene.push(dup.clone());
            duplicates.push(dup);
        }

        let dup_ids: Vec<_> = duplicates.iter().map(Element::id).collect();
        self.history.push(Command::AddElements(duplicates));
        self.selection.set_selection(dup_ids);
        self.mark_dirty();
        self.request_redraw();
        true
    }

    /// Copies all currently selected elements into the internal clipboard.
    ///
    /// Returns `true` if any elements were copied.
    pub fn copy_selected(&mut self) -> bool {
        if self.selection.is_empty() {
            return false;
        }

        let mut copied = Vec::new();
        for elem in self.scene.elements() {
            if self.selection.selected_ids().contains(&elem.id()) {
                copied.push(elem.clone());
            }
        }

        if copied.is_empty() {
            return false;
        }

        self.clipboard = copied;
        true
    }

    /// Pastes elements from the internal clipboard at the cursor position (or with an offset),
    /// assigns fresh IDs and seeds, pushes an undoable command, and updates selection.
    ///
    /// Returns `true` if any elements were pasted.
    pub fn paste_clipboard(&mut self) -> bool {
        if self.clipboard.is_empty() {
            return false;
        }

        // Calculate aggregate bounding box center of clipboard items.
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for elem in &self.clipboard {
            let bb = elem.bounding_box();
            min_x = min_x.min(bb.min.x);
            min_y = min_y.min(bb.min.y);
            max_x = max_x.max(bb.max.x);
            max_y = max_y.max(bb.max.y);
        }

        let center = Point::new((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);

        let offset = if let Some((cx, cy)) = self.pointer.cursor_pos {
            #[allow(clippy::cast_possible_truncation)]
            let screen_pt = Point::new(cx as f32, cy as f32);
            let world_pt = self
                .viewport
                .screen_to_world(screen_pt, self.logical_size());
            Point::new(world_pt.x - center.x, world_pt.y - center.y)
        } else {
            DUPLICATE_OFFSET
        };

        let mut pasted = Vec::with_capacity(self.clipboard.len());
        for elem in &self.clipboard {
            let new_id = self.scene.next_element_id();
            let new_seed = elem
                .style()
                .seed
                .wrapping_add(new_id)
                .wrapping_mul(0x9E37_79B9_7F4A_7C15);
            let new_elem = elem.duplicate_with(new_id, new_seed, offset);
            self.scene.push(new_elem.clone());
            pasted.push(new_elem);
        }

        let pasted_ids: Vec<_> = pasted.iter().map(Element::id).collect();
        self.history.push(Command::AddElements(pasted));
        self.selection.set_selection(pasted_ids);
        self.mark_dirty();
        self.request_redraw();
        true
    }

    /// Deletes all currently selected elements from the scene, registers undo command, and marks dirty.
    pub fn delete_selected(&mut self) -> bool {
        if self.selection.is_empty() {
            return false;
        }

        let mut to_delete = Vec::new();
        for &id in self.selection.selected_ids() {
            if let Some(pos) = self.scene.position(id) {
                to_delete.push((pos, id));
            }
        }

        if to_delete.is_empty() {
            return false;
        }

        to_delete.sort_by_key(|a| std::cmp::Reverse(a.0));
        let mut deleted_entries = Vec::with_capacity(to_delete.len());
        for (pos, id) in to_delete {
            if let Some(elem) = self.scene.remove(id) {
                deleted_entries.push((pos, elem));
            }
        }
        deleted_entries.sort_by_key(|(pos, _)| *pos);

        self.history
            .push(ruqa_core::history::Command::DeleteElements(deleted_entries));
        self.selection.clear();
        self.mark_dirty();
        self.request_redraw();
        true
    }
}
