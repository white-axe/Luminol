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
    DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo, LocationFmt,
    LocationSelection, MapFmt, UpdateState,
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
enum Direction {
    Unchanged = 0,
    Down = 2,
    Left = 4,
    Right = 6,
    Up = 8,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum Transition {
    Enabled = 0,
    Disabled = 1,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self) -> &'static str {
        "Transfer Player"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> Option<String> {
        let [discriminant, map, x, y] = command.parameters.first_chunk().unwrap();

        let location_selection = LocationSelection::new(update_state, discriminant, x, y);
        let location_fmt = match location_selection.fmt()? {
            LocationFmt::Constant((x_fmt, y_fmt)) => format!("({x_fmt}, {y_fmt})"),
            LocationFmt::Variable((x_fmt, y_fmt)) => format!("([{x_fmt}], [{y_fmt}])"),
        };

        let map_fmt = match location_selection.map_parameter(map).fmt()? {
            MapFmt::Constant(fmt) => format!("Map [{fmt}]"),
            MapFmt::Variable(fmt) => format!("Variable [{fmt}]"),
        };

        let direction_fmt = match command.parameters[4].as_integer().unwrap() {
            0 => Some("direction unchanged"),
            2 => Some("facing down"),
            4 => Some("facing left"),
            6 => Some("facing right"),
            8 => Some("facing up"),
            _ => None,
        }?;

        let transition_fmt = match command.parameters[5].as_integer().unwrap() {
            0 => Some("with transition"),
            1 => Some("no transition"),
            _ => None,
        }?;

        Some(format!(
            "{map_fmt} at {location_fmt}, {direction_fmt}, {transition_fmt}"
        ))
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
                let [discriminant, map, x, y] = command.parameters.first_chunk_mut().unwrap();
                modified |= ui
                    .add(
                        LocationSelection::new(update_state, discriminant, x, y)
                            .map_parameter(map)
                            .prepare("map"),
                    )
                    .changed();

                ui.label("Direction");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Direction>,
                        "direction",
                        command.parameters[4].as_integer_mut().unwrap(),
                    ))
                    .changed();

                ui.label("Transition");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Transition>,
                        "transition",
                        command.parameters[5].as_integer_mut().unwrap(),
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
