//! Integration tests for scene document model and z-index reordering.

use ruqa_core::element::{Element, ElementId, ElementStyle};
use ruqa_core::geometry::Rect;
use ruqa_core::scene::Scene;

fn sample_rect(id: ElementId, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle {
            stroke_width: 2.0,
            ..Default::default()
        },
    }
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
