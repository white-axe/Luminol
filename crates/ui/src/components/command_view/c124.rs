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
    DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo, ParameterType,
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
pub enum Operation {
    #[strum(to_string = "Start system timer")]
    Start = 0,
    #[strum(to_string = "Stop system timer")]
    Stop = 1,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self) -> &'static str {
        "Control Timer"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        _update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> String {
        match command.parameters[0].as_integer().unwrap() {
            0 => {
                let start_time = command
                    .parameters
                    .get(1)
                    .map(|parameter| *parameter.as_integer().unwrap())
                    .unwrap_or_default();
                format!("Start timer at {start_time} frames")
            }

            1 => {
                format!("Stop timer")
            }

            _ => String::new(),
        }
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        _stripe: &mut bool,
        _update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                let operation = command.parameters[0].as_integer_mut().unwrap();
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
                        command.parameters.resize(2, ParameterType::Integer(0));

                        ui.label("Timer start time in frames");
                        modified |= ui
                            .add(
                                egui::DragValue::new(
                                    command.parameters[1].as_integer_mut().unwrap(),
                                )
                                .range(0..=i32::MAX),
                            )
                            .changed();
                    }

                    1 => {
                        command.parameters.resize(1, ParameterType::Integer(0));
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
