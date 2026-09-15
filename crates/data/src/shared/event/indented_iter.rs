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

use super::{EventCommand, EventCommandList, IndentedEventCommand, IndentedEventCommandRef};

enum EventCommandListIndentedIterItemInner<'a> {
    Ref(IndentedEventCommandRef<'a>),
    Terminator(IndentedEventCommand),
}

/// An item returned by [`EventCommandListIndentedIter`]. Can be converted to
/// [`IndentedEventCommandRef`] by calling its `.as_ref` method.
pub struct EventCommandListIndentedIterItem<'a>(EventCommandListIndentedIterItemInner<'a>);

impl EventCommandListIndentedIterItem<'_> {
    /// Converts this object into an [`IndentedEventCommandRef`].
    pub fn as_ref(&self) -> IndentedEventCommandRef<'_> {
        self.into()
    }
}

impl<'a> From<&'a EventCommandListIndentedIterItem<'a>> for IndentedEventCommandRef<'a> {
    fn from(value: &'a EventCommandListIndentedIterItem<'a>) -> Self {
        match &value.0 {
            EventCommandListIndentedIterItemInner::Ref(indented_command) => *indented_command,
            EventCommandListIndentedIterItemInner::Terminator(indented_command) => {
                indented_command.as_ref()
            }
        }
    }
}

/// A recursive iterator over the event commands in an [`EventCommandList`].
pub struct EventCommandListIndentedIter<'a> {
    stack: Vec<(
        std::slice::Iter<'a, EventCommand>,
        std::slice::Iter<'a, EventCommand>,
        bool,
        bool,
    )>,
    indent: u64,
    is_terminated: bool,
}

impl<'a> From<std::slice::Iter<'a, EventCommand>> for EventCommandListIndentedIter<'a> {
    fn from(value: std::slice::Iter<'a, EventCommand>) -> Self {
        Self {
            stack: vec![([].iter(), value, false, false)],
            indent: 0,
            is_terminated: false,
        }
    }
}

impl<'a> From<&'a [EventCommand]> for EventCommandListIndentedIter<'a> {
    fn from(value: &'a [EventCommand]) -> Self {
        value.iter().into()
    }
}

impl<'a> From<&'a Vec<EventCommand>> for EventCommandListIndentedIter<'a> {
    fn from(value: &'a Vec<EventCommand>) -> Self {
        value.as_slice().into()
    }
}

impl<'a> From<&'a EventCommandList> for EventCommandListIndentedIter<'a> {
    fn from(value: &'a EventCommandList) -> Self {
        (&value.commands).into()
    }
}

impl<'a> Iterator for EventCommandListIndentedIter<'a> {
    type Item = EventCommandListIndentedIterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((child_iter, sibling_iter, is_indented, is_terminated)) =
            self.stack.last_mut()
        {
            if let Some(command) = {
                if let Some(command) = child_iter.next() {
                    if !*is_indented {
                        *is_indented = true;
                        self.indent += 1;
                    }
                    Some(command)
                } else {
                    if *is_indented {
                        if !*is_terminated {
                            *is_terminated = true;
                            return Some(EventCommandListIndentedIterItem(
                                EventCommandListIndentedIterItemInner::Terminator(
                                    Default::default(),
                                ),
                            ));
                        }
                        *is_indented = false;
                        self.indent -= 1;
                    }
                    sibling_iter.next()
                }
            } {
                self.stack.push((
                    command.child_commands.iter(),
                    command.sibling_commands.iter(),
                    false,
                    false,
                ));
                return Some(EventCommandListIndentedIterItem(
                    EventCommandListIndentedIterItemInner::Ref(IndentedEventCommandRef {
                        indent: self.indent,
                        command,
                    }),
                ));
            } else {
                self.stack.pop();
            }
        }
        if !self.is_terminated {
            self.is_terminated = true;
            Some(EventCommandListIndentedIterItem(
                EventCommandListIndentedIterItemInner::Terminator(Default::default()),
            ))
        } else {
            None
        }
    }
}

impl std::iter::FusedIterator for EventCommandListIndentedIter<'_> {}
