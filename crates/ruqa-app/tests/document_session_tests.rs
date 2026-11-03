//! Integration tests for document session dirty state tracking, window title updates, and Ctrl+S saving.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use std::time::{Duration, Instant};
use winit::event::ElementState;
use winit::keyboard::{Key, NamedKey};

fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle {
            stroke_width: 2.0,
            ..Default::default()
        },
    }
}

fn press_key(app: &mut WhiteboardApp, key: Key) {
    app.handle_key(&key, ElementState::Pressed);
    app.handle_key(&key, ElementState::Released);
}

#[test]
fn test_initial_document_clean_and_untitled() {
    let app = WhiteboardApp::new();
    assert!(!app.is_dirty);
    assert_eq!(app.file_path, None);
    assert_eq!(app.window_title(), "Ruqa - Untitled");
}

#[test]
fn test_mutation_marks_document_dirty_and_updates_title() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));
    app.selection.set_selection([1]);

    // Duplicate selected flips dirty flag
    app.duplicate_selected();
    assert!(app.is_dirty);
    assert_eq!(app.window_title(), "Ruqa - Untitled*");
}

#[test]
fn test_save_as_clears_dirty_and_updates_window_title() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 100.0, 80.0));
    app.mark_dirty();
    assert_eq!(app.window_title(), "Ruqa - Untitled*");

    let tmp_file = std::env::temp_dir().join("algebra_notes.ruqa");
    app.save_as(&tmp_file).expect("save_as succeeds");

    assert!(!app.is_dirty);
    assert_eq!(app.file_path, Some(tmp_file.clone()));
    assert_eq!(app.window_title(), "Ruqa - algebra_notes.ruqa");

    let _ = std::fs::remove_file(tmp_file);
}

#[test]
fn test_ctrl_s_shortcut_saves_existing_document() {
    let mut app = WhiteboardApp::new();
    let tmp_file = std::env::temp_dir().join("chemistry_study.ruqa");
    app.save_as(&tmp_file).expect("save_as succeeds");
    assert!(!app.is_dirty);

    // Modify document via deletion
    app.scene.push(sample_rect(10, 20.0, 20.0, 30.0, 30.0));
    app.selection.set_selection([10]);
    app.delete_selected();

    assert!(app.is_dirty);
    assert_eq!(app.window_title(), "Ruqa - chemistry_study.ruqa*");

    // Trigger Ctrl+S via keyboard
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);
    press_key(&mut app, Key::Character("s".into()));
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Released);

    assert!(!app.is_dirty);
    assert_eq!(app.window_title(), "Ruqa - chemistry_study.ruqa");

    // Verify on disk content matches
    let loaded = ruqa_core::document::RuqaDocument::load_from_file(&tmp_file)
        .expect("file exists and is valid");
    assert_eq!(loaded.elements.len(), 0);

    let _ = std::fs::remove_file(tmp_file);
}

#[test]
fn test_open_file_and_new_document_session_flow() {
    let tmp_file = std::env::temp_dir().join("physics_diagram.ruqa");

    // Save initial document with 2 elements
    let mut app1 = WhiteboardApp::new();
    app1.scene.push(sample_rect(1, 0.0, 0.0, 50.0, 50.0));
    app1.scene.push(sample_rect(2, 60.0, 0.0, 50.0, 50.0));
    app1.save_as(&tmp_file).expect("save succeeds");

    // Open file in another app instance
    let mut app2 = WhiteboardApp::new();
    app2.open_file(&tmp_file).expect("open_file succeeds");
    assert_eq!(app2.scene.len(), 2);
    assert!(!app2.is_dirty);
    assert_eq!(app2.window_title(), "Ruqa - physics_diagram.ruqa");

    // Reset to new document
    app2.new_document();
    assert_eq!(app2.scene.len(), 0);
    assert_eq!(app2.file_path, None);
    assert!(!app2.is_dirty);
    assert_eq!(app2.window_title(), "Ruqa - Untitled");

    let _ = std::fs::remove_file(tmp_file);
}

#[test]
fn test_autosave_triggers_after_inactivity_interval() {
    let tmp_file = std::env::temp_dir().join("autosave_study.ruqa");
    let mut app = WhiteboardApp::new();
    app.save_as(&tmp_file).expect("initial save succeeds");

    // Add element and mark dirty
    app.scene.push(sample_rect(42, 10.0, 10.0, 60.0, 60.0));
    let t0 = Instant::now();
    app.mark_dirty();
    app.last_dirty_time = Some(t0);

    assert!(app.is_dirty);
    assert_eq!(app.window_title(), "Ruqa - autosave_study.ruqa*");
    assert!(app.next_autosave_deadline().is_some());

    // 10 seconds elapsed: should not autosave yet
    let res = app
        .maybe_autosave(t0 + Duration::from_secs(10))
        .expect("check succeeds");
    assert!(!res);
    assert!(app.is_dirty);

    // 31 seconds elapsed: autosave triggers
    let res = app
        .maybe_autosave(t0 + Duration::from_secs(31))
        .expect("autosave succeeds");
    assert!(res);
    assert!(!app.is_dirty);
    assert_eq!(app.window_title(), "Ruqa - autosave_study.ruqa");

    // Verify disk content was updated atomically
    let loaded =
        ruqa_core::document::RuqaDocument::load_from_file(&tmp_file).expect("loaded successfully");
    assert_eq!(loaded.elements.len(), 1);
    assert_eq!(loaded.elements[0].id(), 42);

    let _ = std::fs::remove_file(tmp_file);
}

#[test]
fn test_autosave_inactivity_resets_on_new_edits() {
    let tmp_file = std::env::temp_dir().join("autosave_inactivity.ruqa");
    let mut app = WhiteboardApp::new();
    app.save_as(&tmp_file).expect("initial save succeeds");

    let t0 = Instant::now();
    app.mark_dirty();
    app.last_dirty_time = Some(t0);

    // After 20 seconds, user edits again (mark_dirty resets timer)
    let t_edit = t0 + Duration::from_secs(20);
    app.mark_dirty();
    app.last_dirty_time = Some(t_edit);

    // At t0 + 35s (only 15s since last edit): should NOT autosave yet
    let res = app
        .maybe_autosave(t0 + Duration::from_secs(35))
        .expect("check succeeds");
    assert!(!res);
    assert!(app.is_dirty);

    // At t0 + 52s (32s since last edit): autosave triggers
    let res = app
        .maybe_autosave(t0 + Duration::from_secs(52))
        .expect("autosave succeeds");
    assert!(res);
    assert!(!app.is_dirty);

    let _ = std::fs::remove_file(tmp_file);
}

#[test]
fn test_autosave_does_not_fire_when_untitled_or_clean() {
    let mut app = WhiteboardApp::new();
    let t0 = Instant::now();

    // Untitled and dirty: should not autosave
    app.mark_dirty();
    app.last_dirty_time = Some(t0);
    let res = app
        .maybe_autosave(t0 + Duration::from_secs(60))
        .expect("check succeeds");
    assert!(!res);
    assert!(app.is_dirty);

    // Clean with file path: should not autosave
    let tmp_file = std::env::temp_dir().join("clean_file.ruqa");
    app.file_path = Some(tmp_file);
    app.is_dirty = false;
    let res = app
        .maybe_autosave(t0 + Duration::from_secs(60))
        .expect("check succeeds");
    assert!(!res);
}

#[test]
fn test_untitled_saves_do_not_overwrite_existing_files() {
    let tmp_dir = std::env::temp_dir().join(format!("ruqa_test_untitled_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).expect("create test tmp_dir");

    let mut app = WhiteboardApp::new();
    app.vault.root = tmp_dir.clone();

    // 1. Board 1: Draw a rectangle, save as untitled
    app.scene.push(sample_rect(1, 10.0, 10.0, 100.0, 50.0));
    app.mark_dirty();
    let path1 = app.save().expect("save board 1 succeeds");
    assert_eq!(path1, tmp_dir.join("untitled.ruqa"));
    assert!(path1.is_file());
    assert!(!app.is_dirty);
    assert_eq!(app.window_title(), "Ruqa - untitled.ruqa");

    // 2. Board 2: Create new board, draw an ellipse, save as untitled
    app.new_document();
    assert_eq!(app.file_path, None);
    assert!(!app.is_dirty);
    app.scene.push(Element::Ellipse {
        id: 2,
        bounds: Rect::from_xywh(20.0, 20.0, 80.0, 80.0),
        style: ElementStyle::default(),
    });
    app.mark_dirty();
    let path2 = app.save().expect("save board 2 succeeds");
    assert_eq!(path2, tmp_dir.join("untitled-2.ruqa"));
    assert!(path2.is_file());
    assert_ne!(path1, path2);
    assert_eq!(app.window_title(), "Ruqa - untitled-2.ruqa");

    // 3. Board 3: Create a third board, save as untitled
    app.new_document();
    app.scene.push(sample_rect(3, 5.0, 5.0, 40.0, 40.0));
    app.mark_dirty();
    let path3 = app.save().expect("save board 3 succeeds");
    assert_eq!(path3, tmp_dir.join("untitled-3.ruqa"));
    assert!(path3.is_file());

    // 4. Verify contents of all boards are completely distinct and intact (zero data loss)
    let doc1 = ruqa_core::document::RuqaDocument::load_from_file(&path1).expect("load doc1");
    let doc2 = ruqa_core::document::RuqaDocument::load_from_file(&path2).expect("load doc2");
    let doc3 = ruqa_core::document::RuqaDocument::load_from_file(&path3).expect("load doc3");

    assert_eq!(doc1.elements.len(), 1);
    assert_eq!(doc1.elements[0].id(), 1);
    assert_eq!(doc2.elements.len(), 1);
    assert_eq!(doc2.elements[0].id(), 2);
    assert_eq!(doc3.elements.len(), 1);
    assert_eq!(doc3.elements[0].id(), 3);

    // Cleanup
    let _ = std::fs::remove_file(path1);
    let _ = std::fs::remove_file(path2);
    let _ = std::fs::remove_file(path3);
    let _ = std::fs::remove_dir(tmp_dir);
}

#[test]
fn test_quit_dialog_save_on_untitled_document_uses_safe_naming() {
    let tmp_dir =
        std::env::temp_dir().join(format!("ruqa_test_quit_untitled_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).expect("create test tmp_dir");

    // Pre-create untitled.ruqa to simulate existing board
    let existing = tmp_dir.join("untitled.ruqa");
    std::fs::write(&existing, b"preexisting content").expect("create preexisting untitled");

    let mut app = WhiteboardApp::new();
    app.vault.root = tmp_dir.clone();
    app.scene.push(sample_rect(99, 10.0, 10.0, 30.0, 30.0));
    app.mark_dirty();

    // Trigger close and confirm save
    app.request_close();
    assert!(app.show_quit_dialog);
    app.confirm_quit_save();

    assert!(app.should_exit);
    assert!(!app.show_quit_dialog);

    // Pre-existing untitled.ruqa was NOT overwritten
    assert_eq!(
        std::fs::read(&existing).expect("read existing"),
        b"preexisting content"
    );

    // Saved to untitled-2.ruqa
    let new_path = tmp_dir.join("untitled-2.ruqa");
    assert!(new_path.is_file());
    let doc = ruqa_core::document::RuqaDocument::load_from_file(&new_path).expect("load doc");
    assert_eq!(doc.elements.len(), 1);
    assert_eq!(doc.elements[0].id(), 99);

    let _ = std::fs::remove_file(existing);
    let _ = std::fs::remove_file(new_path);
    let _ = std::fs::remove_dir(tmp_dir);
}

#[test]
fn test_ctrl_s_shortcut_on_untitled_documents_increments_safely() {
    let tmp_dir = std::env::temp_dir().join(format!("ruqa_test_ctrl_s_{}", std::process::id()));
    std::fs::create_dir_all(&tmp_dir).expect("create test tmp_dir");

    let mut app = WhiteboardApp::new();
    app.vault.root = tmp_dir.clone();

    // 1. Board 1: Draw rectangle, press Ctrl+S
    app.scene.push(sample_rect(10, 10.0, 10.0, 50.0, 50.0));
    app.mark_dirty();
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);
    press_key(&mut app, Key::Character("s".into()));
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Released);

    let path1 = tmp_dir.join("untitled.ruqa");
    assert_eq!(app.file_path, Some(path1.clone()));
    assert!(path1.is_file());
    assert!(!app.is_dirty);

    // 2. Board 2: Create new document, draw ellipse, press Ctrl+S
    app.new_document();
    assert_eq!(app.file_path, None);
    app.scene.push(sample_rect(20, 20.0, 20.0, 50.0, 50.0));
    app.mark_dirty();
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);
    press_key(&mut app, Key::Character("s".into()));
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Released);

    let path2 = tmp_dir.join("untitled-2.ruqa");
    assert_eq!(app.file_path, Some(path2.clone()));
    assert!(path2.is_file());
    assert!(!app.is_dirty);

    // Both files exist and differ
    assert!(path1.is_file());
    assert!(path2.is_file());
    assert_ne!(path1, path2);

    let _ = std::fs::remove_file(path1);
    let _ = std::fs::remove_file(path2);
    let _ = std::fs::remove_dir(tmp_dir);
fn test_open_bundled_example_boards_fit_default_viewport() {
    use ruqa_core::geometry::Size;
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let examples_dir = manifest
        .parent()
        .expect("crates dir")
        .parent()
        .expect("workspace root")
        .join("examples");

    for filename in &[
        "dijkstra-shortest-path.ruqa",
        "distributed-systems-cache.ruqa",
        "binary-search-tree.ruqa",
    ] {
        let path = examples_dir.join(filename);
        let mut app = WhiteboardApp::new();
        app.open_file(&path).expect("open bundled example board");
        assert_eq!(app.file_path, Some(path));

        let visible = app.viewport.visible_world_rect(Size::new(1280.0, 800.0));
        for elem in app.scene.elements() {
            let bb = elem.bounding_box();
            assert!(
                visible.contains(bb.min),
                "{filename}: element {:?} min {:?} is outside visible viewport {:?}",
                elem.id(),
                bb.min,
                visible
            );
            assert!(
                visible.contains(bb.max),
                "{filename}: element {:?} max {:?} is outside visible viewport {:?}",
                elem.id(),
                bb.max,
                visible
            );
        }
    }
}
