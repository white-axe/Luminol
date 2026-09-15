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
    ActorSelection, ArmorSelection, CharacterSelection, Collapsing, CommandView,
    DescriptionWidthCallback, EventCommand, EventCommandEditor, EventInfo, ItemSelection,
    ParameterType, SkillSelection, StateSelection, SwitchSelection, UpdateState, ValueFmt,
    ValueSelection, VariableSelection, WeaponSelection,
};
use crate::components::EnumComboBox;
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
enum BranchType {
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
enum SwitchCondition {
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
enum VariableCondition {
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

#[derive(PartialEq, Eq, strum::Display, strum::EnumIter)]
enum SelfSwitchType {
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
enum SelfSwitchCondition {
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
enum TimerCondition {
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
enum ActorCondition {
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
enum EnemyCondition {
    #[strum(to_string = "Enemy exists")]
    Exists = 0,
    #[strum(to_string = "Enemy is affected by a state")]
    State = 1,
}

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum CharacterCondition {
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
enum GoldCondition {
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
enum ButtonType {
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
    ) -> Option<String> {
        match *command.parameters[0].as_integer().unwrap() {
            0 => {
                let switch = SwitchSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(1)
                        .and_then(|parameter| parameter.as_integer().copied())
                        .unwrap_or_default(),
                )
                .fmt()?;
                let condition = match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => Some("is on"),
                    1 => Some("is off"),
                    _ => None,
                }?;
                Some(format!("[{switch}] {condition}"))
            }

            1 => {
                let variable = VariableSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(1)
                        .and_then(|parameter| parameter.as_integer().copied())
                        .unwrap_or_default(),
                )
                .fmt()?;
                let condition = match command
                    .parameters
                    .get(4)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => Some("=="),
                    1 => Some(">="),
                    2 => Some("<="),
                    3 => Some(">"),
                    4 => Some("<"),
                    5 => Some("!="),
                    _ => None,
                }?;
                let value = ValueSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(2)
                        .and_then(|parameter| parameter.as_integer().copied())
                        .unwrap_or_default(),
                    command
                        .parameters
                        .get(3)
                        .and_then(|parameter| parameter.as_integer().copied())
                        .unwrap_or_default(),
                )
                .fmt()?;
                match value {
                    ValueFmt::Constant(constant) => {
                        Some(format!("[{variable}] {condition} {constant}"))
                    }
                    ValueFmt::Variable(variable2) => {
                        Some(format!("[{variable}] {condition} [{variable2}]"))
                    }
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
                    0 => Some("is on"),
                    1 => Some("is off"),
                    _ => None,
                }?;
                Some(format!("[{name}] {condition}"))
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
                    0 => Some(">="),
                    1 => Some("<="),
                    _ => None,
                }?;
                Some(format!("Timer {condition} {value}"))
            }

            4 => {
                let actor = ActorSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(1)
                        .and_then(|parameter| parameter.as_integer().copied())
                        .unwrap_or_default(),
                )
                .fmt()?;
                match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    0 => Some(format!("[{actor}] is in the party")),

                    1 => {
                        let actor_name = command
                            .parameters
                            .get(3)
                            .and_then(|parameter| parameter.as_string())
                            .map(|parameter| parameter.as_str())
                            .unwrap_or_default();
                        Some(format!("[{actor}] is named {actor_name}"))
                    }

                    2 => {
                        let skill = SkillSelection::new(
                            update_state,
                            command
                                .parameters
                                .get(3)
                                .and_then(|parameter| parameter.as_integer().copied())
                                .unwrap_or_default(),
                        )
                        .fmt()?;
                        Some(format!("[{actor}] knows [{skill}]"))
                    }

                    3 => {
                        let weapon = WeaponSelection::new(
                            update_state,
                            command
                                .parameters
                                .get(3)
                                .and_then(|parameter| parameter.as_integer().copied())
                                .unwrap_or_default(),
                        )
                        .fmt()?;
                        Some(format!("[{actor}] is holding [{weapon}]"))
                    }

                    4 => {
                        let armor = ArmorSelection::new(
                            update_state,
                            command
                                .parameters
                                .get(3)
                                .and_then(|parameter| parameter.as_integer().copied())
                                .unwrap_or_default(),
                        )
                        .fmt()?;
                        Some(format!("[{actor}] is wearing [{armor}]"))
                    }

                    5 => {
                        let state = StateSelection::new(
                            update_state,
                            command
                                .parameters
                                .get(3)
                                .and_then(|parameter| parameter.as_integer().copied())
                                .unwrap_or_default(),
                        )
                        .fmt()?;
                        Some(format!("[{actor}] is affected by [{state}]"))
                    }

                    _ => None,
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
                    0 => Some(format!("enemy #{index} exists")),

                    1 => {
                        let state = StateSelection::new(
                            update_state,
                            command
                                .parameters
                                .get(3)
                                .and_then(|parameter| parameter.as_integer().copied())
                                .unwrap_or_default(),
                        )
                        .fmt()?;
                        Some(format!("enemy #{index} is affected by [{state}]"))
                    }

                    _ => None,
                }
            }

            6 => {
                let character = CharacterSelection::new(
                    update_state,
                    event_info,
                    command
                        .parameters
                        .get(1)
                        .and_then(|parameter| parameter.as_integer().copied())
                        .unwrap_or_default(),
                )
                .fmt()?;
                match command
                    .parameters
                    .get(2)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default()
                {
                    2 => Some(format!("[{character}] is facing down")),
                    4 => Some(format!("[{character}] is facing left")),
                    6 => Some(format!("[{character}] is facing right")),
                    8 => Some(format!("[{character}] is facing up")),
                    _ => None,
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
                    0 => Some(">="),
                    1 => Some("<="),
                    _ => None,
                }?;
                Some(format!("Gold {condition} {value}"))
            }

            8 => {
                let item = ItemSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(1)
                        .and_then(|parameter| parameter.as_integer().copied())
                        .unwrap_or_default(),
                )
                .fmt()?;
                Some(format!("[{item}] is in inventory"))
            }

            9 => {
                let weapon = WeaponSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(1)
                        .and_then(|parameter| parameter.as_integer().copied())
                        .unwrap_or_default(),
                )
                .fmt()?;
                Some(format!("[{weapon}] is in inventory"))
            }

            10 => {
                let armor = ArmorSelection::new(
                    update_state,
                    command
                        .parameters
                        .get(1)
                        .and_then(|parameter| parameter.as_integer().copied())
                        .unwrap_or_default(),
                )
                .fmt()?;
                Some(format!("[{armor}] is in inventory"))
            }

            11 => {
                let id = command
                    .parameters
                    .get(1)
                    .and_then(|parameter| parameter.as_integer().copied())
                    .unwrap_or_default();
                let button_type = ButtonType::try_from(id).unwrap_or_default();
                if button_type == ButtonType::Custom {
                    Some(format!("Button {id} is being pressed"))
                } else {
                    Some(format!("{button_type} is being pressed"))
                }
            }

            12 => Some(
                callback.exponential_search_str(
                    command
                        .parameters
                        .get(1)
                        .and_then(|parameter| parameter.as_string())
                        .map(|parameter| parameter.as_str())
                        .unwrap_or_default(),
                ),
            ),

            _ => None,
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

        let (is_custom_self_switch, is_custom_button_code) =
            command.state.get_or_insert((false, false));

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                Collapsing::new(ui, stripe)
                    .expand_by_default(false)
                    .show_header_text("Condition")
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

                                modified |= ui
                                    .add(
                                        SwitchSelection::new(
                                            update_state,
                                            &mut command.parameters[1],
                                        )
                                        .id_salt((0, 1)),
                                    )
                                    .changed();

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

                                modified |= ui
                                    .add(
                                        VariableSelection::new(
                                            update_state,
                                            &mut command.parameters[1],
                                        )
                                        .id_salt((1, 1)),
                                    )
                                    .changed();

                                ui.label("Value 2");

                                let [discriminant, value] =
                                    command.parameters[2..].first_chunk_mut().unwrap();
                                modified |= ui
                                    .add(
                                        ValueSelection::new(update_state, discriminant, value)
                                            .id_salt((1, 2)),
                                    )
                                    .changed();

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

                                modified |= ui
                                    .add(
                                        ActorSelection::new(
                                            update_state,
                                            &mut command.parameters[1],
                                        )
                                        .id_salt((4, 1)),
                                    )
                                    .changed();

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
                                        modified |= ui
                                            .add(
                                                SkillSelection::new(
                                                    update_state,
                                                    &mut command.parameters[3],
                                                )
                                                .id_salt((4, 3, 2)),
                                            )
                                            .changed();
                                    }

                                    3 => {
                                        command.parameters.resize(4, ParameterType::None);
                                        modified |= ui
                                            .add(
                                                WeaponSelection::new(
                                                    update_state,
                                                    &mut command.parameters[3],
                                                )
                                                .id_salt((4, 3, 3)),
                                            )
                                            .changed();
                                    }

                                    4 => {
                                        command.parameters.resize(4, ParameterType::None);
                                        modified |= ui
                                            .add(
                                                ArmorSelection::new(
                                                    update_state,
                                                    &mut command.parameters[3],
                                                )
                                                .id_salt((4, 3, 4)),
                                            )
                                            .changed();
                                    }

                                    5 => {
                                        command.parameters.resize(4, ParameterType::None);
                                        modified |= ui
                                            .add(
                                                StateSelection::new(
                                                    update_state,
                                                    &mut command.parameters[3],
                                                )
                                                .id_salt((4, 3, 5)),
                                            )
                                            .changed();
                                    }

                                    _ => {}
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
                                        modified |= ui
                                            .add(
                                                StateSelection::new(
                                                    update_state,
                                                    &mut command.parameters[3],
                                                )
                                                .id_salt((5, 3, 1)),
                                            )
                                            .changed();
                                    }

                                    _ => {}
                                }
                            }

                            6 => {
                                command.parameters.resize(3, ParameterType::None);

                                ui.label("Character");

                                modified |= ui
                                    .add(
                                        CharacterSelection::new(
                                            update_state,
                                            event_info,
                                            &mut command.parameters[1],
                                        )
                                        .id_salt((6, 1)),
                                    )
                                    .changed();

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

                                modified |= ui
                                    .add(
                                        ItemSelection::new(
                                            update_state,
                                            &mut command.parameters[1],
                                        )
                                        .id_salt((8, 1)),
                                    )
                                    .changed();
                            }

                            9 => {
                                command.parameters.resize(2, ParameterType::None);

                                modified |= ui
                                    .add(
                                        WeaponSelection::new(
                                            update_state,
                                            &mut command.parameters[1],
                                        )
                                        .id_salt((9, 1)),
                                    )
                                    .changed();
                            }

                            10 => {
                                command.parameters.resize(2, ParameterType::None);

                                modified |= ui
                                    .add(
                                        ArmorSelection::new(
                                            update_state,
                                            &mut command.parameters[1],
                                        )
                                        .id_salt((10, 1)),
                                    )
                                    .changed();
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
                    .show_header_text("If condition is true")
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
                    .show_header_text("If condition is false")
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
