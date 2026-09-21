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
use std::marker::PhantomData;

/// A generic combo box widget with a search box.
#[must_use]
pub struct ComboBox<IdSalt> {
    id_salt: IdSalt,
    allow_none: bool,
    is_stale: bool,
    max_width: f32,
}

#[must_use]
pub struct ComboBoxWithArgument<Inner, Argument> {
    inner: Inner,
    argument: Argument,
}

#[must_use]
pub struct ComboBoxWithState<Inner, State, StateInitializer> {
    inner: Inner,
    state_type: PhantomData<State>,
    state_initializer: StateInitializer,
}

#[must_use]
pub struct ComboBoxWithSelectedText<Inner, SelectedText, SelectedTextFactory> {
    inner: Inner,
    selected_text_type: PhantomData<SelectedText>,
    selected_text_factory: SelectedTextFactory,
}

#[must_use]
pub struct ComboBoxWithChoices<Inner, Choice, ChoiceIter, ChoiceIterFactory> {
    inner: Inner,
    choice_type: PhantomData<Choice>,
    choice_iter_type: PhantomData<ChoiceIter>,
    choice_iter_factory: ChoiceIterFactory,
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct ComboBoxPrepared<Inner, ChoiceText, ChoiceFormatter, ChoiceIsSelected, OnChoiceSelect> {
    inner: Inner,
    choice_text_type: PhantomData<ChoiceText>,
    choice_formatter: ChoiceFormatter,
    choice_is_selected: ChoiceIsSelected,
    on_choice_select: OnChoiceSelect,
}

/// The data passed to the combo box's callbacks.
pub struct ComboBoxData<'a, Argument, State> {
    /// The argument that was passed to [`ComboBox::with_argument`] when building the combo box, or
    /// nothing if the combo box does not have an argument.
    pub argument: &'a mut Argument,
    /// The current combo box state or the default as created by
    /// [`ComboBoxWithArgument::with_state_or_insert_with`],
    /// [`ComboBoxWithArgument::with_state_or_insert`] or
    /// [`ComboBoxWithArgument::with_state_or_insert_default`], or nothing if the combo box does not
    /// have state.
    pub state: &'a mut State,
}

impl<IdSalt> ComboBox<IdSalt>
where
    IdSalt: std::hash::Hash,
{
    /// Creates a combo box.
    pub fn new(id_salt: IdSalt) -> Self {
        Self {
            id_salt,
            allow_none: false,
            is_stale: false,
            max_width: f32::INFINITY,
        }
    }

    /// Sets whether or not the "(None)" option in the combo box is selectable.
    ///
    /// The default is `false`.
    pub fn allow_none(mut self, allow_none: bool) -> Self {
        self.allow_none = allow_none;
        self
    }

    /// If set to `true`, the choices are stale and need to be updated this frame using the iterator
    /// of choices.
    ///
    /// The default is `false`.
    pub fn is_stale(mut self, is_stale: bool) -> Self {
        self.is_stale = is_stale;
        self
    }

    /// Sets the maximum allowed width of this widget.
    ///
    /// The default is `f32::INFINITY`.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the argument passed to the combo box's callbacks.
    pub fn with_argument<Argument>(
        self,
        argument: Argument,
    ) -> ComboBoxWithArgument<Self, Argument> {
        ComboBoxWithArgument {
            inner: self,
            argument,
        }
    }

    /// Indicates that the combo box's callbacks don't need to receive an argument.
    pub fn without_argument(self) -> ComboBoxWithArgument<Self, ()> {
        self.with_argument(())
    }
}

impl<Inner, Argument> ComboBoxWithArgument<Inner, Argument> {
    /// If the combo box for the state has never been retrieved, creates it with an initializer.
    pub fn with_state_or_insert_with<State, StateInitializer>(
        self,
        state_initializer: StateInitializer,
    ) -> ComboBoxWithState<Self, State, StateInitializer>
    where
        State: Send + Sync + 'static,
        StateInitializer: FnOnce(ComboBoxData<'_, Argument, ()>) -> State,
    {
        ComboBoxWithState {
            inner: self,
            state_type: PhantomData,
            state_initializer,
        }
    }

    /// If the combo box for the state has never been retrieved, uses this value by default.
    pub fn with_state_or_insert<State>(
        self,
        state: State,
    ) -> ComboBoxWithState<Self, State, impl FnOnce(ComboBoxData<'_, Argument, ()>) -> State>
    where
        State: Send + Sync + 'static,
    {
        self.with_state_or_insert_with(|_data| state)
    }

    /// If the combo box for the state has never been retrieved, uses the default value.
    pub fn with_state_or_insert_default<State>(
        self,
    ) -> ComboBoxWithState<Self, State, impl FnOnce(ComboBoxData<'_, Argument, ()>) -> State>
    where
        State: Default + Send + Sync + 'static,
    {
        self.with_state_or_insert_with(|_data| Default::default())
    }

    /// Indicates that the combo box does not have state.
    pub fn without_state(
        self,
    ) -> ComboBoxWithState<Self, (), impl FnOnce(ComboBoxData<'_, Argument, ()>)> {
        self.with_state_or_insert(())
    }
}

impl<IdSalt, Argument, State, StateInitializer>
    ComboBoxWithState<ComboBoxWithArgument<ComboBox<IdSalt>, Argument>, State, StateInitializer>
{
    /// Sets the combo box's currently selected value using a closure that takes [`ComboBoxData`].
    pub fn selected_text_with<SelectedText, SelectedTextFactory>(
        self,
        selected_text_factory: SelectedTextFactory,
    ) -> ComboBoxWithSelectedText<Self, SelectedText, SelectedTextFactory>
    where
        SelectedText: Into<egui::WidgetText>,
        SelectedTextFactory: FnOnce(ComboBoxData<'_, Argument, State>) -> Option<SelectedText>,
    {
        ComboBoxWithSelectedText {
            inner: self,
            selected_text_type: PhantomData,
            selected_text_factory,
        }
    }

    /// Sets the combo box's currently selected value.
    pub fn selected_text<SelectedText>(
        self,
        selected_text: Option<SelectedText>,
    ) -> ComboBoxWithSelectedText<
        Self,
        SelectedText,
        impl FnOnce(ComboBoxData<'_, Argument, State>) -> Option<SelectedText>,
    >
    where
        SelectedText: Into<egui::WidgetText>,
    {
        self.selected_text_with(|_data| selected_text)
    }
}

impl<IdSalt, Argument, State, StateInitializer, SelectedText, SelectedTextFactory>
    ComboBoxWithSelectedText<
        ComboBoxWithState<
            ComboBoxWithArgument<ComboBox<IdSalt>, Argument>,
            State,
            StateInitializer,
        >,
        SelectedText,
        SelectedTextFactory,
    >
{
    /// Sets the choices that the combo box will show using a closure that takes [`ComboBoxData`].
    pub fn choices_with<Choice, ChoiceIter, ChoiceIterFactory>(
        self,
        choice_iter_factory: ChoiceIterFactory,
    ) -> ComboBoxWithChoices<Self, Choice, ChoiceIter, ChoiceIterFactory>
    where
        Choice: Send + Sync + 'static,
        ChoiceIter: Iterator<Item = Choice>,
        ChoiceIterFactory: FnOnce(ComboBoxData<'_, Argument, State>) -> ChoiceIter,
    {
        ComboBoxWithChoices {
            inner: self,
            choice_type: PhantomData,
            choice_iter_type: PhantomData,
            choice_iter_factory,
        }
    }

    /// Sets the choices that the combo box will show.
    pub fn choices<Choice, ChoiceIter>(
        self,
        choice_iter: ChoiceIter,
    ) -> ComboBoxWithChoices<
        Self,
        Choice,
        ChoiceIter,
        impl FnOnce(ComboBoxData<'_, Argument, State>) -> ChoiceIter,
    >
    where
        Choice: Send + Sync + 'static,
        ChoiceIter: Iterator<Item = Choice>,
    {
        self.choices_with(|_data| choice_iter)
    }
}

impl<
        IdSalt,
        Argument,
        State,
        StateInitializer,
        SelectedText,
        SelectedTextFactory,
        Choice,
        ChoiceIter,
        ChoiceIterFactory,
    >
    ComboBoxWithChoices<
        ComboBoxWithSelectedText<
            ComboBoxWithState<
                ComboBoxWithArgument<ComboBox<IdSalt>, Argument>,
                State,
                StateInitializer,
            >,
            SelectedText,
            SelectedTextFactory,
        >,
        Choice,
        ChoiceIter,
        ChoiceIterFactory,
    >
{
    /// Sets the callbacks for the combo box.
    ///
    /// `choice_formatter`: Used to convert choices into the strings that will be shown in the combo
    /// box.
    ///
    /// `choice_is_selected`: Returns whether or not the given choice is selected.
    ///
    /// `on_choice_select`: This will be run when the user clicks on one of the choices in the combo
    /// box.
    pub fn prepare<ChoiceText, ChoiceFormatter, ChoiceIsSelected, OnChoiceSelect>(
        self,
        choice_formatter: ChoiceFormatter,
        choice_is_selected: ChoiceIsSelected,
        on_choice_select: OnChoiceSelect,
    ) -> ComboBoxPrepared<Self, ChoiceText, ChoiceFormatter, ChoiceIsSelected, OnChoiceSelect>
    where
        ChoiceText: AsRef<str> + Into<egui::WidgetText>,
        ChoiceFormatter: FnMut(ComboBoxData<'_, Argument, State>, &Choice) -> ChoiceText,
        ChoiceIsSelected: FnMut(ComboBoxData<'_, Argument, State>, Option<&Choice>) -> bool,
        OnChoiceSelect: FnOnce(ComboBoxData<'_, Argument, State>, Option<&Choice>),
    {
        ComboBoxPrepared {
            inner: self,
            choice_text_type: PhantomData,
            choice_formatter,
            choice_is_selected,
            on_choice_select,
        }
    }
}

struct ComboBoxStateInner<Choice, State> {
    inner: State,
    /// Number of choices there are in total, including ones that are not matched by the text in the
    /// search box.
    total_choices: usize,
    /// The current text inside of the search box.
    search_string: String,
    /// Vector of choices that are matched by the text in the search box.
    search_matched_choices: Vec<Choice>,
    /// This is initially set to false, and gets set to true once the combo box has scrolled to the
    /// selected choice upon first opening.
    scrolled_to_selected_choice: bool,
}

struct ComboBoxState<Choice, State>(Option<ComboBoxStateInner<Choice, State>>);

impl<Choice, State> Default for ComboBoxState<Choice, State> {
    fn default() -> Self {
        Self(None)
    }
}

impl<Choice, State> Clone for ComboBoxState<Choice, State> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<
        IdSalt,
        Argument,
        State,
        StateInitializer,
        SelectedText,
        SelectedTextFactory,
        Choice,
        ChoiceIter,
        ChoiceIterFactory,
        ChoiceText,
        ChoiceFormatter,
        ChoiceIsSelected,
        OnChoiceSelect,
    > egui::Widget
    for ComboBoxPrepared<
        ComboBoxWithChoices<
            ComboBoxWithSelectedText<
                ComboBoxWithState<
                    ComboBoxWithArgument<ComboBox<IdSalt>, Argument>,
                    State,
                    StateInitializer,
                >,
                SelectedText,
                SelectedTextFactory,
            >,
            Choice,
            ChoiceIter,
            ChoiceIterFactory,
        >,
        ChoiceText,
        ChoiceFormatter,
        ChoiceIsSelected,
        OnChoiceSelect,
    >
where
    IdSalt: std::hash::Hash,
    State: Send + Sync + 'static,
    StateInitializer: FnOnce(ComboBoxData<'_, Argument, ()>) -> State,
    SelectedText: Into<egui::WidgetText>,
    SelectedTextFactory: FnOnce(ComboBoxData<'_, Argument, State>) -> Option<SelectedText>,
    Choice: Send + Sync + 'static,
    ChoiceIter: Iterator<Item = Choice>,
    ChoiceIterFactory: FnOnce(ComboBoxData<'_, Argument, State>) -> ChoiceIter,
    ChoiceText: AsRef<str> + Into<egui::WidgetText>,
    ChoiceFormatter: FnMut(ComboBoxData<'_, Argument, State>, &Choice) -> ChoiceText,
    ChoiceIsSelected: FnMut(ComboBoxData<'_, Argument, State>, Option<&Choice>) -> bool,
    OnChoiceSelect: FnOnce(ComboBoxData<'_, Argument, State>, Option<&Choice>),
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let salt = egui::Id::new(&self.inner.inner.inner.inner.inner.id_salt);
        let state_id = ui.make_persistent_id(salt).with("luminol_combo_box");
        let popup_id = ui.make_persistent_id(salt).with("popup");
        let is_popup_open = egui::Popup::is_id_open(ui.ctx(), popup_id);

        let mut changed = false;

        let argument = &mut self.inner.inner.inner.inner.argument;

        let mut choice_iter_factory = Some(self.inner.choice_iter_factory);

        let mut state = is_popup_open
            .then(|| ui.data_mut(|d| d.remove_temp::<ComboBoxState<Choice, State>>(state_id)))
            .flatten()
            .and_then(|inner| inner.0)
            .unwrap_or_else(|| {
                let mut inner = (self.inner.inner.inner.state_initializer)(ComboBoxData {
                    argument,
                    state: &mut (),
                });
                let choices: Vec<_> = choice_iter_factory.take().unwrap()(ComboBoxData {
                    argument,
                    state: &mut inner,
                })
                .collect();
                ComboBoxStateInner {
                    inner,
                    total_choices: choices.len(),
                    search_string: String::new(),
                    search_matched_choices: choices,
                    scrolled_to_selected_choice: false,
                }
            });

        let available_width = ui.available_width() - ui.spacing().item_spacing.x;
        let width = self
            .inner
            .inner
            .inner
            .inner
            .inner
            .max_width
            .min(available_width);

        let mut on_choice_select = Some(self.on_choice_select);

        let mut inner_response =
            egui::ComboBox::from_id_salt(&self.inner.inner.inner.inner.inner.id_salt)
                .wrap()
                .width(width)
                .selected_text(
                    (self.inner.inner.selected_text_factory)(ComboBoxData {
                        argument,
                        state: &mut state.inner,
                    })
                    .map_or_else(|| "(None)".into(), Into::into),
                )
                .show_ui(ui, |ui| {
                    let button_height = ui.spacing().interact_size.y.max(
                        ui.text_style_height(&egui::TextStyle::Button)
                            + 2. * ui.spacing().button_padding.y,
                    );
                    let spacing = ui.spacing().item_spacing.y;

                    let mut update_choices = |state: &mut ComboBoxStateInner<Choice, State>,
                                              choice_iter_factory: &mut Option<
                        ChoiceIterFactory,
                    >| {
                        if let Some(choice_iter_factory) = choice_iter_factory.take() {
                            let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
                            state.total_choices = 0;
                            state.search_matched_choices.clear();
                            state.search_matched_choices.extend(
                                choice_iter_factory(ComboBoxData {
                                    argument,
                                    state: &mut state.inner,
                                })
                                .filter(|choice| {
                                    state.total_choices += 1;
                                    matcher
                                        .fuzzy(
                                            (self.choice_formatter)(
                                                ComboBoxData {
                                                    argument,
                                                    state: &mut state.inner,
                                                },
                                                choice,
                                            )
                                            .as_ref(),
                                            &state.search_string,
                                            false,
                                        )
                                        .is_some()
                                }),
                            );
                        }
                    };

                    if self.inner.inner.inner.inner.inner.is_stale {
                        update_choices(&mut state, &mut choice_iter_factory);
                    }

                    let have_search_box = !state.search_string.is_empty()
                        || state.total_choices as f32 * (button_height + spacing)
                            > ui.available_height();
                    let search_box_clicked = if have_search_box {
                        let search_box_response = ui.add(
                            egui::TextEdit::singleline(&mut state.search_string)
                                .hint_text("Search 🔎"),
                        );

                        ui.add_space(spacing);

                        // If the combo box popup was not open the previous frame and was opened this
                        // frame, focus the search box
                        if !is_popup_open {
                            search_box_response.request_focus();
                        }

                        // If the user edited the contents of the search box, recalculate the search results
                        if search_box_response.changed() {
                            update_choices(&mut state, &mut choice_iter_factory);
                        }

                        search_box_response.clicked()
                            || search_box_response.secondary_clicked()
                            || search_box_response.middle_clicked()
                            || search_box_response.clicked_by(egui::PointerButton::Extra1)
                            || search_box_response.clicked_by(egui::PointerButton::Extra2)
                    } else {
                        false
                    };

                    let mut scroll_area_output = egui::ScrollArea::vertical()
                        .auto_shrink([!have_search_box; 2])
                        .show_rows(
                            ui,
                            button_height,
                            state.search_matched_choices.len()
                                + self.inner.inner.inner.inner.inner.allow_none as usize,
                            |ui, range| {
                                let mut is_faint = range.clone().start % 2 != 0;

                                if self.inner.inner.inner.inner.inner.allow_none
                                    && range.clone().start == 0
                                {
                                    if ui
                                        .with_stripe(is_faint, |ui| {
                                            ui.style_mut().wrap_mode =
                                                Some(egui::TextWrapMode::Truncate);
                                            ui.selectable_label(
                                                (self.choice_is_selected)(
                                                    ComboBoxData {
                                                        argument,
                                                        state: &mut state.inner,
                                                    },
                                                    None,
                                                ),
                                                "(None)",
                                            )
                                        })
                                        .inner
                                        .clicked()
                                    {
                                        if let Some(on_choice_select) = on_choice_select.take() {
                                            changed = true;
                                            on_choice_select(
                                                ComboBoxData {
                                                    argument,
                                                    state: &mut state.inner,
                                                },
                                                None,
                                            );
                                        }
                                    }
                                    is_faint = !is_faint;
                                }

                                for choice in range.filter_map(|i| {
                                    if self.inner.inner.inner.inner.inner.allow_none {
                                        (i != 0).then(|| &state.search_matched_choices[i - 1])
                                    } else {
                                        Some(&state.search_matched_choices[i])
                                    }
                                }) {
                                    ui.with_stripe(is_faint, |ui| {
                                        ui.style_mut().wrap_mode =
                                            Some(egui::TextWrapMode::Truncate);
                                        if ui
                                            .selectable_label(
                                                (self.choice_is_selected)(
                                                    ComboBoxData {
                                                        argument,
                                                        state: &mut state.inner,
                                                    },
                                                    Some(choice),
                                                ),
                                                (self.choice_formatter)(
                                                    ComboBoxData {
                                                        argument,
                                                        state: &mut state.inner,
                                                    },
                                                    choice,
                                                )
                                                .into(),
                                            )
                                            .clicked()
                                        {
                                            if let Some(on_choice_select) = on_choice_select.take()
                                            {
                                                changed = true;
                                                on_choice_select(
                                                    ComboBoxData {
                                                        argument,
                                                        state: &mut state.inner,
                                                    },
                                                    Some(choice),
                                                );
                                            }
                                        }
                                    });
                                    is_faint = !is_faint;
                                }
                            },
                        );

                    // Scroll the selected choice into view if we haven't already
                    if !state.scrolled_to_selected_choice {
                        state.scrolled_to_selected_choice = true;
                        let selected_index = if self.inner.inner.inner.inner.inner.allow_none
                            && (self.choice_is_selected)(
                                ComboBoxData {
                                    argument,
                                    state: &mut state.inner,
                                },
                                None,
                            ) {
                            Some(0)
                        } else {
                            state
                                .search_matched_choices
                                .iter()
                                .position(|choice| {
                                    (self.choice_is_selected)(
                                        ComboBoxData {
                                            argument,
                                            state: &mut state.inner,
                                        },
                                        Some(choice),
                                    )
                                })
                                .map(|selected_index| {
                                    if self.inner.inner.inner.inner.inner.allow_none {
                                        selected_index + 1
                                    } else {
                                        selected_index
                                    }
                                })
                        };
                        if let Some(selected_index) = selected_index {
                            let max = selected_index as f32 * (button_height + spacing) + spacing;
                            let min = selected_index as f32 * (button_height + spacing)
                                + button_height
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
                    ui.data_mut(|d| {
                        d.insert_temp::<ComboBoxState<Choice, State>>(
                            state_id,
                            ComboBoxState(Some(state)),
                        )
                    });

                    search_box_clicked
                });

        if inner_response.inner == Some(true) {
            // Force the combo box to stay open if the search box was clicked
            egui::Popup::open_id(ui.ctx(), popup_id);
        } else if inner_response.inner.is_none() {
            // Clear the state if the combo box is closed
            ui.data_mut(|d| d.remove_temp::<ComboBoxState<Choice, State>>(state_id));
        }

        if changed {
            inner_response.response.mark_changed();
        }
        inner_response.response
    }
}
