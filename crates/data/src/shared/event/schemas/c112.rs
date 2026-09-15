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
        // Must not have parameters
        if !command.parameters.is_empty() {
            return false;
        }

        // Sibling commands must be terminated by command code 413
        if command
            .sibling_commands
            .last()
            .is_none_or(|sibling| sibling.code != 413)
        {
            return false;
        }

        true
    }

    fn is_sibling(&self, command: &EventCommand, sibling: &EventCommand) -> bool {
        // Can have one empty sibling with code 413
        command.sibling_commands.is_empty()
            && sibling.code == 413
            && sibling.parameters.is_empty()
            && sibling.child_commands.is_empty()
    }
}
