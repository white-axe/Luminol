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

use super::{EventCommand, EventCommandEditor, EventInfo, ParameterType, UpdateState};
use itertools::Itertools;

pub(super) struct Editor {
    pub continuation_code: u16,
    pub label: &'static str,
}

impl EventCommandEditor for Editor {
    fn name(&self, _command: &EventCommand) -> String {
        self.label.into()
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        _stripe: &mut bool,
        _update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let text = command.state.get_or_insert_with(|| {
            std::iter::once(command.parameters[0].as_string().unwrap())
                .chain(
                    command
                        .sibling_commands
                        .iter()
                        .map(|sibling| sibling.parameters[0].as_string().unwrap()),
                )
                .join("\n")
        });

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                modified |= ui.text_edit_multiline(text).changed();
            })
            .response;

        if modified {
            for (i, line) in text.split('\n').enumerate() {
                line.clone_into(if i == 0 {
                    command.parameters[0].as_string_mut().unwrap()
                } else {
                    let sibling = if let Some(sibling) = command.sibling_commands.get_mut(i - 1) {
                        sibling
                    } else {
                        command.sibling_commands.push(Default::default());
                        let sibling = command.sibling_commands.last_mut().unwrap();
                        sibling.code = self.continuation_code;
                        sibling
                            .parameters
                            .push(ParameterType::String(Default::default()));
                        sibling
                    };
                    sibling.parameters[0].as_string_mut().unwrap()
                });
            }

            command
                .sibling_commands
                .truncate(text.matches('\n').count());
        }

        if modified {
            response.mark_changed();
        }
        response
    }
}
