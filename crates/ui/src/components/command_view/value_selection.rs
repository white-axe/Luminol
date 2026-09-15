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

use super::VariableSelection;
use crate::components::EnumComboBox;
use luminol_core::UpdateState;

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum ValueType {
    Constant = 0,
    Variable = 1,
}

pub enum ValueFmt {
    Constant(i32),
    Variable(String),
}

/// A widget for changing the value of two parameters that represent either a constant or variable.
#[must_use = "call `.fmt()` to convert to a `ValueFmt` or `.prepare()` to convert to a `egui::Widget`"]
pub struct ValueSelection<'this, 'update_state, P, Q> {
    update_state: &'this UpdateState<'update_state>,
    discriminant_parameter: P,
    value_parameter: Q,
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct ValueSelectionPrepared<'this, 'update_state, P, Q, H> {
    inner: ValueSelection<'this, 'update_state, P, Q>,
    id_salt: H,
}

impl<'this, 'update_state, P, Q> ValueSelection<'this, 'update_state, P, Q>
where
    P: super::IntegerParameterRef,
    Q: super::IntegerParameterRef,
{
    pub fn new(
        update_state: &'this UpdateState<'update_state>,
        discriminant_parameter: P,
        value_parameter: Q,
    ) -> Self {
        Self {
            update_state,
            discriminant_parameter,
            value_parameter,
        }
    }

    /// Attempts to format the value as an integer or string.
    ///
    /// This will fail if the parameters do not refer to a valid value.
    pub fn fmt(&self) -> Option<ValueFmt> {
        match self.discriminant_parameter.as_integer() {
            0 => Some(ValueFmt::Constant(self.value_parameter.as_integer())),
            1 => VariableSelection::new(self.update_state, &self.value_parameter)
                .fmt()
                .map(ValueFmt::Variable),
            _ => None,
        }
    }

    /// Prepares a [`egui::Widget`] from the character.
    pub fn prepare<H>(self, id_salt: H) -> ValueSelectionPrepared<'this, 'update_state, P, Q, H>
    where
        P: super::IntegerParameterMut,
        Q: super::IntegerParameterMut,
        H: std::hash::Hash,
    {
        ValueSelectionPrepared {
            inner: self,
            id_salt,
        }
    }
}

impl<P, Q, H> egui::Widget for ValueSelectionPrepared<'_, '_, P, Q, H>
where
    P: super::IntegerParameterMut,
    Q: super::IntegerParameterMut,
    H: std::hash::Hash,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut modified = false;
        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                let discriminant = self.inner.discriminant_parameter.as_integer_mut();
                modified |= ui
                    .add(EnumComboBox::new_with_conversion(
                        std::marker::PhantomData::<ValueType>,
                        (&self.id_salt, "discriminant"),
                        discriminant,
                    ))
                    .changed();
                modified |= match *discriminant {
                    0 => ui
                        .add(egui::DragValue::new(
                            self.inner.value_parameter.as_integer_mut(),
                        ))
                        .changed(),
                    1 => ui
                        .add(
                            VariableSelection::new(
                                self.inner.update_state,
                                self.inner.value_parameter.as_integer_mut(),
                            )
                            .prepare((&self.id_salt, "value")),
                        )
                        .changed(),
                    _ => false,
                }
            })
            .response;
        if modified {
            response.mark_changed();
        }
        response
    }
}
