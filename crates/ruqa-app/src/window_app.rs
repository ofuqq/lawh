//! Window application harness implementing `winit::application::ApplicationHandler`.
//!
//! This module owns the [`WhiteboardApp`] state machine that responds to winit
//! events. Frame presentation is delegated to [`Presenter`], which owns the
//! softbuffer surface and the tiny-skia pixmap.

use crate::event_dispatch::PointerState;
use crate::presenter::Presenter;
use crate::selection::SelectionManager;
use ruqa_core::geometry::{Size, Viewport};
use ruqa_core::scene::Scene;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::Window;

/// Default logical window width in CSS pixels.
pub const DEFAULT_WIDTH: u32 = 1280;
/// Default logical window height in CSS pixels.
pub const DEFAULT_HEIGHT: u32 = 800;
/// Minimum logical window width in CSS pixels.
pub const MIN_WIDTH: u32 = 640;
/// Minimum logical window height in CSS pixels.
pub const MIN_HEIGHT: u32 = 480;
/// Default inactivity duration before autosave triggers for dirty documents.
pub const DEFAULT_AUTOSAVE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);

/// Desktop whiteboard application — the top-level winit [`ApplicationHandler`].
///
/// Owns window state, the frame presenter, pointer state, and the canvas viewport.
pub struct WhiteboardApp {
    /// Window handle; set in [`resumed`](ApplicationHandler::resumed).
    pub window: Option<Arc<Window>>,
    /// Frame presenter (softbuffer surface + tiny-skia pixmap).
    pub presenter: Option<Presenter>,
    /// Mouse pointer tracking (zero-allocation).
    pub pointer: PointerState,
    /// Infinite canvas viewport camera.
    pub viewport: Viewport,
    /// Active scene document graph.
    pub scene: Scene,
    /// Selection tool controller and state.
    pub selection: SelectionManager,
    /// Internal clipboard of copied elements.
    pub clipboard: Vec<ruqa_core::element::Element>,
    /// Undo/redo history stack using the command pattern.
    pub history: ruqa_core::history::HistoryStack,
    /// Active document file path on disk, or `None` if untitled.
    pub file_path: Option<std::path::PathBuf>,
    /// Whether the canvas has unsaved modifications.
    pub is_dirty: bool,
    /// Inactivity interval after which an autosave triggers.
    pub autosave_interval: std::time::Duration,
    /// Timestamp of the most recent modification while dirty.
    pub last_dirty_time: Option<std::time::Instant>,
    /// Active tool mode selected in the toolbar HUD.
    pub active_tool: ruqa_core::tool::Tool,
    /// Currently hovered toolbar button, if cursor is over the toolbar.
    pub hovered_tool: Option<ruqa_core::tool::Tool>,
    /// Whether the pointer was pressed inside the toolbar HUD.
    pub toolbar_pressed: bool,
    /// Default creation style and active style for selected elements.
    pub current_style: ruqa_core::style::ElementStyle,
    /// Whether the contextual style HUD panel is visible.
    pub show_style_panel: bool,
    /// Whether the pointer was pressed inside the style HUD panel.
    pub style_panel_pressed: bool,
    /// Active continuous eraser drag state.
    pub eraser: crate::eraser::EraserState,
    /// Active interactive shape creation and pen gesture state machine.
    pub drawing: crate::drawing::DrawingManager,
    /// Active display theme mode (Dark by default).
    pub theme_mode: ruqa_core::theme::ThemeMode,
    /// Whether background orientation grid dots/lines are visible.
    pub show_grid: bool,
    /// Canvas orientation grid configuration.
    pub grid_config: ruqa_render::grid_render::GridConfig,
    /// Recently opened or saved document history.
    pub recent_files: ruqa_core::recent_files::RecentFiles,
    /// Whether the Recent Files / Welcome HUD modal overlay is visible.
    pub show_welcome_hud: bool,
    /// Currently hovered recent file path in the welcome HUD.
    pub hovered_recent_file: Option<std::path::PathBuf>,
    /// Study Vault folder tree manager.
    pub vault: ruqa_core::vault::StudyVault,
    /// Whether the Study Vault drawer sidebar is visible.
    pub show_vault_sidebar: bool,
    /// Cached list of visible entries in the Study Vault.
    pub vault_entries: Vec<ruqa_core::vault::FlatVaultItem>,
    /// Currently hovered entry path in the Study Vault sidebar.
    pub hovered_vault_item: Option<std::path::PathBuf>,
    /// Whether the user is actively typing in a text field (suppressing single-key tool switches).
    pub is_editing_text: bool,
    /// World anchor coordinate of active inline text editing session.
    pub text_edit_position: ruqa_core::geometry::Point,
    /// Active text edit buffer being typed on canvas.
    pub text_edit_buffer: String,
    /// Whether the unsaved changes quit confirmation dialog is visible.
    pub show_quit_dialog: bool,
    /// Currently hovered action button in the quit confirmation dialog.
    pub hovered_quit_action: Option<ruqa_render::QuitDialogAction>,
    /// Flag signaling the event loop to exit on the next event cycle.
    pub should_exit: bool,
    /// Current physical window size.
    pub physical_size: PhysicalSize<u32>,
    /// HiDPI scale factor (e.g. 1.0 on 96 DPI, 2.0 on Retina).
    pub scale_factor: f64,
    /// Total frames presented.
    pub frames_presented: u64,
    /// Total pointer-motion events received.
    pub motion_events_count: u64,
}

impl Default for WhiteboardApp {
    fn default() -> Self {
        Self::new()
    }
}

impl WhiteboardApp {
    /// Creates initial application state (no window yet).
    #[must_use]
    pub fn new() -> Self {
        Self {
            window: None,
            presenter: None,
            pointer: PointerState::new(),
            viewport: Viewport::default(),
            scene: Scene::new(),
            selection: SelectionManager::new(),
            clipboard: Vec::new(),
            history: ruqa_core::history::HistoryStack::new(),
            file_path: None,
            is_dirty: false,
            autosave_interval: DEFAULT_AUTOSAVE_INTERVAL,
            last_dirty_time: None,
            active_tool: ruqa_core::tool::Tool::default(),
            hovered_tool: None,
            toolbar_pressed: false,
            current_style: ruqa_core::style::ElementStyle {
                stroke_color: ruqa_core::style::PALETTE_COLORS[0].to_string(),
                stroke_width: ruqa_core::style::StrokeWidthPreset::Medium.width(),
                roughness: ruqa_core::style::RoughnessPreset::Sketchy.factor(),
                ..Default::default()
            },
            show_style_panel: true,
            style_panel_pressed: false,
            eraser: crate::eraser::EraserState::new(),
            drawing: crate::drawing::DrawingManager::new(),
            theme_mode: ruqa_core::theme::ThemeMode::Dark,
            show_grid: true,
            grid_config: ruqa_render::grid_render::GridConfig::dark_mode(),
            recent_files: ruqa_core::recent_files::RecentFiles::new(),
            show_welcome_hud: false,
            hovered_recent_file: None,
            vault: ruqa_core::vault::StudyVault::new(
                ruqa_core::vault::StudyVault::default_vault_dir(),
            ),
            show_vault_sidebar: false,
            vault_entries: Vec::new(),
            hovered_vault_item: None,
            is_editing_text: false,
            text_edit_position: ruqa_core::geometry::Point::ZERO,
            text_edit_buffer: String::new(),
            show_quit_dialog: false,
            hovered_quit_action: None,
            should_exit: false,
            physical_size: PhysicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            scale_factor: 1.0,
            frames_presented: 0,
            motion_events_count: 0,
        }
    }

    /// Returns the current logical window size in pixels.
    #[must_use]
    pub fn logical_size(&self) -> Size {
        #[allow(clippy::cast_possible_truncation)]
        Size::new(
            (f64::from(self.physical_size.width) / self.scale_factor) as f32,
            (f64::from(self.physical_size.height) / self.scale_factor) as f32,
        )
    }

    /// Asks the window to schedule a redraw, if the window exists.
    pub fn request_redraw(&self) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    /// Toggles visibility of the background canvas orientation grid.
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
        self.request_redraw();
    }

    /// Toggles between dark and light theme modes, updating canvas background, grid, and inverting existing elements.
    pub fn toggle_theme(&mut self) {
        self.theme_mode = self.theme_mode.toggle();
        match self.theme_mode {
            ruqa_core::theme::ThemeMode::Dark => {
                self.grid_config = ruqa_render::grid_render::GridConfig::dark_mode();
            }
            ruqa_core::theme::ThemeMode::Light => {
                self.grid_config = ruqa_render::grid_render::GridConfig::light_mode();
            }
        }
        self.current_style.stroke_color =
            ruqa_core::theme::invert_color(&self.current_style.stroke_color);
        ruqa_core::theme::invert_scene(&mut self.scene);
        self.mark_dirty();
        self.request_redraw();
    }

    /// Populates the whiteboard canvas with sample demonstration primitives.
    pub fn load_demo_board(&mut self) {
        crate::demo::populate_demo_board(&mut self.scene);
    }

    /// Commits the active text editing buffer into a new [`ruqa_core::element::Element::Text`] in the scene.
    pub fn commit_text_edit(&mut self) -> bool {
        if !self.is_editing_text {
            return false;
        }
        self.is_editing_text = false;
        let content = self.text_edit_buffer.trim().to_string();
        self.text_edit_buffer.clear();
        if content.is_empty() {
            return false;
        }

        let id = self.scene.next_element_id();
        let element = ruqa_core::element::Element::Text {
            id,
            position: self.text_edit_position,
            content,
            font_size: 20.0,
            style: self.current_style.clone(),
        };

        self.scene.push(element.clone());
        self.history
            .push(ruqa_core::history::Command::AddElement(element));
        self.mark_dirty();
        self.selection.set_selection([id]);
        true
    }

    /// Handles pointer click for the text tool at `world_pt`, spawning an inline text editing session.
    pub fn handle_text_tool_pointer_down(&mut self, world_pt: ruqa_core::geometry::Point) {
        if self.is_editing_text {
            self.commit_text_edit();
        }
        self.is_editing_text = true;
        self.text_edit_position = world_pt;
        self.text_edit_buffer.clear();
        self.request_redraw();
    }
}
