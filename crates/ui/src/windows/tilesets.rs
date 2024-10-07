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

use luminol_core::Modal;

use crate::components::{DatabaseView, Field, Tilepicker, UiExt};
use crate::modals::graphic_picker::tileset::Modal as TilesetModal;

/// Database - Tilesets management window.
pub struct Window {
    selected_tileset_name: Option<String>,

    previous_tileset: Option<usize>,

    tileset_modal: TilesetModal,

    tilepicker: Option<Tilepicker>,
    view: DatabaseView,
}

impl Window {
    pub fn new(update_state: &luminol_core::UpdateState<'_>) -> Self {
        let tilesets = update_state.data.tilesets();
        let tileset = &tilesets.data[0];
        Self {
            selected_tileset_name: None,
            previous_tileset: None,
            tilepicker: None,
            tileset_modal: TilesetModal::new(tileset, "tileset_graphic_picker".into()),
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
                        let mut needs_update = self.previous_tileset != Some(tileset.id);

                        ui.with_padded_stripe(false, |ui| {
                            modified |= ui
                                .add(Field::new(
                                    "Name",
                                    egui::TextEdit::singleline(&mut tileset.name)
                                        .desired_width(f32::INFINITY),
                                ))
                                .changed();
                        });

                        ui.with_padded_stripe(true, |ui| {
                            let changed = ui
                                .add(Field::new(
                                    "Graphic",
                                    self.tileset_modal.button(tileset, update_state),
                                ))
                                .changed();
                            if changed {
                                modified = true;
                                needs_update = true;
                            }
                        });

                        if needs_update {
                            self.tileset_modal.reset(update_state, tileset);
                            self.tilepicker = Some(Tilepicker::new(
                                update_state,
                                tileset.tileset_name.as_deref(),
                                &tileset.autotile_names,
                                &tileset.passages,
                                None,
                            ));
                        }

                        egui::ScrollArea::both().show_viewport(ui, |ui, scroll_rect| {
                            self.tilepicker
                                .as_mut()
                                .unwrap()
                                .ui(update_state, ui, scroll_rect);
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
