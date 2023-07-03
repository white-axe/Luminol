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
use crate::{id, optional_path, Path, Table2};

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "RPG::Actor")]
pub struct Actor {
    #[serde(with = "id")]
    pub id: usize,
    pub name: String,
    #[serde(with = "id")]
    pub class_id: usize,
    pub initial_level: i32,
    pub final_level: i32,
    pub exp_basis: i32,
    pub exp_inflation: i32,
    #[serde(with = "optional_path")]
    pub character_name: Path,
    pub character_hue: i32,
    #[serde(with = "optional_path")]
    pub battler_name: Path,
    pub battler_hue: i32,
    pub parameters: Table2,
    #[serde(with = "id")]
    pub weapon_id: usize,
    #[serde(with = "id")]
    pub armor1_id: usize,
    #[serde(with = "id")]
    pub armor2_id: usize,
    #[serde(with = "id")]
    pub armor3_id: usize,
    #[serde(with = "id")]
    pub armor4_id: usize,
    pub weapon_fix: bool,
    pub armor1_fix: bool,
    pub armor2_fix: bool,
    pub armor3_fix: bool,
    pub armor4_fix: bool,
}
