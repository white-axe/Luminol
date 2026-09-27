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

use super::{
    IntegerParameterMut, IntegerParameterRef, MapSelection, ValueFmt, ValueSelection,
    VariableSelection,
};
use crate::components::EnumComboBox;
use luminol_core::UpdateState;
use std::{
    borrow::{Borrow, BorrowMut},
    marker::PhantomData,
};

#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
    strum::EnumIter
)]
#[repr(i32)]
pub enum LocationType {
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
#[must_use = "call `.fmt()` to convert to a `String` or `.prepare()` or `.prepare_with_custom_enum()` to convert to a `egui::Widget`"]
pub struct LocationSelection<U, D, X, Y> {
    update_state: U,
    discriminant_parameter: D,
    x_parameter: X,
    y_parameter: Y,
}

/// A widget for changing the value of four parameters that determine a map and coordinates.
#[must_use = "call `.fmt()` to convert to a `String` or `.prepare()` or `.prepare_with_custom_enum()` to convert to a `egui::Widget`"]
pub struct LocationSelectionWithMap<U, D, M, X, Y> {
    inner: LocationSelection<U, D, X, Y>,
    map_parameter: M,
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct LocationSelectionPrepared<U, D, X, Y, H, E> {
    inner: LocationSelection<U, D, X, Y>,
    id_salt: H,
    enum_type: PhantomData<E>,
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct LocationSelectionWithMapPrepared<U, D, M, X, Y, H, E> {
    inner: LocationSelectionWithMap<U, D, M, X, Y>,
    id_salt: H,
    enum_type: PhantomData<E>,
}

impl<'a, U, D, X, Y> LocationSelection<U, D, X, Y>
where
    U: Borrow<UpdateState<'a>>,
    D: IntegerParameterRef,
    X: IntegerParameterRef,
    Y: IntegerParameterRef,
{
    pub fn new(update_state: U, discriminant_parameter: D, x_parameter: X, y_parameter: Y) -> Self {
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
            self.update_state.borrow(),
            &self.discriminant_parameter,
            &self.x_parameter,
        )
        .fmt()?;
        let y = ValueSelection::new(
            self.update_state.borrow(),
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
    pub fn map_parameter<M>(self, map_parameter: M) -> LocationSelectionWithMap<U, D, M, X, Y>
    where
        M: IntegerParameterRef,
    {
        LocationSelectionWithMap {
            inner: self,
            map_parameter,
        }
    }

    /// Prepares a [`egui::Widget`] from the map location.
    pub fn prepare<H>(self, id_salt: H) -> LocationSelectionPrepared<U, D, X, Y, H, LocationType>
    where
        U: BorrowMut<UpdateState<'a>>,
        D: IntegerParameterMut,
        X: IntegerParameterMut,
        Y: IntegerParameterMut,
        H: std::hash::Hash,
    {
        self.prepare_with_custom_enum(id_salt, PhantomData)
    }

    /// Prepares a [`egui::Widget`] from the map location, with a custom enum type for the combo
    /// box.
    pub fn prepare_with_custom_enum<H, E>(
        self,
        id_salt: H,
        enum_type: PhantomData<E>,
    ) -> LocationSelectionPrepared<U, D, X, Y, H, E>
    where
        U: BorrowMut<UpdateState<'a>>,
        D: IntegerParameterMut,
        X: IntegerParameterMut,
        Y: IntegerParameterMut,
        H: std::hash::Hash,
    {
        LocationSelectionPrepared {
            inner: self,
            id_salt,
            enum_type,
        }
    }
}

impl<'a, U, D, M, X, Y> LocationSelectionWithMap<U, D, M, X, Y>
where
    U: Borrow<UpdateState<'a>>,
    D: IntegerParameterRef,
    M: IntegerParameterRef,
    X: IntegerParameterRef,
    Y: IntegerParameterRef,
{
    /// Attempts to format the map (excluding the map location) as a string.
    ///
    /// This will fail if the map parameter does not refer to a valid map or variable.
    pub fn fmt(&self) -> Option<MapFmt> {
        match self.inner.discriminant_parameter.as_integer() {
            0 => MapSelection::new(self.inner.update_state.borrow(), &self.map_parameter)
                .fmt()
                .map(MapFmt::Constant),
            1 => VariableSelection::new(self.inner.update_state.borrow(), &self.map_parameter)
                .fmt()
                .map(MapFmt::Variable),
            _ => None,
        }
    }

    /// Prepares a [`egui::Widget`] from the map and map location.
    pub fn prepare<H>(
        self,
        id_salt: H,
    ) -> LocationSelectionWithMapPrepared<U, D, M, X, Y, H, LocationType>
    where
        U: BorrowMut<UpdateState<'a>>,
        D: IntegerParameterMut,
        M: IntegerParameterMut,
        X: IntegerParameterMut,
        Y: IntegerParameterMut,
        H: std::hash::Hash,
    {
        self.prepare_with_custom_enum(id_salt, PhantomData)
    }

    /// Prepares a [`egui::Widget`] from the map location, with a custom enum type for the combo
    /// box.
    pub fn prepare_with_custom_enum<H, E>(
        self,
        id_salt: H,
        enum_type: PhantomData<E>,
    ) -> LocationSelectionWithMapPrepared<U, D, M, X, Y, H, E>
    where
        U: BorrowMut<UpdateState<'a>>,
        D: IntegerParameterMut,
        M: IntegerParameterMut,
        X: IntegerParameterMut,
        Y: IntegerParameterMut,
        H: std::hash::Hash,
    {
        LocationSelectionWithMapPrepared {
            inner: self,
            id_salt,
            enum_type,
        }
    }
}

impl<'a, U, D, X, Y> LocationSelection<U, D, X, Y>
where
    U: BorrowMut<UpdateState<'a>>,
    D: IntegerParameterMut,
    X: IntegerParameterMut,
    Y: IntegerParameterMut,
{
    fn show_discriminant<E, F, H>(
        &mut self,
        ui: &mut egui::Ui,
        id_salt: H,
        enum_type: PhantomData<E>,
    ) -> bool
    where
        H: std::hash::Hash,
        E: Into<i32> + Send + Sync + ToString + strum::IntoEnumIterator + 'static,
        i32: TryInto<E, Error = F> + Clone,
    {
        let mut modified = false;

        modified |= ui
            .add(EnumComboBox::new_with_conversion(
                enum_type,
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
                        VariableSelection::new(self.update_state.borrow(), &mut self.x_parameter)
                            .prepare((&id_salt, 1, "x")),
                    )
                    .changed();

                ui.label("Map y-coordinate");
                modified |= ui
                    .add(
                        VariableSelection::new(self.update_state.borrow(), &mut self.y_parameter)
                            .prepare((&id_salt, 1, "y")),
                    )
                    .changed();
            }

            _ => {}
        }

        modified
    }

    fn show_picker<M>(
        &mut self,
        ui: &mut egui::Ui,
        id_salt: impl std::hash::Hash,
        maybe_map_parameter: Option<M>,
    ) -> bool
    where
        M: IntegerParameterMut,
    {
        let mut modified = false;

        let picker_id = ui.id().with((id_salt, "picker"));
        let update_state = self.update_state.borrow_mut();
        if let Some(picker) = update_state
            .map_location_picker
            .as_ref()
            .filter(|picker| picker.id == picker_id)
        {
            egui::Frame::NONE.show(ui, |ui| {
                ui.style_mut()
                    .visuals
                    .widgets
                    .noninteractive
                    .bg_stroke
                    .color = ui.style().visuals.warn_fg_color;
                egui::Frame::group(ui.style())
                    .fill(
                        ui.visuals().gray_out(
                            ui.visuals()
                                .gray_out(ui.visuals().gray_out(ui.style().visuals.warn_fg_color)),
                        ),
                    )
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(
                            egui::RichText::new(
                                "Click on a tile in the map editor to choose a location",
                            )
                            .color(ui.style().visuals.warn_fg_color),
                        );
                    });
            });
            if ui.button("Cancel").clicked() {
                *update_state.map_location_picker = None;
            } else if let Some(map_id) = picker.map_id {
                if let Some(mut map_parameter) = maybe_map_parameter {
                    let map = map_parameter.as_integer_mut();
                    if *map != map_id {
                        *map = map_id;
                        modified = true;
                    }
                }
                {
                    let x = self.x_parameter.as_integer_mut();
                    if *x != picker.x {
                        *x = picker.x;
                        modified = true;
                    }
                }
                {
                    let y = self.y_parameter.as_integer_mut();
                    if *y != picker.y {
                        *y = picker.y;
                        modified = true;
                    }
                }
                *update_state.map_location_picker = None;
            }
        } else if ui.button("Choose location in map editor").clicked() {
            *update_state.map_location_picker = Some(luminol_core::MapLocationPicker {
                id: picker_id,
                map_id: None,
                x: self.x_parameter.as_integer(),
                y: self.y_parameter.as_integer(),
            });
        }

        modified
    }
}

impl<'a, U, D, X, Y, H, E, F> egui::Widget for LocationSelectionPrepared<U, D, X, Y, H, E>
where
    U: BorrowMut<UpdateState<'a>>,
    D: IntegerParameterMut,
    X: IntegerParameterMut,
    Y: IntegerParameterMut,
    H: std::hash::Hash,
    E: Into<i32> + Send + Sync + ToString + strum::IntoEnumIterator + 'static,
    i32: TryInto<E, Error = F> + Clone,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                modified |= self
                    .inner
                    .show_discriminant(ui, &self.id_salt, self.enum_type);

                if self.inner.discriminant_parameter.as_integer() == 0 {
                    modified |= self.inner.show_picker::<D>(ui, &self.id_salt, None);
                }

                modified |= self.inner.show_location(ui, self.id_salt);
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}

impl<'a, U, D, M, X, Y, H, E, F> egui::Widget
    for LocationSelectionWithMapPrepared<U, D, M, X, Y, H, E>
where
    U: BorrowMut<UpdateState<'a>>,
    D: IntegerParameterMut,
    M: IntegerParameterMut,
    X: IntegerParameterMut,
    Y: IntegerParameterMut,
    H: std::hash::Hash,
    E: Into<i32> + Send + Sync + ToString + strum::IntoEnumIterator + 'static,
    i32: TryInto<E, Error = F> + Clone,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut modified = false;

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                modified |= self
                    .inner
                    .inner
                    .show_discriminant(ui, &self.id_salt, self.enum_type);

                match self.inner.inner.discriminant_parameter.as_integer() {
                    0 => {
                        modified |= self.inner.inner.show_picker(
                            ui,
                            &self.id_salt,
                            Some(&mut self.inner.map_parameter),
                        );

                        ui.label("Map");
                        modified |= ui
                            .add(
                                MapSelection::new(
                                    self.inner.inner.update_state.borrow(),
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
                                    self.inner.inner.update_state.borrow(),
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
