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

use super::{EventCommand, EventCommandSchema};

pub(super) struct Schema;

impl EventCommandSchema for Schema {
    fn matches(&self, command: &EventCommand) -> bool {
        // Must not have children
        if !command.child_commands.is_empty() {
            return false;
        }

        // Must have exactly two parameters, the first of which is an array containing only strings
        // that have no newline characters, and the second of which is an integer
        let [choices, cancel_type] = &command.parameters[..] else {
            return false;
        };
        let Some(choices) = choices.as_array() else {
            return false;
        };
        for choice in choices.iter() {
            let Some(choice) = choice.as_string() else {
                return false;
            };
            if choice.contains('\n') {
                return false;
            }
        }
        let Some(cancel_type) = cancel_type.as_integer() else {
            return false;
        };
        let cancel_type = *cancel_type;

        // Sibling commands must be terminated by command code 404
        if command
            .sibling_commands
            .last()
            .is_none_or(|sibling| sibling.code != 404)
        {
            return false;
        }

        // Sibling commands should not have duplicate choice indices (command code 403 counts as
        // choice index 4)
        let mut choice_index_set = std::collections::HashSet::new();
        for sibling in command.sibling_commands.iter() {
            if !match sibling.code {
                402 => choice_index_set.insert(sibling.parameters[0].as_integer().unwrap()),
                403 => choice_index_set.insert(&4),
                _ => true,
            } {
                return false;
            }
        }

        // Each choice index must correspond to either an index in the array given as the first
        // parameter or the cancel type given as the second parameter
        for i in 0..choices.len() {
            if let Ok(i) = i32::try_from(i) {
                choice_index_set.remove(&i);
            }
        }
        if cancel_type > 0 {
            choice_index_set.remove(&(cancel_type - 1));
        }
        if !choice_index_set.is_empty() {
            return false;
        }

        true
    }

    fn is_sibling(&self, command: &EventCommand, sibling: &EventCommand) -> bool {
        // Siblings must consist of 402/403 commands followed by a 404 command
        if !match command
            .sibling_commands
            .last()
            .map_or(402, |last| last.code)
        {
            402 | 403 => [402, 403, 404].contains(&sibling.code),
            _ => false,
        } {
            return false;
        }

        match sibling.code {
            402 => {
                // Must have exactly two parameters, the first of which is an integer equal to the
                // sibling index, and the second of which is a string (supposed to be equal to the
                // choice text, but we don't really care if it's actually equal)
                let [choice_index, text] = &sibling.parameters[..] else {
                    return false;
                };
                let Some(choice_index) = choice_index.as_integer() else {
                    return false;
                };
                let Ok(choice_index) = usize::try_from(*choice_index) else {
                    return false;
                };
                if choice_index != command.sibling_commands.len() {
                    return false;
                }
                if !text.is_string() {
                    return false;
                }
                true
            }
            403 => {
                // Must have no parameters
                sibling.parameters.is_empty()
            }
            404 => {
                // Must have no parameters or child commands
                sibling.parameters.is_empty() && sibling.child_commands.is_empty()
            }
            _ => false,
        }
    }
}
