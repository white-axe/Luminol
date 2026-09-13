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
    DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo, ItemSelection,
    UpdateState, VariableSelection,
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
enum OperandType {
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
        let Some(item) = ItemSelection::new(update_state, &command.parameters[1]).fmt() else {
            return String::new();
        };
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
                format!("[{item}] {operation} {constant}")
            }

            1 => {
                let Some(variable) =
                    VariableSelection::new(update_state, &command.parameters[3]).fmt()
                else {
                    return String::new();
                };
                format!("[{item}] {operation} [{variable}]")
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
                modified |= ui
                    .add(
                        ItemSelection::new(update_state, &mut command.parameters[0])
                            .id_salt("item"),
                    )
                    .changed();

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
                        modified |= ui
                            .add(
                                VariableSelection::new(update_state, &mut command.parameters[3])
                                    .id_salt("variable"),
                            )
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
