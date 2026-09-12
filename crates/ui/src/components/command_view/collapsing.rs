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

/// Helper for showing a collapsing view in the command view with the correct UI layout.
pub(super) struct Collapsing<'a> {
    ui: &'a mut egui::Ui,
    id: egui::Id,
    layout: egui::Layout,
    expand_by_default: bool,
}

impl<'a> Collapsing<'a> {
    /// Creates a new [`Collapsing`].
    pub fn new(ui: &'a mut egui::Ui) -> Self {
        Self {
            layout: *ui.layout(),
            id: ui.id(),
            ui,
            expand_by_default: true,
        }
    }

    /// Sets the ID that will be used to store UI state to `id`.
    pub fn id(mut self, id: egui::Id) -> Self {
        self.id = id;
        self
    }

    /// Sets the ID that will be used to store UI state to the current ID with `id_salt` added to
    /// it.
    ///
    /// If the ID has not previously been set, it defaults to `ui.id()`.
    pub fn id_salt(mut self, id_salt: impl std::hash::Hash) -> Self {
        self.id = self.id.with(id_salt);
        self
    }

    /// Sets whether this collapsing view is expanded (`true`) or collapsed (`false`) by default.
    ///
    /// By default, it is expanded by default.
    pub fn expand_by_default(mut self, expand_by_default: bool) -> Self {
        self.expand_by_default = expand_by_default;
        self
    }

    /// Shows the header of the collapsing view with the given contents.
    pub fn show_header<R>(
        self,
        f: impl FnOnce(&mut egui::Ui) -> R,
    ) -> egui::collapsing_header::HeaderResponse<'a, R> {
        egui::collapsing_header::CollapsingState::load_with_default_open(
            self.ui.ctx(),
            self.id,
            self.expand_by_default,
        )
        .show_header(self.ui, |ui| {
            ui.with_layout(
                egui::Layout {
                    main_dir: egui::Direction::LeftToRight,
                    main_wrap: false,
                    main_align: egui::Align::Min,
                    main_justify: self.layout.cross_justify,
                    cross_align: egui::Align::Center,
                    cross_justify: self.layout.main_justify,
                },
                |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                    f(ui)
                },
            )
            .inner
        })
    }
}
