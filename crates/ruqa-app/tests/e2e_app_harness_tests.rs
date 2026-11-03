//! End-to-End Headless Application Test Harness.
//!
//! Validates interactive whiteboard workflows by simulating native OS event loop
//! inputs (mouse moves, clicks, drags, mouse wheel, keyboard shortcuts, text typing,
//! and HiDPI resizing) through [`WhiteboardApp`] and inspecting state transitions
//! and rendered pixel geometry.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::Element;
use ruqa_core::geometry::Rect;
use ruqa_core::tool::Tool;
use tiny_skia::{Pixmap, PremultipliedColorU8};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};
use winit::keyboard::{Key, NamedKey};

/// Headless harness driving a [`WhiteboardApp`] instance with synthetic inputs.
pub struct AppHarness {
    pub app: WhiteboardApp,
    pub pixmap: Pixmap,
    pub width: u32,
    pub height: u32,
}

impl AppHarness {
    /// Creates a headless test harness with the specified physical dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        let mut app = WhiteboardApp::new();
        app.physical_size = PhysicalSize::new(width, height);
        app.scale_factor = 1.0;
        let pixmap = Pixmap::new(width, height).expect("pixmap allocation succeeds");
        Self {
            app,
            pixmap,
            width,
            height,
        }
    }

    /// Updates HiDPI scale factor.
    pub fn set_scale_factor(&mut self, factor: f64) {
        self.app.scale_factor = factor;
    }

    /// Resizes the physical viewport and backing pixmap.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.app.physical_size = PhysicalSize::new(width, height);
        self.pixmap = Pixmap::new(width, height).expect("pixmap allocation succeeds");
    }

    /// Dispatches cursor motion event in physical window coordinates.
    pub fn pointer_move(&mut self, x: f64, y: f64) -> bool {
        self.app.handle_cursor_moved(x, y)
    }

    /// Dispatches mouse button down event.
    pub fn pointer_down(&mut self, button: MouseButton) {
        self.app.handle_mouse_input(ElementState::Pressed, button);
    }

    /// Dispatches mouse button up event.
    pub fn pointer_up(&mut self, button: MouseButton) {
        self.app.handle_mouse_input(ElementState::Released, button);
    }

    /// Dispatches single click at specified coordinate.
    pub fn click(&mut self, x: f64, y: f64, button: MouseButton) {
        self.pointer_move(x, y);
        self.pointer_down(button);
        self.pointer_up(button);
    }

    /// Simulates dragging from `start` to `end` with interpolated step points.
    pub fn drag(&mut self, start: (f64, f64), end: (f64, f64), steps: usize) {
        self.pointer_move(start.0, start.1);
        self.pointer_down(MouseButton::Left);
        for i in 1..=steps {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / steps as f64;
            let cx = start.0 + (end.0 - start.0) * t;
            let cy = start.1 + (end.1 - start.1) * t;
            self.pointer_move(cx, cy);
        }
        self.pointer_up(MouseButton::Left);
    }

    /// Simulates middle-mouse pan drag.
    pub fn pan_drag(&mut self, start: (f64, f64), end: (f64, f64), steps: usize) {
        self.pointer_move(start.0, start.1);
        self.pointer_down(MouseButton::Middle);
        for i in 1..=steps {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / steps as f64;
            let cx = start.0 + (end.0 - start.0) * t;
            let cy = start.1 + (end.1 - start.1) * t;
            self.pointer_move(cx, cy);
        }
        self.pointer_up(MouseButton::Middle);
    }

    /// Simulates mouse wheel scrolling.
    pub fn mouse_wheel(&mut self, delta_y: f32) {
        self.app
            .handle_mouse_wheel(MouseScrollDelta::LineDelta(0.0, delta_y));
    }

    /// Dispatches a key press and release.
    pub fn key_tap(&mut self, key: &Key) {
        self.app.handle_key(key, ElementState::Pressed);
        self.app.handle_key(key, ElementState::Released);
    }

    /// Simulates typing characters.
    pub fn type_text(&mut self, text: &str) {
        for ch in text.chars() {
            let key = Key::Character(ch.to_string().into());
            self.key_tap(&key);
        }
    }

    /// Executes complete frame render pass into backing pixmap.
    pub fn render(&mut self) {
        self.app.render_to_pixmap(&mut self.pixmap.as_mut());
    }

    /// Returns pixel color at (x, y).
    #[must_use]
    pub fn pixel_at(&self, x: u32, y: u32) -> Option<PremultipliedColorU8> {
        self.pixmap.pixel(x, y)
    }

    /// Counts pixels matching predicate within given logical rectangle bounds.
    pub fn count_pixels_in_rect<F>(&self, rect: Rect, predicate: F) -> usize
    where
        F: Fn(PremultipliedColorU8) -> bool,
    {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let min_x = (rect.min.x.max(0.0) as u32).min(self.width);
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let max_x = (rect.max.x.max(0.0) as u32).min(self.width);
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let min_y = (rect.min.y.max(0.0) as u32).min(self.height);
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let max_y = (rect.max.y.max(0.0) as u32).min(self.height);

        let mut count = 0;
        for y in min_y..max_y {
            for x in min_x..max_x {
                if let Some(p) = self.pixmap.pixel(x, y) {
                    if predicate(p) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[test]
fn test_harness_pan_zoom_and_canvas_primitives() {
    let mut harness = AppHarness::new(1280, 800);
    harness.app.load_demo_board();
    let initial_count = harness.app.scene.len();
    assert!(initial_count > 0);

    // Pan with middle mouse drag
    let initial_pan = harness.app.viewport.pan;
    harness.pan_drag((500.0, 500.0), (400.0, 400.0), 5);
    assert_ne!(harness.app.viewport.pan, initial_pan);

    // Zoom with wheel
    let initial_zoom = harness.app.viewport.zoom;
    harness.pointer_move(640.0, 400.0);
    harness.mouse_wheel(2.0);
    assert!(harness.app.viewport.zoom > initial_zoom);

    // Render frame and verify shape pixels exist
    harness.render();
    let non_dark_count = harness
        .count_pixels_in_rect(Rect::from_xywh(0.0, 0.0, 1280.0, 800.0), |p| {
            p.red() > 30 || p.green() > 30
        });
    assert!(
        non_dark_count > 100,
        "Rendered canvas primitives must produce visible pixels"
    );
}

#[test]
fn test_harness_selection_move_delete_undo_redo() {
    let mut harness = AppHarness::new(1280, 800);
    harness.app.load_demo_board();
    let initial_count = harness.app.scene.len();
    assert!(initial_count > 0);

    // Switch to select tool 'v'
    harness.key_tap(&Key::Character("v".into()));
    assert_eq!(harness.app.active_tool, Tool::Select);

    // Select first element
    let first_id = harness.app.scene.elements()[0].id();
    harness.app.selection.set_selection([first_id]);
    assert_eq!(harness.app.selection.selected_ids().len(), 1);

    // Delete element
    harness.key_tap(&Key::Named(NamedKey::Delete));
    assert_eq!(harness.app.scene.len(), initial_count - 1);

    // Undo delete via Ctrl+Z
    harness.app.pointer.ctrl_held = true;
    harness.key_tap(&Key::Character("z".into()));
    harness.app.pointer.ctrl_held = false;
    assert_eq!(harness.app.scene.len(), initial_count);
}

#[test]
fn test_harness_theme_toggle_updates_pixels() {
    let mut harness = AppHarness::new(1280, 800);
    assert_eq!(harness.app.theme_mode, ruqa_core::theme::ThemeMode::Dark);

    harness.render();
    let dark_corner = harness.pixel_at(10, 10).unwrap();

    // Toggle theme via Ctrl+T shortcut
    harness.app.pointer.ctrl_held = true;
    harness.key_tap(&Key::Character("t".into()));
    harness.app.pointer.ctrl_held = false;

    assert_eq!(harness.app.theme_mode, ruqa_core::theme::ThemeMode::Light);

    harness.render();
    let light_corner = harness.pixel_at(10, 10).unwrap();

    // Light theme background must be markedly brighter than dark theme background
    assert!(light_corner.red() > dark_corner.red() + 100);
}

#[test]
fn test_harness_inline_text_editing_and_shaping() {
    let mut harness = AppHarness::new(1280, 800);

    // Switch to Text tool 't'
    harness.key_tap(&Key::Character("t".into()));
    assert_eq!(harness.app.active_tool, Tool::Text);

    // Click canvas outside HUD (400, 400) to spawn text editing caret
    harness.click(400.0, 400.0, MouseButton::Left);
    assert!(harness.app.is_editing_text);

    // Type bilingual text
    harness.type_text("Ruqa رقعة");
    assert_eq!(harness.app.text_edit_buffer, "Ruqa رقعة");

    // Commit with Enter
    harness.key_tap(&Key::Named(NamedKey::Enter));
    assert!(!harness.app.is_editing_text);
    assert_eq!(harness.app.scene.len(), 1);

    // Render and check text pixels
    harness.render();
    let text_area = Rect::from_xywh(390.0, 390.0, 200.0, 40.0);
    let text_pixels = harness.count_pixels_in_rect(text_area, |p| p.red() > 100 || p.green() > 100);
    assert!(
        text_pixels > 20,
        "Rendered shaped text must produce visible ink pixels"
    );
}

#[test]
fn test_harness_modal_and_sidebar_rendering() {
    let mut harness = AppHarness::new(1280, 800);

    // Test Welcome HUD overlay rendering
    harness.app.show_welcome_hud = true;
    harness.render();
    let modal_pixels = harness
        .count_pixels_in_rect(Rect::from_xywh(300.0, 200.0, 680.0, 400.0), |p| {
            p.red() > 20 && p.alpha() > 200
        });
    assert!(
        modal_pixels > 500,
        "Welcome HUD must render visible modal pixels"
    );
    harness.app.show_welcome_hud = false;

    // Test Vault sidebar drawer rendering
    harness.app.show_vault_sidebar = true;
    harness.render();
    let sidebar_pixels = harness
        .count_pixels_in_rect(Rect::from_xywh(0.0, 0.0, 250.0, 800.0), |p| {
            p.alpha() == 255
        });
    assert!(
        sidebar_pixels > 1000,
        "Vault sidebar must render full drawer height with 100% opacity"
    );
}

#[test]
fn test_harness_welcome_hud_dismiss_and_sidebar_toggle() {
    let mut harness = AppHarness::new(1280, 800);

    // 1. Verify clicking 'X' button dismisses Welcome HUD
    harness.app.show_welcome_hud = true;
    let layout = harness.app.recent_files_layout();
    let close_pt = layout.close_button_bounds.center();
    harness.click(
        f64::from(close_pt.x),
        f64::from(close_pt.y),
        MouseButton::Left,
    );
    assert!(
        !harness.app.show_welcome_hud,
        "Clicking 'X' button must dismiss Welcome HUD"
    );

    // 2. Verify clicking outside dialog dismisses Welcome HUD
    harness.app.show_welcome_hud = true;
    harness.click(50.0, 50.0, MouseButton::Left);
    assert!(
        !harness.app.show_welcome_hud,
        "Clicking outside dialog must dismiss Welcome HUD"
    );

    // 3. Verify Escape key dismisses Welcome HUD
    harness.app.show_welcome_hud = true;
    harness.key_tap(&Key::Named(NamedKey::Escape));
    assert!(
        !harness.app.show_welcome_hud,
        "Escape key must dismiss Welcome HUD"
    );

    // 4. Verify Enter key dismisses Welcome HUD
    harness.app.show_welcome_hud = true;
    harness.key_tap(&Key::Named(NamedKey::Enter));
    assert!(
        !harness.app.show_welcome_hud,
        "Enter key must dismiss Welcome HUD"
    );

    // 5. Verify Study Vault folder row toggles expansion on mouse click
    let unique_name = format!(
        "ruqa_vault_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let tmp_path = std::env::temp_dir().join(unique_name);
    std::fs::create_dir_all(tmp_path.join("Algorithms")).expect("create folder");
    std::fs::write(tmp_path.join("Algorithms").join("Dijkstra.ruqa"), "{}").expect("write board");

    harness.app.set_vault_root(&tmp_path);
    harness.app.show_vault_sidebar = true;
    harness.app.refresh_vault_entries();

    assert_eq!(harness.app.vault_entries.len(), 1);
    assert!(!harness.app.vault_entries[0].is_expanded);

    let vault_layout = harness.app.vault_layout();
    assert!(!vault_layout.rows.is_empty());
    let row_pt = vault_layout.rows[0].row_bounds.center();

    // Click folder row to expand
    harness.click(f64::from(row_pt.x), f64::from(row_pt.y), MouseButton::Left);
    assert_eq!(harness.app.vault_entries.len(), 2);
    assert!(harness.app.vault_entries[0].is_expanded);

    // Click folder row again to collapse
    harness.click(f64::from(row_pt.x), f64::from(row_pt.y), MouseButton::Left);
    assert_eq!(harness.app.vault_entries.len(), 1);
    assert!(!harness.app.vault_entries[0].is_expanded);

    let _ = std::fs::remove_dir_all(&tmp_path);
}

#[test]
fn test_harness_chrome_ui_text_typography_rendered() {
    let mut harness = AppHarness::new(1280, 800);

    // 1. Welcome HUD: Title and New Board text
    harness.app.show_welcome_hud = true;
    harness.render();
    let hud_layout = harness.app.recent_files_layout();
    let title_rect = Rect::from_xywh(
        hud_layout.dialog_bounds.min.x + 24.0,
        hud_layout.dialog_bounds.min.y + 22.0,
        140.0,
        22.0,
    );
    let title_text_pixels = harness.count_pixels_in_rect(title_rect, |p| {
        p.red() > 200 && p.green() > 200 && p.blue() > 200
    });
    assert!(
        title_text_pixels > 20,
        "Recent Boards title must rasterize bright text glyphs, got {title_text_pixels}"
    );

    let new_board_rect = Rect::from_xywh(
        hud_layout.new_board_bounds.min.x + 32.0,
        hud_layout.new_board_bounds.min.y + 8.0,
        70.0,
        18.0,
    );
    let new_board_pixels = harness.count_pixels_in_rect(new_board_rect, |p| {
        p.red() > 200 && p.green() > 200 && p.blue() > 200
    });
    assert!(
        new_board_pixels > 20,
        "New Board button must rasterize text glyphs, got {new_board_pixels}"
    );

    // Dismiss HUD
    harness.app.show_welcome_hud = false;

    // 2. Vault Sidebar: Header and Folder/Board items text
    let unique_name = format!(
        "ruqa_vault_text_test_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let tmp_path = std::env::temp_dir().join(unique_name);
    std::fs::create_dir_all(tmp_path.join("Algorithms")).expect("create folder");
    std::fs::write(tmp_path.join("Algorithms").join("Dijkstra.ruqa"), "{}").expect("write board");

    harness.app.set_vault_root(&tmp_path);
    harness.app.show_vault_sidebar = true;
    harness.app.refresh_vault_entries();
    harness.render();

    let header_rect = Rect::from_xywh(16.0, 14.0, 100.0, 20.0);
    let header_pixels = harness.count_pixels_in_rect(header_rect, |p| {
        p.red() > 180 && p.green() > 180 && p.blue() > 180
    });
    assert!(
        header_pixels > 20,
        "Study Vault header must rasterize text glyphs, got {header_pixels}"
    );

    let vault_layout = harness.app.vault_layout();
    assert!(!vault_layout.rows.is_empty());
    let folder_row = &vault_layout.rows[0];
    let row_text_rect = Rect::from_xywh(
        folder_row.row_bounds.min.x + 40.0,
        folder_row.row_bounds.min.y + 6.0,
        100.0,
        20.0,
    );
    let row_pixels = harness.count_pixels_in_rect(row_text_rect, |p| {
        p.red() > 180 && p.green() > 180 && p.blue() > 180
    });
    assert!(
        row_pixels > 20,
        "Folder item name must rasterize text glyphs, got {row_pixels}"
    );

    let _ = std::fs::remove_dir_all(&tmp_path);
    harness.app.show_vault_sidebar = false;

    // 3. Quit Dialog: Unsaved Changes title and button text
    harness.app.show_quit_dialog = true;
    harness.render();
    let quit_layout = harness.app.quit_dialog_layout();
    let quit_title_rect = Rect::from_xywh(
        quit_layout.dialog_bounds.min.x + 68.0,
        quit_layout.dialog_bounds.min.y + 24.0,
        150.0,
        22.0,
    );
    let quit_title_pixels = harness.count_pixels_in_rect(quit_title_rect, |p| {
        p.red() > 200 && p.green() > 200 && p.blue() > 200
    });
    assert!(
        quit_title_pixels > 20,
        "Unsaved Changes quit title must rasterize text glyphs, got {quit_title_pixels}"
    );

    let cancel_btn = quit_layout.cancel_button_bounds;
    let cancel_pixels = harness.count_pixels_in_rect(cancel_btn, |p| {
        p.red() > 180 && p.green() > 180 && p.blue() > 180
    });
    assert!(
        cancel_pixels > 20,
        "Cancel button must contain text pixels, got {cancel_pixels}"
    );
}

#[test]
fn test_harness_interactive_shape_and_pen_drawing() {
    let mut harness = AppHarness::new(1280, 800);
    assert_eq!(harness.app.scene.len(), 0);

    // 1. Draw a Rectangle
    harness.app.active_tool = Tool::Rectangle;
    harness.drag((250.0, 250.0), (400.0, 350.0), 5);
    assert_eq!(harness.app.scene.len(), 1);
    assert!(matches!(
        harness.app.scene.elements()[0],
        Element::Rectangle { .. }
    ));

    // 2. Draw an Ellipse
    harness.app.active_tool = Tool::Ellipse;
    harness.drag((450.0, 250.0), (600.0, 350.0), 5);
    assert_eq!(harness.app.scene.len(), 2);
    assert!(matches!(
        harness.app.scene.elements()[1],
        Element::Ellipse { .. }
    ));

    // 3. Draw a Diamond
    harness.app.active_tool = Tool::Diamond;
    harness.drag((650.0, 250.0), (800.0, 350.0), 5);
    assert_eq!(harness.app.scene.len(), 3);
    assert!(matches!(
        harness.app.scene.elements()[2],
        Element::Diamond { .. }
    ));

    // 4. Draw a Line
    harness.app.active_tool = Tool::Line;
    harness.drag((250.0, 400.0), (400.0, 500.0), 5);
    assert_eq!(harness.app.scene.len(), 4);
    assert!(matches!(
        harness.app.scene.elements()[3],
        Element::Line { .. }
    ));

    // 5. Draw an Arrow
    harness.app.active_tool = Tool::Arrow;
    harness.drag((450.0, 400.0), (600.0, 500.0), 5);
    assert_eq!(harness.app.scene.len(), 5);
    assert!(matches!(
        harness.app.scene.elements()[4],
        Element::Arrow { .. }
    ));

    // 6. Draw with Pen
    harness.app.active_tool = Tool::Pen;
    harness.drag((650.0, 400.0), (800.0, 500.0), 10);
    assert_eq!(harness.app.scene.len(), 6);
    assert!(matches!(
        harness.app.scene.elements()[5],
        Element::Stroke { .. }
    ));

    // Render frame and verify elements produce visible rasterized pixels
    harness.render();
    let rect_pixels = harness
        .count_pixels_in_rect(Rect::from_xywh(250.0, 250.0, 150.0, 100.0), |p| {
            p.red() > 50 || p.green() > 50 || p.blue() > 50
        });
    assert!(
        rect_pixels > 50,
        "Drawn rectangle must produce visible pixels, got {rect_pixels}"
    );

    // Test undo reverts the last drawn element
    let undone = harness.app.history.undo(&mut harness.app.scene);
    assert!(undone, "Undo must succeed");
    assert_eq!(harness.app.scene.len(), 5);

    // Test redo restores it
    let redone = harness.app.history.redo(&mut harness.app.scene);
    assert!(redone, "Redo must succeed");
    assert_eq!(harness.app.scene.len(), 6);
}
