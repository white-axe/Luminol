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
    audio_selection::AudioSelection,
    character_selection::CharacterSelection,
    collapsing::Collapsing,
    database_selection::{
        ActorSelection, ArmorSelection, CommonEventSelection, ItemSelection, SkillSelection,
        StateSelection, SwitchSelection, VariableSelection, WeaponSelection,
    },
    description_width_callback::DescriptionWidthCallback,
    graphic_selection::GraphicSelection,
    location_selection::{LocationFmt, LocationSelection, MapFmt},
    value_selection::{ValueFmt, ValueSelection},
    CommandView, EventCommand, EventInfo, ParameterType, UpdateState,
};

pub trait EventCommandEditor
where
    Self: Sync + 'static,
{
    /// Returns whether or not the UI for this event command editor should be expanded by default.
    ///
    /// The default is to not expand by default.
    fn expand_by_default(&self) -> bool {
        false
    }

    /// Returns the name of the event command that this event command editor edits.
    fn name(&self) -> &'static str;

    /// Returns a description of the given event command.
    ///
    /// By default, there is no description.
    ///
    /// If the description can be long, for optimization purposes, `callback` can be used to
    /// determine whether or not the description is short enough to fit in the UI widget where the
    /// description will be displayed.
    #[allow(unused_variables)]
    fn description(
        &self,
        callback: DescriptionWidthCallback<'_>,
        update_state: &mut UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &EventCommand,
    ) -> Option<String> {
        None
    }

    /// Renders the UI for this event command editor.
    ///
    /// Remember to mark the response returned by this method as changed if the event command was
    /// modified by this editor (by calling the `mark_changed` method of the response).
    ///
    /// The event command is guaranteed to match the schema for the command's event code (i.e. the
    /// `matches_schema` field on the command will be `true`). If there is no schema for the
    /// command's command code, this will never be called.
    fn ui(
        &self,
        ui: &mut egui::Ui,
        stripe: &mut bool,
        update_state: &mut UpdateState<'_>,
        event_info: Option<&EventInfo<'_>>,
        command: &mut EventCommand,
    ) -> egui::Response;
}

mod c101;
mod c102;
mod c103;
mod c104;
mod c105;
mod c106;
mod c111;
mod c112;
mod c113;
mod c115;
mod c116;
mod c117;
mod c118;
mod c119;
mod c121;
mod c122;
mod c123;
mod c124;
mod c125;
mod c126;
mod c127;
mod c128;
mod c129;
mod c131;
mod c132;
mod c133;
mod c134;
mod c135;
mod c136;
mod c201;
mod c202;

/// A mapping from event command codes to the [`EventCommandEditor`] for that event command.
pub static EDITORS: phf::Map<u16, &dyn EventCommandEditor> = phf::phf_map! {
    101u16 => &c101::Editor { continuation_code: 401, name: "Show Text" },
    102u16 => &c102::Editor,
    103u16 => &c103::Editor,
    104u16 => &c104::Editor,
    105u16 => &c105::Editor,
    106u16 => &c106::Editor,
    108u16 => &c101::Editor { continuation_code: 408, name: "Comment" },
    111u16 => &c111::Editor,
    112u16 => &c112::Editor,
    113u16 => &c113::Editor,
    115u16 => &c115::Editor,
    116u16 => &c116::Editor,
    117u16 => &c117::Editor,
    118u16 => &c118::Editor,
    119u16 => &c119::Editor,
    121u16 => &c121::Editor,
    122u16 => &c122::Editor,
    123u16 => &c123::Editor,
    124u16 => &c124::Editor,
    125u16 => &c125::Editor,
    126u16 => &c126::Editor,
    127u16 => &c127::Editor,
    128u16 => &c128::Editor,
    129u16 => &c129::Editor,
    131u16 => &c131::Editor,
    132u16 => &c132::Editor,
    133u16 => &c133::Editor,
    134u16 => &c134::Editor,
    135u16 => &c135::Editor,
    136u16 => &c136::Editor,
    201u16 => &c201::Editor,
    202u16 => &c202::Editor,
    355u16 => &c101::Editor { continuation_code: 655, name: "Script" },
};
