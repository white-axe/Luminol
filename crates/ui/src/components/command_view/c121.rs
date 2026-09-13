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

use super::{
    DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo, SwitchSelection,
    UpdateState,
};
use crate::components::EnumComboBox;
use std::marker::PhantomData;

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum Operation {
    #[strum(to_string = "Set switch to on")]
    On = 0,
    #[strum(to_string = "Set switch to off")]
    Off = 1,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self) -> &'static str {
        "Control Switches"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> String {
        let start_id = command.parameters[0].as_integer().unwrap();
        let end_id = command.parameters[1].as_integer().unwrap();
        let value = match command.parameters[2].as_integer().unwrap() {
            0 => "on",
            1 => "off",
            _ => return String::new(),
        };
        let Some(start) = SwitchSelection::new(update_state, start_id).fmt() else {
            return String::new();
        };
        if start_id == end_id {
            format!("Set [{start}] to {value}")
        } else {
            let Some(end) = SwitchSelection::new(update_state, end_id).fmt() else {
                return String::new();
            };
            format!("Set [{start}] - [{end}] to {value}")
        }
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        _stripe: &mut bool,
        update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let is_batch_edit = command.state.get_or_insert(false);

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                let [start, end] = command.parameters.first_chunk_mut().unwrap();
                let mut start = start.as_integer_mut().unwrap();
                let mut end = end.as_integer_mut().unwrap();

                if start != end {
                    *is_batch_edit = true;
                }
                modified |= {
                    let changed = ui.checkbox(is_batch_edit, "Batch edit").changed();
                    let changed = changed && !*is_batch_edit && start != end;
                    if changed {
                        *end = *start;
                    }
                    changed
                };

                if !*is_batch_edit {
                    ui.label("Switch");
                    modified |= {
                        let changed = ui
                            .add(SwitchSelection::new(update_state, &mut start).id_salt("switch"))
                            .changed();
                        if changed {
                            *end = *start;
                        }
                        changed
                    };
                } else {
                    ui.label("First switch");
                    modified |= {
                        let changed = ui
                            .add(
                                SwitchSelection::new(update_state, &mut start)
                                    .id_salt("start switch"),
                            )
                            .changed();
                        if changed && start > end {
                            *end = *start;
                        }
                        changed
                    };
                    ui.label("Last switch");
                    modified |= {
                        let changed = ui
                            .add(SwitchSelection::new(update_state, &mut end).id_salt("end switch"))
                            .changed();
                        if changed && start > end {
                            *start = *end;
                        }
                        changed
                    };
                }

                let operation = command.parameters[2].as_integer_mut().unwrap();
                ui.label("Operation");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Operation>,
                        "operation",
                        operation,
                    ))
                    .changed();
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
