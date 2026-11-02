//! Selection manager, multi-selection state, and rectangular marquee drag tracking.
//!
//! Handles single-click selection, Shift-click toggling, canvas deselect,
//! marquee box intersection queries, and interactive handle resizing over [`Scene`] elements.

pub use crate::drag::{DragMode, MarqueeDrag, MARQUEE_DRAG_THRESHOLD};
pub use crate::selection_state::SelectionState;
use ruqa_core::element::{Element, ElementId};
use ruqa_core::geometry::{compute_resized_bounds, Point, Size, Viewport};
use ruqa_core::scene::Scene;
use std::collections::{HashMap, HashSet};

/// Selection tool controller orchestrating clicks, Shift modifiers, marquee, translation, and resizing.
#[derive(Debug, Clone, Default)]
pub struct SelectionManager {
    /// Active element selection.
    pub selection: SelectionState,
    /// Active drag operation (marquee, translation, or resizing), if any.
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

    /// Returns `true` if currently resizing selected elements.
    #[must_use]
    pub fn is_resizing(&self) -> bool {
        matches!(self.active_drag, Some(DragMode::Resize { .. }))
    }

    /// Handles pointer press down at a given world and screen coordinate.
    ///
    /// If an active selection handle is hit, initiates resize dragging.
    /// If an element is hit, updates selection and initiates element translation drag.
    /// If empty canvas is pressed, begins marquee drag.
    #[allow(clippy::too_many_arguments)]
    pub fn handle_pointer_down(
        &mut self,
        scene: &Scene,
        world_pt: Point,
        screen_pt: Point,
        viewport: &Viewport,
        window_size: Size,
        shift_held: bool,
        tolerance: f32,
    ) {
        if !self.selection.is_empty() {
            if let Some(bounds) = scene.selection_bounds(self.selection.selected_ids()) {
                let style = ruqa_render::selection_render::SelectionStyle::default();
                if let Some(handle) = ruqa_render::selection_render::hit_test_handle(
                    screen_pt,
                    bounds,
                    viewport,
                    window_size,
                    &style,
                ) {
                    let mut initial_elements = HashMap::new();
                    for &id in self.selection.selected_ids() {
                        if let Some(elem) = scene.get(id) {
                            initial_elements.insert(id, elem.clone());
                        }
                    }
                    self.active_drag = Some(DragMode::Resize {
                        handle,
                        initial_bounds: bounds,
                        initial_elements,
                    });
                    return;
                }
            }
        }

        if let Some(hit) = scene.hit_test_topmost(world_pt, tolerance) {
            let id = hit.id();
            if shift_held {
                self.selection.toggle(id);
                if self.selection.is_selected(id) {
                    self.active_drag = Some(DragMode::Translate {
                        start_world_pos: world_pt,
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
                    start_world_pos: world_pt,
                    last_world_pos: world_pt,
                });
            }
        } else {
            self.active_drag = Some(DragMode::Marquee(MarqueeDrag {
                start: world_pt,
                current: world_pt,
            }));
        }
    }

    /// Handles pointer motion while dragging.
    ///
    /// If resizing, deforms selected elements proportionally.
    /// If translating, translates all selected elements in `scene` by the delta.
    /// If in marquee mode, updates the marquee current coordinate.
    pub fn handle_pointer_move(
        &mut self,
        scene: &mut Scene,
        world_pt: Point,
        shift_held: bool,
    ) -> bool {
        match &mut self.active_drag {
            Some(DragMode::Resize {
                handle,
                initial_bounds,
                initial_elements,
            }) => {
                let new_bounds =
                    compute_resized_bounds(*initial_bounds, *handle, world_pt, shift_held);
                for (id, orig_elem) in initial_elements {
                    if let Some(elem) = scene.get_mut(*id) {
                        let mut scaled = orig_elem.clone();
                        scaled.scale_from_bounds(*initial_bounds, new_bounds);
                        *elem = scaled;
                    }
                }
                true
            }
            Some(DragMode::Translate { last_world_pos, .. }) => {
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
    ///
    /// Returns a [`Command`](ruqa_core::history::Command) if a reversible translation or resize mutation was committed.
    pub fn handle_pointer_up(
        &mut self,
        scene: &Scene,
        shift_held: bool,
    ) -> Option<ruqa_core::history::Command> {
        match self.active_drag.take() {
            Some(DragMode::Marquee(marquee)) => {
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
                    self.selection.clear();
                }
                None
            }
            Some(DragMode::Translate {
                start_world_pos,
                last_world_pos,
            }) => {
                let total_delta = last_world_pos - start_world_pos;
                if total_delta.x.abs() > 1e-4 || total_delta.y.abs() > 1e-4 {
                    Some(ruqa_core::history::Command::MoveElements {
                        ids: self.selection.selected_ids().iter().copied().collect(),
                        delta: total_delta,
                    })
                } else {
                    None
                }
            }
            Some(DragMode::Resize {
                initial_elements, ..
            }) => {
                let mut before = Vec::new();
                let mut after = Vec::new();
                for (id, initial_elem) in initial_elements {
                    if let Some(current) = scene.get(id) {
                        if current != &initial_elem {
                            before.push(initial_elem);
                            after.push(current.clone());
                        }
                    }
                }
                if !before.is_empty() {
                    Some(ruqa_core::history::Command::ModifyElements { before, after })
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruqa_core::element::ElementStyle;
    use ruqa_core::geometry::{HandleKind, Rect};

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

    fn down(m: &mut SelectionManager, s: &Scene, wp: Point, sp: Point, shift: bool) {
        let (vp, ws) = (Viewport::default(), Size::new(800.0, 600.0));
        m.handle_pointer_down(s, wp, sp, &vp, ws, shift, 2.0);
    }

    #[test]
    fn test_single_click_and_toggle() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));
        scene.push(sample_rect(2, 100.0, 10.0, 50.0, 50.0));
        let mut manager = SelectionManager::new();

        down(
            &mut manager,
            &scene,
            Point::new(10.0, 10.0),
            Point::new(410.0, 310.0),
            false,
        );
        manager.handle_pointer_up(&scene, false);
        assert_eq!(manager.selection.selected_ids().len(), 1);

        down(
            &mut manager,
            &scene,
            Point::new(100.0, 10.0),
            Point::new(500.0, 310.0),
            true,
        );
        manager.handle_pointer_up(&scene, true);
        assert_eq!(manager.selection.len(), 2);

        down(
            &mut manager,
            &scene,
            Point::new(500.0, 500.0),
            Point::new(900.0, 800.0),
            false,
        );
        manager.handle_pointer_up(&scene, false);
        assert!(manager.selection.is_empty());
    }

    #[test]
    fn test_marquee_drag_selection() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 20.0, 20.0, 40.0, 40.0));
        scene.push(sample_rect(2, 80.0, 20.0, 40.0, 40.0));
        let mut manager = SelectionManager::new();

        down(
            &mut manager,
            &scene,
            Point::ZERO,
            Point::new(400.0, 300.0),
            false,
        );
        assert!(manager.active_marquee().is_some());
        manager.handle_pointer_move(&mut scene, Point::new(150.0, 100.0), false);
        assert!(manager.active_marquee().unwrap().is_significant());
        manager.handle_pointer_up(&scene, false);
        assert_eq!(manager.selection.len(), 2);
    }

    #[test]
    fn test_drag_translate_and_resize() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));
        let mut manager = SelectionManager::new();
        manager.selection.select_single(1);

        // Translate
        down(
            &mut manager,
            &scene,
            Point::new(20.0, 20.0),
            Point::new(420.0, 320.0),
            false,
        );
        assert!(manager.is_translating());
        manager.handle_pointer_move(&mut scene, Point::new(35.0, 30.0), false);
        let cmd = manager.handle_pointer_up(&scene, false);
        assert!(cmd.is_some());
        assert_eq!(
            scene.get(1).unwrap().bounds(),
            Rect::from_xywh(25.0, 20.0, 50.0, 50.0)
        );

        // Resize
        let bounds = scene.selection_bounds([&1]).unwrap();
        let (vp, ws, style) = (
            Viewport::default(),
            Size::new(800.0, 600.0),
            ruqa_render::selection_render::SelectionStyle::default(),
        );
        let screen_rect =
            ruqa_render::selection_render::selection_screen_rect(bounds, &vp, ws, style.padding);
        let br = HandleKind::BottomRight.point_on_rect(screen_rect);

        down(&mut manager, &scene, Point::new(100.0, 100.0), br, false);
        assert!(manager.is_resizing());
        manager.handle_pointer_move(&mut scene, Point::new(150.0, 140.0), false);
        let resize_cmd = manager.handle_pointer_up(&scene, false);
        assert!(resize_cmd.is_some());
        assert!(scene.get(1).unwrap().bounds().width() > 50.0);
    }
}
