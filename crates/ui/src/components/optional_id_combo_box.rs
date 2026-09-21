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

pub trait IdCast
where
    Self: Sized,
{
    fn from_id(value: usize) -> Option<Self>;
    fn to_id(&self) -> Option<usize>;
}

macro_rules! impl_into_optional_id {
    ($($primitive:ty),* $(,)?) => {
        $(
            impl IdCast for $primitive {
                fn from_id(value: usize) -> Option<Self> {
                    value.try_into().ok()
                }
                fn to_id(&self) -> Option<usize> {
                    (*self).try_into().ok()
                }
            }
        )*
    };
}

impl_into_optional_id!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

#[must_use = "use `ui.add()` to show this widget"]
pub struct OptionalIdComboBox<'a, R, I, H, F> {
    id_salt: H,
    reference: &'a mut R,
    id_iter: I,
    formatter: F,
    is_stale: bool,
    allow_none: bool,
}

impl<T, I, H, F> OptionalIdComboBox<'_, Option<T>, I, H, F>
where
    T: IdCast,
    I: Iterator<Item = usize> + Clone,
    H: std::hash::Hash,
    F: Fn(usize) -> String,
{
    /// Enables or disables selecting the "(None)" option in the combo box. Defaults to `true`.
    pub fn allow_none(mut self, value: bool) -> Self {
        self.allow_none = value;
        self
    }
}

impl<'a, R, I, H, F> OptionalIdComboBox<'a, R, I, H, F>
where
    I: Iterator<Item = usize> + Clone,
    H: std::hash::Hash,
    F: Fn(usize) -> String,
{
    /// Creates a combo box that can be used to change the ID of an `optional_id` field in the data
    /// cache.
    pub fn new(
        update_state: &luminol_core::UpdateState<'_>,
        id_salt: H,
        reference: &'a mut R,
        id_iter: I,
        formatter: F,
    ) -> Self {
        Self {
            id_salt,
            reference,
            id_iter,
            formatter,
            is_stale: *update_state.modified_during_prev_frame,
            allow_none: true,
        }
    }
}

impl<T, I, H, F> egui::Widget for OptionalIdComboBox<'_, Option<T>, I, H, F>
where
    T: IdCast,
    I: Iterator<Item = usize> + Clone,
    H: std::hash::Hash,
    F: Fn(usize) -> String,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let reference_id = self.reference.as_ref().and_then(|r| r.to_id());
        let widget = super::ComboBox::new(self.id_salt)
            .allow_none(self.allow_none)
            .is_stale(self.is_stale)
            .without_argument()
            .without_state()
            .selected_text(self.reference.as_ref().map(|r| {
                if let Some(id) = r.to_id() {
                    (self.formatter)(id)
                } else {
                    "".into()
                }
            }))
            .choices(self.id_iter)
            .prepare(
                |_data, id| (self.formatter)(*id),
                |_data, maybe_id| maybe_id.copied() == reference_id,
                |_data, maybe_id| {
                    *self.reference = maybe_id.and_then(|id| IdCast::from_id(*id));
                },
            );
        egui::Widget::ui(widget, ui)
    }
}

impl<T, I, H, F> egui::Widget for OptionalIdComboBox<'_, T, I, H, F>
where
    T: IdCast,
    I: Iterator<Item = usize> + Clone,
    H: std::hash::Hash,
    F: Fn(usize) -> String,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let reference_id = self.reference.to_id();
        let widget = super::ComboBox::new(self.id_salt)
            .is_stale(self.is_stale)
            .without_argument()
            .without_state()
            .selected_text(Some(if let Some(id) = self.reference.to_id() {
                (self.formatter)(id)
            } else {
                "".into()
            }))
            .choices(self.id_iter)
            .prepare(
                |_data, id| (self.formatter)(*id),
                |_data, maybe_id| maybe_id.copied() == reference_id,
                |_data, maybe_id| {
                    if let Some(new_value) = maybe_id.and_then(|id| IdCast::from_id(*id)) {
                        *self.reference = new_value;
                    }
                },
            );
        egui::Widget::ui(widget, ui)
    }
}
