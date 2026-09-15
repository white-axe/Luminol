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

use crate::components::FileComboBox;
use luminol_audio::Source;
use luminol_core::UpdateState;

/// A widget for changing the value of an audio file parameter.
#[must_use = "call `.fmt()` to convert to a `String` or `.prepare()` to convert to a `egui::Widget`"]
pub struct AudioSelection<P> {
    parameter: P,
}

#[must_use = "use `ui.add()` to show this widget"]
pub struct AudioSelectionPrepared<'this, 'update_state, P, D, H> {
    inner: AudioSelection<P>,
    update_state: &'this mut UpdateState<'update_state>,
    id_salt: H,
    directory_path: D,
    source: Option<Source>,
}

impl<P> AudioSelection<P>
where
    P: super::AudioFileParameterRef,
{
    pub fn new(parameter: P) -> Self {
        Self { parameter }
    }

    /// Formats the audio file as a string.
    pub fn fmt(&self) -> String {
        let audio_file = self.parameter.as_audio_file();
        let name = audio_file.name.0.unwrap_or("(None)".into());
        let volume = audio_file.volume;
        let pitch = audio_file.pitch;
        format!("{name}, {volume}%, {pitch}%")
    }

    /// Prepares a [`egui::Widget`] from the audio file.
    pub fn prepare<'this, 'update_state, D, H>(
        self,
        update_state: &'this mut UpdateState<'update_state>,
        id_salt: H,
        directory_path: D,
        source: Option<Source>,
    ) -> AudioSelectionPrepared<'this, 'update_state, P, D, H>
    where
        D: AsRef<camino::Utf8Path>,
        H: std::hash::Hash,
    {
        AudioSelectionPrepared {
            inner: self,
            update_state,
            id_salt,
            directory_path,
            source,
        }
    }
}

impl<P, D, H> egui::Widget for AudioSelectionPrepared<'_, '_, P, D, H>
where
    P: super::AudioFileParameterMut,
    D: AsRef<camino::Utf8Path>,
    H: std::hash::Hash,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut modified = false;

        let directory_path = self.directory_path.as_ref();
        let audio_file = self.inner.parameter.as_audio_file_mut();

        let mut response = egui::Frame::NONE
            .show(ui, |ui| {
                modified |= ui
                    .add(FileComboBox::new(
                        self.update_state,
                        (&self.id_salt, "filename"),
                        directory_path,
                        &mut audio_file.name,
                    ))
                    .changed();

                ui.columns(2, |columns| {
                    columns[0].label("Volume");
                    modified |= columns[0]
                        .add(
                            egui::DragValue::new(&mut audio_file.volume)
                                .range(0..=100)
                                .custom_formatter(|value, _| format!("{value}%")),
                        )
                        .changed();

                    columns[1].label("Pitch");
                    modified |= columns[1]
                        .add(
                            egui::DragValue::new(&mut audio_file.pitch)
                                .range(50..=150)
                                .custom_formatter(|value, _| format!("{value}%")),
                        )
                        .changed();
                });

                ui.columns(if self.source.is_some() { 2 } else { 1 }, |columns| {
                    if columns[0]
                        .add_enabled(audio_file.name.0.is_some(), egui::Button::new("Play"))
                        .clicked()
                        || (modified
                            && self
                                .source
                                .is_some_and(|source| self.update_state.audio.is_playing(source)))
                    {
                        if let Some(filename) = &audio_file.name.0 {
                            if let Err(e) = self.update_state.audio.play(
                                directory_path.join(filename),
                                self.update_state.filesystem,
                                audio_file.volume,
                                audio_file.pitch,
                                self.source,
                                self.update_state
                                    .project_config
                                    .as_ref()
                                    .expect("project not loaded")
                                    .project
                                    .volume_scale,
                            ) {
                                luminol_core::error!(self.update_state.toasts, e);
                            }
                        }
                    }

                    if let Some(source) = self.source {
                        if columns[1].button("Stop").clicked() {
                            self.update_state.audio.stop(source);
                        }
                    }
                });
            })
            .response;

        if modified {
            response.mark_changed();
        }
        response
    }
}
