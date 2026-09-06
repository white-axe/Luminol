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

pub(super) struct EventCommandEditorState<'a>(&'a mut Option<Box<dyn std::any::Any + Send>>);

impl<'a> EventCommandEditorState<'a> {
    pub fn new(inner: &'a mut Option<Box<dyn std::any::Any + Send>>) -> Self {
        Self(inner)
    }

    /// Returns a mutable reference to the state for this event command editor.
    ///
    /// If the state for this event command editor has never been retrieved before, it will first be
    /// set to the value returned by `initializer`.
    pub fn get_or_insert_with<T>(self, initializer: impl FnOnce() -> T) -> &'a mut T
    where
        T: Send + 'static,
    {
        // For some reason, the borrow checker won't allow just using
        // ```
        // if let Some(state) = self.0.as_mut().and_then(|state| state.downcast_mut()) {
        //     state
        // } else {
        //     self.0.insert(Box::new(initializer())).downcast_mut().unwrap()
        // }
        // ```
        // even though that would be perfectly safe, so we have to do this instead
        if self.0.as_ref().is_some_and(|state| state.is::<T>()) {
            self.0.as_mut().unwrap().downcast_mut().unwrap()
        } else {
            self.0
                .insert(Box::new(initializer()))
                .downcast_mut()
                .unwrap()
        }
    }
}
