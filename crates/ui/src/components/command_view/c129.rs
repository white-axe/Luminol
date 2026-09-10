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
    #[strum(to_string = "Add actor to party")]
    Add = 0,
    #[strum(to_string = "Remove actor from party")]
    Remove = 1,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum ActorInit {
    #[strum(to_string = "Don't initialize actor")]
    NoInit = 0,
    #[strum(to_string = "Initialize actor")]
    Init = 1,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self, _command: &EventCommand) -> String {
        "Change Party Members".into()
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
                ui.label("Actor");
                {
                    let actors = update_state.data.actors();
                    modified |= ui
                        .add(OptionalIdComboBox::new(
                            update_state,
                            "actor",
                            command.parameters[0].as_integer_mut().unwrap(),
                            1..=actors.data.len(),
                            |id| {
                                id.checked_sub(1)
                                    .and_then(|id| actors.data.get(id))
                                    .map_or_else(
                                        || "".into(),
                                        |x| format!("{:0>4}: {}", id, x.name),
                                    )
                            },
                        ))
                        .changed();
                }

                let operation = command.parameters[1].as_integer_mut().unwrap();
                ui.label("Operation");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Operation>,
                        "operation",
                        operation,
                    ))
                    .changed();
                let operation = *operation;

                match operation {
                    0 => {
                        command.parameters.resize(3, ParameterType::Integer(1));

                        ui.label("Actor initialization");
                        modified |= ui
                            .add(EnumComboBox::new_with_conversion(
                                PhantomData::<ActorInit>,
                                "actor init",
                                command.parameters[2].as_integer_mut().unwrap(),
                            ))
                            .changed();
                    }

                    1 => {
                        command.parameters.resize(2, ParameterType::Integer(1));
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
