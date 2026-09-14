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
    ActorSelection, CharacterSelection, DescriptionWidthCallback, EventCommand, EventCommandEditor,
    EventInfo, ItemSelection, ParameterType, UpdateState, VariableSelection,
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
    #[strum(to_string = "Set")]
    Set = 0,
    #[strum(to_string = "Add")]
    Add = 1,
    #[strum(to_string = "Subtract")]
    Sub = 2,
    #[strum(to_string = "Multiply")]
    Mul = 3,
    #[strum(to_string = "Divide")]
    Div = 4,
    #[strum(to_string = "Modulo")]
    Mod = 5,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(u64)]
enum OperandType {
    #[strum(to_string = "Constant")]
    Constant = 0,
    #[strum(to_string = "Variable")]
    Variable = 1,
    #[strum(to_string = "Random value")]
    Random = 2,
    #[strum(to_string = "Amount of an item in inventory")]
    Item = 3,
    #[strum(to_string = "Actor")]
    Actor = 4,
    #[strum(to_string = "Enemy")]
    Enemy = 5,
    #[strum(to_string = "Character")]
    Character = 6,
    #[strum(to_string = "Current map ID")]
    MapId = 7,
    #[strum(to_string = "Party size")]
    PartyMembers = (1 << 32) | 7,
    #[strum(to_string = "Party's gold")]
    Gold = (2 << 32) | 7,
    #[strum(to_string = "Party's step count")]
    Steps = (3 << 32) | 7,
    #[strum(to_string = "Play time")]
    PlayTime = (4 << 32) | 7,
    #[strum(to_string = "System timer")]
    Timer = (5 << 32) | 7,
    #[strum(to_string = "Save count")]
    SaveCount = (6 << 32) | 7,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum ActorProperty {
    #[strum(to_string = "Actor's level")]
    Level = 0,
    #[strum(to_string = "Actor's EXP")]
    Exp = 1,
    #[strum(to_string = "Actor's HP")]
    Hp = 2,
    #[strum(to_string = "Actor's SP")]
    Sp = 3,
    #[strum(to_string = "Actor's max HP")]
    MaxHp = 4,
    #[strum(to_string = "Actor's max SP")]
    MaxSp = 5,
    #[strum(to_string = "Actor's STR")]
    Str = 6,
    #[strum(to_string = "Actor's DEX")]
    Dex = 7,
    #[strum(to_string = "Actor's AGI")]
    Agi = 8,
    #[strum(to_string = "Actor's INT")]
    Int = 9,
    #[strum(to_string = "Actor's ATK")]
    Atk = 10,
    #[strum(to_string = "Actor's PDEF")]
    Pdef = 11,
    #[strum(to_string = "Actor's MDEF")]
    Mdef = 12,
    #[strum(to_string = "Actor's EVA")]
    Eva = 13,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum EnemyProperty {
    #[strum(to_string = "Enemy's HP")]
    Hp = 0,
    #[strum(to_string = "Enemy's SP")]
    Sp = 1,
    #[strum(to_string = "Enemy's max HP")]
    MaxHp = 2,
    #[strum(to_string = "Enemy's max SP")]
    MaxSp = 3,
    #[strum(to_string = "Enemy's STR")]
    Str = 4,
    #[strum(to_string = "Enemy's DEX")]
    Dex = 5,
    #[strum(to_string = "Enemy's AGI")]
    Agi = 6,
    #[strum(to_string = "Enemy's INT")]
    Int = 7,
    #[strum(to_string = "Enemy's ATK")]
    Atk = 8,
    #[strum(to_string = "Enemy's PDEF")]
    Pdef = 9,
    #[strum(to_string = "Enemy's MDEF")]
    Mdef = 10,
    #[strum(to_string = "Enemy's EVA")]
    Eva = 11,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum CharacterProperty {
    #[strum(to_string = "Character's map x-coordinate")]
    X = 0,
    #[strum(to_string = "Character's map y-coordinate")]
    Y = 1,
    #[strum(to_string = "Character's direction")]
    Direction = 2,
    #[strum(to_string = "Character's screen x-coordinate")]
    ScreenX = 3,
    #[strum(to_string = "Character's screen y-coordinate")]
    ScreenY = 4,
    #[strum(to_string = "Character's terrain tag")]
    TerrainTag = 5,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn name(&self) -> &'static str {
        "Control Variables"
    }

    fn description(
        &self,
        _callback: DescriptionWidthCallback<'_>,
        update_state: &UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> Option<String> {
        let start_id = command.parameters[0].as_integer().unwrap();
        let end_id = command.parameters[1].as_integer().unwrap();
        let operation = match command.parameters[2].as_integer().unwrap() {
            0 => Some("="),
            1 => Some("+="),
            2 => Some("-="),
            3 => Some("*="),
            4 => Some("/="),
            5 => Some("%="),
            _ => None,
        }?;
        let start = VariableSelection::new(update_state, start_id).fmt()?;
        let variable_string = if start_id == end_id {
            format!("{start} {operation}")
        } else {
            let end = VariableSelection::new(update_state, end_id).fmt()?;
            format!("[{end}] - [{end}] {operation}")
        };
        match command.parameters[3].as_integer().unwrap() {
            0 => {
                let constant = command
                    .parameters
                    .get(4)
                    .map(|parameter| *parameter.as_integer().unwrap())
                    .unwrap_or_default();
                Some(format!("{variable_string} {constant}"))
            }

            1 => {
                let variable = VariableSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(4)
                        .map(|parameter| *parameter.as_integer().unwrap())
                        .unwrap_or_default(),
                )
                .fmt()?;
                Some(format!("{variable_string} [{variable}]"))
            }

            2 => {
                let random_start = command
                    .parameters
                    .get(4)
                    .map(|parameter| *parameter.as_integer().unwrap())
                    .unwrap_or_default();
                let random_end = command
                    .parameters
                    .get(5)
                    .map(|parameter| *parameter.as_integer().unwrap())
                    .unwrap_or_default();
                Some(format!(
                    "{variable_string} random in range {random_start} - {random_end}"
                ))
            }

            3 => {
                let item = ItemSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(4)
                        .map(|parameter| *parameter.as_integer().unwrap())
                        .unwrap_or_default(),
                )
                .fmt()?;
                Some(format!("{variable_string} amount of [{item}] in inventory"))
            }

            4 => {
                let actor = ActorSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(4)
                        .map(|parameter| *parameter.as_integer().unwrap())
                        .unwrap_or_default(),
                )
                .fmt()?;
                let actor_property = match command
                    .parameters
                    .get(5)
                    .map(|parameter| *parameter.as_integer().unwrap())
                    .unwrap_or_default()
                {
                    0 => Some("level"),
                    1 => Some("EXP"),
                    2 => Some("HP"),
                    3 => Some("SP"),
                    4 => Some("max HP"),
                    5 => Some("max SP"),
                    6 => Some("STR"),
                    7 => Some("DEX"),
                    8 => Some("AGI"),
                    9 => Some("INT"),
                    10 => Some("ATK"),
                    11 => Some("PDEF"),
                    12 => Some("MDEF"),
                    13 => Some("EVA"),
                    _ => None,
                }?;
                Some(format!("{variable_string} [{actor}]'s {actor_property}"))
            }

            5 => {
                let enemy_index = command
                    .parameters
                    .get(4)
                    .map(|parameter| *parameter.as_integer().unwrap())
                    .unwrap_or_default()
                    .wrapping_add(1);
                let enemy_property = match command
                    .parameters
                    .get(5)
                    .map(|parameter| *parameter.as_integer().unwrap())
                    .unwrap_or_default()
                {
                    0 => Some("HP"),
                    1 => Some("SP"),
                    2 => Some("max HP"),
                    3 => Some("max SP"),
                    4 => Some("STR"),
                    5 => Some("DEX"),
                    6 => Some("AGI"),
                    7 => Some("INT"),
                    8 => Some("ATK"),
                    9 => Some("PDEF"),
                    10 => Some("MDEF"),
                    11 => Some("EVA"),
                    _ => None,
                }?;
                Some(format!(
                    "{variable_string} enemy #{enemy_index}'s {enemy_property}"
                ))
            }

            6 => {
                let character = CharacterSelection::new(
                    update_state,
                    event_info,
                    command
                        .parameters
                        .get(4)
                        .map(|parameter| *parameter.as_integer().unwrap())
                        .unwrap_or_default(),
                )
                .fmt()?;
                let character_property = match command
                    .parameters
                    .get(5)
                    .map(|parameter| *parameter.as_integer().unwrap())
                    .unwrap_or_default()
                {
                    0 => Some("map x-coordinate"),
                    1 => Some("map y-coordinate"),
                    2 => Some("direction"),
                    3 => Some("screen x-coordinate"),
                    4 => Some("screen y-coordinate"),
                    5 => Some("terrain tag"),
                    _ => None,
                }?;
                Some(format!(
                    "{variable_string} [{character}]'s {character_property}"
                ))
            }

            7 => {
                match command
                    .parameters
                    .get(4)
                    .map(|parameter| *parameter.as_integer().unwrap())
                    .unwrap_or_default()
                {
                    0 => Some(format!("{variable_string} Map ID")),

                    1 => Some(format!("{variable_string} Party size")),

                    2 => Some(format!("{variable_string} Gold")),

                    3 => Some(format!("{variable_string} Step count")),

                    4 => Some(format!("{variable_string} Play time")),

                    5 => Some(format!("{variable_string} Timer")),

                    6 => Some(format!("{variable_string} Save count")),

                    _ => None,
                }
            }

            _ => None,
        }
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        _stripe: &mut bool,
        update_state: &UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let mut modified = false;

        let is_batch_edit = command.state.get_or_insert(false);

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                let [start, end] = command.parameters.first_chunk_mut().unwrap();
                let mut start = start.as_integer_mut().unwrap();
                let mut end = end.as_integer_mut().unwrap();

                if start != end {
                    *is_batch_edit = true;
                }
                modified |= {
                    let changed = ui.checkbox(is_batch_edit, "Batch edit").changed();
                    let changed = changed && !*is_batch_edit && start != end;
                    if changed {
                        *end = *start;
                    }
                    changed
                };

                if !*is_batch_edit {
                    ui.label("Variable");
                    modified |= {
                        let changed = ui
                            .add(
                                VariableSelection::new(update_state, &mut start)
                                    .id_salt("variable"),
                            )
                            .changed();
                        if changed {
                            *end = *start;
                        }
                        changed
                    };
                } else {
                    ui.label("First variable");
                    modified |= {
                        let changed = ui
                            .add(
                                VariableSelection::new(update_state, &mut start)
                                    .id_salt("start variable"),
                            )
                            .changed();
                        if changed && start > end {
                            *end = *start;
                        }
                        changed
                    };
                    ui.label("Last variable");
                    modified |= {
                        let changed = ui
                            .add(
                                VariableSelection::new(update_state, &mut end)
                                    .id_salt("end variable"),
                            )
                            .changed();
                        if changed && start > end {
                            *start = *end;
                        }
                        changed
                    };
                }

                let operation = command.parameters[2].as_integer_mut().unwrap();
                ui.label("Operation");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<Operation>,
                        "operation",
                        operation,
                    ))
                    .changed();

                let mut operand_packed = *command.parameters[3].as_integer().unwrap() as u32 as u64;
                if operand_packed == 7 {
                    operand_packed |=
                        (*command.parameters[4].as_integer().unwrap() as u32 as u64) << 32;
                }
                ui.label("Operand");
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        PhantomData::<OperandType>,
                        "operand",
                        &mut operand_packed,
                    ))
                    .changed();
                let operand = operand_packed as u32 as i32;
                *command.parameters[3].as_integer_mut().unwrap() = operand;
                if operand == 7 {
                    *command.parameters[4].as_integer_mut().unwrap() =
                        (operand_packed >> 32) as u32 as i32;
                }

                match operand {
                    0 => {
                        command.parameters.resize(5, ParameterType::Integer(0));

                        modified |= ui
                            .add(egui::DragValue::new(
                                command.parameters[4].as_integer_mut().unwrap(),
                            ))
                            .changed();
                    }

                    1 => {
                        command.parameters.resize(5, ParameterType::Integer(0));

                        modified |= ui
                            .add(
                                VariableSelection::new(update_state, &mut command.parameters[4])
                                    .id_salt((1, 4)),
                            )
                            .changed();
                    }

                    2 => {
                        command.parameters.resize(6, ParameterType::Integer(0));

                        let [min, max] = command.parameters[4..].first_chunk_mut().unwrap();
                        let min = min.as_integer_mut().unwrap();
                        let max = max.as_integer_mut().unwrap();

                        if modified && min > max {
                            *max = *min;
                        }

                        ui.label("Minimum");
                        modified |= {
                            let changed = ui.add(egui::DragValue::new(min)).changed();
                            if changed && min > max {
                                *max = *min;
                            }
                            changed
                        };

                        ui.label("Maximum");
                        modified |= {
                            let changed = ui.add(egui::DragValue::new(max)).changed();
                            if changed && min > max {
                                *min = *max;
                            }
                            changed
                        };
                    }

                    3 => {
                        command.parameters.resize(5, ParameterType::Integer(0));

                        modified |= ui
                            .add(
                                ItemSelection::new(update_state, &mut command.parameters[4])
                                    .id_salt((3, 4)),
                            )
                            .changed();
                    }

                    4 => {
                        command.parameters.resize(6, ParameterType::Integer(0));

                        modified |= ui
                            .add(
                                ActorSelection::new(update_state, &mut command.parameters[4])
                                    .id_salt((4, 4)),
                            )
                            .changed();

                        ui.label("Actor property");
                        modified |= ui
                            .add(EnumComboBox::new_with_conversion(
                                PhantomData::<ActorProperty>,
                                (4, 5),
                                command.parameters[5].as_integer_mut().unwrap(),
                            ))
                            .changed();
                    }

                    5 => {
                        command.parameters.resize(6, ParameterType::Integer(0));

                        ui.label("Enemy");
                        let enemy = command.parameters[4].as_integer_mut().unwrap();
                        *enemy = enemy.wrapping_add(1);
                        modified |= ui
                            .add(
                                egui::DragValue::new(enemy)
                                    .range(1..=i32::MAX)
                                    .custom_formatter(|value, _| format!("#{}", (value as i32))),
                            )
                            .changed();
                        *enemy = enemy.wrapping_sub(1);

                        ui.label("Enemy property");
                        modified |= ui
                            .add(EnumComboBox::new_with_conversion(
                                PhantomData::<EnemyProperty>,
                                (5, 5),
                                command.parameters[5].as_integer_mut().unwrap(),
                            ))
                            .changed();
                    }

                    6 => {
                        command.parameters.resize(6, ParameterType::Integer(0));

                        ui.label("Character");
                        modified |= ui
                            .add(
                                CharacterSelection::new(
                                    update_state,
                                    event_info,
                                    &mut command.parameters[4],
                                )
                                .id_salt((6, 4)),
                            )
                            .changed();

                        ui.label("Character property");
                        modified |= ui
                            .add(EnumComboBox::new_with_conversion(
                                PhantomData::<CharacterProperty>,
                                (6, 5),
                                command.parameters[5].as_integer_mut().unwrap(),
                            ))
                            .changed();
                    }

                    7 => {
                        command.parameters.resize(5, ParameterType::Integer(0));
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
