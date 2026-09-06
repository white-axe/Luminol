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
        // Must not have children
        if !command.child_commands.is_empty() {
            return false;
        }

        // Must have exactly two parameters, both of which are integers
        let [variable_index, num_digits] = &command.parameters[..] else {
            return false;
        };
        if !matches!(variable_index, ParameterType::Integer(_)) {
            return false;
        };
        if !matches!(num_digits, ParameterType::Integer(_)) {
            return false;
        };

        true
    }
}
