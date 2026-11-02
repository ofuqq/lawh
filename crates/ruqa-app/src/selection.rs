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

/// Active drag operation being tracked by the selection manager.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DragMode {
    /// Rectangular marquee selection drag.
    Marquee(MarqueeDrag),
    /// Translating selected elements by tracking the previous world position.
    Translate {
        /// World coordinates of the last pointer position during this drag.
        last_world_pos: Point,
    },
}

/// Selection tool controller orchestrating mouse clicks, Shift modifiers, marquee selection, and translation.
#[derive(Debug, Clone, Default)]
pub struct SelectionManager {
    /// Active element selection.
    pub selection: SelectionState,
    /// Active drag operation (marquee or element translation), if any.
    pub active_drag: Option<DragMode>,
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

    /// Returns the active marquee drag gesture, if currently in marquee mode.
    #[must_use]
    pub fn active_marquee(&self) -> Option<MarqueeDrag> {
        match self.active_drag {
            Some(DragMode::Marquee(m)) => Some(m),
            _ => None,
        }
    }

    /// Returns `true` if currently translating selected elements.
    #[must_use]
    pub fn is_translating(&self) -> bool {
        matches!(self.active_drag, Some(DragMode::Translate { .. }))
    }

    /// Handles pointer press down at a world coordinate.
    ///
    /// If an element is hit, updates selection and initiates element translation drag.
    /// If empty canvas is pressed, begins marquee drag.
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
                if self.selection.is_selected(id) {
                    self.active_drag = Some(DragMode::Translate {
                        last_world_pos: world_pt,
                    });
                } else {
                    self.active_drag = None;
                }
            } else {
                if !self.selection.is_selected(id) {
                    self.selection.select_single(id);
                }
                self.active_drag = Some(DragMode::Translate {
                    last_world_pos: world_pt,
                });
            }
        } else {
            // Clicked on empty canvas
            self.active_drag = Some(DragMode::Marquee(MarqueeDrag {
                start: world_pt,
                current: world_pt,
            }));
        }
    }

    /// Handles pointer motion while dragging.
    ///
    /// If translating, translates all selected elements in `scene` by the delta.
    /// If in marquee mode, updates the marquee current coordinate.
    pub fn handle_pointer_move(&mut self, scene: &mut Scene, world_pt: Point) -> bool {
        match &mut self.active_drag {
            Some(DragMode::Translate { last_world_pos }) => {
                let delta = world_pt - *last_world_pos;
                if delta.x != 0.0 || delta.y != 0.0 {
                    scene.translate_elements(self.selection.selected_ids(), delta);
                    *last_world_pos = world_pt;
                    true
                } else {
                    false
                }
            }
            Some(DragMode::Marquee(marquee)) => {
                marquee.current = world_pt;
                true
            }
            None => false,
        }
    }

    /// Handles pointer release, committing marquee selection or concluding drag.
    pub fn handle_pointer_up(&mut self, scene: &Scene, shift_held: bool) {
        if let Some(DragMode::Marquee(marquee)) = self.active_drag.take() {
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
                fill_color: Some("#FFFFFF".into()),
                fill_style: ruqa_core::element::FillStyle::Solid,
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_single_click_and_toggle() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));
        scene.push(sample_rect(2, 100.0, 10.0, 50.0, 50.0));

        let mut manager = SelectionManager::new();
        // Click 1: selects 1
        manager.handle_pointer_down(&scene, Point::new(10.0, 10.0), false, 2.0);
        manager.handle_pointer_up(&scene, false);
        assert_eq!(manager.selection.selected_ids().len(), 1);
        assert!(manager.selection.is_selected(1));

        // Shift-click 2: both selected
        manager.handle_pointer_down(&scene, Point::new(100.0, 10.0), true, 2.0);
        manager.handle_pointer_up(&scene, true);
        assert_eq!(manager.selection.len(), 2);

        // Click empty canvas: clears
        manager.handle_pointer_down(&scene, Point::new(500.0, 500.0), false, 2.0);
        manager.handle_pointer_up(&scene, false);
        assert!(manager.selection.is_empty());
    }

    #[test]
    fn test_marquee_drag_selection() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 20.0, 20.0, 40.0, 40.0));
        scene.push(sample_rect(2, 80.0, 20.0, 40.0, 40.0));
        scene.push(sample_rect(3, 300.0, 300.0, 40.0, 40.0));

        let mut manager = SelectionManager::new();
        manager.handle_pointer_down(&scene, Point::new(0.0, 0.0), false, 2.0);
        assert!(manager.active_marquee().is_some());

        manager.handle_pointer_move(&mut scene, Point::new(150.0, 100.0));
        assert!(manager.active_marquee().unwrap().is_significant());

        manager.handle_pointer_up(&scene, false);
        assert_eq!(manager.selection.len(), 2);
        assert!(manager.selection.is_selected(1) && manager.selection.is_selected(2));
        assert!(!manager.selection.is_selected(3));
    }

    #[test]
    fn test_drag_translate_selected_elements() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));
        scene.push(sample_rect(2, 80.0, 10.0, 50.0, 50.0));

        let mut manager = SelectionManager::new();
        manager.selection.set_selection([1, 2]);

        manager.handle_pointer_down(&scene, Point::new(20.0, 20.0), false, 2.0);
        assert!(manager.is_translating());

        manager.handle_pointer_move(&mut scene, Point::new(35.0, 30.0));
        assert_eq!(
            scene.get(1).unwrap().bounds(),
            Rect::from_xywh(25.0, 20.0, 50.0, 50.0)
        );
        assert_eq!(
            scene.get(2).unwrap().bounds(),
            Rect::from_xywh(95.0, 20.0, 50.0, 50.0)
        );

        manager.handle_pointer_up(&scene, false);
        assert!(!manager.is_translating());
    }
}
