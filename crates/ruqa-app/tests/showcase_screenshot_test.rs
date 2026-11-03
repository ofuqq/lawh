//! Verification test and showcase screenshot generator for Ruqa.
//!
//! Renders the actual live application with demo board primitives, bilingual typography,
//! and UI chrome into an offscreen pixmap and encodes it directly to
//! `docs/assets/ruqa-showcase.png` for documentation and README fidelity.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Point;
use ruqa_core::tool::Tool;
use tiny_skia::Pixmap;
use winit::dpi::PhysicalSize;

#[test]
fn test_generate_real_showcase_screenshot() {
    let width = 1280;
    let height = 800;

    let mut app = WhiteboardApp::new();
    app.physical_size = PhysicalSize::new(width, height);
    app.scale_factor = 1.0;
    app.show_welcome_hud = false;
    app.show_vault_sidebar = false;
    app.active_tool = Tool::Select;

    // Populate demo board primitives
    app.load_demo_board();

    // Add bilingual title text
    let title_id = app.scene.next_element_id();
    app.scene.push(Element::Text {
        id: title_id,
        position: Point::new(-240.0, -200.0),
        content: "Ruqa • رقعة — Hand-drawn whiteboard for studying".to_string(),
        font_size: 22.0,
        style: ElementStyle {
            stroke_color: "#ECEFF4".to_string(),
            ..Default::default()
        },
    });

    // Select the rectangle to showcase selection outline and transform handles
    app.selection.set_selection([1]);

    let mut pixmap = Pixmap::new(width, height).expect("pixmap allocation succeeds");
    app.render_to_pixmap(&mut pixmap.as_mut());

    let assets_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("docs").join("assets"))
        .expect("docs/assets path resolution succeeds");

    let showcase_path = assets_dir.join("ruqa-showcase.png");
    pixmap
        .save_png(&showcase_path)
        .expect("saving showcase PNG screenshot must succeed");

    assert!(showcase_path.exists(), "ruqa-showcase.png must be created");
    let file_len = std::fs::metadata(&showcase_path)
        .expect("metadata succeeds")
        .len();
    assert!(
        file_len > 10_000,
        "showcase PNG must be non-trivial raster data (got {file_len} bytes)"
    );
}
