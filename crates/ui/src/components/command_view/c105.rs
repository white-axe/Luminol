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
use crate::components::OptionalIdComboBox;

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self) -> &'static str {
        "Button Input Processing"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> String {
        let id = command.parameters[0].as_integer().unwrap();
        let system = update_state.data.system();
        let name = id
            .checked_sub(1)
            .and_then(|id| usize::try_from(id).ok())
            .and_then(|id| system.variables.get(id))
            .map(|name| name.as_str())
            .unwrap_or_default();
        format!("[{id:0>4}: {name}]")
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
                let variable = command.parameters[0].as_integer_mut().unwrap();
                {
                    let system = update_state.data.system();
                    modified |= ui
                        .add(OptionalIdComboBox::new(
                            update_state,
                            "variable",
                            variable,
                            1..=system.variables.len(),
                            |id| {
                                id.checked_sub(1)
                                    .and_then(|id| system.variables.get(id))
                                    .map_or_else(|| "".into(), |x| format!("{:0>4}: {}", id, x))
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
