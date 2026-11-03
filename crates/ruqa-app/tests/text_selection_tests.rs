use ruqa_app::selection::SelectionManager;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::{Point, Size, Viewport};
use ruqa_core::scene::Scene;
use ruqa_render::measure_text_dimensions;

fn sample_text(id: u64, x: f32, y: f32, content: &str, font_size: f32) -> Element {
    Element::Text {
        id,
        position: Point::new(x, y),
        content: content.to_string(),
        font_size,
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            ..Default::default()
        },
    }
}

#[test]
fn test_single_click_selects_text_element() {
    let mut scene = Scene::new();
    let text_elem = sample_text(1, 100.0, 100.0, "خوارزمية Dijkstra", 20.0);
    let bounds = text_elem.bounds();
    scene.push(text_elem);

    let mut manager = SelectionManager::new();
    let vp = Viewport::default();
    let ws = Size::new(800.0, 600.0);

    // Click inside the text element
    let click_world = Point::new(bounds.min.x + 10.0, bounds.min.y + 10.0);
    let click_screen = vp.world_to_screen(click_world, ws);
    manager.handle_pointer_down(&scene, click_world, click_screen, &vp, ws, false, 4.0);
    manager.handle_pointer_up(&scene, false);

    assert_eq!(manager.selected_ids().len(), 1);
    assert!(manager.is_selected(1));

    // Click outside deselects
    let outside_world = Point::new(bounds.max.x + 50.0, bounds.max.y + 50.0);
    let outside_screen = vp.world_to_screen(outside_world, ws);
    manager.handle_pointer_down(&scene, outside_world, outside_screen, &vp, ws, false, 4.0);
    manager.handle_pointer_up(&scene, false);

    assert!(manager.is_empty());
}

#[test]
fn test_marquee_selection_intersects_text_element() {
    let mut scene = Scene::new();
    let text1 = sample_text(1, 100.0, 100.0, "Text One", 16.0);
    let text2 = sample_text(2, 400.0, 400.0, "Text Two", 16.0);
    scene.push(text1);
    scene.push(text2);

    let mut manager = SelectionManager::new();
    let vp = Viewport::default();
    let ws = Size::new(800.0, 600.0);

    // Marquee enclosing Text 1 only
    let start_world = Point::new(50.0, 50.0);
    let start_screen = vp.world_to_screen(start_world, ws);
    manager.handle_pointer_down(&scene, start_world, start_screen, &vp, ws, false, 4.0);

    let end_world = Point::new(250.0, 200.0);
    manager.handle_pointer_move(&mut scene, end_world, false);
    manager.handle_pointer_up(&scene, false);

    assert_eq!(manager.selected_ids().len(), 1);
    assert!(manager.is_selected(1));
    assert!(!manager.is_selected(2));
}

#[test]
fn test_measure_text_dimensions_bilingual() {
    let (w1, h1) = measure_text_dimensions("Hello", 16.0);
    let (w2, h2) = measure_text_dimensions("Hello World and Universe", 16.0);
    assert!(w2 > w1, "Longer text must have wider measured width");
    assert!(
        (h2 - h1).abs() < 5.0,
        "Single line text height should remain similar"
    );

    let (wa, ha) = measure_text_dimensions("خوارزمية Dijkstra", 24.0);
    assert!(wa > 50.0);
    assert!(ha >= 24.0);

    let (wm, hm) = measure_text_dimensions("Line 1\nLine 2\nLine 3", 16.0);
    assert!(
        hm > h1 * 2.0,
        "Multiline text height must scale with line count"
    );
    assert!(wm > 0.0);
}
