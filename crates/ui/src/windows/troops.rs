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

use crate::components::{DatabaseView, Field, UiExt};

/// Database - Troops management window.
pub struct Window {
    selected_troop_name: Option<String>,

    previous_troop: Option<usize>,

    troop_view: crate::components::TroopView,
    view: DatabaseView,
}

impl Window {
    pub fn new(update_state: &luminol_core::UpdateState<'_>) -> Self {
        Self {
            selected_troop_name: None,
            previous_troop: None,
            troop_view: crate::components::TroopView::new(update_state),
            view: DatabaseView::new(),
        }
    }
}

impl luminol_core::Window for Window {
    fn id(&self) -> egui::Id {
        egui::Id::new("troop_editor")
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
        let mut troops = data.troops();
        let enemies = data.enemies();

        let mut modified = false;

        self.selected_troop_name = None;

        let name = if let Some(name) = &self.selected_troop_name {
            format!("Editing troop {:?}", name)
        } else {
            "Troop Editor".into()
        };

        let response = egui::Window::new(name)
            .id(self.id())
            .default_width(720.)
            .open(open)
            .show(ctx, |ui| {
                self.view.show(
                    ui,
                    update_state,
                    "Troops",
                    &mut troops.data,
                    |troop| format!("{:0>4}: {}", troop.id + 1, troop.name),
                    |ui, troops, id, update_state| {
                        let troop = &mut troops[id];
                        self.selected_troop_name = Some(troop.name.clone());

                        let clip_rect = ui.clip_rect();

                        ui.with_padded_stripe(false, |ui| {
                            modified |= ui
                                .add(Field::new(
                                    "Name",
                                    egui::TextEdit::singleline(&mut troop.name)
                                        .desired_width(f32::INFINITY),
                                ))
                                .changed();
                        });

                        ui.with_padded_stripe(true, |ui| {
                            let canvas_rect = egui::Resize::default()
                                .resizable([false, true])
                                .min_width(ui.available_width())
                                .max_width(ui.available_width())
                                .default_height(240.)
                                .show(ui, |ui| {
                                    egui::Frame::dark_canvas(ui.style())
                                        .show(ui, |ui| {
                                            let (_, rect) = ui.allocate_space(ui.available_size());
                                            rect
                                        })
                                        .inner
                                });

                            if self.previous_troop != Some(troop.id) {
                                self.troop_view.troop.rebuild_members(
                                    &update_state.graphics,
                                    update_state.filesystem,
                                    &enemies,
                                    troop,
                                );
                            }

                            ui.allocate_ui_at_rect(canvas_rect, |ui| {
                                self.troop_view.ui(ui, update_state, clip_rect);
                            });
                        });

                        self.previous_troop = Some(troop.id);
                    },
                )
            });

        if response.is_some_and(|ir| ir.inner.is_some_and(|ir| ir.inner.modified)) {
            modified = true;
        }

        if modified {
            update_state.modified.set(true);
            troops.modified = true;
        }

        drop(troops);
        drop(enemies);

        *update_state.data = data; // restore data
    }
}
