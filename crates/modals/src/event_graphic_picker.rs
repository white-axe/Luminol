// Copyright (C) 2024 Lily Lyons
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

use luminol_core::prelude::*;

pub struct Modal {
    entries: Vec<camino::Utf8PathBuf>,
    open: bool,
    id_source: egui::Id,

    selected: Selected,
    opacity: i32,
    hue: i32,
    blend_mode: rpg::BlendMode,
    first_open: bool,

    tilepicker: Tilepicker,

    button_viewport: Viewport,
    button_sprite: Option<Event>,

    sprite: Option<PreviewSprite>,
}

struct PreviewSprite {
    sprite: Sprite,
    sprite_size: egui::Vec2,
    viewport: Viewport
}

#[derive(PartialEq)]
enum Selected {
    None,
    Tile(usize),
    Graphic {
        path: camino::Utf8PathBuf,
        direction: i32,
        pattern: i32,
    },
}

impl Modal {
    pub fn new(
        update_state: &UpdateState<'_>,
        graphic: &rpg::Graphic,
        tileset: &rpg::Tileset,
        id_source: egui::Id,
    ) -> Self {
        // TODO error handling
        let entries = update_state
            .filesystem
            .read_dir("Graphics/Characters")
            .unwrap()
            .into_iter()
            .map(|m| {
                m.path
                    .strip_prefix("Graphics/Characters")
                    .unwrap_or(&m.path)
                    .with_extension("")
            })
            .collect();

        let tilepicker = Tilepicker::new(
            &update_state.graphics,
            tileset,
            update_state.filesystem,
            true,
        )
        .unwrap();

        let button_viewport = Viewport::new(&update_state.graphics, Default::default());
        let button_sprite = Event::new_standalone(
            &update_state.graphics,
            update_state.filesystem,
            &button_viewport,
            graphic,
            &tilepicker.atlas,
        )
        .unwrap();


        Self {
            entries,
            open: false,
            id_source,

            selected: Selected::None,
            opacity: graphic.opacity,
            hue: graphic.character_hue,
            blend_mode: graphic.blend_type,
            first_open: false,

            tilepicker,

            button_viewport,
            button_sprite,

            sprite: None,
        }
    }
}

impl luminol_core::Modal for Modal {
    type Data = luminol_data::rpg::Graphic;

    fn button<'m>(
        &'m mut self,
        data: &'m mut Self::Data,
        update_state: &'m mut UpdateState<'_>,
    ) -> impl egui::Widget + 'm {
        move |ui: &mut egui::Ui| {
            let desired_size = egui::vec2(64., 96.) + ui.spacing().button_padding * 2.;
            let (rect, response) = ui.allocate_at_least(desired_size, egui::Sense::click());

            let visuals = ui.style().interact_selectable(&response, self.open);
            let rect = rect.expand(visuals.expansion);
            ui.painter()
                .rect(rect, visuals.rounding, visuals.bg_fill, visuals.bg_stroke);

            if let Some(sprite) = &mut self.button_sprite {
                let translation = (desired_size - sprite.sprite_size) / 2.;
                self.button_viewport.set(
                    &update_state.graphics.render_state,
                    glam::vec2(desired_size.x, desired_size.y),
                    glam::vec2(translation.x, translation.y),
                    glam::Vec2::ONE,
                );
                let callback = luminol_egui_wgpu::Callback::new_paint_callback(
                    response.rect,
                    Painter::new(sprite.prepare(&update_state.graphics)),
                );
                ui.painter().add(callback);
            }

            if response.clicked() {

                self.selected = if let Some(id) = data.tile_id {
                    Selected::Tile(id)
                } else if let Some(path) = data.character_name.clone() {
                    Selected::Graphic { path, direction: data.direction, pattern: data.pattern }
                } else {
                    Selected::None
                };
                self.blend_mode = data.blend_type;
                self.hue = data.character_hue;
                self.opacity = data.opacity;
                self.first_open = true;
                self.sprite = None;

                self.open = true;
            }
            self.show_window(update_state, ui.ctx(), data);

            response
        }
    }

    fn reset(&mut self) {
        self.open = false;
    }
}

impl Modal {
    pub fn update_graphic(&mut self, update_state: &UpdateState<'_>, graphic: &rpg::Graphic) {
        self.button_sprite = Event::new_standalone(
            &update_state.graphics,
            update_state.filesystem,
            &self.button_viewport,
            graphic,
            &self.tilepicker.atlas,
        )
        .unwrap();
        self.sprite = None;
    }

    fn show_window(
        &mut self,
        update_state: &luminol_core::UpdateState<'_>,
        ctx: &egui::Context,
        data: &mut rpg::Graphic,
    ) {
        let mut keep_open = true;
        let mut needs_save = false;

        egui::Window::new("Event Graphic Picker")
            .resizable(true)
            .open(&mut self.open)
            .id(self.id_source.with("window"))
            .show(ctx, |ui| {
                egui::SidePanel::left(self.id_source.with("sidebar")).show_inside(ui, |ui| {
                    // FIXME: Its better to use show_rows!
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let res = ui.selectable_value(&mut self.selected, Selected::None, "(None)");
                        if self.first_open && matches!(self.selected, Selected::None) {
                            res.scroll_to_me(Some(egui::Align::Center));
                        }

                        let checked = matches!(self.selected, Selected::Tile(_));
                        let res = ui.selectable_label(
                            checked,
                            "(Tileset)",
                        );

                        if self.first_open && checked {
                            res.scroll_to_me(Some(egui::Align::Center));
                        }

                        if res.clicked() && !checked {
                            self.selected = Selected::Tile(384);
                        }

                        for entry in &self.entries {
                            let checked =
                                matches!(self.selected, Selected::Graphic { ref path, .. } if path == entry);
                            if ui.selectable_label(checked, entry.as_str()).clicked() {
                                self.selected = Selected::Graphic { path: entry.clone(), direction: 2, pattern: 0 };
                                self.sprite = None;
                            }
                            if self.first_open && checked {
                                res.scroll_to_me(Some(egui::Align::Center));
                            }
                        }
                    });
                });

                match &mut self.selected {
                    Selected::None => {}
                    Selected::Graphic { path, direction, pattern } => {
                        let sprite = self.sprite.get_or_insert_with(|| {
                            let texture = update_state.graphics
                                .texture_loader
                                .load_now_dir(update_state.filesystem, "Graphics/Characters", path).unwrap();
                            let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, texture.size_vec2());
                            let quad = Quad::new(rect, rect);
                            let viewport = Viewport::new(&update_state.graphics, glam::vec2(texture.width() as f32, texture.height() as f32));
                            
                            let sprite = Sprite::new(&update_state.graphics, quad, 0, 255, rpg::BlendMode::Normal, &texture, &viewport, Transform::unit(&update_state.graphics));
                            PreviewSprite {
                                sprite,
                                sprite_size: texture.size_vec2(),
                                viewport,
                            }
                        });

                        let (canvas_rect, response) = ui.allocate_exact_size(
                            sprite.sprite_size,
                            egui::Sense::click(),
                        );
                        let painter = Painter::new(sprite.sprite.prepare(&update_state.graphics));
                        ui.painter()
                            .add(luminol_egui_wgpu::Callback::new_paint_callback(
                                canvas_rect,
                                painter,
                            ));

                        let ch = sprite.sprite_size.y / 4.;
                        let cw = sprite.sprite_size.x / 4.;
                        let rect = egui::Rect::from_min_size(egui::pos2(cw ** pattern as f32, ch * (*direction as f32 - 2.) / 2.), egui::vec2(cw, ch)).translate(canvas_rect.min.to_vec2());
                        ui.painter().rect_stroke(rect, 5.0, egui::Stroke::new(1.0, egui::Color32::WHITE));

                        if response.clicked() {
                            let pos = (response.interact_pointer_pos().unwrap() - response.rect.min) / egui::vec2(cw, ch);
                            *direction = pos.y as i32 * 2 + 2;
                            *pattern = pos.x as i32;
                        }
                    }
                    Selected::Tile(id) => {
                        egui::ScrollArea::vertical().show_viewport(ui, |ui, viewport| {
                            let (canvas_rect, response) = ui.allocate_exact_size(
                                egui::vec2(256., self.tilepicker.atlas.tileset_height as f32),
                                egui::Sense::click(),
                            );

                            let absolute_scroll_rect = ui
                                .ctx()
                                .screen_rect()
                                .intersect(viewport.translate(canvas_rect.min.to_vec2()));
                            let scroll_rect = absolute_scroll_rect.translate(-canvas_rect.min.to_vec2());

                            self.tilepicker.grid.display.set_pixels_per_point(
                                &update_state.graphics.render_state,
                                ui.ctx().pixels_per_point(),
                            );

                            self.tilepicker.set_position(
                                &update_state.graphics.render_state,
                                glam::vec2(0.0, -scroll_rect.top()),
                            );
                            self.tilepicker.viewport.set(
                                &update_state.graphics.render_state,
                                glam::vec2(scroll_rect.width(), scroll_rect.height()),
                                glam::Vec2::ZERO,
                                glam::Vec2::ONE,
                            );

                            self.tilepicker
                                .update_animation(&update_state.graphics.render_state, ui.input(|i| i.time));

                            let painter = Painter::new(self.tilepicker.prepare(&update_state.graphics));
                            ui.painter()
                                .add(luminol_egui_wgpu::Callback::new_paint_callback(
                                    absolute_scroll_rect,
                                    painter,
                                ));

                            let tile_x = (*id - 384) % 8;
                            let tile_y = (*id - 384) / 8;
                            let rect = egui::Rect::from_min_size(egui::Pos2::new(tile_x as f32, tile_y as f32) * 32., egui::Vec2::splat(32.)).translate(canvas_rect.min.to_vec2());
                            ui.painter().rect_stroke(rect, 5.0, egui::Stroke::new(1.0, egui::Color32::WHITE));

                            if response.clicked() {
                                let pos = (response.interact_pointer_pos().unwrap() - response.rect.min) / 32.;
                                *id = pos.x as usize + pos.y as usize * 8 + 384;
                            }
                        });
                    }
                }

            });

        self.first_open = false;

        if !keep_open {
            self.open = false;
        }
    }
}
