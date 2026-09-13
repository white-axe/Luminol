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

use super::EventInfo;
use crate::components::{EnumComboBox, OptionalIdComboBox};
use luminol_core::UpdateState;

#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum CharacterType {
    Player = -1,
    #[strum(to_string = "This event")]
    ThisEvent = 0,
    #[default]
    #[strum(to_string = "Map event")]
    MapEvent = 1,
}

#[must_use = "call `.fmt()` to convert to a `String` or `.id_salt()` to convert to a `egui::Widget`"]
pub struct CharacterSelection<'this, 'update_state, P> {
    update_state: &'this UpdateState<'update_state>,
    event_info: Option<&'this EventInfo<'this>>,
    parameter: P,
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct CharacterSelectionWithId<'this, 'update_state, P, H> {
    inner: CharacterSelection<'this, 'update_state, P>,
    id_salt: H,
}

/// A widget for changing the value of a character parameter.
impl<'this, 'update_state, P> CharacterSelection<'this, 'update_state, P>
where
    P: super::IntegerParameterRef,
{
    pub fn new(
        update_state: &'this UpdateState<'update_state>,
        event_info: Option<&'this EventInfo<'this>>,
        parameter: P,
    ) -> Self {
        Self {
            update_state,
            event_info,
            parameter,
        }
    }

    fn fmt_impl_with_event_info(id: usize, name: &str) -> String {
        format!("{id:0>4}: {name}")
    }

    fn fmt_impl_without_event_info(id: usize) -> String {
        format!("{id:0>4}")
    }

    /// Attempts to format the character as a string.
    ///
    /// This will fail if the parameter does not refer to a valid character.
    pub fn fmt(&self) -> Option<String> {
        let id = self.parameter.to_integer();
        match id {
            -1 => Some("Player".into()),
            0 => Some("This event".into()),
            _ => {
                let id = usize::try_from(id).ok()?;
                if let Some(event_info) = self.event_info.copied() {
                    let map = self.update_state.data.get_map(event_info.map_id);
                    let name = if id == event_info.event_id {
                        event_info.event_name
                    } else {
                        map.events.get(id).map(|event| event.name.as_str())?
                    };
                    Some(Self::fmt_impl_with_event_info(id, name))
                } else {
                    Some(Self::fmt_impl_without_event_info(id))
                }
            }
        }
    }

    /// Sets the ID salt that this widget will use to persist UI state.
    pub fn id_salt<H>(self, id_salt: H) -> CharacterSelectionWithId<'this, 'update_state, P, H>
    where
        H: std::hash::Hash,
    {
        CharacterSelectionWithId {
            inner: self,
            id_salt,
        }
    }
}

impl<P, H> egui::Widget for CharacterSelectionWithId<'_, '_, P, H>
where
    P: super::IntegerParameterMut,
    H: std::hash::Hash,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let character = self.inner.parameter.as_integer_mut();
        if let Some(event_info) = self.inner.event_info.copied() {
            *character = character.wrapping_add(1);
            let map = self.inner.update_state.data.get_map(event_info.map_id);
            let response = ui.add(OptionalIdComboBox::new(
                self.inner.update_state,
                (self.id_salt, true),
                character,
                0..=map.events.len(),
                |id| match id {
                    0 => "Player".into(),
                    1 => "This event".into(),
                    _ => {
                        let id = id - 1;
                        if id == event_info.event_id {
                            CharacterSelection::<P>::fmt_impl_with_event_info(
                                id,
                                event_info.event_name,
                            )
                        } else {
                            map.events
                                .get(id)
                                .map(|event| {
                                    CharacterSelection::<P>::fmt_impl_with_event_info(
                                        id,
                                        &event.name,
                                    )
                                })
                                .unwrap_or_default()
                        }
                    }
                },
            ));
            *character = character.wrapping_sub(1);
            response
        } else {
            let mut character_type = CharacterType::try_from(*character).unwrap_or_default();
            let mut modified = false;
            let mut response = egui::Frame::NONE
                .show(ui, |ui| {
                    modified |= {
                        let changed = ui
                            .add(EnumComboBox::new(
                                (self.id_salt, false),
                                &mut character_type,
                            ))
                            .changed();
                        if changed {
                            *character = character_type.into();
                        }
                        changed
                    };
                    if character_type == CharacterType::MapEvent {
                        modified |= ui
                            .add(egui::DragValue::new(character).range(1..=i32::MAX))
                            .changed();
                    }
                })
                .response;
            if modified {
                response.mark_changed();
            }
            response
        }
    }
}
