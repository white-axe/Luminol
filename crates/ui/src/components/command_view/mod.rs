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

pub mod editors;
use editors::EDITORS;

pub mod collapsing;
use collapsing::Collapsing;

pub mod description_width_callback;
use description_width_callback::DescriptionWidthCallback;

pub mod parameter_ref;
use parameter_ref::{
    AudioFileParameterMut, AudioFileParameterRef, IntegerParameterMut, IntegerParameterRef,
    StringParameterMut,
};

pub mod audio_selection;
pub mod character_selection;
pub mod database_selection;
pub mod graphic_selection;
pub mod value_selection;
use database_selection::VariableSelection;

use super::UiExt;
use crate::UpdateState;
use luminol_data::{
    rpg::{AudioFile, AudioFileRef, EventCommand},
    ParameterType,
};

#[derive(Debug, Clone, Copy)]
pub struct EventInfo<'a> {
    /// The ID of the map in which the event is located.
    pub map_id: usize,
    /// The ID of the event within the current map.
    pub event_id: usize,
    /// The name of the event.
    pub event_name: &'a str,
}

enum Stripe<'a> {
    Borrowed(&'a mut bool),
    Owned(bool),
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct CommandView<'this, 'update_state> {
    stripe: Stripe<'this>,
    update_state: &'this mut UpdateState<'update_state>,
    event_info: Option<&'this EventInfo<'this>>,
    commands: &'this mut Vec<EventCommand>,
}

impl<'this, 'update_state> CommandView<'this, 'update_state> {
    pub fn new(
        update_state: &'this mut UpdateState<'update_state>,
        event_info: Option<&'this EventInfo<'this>>,
        commands: &'this mut Vec<EventCommand>,
    ) -> Self {
        Self {
            stripe: Stripe::Owned(false),
            update_state,
            event_info,
            commands,
        }
    }

    fn with_stripe(mut self, stripe: &'this mut bool) -> Self {
        self.stripe = Stripe::Borrowed(stripe);
        self
    }
}

#[cfg(not(debug_assertions))]
fn assert_command_matches_schema(_command: &mut EventCommand) {}

#[cfg(debug_assertions)]
fn assert_command_matches_schema(command: &mut EventCommand) {
    if command.matches_schema && EDITORS.contains_key(&command.code) {
        let schema = *luminol_data::rpg::event::SCHEMAS
            .get(&command.code)
            .unwrap();
        for sibling in std::mem::take(&mut command.sibling_commands) {
            assert!(schema.is_sibling(command, &sibling));
            command.sibling_commands.push(sibling);
        }
        assert!(schema.matches(command));
    }
}

fn show_parameter_label(ui: &mut egui::Ui, index: usize, type_name: &str) {
    let index = index + 1;
    ui.label(format!("Parameter {index} ({type_name})"));
}

/// Returns whether or not at least one parameter was modified.
fn show_parameters<'a>(
    ui: &mut egui::Ui,
    stripe: &mut bool,
    parameters: impl Iterator<Item = &'a mut ParameterType>,
) -> bool {
    let mut modified = false;

    for (i, parameter) in parameters.enumerate() {
        match parameter {
            ParameterType::Array(value) => {
                show_parameter_label(ui, i, "array");
                Collapsing::new(ui, stripe)
                    .id_salt((i, "array"))
                    .expand_by_default(false)
                    .show_header_text("Contents")
                    .body(|ui| {
                        modified |= show_parameters(ui, stripe, value.iter_mut());
                    });
            }
            ParameterType::None => {
                show_parameter_label(ui, i, "nil");
            }
            ParameterType::Bool(value) => {
                show_parameter_label(ui, i, "boolean");
                modified |= ui.checkbox(value, ()).changed();
            }
            ParameterType::Integer(value) => {
                show_parameter_label(ui, i, "integer");
                modified |= ui.add(egui::DragValue::new(value)).changed();
            }
            ParameterType::Float(value) => {
                show_parameter_label(ui, i, "float");
                modified |= ui.add(egui::DragValue::new(value)).changed();
            }
            ParameterType::String(value) => {
                show_parameter_label(ui, i, "string");
                modified |= ui.text_edit_multiline(value).changed();
            }
            ParameterType::Symbol(value) => {
                show_parameter_label(ui, i, "symbol");
                modified |= ui.text_edit_multiline(value).changed();
            }
            ParameterType::Color(value) => {
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
            ParameterType::Tone(_value) => {
                show_parameter_label(ui, i, "tone");
            }
            ParameterType::AudioFile(value) => {
                show_parameter_label(ui, i, "audio file");
                modified |= ui
                    .add(audio_selection::AudioSelection::new(value).prepare_raw("audio"))
                    .changed();
            }
            ParameterType::MoveRoute(_value) => {
                show_parameter_label(ui, i, "move route");
            }
            ParameterType::MoveCommand(_value) => {
                show_parameter_label(ui, i, "move command");
            }
        }
    }

    modified
}

impl egui::Widget for CommandView<'_, '_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let stripe = match &mut self.stripe {
            Stripe::Borrowed(reference) => reference,
            Stripe::Owned(value) => value,
        };

        let mut modified = false;

        let mut response = ui
            .with_cross_justify(|ui| {
                for command in self.commands {
                    let maybe_editor = command
                        .matches_schema
                        .then(|| EDITORS.get(&command.code))
                        .flatten();

                    Collapsing::new(ui, stripe)
                        .id(egui::Id::new("luminol_command_view").with(&command.guid))
                        .expand_by_default(if let Some(editor) = maybe_editor {
                            editor.expand_by_default()
                        } else {
                            !command.child_commands.is_empty()
                        })
                        .show_header(|ui| {
                            if let Some(editor) = maybe_editor {
                                let code = command.code;
                                let name = editor.name();
                                let description = editor
                                    .description(
                                        DescriptionWidthCallback::new(ui, name),
                                        self.update_state,
                                        self.event_info,
                                        command,
                                    )
                                    .unwrap_or_default();
                                if description.is_empty() {
                                    ui.label(format!("{code} {name}"));
                                } else {
                                    ui.label(format!("{code} {name}: {description}"));
                                }
                            } else {
                                ui.label(format!("{} Custom Command", command.code));
                            }
                        })
                        .body(|ui| {
                            assert_command_matches_schema(command);
                            if let Some(editor) = maybe_editor {
                                ui.push_id(command.code, |ui| {
                                    modified |= editor
                                        .ui(ui, stripe, self.update_state, self.event_info, command)
                                        .changed();
                                });
                            } else {
                                Collapsing::new(ui, stripe)
                                    .id_salt("parameters")
                                    .expand_by_default(command.child_commands.is_empty())
                                    .show_header_text("Parameters")
                                    .body(|ui| {
                                        modified |= show_parameters(
                                            ui,
                                            stripe,
                                            command.parameters.iter_mut(),
                                        );
                                    });
                                Collapsing::new(ui, stripe)
                                    .id_salt("child commands")
                                    .show_header_text("Child commands")
                                    .body(|ui| {
                                        modified |= ui
                                            .add(
                                                CommandView::new(
                                                    self.update_state,
                                                    self.event_info,
                                                    &mut command.child_commands,
                                                )
                                                .with_stripe(stripe),
                                            )
                                            .changed();
                                    });
                            }
                            assert_command_matches_schema(command);
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
