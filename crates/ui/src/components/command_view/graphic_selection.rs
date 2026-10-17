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

use super::{IntegerParameterMut, StringParameterMut};
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
    inner: Option<fragile::Sticky<GraphicSelectionStateInner>>,
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

/// A widget for changing the value of a graphic parameter and a corresponding hue parameter.
#[must_use = "use `ui.add()` to show this widget"]
pub struct GraphicSelectionWithHue<'this, 'update_state, P, Q, D, H> {
    inner: GraphicSelection<'this, 'update_state, P, D, H>,
    hue_parameter: Q,
}

/// A widget for changing the value of a graphic parameter and corresponding hue and opacity
/// parameters.
#[must_use = "use `ui.add()` to show this widget"]
pub struct GraphicSelectionWithHueOpacity<'this, 'update_state, P, Q, R, D, H> {
    inner: GraphicSelectionWithHue<'this, 'update_state, P, Q, D, H>,
    opacity_parameter: R,
}

impl<'this, 'update_state, P, D, H> GraphicSelection<'this, 'update_state, P, D, H>
where
    P: StringParameterMut,
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

    /// Adds an additional parameter that controls the graphic's hue.
    pub fn hue_parameter<Q>(
        self,
        hue_parameter: Q,
    ) -> GraphicSelectionWithHue<'this, 'update_state, P, Q, D, H>
    where
        Q: IntegerParameterMut,
    {
        GraphicSelectionWithHue {
            inner: self,
            hue_parameter,
        }
    }
}

impl<'this, 'update_state, P, Q, D, H> GraphicSelectionWithHue<'this, 'update_state, P, Q, D, H>
where
    P: StringParameterMut,
    Q: IntegerParameterMut,
    D: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    /// Adds an additional parameter that controls the graphic's opacity.
    pub fn opacity_parameter<R>(
        self,
        opacity_parameter: R,
    ) -> GraphicSelectionWithHueOpacity<'this, 'update_state, P, Q, R, D, H>
    where
        R: IntegerParameterMut,
    {
        GraphicSelectionWithHueOpacity {
            inner: self,
            opacity_parameter,
        }
    }
}

impl<P, D, H> GraphicSelection<'_, '_, P, D, H>
where
    P: StringParameterMut,
    D: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    fn ui_inner<Q, R>(
        mut self,
        ui: &mut egui::Ui,
        maybe_hue_parameter: Option<Q>,
        maybe_opacity_parameter: Option<R>,
    ) -> egui::Response
    where
        Q: IntegerParameterMut,
        R: IntegerParameterMut,
    {
        let mut modified = false;

        let directory_path = self.directory_path.as_ref();
        let filename = self.parameter.as_string_mut();

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                let (hue, opacity) = {
                    let changed = ui
                        .add(FileComboBox::new(
                            self.update_state,
                            (&self.id_salt, "filename"),
                            directory_path,
                            filename,
                        ))
                        .changed();

                    let hue = maybe_hue_parameter
                        .map(|mut hue_parameter| {
                            ui.label("Hue");
                            modified |= ui
                                .add(
                                    egui::DragValue::new(hue_parameter.as_integer_mut())
                                        .range(0..=360),
                                )
                                .changed();
                            hue_parameter.as_integer()
                        })
                        .unwrap_or(0);

                    let opacity = maybe_opacity_parameter
                        .map(|mut opacity_parameter| {
                            ui.label("Opacity");
                            modified |= ui
                                .add(
                                    egui::DragValue::new(opacity_parameter.as_integer_mut())
                                        .range(0..=255),
                                )
                                .changed();
                            opacity_parameter.as_integer()
                        })
                        .unwrap_or(255);

                    if changed
                        || !self.state.attempted_to_load
                        || self
                            .state
                            .inner
                            .as_ref()
                            .is_some_and(|state| !state.is_valid())
                    {
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
                                let sprite = Sprite::basic_hue_opacity(
                                    &self.update_state.graphics,
                                    hue,
                                    opacity,
                                    &texture,
                                    &viewport,
                                );
                                fragile::Sticky::new(GraphicSelectionStateInner {
                                    texture,
                                    viewport,
                                    sprite,
                                })
                            });
                    }

                    (hue, opacity)
                };

                if let Some(state) = &mut self.state.inner {
                    let _ = state.try_with_mut(|state| {
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
                                state
                                    .sprite
                                    .graphic
                                    .set_hue(&self.update_state.graphics.render_state, hue);
                                state
                                    .sprite
                                    .graphic
                                    .set_opacity(&self.update_state.graphics.render_state, opacity);
                                let painter =
                                    Painter::new(state.sprite.prepare(&self.update_state.graphics));
                                ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                                    absolute_scroll_rect,
                                    painter,
                                ));
                            });
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

impl<P, D, H> egui::Widget for GraphicSelection<'_, '_, P, D, H>
where
    P: StringParameterMut,
    D: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.ui_inner::<&mut i32, &mut i32>(ui, None, None)
    }
}

impl<P, Q, D, H> egui::Widget for GraphicSelectionWithHue<'_, '_, P, Q, D, H>
where
    P: StringParameterMut,
    Q: IntegerParameterMut,
    D: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.inner
            .ui_inner::<_, &mut i32>(ui, Some(self.hue_parameter), None)
    }
}

impl<P, Q, R, D, H> egui::Widget for GraphicSelectionWithHueOpacity<'_, '_, P, Q, R, D, H>
where
    P: StringParameterMut,
    Q: IntegerParameterMut,
    R: IntegerParameterMut,
    D: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.inner.inner.ui_inner(
            ui,
            Some(self.inner.hue_parameter),
            Some(self.opacity_parameter),
        )
    }
}
