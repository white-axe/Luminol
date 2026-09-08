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
use crate::components::OptionalIdComboBox;

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self, _command: &EventCommand) -> String {
        "Call Common Event".into()
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                let common_event = match &mut command.parameters[0] {
                    ParameterType::Integer(parameter) => Some(parameter),
                    _ => None,
                }
                .unwrap();
                {
                    let common_events = update_state.data.common_events();
                    modified |= ui
                        .add(OptionalIdComboBox::new(
                            update_state,
                            "common event",
                            common_event,
                            1..=common_events.data.len(),
                            |id| {
                                id.checked_sub(1)
                                    .and_then(|id| common_events.data.get(id))
                                    .map_or_else(
                                        || "".into(),
                                        |x| format!("{:0>4}: {}", id, x.name),
                                    )
                            },
                        ))
                        .changed();
                };
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
