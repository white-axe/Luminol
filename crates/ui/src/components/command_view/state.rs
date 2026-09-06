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

    /// Calls `closure` with a mutable reference to the state for this event command editor.
    ///
    /// If the state for this event command editor has never been retrieved before, it will first be
    /// set to the value returned by `initializer`.
    ///
    /// `arg` will be passed to both `initializer` and `closure`. This is useful for avoiding borrow
    /// checker issues if both `initializer` and `closure` borrow the same variable.
    pub fn with_initializer_and_arg<R, S, A>(
        self,
        mut arg: A,
        initializer: impl FnOnce(&mut A) -> S,
        closure: impl FnOnce(A, &mut S) -> R,
    ) -> R
    where
        S: Send + 'static,
    {
        let state = if let Some(state) = self.0.as_mut().and_then(|state| state.downcast_mut()) {
            state
        } else {
            self.0
                .insert(Box::new(initializer(&mut arg)))
                .downcast_mut()
                .unwrap()
        };
        closure(arg, state)
    }

    /// Calls `closure` with a mutable reference to the state for this event command editor.
    ///
    /// If the state for this event command editor has never been retrieved before, it will first be
    /// set to the value returned by `initializer`.
    pub fn with_initializer<R, S>(
        self,
        initializer: impl FnOnce() -> S,
        closure: impl FnOnce(&mut S) -> R,
    ) -> R
    where
        S: Send + 'static,
    {
        self.with_initializer_and_arg((), |()| initializer(), |(), state| closure(state))
    }

    /// Calls `closure` with a mutable reference to the state for this event command editor.
    ///
    /// If the state for this event command editor has never been retrieved before, it will first be
    /// set to the default value.
    pub fn _with_default<R, S>(self, closure: impl FnOnce(&mut S) -> R) -> R
    where
        S: Default + Send + 'static,
    {
        self.with_initializer(Default::default, closure)
    }
}
