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

use super::{DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo, UpdateState};
use crate::components::{EnumComboBox, OptionalIdComboBox};
use std::marker::PhantomData;

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum Operation {
    #[strum(to_string = "Increase amount of item by operand")]
    Increase = 0,
    #[strum(to_string = "Decrease amount of item by operand")]
    Decrease = 1,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum OperandType {
    #[strum(to_string = "Constant")]
    Constant = 0,
    #[strum(to_string = "Variable")]
    Variable = 1,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self) -> &'static str {
        "Change Items"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> String {
        let id = command.parameters[0].as_integer().unwrap();
        let items = update_state.data.items();
        let name = id
            .checked_sub(1)
            .and_then(|id| usize::try_from(id).ok())
            .and_then(|id| items.data.get(id))
            .map(|data| data.name.as_str())
            .unwrap_or_default();
        let operation = match command.parameters[1].as_integer().unwrap() {
            0 => "+=",
            1 => "-=",
            _ => {
                return String::new();
            }
        };
        match command.parameters[2].as_integer().unwrap() {
            0 => {
                let constant = command.parameters[3].as_integer().unwrap();
                format!("[{id:0>4}: {name}] {operation} {constant}")
            }

            1 => {
                let variable_id = command.parameters[3].as_integer().unwrap();
                let system = update_state.data.system();
                let variable_name = variable_id
                    .checked_sub(1)
                    .and_then(|id| usize::try_from(id).ok())
                    .and_then(|id| system.variables.get(id))
                    .map(|name| name.as_str())
                    .unwrap_or_default();
                format!("[{id:0>4}: {name}] {operation} [{variable_id:0>4}: {variable_name}]")
            }

            _ => String::new(),
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

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                ui.label("Item");
                {
                    let items = update_state.data.items();
                    modified |= ui
                        .add(OptionalIdComboBox::new(
                            update_state,
                            "item",
                            command.parameters[0].as_integer_mut().unwrap(),
                            1..=items.data.len(),
                            |id| {
                                id.checked_sub(1)
                                    .and_then(|id| items.data.get(id))
                                    .map_or_else(
                                        || "".into(),
                                        |x| format!("{:0>4}: {}", id, x.name),
                                    )
                            },
                        ))
                        .changed();
                }

                ui.label("Operation");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Operation>,
                        "operation",
                        command.parameters[1].as_integer_mut().unwrap(),
                    ))
                    .changed();

                let operand = command.parameters[2].as_integer_mut().unwrap();
                ui.label("Operand");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<OperandType>,
                        "operand",
                        operand,
                    ))
                    .changed();
                let operand = *operand;

                match operand {
                    0 => {
                        modified |= ui
                            .add(
                                egui::DragValue::new(
                                    command.parameters[3].as_integer_mut().unwrap(),
                                )
                                .range(1..=i32::MAX),
                            )
                            .changed();
                    }

                    1 => {
                        let system = update_state.data.system();
                        modified |= ui
                            .add(OptionalIdComboBox::new(
                                update_state,
                                "variable",
                                command.parameters[3].as_integer_mut().unwrap(),
                                1..=system.variables.len(),
                                |id| {
                                    id.checked_sub(1)
                                        .and_then(|id| system.variables.get(id))
                                        .map_or_else(|| "".into(), |x| format!("{:0>4}: {}", id, x))
                                },
                            ))
                            .changed();
                    }

                    _ => unreachable!(),
                }
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
