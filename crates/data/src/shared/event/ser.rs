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

use super::{EventCommandList, IndentedEventCommandRef};
use alox_48::{SerializeArray, SerializeIvars};
use serde::ser::{SerializeMap, SerializeSeq};

impl serde::Serialize for EventCommandList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.indented_iter().count()))?;
        for indented_command in self.indented_iter() {
            seq.serialize_element(&indented_command.as_ref())?;
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
            seq.serialize_element(&indented_command.as_ref())?;
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
