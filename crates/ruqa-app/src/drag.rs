//! Drag mode enumeration and marquee drag tracking for selection and manipulation.

use ruqa_core::element::{Element, ElementId};
use ruqa_core::geometry::{HandleKind, Point, Rect};
use std::collections::HashMap;

/// Minimum drag distance in world units to distinguish a click from a marquee drag.
pub const MARQUEE_DRAG_THRESHOLD: f32 = 3.0;

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
#[derive(Debug, Clone, PartialEq)]
pub enum DragMode {
    /// Rectangular marquee selection drag.
    Marquee(MarqueeDrag),
    /// Translating selected elements by tracking start and previous world positions.
    Translate {
        /// World coordinates where translation drag initiated.
        start_world_pos: Point,
        /// World coordinates of the last pointer position during this drag.
        last_world_pos: Point,
    },
    /// Interactively resizing selected elements from a selection handle.
    Resize {
        /// Which of the 8 selection handles is being dragged.
        handle: HandleKind,
        /// Initial bounding box of the selection before resizing began.
        initial_bounds: Rect,
        /// Snapshot of selected elements in their initial pre-resize state.
        initial_elements: HashMap<ElementId, Element>,
    },
}
