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

use super::UiExt;
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

enum Stripe<'a> {
    Borrowed(&'a mut bool),
    Owned(bool),
}

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

struct DescriptionWidthCallback<'a> {
    ui: &'a egui::Ui,
    name: &'a str,
}

impl DescriptionWidthCallback<'_> {
    /// Returns `true` if the description is too long to fit in the label for this event command
    /// editor, otherwise `false`.
    fn is_truncated(&self, description: &str) -> bool {
        self.ui.text_width(
            if description.is_empty() {
                self.name.into()
            } else {
                format!("{}: {}", self.name, description)
            },
            egui::FontSelection::Default,
        ) > self.ui.available_width()
    }

    /// Returns a short prefix of `description_chars` such that `is_truncated` returns `true`.
    fn exponential_search_chars(
        &self,
        description_chars: impl Iterator<Item = char> + Clone,
    ) -> String {
        let mut length_in_chars = 16;
        loop {
            let text: String = description_chars.clone().take(length_in_chars).collect();
            if text.chars().count() < length_in_chars || self.is_truncated(&text) {
                return text;
            }
            length_in_chars *= 2;
        }
    }

    /// Returns a short prefix of `description_segments` such that `is_truncated` returns `true`.
    fn exponential_search_segments<'a>(
        &self,
        description_segments: impl Iterator<Item = &'a str> + Clone,
    ) -> String {
        self.exponential_search_chars(description_segments.flat_map(|segment| segment.chars()))
    }

    /// Returns a short prefix of `description` such that `is_truncated` returns `true`.
    fn exponential_search_str(&self, description: &str) -> String {
        self.exponential_search_chars(description.chars())
    }
}

trait EventCommandEditor
where
    Self: Sync + 'static,
{
    /// Returns whether or not the UI for this event command editor should be expanded by default.
    ///
    /// The default is to not expand by default.
    fn expand_by_default(&self) -> bool {
        false
    }

    /// Returns the name of the event command that this event command editor edits.
    fn name(&self) -> &'static str;

    /// Returns a description of the given event command.
    ///
    /// By default, there is no description.
    ///
    /// If the description can be long, for optimization purposes, `callback` can be used to
    /// determine whether or not the description is short enough to fit in the UI widget where the
    /// description will be displayed.
    #[allow(unused_variables)]
    fn description(
        &self,
        callback: DescriptionWidthCallback<'_>,
        update_state: &mut UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> String {
        String::new()
    }

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
        stripe: &mut bool,
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
mod c121;
mod c122;
mod c123;
mod c124;
mod c125;
mod c126;
mod c127;
mod c128;
mod c129;

static EDITORS: phf::Map<u16, &dyn EventCommandEditor> = phf::phf_map! {
    101u16 => &c101::Editor { continuation_code: 401, name: "Show Text" },
    102u16 => &c102::Editor,
    103u16 => &c103::Editor,
    104u16 => &c104::Editor,
    105u16 => &c105::Editor,
    106u16 => &c106::Editor,
    108u16 => &c101::Editor { continuation_code: 408, name: "Comment" },
    111u16 => &c111::Editor,
    112u16 => &c112::Editor,
    113u16 => &c113::Editor,
    115u16 => &c115::Editor,
    116u16 => &c116::Editor,
    117u16 => &c117::Editor,
    118u16 => &c118::Editor,
    119u16 => &c119::Editor,
    121u16 => &c121::Editor,
    122u16 => &c122::Editor,
    123u16 => &c123::Editor,
    124u16 => &c124::Editor,
    125u16 => &c125::Editor,
    126u16 => &c126::Editor,
    127u16 => &c127::Editor,
    128u16 => &c128::Editor,
    129u16 => &c129::Editor,
    355u16 => &c101::Editor { continuation_code: 655, name: "Script" },
};

fn show_parameter_label(ui: &mut egui::Ui, index: usize, type_name: &str) {
    let index = index + 1;
    ui.label(format!("Parameter {index} ({type_name})"));
}

struct Collapsing<'a> {
    ui: &'a mut egui::Ui,
    id: egui::Id,
    layout: egui::Layout,
    expand_by_default: bool,
}

impl<'a> Collapsing<'a> {
    fn new(ui: &'a mut egui::Ui) -> Self {
        Self {
            layout: *ui.layout(),
            id: ui.id(),
            ui,
            expand_by_default: true,
        }
    }

    fn id(mut self, id: egui::Id) -> Self {
        self.id = id;
        self
    }

    fn id_salt(mut self, id_salt: impl std::hash::Hash) -> Self {
        self.id = self.id.with(id_salt);
        self
    }

    fn expand_by_default(mut self, expand_by_default: bool) -> Self {
        self.expand_by_default = expand_by_default;
        self
    }

    fn show_header<R>(
        self,
        f: impl FnOnce(&mut egui::Ui) -> R,
    ) -> egui::collapsing_header::HeaderResponse<'a, R> {
        egui::collapsing_header::CollapsingState::load_with_default_open(
            self.ui.ctx(),
            self.id,
            self.expand_by_default,
        )
        .show_header(self.ui, |ui| {
            ui.with_layout(
                egui::Layout {
                    main_dir: egui::Direction::LeftToRight,
                    main_wrap: false,
                    main_align: egui::Align::Min,
                    main_justify: self.layout.cross_justify,
                    cross_align: egui::Align::Center,
                    cross_justify: self.layout.main_justify,
                },
                |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                    f(ui)
                },
            )
            .inner
        })
    }
}

/// Returns whether or not at least one parameter was modified.
fn show_parameters<'a>(
    ui: &mut egui::Ui,
    parameters: impl Iterator<Item = &'a mut ParameterType>,
) -> bool {
    let mut modified = false;

    for (i, parameter) in parameters.enumerate() {
        match parameter {
            ParameterType::Array(value) => {
                show_parameter_label(ui, i, "array");
                Collapsing::new(ui)
                    .id_salt((i, "array"))
                    .expand_by_default(false)
                    .show_header(|ui| {
                        ui.label("Contents");
                    })
                    .body(|ui| {
                        modified |= show_parameters(ui, value.iter_mut());
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
            ParameterType::AudioFile(_value) => {
                show_parameter_label(ui, i, "audio file");
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
                    ui.with_stripe_mut(stripe, |ui, stripe| {
                        let maybe_editor = command
                            .matches_schema
                            .then(|| EDITORS.get(&command.code))
                            .flatten();

                        Collapsing::new(ui)
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
                                    let description = editor.description(
                                        DescriptionWidthCallback { ui, name },
                                        self.update_state,
                                        self.event_info,
                                        command,
                                    );
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
                                if let Some(editor) = maybe_editor {
                                    ui.push_id(command.code, |ui| {
                                        modified |= editor
                                            .ui(
                                                ui,
                                                stripe,
                                                self.update_state,
                                                self.event_info,
                                                command,
                                            )
                                            .changed();
                                    });
                                } else {
                                    Collapsing::new(ui)
                                        .id_salt("parameters")
                                        .expand_by_default(command.child_commands.is_empty())
                                        .show_header(|ui| {
                                            ui.label("Parameters");
                                        })
                                        .body(|ui| {
                                            modified |=
                                                show_parameters(ui, command.parameters.iter_mut());
                                        });
                                    Collapsing::new(ui)
                                        .id_salt("child commands")
                                        .show_header(|ui| {
                                            ui.label("Child commands");
                                        })
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
                            });
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
