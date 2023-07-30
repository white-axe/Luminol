// Copyright (C) 2023 Lily Lyons
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
pub use crate::prelude::*;

use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Tilepicker {
    resources: Arc<Resources>,
    ani_instant: Instant,
    map_id: usize,
    selected_tile: SelectedTile,
}

#[derive(Debug)]
struct Resources {
    tiles: Tiles,
    viewport: Viewport,
    atlas: Atlas,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum SelectedTile {
    Autotile(i16),
    Tile(i16),
}
impl Default for SelectedTile {
    fn default() -> Self {
        SelectedTile::Autotile(0)
    }
}

type ResourcesHash = HashMap<usize, Arc<Resources>>;

impl Tilepicker {
    pub fn new(map_id: usize, tileset: &rpg::Tileset) -> Result<Tilepicker, String> {
        let atlas = state!().atlas_cache.load_atlas(tileset)?;

        let tilepicker_data = [0, 48, 96, 144, 192, 240, 288, 336]
            .into_iter()
            .chain(384..(atlas.tileset_height as i16 / 32 * 8 + 384))
            .collect_vec();
        let tilepicker_data = Table3::new_data(
            8,
            1 + (atlas.tileset_height / 32) as usize,
            1,
            tilepicker_data,
        );
        let tiles = Tiles::new(atlas.clone(), &tilepicker_data);

        let viewport = Viewport::new();
        viewport.set_proj(cgmath::ortho(
            0.0,
            256.,
            atlas.tileset_height as f32,
            0.0,
            -1.0,
            1.0,
        ));

        Ok(Self {
            resources: Arc::new(Resources {
                tiles,
                viewport,
                atlas,
            }),
            map_id,
            ani_instant: Instant::now(),
            selected_tile: SelectedTile::default(),
        })
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        if self.ani_instant.elapsed() >= Duration::from_secs_f32((1. / 60.) * 16.) {
            self.ani_instant = Instant::now();
            self.resources.tiles.autotiles.inc_ani_index();
        }
        ui.ctx().request_repaint_after(Duration::from_millis(16));

        let (canvas_rect, response) = ui.allocate_exact_size(
            egui::vec2(256., self.resources.atlas.tileset_height as f32),
            egui::Sense::click_and_drag(),
        );

        let resources = self.resources.clone();
        let map_id = self.map_id;
        ui.painter().add(egui::PaintCallback {
            rect: canvas_rect,
            callback: Arc::new(
                egui_wgpu::CallbackFn::new()
                    .prepare(move |_, _, _encoder, paint_callback_resources| {
                        let res_hash: &mut ResourcesHash = paint_callback_resources
                            .entry()
                            .or_insert_with(Default::default);
                        res_hash.insert(map_id, resources.clone());

                        vec![]
                    })
                    .paint(move |_info, render_pass, paint_callback_resources| {
                        //
                        let res_hash: &ResourcesHash = paint_callback_resources.get().unwrap();
                        let resources = &res_hash[&map_id];
                        let Resources {
                            tiles, viewport, ..
                        } = resources.as_ref();

                        viewport.bind(render_pass);
                        tiles.draw(render_pass, &[]);
                    }),
            ),
        });

        let pos = match self.selected_tile {
            SelectedTile::Autotile(t) => egui::vec2(t as f32 * 32., 0.),
            SelectedTile::Tile(t) => {
                let tile_x = t % 8 * 32;
                let tile_y = (t / 8) * 32 - 1_504;
                egui::vec2(tile_x as f32, tile_y as f32)
            }
        };
        let rect = egui::Rect::from_min_size(canvas_rect.min + pos, egui::Vec2::splat(32.));
        ui.painter()
            .rect_stroke(rect, 5.0, egui::Stroke::new(1.0, egui::Color32::WHITE));

        let Some(pos) = response.interact_pointer_pos() else {
            return response;
        };
        let pos = (pos - canvas_rect.min) / 32.;
        let cursor_x = pos.x as i16;
        let cursor_y = pos.y as i16;

        if response.clicked() {
            self.selected_tile = match cursor_y {
                ..=0 => SelectedTile::Autotile(cursor_x),
                _ => SelectedTile::Tile(cursor_x + (cursor_y - 1) * 8 + 384),
            };
        }

        response
    }
}
