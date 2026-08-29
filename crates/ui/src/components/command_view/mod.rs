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
    commands: &'a mut Vec<luminol_data::rpg::EventCommand>,
}

impl<'a> CommandView<'a> {
    pub fn new(commands: &'a mut Vec<luminol_data::rpg::EventCommand>) -> Self {
        Self { commands }
    }
}

impl egui::Widget for CommandView<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::new()
            .show(ui, |ui| {
                for command in self.commands {
                    let header = egui::collapsing_header::CollapsingState::load_with_default_open(
                        ui.ctx(),
                        egui::Id::new("luminol_command_view").with(&command.guid),
                        !command.child_commands.is_empty(),
                    );

                    let header_response = header.show_header(ui, |ui| {
                        ui.label(format!("{} Custom Command", command.code));
                    });

                    header_response.body(|ui| {
                        ui.add(Self::new(&mut command.child_commands));
                    });
                }
            })
            .response
    }
}
