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

/// Helper for determining the width of the description text in event command editors.
pub(super) struct DescriptionWidthCallback<'a> {
    ui: &'a egui::Ui,
    name: &'a str,
}

impl<'a> DescriptionWidthCallback<'a> {
    pub fn new(ui: &'a mut egui::Ui, name: &'a str) -> Self {
        Self { ui, name }
    }

    /// Returns `true` if the description is too long to fit in the label for this event command
    /// editor, otherwise `false`.
    pub fn is_truncated(&self, description: &str) -> bool {
        self.ui.text_width(
            if description.is_empty() {
                self.name.into()
            } else {
                format!("{}: {}", self.name, description)
            },
            egui::FontSelection::Default,
        ) > self.ui.available_width()
    }

    /// Returns a short prefix of `description_chars` such that `is_truncated` returns `true`.
    pub fn exponential_search_chars(
        &self,
        description_chars: impl Iterator<Item = char> + Clone,
    ) -> String {
        let mut length_in_chars = 16;
        loop {
            let text: String = description_chars.clone().take(length_in_chars).collect();
            if text.chars().count() < length_in_chars || self.is_truncated(&text) {
                return text;
            }
            length_in_chars *= 2;
        }
    }

    /// Returns a short prefix of `description_segments` such that `is_truncated` returns `true`.
    pub fn exponential_search_segments<'b>(
        &self,
        description_segments: impl Iterator<Item = &'b str> + Clone,
    ) -> String {
        self.exponential_search_chars(description_segments.flat_map(|segment| segment.chars()))
    }

    /// Returns a short prefix of `description` such that `is_truncated` returns `true`.
    pub fn exponential_search_str(&self, description: &str) -> String {
        self.exponential_search_chars(description.chars())
    }
}
