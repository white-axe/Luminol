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

use crate::UpdateState;
use luminol_data::{rpg::EventCommand, ParameterType};

#[derive(Debug, Clone, Copy)]
pub struct EventInfo<'a> {
    /// The ID of the map in which the event is located.
    pub map_id: usize,
    /// The ID of the event within the current map.
    pub event_id: usize,
    /// The name of the event.
    pub event_name: &'a str,
}

pub struct CommandView<'this, 'update_state, 'event_info> {
    update_state: &'this mut UpdateState<'update_state>,
    event_info: Option<&'this EventInfo<'event_info>>,
    commands: &'this mut Vec<EventCommand>,
}

impl<'this, 'update_state, 'event_info> CommandView<'this, 'update_state, 'event_info> {
    pub fn new(
        update_state: &'this mut UpdateState<'update_state>,
        event_info: Option<&'this EventInfo<'event_info>>,
        commands: &'this mut Vec<EventCommand>,
    ) -> Self {
        Self {
            update_state,
            event_info,
            commands,
        }
    }
}

trait EventCommandEditor
where
    Self: Sync + 'static,
{
    /// Returns a descriptive name for the given event command.
    fn name(&self, command: &EventCommand) -> String;

    /// Renders the UI for this event command editor.
    ///
    /// Remember to mark the response returned by this method as changed if the event command was
    /// modified by this editor (by calling the `mark_changed` method of the response).
    ///
    /// The event command is guaranteed to match the schema for the command's event code (i.e. the
    /// `matches_schema` field on the command will be `true`). If there is no schema for the
    /// command's command code, this will never be called.
    fn ui(
        &self,
        ui: &mut egui::Ui,
        update_state: &mut UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response;
}

mod c101;
mod c102;
mod c103;
mod c104;
mod c105;
mod c106;
mod c111;
mod c112;
mod c113;
mod c115;
mod c116;
mod c117;
mod c118;
mod c119;

static EDITORS: phf::Map<u16, &dyn EventCommandEditor> = phf::phf_map! {
    101u16 => &c101::Editor { continuation_code: 401, label: "Show Text" },
    102u16 => &c102::Editor,
    103u16 => &c103::Editor,
    104u16 => &c104::Editor,
    105u16 => &c105::Editor,
    106u16 => &c106::Editor,
    108u16 => &c101::Editor { continuation_code: 408, label: "Comment" },
    111u16 => &c111::Editor,
    112u16 => &c112::Editor,
    113u16 => &c113::Editor,
    115u16 => &c115::Editor,
    116u16 => &c116::Editor,
    117u16 => &c117::Editor,
    118u16 => &c118::Editor,
    119u16 => &c119::Editor,
    355u16 => &c101::Editor { continuation_code: 655, label: "Script" },
};

fn show_parameter_label(ui: &mut egui::Ui, index: usize, type_name: &str) {
    let index = index + 1;
    ui.label(format!("Parameter {index} ({type_name})"));
}

/// Returns whether or not at least one parameter was modified.
fn show_parameters<'a>(
    ui: &mut egui::Ui,
    parameters: impl Iterator<Item = &'a mut luminol_data::ParameterType>,
) -> bool {
    let mut modified = false;

    for (i, parameter) in parameters.enumerate() {
        match parameter {
            luminol_data::ParameterType::Array(value) => {
                show_parameter_label(ui, i, "array");
                let header = egui::collapsing_header::CollapsingState::load_with_default_open(
                    ui.ctx(),
                    ui.id().with((i, "array")),
                    false,
                );
                let header_response = header.show_header(ui, |ui| {
                    ui.label("Contents");
                });
                header_response.body(|ui| {
                    modified |= show_parameters(ui, value.iter_mut());
                });
            }
            luminol_data::ParameterType::None => {
                show_parameter_label(ui, i, "nil");
            }
            luminol_data::ParameterType::Bool(value) => {
                show_parameter_label(ui, i, "boolean");
                modified |= ui.checkbox(value, ()).changed();
            }
            luminol_data::ParameterType::Integer(value) => {
                show_parameter_label(ui, i, "integer");
                modified |= ui.add(egui::DragValue::new(value)).changed();
            }
            luminol_data::ParameterType::Float(value) => {
                show_parameter_label(ui, i, "float");
                modified |= ui.add(egui::DragValue::new(value)).changed();
            }
            luminol_data::ParameterType::String(value) => {
                show_parameter_label(ui, i, "string");
                modified |= ui.text_edit_multiline(value).changed();
            }
            luminol_data::ParameterType::Symbol(value) => {
                show_parameter_label(ui, i, "symbol");
                modified |= ui.text_edit_multiline(value).changed();
            }
            luminol_data::ParameterType::Color(value) => {
                show_parameter_label(ui, i, "color");
                let mut color = [
                    value.red.clamp(0., 255.) as u8,
                    value.green.clamp(0., 255.) as u8,
                    value.blue as u8,
                    value.alpha as u8,
                ];
                if ui
                    .color_edit_button_srgba_unmultiplied(&mut color)
                    .changed()
                {
                    modified = true;
                    (value.red, value.green, value.blue, value.alpha) = (
                        color[0] as f64,
                        color[1] as f64,
                        color[2] as f64,
                        color[3] as f64,
                    );
                }
            }
            luminol_data::ParameterType::Tone(_value) => {
                show_parameter_label(ui, i, "tone");
            }
            luminol_data::ParameterType::AudioFile(_value) => {
                show_parameter_label(ui, i, "audio file");
            }
            luminol_data::ParameterType::MoveRoute(_value) => {
                show_parameter_label(ui, i, "move route");
            }
            luminol_data::ParameterType::MoveCommand(_value) => {
                show_parameter_label(ui, i, "move command");
            }
        }
    }

    modified
}

impl egui::Widget for CommandView<'_, '_, '_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::new()
            .show(ui, |ui| {
                for command in self.commands {
                    let header = egui::collapsing_header::CollapsingState::load_with_default_open(
                        ui.ctx(),
                        egui::Id::new("luminol_command_view").with(&command.guid),
                        !command.child_commands.is_empty(),
                    );

                    let maybe_editor = command
                        .matches_schema
                        .then(|| EDITORS.get(&command.code))
                        .flatten();

                    let header_response = header.show_header(ui, |ui| {
                        if let Some(editor) = maybe_editor {
                            ui.label(format!("{} {}", command.code, editor.name(command)));
                        } else {
                            ui.label(format!("{} Custom Command", command.code));
                        }
                    });

                    header_response.body(|ui| {
                        if let Some(editor) = maybe_editor {
                            ui.push_id(command.code, |ui| {
                                modified |= editor
                                    .ui(ui, self.update_state, self.event_info, command)
                                    .changed();
                            });
                        } else {
                            modified |= show_parameters(ui, command.parameters.iter_mut());
                            modified |= ui
                                .add(CommandView::new(
                                    self.update_state,
                                    self.event_info,
                                    &mut command.child_commands,
                                ))
                                .changed();
                        }
                    });
                }
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
