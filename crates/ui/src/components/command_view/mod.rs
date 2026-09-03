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

impl egui::Widget for CommandView<'_> {
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

                    let header_response = header.show_header(ui, |ui| {
                        ui.label(format!("{} Custom Command", command.code));
                    });

                    header_response.body(|ui| {
                        modified |= show_parameters(ui, command.parameters.iter_mut());
                        modified |= ui.add(Self::new(&mut command.child_commands)).changed();
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
