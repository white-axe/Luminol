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

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self, _command: &EventCommand) -> String {
        "Show Choices".into()
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        update_state: &mut UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                let terminator = command.sibling_commands.pop().unwrap();

                let mut choice_map = std::collections::HashMap::new();
                for (sibling_index, sibling) in command.sibling_commands.iter().enumerate() {
                    match sibling.code {
                        402 => {
                            choice_map.insert(
                                match sibling.parameters.first().unwrap() {
                                    ParameterType::Integer(choice_index) => Some(*choice_index),
                                    _ => None,
                                }
                                .unwrap(),
                                sibling_index,
                            );
                        }
                        403 => {
                            choice_map.insert(4, sibling_index);
                        }
                        _ => {}
                    }
                }

                let choices = match command.parameters.first_mut().unwrap() {
                    ParameterType::Array(choices) => Some(choices),
                    _ => None,
                }
                .unwrap();
                let num_choices = choices.len();

                for (choice_index, choice) in choices.iter_mut().enumerate() {
                    let choice = match choice {
                        ParameterType::String(choice) => Some(choice),
                        _ => None,
                    }
                    .unwrap();

                    ui.label(format!("Choice {}", choice_index + 1));

                    modified |= ui.text_edit_singleline(choice).changed();

                    let mut choice_commands_fallback = Vec::new();
                    let choice_commands = choice_map.get(&(choice_index as _)).map_or(
                        &mut choice_commands_fallback,
                        |&sibling_index| {
                            &mut command.sibling_commands[sibling_index].child_commands
                        },
                    );
                    modified |= ui
                        .add(super::CommandView::new(
                            update_state,
                            event_info,
                            choice_commands,
                        ))
                        .changed();
                }

                let cancel_type = match command.parameters[1] {
                    ParameterType::Integer(cancel_type) => Some(cancel_type),
                    _ => None,
                }
                .unwrap();
                if cancel_type > 0 && (cancel_type - 1) as usize >= num_choices {
                    ui.label("Cancel");

                    let mut cancel_commands_fallback = Vec::new();
                    let cancel_commands = choice_map.get(&(cancel_type - 1)).map_or(
                        &mut cancel_commands_fallback,
                        |&sibling_index| {
                            &mut command.sibling_commands[sibling_index].child_commands
                        },
                    );
                    modified |= ui
                        .add(super::CommandView::new(
                            update_state,
                            event_info,
                            cancel_commands,
                        ))
                        .changed();
                }

                command.sibling_commands.push(terminator);
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
