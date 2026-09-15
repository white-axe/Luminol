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
use crate::{Path, PathRef};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(alox_48::Deserialize, alox_48::Serialize)]
#[marshal(class = "RPG::AudioFile")]
pub struct AudioFile {
    pub name: Path,
    pub volume: u8,
    pub pitch: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioFileRef<'a> {
    pub name: PathRef<'a>,
    pub volume: u8,
    pub pitch: u8,
}

impl<'a> From<&'a AudioFile> for AudioFileRef<'a> {
    fn from(value: &'a AudioFile) -> Self {
        Self {
            name: value.name.0.as_ref().map(|name| name.as_path()).into(),
            volume: value.volume,
            pitch: value.pitch,
        }
    }
}

impl Default for AudioFile {
    fn default() -> Self {
        Self {
            name: None.into(),
            volume: 100,
            pitch: 100,
        }
    }
}

impl Default for AudioFileRef<'_> {
    fn default() -> Self {
        Self {
            name: None.into(),
            volume: 100,
            pitch: 100,
        }
    }
}
