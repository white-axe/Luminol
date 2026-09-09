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

#[derive(Debug)]
pub struct EventCommandState(Option<Box<dyn std::any::Any + Send + Sync>>);

impl EventCommandState {
    pub(super) const fn new() -> Self {
        Self(None)
    }

    /// Returns a mutable reference to the state for this event command. This can be used by the
    /// event command editors to store UI state for individual commands.
    ///
    /// If the state for this event command has never been retrieved before, it will first be set to
    /// the value returned by `initializer`.
    pub fn get_or_insert_with<T>(&mut self, initializer: impl FnOnce() -> T) -> &mut T
    where
        T: Send + Sync + 'static,
    {
        if self.0.as_ref().is_none_or(|inner| !inner.is::<T>()) {
            self.0 = Some(Box::new(initializer()));
        }
        self.0.as_mut().unwrap().downcast_mut().unwrap()
    }

    /// Returns a mutable reference to the state for this event command. This can be used by the
    /// event command editors to store UI state for individual commands.
    ///
    /// If the state for this event command has never been retrieved before, it will first be set to
    /// `initial_value`.
    pub fn get_or_insert<T>(&mut self, initial_value: T) -> &mut T
    where
        T: Send + Sync + 'static,
    {
        self.get_or_insert_with(|| initial_value)
    }

    /// Returns a mutable reference to the state for this event command. This can be used by the
    /// event command editors to store UI state for individual commands.
    ///
    /// If the state for this event command has never been retrieved before, it will first be set to
    /// the default value of `T`.
    pub fn get_or_insert_default<T>(&mut self) -> &mut T
    where
        T: Default + Send + Sync + 'static,
    {
        self.get_or_insert_with(Default::default)
    }
}
