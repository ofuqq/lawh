//! Document scene graph and element collection manager.
//!
//! Provides the central [`Scene`] container owning an ordered sequence of [`Element`]s,
//! generating unique monotonic element IDs, managing z-index layer ordering, and computing
//! spatial bounds and viewport visibility queries.

use crate::element::{Element, ElementId};
use crate::geometry::{Point, Rect};
use serde::{Deserialize, Serialize};

/// Document scene graph owning whiteboard elements in z-index order.
///
/// Elements at lower indices are rendered first (bottom-most layer); elements at higher
/// indices are rendered on top (top-most layer).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Scene {
    /// Ordered collection of whiteboard canvas elements.
    elements: Vec<Element>,
    /// Monotonic ID counter for assigning unique element IDs.
    next_id: ElementId,
}

impl Scene {
    /// Creates an empty canvas scene with the ID generator starting at 1.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
            next_id: 1,
        }
    }

    /// Creates a scene from an existing collection of elements.
    ///
    /// Automatically synchronizes the next unique ID to be strictly greater than
    /// any existing element ID in the collection.
    #[must_use]
    pub fn from_elements(elements: Vec<Element>) -> Self {
        let max_id = elements.iter().map(Element::id).max().unwrap_or(0);
        Self {
            elements,
            next_id: max_id.saturating_add(1),
        }
    }

    /// Generates a new, unique monotonic element ID.
    pub fn next_element_id(&mut self) -> ElementId {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        id
    }

    /// Adds a new element to the top of the scene (highest z-index layer).
    pub fn push(&mut self, element: Element) {
        if element.id() >= self.next_id {
            self.next_id = element.id().saturating_add(1);
        }
        self.elements.push(element);
    }

    /// Retrieves an immutable reference to an element by its unique identifier.
    #[must_use]
    pub fn get(&self, id: ElementId) -> Option<&Element> {
        self.elements.iter().find(|e| e.id() == id)
    }

    /// Retrieves a mutable reference to an element by its unique identifier.
    #[must_use]
    pub fn get_mut(&mut self, id: ElementId) -> Option<&mut Element> {
        self.elements.iter_mut().find(|e| e.id() == id)
    }

    /// Removes an element from the scene by its unique identifier.
    ///
    /// Returns the removed element, or `None` if no element matched the ID.
    pub fn remove(&mut self, id: ElementId) -> Option<Element> {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == id) {
            Some(self.elements.remove(pos))
        } else {
            None
        }
    }

    /// Returns the number of elements in the scene.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns `true` if the scene contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns a slice of all elements in ascending z-index order.
    #[must_use]
    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    /// Returns an iterator over all elements in ascending z-index order.
    pub fn iter(&self) -> std::slice::Iter<'_, Element> {
        self.elements.iter()
    }

    /// Iterates over all elements whose bounding boxes intersect the specified viewport rectangle.
    ///
    /// This enables coarse spatial culling to skip rasterizing elements that lie
    /// outside the current screen camera view.
    pub fn visible_elements(&self, viewport_rect: Rect) -> impl Iterator<Item = &Element> {
        self.elements
            .iter()
            .filter(move |elem| elem.bounds().intersects(&viewport_rect))
    }

    /// Computes the aggregate bounding box enclosing every element in the scene.
    ///
    /// Returns `None` if the scene contains zero elements.
    #[must_use]
    pub fn aggregate_bounds(&self) -> Option<Rect> {
        let mut iter = self.elements.iter();
        let first = iter.next()?.bounds();
        Some(iter.fold(first, |acc, elem| acc.union(&elem.bounds())))
    }

    /// Computes the aggregate bounding box enclosing the specified element IDs.
    ///
    /// Uses [`Element::bounding_box`] to include stroke width padding and arrowhead geometry.
    /// Returns `None` if `ids` is empty or none of the elements are in the scene.
    #[must_use]
    pub fn selection_bounds<'a, I>(&self, ids: I) -> Option<Rect>
    where
        I: IntoIterator<Item = &'a ElementId>,
    {
        let mut iter = ids
            .into_iter()
            .filter_map(|id| self.get(*id))
            .map(|elem| elem.bounding_box());
        let first = iter.next()?;
        Some(iter.fold(first, |acc, b| acc.union(&b)))
    }

    /// Returns the topmost element (highest z-index) that hits the given world coordinate point.
    #[must_use]
    pub fn hit_test_topmost(&self, point: Point, tolerance: f32) -> Option<&Element> {
        self.elements
            .iter()
            .rev()
            .find(|elem| elem.hit_test(point, tolerance))
    }

    /// Returns an iterator of all elements that hit the given world coordinate point,
    /// ordered from topmost (highest z-index) to bottommost.
    pub fn hit_test_all(&self, point: Point, tolerance: f32) -> impl Iterator<Item = &Element> {
        self.elements
            .iter()
            .rev()
            .filter(move |elem| elem.hit_test(point, tolerance))
    }

    // ── Z-Index Layer Reordering ─────────────────────────────────────────────

    /// Moves the specified element to the top of the z-index stack (rendered last/on top).
    ///
    /// Returns `true` if the element was found and moved.
    pub fn bring_to_front(&mut self, id: ElementId) -> bool {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == id) {
            let last_idx = self.elements.len().saturating_sub(1);
            if pos != last_idx {
                let elem = self.elements.remove(pos);
                self.elements.push(elem);
            }
            true
        } else {
            false
        }
    }

    /// Moves the specified element to the bottom of the z-index stack (rendered first/below all).
    ///
    /// Returns `true` if the element was found and moved.
    pub fn send_to_back(&mut self, id: ElementId) -> bool {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == id) {
            if pos != 0 {
                let elem = self.elements.remove(pos);
                self.elements.insert(0, elem);
            }
            true
        } else {
            false
        }
    }

    /// Steps the specified element one layer forward (towards the top of the stack).
    ///
    /// Returns `true` if the element was found and moved.
    pub fn bring_forward(&mut self, id: ElementId) -> bool {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == id) {
            if pos + 1 < self.elements.len() {
                self.elements.swap(pos, pos + 1);
            }
            true
        } else {
            false
        }
    }

    /// Steps the specified element one layer backward (towards the bottom of the stack).
    ///
    /// Returns `true` if the element was found and moved.
    pub fn send_backward(&mut self, id: ElementId) -> bool {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == id) {
            if pos > 0 {
                self.elements.swap(pos, pos - 1);
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::ElementStyle;
    use crate::geometry::Point;

    fn sample_rect(id: ElementId, x: f32, y: f32, w: f32, h: f32) -> Element {
        Element::Rectangle {
            id,
            bounds: Rect::from_xywh(x, y, w, h),
            style: ElementStyle::default(),
        }
    }

    #[test]
    fn test_monotonic_id_generation() {
        let mut scene = Scene::new();
        assert_eq!(scene.next_element_id(), 1);
        assert_eq!(scene.next_element_id(), 2);
        assert_eq!(scene.next_element_id(), 3);
    }

    #[test]
    fn test_add_get_mutate_remove() {
        let mut scene = Scene::new();
        let id = scene.next_element_id();
        let rect = sample_rect(id, 10.0, 20.0, 100.0, 50.0);
        scene.push(rect);

        assert_eq!(scene.len(), 1);
        assert!(!scene.is_empty());
        assert_eq!(scene.get(id).map(Element::id), Some(id));
        assert!(scene.get(999).is_none());

        if let Some(elem) = scene.get_mut(id) {
            elem.style_mut().stroke_color = "#FF5555".into();
        }
        assert_eq!(scene.get(id).unwrap().style().stroke_color, "#FF5555");

        let removed = scene.remove(id);
        assert!(removed.is_some());
        assert_eq!(scene.len(), 0);
        assert!(scene.is_empty());
    }

    #[test]
    fn test_aggregate_bounds() {
        let mut scene = Scene::new();
        assert_eq!(scene.aggregate_bounds(), None);

        scene.push(sample_rect(1, 10.0, 10.0, 20.0, 20.0));
        assert_eq!(
            scene.aggregate_bounds(),
            Some(Rect::from_xywh(10.0, 10.0, 20.0, 20.0))
        );

        scene.push(sample_rect(2, 100.0, 100.0, 50.0, 50.0));
        let agg = scene.aggregate_bounds().unwrap();
        assert_eq!(agg.min, Point::new(10.0, 10.0));
        assert_eq!(agg.max, Point::new(150.0, 150.0));
    }

    #[test]
    fn test_visible_elements_culling() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 0.0, 0.0, 50.0, 50.0));
        scene.push(sample_rect(2, 200.0, 200.0, 50.0, 50.0));
        scene.push(sample_rect(3, 500.0, 500.0, 50.0, 50.0));

        let viewport = Rect::from_xywh(0.0, 0.0, 100.0, 100.0);
        let visible: Vec<ElementId> = scene.visible_elements(viewport).map(Element::id).collect();
        assert_eq!(visible, vec![1]);

        let wide_viewport = Rect::from_xywh(0.0, 0.0, 300.0, 300.0);
        let visible_wide: Vec<ElementId> = scene
            .visible_elements(wide_viewport)
            .map(Element::id)
            .collect();
        assert_eq!(visible_wide, vec![1, 2]);
    }

    #[test]
    fn test_z_index_reordering() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 0.0, 0.0, 10.0, 10.0));
        scene.push(sample_rect(2, 0.0, 0.0, 10.0, 10.0));
        scene.push(sample_rect(3, 0.0, 0.0, 10.0, 10.0));

        let ids: Vec<ElementId> = scene.iter().map(Element::id).collect();
        assert_eq!(ids, vec![1, 2, 3]);

        // Bring 1 forward: [2, 1, 3]
        assert!(scene.bring_forward(1));
        let ids: Vec<ElementId> = scene.iter().map(Element::id).collect();
        assert_eq!(ids, vec![2, 1, 3]);

        // Bring 1 to front: [2, 3, 1]
        assert!(scene.bring_to_front(1));
        let ids: Vec<ElementId> = scene.iter().map(Element::id).collect();
        assert_eq!(ids, vec![2, 3, 1]);

        // Send 1 backward: [2, 1, 3]
        assert!(scene.send_backward(1));
        let ids: Vec<ElementId> = scene.iter().map(Element::id).collect();
        assert_eq!(ids, vec![2, 1, 3]);

        // Send 3 to back: [3, 2, 1]
        assert!(scene.send_to_back(3));
        let ids: Vec<ElementId> = scene.iter().map(Element::id).collect();
        assert_eq!(ids, vec![3, 2, 1]);
    }

    #[test]
    fn test_scene_serde_round_trip() {
        let mut scene = Scene::new();
        scene.push(sample_rect(10, 50.0, 50.0, 100.0, 80.0));
        scene.push(sample_rect(20, 120.0, 90.0, 60.0, 40.0));

        let json = serde_json::to_string(&scene).expect("serialization succeeds");
        let deserialized: Scene = serde_json::from_str(&json).expect("deserialization succeeds");
        assert_eq!(scene, deserialized);
    }
}
