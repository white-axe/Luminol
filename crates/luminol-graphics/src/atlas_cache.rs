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

#[derive(Default, Debug)]
pub struct Cache {
    atlases: dashmap::DashMap<usize, crate::tiles::Atlas>,
}

impl Cache {
    pub fn load_atlas(
        &self,
        render_state: &egui_wgpu::RenderState,
        filesystem: &impl luminol_core::filesystem::FileSystem,
        image_cache: &crate::image_cache::Cache,
        tileset: &luminol_data::rpg::Tileset,
    ) -> Result<crate::tiles::Atlas, String> {
        Ok(self
            .atlases
            .entry(tileset.id)
            .or_try_insert_with(|| {
                crate::tiles::Atlas::new(render_state, filesystem, image_cache, tileset)
            })?
            .clone())
    }

    pub fn reload_atlas(
        &self,
        render_state: &egui_wgpu::RenderState,
        filesystem: &impl luminol_core::filesystem::FileSystem,
        image_cache: &crate::image_cache::Cache,
        tileset: &luminol_data::rpg::Tileset,
    ) -> Result<crate::tiles::Atlas, String> {
        Ok(self
            .atlases
            .entry(tileset.id)
            .insert(crate::tiles::Atlas::new(
                render_state,
                filesystem,
                image_cache,
                tileset,
            )?)
            .clone())
    }

    pub fn clear(&self) {
        self.atlases.clear()
    }
}
