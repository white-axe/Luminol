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
    DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo, GraphicSelection,
    ParameterType, UpdateState,
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
enum GraphicType {
    Panorama = 0,
    Fog = 1,
    Battleback = 2,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum BlendMode {
    Normal = 0,
    Add = 1,
    Subtract = 2,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self) -> &'static str {
        "Change Map Settings"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        _update_state: &mut UpdateState<'_>,
        _event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> Option<String> {
        let graphic_name = command.parameters[1].as_string().unwrap();
        match command.parameters[0].as_integer().unwrap() {
            0 => {
                let hue = command.parameters[2].as_integer().unwrap();
                Some(format!("Set panorama to [{graphic_name}] with hue {hue}"))
            }

            1 => {
                let hue = command.parameters[2].as_integer().unwrap();
                let opacity = command.parameters[3].as_integer().unwrap();
                let blend_mode = match command.parameters[4].as_integer().unwrap() {
                    0 => Some("normal"),
                    1 => Some("add"),
                    2 => Some("subtract"),
                    _ => None,
                }?;
                let zoom = command.parameters[5].as_integer().unwrap();
                let sx = command.parameters[6].as_integer().unwrap();
                let sy = command.parameters[7].as_integer().unwrap();
                Some(format!("Set fog to [{graphic_name}] with hue {hue}, opacity {opacity}%, blend mode {blend_mode}, zoom {zoom}%, horizontal offset {sx}, vertical offset {sy}"))
            }

            2 => Some(format!("Set battleback to [{graphic_name}]")),

            _ => None,
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
                let graphic_type = command.parameters[0].as_integer_mut().unwrap();
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<GraphicType>,
                        "graphic type",
                        graphic_type,
                    ))
                    .changed();

                match *graphic_type {
                    0 => {
                        command.parameters.resize(3, ParameterType::Integer(0));

                        let [name, hue] = command.parameters[1..].first_chunk_mut().unwrap();
                        modified |= ui
                            .add(
                                GraphicSelection::new(
                                    update_state,
                                    command.state.get_or_insert_default(),
                                    (0, "graphic"),
                                    "Graphics/Panoramas",
                                    name,
                                )
                                .hue_parameter(hue),
                            )
                            .changed();
                    }

                    1 => {
                        let need_default_opacity = command.parameters.len() <= 3;
                        command.parameters.resize(8, ParameterType::Integer(0));
                        if need_default_opacity {
                            command.parameters[3] = ParameterType::Integer(255);
                        }

                        let [name, hue, opacity] =
                            command.parameters[1..].first_chunk_mut().unwrap();
                        modified |= ui
                            .add(
                                GraphicSelection::new(
                                    update_state,
                                    command.state.get_or_insert_default(),
                                    (1, "graphic"),
                                    "Graphics/Fogs",
                                    name,
                                )
                                .hue_parameter(hue)
                                .opacity_parameter(opacity),
                            )
                            .changed();

                        ui.label("Blend mode");
                        modified |= ui
                            .add(EnumComboBox::new_with_conversion(
                                PhantomData::<BlendMode>,
                                "blend mode",
                                command.parameters[4].as_integer_mut().unwrap(),
                            ))
                            .changed();

                        ui.label("Zoom");
                        modified |= ui
                            .add(
                                egui::DragValue::new(
                                    command.parameters[5].as_integer_mut().unwrap(),
                                )
                                .range(0..=i32::MAX)
                                .custom_formatter(|value, _| format!("{}%", value as i32)),
                            )
                            .changed();

                        ui.label("Horizontal offset");
                        modified |= ui
                            .add(egui::DragValue::new(
                                command.parameters[6].as_integer_mut().unwrap(),
                            ))
                            .changed();

                        ui.label("Vertical offset");
                        modified |= ui
                            .add(egui::DragValue::new(
                                command.parameters[7].as_integer_mut().unwrap(),
                            ))
                            .changed();
                    }

                    2 => {
                        command.parameters.resize(2, ParameterType::Integer(0));

                        modified |= ui
                            .add(GraphicSelection::new(
                                update_state,
                                command.state.get_or_insert_default(),
                                (2, "graphic"),
                                "Graphics/Battlebacks",
                                &mut command.parameters[1],
                            ))
                            .changed();
                    }

                    _ => {}
                }
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
