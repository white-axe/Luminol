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

use super::{MapSelection, ValueFmt, ValueSelection, VariableSelection};
use crate::components::EnumComboBox;
use luminol_core::UpdateState;
use std::marker::PhantomData;

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
enum LocationType {
    Constant = 0,
    Variable = 1,
}

pub enum LocationFmt {
    Constant((i32, i32)),
    Variable((String, String)),
}

pub enum MapFmt {
    Constant(String),
    Variable(String),
}

/// A widget for changing the value of three parameters that determine coordinates on a map.
#[must_use = "call `.fmt()` to convert to a `String` or `.prepare()` to convert to a `egui::Widget`"]
pub struct LocationSelection<'this, 'update_state, D, X, Y> {
    update_state: &'this UpdateState<'update_state>,
    discriminant_parameter: D,
    x_parameter: X,
    y_parameter: Y,
}

/// A widget for changing the value of four parameters that determine a map and coordinates.
#[must_use = "call `.fmt()` to convert to a `String` or `.prepare()` to convert to a `egui::Widget`"]
pub struct LocationSelectionWithMap<'this, 'update_state, D, M, X, Y> {
    inner: LocationSelection<'this, 'update_state, D, X, Y>,
    map_parameter: M,
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct LocationSelectionPrepared<'this, 'update_state, D, X, Y, H> {
    inner: LocationSelection<'this, 'update_state, D, X, Y>,
    id_salt: H,
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct LocationSelectionWithMapPrepared<'this, 'update_state, D, M, X, Y, H> {
    inner: LocationSelectionWithMap<'this, 'update_state, D, M, X, Y>,
    id_salt: H,
}

impl<'this, 'update_state, D, X, Y> LocationSelection<'this, 'update_state, D, X, Y>
where
    D: super::IntegerParameterRef,
    X: super::IntegerParameterRef,
    Y: super::IntegerParameterRef,
{
    pub fn new(
        update_state: &'this UpdateState<'update_state>,
        discriminant_parameter: D,
        x_parameter: X,
        y_parameter: Y,
    ) -> Self {
        Self {
            update_state,
            discriminant_parameter,
            x_parameter,
            y_parameter,
        }
    }

    /// Attempts to format the map location as two integers or two strings.
    ///
    /// This will fail if the map location does not have valid variables.
    pub fn fmt(&self) -> Option<LocationFmt> {
        let x = ValueSelection::new(
            self.update_state,
            &self.discriminant_parameter,
            &self.x_parameter,
        )
        .fmt()?;
        let y = ValueSelection::new(
            self.update_state,
            &self.discriminant_parameter,
            &self.y_parameter,
        )
        .fmt()?;
        match (x, y) {
            (ValueFmt::Constant(x), ValueFmt::Constant(y)) => Some(LocationFmt::Constant((x, y))),
            (ValueFmt::Variable(x), ValueFmt::Variable(y)) => Some(LocationFmt::Variable((x, y))),
            _ => None,
        }
    }

    /// Adds an additional parameter that controls the map on which the map location refers to.
    pub fn map_parameter<M>(
        self,
        map_parameter: M,
    ) -> LocationSelectionWithMap<'this, 'update_state, D, M, X, Y>
    where
        M: super::IntegerParameterRef,
    {
        LocationSelectionWithMap {
            inner: self,
            map_parameter,
        }
    }

    /// Prepares a [`egui::Widget`] from the map location.
    pub fn prepare<H>(
        self,
        id_salt: H,
    ) -> LocationSelectionPrepared<'this, 'update_state, D, X, Y, H>
    where
        D: super::IntegerParameterMut,
        X: super::IntegerParameterMut,
        Y: super::IntegerParameterMut,
        H: std::hash::Hash,
    {
        LocationSelectionPrepared {
            inner: self,
            id_salt,
        }
    }
}

impl<'this, 'update_state, D, M, X, Y> LocationSelectionWithMap<'this, 'update_state, D, M, X, Y>
where
    D: super::IntegerParameterRef,
    M: super::IntegerParameterRef,
    X: super::IntegerParameterRef,
    Y: super::IntegerParameterRef,
{
    /// Attempts to format the map (excluding the map location) as a string.
    ///
    /// This will fail if the map parameter does not refer to a valid map or variable.
    pub fn fmt(&self) -> Option<MapFmt> {
        match self.inner.discriminant_parameter.as_integer() {
            0 => MapSelection::new(self.inner.update_state, &self.map_parameter)
                .fmt()
                .map(MapFmt::Constant),
            1 => VariableSelection::new(self.inner.update_state, &self.map_parameter)
                .fmt()
                .map(MapFmt::Variable),
            _ => None,
        }
    }

    /// Prepares a [`egui::Widget`] from the map and map location.
    pub fn prepare<H>(
        self,
        id_salt: H,
    ) -> LocationSelectionWithMapPrepared<'this, 'update_state, D, M, X, Y, H>
    where
        D: super::IntegerParameterMut,
        M: super::IntegerParameterMut,
        X: super::IntegerParameterMut,
        Y: super::IntegerParameterMut,
        H: std::hash::Hash,
    {
        LocationSelectionWithMapPrepared {
            inner: self,
            id_salt,
        }
    }
}

impl<D, X, Y> LocationSelection<'_, '_, D, X, Y>
where
    D: super::IntegerParameterMut,
    X: super::IntegerParameterMut,
    Y: super::IntegerParameterMut,
{
    fn show_discriminant(&mut self, ui: &mut egui::Ui, id_salt: impl std::hash::Hash) -> bool {
        let mut modified = false;

        modified |= ui
            .add(EnumComboBox::new_with_conversion(
                PhantomData::<LocationType>,
                (id_salt, "discriminant"),
                self.discriminant_parameter.as_integer_mut(),
            ))
            .changed();

        modified
    }

    fn show_location(&mut self, ui: &mut egui::Ui, id_salt: impl std::hash::Hash) -> bool {
        let mut modified = false;

        match self.discriminant_parameter.as_integer() {
            0 => {
                ui.label("Map x-coordinate");
                modified |= ui
                    .add(
                        egui::DragValue::new(self.x_parameter.as_integer_mut()).range(0..=i32::MAX),
                    )
                    .changed();

                ui.label("Map y-coordinate");
                modified |= ui
                    .add(
                        egui::DragValue::new(self.y_parameter.as_integer_mut()).range(0..=i32::MAX),
                    )
                    .changed();
            }

            1 => {
                ui.label("Map x-coordinate");
                modified |= ui
                    .add(
                        VariableSelection::new(self.update_state, &mut self.x_parameter)
                            .prepare((&id_salt, 1, "x")),
                    )
                    .changed();

                ui.label("Map y-coordinate");
                modified |= ui
                    .add(
                        VariableSelection::new(self.update_state, &mut self.y_parameter)
                            .prepare((&id_salt, 1, "y")),
                    )
                    .changed();
            }

            _ => {}
        }

        modified
    }
}

impl<D, X, Y, H> egui::Widget for LocationSelectionPrepared<'_, '_, D, X, Y, H>
where
    D: super::IntegerParameterMut,
    X: super::IntegerParameterMut,
    Y: super::IntegerParameterMut,
    H: std::hash::Hash,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                modified |= self.inner.show_discriminant(ui, &self.id_salt);
                modified |= self.inner.show_location(ui, self.id_salt);
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}

impl<D, M, X, Y, H> egui::Widget for LocationSelectionWithMapPrepared<'_, '_, D, M, X, Y, H>
where
    D: super::IntegerParameterMut,
    M: super::IntegerParameterMut,
    X: super::IntegerParameterMut,
    Y: super::IntegerParameterMut,
    H: std::hash::Hash,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                modified |= self.inner.inner.show_discriminant(ui, &self.id_salt);

                match self.inner.inner.discriminant_parameter.as_integer() {
                    0 => {
                        ui.label("Map");
                        modified |= ui
                            .add(
                                MapSelection::new(
                                    self.inner.inner.update_state,
                                    self.inner.map_parameter,
                                )
                                .prepare((&self.id_salt, 0, "map")),
                            )
                            .changed();
                    }

                    1 => {
                        ui.label("Map ID");
                        modified |= ui
                            .add(
                                VariableSelection::new(
                                    self.inner.inner.update_state,
                                    self.inner.map_parameter,
                                )
                                .prepare((&self.id_salt, 1, "map")),
                            )
                            .changed();
                    }

                    _ => {}
                }

                modified |= self.inner.inner.show_location(ui, self.id_salt);
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
