//! 2D geometry primitives, bounding boxes, and infinite canvas viewport transforms.

pub mod handle;
pub mod point;
pub mod rect;
pub mod viewport;

pub use handle::{compute_resized_bounds, HandleKind, MIN_RESIZE_DIMENSION};
pub use point::{Point, Size};
pub use rect::Rect;
pub use viewport::Viewport;
