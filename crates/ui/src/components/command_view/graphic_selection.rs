// Copyright (C) 2024 Melody Madeline Lyons
//
// This file is part of Luminol.
//
// Luminol is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Luminol is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Luminol.  If not, see <http://www.gnu.org/licenses/>.
//
//     Additional permission under GNU GPL version 3 section 7
//
// If you modify this Program, or any covered work, by linking or combining
// it with Steamworks API by Valve Corporation, containing parts covered by
// terms of the Steamworks API by Valve Corporation, the licensors of this
// Program grant you additional permission to convey the resulting work.

use crate::components::FileComboBox;
use luminol_core::UpdateState;
use luminol_graphics::{Painter, Renderable, Sprite, Texture, Viewport};

struct GraphicSelectionStateInner {
    texture: std::sync::Arc<Texture>,
    viewport: Viewport,
    sprite: Sprite,
}

#[derive(Default)]
pub struct GraphicSelectionState {
    attempted_to_load: bool,
    inner: Option<GraphicSelectionStateInner>,
}

/// A widget for changing the value of a graphic parameter.
#[must_use = "use `ui.add()` to show this widget"]
pub struct GraphicSelection<'this, 'update_state, P, D, H> {
    update_state: &'this mut UpdateState<'update_state>,
    state: &'this mut GraphicSelectionState,
    id_salt: H,
    directory_path: D,
    parameter: P,
}

impl<'this, 'update_state, P, D, H> GraphicSelection<'this, 'update_state, P, D, H>
where
    P: super::StringParameterMut,
    D: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    pub fn new(
        update_state: &'this mut UpdateState<'update_state>,
        state: &'this mut GraphicSelectionState,
        id_salt: H,
        directory_path: D,
        parameter: P,
    ) -> Self {
        Self {
            update_state,
            state,
            id_salt,
            directory_path,
            parameter,
        }
    }
}

impl<P, D, H> egui::Widget for GraphicSelection<'_, '_, P, D, H>
where
    P: super::StringParameterMut,
    D: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut modified = false;

        let directory_path = self.directory_path.as_ref();
        let filename = self.parameter.as_string_mut();

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                modified |= {
                    let changed = ui
                        .add(FileComboBox::new(
                            self.update_state,
                            (&self.id_salt, "filename"),
                            directory_path,
                            filename,
                        ))
                        .changed();

                    if changed || !self.state.attempted_to_load {
                        self.state.attempted_to_load = true;
                        self.state.inner = self
                            .update_state
                            .graphics
                            .texture_loader
                            .load_now_dir(self.update_state.filesystem, directory_path, filename)
                            .map_err(|e| luminol_core::error!(self.update_state.toasts, e))
                            .ok()
                            .map(|texture| {
                                let viewport =
                                    Viewport::new(&self.update_state.graphics, Default::default());
                                let sprite =
                                    Sprite::basic(&self.update_state.graphics, &texture, &viewport);
                                GraphicSelectionStateInner {
                                    texture,
                                    viewport,
                                    sprite,
                                }
                            });
                    }

                    changed
                };

                if let Some(state) = &mut self.state.inner {
                    egui::ScrollArea::both()
                        .id_salt((&self.id_salt, "scroll"))
                        .max_height(200.)
                        .show_viewport(ui, |ui, viewport_rect| {
                            let (canvas_rect, _) = ui.allocate_exact_size(
                                state.texture.size_vec2(),
                                egui::Sense::click(),
                            );
                            let absolute_scroll_rect = ui
                                .ctx()
                                .screen_rect()
                                .intersect(viewport_rect.translate(canvas_rect.min.to_vec2()));
                            let scroll_rect =
                                absolute_scroll_rect.translate(-canvas_rect.min.to_vec2());
                            state.sprite.transform.set_position(
                                &self.update_state.graphics.render_state,
                                glam::vec2(-scroll_rect.left(), -scroll_rect.top()),
                            );
                            state.viewport.set(
                                &self.update_state.graphics.render_state,
                                glam::vec2(
                                    absolute_scroll_rect.width(),
                                    absolute_scroll_rect.height(),
                                ),
                                glam::Vec2::ZERO,
                                glam::Vec2::ONE,
                            );
                            let painter =
                                Painter::new(state.sprite.prepare(&self.update_state.graphics));
                            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                                absolute_scroll_rect,
                                painter,
                            ));
                        });
                }
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
