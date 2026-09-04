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
use crate::{id_alox, id_serde, rpg::MoveRoute, BlendMode, ParameterType, Path, RpgOption};
use alox_48::{SerializeArray, SerializeIvars};
use rand::Rng;
use serde::ser::{SerializeMap, SerializeSeq};

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
    pub matches_schema: bool,
    pub code: u16,
    pub parameters: Vec<ParameterType>,
    pub child_commands: Vec<EventCommand>,
    pub sibling_commands: Vec<EventCommand>,
}

impl EventCommand {
    const fn new() -> Self {
        Self {
            guid: String::new(),
            matches_schema: false,
            code: 0,
            parameters: Vec::new(),
            child_commands: Vec::new(),
            sibling_commands: Vec::new(),
        }
    }

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
            ..Self::new()
        }
    }
}

impl Clone for EventCommand {
    fn clone(&self) -> Self {
        Self {
            guid: Self::generate_guid(),
            matches_schema: self.matches_schema,
            code: self.code,
            parameters: self.parameters.clone(),
            child_commands: self.child_commands.clone(),
            sibling_commands: self.sibling_commands.clone(),
        }
    }
}

static EVENT_COMMAND_TERMINATOR: EventCommand = EventCommand::new();

#[derive(Default)]
struct IndentedEventCommand {
    indent: u64,
    command: EventCommand,
}

struct IndentedEventCommandRef<'a> {
    indent: u64,
    command: &'a EventCommand,
}

#[derive(Debug, Default, Clone)]
pub struct EventCommandList {
    pub commands: Vec<EventCommand>,
}

struct EventCommandListIndentedIter<'a> {
    stack: Vec<(
        std::slice::Iter<'a, EventCommand>,
        std::slice::Iter<'a, EventCommand>,
        bool,
        bool,
    )>,
    indent: u64,
    is_terminated: bool,
}

struct EventCommandListVisitor;

struct IndentedEventCommandVisitor;

impl EventCommandList {
    fn indented_iter(&self) -> EventCommandListIndentedIter<'_> {
        EventCommandListIndentedIter {
            stack: vec![([].iter(), self.commands.iter(), false, false)],
            indent: 0,
            is_terminated: false,
        }
    }
}

impl<'a> Iterator for EventCommandListIndentedIter<'a> {
    type Item = IndentedEventCommandRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((child_iter, sibling_iter, is_indented, is_terminated)) =
            self.stack.last_mut()
        {
            if let Some(command) = {
                if let Some(command) = child_iter.next() {
                    if !*is_indented {
                        *is_indented = true;
                        self.indent += 1;
                    }
                    Some(command)
                } else {
                    if *is_indented {
                        if !*is_terminated {
                            *is_terminated = true;
                            return Some(IndentedEventCommandRef {
                                indent: self.indent,
                                command: &EVENT_COMMAND_TERMINATOR,
                            });
                        }
                        *is_indented = false;
                        self.indent -= 1;
                    }
                    sibling_iter.next()
                }
            } {
                self.stack.push((
                    command.child_commands.iter(),
                    command.sibling_commands.iter(),
                    false,
                    false,
                ));
                return Some(IndentedEventCommandRef {
                    indent: self.indent,
                    command,
                });
            } else {
                self.stack.pop();
            }
        }
        if !self.is_terminated {
            self.is_terminated = true;
            Some(IndentedEventCommandRef {
                indent: self.indent,
                command: &EVENT_COMMAND_TERMINATOR,
            })
        } else {
            None
        }
    }
}

impl std::iter::FusedIterator for EventCommandListIndentedIter<'_> {}

impl serde::Serialize for EventCommandList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.indented_iter().count()))?;
        for indented_command in self.indented_iter() {
            seq.serialize_element(&indented_command)?;
        }
        seq.end()
    }
}

impl alox_48::Serialize for EventCommandList {
    fn serialize<S>(&self, serializer: S) -> alox_48::SerResult<S::Ok>
    where
        S: alox_48::SerializerTrait,
    {
        let mut seq = serializer.serialize_array(self.indented_iter().count())?;
        for indented_command in self.indented_iter() {
            seq.serialize_element(&indented_command)?;
        }
        seq.end()
    }
}

impl serde::Serialize for IndentedEventCommandRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("code", &self.command.code)?;
        map.serialize_entry("indent", &self.indent)?;
        map.serialize_entry("parameters", &self.command.parameters)?;
        map.end()
    }
}

impl alox_48::Serialize for IndentedEventCommandRef<'_> {
    fn serialize<S>(&self, serializer: S) -> alox_48::SerResult<S::Ok>
    where
        S: alox_48::SerializerTrait,
    {
        let mut map = serializer.serialize_object("RPG::EventCommand".into(), 3)?;
        map.serialize_entry("@code".into(), &self.command.code)?;
        map.serialize_entry("@indent".into(), &self.indent)?;
        map.serialize_entry("@parameters".into(), &self.command.parameters)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for EventCommandList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(EventCommandListVisitor)
    }
}

impl<'de> alox_48::Deserialize<'de> for EventCommandList {
    fn deserialize<D>(deserializer: D) -> alox_48::DeResult<Self>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        deserializer.deserialize(EventCommandListVisitor)
    }
}

fn push_child_command(child_commands: &mut Vec<EventCommand>, mut child_command: EventCommand) {
    if let Some(child_command_last_child_command) = child_command.child_commands.last_mut() {
        child_command_last_child_command.matches_schema = super::command::SCHEMAS
            .get(&child_command_last_child_command.code)
            .is_some_and(|schema| schema.matches(child_command_last_child_command));
    }

    if let Some(last_child_command) = child_commands.last_mut().and_then(|last_child_command| {
        super::command::SCHEMAS
            .get(&last_child_command.code)
            .is_some_and(|schema| schema.is_sibling(last_child_command, &child_command))
            .then_some(last_child_command)
    }) {
        last_child_command.sibling_commands.push(child_command);
    } else {
        if let Some(last_child_command) = child_commands.last_mut() {
            last_child_command.matches_schema = super::command::SCHEMAS
                .get(&last_child_command.code)
                .is_some_and(|schema| schema.matches(last_child_command));
        }
        child_commands.push(child_command);
    }
}

fn deserialize_event_command_list<E>(
    indented_commands: impl Iterator<Item = Result<IndentedEventCommand, E>>,
) -> Result<EventCommandList, E> {
    let mut list = EventCommandList::default();
    let mut stack = Vec::<IndentedEventCommand>::new();

    for maybe_indented_command in indented_commands {
        let indented_command = maybe_indented_command?;
        if indented_command.command.code == 0 {
            // Ignore commands that have code equal to 0; these are terminator commands
            continue;
        }

        while stack
            .last()
            .is_some_and(|stack_top| indented_command.indent <= stack_top.indent)
        {
            let child_command = stack.pop().unwrap().command;
            push_child_command(
                stack
                    .last_mut()
                    .map(|stack_top| &mut stack_top.command.child_commands)
                    .unwrap_or(&mut list.commands),
                child_command,
            );
        }

        stack.push(indented_command);
    }

    while let Some(stack_top) = stack.pop() {
        push_child_command(
            stack
                .last_mut()
                .map(|stack_top| &mut stack_top.command.child_commands)
                .unwrap_or(&mut list.commands),
            stack_top.command,
        );
    }

    Ok(list)
}

impl<'de> serde::de::Visitor<'de> for EventCommandListVisitor {
    type Value = EventCommandList;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        deserialize_event_command_list(std::iter::from_fn(|| seq.next_element().transpose()))
    }
}

impl<'de> alox_48::Visitor<'de> for EventCommandListVisitor {
    type Value = EventCommandList;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an array")
    }

    fn visit_array<A>(self, mut seq: A) -> alox_48::DeResult<Self::Value>
    where
        A: alox_48::ArrayAccess<'de>,
    {
        deserialize_event_command_list(std::iter::from_fn(|| seq.next_element().transpose()))
    }
}

impl<'de> serde::Deserialize<'de> for IndentedEventCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(IndentedEventCommandVisitor)
    }
}

impl<'de> alox_48::Deserialize<'de> for IndentedEventCommand {
    fn deserialize<D>(deserializer: D) -> alox_48::DeResult<Self>
    where
        D: alox_48::DeserializerTrait<'de>,
    {
        deserializer.deserialize(IndentedEventCommandVisitor)
    }
}

impl<'de> serde::de::Visitor<'de> for IndentedEventCommandVisitor {
    type Value = IndentedEventCommand;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a key-value mapping")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut indented_command = IndentedEventCommand::default();
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "code" => indented_command.command.code = map.next_value()?,
                "indent" => indented_command.indent = map.next_value()?,
                "parameters" => indented_command.command.parameters = map.next_value()?,
                _ => {}
            }
        }
        Ok(indented_command)
    }
}

impl<'de> alox_48::Visitor<'de> for IndentedEventCommandVisitor {
    type Value = IndentedEventCommand;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an instance of RPG::EventCommand")
    }

    fn visit_object<A>(self, class: &'de alox_48::Sym, mut map: A) -> alox_48::DeResult<Self::Value>
    where
        A: alox_48::IvarAccess<'de>,
    {
        if class.as_str() != "RPG::EventCommand" {
            return Err(alox_48::DeError::invalid_type(
                alox_48::de::Unexpected::Class(class),
                &self,
            ));
        }
        let mut indented_command = IndentedEventCommand::default();
        while let Some(key) = map.next_ivar()? {
            match key.as_str() {
                "@code" => indented_command.command.code = map.next_value()?,
                "@indent" => indented_command.indent = map.next_value()?,
                "@parameters" => indented_command.command.parameters = map.next_value()?,
                _ => {}
            }
        }
        Ok(indented_command)
    }
}
