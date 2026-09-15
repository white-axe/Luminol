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

use super::{EventCommand, EventCommandList, IndentedEventCommand, SCHEMAS};

struct EventCommandListVisitor;

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

fn match_last_child_command_with_schema(child_commands: &mut Vec<EventCommand>) {
    let Some(last_child_command) = child_commands.last_mut() else {
        return;
    };

    last_child_command.matches_schema = SCHEMAS
        .get(&last_child_command.code)
        .is_some_and(|schema| schema.matches(last_child_command));

    if !last_child_command.matches_schema {
        let extra_child_commands = std::mem::take(&mut last_child_command.sibling_commands);
        child_commands.extend(extra_child_commands);
    }
}

fn push_child_command(child_commands: &mut Vec<EventCommand>, mut child_command: EventCommand) {
    match_last_child_command_with_schema(&mut child_command.child_commands);

    if let Some(last_child_command) = child_commands.last_mut().and_then(|last_child_command| {
        SCHEMAS
            .get(&last_child_command.code)
            .is_some_and(|schema| schema.is_sibling(last_child_command, &child_command))
            .then_some(last_child_command)
    }) {
        last_child_command.sibling_commands.push(child_command);
    } else {
        match_last_child_command_with_schema(child_commands);
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

    match_last_child_command_with_schema(&mut list.commands);

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

struct IndentedEventCommandVisitor;

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
