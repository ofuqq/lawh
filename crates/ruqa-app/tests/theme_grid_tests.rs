//! Integration tests for default dark theme palette, color inversion, and canvas grid toggling.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use ruqa_core::theme::{
    ThemeMode, DARK_BACKGROUND_COLOR, DARK_PEN_INK, LIGHT_BACKGROUND_COLOR, LIGHT_PEN_INK,
};
use ruqa_render::grid_render::GridStyle;
use winit::event::ElementState;
use winit::keyboard::{Key, NamedKey};

#[test]
fn test_default_dark_theme_and_grid_state() {
    let app = WhiteboardApp::new();

    // Ruqa starts in dark theme by default
    assert_eq!(app.theme_mode, ThemeMode::Dark);
    assert_eq!(app.theme_mode.background_color(), DARK_BACKGROUND_COLOR);
    assert_eq!(DARK_BACKGROUND_COLOR, "#121212");

    // Default creation pen ink is crisp off-white (#ECEFF4)
    assert_eq!(app.current_style.stroke_color, DARK_PEN_INK);
    assert_eq!(DARK_PEN_INK, "#ECEFF4");

    // Grid orientation dots are visible by default
    assert!(app.show_grid);
    assert_eq!(app.grid_config.style, GridStyle::Dots);
}

#[test]
fn test_ctrl_quote_shortcut_toggles_grid() {
    let mut app = WhiteboardApp::new();
    assert!(app.show_grid);

    // Hold Ctrl
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);

    // Press single quote (')
    app.handle_key(&Key::Character("'".into()), ElementState::Pressed);
    app.handle_key(&Key::Character("'".into()), ElementState::Released);
    assert!(!app.show_grid, "Ctrl+' must toggle grid off");

    // Press single quote again
    app.handle_key(&Key::Character("'".into()), ElementState::Pressed);
    app.handle_key(&Key::Character("'".into()), ElementState::Released);
    assert!(app.show_grid, "Ctrl+' must toggle grid back on");
}

#[test]
fn test_toggle_theme_inverts_monochrome_strokes_and_preserves_accents() {
    let mut app = WhiteboardApp::new();

    // Add elements with monochrome and chromatic accent colors
    let offwhite_rect = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(10.0, 10.0, 40.0, 40.0),
        style: ElementStyle {
            stroke_color: DARK_PEN_INK.to_string(), // #ECEFF4
            fill_color: None,
            ..Default::default()
        },
    };
    let coral_circle = Element::Ellipse {
        id: 2,
        bounds: Rect::from_xywh(60.0, 10.0, 40.0, 40.0),
        style: ElementStyle {
            stroke_color: "#FF6B6B".to_string(),     // Coral accent
            fill_color: Some("#FFD166".to_string()), // Yellow accent
            ..Default::default()
        },
    };
    let dark_line = Element::Line {
        id: 3,
        start: ruqa_core::geometry::Point::new(0.0, 0.0),
        end: ruqa_core::geometry::Point::new(100.0, 0.0),
        style: ElementStyle {
            stroke_color: LIGHT_PEN_INK.to_string(), // Dark ink (#1E1E1E)
            fill_color: None,
            ..Default::default()
        },
    };

    app.scene.push(offwhite_rect);
    app.scene.push(coral_circle);
    app.scene.push(dark_line);

    // Switch to light mode
    app.toggle_theme();
    assert_eq!(app.theme_mode, ThemeMode::Light);
    assert_eq!(app.theme_mode.background_color(), LIGHT_BACKGROUND_COLOR);
    assert_eq!(LIGHT_BACKGROUND_COLOR, "#FFFFFF");

    // Default pen stroke now creates dark ink
    assert_eq!(app.current_style.stroke_color, LIGHT_PEN_INK);
    assert_eq!(LIGHT_PEN_INK, "#1E1E1E");

    // Off-white stroke inverted to dark ink
    assert_eq!(
        app.scene.get(1).unwrap().style().stroke_color,
        LIGHT_PEN_INK
    );

    // Accent colors were completely preserved without alteration
    assert_eq!(app.scene.get(2).unwrap().style().stroke_color, "#FF6B6B");
    assert_eq!(
        app.scene.get(2).unwrap().style().fill_color.as_deref(),
        Some("#FFD166")
    );

    // Dark stroke inverted to crisp off-white
    assert_eq!(app.scene.get(3).unwrap().style().stroke_color, DARK_PEN_INK);

    // Toggle back to dark mode restores original dark canvas palette
    app.toggle_theme();
    assert_eq!(app.theme_mode, ThemeMode::Dark);
    assert_eq!(app.scene.get(1).unwrap().style().stroke_color, DARK_PEN_INK);
    assert_eq!(app.scene.get(2).unwrap().style().stroke_color, "#FF6B6B");
    assert_eq!(
        app.scene.get(3).unwrap().style().stroke_color,
        LIGHT_PEN_INK
    );
}

#[test]
fn test_grid_config_mode_switching() {
    let mut app = WhiteboardApp::new();
    assert_eq!(app.grid_config.color, (45, 45, 45, 255));

    app.toggle_theme();
    // Light mode uses light gray dots on white
    assert_eq!(app.grid_config.color, (225, 225, 225, 255));

    app.toggle_theme();
    // Dark mode restores subtle dark gray dots on #121212
    assert_eq!(app.grid_config.color, (45, 45, 45, 255));
}
