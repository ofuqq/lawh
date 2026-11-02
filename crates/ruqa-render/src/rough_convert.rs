//! Converter from `roughr` operations into `tiny_skia::Path`.
//!
//! Maps renderer-agnostic [`roughr::core::OpSet`] and [`roughr::core::Drawable`]
//! vector operations into rasterizable [`tiny_skia::Path`] geometries with
//! configurable stroke capping and closing.

use roughr::core::{Drawable, OpSet, OpType};
use tiny_skia::{LineCap, LineJoin, Path, PathBuilder, Stroke};

/// Converts a `roughr` [`OpSet`] into an open [`tiny_skia::Path`].
///
/// Returns `None` if the operation set contains zero valid drawing commands.
#[must_use]
pub fn opset_to_path(op_set: &OpSet<f32>) -> Option<Path> {
    build_path_from_opset(op_set, false)
}

/// Converts a `roughr` [`OpSet`] into a closed [`tiny_skia::Path`].
///
/// Returns `None` if the operation set contains zero valid drawing commands.
#[must_use]
pub fn opset_to_path_closed(op_set: &OpSet<f32>) -> Option<Path> {
    build_path_from_opset(op_set, true)
}

/// Converts all operation sets in a `roughr` [`Drawable`] into a list of [`tiny_skia::Path`]s.
#[must_use]
pub fn drawable_to_paths(drawable: &Drawable<f32>) -> Vec<Path> {
    drawable.sets.iter().filter_map(opset_to_path).collect()
}

/// Helper to build a `tiny_skia::Path` with optional path closing.
fn build_path_from_opset(op_set: &OpSet<f32>, close: bool) -> Option<Path> {
    let mut pb = PathBuilder::new();
    let mut has_ops = false;

    for op in &op_set.ops {
        match op.op {
            OpType::Move => {
                if op.data.len() >= 2 {
                    pb.move_to(op.data[0], op.data[1]);
                    has_ops = true;
                }
            }
            OpType::LineTo => {
                if op.data.len() >= 2 {
                    pb.line_to(op.data[0], op.data[1]);
                    has_ops = true;
                }
            }
            OpType::BCurveTo => {
                if op.data.len() >= 6 {
                    pb.cubic_to(
                        op.data[0], op.data[1], op.data[2], op.data[3], op.data[4], op.data[5],
                    );
                    has_ops = true;
                }
            }
        }
    }

    if has_ops {
        if close {
            pb.close();
        }
        pb.finish()
    } else {
        None
    }
}

/// Creates a default hand-drawn stroke with round caps and round joins.
#[must_use]
pub fn default_rough_stroke(width: f32) -> Stroke {
    stroke_with_capping(width, LineCap::Round, LineJoin::Round)
}

/// Creates a `tiny_skia::Stroke` with explicit line capping and join styling.
#[must_use]
pub fn stroke_with_capping(width: f32, line_cap: LineCap, line_join: LineJoin) -> Stroke {
    Stroke {
        width,
        line_cap,
        line_join,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roughr::core::{Op, OpSetType};

    #[test]
    fn test_empty_opset_returns_none() {
        let empty_set = OpSet {
            op_set_type: OpSetType::Path,
            ops: Vec::new(),
            size: None,
            path: None,
        };
        assert!(opset_to_path(&empty_set).is_none());
        assert!(opset_to_path_closed(&empty_set).is_none());
    }

    #[test]
    fn test_valid_line_ops_convert() {
        let op_set = OpSet {
            op_set_type: OpSetType::Path,
            ops: vec![
                Op {
                    op: OpType::Move,
                    data: vec![10.0, 10.0],
                },
                Op {
                    op: OpType::LineTo,
                    data: vec![50.0, 50.0],
                },
            ],
            size: None,
            path: None,
        };
        assert!(opset_to_path(&op_set).is_some());
        assert!(opset_to_path_closed(&op_set).is_some());
    }

    #[test]
    fn test_stroke_capping_defaults() {
        let stroke = default_rough_stroke(3.0);
        assert_eq!(stroke.width, 3.0);
        assert_eq!(stroke.line_cap, LineCap::Round);
        assert_eq!(stroke.line_join, LineJoin::Round);
    }
}
