//! Continuous stroke and element eraser tool state machine and trail intersection.

use crate::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementId};
use ruqa_core::geometry::Point;
use ruqa_core::history::Command;
use std::collections::HashMap;

/// Default eraser contact tolerance radius in world units (12.0 logical px).
pub const DEFAULT_ERASER_TOLERANCE: f32 = 12.0;

/// State tracker for an active continuous eraser drag gesture.
#[derive(Debug, Clone, Default)]
pub struct EraserState {
    /// Whether an eraser drag gesture is actively in progress.
    pub is_erasing: bool,
    /// Elements erased during this continuous gesture, paired with their initial z-index.
    pub erased_elements: Vec<(usize, Element)>,
    /// Snapshot of element IDs mapped to their original z-indices before the drag began.
    pub initial_indices: HashMap<ElementId, usize>,
    /// Last sampled pointer world coordinate along the drag trail.
    pub last_world_pos: Option<Point>,
}

impl EraserState {
    /// Creates a new inactive eraser state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets active eraser drag state.
    pub fn reset(&mut self) {
        self.is_erasing = false;
        self.erased_elements.clear();
        self.initial_indices.clear();
        self.last_world_pos = None;
    }
}

/// Tests whether an element is intersected by an eraser contact point with given tolerance.
///
/// Geometric shapes are erased if the contact point touches their bounding box.
/// Freehand strokes, lines, and arrows are erased if the contact point touches their path within tolerance.
#[must_use]
pub fn eraser_hit_test(element: &Element, point: Point, tolerance: f32) -> bool {
    let tolerance = tolerance.max(0.0);
    if !element.bounding_box().inflate(tolerance).contains(point) {
        return false;
    }
    match element {
        Element::Stroke { .. } | Element::Line { .. } | Element::Arrow { .. } => {
            element.hit_test(point, tolerance)
        }
        Element::Rectangle { .. } | Element::Ellipse { .. } | Element::Diamond { .. } => true,
    }
}

impl WhiteboardApp {
    /// Handles mouse press for the eraser tool at `world_pt`.
    pub fn handle_eraser_pointer_down(&mut self, world_pt: Point) -> bool {
        self.eraser.is_erasing = true;
        self.eraser.erased_elements.clear();
        self.eraser.initial_indices = self
            .scene
            .elements()
            .iter()
            .enumerate()
            .map(|(idx, e)| (e.id(), idx))
            .collect();
        self.eraser.last_world_pos = Some(world_pt);

        self.erase_elements_at(world_pt, DEFAULT_ERASER_TOLERANCE);
        true
    }

    /// Handles continuous mouse motion while the eraser is dragging along `world_pt`.
    pub fn handle_eraser_pointer_move(&mut self, world_pt: Point) -> bool {
        if !self.eraser.is_erasing {
            return false;
        }

        let prev_pt = self.eraser.last_world_pos.unwrap_or(world_pt);
        self.eraser.last_world_pos = Some(world_pt);

        self.erase_along_trail(prev_pt, world_pt, DEFAULT_ERASER_TOLERANCE);
        true
    }

    /// Handles mouse release, committing all erased elements as a single atomic [`Command::DeleteElements`].
    pub fn handle_eraser_pointer_up(&mut self) -> bool {
        if !self.eraser.is_erasing {
            return false;
        }

        self.eraser.is_erasing = false;
        self.eraser.initial_indices.clear();
        self.eraser.last_world_pos = None;

        if !self.eraser.erased_elements.is_empty() {
            let mut entries = std::mem::take(&mut self.eraser.erased_elements);
            entries.sort_by_key(|(pos, _)| *pos);
            self.history.push(Command::DeleteElements(entries));
            self.mark_dirty();
            self.request_redraw();
            true
        } else {
            false
        }
    }

    /// Samples along the line segment between `prev` and `curr` to erase all intersected elements.
    pub fn erase_along_trail(&mut self, prev: Point, curr: Point, tolerance: f32) -> bool {
        let dist = prev.distance(curr);
        let step_size = (tolerance * 0.75).max(2.0);

        let mut erased_any = false;
        if dist <= step_size {
            erased_any |= self.erase_elements_at(curr, tolerance);
        } else {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let steps = ((dist / step_size).ceil() as usize).max(1);
            for i in 1..=steps {
                #[allow(clippy::cast_precision_loss)]
                let t = i as f32 / steps as f32;
                let sample_pt = Point::new(
                    prev.x + t * (curr.x - prev.x),
                    prev.y + t * (curr.y - prev.y),
                );
                erased_any |= self.erase_elements_at(sample_pt, tolerance);
            }
        }

        erased_any
    }

    /// Tests for elements intersecting `point` within `tolerance`, immediately removing touched shapes.
    pub fn erase_elements_at(&mut self, point: Point, tolerance: f32) -> bool {
        let hit_ids: Vec<ElementId> = self
            .scene
            .elements()
            .iter()
            .rev()
            .filter(|e| eraser_hit_test(e, point, tolerance))
            .map(Element::id)
            .collect();

        if hit_ids.is_empty() {
            return false;
        }

        let mut erased = false;
        for id in hit_ids {
            let pos = self
                .eraser
                .initial_indices
                .get(&id)
                .copied()
                .or_else(|| self.scene.position(id))
                .unwrap_or(0);

            if let Some(elem) = self.scene.remove(id) {
                self.selection.deselect(id);
                self.eraser.erased_elements.push((pos, elem));
                erased = true;
            }
        }

        if erased {
            self.mark_dirty();
            self.request_redraw();
        }

        erased
    }
}
