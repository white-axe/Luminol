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

use super::{EventCommand, EventCommandEditor, EventInfo, UiExt, UpdateState};
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
    #[strum(to_string = "Increase amount of armor by operand")]
    Increase = 0,
    #[strum(to_string = "Decrease amount of armor by operand")]
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
    fn name(&self, _command: &EventCommand) -> String {
        "Change Armor".into()
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        stripe: &mut bool,
        update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                ui.with_stripe_mut(stripe, |ui, _stripe| {
                    ui.label("Armor");
                    let armors = update_state.data.armors();
                    modified |= ui
                        .add(OptionalIdComboBox::new(
                            update_state,
                            "armor",
                            command.parameters[0].as_integer_mut().unwrap(),
                            1..=armors.data.len(),
                            |id| {
                                id.checked_sub(1)
                                    .and_then(|id| armors.data.get(id))
                                    .map_or_else(
                                        || "".into(),
                                        |x| format!("{:0>4}: {}", id, x.name),
                                    )
                            },
                        ))
                        .changed();
                });

                ui.with_stripe_mut(stripe, |ui, _stripe| {
                    ui.label("Operation");
                    modified |= ui
                        .add(EnumComboBox::new_with_conversion(
                            PhantomData::<Operation>,
                            "operation",
                            command.parameters[1].as_integer_mut().unwrap(),
                        ))
                        .changed();
                });

                let operand = command.parameters[2].as_integer_mut().unwrap();
                ui.with_stripe_mut(stripe, |ui, _stripe| {
                    ui.label("Operand");
                    modified |= ui
                        .add(EnumComboBox::new_with_conversion(
                            PhantomData::<OperandType>,
                            "operand",
                            operand,
                        ))
                        .changed();
                });
                let operand = *operand;

                match operand {
                    0 => {
                        ui.with_stripe_mut(stripe, |ui, _stripe| {
                            modified |= ui
                                .add(
                                    egui::DragValue::new(
                                        command.parameters[3].as_integer_mut().unwrap(),
                                    )
                                    .range(1..=i32::MAX),
                                )
                                .changed();
                        });
                    }

                    1 => {
                        ui.with_stripe_mut(stripe, |ui, _stripe| {
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
                                            .map_or_else(
                                                || "".into(),
                                                |x| format!("{:0>4}: {}", id, x),
                                            )
                                    },
                                ))
                                .changed();
                        });
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
