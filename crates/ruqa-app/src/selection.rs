//! Selection manager, multi-selection state, and rectangular marquee drag tracking.
//!
//! Handles single-click selection, Shift-click toggling, canvas deselect, and
//! marquee box intersection queries over [`Scene`] elements.

use ruqa_core::element::{Element, ElementId};
use ruqa_core::geometry::{Point, Rect};
use ruqa_core::scene::Scene;
use std::collections::HashSet;

/// Minimum drag distance in world units to distinguish a click from a marquee drag.
pub const MARQUEE_DRAG_THRESHOLD: f32 = 3.0;

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

/// Marquee dragging interaction state in world coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarqueeDrag {
    /// World position where the drag initiated.
    pub start: Point,
    /// Current world position of the drag cursor.
    pub current: Point,
}

impl MarqueeDrag {
    /// Computes the normalized bounding rectangle of the marquee in world coordinates.
    #[must_use]
    pub fn rect(&self) -> Rect {
        Rect::from_min_max(self.start, self.current)
    }

    /// Returns `true` if the drag distance exceeds the click threshold.
    #[must_use]
    pub fn is_significant(&self) -> bool {
        let dx = self.current.x - self.start.x;
        let dy = self.current.y - self.start.y;
        dx.hypot(dy) >= MARQUEE_DRAG_THRESHOLD
    }
}

/// Selection tool controller orchestrating mouse clicks, Shift modifiers, and marquee drag-selection.
#[derive(Debug, Clone, Default)]
pub struct SelectionManager {
    /// Active element selection.
    pub selection: SelectionState,
    /// Active marquee drag gesture, if any.
    pub active_marquee: Option<MarqueeDrag>,
}

impl SelectionManager {
    /// Creates a new selection manager with empty selection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the set of currently selected element IDs.
    #[must_use]
    pub fn selected_ids(&self) -> &HashSet<ElementId> {
        self.selection.selected_ids()
    }

    /// Handles pointer press down at a world coordinate.
    ///
    /// If an element is hit, updates selection immediately.
    /// If empty canvas is pressed, begins potential marquee dragging.
    pub fn handle_pointer_down(
        &mut self,
        scene: &Scene,
        world_pt: Point,
        shift_held: bool,
        tolerance: f32,
    ) {
        if let Some(hit) = scene.hit_test_topmost(world_pt, tolerance) {
            let id = hit.id();
            if shift_held {
                self.selection.toggle(id);
            } else if !self.selection.is_selected(id) {
                self.selection.select_single(id);
            }
            self.active_marquee = None;
        } else {
            // Clicked on empty canvas
            self.active_marquee = Some(MarqueeDrag {
                start: world_pt,
                current: world_pt,
            });
        }
    }

    /// Handles pointer motion while dragging.
    pub fn handle_pointer_move(&mut self, world_pt: Point) {
        if let Some(marquee) = &mut self.active_marquee {
            marquee.current = world_pt;
        }
    }

    /// Handles pointer release, committing marquee selection or clearing on empty click.
    pub fn handle_pointer_up(&mut self, scene: &Scene, shift_held: bool) {
        if let Some(marquee) = self.active_marquee.take() {
            if marquee.is_significant() {
                let marquee_rect = marquee.rect();
                let intersected: Vec<ElementId> = scene
                    .elements()
                    .iter()
                    .filter(|elem| elem.bounding_box().intersects(&marquee_rect))
                    .map(Element::id)
                    .collect();

                if shift_held {
                    self.selection.extend(intersected);
                } else {
                    self.selection.set_selection(intersected);
                }
            } else if !shift_held {
                // Insignificant drag on empty canvas constitutes a click to clear
                self.selection.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruqa_core::element::ElementStyle;

    fn sample_rect(id: ElementId, x: f32, y: f32, w: f32, h: f32) -> Element {
        Element::Rectangle {
            id,
            bounds: Rect::from_xywh(x, y, w, h),
            style: ElementStyle {
                stroke_width: 2.0,
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_single_click_select_and_clear_previous() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));
        scene.push(sample_rect(2, 100.0, 10.0, 50.0, 50.0));

        let mut manager = SelectionManager::new();

        // Click rect 1: selects 1
        manager.handle_pointer_down(&scene, Point::new(10.0, 10.0), false, 2.0);
        manager.handle_pointer_up(&scene, false);
        assert_eq!(manager.selection.selected_ids().len(), 1);
        assert!(manager.selection.is_selected(1));

        // Click rect 2 without shift: clears 1 and selects 2
        manager.handle_pointer_down(&scene, Point::new(100.0, 10.0), false, 2.0);
        manager.handle_pointer_up(&scene, false);
        assert_eq!(manager.selection.selected_ids().len(), 1);
        assert!(!manager.selection.is_selected(1));
        assert!(manager.selection.is_selected(2));
    }

    #[test]
    fn test_shift_click_toggles_inclusion() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));
        scene.push(sample_rect(2, 100.0, 10.0, 50.0, 50.0));

        let mut manager = SelectionManager::new();

        // Select 1
        manager.handle_pointer_down(&scene, Point::new(10.0, 10.0), false, 2.0);
        manager.handle_pointer_up(&scene, false);

        // Shift-click 2: both 1 and 2 selected
        manager.handle_pointer_down(&scene, Point::new(100.0, 10.0), true, 2.0);
        manager.handle_pointer_up(&scene, true);
        assert_eq!(manager.selection.len(), 2);
        assert!(manager.selection.is_selected(1));
        assert!(manager.selection.is_selected(2));

        // Shift-click 1: deselects 1, leaving 2
        manager.handle_pointer_down(&scene, Point::new(10.0, 10.0), true, 2.0);
        manager.handle_pointer_up(&scene, true);
        assert_eq!(manager.selection.len(), 1);
        assert!(!manager.selection.is_selected(1));
        assert!(manager.selection.is_selected(2));
    }

    #[test]
    fn test_click_empty_canvas_clears_selection() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));

        let mut manager = SelectionManager::new();
        manager.selection.select_single(1);
        assert!(manager.selection.is_selected(1));

        // Click on empty canvas at (500, 500)
        manager.handle_pointer_down(&scene, Point::new(500.0, 500.0), false, 2.0);
        manager.handle_pointer_up(&scene, false);
        assert!(manager.selection.is_empty());
    }

    #[test]
    fn test_marquee_drag_selects_intersected_shapes() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 20.0, 20.0, 40.0, 40.0));
        scene.push(sample_rect(2, 80.0, 20.0, 40.0, 40.0));
        scene.push(sample_rect(3, 300.0, 300.0, 40.0, 40.0));

        let mut manager = SelectionManager::new();

        // Start marquee drag at (0, 0)
        manager.handle_pointer_down(&scene, Point::new(0.0, 0.0), false, 2.0);
        assert!(manager.active_marquee.is_some());

        // Drag to (150, 100), covering rects 1 and 2
        manager.handle_pointer_move(Point::new(150.0, 100.0));
        assert!(manager.active_marquee.unwrap().is_significant());

        // Release drag
        manager.handle_pointer_up(&scene, false);
        assert!(manager.active_marquee.is_none());
        assert_eq!(manager.selection.len(), 2);
        assert!(manager.selection.is_selected(1));
        assert!(manager.selection.is_selected(2));
        assert!(!manager.selection.is_selected(3));
    }
}
