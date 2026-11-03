//! Active interactive shape creation and freehand pen stroke state machine.

use ruqa_core::element::{Element, ElementId};
use ruqa_core::geometry::{Point, Rect};
use ruqa_core::stroke::{InputPoint, StrokeSmoother};
use ruqa_core::style::ElementStyle;
use ruqa_core::tool::Tool;

/// Minimum drag delta in world coordinates to commit a shape or connector.
const MIN_DRAG_THRESHOLD: f32 = 2.0;

/// Active shape drawing session in progress on the canvas.
#[derive(Debug, Clone)]
pub enum ActiveDrawing {
    /// Two-point geometric shape drag (Rectangle, Ellipse, Diamond).
    Shape {
        /// Active geometric shape tool.
        tool: Tool,
        /// Initial pointer press anchor coordinate.
        start: Point,
        /// Current live cursor coordinate.
        current: Point,
    },
    /// Linear segment or arrow connector drag (Line, Arrow).
    Connector {
        /// Active connector tool.
        tool: Tool,
        /// Initial pointer press start coordinate.
        start: Point,
        /// Current live tip coordinate.
        current: Point,
    },
    /// Freehand pen stroke accumulating smoothed coordinates.
    Pen {
        /// Dynamic stroke smoother accumulating raw inputs.
        smoother: StrokeSmoother,
        /// Initial gesture timestamp in milliseconds.
        start_time_ms: f64,
    },
}

/// Manages interactive shape creation and pen stroke gestures.
#[derive(Debug, Default)]
pub struct DrawingManager {
    /// Active interactive drag state, if a drawing gesture is underway.
    pub active: Option<ActiveDrawing>,
}

impl DrawingManager {
    /// Creates a new, idle drawing manager.
    #[must_use]
    pub const fn new() -> Self {
        Self { active: None }
    }

    /// Returns `true` if a drawing gesture is currently in progress.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.active.is_some()
    }

    /// Begins a new drawing gesture at the specified canvas position.
    pub fn start_drawing(&mut self, tool: Tool, world_pt: Point, timestamp_ms: f64) {
        match tool {
            Tool::Rectangle | Tool::Ellipse | Tool::Diamond => {
                self.active = Some(ActiveDrawing::Shape {
                    tool,
                    start: world_pt,
                    current: world_pt,
                });
            }
            Tool::Line | Tool::Arrow => {
                self.active = Some(ActiveDrawing::Connector {
                    tool,
                    start: world_pt,
                    current: world_pt,
                });
            }
            Tool::Pen => {
                let mut smoother = StrokeSmoother::new();
                smoother.add_point(InputPoint::new(world_pt.x, world_pt.y, 0.0));
                self.active = Some(ActiveDrawing::Pen {
                    smoother,
                    start_time_ms: timestamp_ms,
                });
            }
            _ => {
                self.active = None;
            }
        }
    }

    /// Updates the active drawing gesture with the latest cursor position.
    pub fn update_drawing(&mut self, world_pt: Point, timestamp_ms: f64) -> bool {
        match &mut self.active {
            Some(ActiveDrawing::Shape { current, .. })
            | Some(ActiveDrawing::Connector { current, .. }) => {
                *current = world_pt;
                true
            }
            Some(ActiveDrawing::Pen {
                smoother,
                start_time_ms,
            }) => {
                let rel_time = (timestamp_ms - *start_time_ms).max(0.0);
                smoother.add_point(InputPoint::new(world_pt.x, world_pt.y, rel_time));
                true
            }
            None => false,
        }
    }

    /// Generates a temporary live preview [`Element`] for rendering while dragging.
    #[must_use]
    pub fn preview_element(&self, style: &ElementStyle) -> Option<Element> {
        let mut style = style.clone();
        style.seed = 0x85EB_CA6B;

        match self.active.as_ref()? {
            ActiveDrawing::Shape {
                tool,
                start,
                current,
            } => {
                let bounds = Rect::from_min_max(*start, *current);
                match tool {
                    Tool::Rectangle => Some(Element::Rectangle {
                        id: 0,
                        bounds,
                        style,
                    }),
                    Tool::Ellipse => Some(Element::Ellipse {
                        id: 0,
                        bounds,
                        style,
                    }),
                    Tool::Diamond => Some(Element::Diamond {
                        id: 0,
                        bounds,
                        style,
                    }),
                    _ => None,
                }
            }
            ActiveDrawing::Connector {
                tool,
                start,
                current,
            } => match tool {
                Tool::Line => Some(Element::Line {
                    id: 0,
                    start: *start,
                    end: *current,
                    style,
                }),
                Tool::Arrow => Some(Element::Arrow {
                    id: 0,
                    start: *start,
                    end: *current,
                    style,
                }),
                _ => None,
            },
            ActiveDrawing::Pen { smoother, .. } => {
                let (points, pressures) = smoother.finish();
                if points.is_empty() {
                    None
                } else {
                    Some(Element::Stroke {
                        id: 0,
                        points,
                        pressures,
                        style,
                    })
                }
            }
        }
    }

    /// Completes the drawing gesture and commits the final [`Element`], if size exceeds threshold.
    pub fn finish_drawing(&mut self, id: ElementId, style: &ElementStyle) -> Option<Element> {
        let active = self.active.take()?;
        let mut style = style.clone();
        // Give each committed element a unique organic seed based on its monotonic ID
        style.seed = id.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0x85EB_CA6B_C75B_1FE1;

        match active {
            ActiveDrawing::Shape {
                tool,
                start,
                current,
            } => {
                let bounds = Rect::from_min_max(start, current);
                if bounds.width().max(bounds.height()) < MIN_DRAG_THRESHOLD {
                    return None;
                }
                match tool {
                    Tool::Rectangle => Some(Element::Rectangle { id, bounds, style }),
                    Tool::Ellipse => Some(Element::Ellipse { id, bounds, style }),
                    Tool::Diamond => Some(Element::Diamond { id, bounds, style }),
                    _ => None,
                }
            }
            ActiveDrawing::Connector {
                tool,
                start,
                current,
            } => {
                let length = (current.x - start.x).hypot(current.y - start.y);
                if length < MIN_DRAG_THRESHOLD {
                    return None;
                }
                match tool {
                    Tool::Line => Some(Element::Line {
                        id,
                        start,
                        end: current,
                        style,
                    }),
                    Tool::Arrow => Some(Element::Arrow {
                        id,
                        start,
                        end: current,
                        style,
                    }),
                    _ => None,
                }
            }
            ActiveDrawing::Pen { smoother, .. } => {
                let (points, pressures) = smoother.finish();
                if points.is_empty() {
                    None
                } else {
                    Some(Element::Stroke {
                        id,
                        points,
                        pressures,
                        style,
                    })
                }
            }
        }
    }

    /// Cancels any active drawing gesture without committing.
    pub fn cancel(&mut self) {
        self.active = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_drawing_lifecycle() {
        let mut dm = DrawingManager::new();
        assert!(!dm.is_active());

        dm.start_drawing(Tool::Rectangle, Point::new(10.0, 10.0), 0.0);
        assert!(dm.is_active());

        let moved = dm.update_drawing(Point::new(50.0, 40.0), 10.0);
        assert!(moved);

        let style = ElementStyle::default();
        let preview = dm.preview_element(&style).expect("preview exists");
        if let Element::Rectangle { bounds, .. } = preview {
            assert_eq!(bounds.width(), 40.0);
            assert_eq!(bounds.height(), 30.0);
        } else {
            panic!("Expected Rectangle element");
        }

        let committed = dm.finish_drawing(42, &style).expect("committed element");
        assert_eq!(committed.id(), 42);
        assert!(!dm.is_active());
    }

    #[test]
    fn test_sub_threshold_drag_discarded() {
        let mut dm = DrawingManager::new();
        dm.start_drawing(Tool::Line, Point::new(10.0, 10.0), 0.0);
        dm.update_drawing(Point::new(10.5, 10.5), 5.0);

        let style = ElementStyle::default();
        let committed = dm.finish_drawing(1, &style);
        assert!(committed.is_none(), "Small twitch drag should be discarded");
    }

    #[test]
    fn test_pen_stroke_lifecycle() {
        let mut dm = DrawingManager::new();
        dm.start_drawing(Tool::Pen, Point::new(0.0, 0.0), 0.0);
        dm.update_drawing(Point::new(10.0, 10.0), 10.0);
        dm.update_drawing(Point::new(20.0, 20.0), 20.0);

        let style = ElementStyle::default();
        let committed = dm.finish_drawing(99, &style).expect("committed stroke");
        if let Element::Stroke {
            id,
            points,
            pressures,
            ..
        } = committed
        {
            assert_eq!(id, 99);
            assert!(!points.is_empty());
            assert_eq!(points.len(), pressures.len());
        } else {
            panic!("Expected Stroke element");
        }
    }
}
