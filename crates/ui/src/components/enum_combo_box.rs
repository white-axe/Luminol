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

trait EnumCast {
    type Enum;
    type Reference;
    fn get_mut(&mut self) -> &mut Self::Reference;
    fn cast(&self) -> Option<(std::mem::Discriminant<Self::Enum>, String)>;
}

pub struct TrivialEnumCast<'a, T>(&'a mut T);

impl<T> EnumCast for TrivialEnumCast<'_, T>
where
    T: ToString,
{
    type Enum = T;
    type Reference = T;

    fn get_mut(&mut self) -> &mut Self::Reference {
        self.0
    }

    fn cast(&self) -> Option<(std::mem::Discriminant<Self::Enum>, String)> {
        Some((std::mem::discriminant(self.0), self.0.to_string()))
    }
}

pub struct TryIntoEnumCast<'a, T, R>(&'a mut R, std::marker::PhantomData<T>);

impl<T, R, E> EnumCast for TryIntoEnumCast<'_, T, R>
where
    T: ToString,
    R: TryInto<T, Error = E> + Clone,
{
    type Enum = T;
    type Reference = R;

    fn get_mut(&mut self) -> &mut Self::Reference {
        self.0
    }

    fn cast(&self) -> Option<(std::mem::Discriminant<Self::Enum>, String)> {
        let value = self.0.clone().try_into().ok()?;
        Some((std::mem::discriminant(&value), value.to_string()))
    }
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct EnumComboBox<C, H> {
    id_salt: H,
    reference: C,

    max_width: f32,
    wrap_mode: egui::TextWrapMode,
}

impl<'a, T, H> EnumComboBox<TrivialEnumCast<'a, T>, H>
where
    T: ToString + strum::IntoEnumIterator + 'static,
    H: std::hash::Hash,
{
    /// Creates a combo box that can be used to change the variant of an enum `T` that implements
    /// `ToString + strum::IntoEnumIterator`.
    pub fn new(id_salt: H, reference: &'a mut T) -> Self {
        Self::new_impl(id_salt, TrivialEnumCast(reference))
    }
}

impl<'a, T, R, H, E> EnumComboBox<TryIntoEnumCast<'a, T, R>, H>
where
    T: Into<R> + ToString + strum::IntoEnumIterator + 'static,
    R: TryInto<T, Error = E> + Clone,
    H: std::hash::Hash,
{
    /// Creates a combo box that can be used to change a value that can be converted to/from an enum
    /// `T` that implements `ToString + strum::IntoEnumIterator`.
    pub fn new_with_conversion(
        enum_type: std::marker::PhantomData<T>,
        id_salt: H,
        reference: &'a mut R,
    ) -> Self {
        Self::new_impl(id_salt, TryIntoEnumCast(reference, enum_type))
    }
}

impl<C, H> EnumComboBox<C, H> {
    fn new_impl(id_salt: H, reference: C) -> Self {
        Self {
            id_salt,
            reference,
            max_width: f32::INFINITY,
            wrap_mode: egui::TextWrapMode::Wrap,
        }
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn wrap_mode(mut self, wrap_mode: egui::TextWrapMode) -> Self {
        self.wrap_mode = wrap_mode;
        self
    }
}

impl<C, T, R, H> egui::Widget for EnumComboBox<C, H>
where
    C: EnumCast<Enum = T, Reference = R>,
    T: Into<R> + ToString + strum::IntoEnumIterator + 'static,
    H: std::hash::Hash,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let (reference_discriminant, reference_text) =
            if let Some((discriminant, text)) = self.reference.cast() {
                (Some(discriminant), text)
            } else {
                (None, Default::default())
            };
        let widget = super::ComboBox::new(self.id_salt)
            .without_argument()
            .without_state()
            .choices(T::iter().enumerate().map(|(index, variant)| {
                (index, std::mem::discriminant(&variant), variant.to_string())
            }))
            .selected_text(Some(reference_text))
            .prepare(
                |_data, (_index, _discriminant, text)| text.clone(),
                |_data, maybe_variant| {
                    maybe_variant.is_some_and(|(_index, discriminant, _text)| {
                        Some(*discriminant) == reference_discriminant
                    })
                },
                |_data, maybe_variant| {
                    if let Some((index, _discriminant, _text)) = maybe_variant {
                        *self.reference.get_mut() = T::iter().nth(*index).unwrap().into();
                    }
                },
            );
        egui::Widget::ui(widget, ui)
    }
}
