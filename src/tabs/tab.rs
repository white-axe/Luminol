// Copyright (C) 2023 Lily Lyons
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

use parking_lot::Mutex;
use std::hash::Hash;

/// Helper struct for tabs.
pub struct Tabs<T> {
    state: Mutex<egui_dock::dock_state::DockState<T>>,
    id: egui::Id,
}

impl<T> Tabs<T>
where
    T: Tab,
{
    /// Create a new Tab viewer without any tabs.
    pub fn new(id: impl Hash, tabs: Vec<T>) -> Self {
        Self {
            id: egui::Id::new(id),
            state: egui_dock::dock_state::DockState::new(tabs).into(),
        }
    }

    /// Display all tabs.
    pub fn ui(&self, ui: &mut egui::Ui) {
        // This scroll area with hidden scrollbars is a hacky workaround for
        // https://github.com/Adanos020/egui_dock/issues/90
        // which, for us, seems to manifest when the user moves tabs around
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                let mut style = egui_dock::Style::from_egui(ui.style());
                style.overlay.surface_fade_opacity = 1.;

                let mut state = self.state.lock();
                let focused_id = if ui.memory(|m| m.focus().is_none()) {
                    state.find_active_focused().map(|(_, t)| t.id())
                } else {
                    None
                };
                egui_dock::DockArea::new(&mut state)
                    .id(self.id)
                    .style(style)
                    .show_inside(
                        ui,
                        &mut TabViewer {
                            focused_id,
                            marker: std::marker::PhantomData,
                        },
                    );
            });
    }

    /// Add a tab.
    pub fn add_tab(&self, tab: T) {
        let mut state = self.state.lock();
        for n in state.iter_nodes() {
            if let egui_dock::Node::Leaf { tabs, .. } = n {
                if tabs.iter().any(|t| t.id() == tab.id()) {
                    return;
                }
            }
        }
        state.push_to_focused_leaf(tab);
    }

    /// Removes tabs that the provided closure returns `false` when called.
    pub fn clean_tabs<F: FnMut(&mut T) -> bool>(&self, mut f: F) {
        let mut i = 0;
        let mut state = self.state.lock();

        let focused_id = state.find_active_focused().map(|(_, tab)| tab.id());
        let mut active_tab = None;

        loop {
            let Some(surface) = state.get_surface_mut(egui_dock::SurfaceIndex(i)) else {
                break;
            };

            if let Some(tree) = surface.node_tree_mut() {
                let mut is_empty = !egui_dock::SurfaceIndex(i).is_main();

                for (j, node) in tree.iter_mut().enumerate() {
                    if let egui_dock::Node::Leaf { tabs, .. } = node {
                        tabs.retain_mut(&mut f);

                        if !tabs.is_empty() {
                            is_empty = false;
                        }

                        // Check if the originally focused tab is inside this node
                        if let Some((k, _)) = focused_id
                            .and_then(|id| tabs.iter().enumerate().find(|(_, tab)| tab.id() == id))
                        {
                            active_tab = Some((i, j, k));
                        }
                    }
                }

                // We need to remove empty windows or we'll cause a crash
                if is_empty {
                    state.remove_surface(egui_dock::SurfaceIndex(i));
                }
            }
            i += 1;
        }

        // If the previously focused tab hasn't been removed, refocus it since the index may have
        // changed, otherwise the application may crash due to out-of-bounds vector access
        if let Some((i, j, k)) = active_tab {
            state.set_active_tab((
                egui_dock::SurfaceIndex(i),
                egui_dock::NodeIndex(j),
                egui_dock::TabIndex(k),
            ));
        }
        // Otherwise we need to unfocus all tabs, again to prevent a crash
        else {
            state.set_focused_node_and_surface((
                egui_dock::SurfaceIndex(usize::MAX),
                egui_dock::NodeIndex(usize::MAX),
            ));
        }
    }

    /// Returns the name of the focused tab.
    pub fn focused_name(&self) -> Option<String> {
        let mut state = self.state.lock();
        state.find_active_focused().map(|(_, t)| t.name())
    }
}

struct TabViewer<T: Tab> {
    focused_id: Option<egui::Id>,

    // we don't actually own any types of T, but we use them in TabViewer
    // *const is used here to avoid needing lifetimes and to indicate to the drop checker that we don't own any types of T
    marker: std::marker::PhantomData<*const T>,
}

impl<T> egui_dock::TabViewer for TabViewer<T>
where
    T: Tab,
{
    type Tab = T;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.name().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let id = tab.id();
        ui.push_id(id, |ui| {
            tab.show(
                ui,
                self.focused_id.is_some_and(|focused_id| focused_id == id),
            )
        });
    }

    fn force_close(&mut self, tab: &mut Self::Tab) -> bool {
        tab.force_close()
    }

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        // We need to disable scroll bars for at least the map editor because otherwise it'll start
        // jiggling when the screen or tab is resized. We're not making that type of game.
        [false, false]
    }
}

/// A tab trait.
pub trait Tab {
    /// Optionally used as the title of the tab.
    fn name(&self) -> String {
        "Untitled Window".to_string()
    }

    /// Required to prevent duplication.
    fn id(&self) -> egui::Id;

    /// Show this tab.
    fn show(&mut self, ui: &mut egui::Ui, is_focused: bool);

    /// Does this tab need the filesystem?
    fn requires_filesystem(&self) -> bool {
        false
    }

    /// Does this tab need to be closed?
    fn force_close(&mut self) -> bool {
        false
    }
}

impl Tab for Box<dyn Tab + Send> {
    fn force_close(&mut self) -> bool {
        self.as_mut().force_close()
    }

    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn id(&self) -> egui::Id {
        self.as_ref().id()
    }

    fn requires_filesystem(&self) -> bool {
        self.as_ref().requires_filesystem()
    }

    fn show(&mut self, ui: &mut egui::Ui, is_focused: bool) {
        self.as_mut().show(ui, is_focused)
    }
}
