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
//
//     Additional permission under GNU GPL version 3 section 7
//
// If you modify this Program, or any covered work, by linking or combining
// it with Steamworks API by Valve Corporation, containing parts covered by
// terms of the Steamworks API by Valve Corporation, the licensors of this
// Program grant you additional permission to convey the resulting work.

use crate::components::OptionalIdComboBox;
use luminol_core::{Data, UpdateState};
use luminol_data::rpg;
use std::cell::RefMut;

pub trait DatabaseType {
    type Collection;
    fn get_collection(database: &Data) -> RefMut<'_, Self::Collection>;
    fn get_len(collection: &RefMut<'_, Self::Collection>) -> usize;
    fn get_name<'a>(collection: &'a RefMut<'a, Self::Collection>, index: usize) -> Option<&'a str>;
}

/// A widget for changing the value of an integer parameter that refers to a database entry, such as
/// an item or weapon.
#[must_use = "call `.fmt()` to convert to a `String` or `.id_salt()` to convert to a `egui::Widget`"]
pub struct DatabaseSelection<'this, 'update_state, T, P> {
    database_type: std::marker::PhantomData<T>,
    update_state: &'this UpdateState<'update_state>,
    parameter: P,
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct DatabaseSelectionWithId<'this, 'update_state, T, P, H> {
    inner: DatabaseSelection<'this, 'update_state, T, P>,
    id_salt: H,
}

macro_rules! impl_database_selection {
    ($($type:ty => $selection_type_name:ident),* $(,)?) => {
        $(
            pub type $selection_type_name<'this, 'update_state, P> = DatabaseSelection<'this, 'update_state, $type, P>;
        )*
    };
}

macro_rules! impl_database_type {
    ($($type:ty => $selection_type_name:ident, $method:ident),* $(,)?) => {
        $(
            impl_database_selection! {
                $type => $selection_type_name,
            }
            impl DatabaseType for $type {
                type Collection = $type;
                fn get_collection(database: &Data) -> RefMut<'_, Self::Collection> {
                    database.$method()
                }
                fn get_len(collection: &RefMut<'_, Self::Collection>) -> usize {
                    collection.data.len()
                }
                fn get_name<'a>(collection: &'a RefMut<'a, Self::Collection>, index: usize) -> Option<&'a str> {
                    Some(&collection.data.get(index)?.name)
                }
            }
        )*
    };
}

macro_rules! impl_system_database_type {
    ($($type:ident => $selection_type_name:ident, $field:ident),* $(,)?) => {
        $(
            pub struct $type;
            impl_database_selection! {
                $type => $selection_type_name,
            }
            impl DatabaseType for $type {
                type Collection = rpg::System;
                fn get_collection(database: &Data) -> RefMut<'_, Self::Collection> {
                    database.system()
                }
                fn get_len(collection: &RefMut<'_, Self::Collection>) -> usize {
                    collection.$field.len()
                }
                fn get_name<'a>(collection: &'a RefMut<'a, Self::Collection>, index: usize) -> Option<&'a str> {
                    Some(&collection.$field.get(index)?)
                }
            }
        )*
    };
}

impl_database_type! {
    rpg::Actors => ActorSelection, actors,
    rpg::Animations => AnimationSelection, animations,
    rpg::Armors => ArmorSelection, armors,
    rpg::Classes => ClassSelection, classes,
    rpg::CommonEvents => CommonEventSelection, common_events,
    rpg::Enemies => EnemySelection, enemies,
    rpg::Items => ItemSelection, items,
    rpg::Scripts => ScriptSelection, scripts,
    rpg::Skills => SkillSelection, skills,
    rpg::States => StateSelection, states,
    rpg::Tilesets => TilesetSelection, tilesets,
    rpg::Troops => TroopSelection, troops,
    rpg::Weapons => WeaponSelection, weapons,
}

impl_system_database_type! {
    Elements => ElementSelection, elements,
    Switches => SwitchSelection, switches,
    Variables => VariableSelection, variables,
}

impl<'this, 'update_state, T, P> DatabaseSelection<'this, 'update_state, T, P>
where
    T: DatabaseType,
    P: super::IntegerParameterRef,
{
    pub fn new(update_state: &'this UpdateState<'update_state>, parameter: P) -> Self {
        Self {
            database_type: std::marker::PhantomData,
            update_state,
            parameter,
        }
    }

    fn fmt_impl(id: usize, name: &str) -> String {
        format!("{id:0>4}: {name}")
    }

    /// Attempts to format the database entry as a string.
    ///
    /// This will fail if the parameter does not refer to a valid database entry.
    pub fn fmt(&self) -> Option<String> {
        let id = usize::try_from(self.parameter.to_integer()).ok()?;
        let collection = T::get_collection(self.update_state.data);
        let name = T::get_name(&collection, id.checked_sub(1)?)?;
        Some(Self::fmt_impl(id, name))
    }

    /// Sets the ID salt that this widget will use to persist UI state.
    pub fn id_salt<H>(self, id_salt: H) -> DatabaseSelectionWithId<'this, 'update_state, T, P, H>
    where
        H: std::hash::Hash,
    {
        DatabaseSelectionWithId {
            inner: self,
            id_salt,
        }
    }
}

impl<T, P, H> egui::Widget for DatabaseSelectionWithId<'_, '_, T, P, H>
where
    T: DatabaseType,
    P: super::IntegerParameterMut,
    H: std::hash::Hash,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let collection = T::get_collection(self.inner.update_state.data);
        ui.add(OptionalIdComboBox::new(
            self.inner.update_state,
            self.id_salt,
            self.inner.parameter.as_integer_mut(),
            1..=T::get_len(&collection),
            |id| {
                id.checked_sub(1)
                    .and_then(|id| T::get_name(&collection, id))
                    .map(|name| DatabaseSelection::<T, P>::fmt_impl(id, name))
                    .unwrap_or_default()
            },
        ))
    }
}
