//! 2D geometry primitives, bounding boxes, and infinite canvas viewport transforms.

pub mod point;
pub mod rect;
pub mod viewport;

pub use point::{Point, Size};
pub use rect::Rect;
pub use viewport::Viewport;
