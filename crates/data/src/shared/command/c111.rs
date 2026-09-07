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

use super::{EventCommand, EventCommandSchema, ParameterType};

pub(super) struct Schema;

impl EventCommandSchema for Schema {
    fn matches(&self, command: &EventCommand) -> bool {
        // Must have at least one parameter, of type integer
        let Some(branch_type) = command.parameters.first() else {
            return false;
        };
        let ParameterType::Integer(branch_type) = *branch_type else {
            return false;
        };

        // Sibling commands must be terminated by command code 412
        if command
            .sibling_commands
            .last()
            .is_none_or(|sibling| sibling.code != 412)
        {
            return false;
        }

        // The number of additional parameters required depends on the first one
        let expected_num_parameters = match branch_type {
            0 => 3,
            1 => 5,
            2 => 3,
            3 => 3,
            4 => match command.parameters.get(2) {
                Some(ParameterType::Integer(0)) => 3,
                Some(ParameterType::Integer(1..=5)) => 4,
                _ => {
                    return false;
                }
            },
            5 => match command.parameters.get(2) {
                Some(ParameterType::Integer(0)) => 3,
                Some(ParameterType::Integer(1)) => 4,
                _ => {
                    return false;
                }
            },
            6 => 3,
            7 => 3,
            8..=12 => 2,
            _ => {
                return false;
            }
        };
        if command.parameters.len() != expected_num_parameters {
            return false;
        }

        // All parameters must be integers, except in a few cases
        let actor_condition = match branch_type {
            4 => match command.parameters.get(2) {
                Some(ParameterType::Integer(value)) => Some(*value),
                _ => None,
            },
            _ => None,
        };
        if !command.parameters.iter().enumerate().all(|(i, parameter)| {
            match (branch_type, actor_condition, i) {
                (2 | 12, None, 1) | (4, Some(1), 3) => {
                    matches!(parameter, ParameterType::String(_))
                }
                _ => matches!(parameter, ParameterType::Integer(_)),
            }
        }) {
            return false;
        }

        true
    }

    fn is_sibling(&self, command: &EventCommand, sibling: &EventCommand) -> bool {
        // Siblings must consist of at most one 411 command followed by a 412 command
        if !match command.sibling_commands.last().map(|last| last.code) {
            None => [411, 412].contains(&sibling.code),
            Some(411) => sibling.code == 412,
            _ => false,
        } {
            return false;
        }

        match sibling.code {
            411 => {
                // Must have no parameters
                sibling.parameters.is_empty()
            }
            412 => {
                // Must have no parameters or child commands
                sibling.parameters.is_empty() && sibling.child_commands.is_empty()
            }
            _ => false,
        }
    }
}
