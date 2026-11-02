//! Multi-element selection state tracking.

use ruqa_core::element::ElementId;
use std::collections::HashSet;

/// Selection state managing a set of selected element IDs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SelectionState {
    selected_ids: HashSet<ElementId>,
}

impl SelectionState {
    /// Creates a new empty selection state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            selected_ids: HashSet::new(),
        }
    }

    /// Returns `true` if the element with the given ID is currently selected.
    #[must_use]
    pub fn is_selected(&self, id: ElementId) -> bool {
        self.selected_ids.contains(&id)
    }

    /// Selects a single element, clearing any prior selection.
    pub fn select_single(&mut self, id: ElementId) {
        self.selected_ids.clear();
        self.selected_ids.insert(id);
    }

    /// Toggles the selection state of the given element ID (Shift-click behavior).
    pub fn toggle(&mut self, id: ElementId) {
        if self.selected_ids.contains(&id) {
            self.selected_ids.remove(&id);
        } else {
            self.selected_ids.insert(id);
        }
    }

    /// Deselects an element if it is currently selected.
    pub fn deselect(&mut self, id: ElementId) {
        self.selected_ids.remove(&id);
    }

    /// Clears all selected elements.
    pub fn clear(&mut self) {
        self.selected_ids.clear();
    }

    /// Selects multiple elements, replacing existing selection.
    pub fn set_selection(&mut self, ids: impl IntoIterator<Item = ElementId>) {
        self.selected_ids.clear();
        self.selected_ids.extend(ids);
    }

    /// Adds multiple elements to the current selection without clearing.
    pub fn extend(&mut self, ids: impl IntoIterator<Item = ElementId>) {
        self.selected_ids.extend(ids);
    }

    /// Returns the number of selected elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.selected_ids.len()
    }

    /// Returns `true` if no elements are selected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.selected_ids.is_empty()
    }

    /// Returns a reference to the set of selected element IDs.
    #[must_use]
    pub fn selected_ids(&self) -> &HashSet<ElementId> {
        &self.selected_ids
    }
}
