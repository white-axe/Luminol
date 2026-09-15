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

pub mod schemas;
pub use schemas::SCHEMAS;

pub mod indented_iter;
use indented_iter::EventCommandListIndentedIter;

pub mod state;
use state::EventCommandState;

mod de;
mod ser;

use crate::{id_alox, id_serde, rpg::MoveRoute, BlendMode, ParameterType, Path, RpgOption};
use rand::Rng;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event")]
pub struct Event {
    // #[serde(with = "id_serde")]
    // #[marshal(with = "id_alox")]
    pub id: usize,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub pages: Vec<EventPage>,

    #[serde(skip)]
    #[marshal(skip)]
    pub extra_data: EventExtraData,
}

#[derive(Debug, Default, Clone)]
pub struct EventExtraData {
    /// Whether or not the event editor for this event is open
    pub is_editor_open: bool,
    pub graphic_modified: std::cell::Cell<bool>,
}

impl Event {
    #[must_use]
    pub fn new(x: i32, y: i32, id: usize) -> Self {
        Self {
            id,
            name: format!("EV{id:0>3}"),
            x,
            y,
            pages: vec![EventPage::default()],

            extra_data: EventExtraData::default(),
        }
    }
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::CommonEvent")]
pub struct CommonEvent {
    #[serde(with = "id_serde")]
    #[marshal(with = "id_alox")]
    pub id: usize,
    pub name: String,
    pub trigger: usize,
    pub switch_id: usize,
    pub list: EventCommandList,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event::Page")]
pub struct EventPage {
    pub condition: EventCondition,
    pub graphic: Graphic,
    pub move_type: MoveType,
    pub move_speed: MoveSpeed,
    pub move_frequency: MoveFreq,
    pub move_route: MoveRoute,
    pub walk_anime: bool,
    pub step_anime: bool,
    pub direction_fix: bool,
    pub through: bool,
    pub always_on_top: bool,
    pub trigger: EventTrigger,
    pub list: EventCommandList,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum EventTrigger {
    #[strum(to_string = "Action Button")]
    ActionButton,
    #[strum(to_string = "Player Touch")]
    PlayerTouch,
    #[strum(to_string = "Event Touch")]
    EventTouch,
    #[strum(to_string = "Autorun")]
    Autorun,
    #[strum(to_string = "Parallel Process")]
    Parallel,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum MoveType {
    Fixed,
    Random,
    Approach,
    Custom,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum MoveFreq {
    Lowest = 1,
    Lower,
    Low,
    High,
    Higher,
    Highest,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(try_from = "u8", into = "u8")]
#[marshal(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum MoveSpeed {
    Slowest = 1,
    Slower,
    Slow,
    Fast,
    Faster,
    Fastest,
}

impl Default for EventPage {
    fn default() -> Self {
        Self {
            condition: EventCondition::default(),
            graphic: Graphic::default(),
            move_type: MoveType::Fixed,
            move_speed: MoveSpeed::Slow,
            move_frequency: MoveFreq::Low,
            move_route: MoveRoute::default(),
            walk_anime: true,
            step_anime: false,
            direction_fix: false,
            through: false,
            always_on_top: false,
            trigger: EventTrigger::ActionButton,
            list: Default::default(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event::Page::Graphic")]
pub struct Graphic {
    pub tile_id: RpgOption<usize>,
    pub character_name: Path,
    pub character_hue: i32,
    pub direction: i32,
    pub pattern: i32,
    pub opacity: i32,
    pub blend_type: BlendMode,
}

impl Default for Graphic {
    fn default() -> Self {
        Self {
            tile_id: None.into(),
            character_name: None.into(),
            character_hue: 0,
            direction: 2,
            pattern: 0,
            opacity: 255,
            blend_type: BlendMode::Normal,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::Event::Page::Condition")]
pub struct EventCondition {
    pub switch1_valid: bool,
    pub switch2_valid: bool,
    pub variable_valid: bool,
    pub self_switch_valid: bool,
    #[serde(with = "id_serde")]
    #[marshal(with = "id_alox")]
    pub switch1_id: usize,
    #[serde(with = "id_serde")]
    #[marshal(with = "id_alox")]
    pub switch2_id: usize,
    #[serde(with = "id_serde")]
    #[marshal(with = "id_alox")]
    pub variable_id: usize,
    pub variable_value: i32,
    pub self_switch_ch: SelfSwitch,
}

impl Default for EventCondition {
    fn default() -> Self {
        Self {
            switch1_valid: false,
            switch2_valid: false,
            variable_valid: false,
            self_switch_valid: false,
            switch1_id: 0,
            switch2_id: 0,
            variable_id: 0,
            variable_value: 0,
            self_switch_ch: SelfSwitch::A,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(strum::Display, strum::EnumIter)]
#[serde(from = "String", into = "String")]
#[marshal(from = "String", into = "String")]
pub enum SelfSwitch {
    A,
    B,
    C,
    D,
}

impl From<String> for SelfSwitch {
    fn from(value: String) -> Self {
        match value.as_str() {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            _ => panic!("wrong value for self switch"),
        }
    }
}

impl From<SelfSwitch> for String {
    fn from(val: SelfSwitch) -> Self {
        match val {
            SelfSwitch::A => "A".to_string(),
            SelfSwitch::B => "B".to_string(),
            SelfSwitch::C => "C".to_string(),
            SelfSwitch::D => "D".to_string(),
        }
    }
}

#[derive(Debug)]
#[allow(missing_docs)]
pub struct EventCommand {
    pub guid: String,
    pub state: EventCommandState,
    pub matches_schema: bool,
    pub code: u16,
    pub parameters: Vec<ParameterType>,
    pub child_commands: Vec<EventCommand>,
    pub sibling_commands: Vec<EventCommand>,
}

impl EventCommand {
    fn generate_guid() -> String {
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(42) // This should be enough to avoid collisions
            .map(char::from)
            .collect()
    }
}

impl Default for EventCommand {
    fn default() -> Self {
        Self {
            guid: Self::generate_guid(),
            state: EventCommandState::default(),
            matches_schema: false,
            code: 0,
            parameters: Vec::new(),
            child_commands: Vec::new(),
            sibling_commands: Vec::new(),
        }
    }
}

impl Clone for EventCommand {
    fn clone(&self) -> Self {
        Self {
            guid: Self::generate_guid(),
            state: EventCommandState::default(),
            matches_schema: self.matches_schema,
            code: self.code,
            parameters: self.parameters.clone(),
            child_commands: self.child_commands.clone(),
            sibling_commands: self.sibling_commands.clone(),
        }
    }
}

/// A borrowed instance of [`EventCommand`] with an indentation.
#[derive(Clone, Copy)]
pub struct IndentedEventCommandRef<'a> {
    /// The indentation of the `command` field.
    pub indent: u64,
    /// The inner event command.
    pub command: &'a EventCommand,
}

/// An owned instance of [`EventCommand`] with an indentation.
#[derive(Default)]
pub struct IndentedEventCommand {
    /// The indentation of the `command` field.
    pub indent: u64,
    /// The inner event command.
    pub command: EventCommand,
}

impl IndentedEventCommand {
    /// Borrows this object as an [`IndentedEventCommandRef`].
    pub fn as_ref(&self) -> IndentedEventCommandRef<'_> {
        self.into()
    }
}

impl<'a> From<&'a IndentedEventCommand> for IndentedEventCommandRef<'a> {
    fn from(value: &'a IndentedEventCommand) -> Self {
        IndentedEventCommandRef {
            indent: value.indent,
            command: &value.command,
        }
    }
}

/// A wrapper around [`Vec<EventCommand>`] that supports serialization and deserialization.
#[derive(Debug, Default, Clone)]
pub struct EventCommandList {
    /// The inner event commands.
    pub commands: Vec<EventCommand>,
}

impl EventCommandList {
    /// Returns a recursive iterator over the event commands.
    pub fn indented_iter(&self) -> EventCommandListIndentedIter<'_> {
        self.into()
    }
}
