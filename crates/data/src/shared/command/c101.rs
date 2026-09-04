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

pub(super) struct Schema {
    pub continuation_code: u16,
}

fn matches(command: &EventCommand) -> bool {
    // Must not have children
    if !command.child_commands.is_empty() {
        return false;
    }

    // Must have exactly one parameter, of type string, that doesn't contain any newlines
    let [argument] = &command.parameters[..] else {
        return false;
    };
    let ParameterType::String(argument) = argument else {
        return false;
    };
    if argument.contains('\n') {
        return false;
    }

    true
}

impl EventCommandSchema for Schema {
    fn matches(&'static self, command: &EventCommand) -> bool {
        matches(command)
    }

    fn is_sibling(&'static self, _command: &EventCommand, sibling: &EventCommand) -> bool {
        // Can have zero or more siblings with code `continuation_code`
        sibling.code == self.continuation_code && matches(sibling)
    }
}
