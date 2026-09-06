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

use super::EventCommand;
use crate::ParameterType;

pub(super) trait EventCommandSchema
where
    Self: Sync + 'static,
{
    /// Returns true if the given command matches the schema, otherwise false.
    ///
    /// This method is called after all children and siblings are added to the command. Implement
    /// the `is_sibling` method to control which commands can be added as a sibling to this one.
    fn matches(&self, command: &EventCommand) -> bool;

    /// When deserializing commands, this method is called repeatedly to add siblings to a command.
    /// All siblings for which this method returns `true` are added as siblings; the first time this
    /// method returns `false` indicates the end of the siblings.
    ///
    /// The default implementation makes it so that the command cannot have siblings.
    #[allow(unused_variables)]
    fn is_sibling(&self, command: &EventCommand, sibling: &EventCommand) -> bool {
        false
    }
}

mod c101;
mod c102;
mod c103;
mod c104;
mod c105;
mod c106;

pub(super) static SCHEMAS: phf::Map<u16, &dyn EventCommandSchema> = phf::phf_map! {
    101u16 => &c101::Schema { continuation_code: 401 },
    102u16 => &c102::Schema,
    103u16 => &c103::Schema,
    104u16 => &c104::Schema,
    105u16 => &c105::Schema,
    106u16 => &c106::Schema,
    108u16 => &c101::Schema { continuation_code: 408 },
};
