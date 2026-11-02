//! Command-pattern undo and redo history stack.
//!
//! Provides invertible [`Command`] operations and a configurable [`HistoryStack`]
//! supporting discrete actions (add, delete, move, modify), unlimited or capped depth,
//! and linear redo truncation upon new mutations.

use crate::element::{Element, ElementId};
use crate::geometry::Point;
use crate::scene::Scene;

/// Reversible editing command applied to a [`Scene`].
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Adds a single element to the scene.
    AddElement(Element),
    /// Adds multiple elements to the scene.
    AddElements(Vec<Element>),
    /// Deletes a single element, storing its original z-index for restoration.
    DeleteElement {
        /// Element to delete.
        element: Element,
        /// Original index within the scene element list.
        index: usize,
    },
    /// Deletes multiple elements, storing each with its original z-index for restoration.
    DeleteElements(Vec<(usize, Element)>),
    /// Translates a collection of elements by a delta in world units.
    MoveElements {
        /// IDs of the translated elements.
        ids: Vec<ElementId>,
        /// Translation delta.
        delta: Point,
    },
    /// Modifies a single element in place.
    ModifyElement {
        /// State before modification.
        before: Element,
        /// State after modification.
        after: Element,
    },
    /// Modifies multiple elements in place.
    ModifyElements {
        /// Elements before modification.
        before: Vec<Element>,
        /// Elements after modification.
        after: Vec<Element>,
    },
}

impl Command {
    /// Applies the forward command mutation to `scene`.
    pub fn apply(&self, scene: &mut Scene) {
        match self {
            Self::AddElement(elem) => {
                scene.push(elem.clone());
            }
            Self::AddElements(elems) => {
                for elem in elems {
                    scene.push(elem.clone());
                }
            }
            Self::DeleteElement { element, .. } => {
                scene.remove(element.id());
            }
            Self::DeleteElements(entries) => {
                for (_, elem) in entries {
                    scene.remove(elem.id());
                }
            }
            Self::MoveElements { ids, delta } => {
                scene.translate_elements(ids, *delta);
            }
            Self::ModifyElement { after, .. } => {
                if let Some(elem) = scene.get_mut(after.id()) {
                    *elem = after.clone();
                }
            }
            Self::ModifyElements { after, .. } => {
                for new_elem in after {
                    if let Some(elem) = scene.get_mut(new_elem.id()) {
                        *elem = new_elem.clone();
                    }
                }
            }
        }
    }

    /// Reverses the command mutation on `scene`.
    pub fn undo(&self, scene: &mut Scene) {
        match self {
            Self::AddElement(elem) => {
                scene.remove(elem.id());
            }
            Self::AddElements(elems) => {
                for elem in elems {
                    scene.remove(elem.id());
                }
            }
            Self::DeleteElement { element, index } => {
                scene.insert(*index, element.clone());
            }
            Self::DeleteElements(entries) => {
                let mut sorted = entries.clone();
                sorted.sort_by_key(|(idx, _)| *idx);
                for (idx, elem) in sorted {
                    scene.insert(idx, elem);
                }
            }
            Self::MoveElements { ids, delta } => {
                let inverse_delta = Point::new(-delta.x, -delta.y);
                scene.translate_elements(ids, inverse_delta);
            }
            Self::ModifyElement { before, .. } => {
                if let Some(elem) = scene.get_mut(before.id()) {
                    *elem = before.clone();
                }
            }
            Self::ModifyElements { before, .. } => {
                for old_elem in before {
                    if let Some(elem) = scene.get_mut(old_elem.id()) {
                        *elem = old_elem.clone();
                    }
                }
            }
        }
    }
}

/// Command-pattern undo and redo history manager.
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryStack {
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
    max_depth: Option<usize>,
}

impl Default for HistoryStack {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryStack {
    /// Creates a history stack with default depth ceiling of 100 steps.
    #[must_use]
    pub fn new() -> Self {
        Self::with_max_depth(Some(100))
    }

    /// Creates an unbounded history stack with unlimited undo/redo steps.
    #[must_use]
    pub fn unlimited() -> Self {
        Self::with_max_depth(None)
    }

    /// Creates a history stack with configurable maximum depth (`None` for unlimited).
    #[must_use]
    pub fn with_max_depth(max_depth: Option<usize>) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_depth,
        }
    }

    /// Pushes a newly executed command onto the undo stack and clears redo history.
    pub fn push(&mut self, command: Command) {
        self.undo_stack.push(command);
        if let Some(max) = self.max_depth {
            if self.undo_stack.len() > max {
                self.undo_stack.remove(0);
            }
        }
        self.redo_stack.clear();
    }

    /// Undoes the topmost command on `scene`, returning `true` if an action was undone.
    pub fn undo(&mut self, scene: &mut Scene) -> bool {
        let Some(command) = self.undo_stack.pop() else {
            return false;
        };
        command.undo(scene);
        self.redo_stack.push(command);
        true
    }

    /// Redoes the topmost undone command on `scene`, returning `true` if an action was redone.
    pub fn redo(&mut self, scene: &mut Scene) -> bool {
        let Some(command) = self.redo_stack.pop() else {
            return false;
        };
        command.apply(scene);
        self.undo_stack.push(command);
        true
    }

    /// Returns `true` if there are reversible commands on the undo stack.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns `true` if there are commands that can be redone.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Returns the number of commands currently on the undo stack.
    #[must_use]
    pub fn undo_len(&self) -> usize {
        self.undo_stack.len()
    }

    /// Returns the number of commands currently on the redo stack.
    #[must_use]
    pub fn redo_len(&self) -> usize {
        self.redo_stack.len()
    }

    /// Returns the configured maximum undo depth.
    #[must_use]
    pub fn max_depth(&self) -> Option<usize> {
        self.max_depth
    }

    /// Clears all undo and redo history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::ElementStyle;
    use crate::geometry::Rect;

    fn sample_rect(id: ElementId, x: f32, y: f32, w: f32, h: f32) -> Element {
        Element::Rectangle {
            id,
            bounds: Rect::from_xywh(x, y, w, h),
            style: ElementStyle::default(),
        }
    }

    #[test]
    fn test_add_and_undo_element() {
        let mut scene = Scene::new();
        let mut history = HistoryStack::new();

        let elem = sample_rect(1, 0.0, 0.0, 50.0, 50.0);
        let cmd = Command::AddElement(elem.clone());
        cmd.apply(&mut scene);
        history.push(cmd);
        assert!(history.can_undo());

        assert!(history.undo(&mut scene));
        assert_eq!(scene.len(), 0);
        assert!(history.redo(&mut scene));
        assert_eq!(scene.len(), 1);
    }

    #[test]
    fn test_delete_and_undo_element_restores_index() {
        let mut scene = Scene::new();
        let mut history = HistoryStack::new();
        scene.push(sample_rect(1, 0.0, 0.0, 50.0, 50.0));
        scene.push(sample_rect(2, 60.0, 0.0, 50.0, 50.0));

        let elem2 = scene.get(2).cloned().unwrap();
        let cmd = Command::DeleteElement {
            element: elem2,
            index: 1,
        };
        cmd.apply(&mut scene);
        history.push(cmd);
        assert!(history.undo(&mut scene));
        assert_eq!(scene.position(2), Some(1));
    }

    #[test]
    fn test_new_action_truncates_redo_history() {
        let mut scene = Scene::new();
        let mut history = HistoryStack::new();
        history.push(Command::AddElement(sample_rect(1, 0.0, 0.0, 50.0, 50.0)));
        history.undo(&mut scene);
        assert_eq!(history.redo_len(), 1);

        history.push(Command::AddElement(sample_rect(2, 50.0, 50.0, 50.0, 50.0)));
        assert_eq!(history.redo_len(), 0);
        assert!(!history.can_redo());
    }

    #[test]
    fn test_depth_and_empty_stack_safety() {
        let mut scene = Scene::new();
        let mut history = HistoryStack::with_max_depth(Some(2));
        assert!(!history.undo(&mut scene));
        assert!(!history.redo(&mut scene));

        for i in 1..=4 {
            let cmd = Command::AddElement(sample_rect(i, 0.0, 0.0, 10.0, 10.0));
            cmd.apply(&mut scene);
            history.push(cmd);
        }
        assert_eq!(history.undo_len(), 2);
    }
}
