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

use crate::components::{DatabaseView, Field, Tilepicker, UiExt};

/// Database - Tilesets management window.
pub struct Window {
    selected_tileset_name: Option<String>,

    previous_tileset: Option<usize>,

    tilepicker: Option<Tilepicker>,
    view: DatabaseView,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            selected_tileset_name: None,
            previous_tileset: None,
            tilepicker: None,
            view: DatabaseView::new(),
        }
    }
}

impl luminol_core::Window for Window {
    fn id(&self) -> egui::Id {
        egui::Id::new("tileset_editor")
    }

    fn requires_filesystem(&self) -> bool {
        true
    }

    fn show(
        &mut self,
        ctx: &egui::Context,
        open: &mut bool,
        update_state: &mut luminol_core::UpdateState<'_>,
    ) {
        let data = std::mem::take(update_state.data); // take data to avoid borrow checker issues
        let mut tilesets = data.tilesets();

        let mut modified = false;

        self.selected_tileset_name = None;

        let name = if let Some(name) = &self.selected_tileset_name {
            format!("Editing tileset {:?}", name)
        } else {
            "Tileset Editor".into()
        };

        let response = egui::Window::new(name)
            .id(self.id())
            .default_width(500.)
            .open(open)
            .show(ctx, |ui| {
                self.view.show(
                    ui,
                    update_state,
                    "Tilesets",
                    &mut tilesets.data,
                    |tileset| format!("{:0>4}: {}", tileset.id + 1, tileset.name),
                    |ui, tilesets, id, update_state| {
                        let tileset = &mut tilesets[id];
                        self.selected_tileset_name = Some(tileset.name.clone());

                        if self.previous_tileset != Some(id) {
                            self.tilepicker = Some(Tilepicker::new(update_state, tileset, None));
                        }
                        let tilepicker = self.tilepicker.as_mut().unwrap();

                        ui.with_padded_stripe(false, |ui| {
                            modified |= ui
                                .add(Field::new(
                                    "Name",
                                    egui::TextEdit::singleline(&mut tileset.name)
                                        .desired_width(f32::INFINITY),
                                ))
                                .changed();
                        });

                        egui::ScrollArea::both().show_viewport(ui, |ui, scroll_rect| {
                            tilepicker.ui(update_state, ui, scroll_rect);
                        });

                        self.previous_tileset = Some(tileset.id);
                    },
                )
            });

        if response.is_some_and(|ir| ir.inner.is_some_and(|ir| ir.inner.modified)) {
            modified = true;
        }

        if modified {
            update_state.modified.set(true);
            tilesets.modified = true;
        }

        drop(tilesets);

        *update_state.data = data; // restore data
    }
}
