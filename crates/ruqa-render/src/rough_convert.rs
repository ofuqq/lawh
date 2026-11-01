//! Converter from `roughr` operations into `tiny_skia::Path`.

use roughr::core::{OpSet, OpType};
use tiny_skia::{Path, PathBuilder};

/// Converts a `roughr` `OpSet` into an optional `tiny_skia::Path`.
///
/// Returns `None` if the operation set contains zero commands or if the path builder fails.
#[must_use]
pub fn opset_to_path(op_set: &OpSet<f32>) -> Option<Path> {
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
        pb.finish()
    } else {
        None
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
        let path = opset_to_path(&op_set);
        assert!(path.is_some());
    }

    #[test]
    fn test_cubic_curve_ops_convert() {
        let op_set = OpSet {
            op_set_type: OpSetType::Path,
            ops: vec![
                Op {
                    op: OpType::Move,
                    data: vec![0.0, 0.0],
                },
                Op {
                    op: OpType::BCurveTo,
                    data: vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0],
                },
            ],
            size: None,
            path: None,
        };
        let path = opset_to_path(&op_set);
        assert!(path.is_some());
    }
}
