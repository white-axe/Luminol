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

pub struct CommandView<'a> {
    id: egui::Id,
    commands: &'a mut Vec<luminol_data::rpg::EventCommand>,
}

impl<'a> CommandView<'a> {
    pub fn new(
        id_source: impl std::hash::Hash,
        commands: &'a mut Vec<luminol_data::rpg::EventCommand>,
    ) -> Self {
        Self {
            id: egui::Id::new(id_source),
            commands,
        }
    }

    fn get_ids_for_command(root_id: egui::Id, command_index: usize) -> (egui::Id, egui::Id) {
        let root_id = root_id.with(command_index);
        (root_id.with(0usize), root_id.with(1usize))
    }
}

impl egui::Widget for CommandView<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.push_id(self.id, |ui| {
            for (i, command) in self.commands.iter_mut().enumerate() {
                let (header_id, view_id) = Self::get_ids_for_command(self.id, i);

                let header = egui::collapsing_header::CollapsingState::load_with_default_open(
                    ui.ctx(),
                    header_id,
                    false,
                );

                let header_response = header.show_header(ui, |ui| {
                    ui.label(format!("{} Custom Command", command.code));
                });

                header_response.body(|ui| {
                    ui.add(Self::new(view_id, &mut command.child_commands));
                });
            }
        })
        .response
    }
}
