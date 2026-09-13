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
use crate::components::EnumComboBox;
use std::marker::PhantomData;

#[derive(PartialEq, Eq, strum::Display, strum::EnumIter)]
pub enum SelfSwitchType {
    A,
    B,
    C,
    D,
    #[strum(to_string = "Custom self switch")]
    Custom,
}

impl From<&str> for SelfSwitchType {
    fn from(value: &str) -> Self {
        match value {
            "A" => SelfSwitchType::A,
            "B" => SelfSwitchType::B,
            "C" => SelfSwitchType::C,
            "D" => SelfSwitchType::D,
            _ => SelfSwitchType::Custom,
        }
    }
}

impl From<&SelfSwitchType> for &str {
    fn from(value: &SelfSwitchType) -> Self {
        match value {
            SelfSwitchType::A => "A",
            SelfSwitchType::B => "B",
            SelfSwitchType::C => "C",
            SelfSwitchType::D => "D",
            _ => "E",
        }
    }
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum Operation {
    #[strum(to_string = "Set self switch to on")]
    On = 0,
    #[strum(to_string = "Set self switch to off")]
    Off = 1,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self) -> &'static str {
        "Control Self Switch"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        _update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> String {
        let name = command.parameters[0].as_string().unwrap();
        let value = match command.parameters[1].as_integer().unwrap() {
            0 => "on",
            1 => "off",
            _ => return String::new(),
        };
        format!("Set [{name}] to {value}")
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

        let is_custom_self_switch = command.state.get_or_insert(false);

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                ui.label("Self switch");

                let self_switch = command.parameters[0].as_string_mut().unwrap();
                let mut self_switch_type = if *is_custom_self_switch {
                    SelfSwitchType::Custom
                } else {
                    SelfSwitchType::from(self_switch.as_str())
                };
                modified |= {
                    let changed = ui
                        .add(EnumComboBox::new((3, 1), &mut self_switch_type))
                        .changed();
                    if changed {
                        *self_switch = <&str>::from(&self_switch_type).to_string();
                    }
                    changed
                };

                *is_custom_self_switch = self_switch_type == SelfSwitchType::Custom;
                if *is_custom_self_switch {
                    modified |= ui.text_edit_singleline(self_switch).changed();
                }
                modified |= ui.text_edit_singleline(self_switch).changed();

                let operation = command.parameters[1].as_integer_mut().unwrap();
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
