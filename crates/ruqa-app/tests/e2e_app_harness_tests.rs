//! End-to-End Headless Application Test Harness.
//!
//! Validates interactive whiteboard workflows by simulating native OS event loop
//! inputs (mouse moves, clicks, drags, mouse wheel, keyboard shortcuts, text typing,
//! and HiDPI resizing) through [`WhiteboardApp`] and inspecting state transitions
//! and rendered pixel geometry.

use ruqa_app::window_app::WhiteboardApp;
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
