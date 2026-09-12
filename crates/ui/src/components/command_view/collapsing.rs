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

/// Helper for showing a collapsing view in the command view with the correct UI styling.
#[must_use = "call `.show_header()` or `.show_header_text()` and then `.body()` to build the collapsing view"]
pub(super) struct Collapsing<'a> {
    layout: egui::Layout,
    id: egui::Id,
    ui: &'a mut egui::Ui,
    stripe: bool,
    expand_by_default: bool,
}

#[ouroboros::self_referencing]
struct CollapsingHeaderResponseInner<H> {
    frame: egui::frame::Prepared,
    #[borrows(mut frame)]
    #[covariant]
    maybe_header_response: Option<egui::collapsing_header::HeaderResponse<'this, H>>,
}

#[must_use = "call `.body()` to build the collapsing view"]
pub(super) struct CollapsingHeaderResponse<'a, H> {
    inner: CollapsingHeaderResponseInner<H>,
    ui: &'a mut egui::Ui,
}

impl<'a> Collapsing<'a> {
    /// Creates a new [`Collapsing`].
    ///
    /// `stripe_ref` is a reference to a [`bool`] indicating whether or not this collapsing view has
    /// a faint background. Its value will be inverted when this method is called and its value
    /// prior to the inversion will be used: `true` indicates a faint background and `false`
    /// indicates a normal background.
    pub fn new(ui: &'a mut egui::Ui, stripe_ref: &mut bool) -> Self {
        let stripe = *stripe_ref;
        *stripe_ref = !*stripe_ref;
        Self {
            layout: *ui.layout(),
            id: ui.id(),
            ui,
            stripe,
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
    pub fn show_header<H>(
        self,
        add_contents: impl FnOnce(&mut egui::Ui) -> H,
    ) -> CollapsingHeaderResponse<'a, H> {
        CollapsingHeaderResponse {
            inner: CollapsingHeaderResponseInnerBuilder {
                frame: egui::Frame::NONE
                    .fill(if self.stripe {
                        self.ui
                            .visuals()
                            .window_fill()
                            .blend(self.ui.visuals().faint_bg_color)
                    } else {
                        self.ui.visuals().window_fill()
                    })
                    .begin(self.ui),
                maybe_header_response_builder: |frame| {
                    Some(
                        egui::collapsing_header::CollapsingState::load_with_default_open(
                            frame.content_ui.ctx(),
                            self.id,
                            self.expand_by_default,
                        )
                        .show_header(&mut frame.content_ui, |ui| {
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
                                    add_contents(ui)
                                },
                            )
                            .inner
                        }),
                    )
                },
            }
            .build(),
            ui: self.ui,
        }
    }

    /// Shows the header of the collapsing view with the given text.
    pub fn show_header_text(
        self,
        text: impl Into<egui::WidgetText>,
    ) -> CollapsingHeaderResponse<'a, egui::Response> {
        self.show_header(|ui| ui.label(text))
    }
}

impl<H> CollapsingHeaderResponse<'_, H> {
    /// Shows the body of the collapsing view with the given contents.
    pub fn body<B>(
        mut self,
        add_contents: impl FnOnce(&mut egui::Ui) -> B,
    ) -> (
        egui::Response,
        egui::InnerResponse<H>,
        Option<egui::InnerResponse<B>>,
    ) {
        let ret = self
            .inner
            .with_maybe_header_response_mut(|maybe_header_response| maybe_header_response.take())
            .unwrap()
            .body(add_contents);
        self.inner.into_heads().frame.end(self.ui);
        ret
    }
}
