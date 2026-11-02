use ruqa_core::document::RuqaDocument;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::{Point, Rect, Viewport};
use ruqa_core::scene::Scene;
use std::fs;
use std::path::{Path, PathBuf};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("ruqa_test_{prefix}_{epoch}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn test_temp_file_path_generation() {
    let p1 = Path::new("canvas.ruqa");
    assert_eq!(
        RuqaDocument::temp_file_path(p1),
        PathBuf::from(".canvas.ruqa.tmp")
    );

    let p2 = Path::new("/var/data/notes.ruqa");
    assert_eq!(
        RuqaDocument::temp_file_path(p2),
        PathBuf::from("/var/data/.notes.ruqa.tmp")
    );

    let p3 = Path::new("./sub/board.ruqa");
    assert_eq!(
        RuqaDocument::temp_file_path(p3),
        PathBuf::from("./sub/.board.ruqa.tmp")
    );
}

#[test]
fn test_atomic_save_and_load_roundtrip() {
    let temp_dir = unique_temp_dir("atomic_roundtrip");
    let file_path = temp_dir.join("test_board.ruqa");
    let temp_path = RuqaDocument::temp_file_path(&file_path);

    let mut scene = Scene::new();
    let elem = Element::Rectangle {
        id: 42,
        bounds: Rect::from_xywh(10.0, 20.0, 100.0, 50.0),
        style: ElementStyle::default(),
    };
    scene.push(elem.clone());
    let viewport = Viewport::new(Point::new(50.0, -100.0), 1.5);

    let doc = RuqaDocument::new(&scene, viewport);
    doc.save_to_file(&file_path).expect("save must succeed");

    // Target file must exist and temp file must be gone (renamed)
    assert!(file_path.exists(), "target file must exist");
    assert!(
        !temp_path.exists(),
        "temporary file must not remain after successful save"
    );

    let loaded = RuqaDocument::load_from_file(&file_path).expect("load must succeed");
    let (loaded_scene, loaded_viewport) = loaded.into_scene_and_viewport();

    assert_eq!(loaded_viewport, viewport);
    assert_eq!(loaded_scene.elements().len(), 1);
    assert_eq!(loaded_scene.elements()[0].id(), elem.id());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_atomic_save_overwrites_existing_file_cleanly() {
    let temp_dir = unique_temp_dir("atomic_overwrite");
    let file_path = temp_dir.join("overwrite_board.ruqa");

    // Pre-create file with existing content
    fs::write(&file_path, "pre-existing dummy data").expect("failed to pre-write");
    assert_eq!(
        fs::read_to_string(&file_path).unwrap(),
        "pre-existing dummy data"
    );

    let scene = Scene::new();
    let doc = RuqaDocument::new(&scene, Viewport::default());
    doc.save_to_file(&file_path)
        .expect("atomic save over existing file must succeed");

    let contents = fs::read_to_string(&file_path).unwrap();
    assert!(
        contents.contains("\"format\": \"ruqa\""),
        "overwritten file must be valid ruqa document"
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_atomic_save_fails_gracefully_on_invalid_directory() {
    let invalid_path = PathBuf::from("/nonexistent_ruqa_dir_xyz/test.ruqa");
    let doc = RuqaDocument::new(&Scene::new(), Viewport::default());
    let res = doc.save_to_file(&invalid_path);
    assert!(
        res.is_err(),
        "saving to nonexistent directory must return error"
    );
}
