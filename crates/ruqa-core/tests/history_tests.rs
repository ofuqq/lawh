//! Integration tests for the command pattern and history stack in `ruqa-core`.

use ruqa_core::element::{Element, ElementId, ElementStyle};
use ruqa_core::geometry::{Point, Rect};
use ruqa_core::history::{Command, HistoryStack};
use ruqa_core::scene::Scene;

fn sample_rect(id: ElementId, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle::default(),
    }
}

#[test]
fn test_complex_history_sequence() {
    let mut scene = Scene::new();
    let mut history = HistoryStack::new();

    // 1. Add element 1
    let e1 = sample_rect(1, 0.0, 0.0, 50.0, 50.0);
    let cmd1 = Command::AddElement(e1.clone());
    cmd1.apply(&mut scene);
    history.push(cmd1);

    // 2. Add element 2
    let e2 = sample_rect(2, 60.0, 0.0, 50.0, 50.0);
    let cmd2 = Command::AddElement(e2.clone());
    cmd2.apply(&mut scene);
    history.push(cmd2);

    // 3. Move elements
    let cmd3 = Command::MoveElements {
        ids: vec![1, 2],
        delta: Point::new(10.0, 20.0),
    };
    cmd3.apply(&mut scene);
    history.push(cmd3);

    assert_eq!(scene.get(1).unwrap().bounds().min, Point::new(10.0, 20.0));
    assert_eq!(scene.get(2).unwrap().bounds().min, Point::new(70.0, 20.0));

    // Undo move
    assert!(history.undo(&mut scene));
    assert_eq!(scene.get(1).unwrap().bounds().min, Point::new(0.0, 0.0));
    assert_eq!(scene.get(2).unwrap().bounds().min, Point::new(60.0, 0.0));

    // Undo add element 2
    assert!(history.undo(&mut scene));
    assert_eq!(scene.len(), 1);
    assert!(scene.get(2).is_none());

    // Redo add element 2
    assert!(history.redo(&mut scene));
    assert_eq!(scene.len(), 2);
    assert!(scene.get(2).is_some());

    // Redo move
    assert!(history.redo(&mut scene));
    assert_eq!(scene.get(1).unwrap().bounds().min, Point::new(10.0, 20.0));
    assert_eq!(scene.get(2).unwrap().bounds().min, Point::new(70.0, 20.0));
}

#[test]
fn test_batch_elements_commands() {
    let mut scene = Scene::new();
    let mut history = HistoryStack::new();

    let e1 = sample_rect(1, 0.0, 0.0, 50.0, 50.0);
    let e2 = sample_rect(2, 50.0, 50.0, 50.0, 50.0);
    let cmd = Command::AddElements(vec![e1.clone(), e2.clone()]);
    cmd.apply(&mut scene);
    history.push(cmd);
    assert_eq!(scene.len(), 2);

    // Batch delete
    let del_cmd = Command::DeleteElements(vec![(0, e1), (1, e2)]);
    del_cmd.apply(&mut scene);
    history.push(del_cmd);
    assert_eq!(scene.len(), 0);

    // Undo restore both with correct indices
    assert!(history.undo(&mut scene));
    assert_eq!(scene.len(), 2);
    assert_eq!(scene.position(1), Some(0));
    assert_eq!(scene.position(2), Some(1));
}
