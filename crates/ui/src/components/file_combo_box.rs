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
            remove_extension: true,
            allow_none: true,
        }
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

impl<R, P, H> egui::Widget for FileComboBox<'_, '_, R, P, H>
where
    R: FileCast,
    P: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let reference_file_stem = self.reference.get().map(|filename| {
            camino::Utf8Path::new(filename)
                .file_stem()
                .unwrap_or_default()
                .to_lowercase()
        });
        let selected_text = self.reference.get().map(|filename| filename.to_string());
        let widget = super::ComboBox::new(self.id_salt)
            .allow_none(self.allow_none && R::allow_none())
            .with_argument(self.update_state)
            .with_state_or_insert_with(|data| {
                let mut filenames = data
                    .argument
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
                filenames
            })
            .choices_with(|data| 0..data.state.len())
            .selected_text(selected_text)
            .prepare(
                |data, &filename_index| data.state[filename_index].clone(),
                |data, maybe_filename_index| {
                    reference_file_stem
                        == maybe_filename_index.map(|&filename_index| {
                            camino::Utf8Path::new(&data.state[filename_index])
                                .file_stem()
                                .unwrap_or_default()
                                .to_lowercase()
                        })
                },
                |data, maybe_filename_index| {
                    self.reference.set(
                        maybe_filename_index
                            .map(|&filename_index| data.state[filename_index].clone()),
                    );
                },
            );
        egui::Widget::ui(widget, ui)
    }
}
