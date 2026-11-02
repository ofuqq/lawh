//! Integration tests for selection outline, multi-selection bounds, and resize handles.

use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::{Point, Rect, Size, Viewport};
use ruqa_core::scene::Scene;
use ruqa_render::selection_render::{
    compute_handles, hit_test_handle, render_selection, selection_screen_rect, HandleKind,
    SelectionStyle,
};
use tiny_skia::Pixmap;

#[test]
fn test_handle_kind_classification() {
    for &kind in &HandleKind::ALL {
        if matches!(
            kind,
            HandleKind::TopLeft
                | HandleKind::TopRight
                | HandleKind::BottomRight
                | HandleKind::BottomLeft
        ) {
            assert!(kind.is_corner());
            assert!(!kind.is_edge());
        } else {
            assert!(kind.is_edge());
            assert!(!kind.is_corner());
        }
    }
}

#[test]
fn test_multi_selection_bounds_encloses_all_elements() {
    let mut scene = Scene::new();
    let e1 = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(10.0, 10.0, 50.0, 40.0),
        style: ElementStyle::default(),
    };
    let e2 = Element::Rectangle {
        id: 2,
        bounds: Rect::from_xywh(200.0, 150.0, 60.0, 80.0),
        style: ElementStyle::default(),
    };
    scene.push(e1);
    scene.push(e2);

    let single_bounds = scene.selection_bounds(&[1]).expect("element 1 exists");
    assert!(single_bounds.contains(Point::new(10.0, 10.0)));
    assert!(single_bounds.contains(Point::new(60.0, 50.0)));
    assert!(!single_bounds.contains(Point::new(200.0, 150.0)));

    let multi_bounds = scene
        .selection_bounds(&[1, 2])
        .expect("both elements exist");
    assert!(multi_bounds.contains(Point::new(10.0, 10.0)));
    assert!(multi_bounds.contains(Point::new(260.0, 230.0)));
}

#[test]
fn test_constant_8x8_screen_handle_size_at_all_zoom_levels() {
    let world_bounds = Rect::from_xywh(50.0, 50.0, 100.0, 80.0);
    let window_size = Size::new(1024.0, 768.0);
    let style = SelectionStyle::default();

    for zoom in [0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0] {
        let viewport = Viewport::new(Point::new(50.0, 50.0), zoom);
        let screen_rect =
            selection_screen_rect(world_bounds, &viewport, window_size, style.padding);
        let handles = compute_handles(screen_rect, style.handle_size);

        assert_eq!(handles.len(), 8);
        for handle in &handles {
            assert_eq!(
                handle.bounds.width(),
                8.0,
                "handle width must stay 8px at zoom {zoom}"
            );
            assert_eq!(
                handle.bounds.height(),
                8.0,
                "handle height must stay 8px at zoom {zoom}"
            );
        }
    }
}

#[test]
fn test_pan_shifts_handles_in_screen_space() {
    let world_bounds = Rect::from_xywh(0.0, 0.0, 100.0, 100.0);
    let window_size = Size::new(800.0, 600.0);
    let style = SelectionStyle::default();

    let v1 = Viewport::new(Point::ZERO, 1.0);
    let screen_r1 = selection_screen_rect(world_bounds, &v1, window_size, style.padding);
    let h1 = compute_handles(screen_r1, style.handle_size);

    let v2 = Viewport::new(Point::new(50.0, 30.0), 1.0);
    let screen_r2 = selection_screen_rect(world_bounds, &v2, window_size, style.padding);
    let h2 = compute_handles(screen_r2, style.handle_size);

    // Camera panned right by 50 and down by 30 => elements move left by 50 and up by 30
    assert_eq!(h2[0].center.x, h1[0].center.x - 50.0);
    assert_eq!(h2[0].center.y, h1[0].center.y - 30.0);
}

#[test]
fn test_all_8_handles_hit_testable() {
    let world_bounds = Rect::from_xywh(-50.0, -50.0, 100.0, 100.0);
    let window_size = Size::new(800.0, 600.0);
    let viewport = Viewport::new(Point::ZERO, 1.0);
    let style = SelectionStyle::default();

    let screen_rect = selection_screen_rect(world_bounds, &viewport, window_size, style.padding);
    let handles = compute_handles(screen_rect, style.handle_size);

    for handle in &handles {
        let hit = hit_test_handle(handle.center, world_bounds, &viewport, window_size, &style);
        assert_eq!(hit, Some(handle.kind));
    }
}

#[test]
fn test_render_selection_multi_element_rasterization() {
    let mut pixmap = Pixmap::new(400, 400).expect("pixmap allocation succeeds");
    let world_bounds = Rect::from_xywh(-40.0, -40.0, 80.0, 80.0);
    let window_size = Size::new(400.0, 400.0);
    let viewport = Viewport::new(Point::ZERO, 1.0);
    let style = SelectionStyle::default();

    render_selection(
        world_bounds,
        &viewport,
        window_size,
        &mut pixmap.as_mut(),
        &style,
    );

    let non_zero_pixels = pixmap.data().iter().filter(|&&b| b != 0).count();
    assert!(non_zero_pixels > 0);
}
