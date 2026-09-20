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
#![cfg_attr(target_arch = "wasm32", allow(clippy::arc_with_non_send_sync))]

/// Syntax highlighter
pub mod syntax_highlighting;

/// The tilemap.
mod map_view;
pub use map_view::{MapView, SelectedLayer};
mod tilepicker;
pub use tilepicker::{SelectedTile, Tilepicker};

mod sound_tab;
pub use sound_tab::SoundTab;

pub mod command_view;
pub use command_view::{CommandView, EventInfo};

mod filesystem_view;
pub use filesystem_view::FileSystemView;

mod database_view;
pub use database_view::DatabaseView;

mod collapsing_view;
pub use collapsing_view::CollapsingView;

mod animation_frame_view;
pub use animation_frame_view::AnimationFrameView;
mod cellpicker;
pub use cellpicker::Cellpicker;

mod troop_view;
pub use troop_view::TroopView;

pub mod combo_box;
pub use combo_box::ComboBox;
pub mod enum_combo_box;
pub use enum_combo_box::EnumComboBox;
pub mod file_combo_box;
pub use file_combo_box::FileComboBox;
pub mod id_vec;
pub use id_vec::{IdVecPlusMinusSelection, IdVecSelection, RankSelection};
pub mod optional_id_combo_box;
pub use optional_id_combo_box::OptionalIdComboBox;

mod ui_ext;
pub use ui_ext::UiExt;

pub struct EnumMenuButton<'e, T> {
    current_value: &'e mut T,
    id: egui::Id,
}

impl<'e, T> EnumMenuButton<'e, T> {
    pub fn new(current_value: &'e mut T, id_salt: impl std::hash::Hash) -> Self {
        Self {
            current_value,
            id: egui::Id::new(id_salt),
        }
    }
}

impl<T: ToString + PartialEq + strum::IntoEnumIterator> egui::Widget for EnumMenuButton<'_, T> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::ComboBox::from_id_salt(self.id)
            .selected_text(self.current_value.to_string())
            .show_ui(ui, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

                for variant in T::iter() {
                    let text = variant.to_string();
                    ui.selectable_value(self.current_value, variant, text);
                }
            })
            .response
    }
}

pub struct EnumRadioList<'e, T> {
    current_value: &'e mut T,
}

impl<'e, T> EnumRadioList<'e, T> {
    pub fn new(current_value: &'e mut T) -> Self {
        Self { current_value }
    }
}

impl<T: ToString + PartialEq + strum::IntoEnumIterator> egui::Widget for EnumRadioList<'_, T> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut changed = false;
        let mut response = ui
            .vertical(|ui| {
                ui.with_cross_justify(|ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

                    for variant in T::iter() {
                        let text = variant.to_string();
                        if ui.radio_value(self.current_value, variant, text).changed() {
                            changed = true;
                        }
                    }
                });
            })
            .response;
        if changed {
            response.mark_changed();
        }
        response
    }
}

pub struct Field<T> {
    name: String,
    widget: T,
}
impl<T> Field<T>
where
    T: egui::Widget,
{
    /// Creates a new vertical input widget with specified name.
    // * Design notes:
    // * Why not use `ToString` trait in `name` argument? Isn't it specifically built for casting to a string?
    // * Yes, but there's a fundamental differences between `to_string` and `into`, which has to do with move semantics.
    // * TLDR; `to_string` simply creates a string without consuming the original value, which may result in failed to move exceptions.
    // *       `into`, on the other hand, *consumes* the value and converts it to a `String`.
    // * It's not like we're going to use `name` argument after creating the field, so, we can consume it.
    pub fn new(name: impl Into<String>, widget: T) -> Self {
        Self {
            name: name.into(),
            widget,
        }
    }
}

impl<T> egui::Widget for Field<T>
where
    T: egui::Widget,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut changed = false;
        let mut response = ui
            .vertical(|ui| {
                let spacing = ui.spacing().item_spacing.y;
                ui.add_space(spacing);
                ui.add(egui::Label::new(format!("{}:", self.name)).truncate());
                if ui.add(self.widget).changed() {
                    changed = true;
                };
                ui.add_space(spacing);
            })
            .response;
        if changed {
            response.mark_changed();
        }
        response
    }
}

pub struct FieldWithCheckbox<'a, T> {
    name: String,
    checked: &'a mut bool,
    widget: T,
}
impl<'a, T> FieldWithCheckbox<'a, T>
where
    T: egui::Widget,
{
    pub fn new(name: impl Into<String>, checked: &'a mut bool, widget: T) -> Self {
        Self {
            name: name.into(),
            checked,
            widget,
        }
    }
}

impl<T> egui::Widget for FieldWithCheckbox<'_, T>
where
    T: egui::Widget,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut changed = false;
        let mut response = ui
            .vertical(|ui| {
                let spacing = ui.spacing().item_spacing.y;
                ui.add_space(spacing);
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(format!("{}:", self.name)).truncate());
                    ui.add(egui::Checkbox::without_text(self.checked));
                });
                if ui.add_enabled(*self.checked, self.widget).changed() {
                    changed = true;
                };
                ui.add_space(spacing);
            })
            .response;
        if changed {
            response.mark_changed();
        }
        response
    }
}

pub fn close_options_ui(ui: &mut egui::Ui, open: &mut bool, save: &mut bool) {
    ui.horizontal(|ui| {
        if ui.button("Ok").clicked() {
            *open = false;
            *save = true;
        }

        if ui.button("Cancel").clicked() {
            *open = false;
        }

        if ui.button("Apply").clicked() {
            *save = true;
        }
    });
}

pub fn colored_text(text: impl Into<String>, color: egui::Color32) -> egui::RichText {
    egui::RichText::new(text).color(color)
}
