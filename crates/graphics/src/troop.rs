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

use crate::{Drawable, GraphicsState, Quad, Renderable, Sprite, Transform, Viewport};
use luminol_data::OptionVec;

pub const TROOP_WIDTH: usize = 640;
pub const TROOP_HEIGHT: usize = 320;

pub struct Troop {
    pub viewport: Viewport,

    members: OptionVec<Member>,
}

pub struct Member {
    pub sprite: Sprite,
    pub rect: egui::Rect,
}

impl Troop {
    pub fn new(graphics_state: &GraphicsState) -> Self {
        let viewport = Viewport::new(
            graphics_state,
            glam::vec2(TROOP_WIDTH as f32, TROOP_HEIGHT as f32),
        );

        Self {
            viewport,
            members: Default::default(),
        }
    }

    pub fn rebuild_members(
        &mut self,
        graphics_state: &GraphicsState,
        filesystem: &impl luminol_filesystem::FileSystem,
        enemies: &luminol_data::rpg::Enemies,
        troop: &luminol_data::rpg::Troop,
    ) {
        let mut members = std::mem::take(&mut self.members);
        members.clear();
        members.extend(troop.members.iter().enumerate().filter_map(|(i, member)| {
            self.create_member(graphics_state, filesystem, enemies, member)
                .map(|member| (i, member))
        }));
        self.members = members;
    }

    fn create_member(
        &self,
        graphics_state: &GraphicsState,
        filesystem: &impl luminol_filesystem::FileSystem,
        enemies: &luminol_data::rpg::Enemies,
        member: &luminol_data::rpg::troop::Member,
    ) -> Option<Member> {
        member.enemy_id.and_then(|enemy_id| {
            let filename = enemies.data.get(enemy_id)?.battler_name.as_ref()?;
            let texture = graphics_state
                .texture_loader
                .load_now_dir(filesystem, "Graphics/Battlers", filename)
                .map_err(|e| {
                    graphics_state.send_texture_error(
                        e.wrap_err(format!("Error loading battler graphic {filename:?}")),
                    );
                })
                .ok()?;
            let rect = egui::Rect::from_min_max(
                egui::Pos2::ZERO,
                egui::pos2(
                    texture.texture.width() as f32,
                    texture.texture.height() as f32,
                ),
            );
            let offset = glam::vec2(
                member.x as f32 - (texture.texture.width() + TROOP_WIDTH as u32) as f32 / 2.,
                member.y as f32 - texture.texture.height() as f32 - TROOP_HEIGHT as f32 / 2.,
            );
            Some(Member {
                sprite: Sprite::new(
                    graphics_state,
                    Quad::new(rect, rect),
                    0,
                    if member.hidden { 32 } else { 255 },
                    luminol_data::BlendMode::Normal,
                    &texture,
                    &self.viewport,
                    Transform::new_position(graphics_state, offset),
                ),
                rect: rect.translate(egui::vec2(offset.x, offset.y)),
            })
        })
    }
}

pub struct Prepared {
    members: Vec<<Sprite as Renderable>::Prepared>,
}

impl Renderable for Troop {
    type Prepared = Prepared;

    fn prepare(&mut self, graphics_state: &std::sync::Arc<GraphicsState>) -> Self::Prepared {
        Self::Prepared {
            members: self
                .members
                .iter_mut()
                .rev()
                .map(|(_, member)| member.sprite.prepare(graphics_state))
                .collect(),
        }
    }
}

impl Drawable for Prepared {
    fn draw<'rpass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'rpass>) {
        for sprite in &self.members {
            sprite.draw(render_pass);
        }
    }
}
