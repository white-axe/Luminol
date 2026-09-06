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

use super::{EventCommand, EventCommandEditor, ParameterType, UpdateState};
use crate::components::EnumComboBox;
use std::marker::PhantomData;

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum Position {
    Top = 0,
    Middle = 1,
    Bottom = 2,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum Frame {
    Show = 0,
    Hide = 1,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self, _command: &EventCommand) -> String {
        "Change Text Options".into()
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        _update_state: &mut UpdateState<'_>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                ui.label("Position");

                let position = match &mut command.parameters[0] {
                    ParameterType::Integer(parameter) => Some(parameter),
                    _ => None,
                }
                .unwrap();
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Position>,
                        "position",
                        position,
                    ))
                    .changed();

                ui.label("Frame");

                let frame = match &mut command.parameters[1] {
                    ParameterType::Integer(parameter) => Some(parameter),
                    _ => None,
                }
                .unwrap();
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Frame>,
                        "frame",
                        frame,
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
