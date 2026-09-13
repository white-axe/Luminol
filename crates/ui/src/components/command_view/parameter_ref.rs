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

use super::ParameterType;

/// An immutable reference to an integer parameter.
pub trait IntegerParameterRef {
    /// Converts this parameter into an integer.
    fn to_integer(&self) -> i32;
}

/// A mutable reference to an integer parameter.
pub trait IntegerParameterMut
where
    Self: IntegerParameterRef,
{
    /// Mutably borrows this parameter as an integer.
    fn as_integer_mut(&mut self) -> &mut i32;
}

impl<T> IntegerParameterRef for &T
where
    T: IntegerParameterRef,
{
    fn to_integer(&self) -> i32 {
        (*self).to_integer()
    }
}

impl<T> IntegerParameterRef for &mut T
where
    T: IntegerParameterRef,
{
    fn to_integer(&self) -> i32 {
        let self_immutable: &T = self;
        self_immutable.to_integer()
    }
}

impl<T> IntegerParameterMut for &mut T
where
    T: IntegerParameterMut,
{
    fn as_integer_mut(&mut self) -> &mut i32 {
        (*self).as_integer_mut()
    }
}

impl IntegerParameterRef for i32 {
    fn to_integer(&self) -> i32 {
        *self
    }
}

impl IntegerParameterMut for i32 {
    fn as_integer_mut(&mut self) -> &mut i32 {
        self
    }
}

impl IntegerParameterRef for ParameterType {
    fn to_integer(&self) -> i32 {
        self.as_integer().copied().unwrap_or(0)
    }
}

impl IntegerParameterMut for ParameterType {
    fn as_integer_mut(&mut self) -> &mut i32 {
        if !self.is_integer() {
            *self = ParameterType::Integer(0);
        }
        self.as_integer_mut().unwrap()
    }
}
