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

use super::UiExt;
use itertools::Itertools;

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

pub struct OptionalIdComboBox<'a, R, I, H, F> {
    id_salt: H,
    reference: &'a mut R,
    id_iter: I,
    formatter: F,
    search_needs_update: bool,
    allow_none: bool,
}

#[derive(Default, Clone)]
struct State {
    search_string: String,
    search_matched_ids: Vec<usize>,
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
            search_needs_update: *update_state.modified_during_prev_frame,
            allow_none: true,
        }
    }

    fn ui_inner(
        self,
        ui: &mut egui::Ui,
        formatter: impl Fn(&Self) -> String,
        f: impl FnOnce(Self, &mut egui::Ui, Vec<usize>, bool, bool) -> bool,
    ) -> egui::Response {
        let salt = egui::Id::new(&self.id_salt);
        let state_id = ui.make_persistent_id(salt).with("OptionalIdComboBox");
        let popup_id = ui.make_persistent_id(salt).with("popup");
        let is_popup_open = egui::Popup::is_id_open(ui.ctx(), popup_id);

        let mut changed = false;
        let inner_response = egui::ComboBox::from_id_salt(&self.id_salt)
            .wrap()
            .width(ui.available_width() - ui.spacing().item_spacing.x)
            .selected_text(formatter(&self))
            .show_ui(ui, |ui| {
                // Get cached search string and search matches from egui memory
                let mut state: State = is_popup_open
                    .then(|| ui.data_mut(|d| d.remove_temp(state_id)))
                    .flatten()
                    .unwrap_or_else(|| State {
                        search_string: String::new(),
                        search_matched_ids: self.id_iter.clone().collect(),
                    });

                let search_box_response = ui.add(
                    egui::TextEdit::singleline(&mut state.search_string).hint_text("Search 🔎"),
                );

                ui.add_space(ui.spacing().item_spacing.y);

                // If the combo box popup was not open the previous frame and was opened this
                // frame, focus the search box
                if !is_popup_open {
                    search_box_response.request_focus();
                }

                let search_box_clicked = search_box_response.clicked()
                    || search_box_response.secondary_clicked()
                    || search_box_response.middle_clicked()
                    || search_box_response.clicked_by(egui::PointerButton::Extra1)
                    || search_box_response.clicked_by(egui::PointerButton::Extra2);

                // If the user edited the contents of the search box or if the data cache changed
                // this frame, recalculate the search results
                let search_needs_update = self.search_needs_update || search_box_response.changed();
                if search_needs_update {
                    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
                    state.search_matched_ids.clear();
                    state
                        .search_matched_ids
                        .extend(self.id_iter.clone().filter(|id| {
                            matcher
                                .fuzzy(&(self.formatter)(*id), &state.search_string, false)
                                .is_some()
                        }));
                }

                let button_height = ui.spacing().interact_size.y.max(
                    ui.text_style_height(&egui::TextStyle::Button)
                        + 2. * ui.spacing().button_padding.y,
                );
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    button_height,
                    state.search_matched_ids.len() + self.allow_none as usize,
                    |ui, range| {
                        let first_row_is_faint = range.clone().start % 2 != 0;
                        let show_none = self.allow_none && range.clone().start == 0;
                        let ids = range
                            .filter_map(|i| {
                                if self.allow_none {
                                    (i != 0).then(|| state.search_matched_ids[i - 1])
                                } else {
                                    Some(state.search_matched_ids[i])
                                }
                            })
                            .collect_vec();
                        changed = f(self, ui, ids, first_row_is_faint, show_none);
                    },
                );

                // Save the search string and the search results back into egui memory
                ui.data_mut(|d| d.insert_temp(state_id, state));

                search_box_clicked
            });
        let mut response = inner_response.response;

        if inner_response.inner == Some(true) {
            // Force the combo box to stay open if the search box was clicked
            egui::Popup::open_id(ui.ctx(), popup_id);
        } else if inner_response.inner.is_none() {
            // Clear the search box if the combo box is closed
            ui.data_mut(|d| d.remove_temp::<State>(state_id));
        }

        if changed {
            response.mark_changed();
        }
        response
    }
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

impl<T, I, H, F> egui::Widget for OptionalIdComboBox<'_, Option<T>, I, H, F>
where
    T: IdCast,
    I: Iterator<Item = usize> + Clone,
    H: std::hash::Hash,
    F: Fn(usize) -> String,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut changed = false;

        self.ui_inner(
            ui,
            |this| {
                if let Some(id) = this.reference.as_ref() {
                    if let Some(id) = id.to_id() {
                        (this.formatter)(id)
                    } else {
                        "".into()
                    }
                } else {
                    "(None)".into()
                }
            },
            |this, ui, ids, first_row_is_faint, show_none| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

                if show_none
                    && ui
                        .with_stripe(false, |ui| {
                            ui.selectable_label(this.reference.is_none(), "(None)")
                        })
                        .inner
                        .clicked()
                {
                    *this.reference = None;
                    changed = true;
                }

                let reference_id = this.reference.as_ref().and_then(|inner| inner.to_id());

                let mut is_faint = first_row_is_faint != show_none;

                for id in ids {
                    ui.with_stripe(is_faint, |ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

                        if ui
                            .selectable_label(reference_id == Some(id), (this.formatter)(id))
                            .clicked()
                        {
                            if let Some(new_value) = IdCast::from_id(id) {
                                *this.reference = Some(new_value);
                                changed = true;
                            }
                        }
                    });
                    is_faint = !is_faint;
                }

                changed
            },
        )
    }
}

impl<T, I, H, F> egui::Widget for OptionalIdComboBox<'_, T, I, H, F>
where
    T: IdCast,
    I: Iterator<Item = usize> + Clone,
    H: std::hash::Hash,
    F: Fn(usize) -> String,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        self.allow_none = false;

        let mut changed = false;

        self.ui_inner(
            ui,
            |this| {
                if let Some(id) = this.reference.to_id() {
                    (this.formatter)(id)
                } else {
                    "".into()
                }
            },
            |this, ui, ids, first_row_is_faint, _| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

                let reference_id = this.reference.to_id();

                let mut is_faint = first_row_is_faint;

                for id in ids {
                    ui.with_stripe(is_faint, |ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

                        if ui
                            .selectable_label(reference_id == Some(id), (this.formatter)(id))
                            .clicked()
                        {
                            if let Some(new_value) = IdCast::from_id(id) {
                                *this.reference = new_value;
                                changed = true;
                            }
                        }
                    });
                    is_faint = !is_faint;
                }

                changed
            },
        )
    }
}
