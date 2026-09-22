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
    CharacterSelection, DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo,
    LocationFmt, LocationSelection, UpdateState,
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
enum LocationType {
    #[strum(to_string = "Move to constant location")]
    Constant = 0,
    #[strum(to_string = "Move to variable location")]
    Variable = 1,
    #[strum(to_string = "Swap with event")]
    Swap = 2,
}

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
        "Set Event Location"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        update_state: &mut UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> Option<String> {
        let event_fmt =
            CharacterSelection::new(update_state, event_info, &command.parameters[0]).fmt()?;

        let [discriminant, x, y] = command.parameters[1..].first_chunk().unwrap();

        let location_fmt = match *discriminant.as_integer().unwrap() {
            2 => {
                let location_fmt = CharacterSelection::new(update_state, event_info, x).fmt()?;
                format!("Swap [{event_fmt}] with [{location_fmt}]")
            }

            _ => {
                let location_fmt =
                    match LocationSelection::new(update_state, discriminant, x, y).fmt()? {
                        LocationFmt::Constant((x_fmt, y_fmt)) => format!("({x_fmt}, {y_fmt})"),
                        LocationFmt::Variable((x_fmt, y_fmt)) => format!("([{x_fmt}], [{y_fmt}])"),
                    };
                format!("Move [{event_fmt}] to {location_fmt}")
            }
        };

        let direction_fmt = match command.parameters[4].as_integer().unwrap() {
            0 => Some("direction unchanged"),
            2 => Some("facing down"),
            4 => Some("facing left"),
            6 => Some("facing right"),
            8 => Some("facing up"),
            _ => None,
        }?;

        Some(format!("{location_fmt}, {direction_fmt}"))
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        _stripe: &mut bool,
        update_state: &mut UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                modified |= ui
                    .add(
                        CharacterSelection::new(
                            update_state,
                            event_info,
                            &mut command.parameters[0],
                        )
                        .prepare("event"),
                    )
                    .changed();

                let [discriminant, x, y] = command.parameters[1..].first_chunk_mut().unwrap();
                modified |= ui
                    .add(
                        LocationSelection::new(update_state, discriminant, x, y)
                            .prepare_with_custom_enum("map", PhantomData::<LocationType>),
                    )
                    .changed();

                if *command.parameters[1].as_integer().unwrap() == 2 {
                    modified |= ui
                        .add(
                            CharacterSelection::new(
                                update_state,
                                event_info,
                                &mut command.parameters[2],
                            )
                            .prepare((2, "event")),
                        )
                        .changed();
                }

                ui.label("Direction");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Direction>,
                        "direction",
                        command.parameters[4].as_integer_mut().unwrap(),
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
