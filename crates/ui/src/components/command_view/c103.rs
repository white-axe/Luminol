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
    EventCommand, EventCommandEditor, EventCommandEditorState, ParameterType, UpdateState,
};
use luminol_core::Modal;

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self, _command: &EventCommand) -> String {
        "Input Number".into()
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        update_state: &mut UpdateState<'_>,
        state: EventCommandEditorState<'_>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let id = ui.id();
        let variable_modal = state.get_or_insert_with(|| {
            crate::modals::database_modal::VariableModal::new(id.with("variable"))
        });

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                let mut variable_index = match command.parameters[0] {
                    ParameterType::Integer(parameter) => Some(parameter),
                    _ => None,
                }
                .unwrap() as _;
                modified |= {
                    let changed = ui
                        .add(variable_modal.button(&mut variable_index, update_state))
                        .changed();
                    if changed {
                        command.parameters[0] = ParameterType::Integer(variable_index as _);
                    }
                    changed
                };

                ui.label("Digits");

                modified |= ui
                    .add(
                        egui::DragValue::new(
                            match &mut command.parameters[1] {
                                ParameterType::Integer(parameter) => Some(parameter),
                                _ => None,
                            }
                            .unwrap(),
                        )
                        .range(1..=i32::MAX),
                    )
                    .changed();
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
