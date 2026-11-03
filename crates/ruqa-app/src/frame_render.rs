//! Frame rendering pass for the desktop whiteboard application.

use crate::window_app::WhiteboardApp;
use tiny_skia::{Color, PixmapMut, Transform};
use tracing::warn;

impl WhiteboardApp {
    /// Renders the scene and interactive overlays into the presenter surface.
    pub fn render_frame(&mut self) {
        let Some(mut presenter) = self.presenter.take() else {
            return;
        };

        self.render_to_pixmap(&mut presenter.pixmap_mut().as_mut());
        let res = presenter.present();
        self.presenter = Some(presenter);

        if let Err(e) = res {
            warn!(?e, "frame present failed");
        } else {
            self.frames_presented += 1;
        }
    }

    /// Renders the scene, orientation grid, elements, and interactive overlays directly into `pixmap`.
    pub fn render_to_pixmap(&self, pixmap: &mut PixmapMut<'_>) {
        let logical_size = self.logical_size();
        let center = logical_size.center();
        let zoom = self.viewport.zoom;
        let pan = self.viewport.pan;
        let active_marquee = self.selection.active_marquee();
        let selection_bounds = self.scene.selection_bounds(self.selection.selected_ids());
        let toolbar_layout = ruqa_render::toolbar_render::ToolbarLayout::new(logical_size.width);
        let style_layout = self.style_panel_layout();
        let recent_layout = if self.show_welcome_hud {
            Some(self.recent_files_layout())
        } else {
            None
        };
        let vault_layout = if self.show_vault_sidebar {
            Some(self.vault_layout())
        } else {
            None
        };
        let quit_layout = if self.show_quit_dialog {
            Some(self.quit_dialog_layout())
        } else {
            None
        };

        if let Some((r, g, b, a)) =
            ruqa_core::theme::parse_hex_color(self.theme_mode.background_color())
        {
            pixmap.fill(Color::from_rgba8(r, g, b, a));
        } else {
            pixmap.fill(Color::from_rgba8(0x12, 0x12, 0x12, 0xFF));
        }

        #[allow(clippy::cast_possible_truncation)]
        let scale_f32 = self.scale_factor as f32;

        if self.show_grid {
            ruqa_render::grid_render::render_grid_scaled(
                pixmap,
                &self.viewport,
                logical_size,
                scale_f32,
                &self.grid_config,
            );
        }

        ruqa_render::shape_render::render_scene_viewport_scaled(
            &self.scene,
            &self.viewport,
            logical_size,
            scale_f32,
            pixmap,
        );

        if self.is_editing_text {
            let t = Transform::from_row(
                zoom,
                0.0,
                0.0,
                zoom,
                center.x - pan.x * zoom,
                center.y - pan.y * zoom,
            )
            .post_scale(scale_f32, scale_f32);

            let font_size = 20.0;
            if !self.text_edit_buffer.is_empty() {
                ruqa_render::text_render::render_text_element(
                    self.text_edit_position,
                    &self.text_edit_buffer,
                    font_size,
                    &self.current_style,
                    pixmap,
                    t,
                );
            }

            let (tw, th) = if self.text_edit_buffer.is_empty() {
                (0.0, font_size * 1.2)
            } else {
                ruqa_render::measure_text_dimensions(&self.text_edit_buffer, font_size)
            };

            let caret_screen_x =
                (center.x + (self.text_edit_position.x + tw - pan.x) * zoom) * scale_f32;
            let caret_screen_y =
                (center.y + (self.text_edit_position.y - pan.y) * zoom) * scale_f32;
            let caret_h = th * zoom * scale_f32;

            if let Some(rect) = tiny_skia::Rect::from_xywh(
                caret_screen_x + 2.0 * scale_f32,
                caret_screen_y,
                2.0 * scale_f32,
                caret_h,
            ) {
                let mut paint = tiny_skia::Paint::default();
                paint.set_color(tiny_skia::Color::from_rgba8(56, 189, 248, 255));
                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            }
        }

        if let Some(bounds) = selection_bounds {
            ruqa_render::selection_render::render_selection_scaled(
                bounds,
                &self.viewport,
                logical_size,
                scale_f32,
                pixmap,
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
            )
            .post_scale(scale_f32, scale_f32);
            ruqa_render::shape_render::render_marquee(marquee.rect(), pixmap, t);
        }

        ruqa_render::toolbar_render::render_toolbar(
            pixmap,
            &toolbar_layout,
            self.active_tool,
            self.hovered_tool,
            self.scale_factor,
            &ruqa_render::toolbar_render::ToolbarStyle::default(),
        );

        if self.show_style_panel {
            ruqa_render::style_panel_render::render_style_panel(
                pixmap,
                &style_layout,
                &self.current_style.stroke_color,
                self.current_style.stroke_width,
                self.current_style.roughness,
                self.scale_factor,
                &ruqa_render::style_panel_render::StylePanelTheme::default(),
            );
        }

        if let Some(recent_layout) = recent_layout {
            ruqa_render::recent_files_render::render_recent_files_hud_scaled(
                pixmap,
                &recent_layout,
                &self.recent_files.entries,
                self.hovered_recent_file.as_deref(),
                self.scale_factor,
                &ruqa_render::recent_files_render::RecentFilesTheme::default(),
            );
        }

        if let Some(vault_layout) = vault_layout {
            ruqa_render::vault_render::render_vault_sidebar_scaled(
                pixmap,
                &vault_layout,
                self.file_path.as_deref(),
                self.hovered_vault_item.as_deref(),
                self.scale_factor,
                &ruqa_render::vault_render::VaultTheme::default(),
            );
        }

        if let Some(quit_layout) = quit_layout {
            ruqa_render::render_quit_dialog_scaled(
                pixmap,
                &quit_layout,
                self.hovered_quit_action,
                self.scale_factor,
                &ruqa_render::QuitDialogTheme::default(),
            );
        }
    }
}
