//! Integration tests for roughr to tiny-skia path conversion.

use roughr::core::{Drawable, Op, OpSet, OpSetType, OpType};
use ruqa_render::rough_convert::{
    default_rough_stroke, drawable_to_paths, opset_to_path, opset_to_path_closed,
    stroke_with_capping,
};
use tiny_skia::{LineCap, LineJoin};

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
fn test_valid_line_ops_convert_open_and_closed() {
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
            Op {
                op: OpType::LineTo,
                data: vec![50.0, 10.0],
            },
        ],
        size: None,
        path: None,
    };
    let open_path = opset_to_path(&op_set).expect("open path should construct");
    let closed_path = opset_to_path_closed(&op_set).expect("closed path should construct");

    let open_bounds = open_path.bounds();
    let closed_bounds = closed_path.bounds();
    assert_eq!(open_bounds.left(), 10.0);
    assert_eq!(open_bounds.right(), 50.0);
    assert_eq!(open_bounds.top(), 10.0);
    assert_eq!(open_bounds.bottom(), 50.0);
    assert_eq!(open_bounds, closed_bounds);
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

#[test]
fn test_drawable_to_paths() {
    let drawable = Drawable {
        shape: "rectangle".into(),
        options: roughr::core::Options::default(),
        sets: vec![
            OpSet {
                op_set_type: OpSetType::Path,
                ops: vec![
                    Op {
                        op: OpType::Move,
                        data: vec![0.0, 0.0],
                    },
                    Op {
                        op: OpType::LineTo,
                        data: vec![20.0, 20.0],
                    },
                ],
                size: None,
                path: None,
            },
            OpSet {
                op_set_type: OpSetType::Path,
                ops: vec![
                    Op {
                        op: OpType::Move,
                        data: vec![20.0, 20.0],
                    },
                    Op {
                        op: OpType::LineTo,
                        data: vec![40.0, 40.0],
                    },
                ],
                size: None,
                path: None,
            },
        ],
    };
    let paths = drawable_to_paths(&drawable);
    assert_eq!(paths.len(), 2);
}

#[test]
fn test_stroke_capping_and_styling() {
    let default_stroke = default_rough_stroke(2.5);
    assert_eq!(default_stroke.width, 2.5);
    assert_eq!(default_stroke.line_cap, LineCap::Round);
    assert_eq!(default_stroke.line_join, LineJoin::Round);

    let custom_stroke = stroke_with_capping(4.0, LineCap::Square, LineJoin::Miter);
    assert_eq!(custom_stroke.width, 4.0);
    assert_eq!(custom_stroke.line_cap, LineCap::Square);
    assert_eq!(custom_stroke.line_join, LineJoin::Miter);
}
