// Copyright (C) 2023 Lily Lyons
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("filesystem error: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("marshal parser error: {0}")]
    Alox(#[from] alox_48::Error),
    #[error("ron parser error: {0}")]
    Ron(#[from] ron::Error),
    #[error("{0}")]
    Custom(String),
    #[error("project not open")]
    NotOpen,
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
