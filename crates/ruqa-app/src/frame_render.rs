//! Frame rendering pass for the desktop whiteboard application.

use crate::window_app::WhiteboardApp;
use tiny_skia::Transform;
use tracing::warn;

impl WhiteboardApp {
    /// Renders the scene and interactive overlays into the presenter surface.
    pub fn render_frame(&mut self) {
        let logical_size = self.logical_size();
        let center = logical_size.center();
        let zoom = self.viewport.zoom;
        let pan = self.viewport.pan;
        let active_marquee = self.selection.active_marquee();
        let selection_bounds = self.scene.selection_bounds(self.selection.selected_ids());
        let toolbar_layout = ruqa_render::toolbar_render::ToolbarLayout::new(logical_size.width);
        let style_layout = self.style_panel_layout();

        let Some(presenter) = &mut self.presenter else {
            return;
        };

        presenter.clear_color_hex(self.theme_mode.background_color());

        if self.show_grid {
            ruqa_render::grid_render::render_grid(
                &mut presenter.pixmap_mut().as_mut(),
                &self.viewport,
                logical_size,
                &self.grid_config,
            );
        }

        ruqa_render::shape_render::render_scene_viewport(
            &self.scene,
            &self.viewport,
            logical_size,
            &mut presenter.pixmap_mut().as_mut(),
        );

        if let Some(bounds) = selection_bounds {
            ruqa_render::selection_render::render_selection(
                bounds,
                &self.viewport,
                logical_size,
                &mut presenter.pixmap_mut().as_mut(),
                &ruqa_render::selection_render::SelectionStyle::default(),
            );
        }

        if let Some(marquee) = active_marquee {
            let t = Transform::from_row(
                zoom,
                0.0,
                0.0,
                zoom,
                center.x - pan.x * zoom,
                center.y - pan.y * zoom,
            );
            ruqa_render::shape_render::render_marquee(
                marquee.rect(),
                &mut presenter.pixmap_mut().as_mut(),
                t,
            );
        }

        ruqa_render::toolbar_render::render_toolbar(
            &mut presenter.pixmap_mut().as_mut(),
            &toolbar_layout,
            self.active_tool,
            self.hovered_tool,
            self.scale_factor,
            &ruqa_render::toolbar_render::ToolbarStyle::default(),
        );

        if self.show_style_panel {
            ruqa_render::style_panel_render::render_style_panel(
                &mut presenter.pixmap_mut().as_mut(),
                &style_layout,
                &self.current_style.stroke_color,
                self.current_style.stroke_width,
                self.current_style.roughness,
                self.scale_factor,
                &ruqa_render::style_panel_render::StylePanelTheme::default(),
            );
        }

        if let Err(e) = presenter.present() {
            warn!(?e, "frame present failed");
        } else {
            self.frames_presented += 1;
        }
    }
}
