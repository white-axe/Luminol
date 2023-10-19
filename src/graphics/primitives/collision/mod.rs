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
use crate::prelude::*;

use instance::Instances;
use shader::Shader;
use vertex::Vertex;

mod instance;
mod shader;
mod vertex;

#[derive(Debug)]
pub struct Collision {
    pub instances: Instances,
    pub use_push_constants: bool,
}

impl Collision {
    pub fn new(passages: &Table2, use_push_constants: bool) -> Self {
        let instances = Instances::new(passages);

        Self {
            instances,
            use_push_constants,
        }
    }

    pub fn set_passage(&self, passage: i16, position: (usize, usize)) {
        self.instances.set_passage(passage, position)
    }

    pub fn draw<'rpass>(
        &'rpass self,
        viewport: &primitives::Viewport,
        render_pass: &mut wgpu::RenderPass<'rpass>,
    ) {
        #[repr(C)]
        #[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
        struct VertexPushConstant {
            viewport: [u8; 64],
        }

        render_pass.push_debug_group("tilemap collision renderer");
        Shader::bind(self.use_push_constants, render_pass);
        if self.use_push_constants {
            render_pass.set_push_constants(
                wgpu::ShaderStages::VERTEX,
                0,
                bytemuck::bytes_of(&VertexPushConstant {
                    viewport: viewport.as_bytes(),
                }),
            );
        }
        self.instances.draw(render_pass);
        render_pass.pop_debug_group();
    }
}
