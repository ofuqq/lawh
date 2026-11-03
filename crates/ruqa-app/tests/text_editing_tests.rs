use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::Element;
use ruqa_core::geometry::Point;
use ruqa_core::tool::Tool;
use winit::event::ElementState;
use winit::keyboard::{Key, NamedKey};

#[test]
fn test_clicking_with_text_tool_spawns_inline_caret() {
    let mut app = WhiteboardApp::new();
    app.set_active_tool(Tool::Text);

    let click_pt = Point::new(120.0, 85.0);
    app.handle_text_tool_pointer_down(click_pt);

    assert!(
        app.is_editing_text,
        "Clicking with Text tool must activate editing"
    );
    assert_eq!(app.text_edit_position, click_pt);
    assert!(app.text_edit_buffer.is_empty());
}

#[test]
fn test_typing_characters_and_backspace_in_text_buffer() {
    let mut app = WhiteboardApp::new();
    app.set_active_tool(Tool::Text);
    app.handle_text_tool_pointer_down(Point::new(50.0, 50.0));

    // Type "Hello"
    for c in ["H", "e", "l", "l", "o"] {
        app.handle_key(&Key::Character(c.into()), ElementState::Pressed);
    }
    assert_eq!(app.text_edit_buffer, "Hello");

    // Backspace
    app.handle_key(&Key::Named(NamedKey::Backspace), ElementState::Pressed);
    assert_eq!(app.text_edit_buffer, "Hell");
}

#[test]
fn test_enter_key_commits_text_element_with_undo_redo() {
    let mut app = WhiteboardApp::new();
    app.set_active_tool(Tool::Text);
    app.handle_text_tool_pointer_down(Point::new(100.0, 150.0));

    for c in ["R", "u", "q", "a"] {
        app.handle_key(&Key::Character(c.into()), ElementState::Pressed);
    }

    // Press Enter to commit
    app.handle_key(&Key::Named(NamedKey::Enter), ElementState::Pressed);

    assert!(!app.is_editing_text, "Text edit must be closed on Enter");
    assert_eq!(app.scene.len(), 1, "Committed text should be in the scene");

    let elem = app.scene.elements().first().expect("element present");
    match elem {
        Element::Text {
            position, content, ..
        } => {
            assert_eq!(*position, Point::new(100.0, 150.0));
            assert_eq!(content, "Ruqa");
        }
        _ => panic!("Expected Element::Text variant"),
    }

    let elem_id = elem.id();
    assert!(app.selection.is_selected(elem_id));
    assert!(app.is_dirty);

    // Test Undo
    assert!(app.history.undo(&mut app.scene));
    assert!(app.scene.is_empty(), "Undo must remove committed text");

    // Test Redo
    assert!(app.history.redo(&mut app.scene));
    assert_eq!(app.scene.len(), 1, "Redo must restore committed text");
}

#[test]
fn test_typing_bilingual_arabic_note() {
    let mut app = WhiteboardApp::new();
    app.set_active_tool(Tool::Text);
    app.handle_text_tool_pointer_down(Point::new(200.0, 200.0));

    let sample_arabic = "خوارزمية Dijkstra";
    for ch in sample_arabic.chars() {
        app.handle_key(
            &Key::Character(ch.to_string().into()),
            ElementState::Pressed,
        );
    }
    assert_eq!(app.text_edit_buffer, sample_arabic);

    // Commit via Enter
    app.handle_key(&Key::Named(NamedKey::Enter), ElementState::Pressed);
    assert_eq!(app.scene.len(), 1);

    if let Element::Text { content, .. } = &app.scene.elements()[0] {
        assert_eq!(content, sample_arabic);
    } else {
        panic!("Expected Element::Text");
    }
}

#[test]
fn test_switching_tools_commits_ongoing_text_edit() {
    let mut app = WhiteboardApp::new();
    app.set_active_tool(Tool::Text);
    app.handle_text_tool_pointer_down(Point::new(30.0, 40.0));

    for c in ["N", "o", "t", "e"] {
        app.handle_key(&Key::Character(c.into()), ElementState::Pressed);
    }
    assert!(app.is_editing_text);

    // Switch tool to Select
    app.set_active_tool(Tool::Select);

    assert!(!app.is_editing_text, "Switching tool must commit text");
    assert_eq!(app.scene.len(), 1);
    if let Element::Text { content, .. } = &app.scene.elements()[0] {
        assert_eq!(content, "Note");
    }
}

#[test]
fn test_shift_enter_inserts_newline_for_multiline_note() {
    let mut app = WhiteboardApp::new();
    app.set_active_tool(Tool::Text);
    app.handle_text_tool_pointer_down(Point::new(10.0, 10.0));

    app.handle_key(&Key::Character("Line1".into()), ElementState::Pressed);

    // Hold Shift and press Enter
    app.handle_key(&Key::Named(NamedKey::Shift), ElementState::Pressed);
    app.handle_key(&Key::Named(NamedKey::Enter), ElementState::Pressed);
    app.handle_key(&Key::Named(NamedKey::Shift), ElementState::Released);

    app.handle_key(&Key::Character("Line2".into()), ElementState::Pressed);
    assert_eq!(app.text_edit_buffer, "Line1\nLine2");

    // Commit
    app.handle_key(&Key::Named(NamedKey::Enter), ElementState::Pressed);
    assert_eq!(app.scene.len(), 1);
    if let Element::Text { content, .. } = &app.scene.elements()[0] {
        assert_eq!(content, "Line1\nLine2");
    }
}
