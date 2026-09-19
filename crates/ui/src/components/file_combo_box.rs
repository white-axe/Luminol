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
use luminol_core::UpdateState;
use luminol_filesystem::FileSystem;

pub trait FileCast {
    fn allow_none() -> bool {
        false
    }

    fn get(&self) -> Option<&str>;

    fn set(&mut self, value: Option<String>);
}

impl<T> FileCast for Option<T>
where
    T: FileCast + Default,
{
    fn allow_none() -> bool {
        true
    }

    fn get(&self) -> Option<&str> {
        self.as_ref()?.get()
    }

    fn set(&mut self, value: Option<String>) {
        if value.is_some() {
            self.get_or_insert_default().set(value)
        } else {
            *self = None;
        }
    }
}

impl<T> FileCast for luminol_data::RpgOption<T>
where
    Option<T>: FileCast,
{
    fn allow_none() -> bool {
        <Option<T> as FileCast>::allow_none()
    }

    fn get(&self) -> Option<&str> {
        self.0.get()
    }

    fn set(&mut self, value: Option<String>) {
        self.0.set(value)
    }
}

impl FileCast for String {
    fn get(&self) -> Option<&str> {
        Some(self)
    }

    fn set(&mut self, value: Option<String>) {
        *self = value.unwrap_or_default();
    }
}

impl FileCast for camino::Utf8PathBuf {
    fn get(&self) -> Option<&str> {
        Some(self.as_str())
    }

    fn set(&mut self, value: Option<String>) {
        *self = value.unwrap_or_default().into()
    }
}

/// A combo box widget for changing the value of a field containing a filename of a file within a
/// specific subdirectory of the project.
#[must_use = "use `ui.add()` to show this widget"]
pub struct FileComboBox<'this, 'update_state, R, P, H> {
    update_state: &'this UpdateState<'update_state>,
    id_salt: H,
    directory_path: P,
    reference: &'this mut R,
    max_width: f32,
    remove_extension: bool,
    allow_none: bool,
}

impl<'this, 'update_state, R, P, H> FileComboBox<'this, 'update_state, R, P, H>
where
    R: FileCast,
    P: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    /// Creates a combo box that can be used to change a filename field.
    ///
    /// `directory_path` should be a relative path to a directory within the project, such as
    /// `"Audio/BGM"`.
    pub fn new(
        update_state: &'this UpdateState<'update_state>,
        id_salt: H,
        directory_path: P,
        reference: &'this mut R,
    ) -> Self {
        Self {
            update_state,
            id_salt,
            directory_path,
            reference,
            max_width: f32::INFINITY,
            remove_extension: true,
            allow_none: true,
        }
    }

    /// Sets the maximum allowed width of this widget.
    ///
    /// The default is `f32::INFINITY`.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets whether or not the "(None)" option can be selected if `reference` is an optional type.
    ///
    /// The default is `true`.
    pub fn allow_none(mut self, allow_none: bool) -> Self {
        self.allow_none = allow_none;
        self
    }

    /// Sets whether or not the file extensions are removed from the choices that appear in the
    /// combo box.
    ///
    /// The default is `true` (meaning the file extensions will be removed).
    pub fn remove_extension(mut self, remove_extension: bool) -> Self {
        self.remove_extension = remove_extension;
        self
    }
}

#[derive(Default, Clone)]
struct State {
    search_string: String,
    search_matched_ids: Vec<usize>,
    scrolled_to_selected_choice: bool,
    filenames: Vec<String>,
}

impl<R, P, H> egui::Widget for FileComboBox<'_, '_, R, P, H>
where
    R: FileCast,
    P: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let salt = egui::Id::new(&self.id_salt);
        let state_id = ui.make_persistent_id(salt).with("FileComboBox");
        let popup_id = ui.make_persistent_id(salt).with("popup");
        let is_popup_open = egui::Popup::is_id_open(ui.ctx(), popup_id);

        let allow_none = R::allow_none() && self.allow_none;

        let mut changed = false;

        let available_width = ui.available_width() - ui.spacing().item_spacing.x;
        let width = self.max_width.min(available_width);

        let mut inner_response = egui::ComboBox::from_id_salt(&self.id_salt)
            .wrap()
            .width(width)
            .selected_text(self.reference.get().unwrap_or("(None)"))
            .show_ui(ui, |ui| {
                let mut state: State = is_popup_open
                    .then(|| ui.data_mut(|d| d.remove_temp(state_id)))
                    .flatten()
                    .unwrap_or_else(|| {
                        let mut filenames = self
                            .update_state
                            .filesystem
                            .read_dir(self.directory_path)
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(|entry| {
                                entry.metadata.is_file.then(|| {
                                    if self.remove_extension {
                                        let mut path = camino::Utf8PathBuf::from(entry.name);
                                        path.set_extension("");
                                        path.into_string()
                                    } else {
                                        entry.name
                                    }
                                })
                            })
                            .collect::<Vec<_>>();
                        filenames.sort_unstable_by(|a, b| lexical_sort::natural_lexical_cmp(a, b));
                        State {
                            search_string: String::new(),
                            search_matched_ids: (0..filenames.len()).collect(),
                            scrolled_to_selected_choice: false,
                            filenames,
                        }
                    });

                let search_box_response = ui.add(
                    egui::TextEdit::singleline(&mut state.search_string).hint_text("Search 🔎"),
                );

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

                // If the user edited the contents of the search box, recalculate the search results
                if search_box_response.changed() {
                    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
                    state.search_matched_ids.clear();
                    state
                        .search_matched_ids
                        .extend(
                            state
                                .filenames
                                .iter()
                                .enumerate()
                                .filter_map(|(id, filename)| {
                                    matcher
                                        .fuzzy(filename, &state.search_string, false)
                                        .is_some()
                                        .then_some(id)
                                }),
                        );
                }

                let reference_file_stem = self.reference.get().map(|filename| {
                    camino::Utf8Path::new(filename)
                        .file_stem()
                        .unwrap_or_default()
                        .to_lowercase()
                });

                let button_height = ui.spacing().interact_size.y.max(
                    ui.text_style_height(&egui::TextStyle::Button)
                        + 2. * ui.spacing().button_padding.y,
                );
                let mut scroll_area_output = egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show_rows(
                        ui,
                        button_height,
                        state.search_matched_ids.len() + allow_none as usize,
                        |ui, range| {
                            let mut is_faint = range.clone().start % 2 != 0;

                            if allow_none && range.clone().start == 0 {
                                if ui
                                    .with_stripe(is_faint, |ui| {
                                        ui.style_mut().wrap_mode =
                                            Some(egui::TextWrapMode::Truncate);
                                        ui.selectable_label(reference_file_stem.is_none(), "(None)")
                                    })
                                    .inner
                                    .clicked()
                                {
                                    self.reference.set(None);
                                    changed = true;
                                }
                                is_faint = !is_faint;
                            }

                            for i in range.filter_map(|i| {
                                if allow_none {
                                    (i != 0).then(|| state.search_matched_ids[i - 1])
                                } else {
                                    Some(state.search_matched_ids[i])
                                }
                            }) {
                                ui.with_stripe(is_faint, |ui| {
                                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                                    let filename = &state.filenames[i];
                                    let file_stem = camino::Utf8Path::new(filename)
                                        .file_stem()
                                        .unwrap_or_default();
                                    if ui
                                        .selectable_label(
                                            Some(file_stem.to_lowercase()) == reference_file_stem,
                                            filename,
                                        )
                                        .clicked()
                                    {
                                        self.reference.set(Some(file_stem.to_string()));
                                        changed = true;
                                    }
                                });
                                is_faint = !is_faint;
                            }
                        },
                    );

                // Scroll the selected choice into view if we haven't already
                if !state.scrolled_to_selected_choice {
                    state.scrolled_to_selected_choice = true;
                    let selected_index =
                        if let Some(reference_file_stem) = reference_file_stem.as_ref() {
                            state
                                .search_matched_ids
                                .iter()
                                .copied()
                                .position(|id| {
                                    let filename = &state.filenames[id];
                                    let file_stem = camino::Utf8Path::new(filename)
                                        .file_stem()
                                        .unwrap_or_default();
                                    file_stem.to_lowercase() == *reference_file_stem
                                })
                                .map(|selected_index| {
                                    if allow_none {
                                        selected_index + 1
                                    } else {
                                        selected_index
                                    }
                                })
                        } else {
                            Some(0)
                        };
                    if let Some(selected_index) = selected_index {
                        let spacing = ui.spacing().item_spacing.y;
                        let max = selected_index as f32 * (button_height + spacing) + spacing;
                        let min = selected_index as f32 * (button_height + spacing) + button_height
                            - spacing
                            - scroll_area_output.inner_rect.height();
                        if scroll_area_output.state.offset.y > max {
                            scroll_area_output.state.offset.y = max;
                            scroll_area_output
                                .state
                                .store(ui.ctx(), scroll_area_output.id);
                        } else if scroll_area_output.state.offset.y < min {
                            scroll_area_output.state.offset.y = min;
                            scroll_area_output
                                .state
                                .store(ui.ctx(), scroll_area_output.id);
                        }
                    }
                }

                // Save the search string and the search results back into egui memory
                ui.data_mut(|d| d.insert_temp(state_id, state));

                search_box_clicked
            });

        if inner_response.inner == Some(true) {
            // Force the combo box to stay open if the search box was clicked
            egui::Popup::open_id(ui.ctx(), popup_id);
        } else if inner_response.inner.is_none() {
            // Clear the state if the combo box is closed
            ui.data_mut(|d| d.remove_temp::<State>(state_id));
        }

        if changed {
            inner_response.response.mark_changed();
        }
        inner_response.response
    }
}
