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

use super::{EventCommand, EventCommandEditor, EventCommandEditorState, ParameterType};
use itertools::Itertools;

pub(super) struct Editor {
    pub continuation_code: u16,
    pub label: &'static str,
}

impl EventCommandEditor for Editor {
    fn name(&'static self, _command: &EventCommand) -> String {
        self.label.into()
    }

    fn ui(
        &'static self,
        ui: &mut egui::Ui,
        state: EventCommandEditorState<'_>,
        command: &mut EventCommand,
    ) -> egui::Response {
        state.with(
            command,
            |command| {
                std::iter::once(
                    match &command.parameters[0] {
                        ParameterType::String(parameter) => Some(parameter),
                        _ => None,
                    }
                    .unwrap(),
                )
                .chain(command.sibling_commands.iter().map(|sibling| {
                    match &sibling.parameters[0] {
                        ParameterType::String(parameter) => Some(parameter),
                        _ => None,
                    }
                    .unwrap()
                }))
                .join("\n")
            },
            |command, state| {
                let mut modified = false;

                let mut response = egui::Frame::NONE
                    .show(ui, |ui| {
                        modified |= ui.text_edit_multiline(state).changed();

                        if modified {
                            for (i, line) in state.split('\n').enumerate() {
                                line.clone_into(if i == 0 {
                                    match &mut command.parameters[0] {
                                        ParameterType::String(parameter) => Some(parameter),
                                        _ => None,
                                    }
                                    .unwrap()
                                } else {
                                    let sibling = if let Some(sibling) =
                                        command.sibling_commands.get_mut(i - 1)
                                    {
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
                                    match &mut sibling.parameters[0] {
                                        ParameterType::String(parameter) => Some(parameter),
                                        _ => None,
                                    }
                                    .unwrap()
                                });
                            }

                            command
                                .sibling_commands
                                .truncate(state.matches('\n').count());
                        }
                    })
                    .response;

                if modified {
                    response.mark_changed();
                }
                response
            },
        )
    }
}
