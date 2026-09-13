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
    DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo, UpdateState, ValueFmt,
    ValueSelection,
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
    #[strum(to_string = "Increase party's gold by operand")]
    Increase = 0,
    #[strum(to_string = "Decrease party's gold by operand")]
    Decrease = 1,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self) -> &'static str {
        "Change Gold"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> String {
        let operation = match command.parameters[0].as_integer().unwrap() {
            0 => "Increase by",
            1 => "Decrease by",
            _ => {
                return String::new();
            }
        };
        let Some(operand) = ValueSelection::new(
            update_state,
            command.parameters[1].as_integer().unwrap(),
            command.parameters[2].as_integer().unwrap(),
        )
        .fmt() else {
            return String::new();
        };
        match operand {
            ValueFmt::Constant(constant) => {
                format!("{operation} {constant}")
            }
            ValueFmt::Variable(variable) => {
                format!("{operation} [{variable}]")
            }
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
                ui.label("Operation");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Operation>,
                        "operation",
                        command.parameters[0].as_integer_mut().unwrap(),
                    ))
                    .changed();

                let [discriminant, value] = command.parameters[1..].first_chunk_mut().unwrap();
                ui.label("Operand");
                modified |= ui
                    .add(ValueSelection::new(update_state, discriminant, value).id_salt("operand"))
                    .changed();
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
