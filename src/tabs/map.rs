// Copyright (C) 2022 Lily Lyons
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
#![allow(unused_imports)]
use crate::{fl, prelude::*};
use egui::Pos2;
use std::{cell::RefMut, collections::HashMap};

/// The map editor.
pub struct Tab {
    /// ID of the map that is being edited.
    pub id: i32,
    /// The tilemap.
    pub tilemap: Tilemap,

    dragged_event: usize,
    dragging_event: bool,
    event_windows: window::Windows,
    force_close: bool,
}

impl Tab {
    /// Create a new map editor.
    pub fn new(id: i32) -> Result<Self, String> {
        let map = state!().data_cache.map(id);
        Ok(Self {
            id,
            tilemap: Tilemap::new(id, &map)?,
            dragged_event: 0,
            dragging_event: false,
            event_windows: window::Windows::default(),
            force_close: false,
        })
    }
}

impl tab::Tab for Tab {
    fn name(&self) -> String {
        let mapinfos = state!().data_cache.mapinfos();
        fl!(
            "tab_map_title_label",
            id = self.id,
            name = mapinfos[&self.id].name.clone()
        )
    }

    fn id(&self) -> egui::Id {
        egui::Id::new("luminol_map").with(self.id)
    }

    fn force_close(&mut self) -> bool {
        self.force_close
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        let state = state!();

        // Get the map.
        let mut map = state.data_cache.map(self.id);

        // Display the toolbar.
        egui::TopBottomPanel::top(format!("map_{}_toolbar", self.id)).show_inside(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.tilemap.scale, 15.0..=300.)
                        .text(fl!("scale"))
                        .fixed_decimals(0),
                );

                ui.separator();

                ui.menu_button(
                    // Format the text based on what layer is selected.
                    match self.tilemap.selected_layer {
                        SelectedLayer::Events => format!("{} ⏷", fl!("events")),
                        SelectedLayer::Tiles(layer) => {
                            format!("{} ⏷", fl!("tab_map_layer_section_sv", num = layer))
                        }
                    },
                    |ui| {
                        // TODO: Add layer enable button
                        // Display all layers.
                        ui.columns(2, |columns| {
                            columns[1].visuals_mut().button_frame = true;
                            columns[0].label(
                                egui::RichText::new(fl!("tab_map_panorama_label")).underline(),
                            );
                            columns[1].checkbox(&mut self.tilemap.pano_enabled, "👁");

                            for (index, layer) in self.tilemap.enabled_layers.iter_mut().enumerate()
                            {
                                columns[0].selectable_value(
                                    &mut self.tilemap.selected_layer,
                                    SelectedLayer::Tiles(index),
                                    fl!("tab_map_layer_section_sv", num = (index + 1)),
                                );
                                columns[1].checkbox(layer, "👁");
                            }

                            // Display event layer.
                            columns[0].selectable_value(
                                &mut self.tilemap.selected_layer,
                                SelectedLayer::Events,
                                egui::RichText::new(fl!("events")).italics(),
                            );
                            columns[1].checkbox(&mut self.tilemap.event_enabled, "👁");

                            columns[0].label(egui::RichText::new("Fog").underline());
                            columns[1].checkbox(&mut self.tilemap.fog_enabled, "👁");
                        });
                    },
                );

                ui.separator();

                ui.checkbox(&mut self.tilemap.visible_display, fl!("tab_map_dva_cb"));
                ui.checkbox(&mut self.tilemap.move_preview, fl!("tab_map_pemr_cb"));

                /*
                if ui.button("Save map preview").clicked() {
                    self.tilemap.save_to_disk();
                }
                */

                if map.preview_move_route.is_some() && ui.button(fl!("tab_map_cmrp_btn")).clicked()
                {
                    map.preview_move_route = None;
                }
            });
        });

        // Display the tilepicker.
        egui::SidePanel::left(format!("map_{}_tilepicker", self.id))
            .default_width(256.)
            .show_inside(ui, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    self.tilemap.tilepicker(ui);
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let response = self.tilemap.ui(ui, &map, self.dragging_event);

                let layers_max = map.data.zsize();
                let map_x = self.tilemap.cursor_pos.x as i32;
                let map_y = self.tilemap.cursor_pos.y as i32;

                if ui.input(|i| {
                    i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)
                }) {
                    if let Some((id, _)) = map
                        .events
                        .iter()
                        .find(|(_, event)| event.x == map_x && event.y == map_y)
                    {
                        map.events.remove(id);
                    }
                }
            })
        });

        self.event_windows.update(ui.ctx());
    }

    fn requires_filesystem(&self) -> bool {
        true
    }
}
