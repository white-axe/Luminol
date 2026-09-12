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
    Collapsing, CommandView, DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo,
    ParameterType, UpdateState,
};
use crate::components::{EnumComboBox, OptionalIdComboBox};
use std::marker::PhantomData;

fn coerce_to_integer(parameter: &mut ParameterType) -> &mut i32 {
    if !parameter.is_integer() {
        *parameter = ParameterType::Integer(0);
    }
    parameter.as_integer_mut().unwrap()
}

fn coerce_to_string<'a>(
    parameter: &'a mut ParameterType,
    default_value: &'static str,
) -> &'a mut String {
    if !parameter.is_string() {
        *parameter = ParameterType::String(default_value.into());
    }
    parameter.as_string_mut().unwrap()
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum BranchType {
    Switch = 0,
    Variable = 1,
    #[strum(to_string = "Self switch")]
    SelfSwitch = 2,
    Timer = 3,
    Actor = 4,
    Enemy = 5,
    Character = 6,
    Gold = 7,
    #[strum(to_string = "Item is in inventory")]
    Item = 8,
    #[strum(to_string = "Weapon is in inventory")]
    Weapon = 9,
    #[strum(to_string = "Armor is in inventory")]
    Armor = 10,
    #[strum(to_string = "Button is being pressed")]
    Button = 11,
    #[strum(to_string = "Script evaluates to true")]
    Script = 12,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum SwitchCondition {
    #[strum(to_string = "Switch is on")]
    On = 0,
    #[strum(to_string = "Switch is off")]
    Off = 1,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum VariableValueType {
    Constant = 0,
    Variable = 1,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum VariableCondition {
    #[strum(to_string = "Value 1 == Value 2")]
    Eq = 0,
    #[strum(to_string = "Value 1 >= Value 2")]
    Ge = 1,
    #[strum(to_string = "Value 1 <= Value 2")]
    Le = 2,
    #[strum(to_string = "Value 1 > Value 2")]
    Gt = 3,
    #[strum(to_string = "Value 1 < Value 2")]
    Lt = 4,
    #[strum(to_string = "Value 1 != Value 2")]
    Ne = 5,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum SelfSwitchCondition {
    #[strum(to_string = "Self switch is on")]
    On = 0,
    #[strum(to_string = "Self switch is off")]
    Off = 1,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum TimerCondition {
    #[strum(to_string = "System timer >= value")]
    Ge = 0,
    #[strum(to_string = "System timer <= value")]
    Le = 1,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum ActorCondition {
    #[strum(to_string = "Actor is in the party")]
    InParty = 0,
    #[strum(to_string = "Actor has name")]
    Name = 1,
    #[strum(to_string = "Actor knows a skill")]
    Skill = 2,
    #[strum(to_string = "Actor has a weapon equipped")]
    Weapon = 3,
    #[strum(to_string = "Actor has an armor equipped")]
    Armor = 4,
    #[strum(to_string = "Actor is affected by a state")]
    State = 5,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum EnemyCondition {
    #[strum(to_string = "Enemy exists")]
    Exists = 0,
    #[strum(to_string = "Enemy is affected by a state")]
    State = 1,
}

#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum CharacterType {
    Player = -1,
    #[strum(to_string = "This event")]
    ThisEvent = 0,
    #[default]
    #[strum(to_string = "Map event")]
    MapEvent = 1,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum CharacterCondition {
    #[strum(to_string = "Character is facing down")]
    Down = 2,
    #[strum(to_string = "Character is facing left")]
    Left = 4,
    #[strum(to_string = "Character is facing right")]
    Right = 6,
    #[strum(to_string = "Character is facing up")]
    Up = 8,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum GoldCondition {
    #[strum(to_string = "Party's gold >= value")]
    Ge = 0,
    #[strum(to_string = "Party's gold <= value")]
    Le = 1,
}

#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum ButtonType {
    Down = 2,
    Left = 4,
    Right = 6,
    Up = 8,
    A = 11,
    B = 12,
    C = 13,
    X = 14,
    Y = 15,
    Z = 16,
    L = 17,
    R = 18,
    Shift = 21,
    Ctrl = 22,
    Alt = 23,
    F5 = 25,
    F6 = 26,
    F7 = 27,
    F8 = 28,
    F9 = 29,
    #[strum(to_string = "Primary mouse button")]
    MouseLeft = 38,
    #[strum(to_string = "Middle mouse button")]
    MouseMiddle = 39,
    #[strum(to_string = "Secondary mouse button")]
    MouseRight = 40,
    #[strum(to_string = "Mouse button 4")]
    MouseX1 = 41,
    #[strum(to_string = "Mouse button 5")]
    MouseX2 = 42,
    #[default]
    #[strum(to_string = "Custom button code")]
    Custom = 0,
}

pub(super) struct Editor;

impl EventCommandEditor for Editor {
    fn expand_by_default(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "Conditional Branch"
    }

    fn description(
        &self,
        callback: DescriptionWidthCallback<'_>,
        update_state: &mut UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> String {
        match *command.parameters[0].as_integer().unwrap() {
            0 => {
                let id = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let system = update_state.data.system();
                let name = id
                    .checked_sub(1)
                    .and_then(|id| usize::try_from(id).ok())
                    .and_then(|id| system.variables.get(id))
                    .map(|name| name.as_str())
                    .unwrap_or_default();
                let condition = match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => "is on",
                    1 => "is off",
                    _ => {
                        return String::new();
                    }
                };
                format!("[{id:0>4}: {name}] {condition}")
            }

            1 => {
                let id = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let system = update_state.data.system();
                let name = id
                    .checked_sub(1)
                    .and_then(|id| usize::try_from(id).ok())
                    .and_then(|id| system.variables.get(id))
                    .map(|name| name.as_str())
                    .unwrap_or_default();
                let condition = match command
                    .parameters
                    .get(4)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => "==",
                    1 => ">=",
                    2 => "<=",
                    3 => ">",
                    4 => "<",
                    5 => "!=",
                    _ => {
                        return String::new();
                    }
                };
                match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => {
                        let constant = command
                            .parameters
                            .get(3)
                            .and_then(|parameter| parameter.as_integer().copied())
                            .unwrap_or_default();
                        format!("[{id:0>4}: {name}] {condition} {constant}")
                    }
                    1 => {
                        let variable_id = command
                            .parameters
                            .get(3)
                            .and_then(|parameter| parameter.as_integer().copied())
                            .unwrap_or_default();
                        let variable_name = variable_id
                            .checked_sub(1)
                            .and_then(|id| usize::try_from(id).ok())
                            .and_then(|id| system.variables.get(id))
                            .map(|name| name.as_str())
                            .unwrap_or_default();
                        format!(
                            "[{id:0>4}: {name}] {condition} [{variable_id:0>4}: {variable_name}]"
                        )
                    }
                    _ => String::new(),
                }
            }

            2 => {
                let name = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_string())
                    .map(|parameter| parameter.as_str())
                    .unwrap_or_default();
                let condition = match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => "is on",
                    1 => "is off",
                    _ => return String::new(),
                };
                format!("[{name}] {condition}")
            }

            3 => {
                let value = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let condition = match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => ">=",
                    1 => "<=",
                    _ => {
                        return String::new();
                    }
                };
                format!("Timer {condition} {value}")
            }

            4 => {
                let id = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let actors = update_state.data.actors();
                let name = id
                    .checked_sub(1)
                    .and_then(|id| usize::try_from(id).ok())
                    .and_then(|id| actors.data.get(id))
                    .map(|data| data.name.as_str())
                    .unwrap_or_default();
                match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => format!("[{id:0>4}: {name}] is in the party"),

                    1 => {
                        let actor_name = command
                            .parameters
                            .get(3)
                            .and_then(|parameter| parameter.as_string())
                            .map(|parameter| parameter.as_str())
                            .unwrap_or_default();
                        format!("[{id:0>4}: {name}] is named {actor_name}")
                    }

                    2 => {
                        let skill_id = command
                            .parameters
                            .get(3)
                            .and_then(|parameter| parameter.as_integer().copied())
                            .unwrap_or_default();
                        let skills = update_state.data.skills();
                        let skill_name = skill_id
                            .checked_sub(1)
                            .and_then(|id| usize::try_from(id).ok())
                            .and_then(|id| skills.data.get(id))
                            .map(|data| data.name.as_str())
                            .unwrap_or_default();
                        format!("[{id:0>4}: {name}] knows [{skill_id:0>4}: {skill_name}]")
                    }

                    3 => {
                        let weapon_id = command
                            .parameters
                            .get(3)
                            .and_then(|parameter| parameter.as_integer().copied())
                            .unwrap_or_default();
                        let weapons = update_state.data.weapons();
                        let weapon_name = weapon_id
                            .checked_sub(1)
                            .and_then(|id| usize::try_from(id).ok())
                            .and_then(|id| weapons.data.get(id))
                            .map(|data| data.name.as_str())
                            .unwrap_or_default();
                        format!("[{id:0>4}: {name}] is holding [{weapon_id:0>4}: {weapon_name}]")
                    }

                    4 => {
                        let armor_id = command
                            .parameters
                            .get(3)
                            .and_then(|parameter| parameter.as_integer().copied())
                            .unwrap_or_default();
                        let armors = update_state.data.armors();
                        let armor_name = armor_id
                            .checked_sub(1)
                            .and_then(|id| usize::try_from(id).ok())
                            .and_then(|id| armors.data.get(id))
                            .map(|data| data.name.as_str())
                            .unwrap_or_default();
                        format!("[{id:0>4}: {name}] is wearing [{armor_id:0>4}: {armor_name}]")
                    }

                    5 => {
                        let state_id = command
                            .parameters
                            .get(3)
                            .and_then(|parameter| parameter.as_integer().copied())
                            .unwrap_or_default();
                        let states = update_state.data.states();
                        let state_name = state_id
                            .checked_sub(1)
                            .and_then(|id| usize::try_from(id).ok())
                            .and_then(|id| states.data.get(id))
                            .map(|data| data.name.as_str())
                            .unwrap_or_default();
                        format!("[{id:0>4}: {name}] is affected by [{state_id:0>4}: {state_name}]")
                    }

                    _ => String::new(),
                }
            }

            5 => {
                let index = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                    .wrapping_add(1);
                match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => format!("enemy #{index} exists"),

                    1 => {
                        let state_id = command
                            .parameters
                            .get(3)
                            .and_then(|parameter| parameter.as_integer().copied())
                            .unwrap_or_default();
                        let states = update_state.data.states();
                        let state_name = state_id
                            .checked_sub(1)
                            .and_then(|id| usize::try_from(id).ok())
                            .and_then(|id| states.data.get(id))
                            .map(|data| data.name.as_str())
                            .unwrap_or_default();
                        format!("enemy #{index} is affected by [{state_id:0>4}: {state_name}]")
                    }

                    _ => String::new(),
                }
            }

            6 => {
                let id = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let event_string = match id {
                    -1 => "Player".into(),
                    0 => "This event".into(),
                    _ => {
                        if let Some(event_info) = event_info.copied() {
                            let map = update_state.data.get_map(event_info.map_id);
                            let name =
                                if usize::try_from(id).is_ok_and(|id| id == event_info.event_id) {
                                    event_info.event_name
                                } else {
                                    usize::try_from(id)
                                        .ok()
                                        .and_then(|id| map.events.get(id))
                                        .map(|data| data.name.as_str())
                                        .unwrap_or_default()
                                };
                            format!("[{id:0>4}: {name}]")
                        } else {
                            format!("[{id:0>4}]")
                        }
                    }
                };
                match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    2 => format!("{event_string} is facing down"),
                    4 => format!("{event_string} is facing left"),
                    6 => format!("{event_string} is facing right"),
                    8 => format!("{event_string} is facing up"),
                    _ => String::new(),
                }
            }

            7 => {
                let value = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let condition = match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => ">=",
                    1 => "<=",
                    _ => {
                        return String::new();
                    }
                };
                format!("Gold {condition} {value}")
            }

            8 => {
                let id = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let items = update_state.data.items();
                let name = id
                    .checked_sub(1)
                    .and_then(|id| usize::try_from(id).ok())
                    .and_then(|id| items.data.get(id))
                    .map(|data| data.name.as_str())
                    .unwrap_or_default();
                format!("[{id:0>4}: {name}] is in inventory")
            }

            9 => {
                let id = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let weapons = update_state.data.weapons();
                let name = id
                    .checked_sub(1)
                    .and_then(|id| usize::try_from(id).ok())
                    .and_then(|id| weapons.data.get(id))
                    .map(|data| data.name.as_str())
                    .unwrap_or_default();
                format!("[{id:0>4}: {name}] is in inventory")
            }

            10 => {
                let id = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let armors = update_state.data.armors();
                let name = id
                    .checked_sub(1)
                    .and_then(|id| usize::try_from(id).ok())
                    .and_then(|id| armors.data.get(id))
                    .map(|data| data.name.as_str())
                    .unwrap_or_default();
                format!("[{id:0>4}: {name}] is in inventory")
            }

            11 => {
                let id = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let button_type = ButtonType::try_from(id).unwrap_or_default();
                if button_type == ButtonType::Custom {
                    format!("Button {id} is being pressed")
                } else {
                    format!("{button_type} is being pressed")
                }
            }

            12 => callback.exponential_search_str(
                command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_string())
                    .map(|parameter| parameter.as_str())
                    .unwrap_or_default(),
            ),

            _ => String::new(),
        }
    }

    fn ui(
        &self,
        ui: &mut egui::Ui,
        stripe: &mut bool,
        update_state: &mut UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response {
        let terminator = command.sibling_commands.pop().unwrap();
        let mut modified = false;

        let is_custom_button_code = command.state.get_or_insert(false);

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                Collapsing::new(ui, stripe)
                    .expand_by_default(false)
                    .show_header(|ui| {
                        ui.label("Condition");
                    })
                    .body(|ui| {
                        ui.label("Condition type");

                        let branch_type = command.parameters[0].as_integer_mut().unwrap();
                        modified |= ui
                            .add(EnumComboBox::new_with_conversion(
                                PhantomData::<BranchType>,
                                "branch type",
                                branch_type,
                            ))
                            .changed();
                        let branch_type = *branch_type;

                        match branch_type {
                            0 => {
                                command.parameters.resize(3, ParameterType::None);

                                ui.label("Switch");

                                let switch = coerce_to_integer(&mut command.parameters[1]);
                                {
                                    let system = update_state.data.system();
                                    modified |= ui
                                        .add(OptionalIdComboBox::new(
                                            update_state,
                                            (1, 1),
                                            switch,
                                            1..=system.switches.len(),
                                            |id| {
                                                id.checked_sub(1)
                                                    .and_then(|id| system.switches.get(id))
                                                    .map_or_else(
                                                        || "".into(),
                                                        |x| format!("{:0>4}: {}", id, x),
                                                    )
                                            },
                                        ))
                                        .changed();
                                }

                                ui.label("Condition");

                                let condition = coerce_to_integer(&mut command.parameters[2]);
                                modified |= ui
                                    .add(EnumComboBox::new_with_conversion(
                                        PhantomData::<SwitchCondition>,
                                        (0, 2),
                                        condition,
                                    ))
                                    .changed();
                            }

                            1 => {
                                command.parameters.resize(5, ParameterType::None);

                                ui.label("Value 1");

                                let variable = coerce_to_integer(&mut command.parameters[1]);
                                {
                                    let system = update_state.data.system();
                                    modified |= ui
                                        .add(OptionalIdComboBox::new(
                                            update_state,
                                            (1, 1),
                                            variable,
                                            1..=system.variables.len(),
                                            |id| {
                                                id.checked_sub(1)
                                                    .and_then(|id| system.variables.get(id))
                                                    .map_or_else(
                                                        || "".into(),
                                                        |x| format!("{:0>4}: {}", id, x),
                                                    )
                                            },
                                        ))
                                        .changed();
                                };

                                ui.label("Value 2");

                                let value_type = coerce_to_integer(&mut command.parameters[2]);
                                modified |= ui
                                    .add(EnumComboBox::new_with_conversion(
                                        PhantomData::<VariableValueType>,
                                        (1, 2),
                                        value_type,
                                    ))
                                    .changed();
                                let value_type = *value_type;

                                let value = coerce_to_integer(&mut command.parameters[3]);
                                match value_type {
                                    1 => {
                                        let system = update_state.data.system();
                                        modified |= ui
                                            .add(OptionalIdComboBox::new(
                                                update_state,
                                                (1, 3),
                                                value,
                                                1..=system.variables.len(),
                                                |id| {
                                                    id.checked_sub(1)
                                                        .and_then(|id| system.variables.get(id))
                                                        .map_or_else(
                                                            || "".into(),
                                                            |x| format!("{:0>4}: {}", id, x),
                                                        )
                                                },
                                            ))
                                            .changed();
                                    }
                                    _ => {
                                        modified |= ui.add(egui::DragValue::new(value)).changed();
                                    }
                                }

                                ui.label("Condition");

                                let condition = coerce_to_integer(&mut command.parameters[4]);
                                modified |= ui
                                    .add(EnumComboBox::new_with_conversion(
                                        PhantomData::<VariableCondition>,
                                        (1, 4),
                                        condition,
                                    ))
                                    .changed();
                            }

                            2 => {
                                command.parameters.resize(3, ParameterType::None);

                                ui.label("Self switch");

                                let self_switch = coerce_to_string(&mut command.parameters[1], "A");
                                modified |= ui.text_edit_singleline(self_switch).changed();

                                ui.label("Condition");

                                let condition = coerce_to_integer(&mut command.parameters[2]);
                                modified |= ui
                                    .add(EnumComboBox::new_with_conversion(
                                        PhantomData::<SelfSwitchCondition>,
                                        (2, 2),
                                        condition,
                                    ))
                                    .changed();
                            }

                            3 => {
                                command.parameters.resize(3, ParameterType::None);

                                ui.label("Value");

                                let frames = coerce_to_integer(&mut command.parameters[1]);
                                modified |= ui
                                    .add(egui::DragValue::new(frames).range(0..=i32::MAX))
                                    .changed();

                                ui.label("Condition");

                                let condition = coerce_to_integer(&mut command.parameters[2]);
                                modified |= ui
                                    .add(EnumComboBox::new_with_conversion(
                                        PhantomData::<TimerCondition>,
                                        (3, 2),
                                        condition,
                                    ))
                                    .changed();
                            }

                            4 => {
                                if command.parameters.len() < 3 {
                                    command.parameters.resize(3, ParameterType::None);
                                }

                                ui.label("Actor");

                                let actor = coerce_to_integer(&mut command.parameters[1]);
                                {
                                    let actors = update_state.data.actors();
                                    modified |= ui
                                        .add(OptionalIdComboBox::new(
                                            update_state,
                                            (4, 1),
                                            actor,
                                            1..=actors.data.len(),
                                            |id| {
                                                id.checked_sub(1)
                                                    .and_then(|id| actors.data.get(id))
                                                    .map_or_else(
                                                        || "".into(),
                                                        |x| format!("{:0>4}: {}", id, x.name),
                                                    )
                                            },
                                        ))
                                        .changed();
                                };

                                ui.label("Condition");

                                let condition = coerce_to_integer(&mut command.parameters[2]);
                                modified |= ui
                                    .add(EnumComboBox::new_with_conversion(
                                        PhantomData::<ActorCondition>,
                                        (4, 2),
                                        condition,
                                    ))
                                    .changed();
                                let condition = *condition;

                                match condition {
                                    0 => {
                                        command.parameters.resize(3, ParameterType::None);
                                    }

                                    1 => {
                                        command.parameters.resize(4, ParameterType::None);
                                        let name = coerce_to_string(&mut command.parameters[3], "");
                                        modified |= ui.text_edit_singleline(name).changed();
                                    }

                                    2 => {
                                        command.parameters.resize(4, ParameterType::None);
                                        let skill = coerce_to_integer(&mut command.parameters[3]);
                                        {
                                            let skills = update_state.data.skills();
                                            modified |= ui
                                                .add(OptionalIdComboBox::new(
                                                    update_state,
                                                    (4, 3, 2),
                                                    skill,
                                                    1..=skills.data.len(),
                                                    |id| {
                                                        id.checked_sub(1)
                                                            .and_then(|id| skills.data.get(id))
                                                            .map_or_else(
                                                                || "".into(),
                                                                |x| {
                                                                    format!(
                                                                        "{:0>4}: {}",
                                                                        id, x.name
                                                                    )
                                                                },
                                                            )
                                                    },
                                                ))
                                                .changed();
                                        };
                                    }

                                    3 => {
                                        command.parameters.resize(4, ParameterType::None);
                                        let weapon = coerce_to_integer(&mut command.parameters[3]);
                                        {
                                            let weapons = update_state.data.weapons();
                                            modified |= ui
                                                .add(OptionalIdComboBox::new(
                                                    update_state,
                                                    (4, 3, 3),
                                                    weapon,
                                                    1..=weapons.data.len(),
                                                    |id| {
                                                        id.checked_sub(1)
                                                            .and_then(|id| weapons.data.get(id))
                                                            .map_or_else(
                                                                || "".into(),
                                                                |x| {
                                                                    format!(
                                                                        "{:0>4}: {}",
                                                                        id, x.name
                                                                    )
                                                                },
                                                            )
                                                    },
                                                ))
                                                .changed();
                                        };
                                    }

                                    4 => {
                                        command.parameters.resize(4, ParameterType::None);
                                        let armor = coerce_to_integer(&mut command.parameters[3]);
                                        {
                                            let armors = update_state.data.armors();
                                            modified |= ui
                                                .add(OptionalIdComboBox::new(
                                                    update_state,
                                                    (4, 3, 4),
                                                    armor,
                                                    1..=armors.data.len(),
                                                    |id| {
                                                        id.checked_sub(1)
                                                            .and_then(|id| armors.data.get(id))
                                                            .map_or_else(
                                                                || "".into(),
                                                                |x| {
                                                                    format!(
                                                                        "{:0>4}: {}",
                                                                        id, x.name
                                                                    )
                                                                },
                                                            )
                                                    },
                                                ))
                                                .changed();
                                        };
                                    }

                                    5 => {
                                        command.parameters.resize(4, ParameterType::None);
                                        let state = coerce_to_integer(&mut command.parameters[3]);
                                        {
                                            let states = update_state.data.states();
                                            modified |= ui
                                                .add(OptionalIdComboBox::new(
                                                    update_state,
                                                    (4, 3, 5),
                                                    state,
                                                    1..=states.data.len(),
                                                    |id| {
                                                        id.checked_sub(1)
                                                            .and_then(|id| states.data.get(id))
                                                            .map_or_else(
                                                                || "".into(),
                                                                |x| {
                                                                    format!(
                                                                        "{:0>4}: {}",
                                                                        id, x.name
                                                                    )
                                                                },
                                                            )
                                                    },
                                                ))
                                                .changed();
                                        };
                                    }

                                    _ => unreachable!(),
                                }
                            }

                            5 => {
                                if command.parameters.len() < 3 {
                                    command.parameters.resize(3, ParameterType::None);
                                }
                                let condition = *coerce_to_integer(&mut command.parameters[2]);
                                command.parameters.resize(
                                    if condition != 0 { 4 } else { 3 },
                                    ParameterType::None,
                                );

                                ui.label("Enemy");

                                let enemy = coerce_to_integer(&mut command.parameters[1]);
                                *enemy = enemy.wrapping_add(1);
                                modified |= ui
                                    .add(
                                        egui::DragValue::new(enemy)
                                            .range(1..=i32::MAX)
                                            .custom_formatter(|value, _| {
                                                format!("#{}", (value as i32))
                                            }),
                                    )
                                    .changed();
                                *enemy = enemy.wrapping_sub(1);

                                ui.label("Condition");

                                let condition = coerce_to_integer(&mut command.parameters[2]);
                                modified |= ui
                                    .add(EnumComboBox::new_with_conversion(
                                        PhantomData::<EnemyCondition>,
                                        (5, 2),
                                        condition,
                                    ))
                                    .changed();
                                let condition = *condition;

                                match condition {
                                    0 => {
                                        command.parameters.resize(3, ParameterType::None);
                                    }

                                    1 => {
                                        command.parameters.resize(4, ParameterType::None);
                                        let state = coerce_to_integer(&mut command.parameters[3]);
                                        {
                                            let states = update_state.data.states();
                                            modified |= ui
                                                .add(OptionalIdComboBox::new(
                                                    update_state,
                                                    (5, 3, 1),
                                                    state,
                                                    1..=states.data.len(),
                                                    |id| {
                                                        id.checked_sub(1)
                                                            .and_then(|id| states.data.get(id))
                                                            .map_or_else(
                                                                || "".into(),
                                                                |x| {
                                                                    format!(
                                                                        "{:0>4}: {}",
                                                                        id, x.name
                                                                    )
                                                                },
                                                            )
                                                    },
                                                ))
                                                .changed();
                                        };
                                    }

                                    _ => unreachable!(),
                                }
                            }

                            6 => {
                                command.parameters.resize(3, ParameterType::None);

                                ui.label("Character");

                                let character = coerce_to_integer(&mut command.parameters[1]);
                                if let Some(event_info) = event_info.copied() {
                                    *character = character.wrapping_add(1);
                                    let map = update_state.data.get_map(event_info.map_id);
                                    modified |= ui
                                        .add(OptionalIdComboBox::new(
                                            update_state,
                                            (6, 1, true),
                                            character,
                                            0..=map.events.len(),
                                            |id| match id {
                                                0 => "Player".into(),
                                                1 => "This event".into(),
                                                _ => {
                                                    let id = id - 1;
                                                    if id == event_info.event_id {
                                                        format!(
                                                            "{:0>4}: {}",
                                                            id, event_info.event_name
                                                        )
                                                    } else {
                                                        map.events.get(id).map_or_else(
                                                            || "".into(),
                                                            |x| format!("{:0>4}: {}", id, x.name),
                                                        )
                                                    }
                                                }
                                            },
                                        ))
                                        .changed();
                                    *character = character.wrapping_sub(1);
                                } else {
                                    let mut character_type =
                                        CharacterType::try_from(*character).unwrap_or_default();
                                    modified |= {
                                        let changed = ui
                                            .add(EnumComboBox::new(
                                                (6, 1, false),
                                                &mut character_type,
                                            ))
                                            .changed();
                                        if changed {
                                            *character = character_type.into();
                                        }
                                        changed
                                    };
                                    if character_type == CharacterType::MapEvent {
                                        modified |= ui
                                            .add(
                                                egui::DragValue::new(character).range(1..=i32::MAX),
                                            )
                                            .changed();
                                    }
                                };

                                ui.label("Condition");

                                let condition = coerce_to_integer(&mut command.parameters[2]);
                                modified |= ui
                                    .add(EnumComboBox::new_with_conversion(
                                        PhantomData::<CharacterCondition>,
                                        (6, 2),
                                        condition,
                                    ))
                                    .changed();
                            }

                            7 => {
                                command.parameters.resize(3, ParameterType::None);

                                ui.label("Value");

                                let frames = coerce_to_integer(&mut command.parameters[1]);
                                modified |= ui
                                    .add(egui::DragValue::new(frames).range(0..=i32::MAX))
                                    .changed();

                                ui.label("Condition");

                                let condition = coerce_to_integer(&mut command.parameters[2]);
                                modified |= ui
                                    .add(EnumComboBox::new_with_conversion(
                                        PhantomData::<GoldCondition>,
                                        (7, 2),
                                        condition,
                                    ))
                                    .changed();
                            }

                            8 => {
                                command.parameters.resize(2, ParameterType::None);

                                let item = coerce_to_integer(&mut command.parameters[1]);
                                {
                                    let items = update_state.data.items();
                                    modified |= ui
                                        .add(OptionalIdComboBox::new(
                                            update_state,
                                            (4, 1),
                                            item,
                                            1..=items.data.len(),
                                            |id| {
                                                id.checked_sub(1)
                                                    .and_then(|id| items.data.get(id))
                                                    .map_or_else(
                                                        || "".into(),
                                                        |x| format!("{:0>4}: {}", id, x.name),
                                                    )
                                            },
                                        ))
                                        .changed();
                                };
                            }

                            9 => {
                                command.parameters.resize(2, ParameterType::None);

                                let weapon = coerce_to_integer(&mut command.parameters[1]);
                                {
                                    let weapons = update_state.data.weapons();
                                    modified |= ui
                                        .add(OptionalIdComboBox::new(
                                            update_state,
                                            (4, 1),
                                            weapon,
                                            1..=weapons.data.len(),
                                            |id| {
                                                id.checked_sub(1)
                                                    .and_then(|id| weapons.data.get(id))
                                                    .map_or_else(
                                                        || "".into(),
                                                        |x| format!("{:0>4}: {}", id, x.name),
                                                    )
                                            },
                                        ))
                                        .changed();
                                };
                            }

                            10 => {
                                command.parameters.resize(2, ParameterType::None);

                                let armor = coerce_to_integer(&mut command.parameters[1]);
                                {
                                    let armors = update_state.data.armors();
                                    modified |= ui
                                        .add(OptionalIdComboBox::new(
                                            update_state,
                                            (4, 1),
                                            armor,
                                            1..=armors.data.len(),
                                            |id| {
                                                id.checked_sub(1)
                                                    .and_then(|id| armors.data.get(id))
                                                    .map_or_else(
                                                        || "".into(),
                                                        |x| format!("{:0>4}: {}", id, x.name),
                                                    )
                                            },
                                        ))
                                        .changed();
                                };
                            }

                            11 => {
                                command.parameters.resize(2, ParameterType::None);

                                let button = coerce_to_integer(&mut command.parameters[1]);
                                let mut button_type = if *is_custom_button_code {
                                    ButtonType::Custom
                                } else {
                                    ButtonType::try_from(*button).unwrap_or_default()
                                };
                                modified |= {
                                    let changed = ui
                                        .add(EnumComboBox::new((11, 1), &mut button_type))
                                        .changed();
                                    if changed {
                                        *button = button_type.into();
                                    }
                                    changed
                                };

                                *is_custom_button_code = button_type == ButtonType::Custom;
                                if *is_custom_button_code {
                                    modified |= ui.add(egui::DragValue::new(button)).changed();
                                }
                            }

                            12 => {
                                command.parameters.resize(2, ParameterType::None);

                                let script = coerce_to_string(&mut command.parameters[1], "");
                                modified |= ui.text_edit_multiline(script).changed();
                            }

                            _ => {}
                        }
                    });

                Collapsing::new(ui, stripe)
                    .id_salt(true)
                    .show_header(|ui| {
                        ui.label("If condition is true");
                    })
                    .body(|ui| {
                        modified |= ui
                            .add(
                                CommandView::new(
                                    update_state,
                                    event_info,
                                    &mut command.child_commands,
                                )
                                .with_stripe(stripe),
                            )
                            .changed();
                    });

                Collapsing::new(ui, stripe)
                    .id_salt(false)
                    .show_header(|ui| {
                        ui.label("If condition is false");
                    })
                    .body(|ui| {
                        let mut if_false_fallback = Vec::new();
                        let if_false = command
                            .sibling_commands
                            .first_mut()
                            .map_or(&mut if_false_fallback, |sibling| {
                                &mut sibling.child_commands
                            });
                        modified |= ui
                            .add(
                                CommandView::new(update_state, event_info, if_false)
                                    .with_stripe(stripe),
                            )
                            .changed();
                    });
            })
            .response;

        command.sibling_commands.push(terminator);
        if modified {
            response.mark_changed();
        }
        response
    }
}
