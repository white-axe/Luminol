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

use itertools::Itertools;

pub struct FileSystemView<T> {
    arena: indextree::Arena<Entry>,
    id: egui::Id,
    filesystem: T,
    root_name: String,
    root_node_id: indextree::NodeId,
}

enum Entry {
    File {
        /// Name of this file with extension.
        name: String,
        /// Whether or not this file is selected in the filesystem view.
        selected: bool,
    },
    Dir {
        /// Name of this directory.
        name: String,
        /// Whether or not we've cached the contents of this directory.
        initialized: bool,
        /// If all of this directory's contents are selected, this will be `Some(true)`.
        /// If all of this directory's contents are deselected this will be `Some(false)`.
        /// Otherwise, this will be `None`.
        selected: Option<bool>,
        /// Whether or not the subtree for this directory is expanded.
        expanded: bool,
    },
}

impl Entry {
    fn name(&self) -> &str {
        match self {
            Entry::File { name, .. } => name,
            Entry::Dir { name, .. } => name,
        }
    }
}

impl<T> FileSystemView<T>
where
    T: luminol_filesystem::FileSystem,
{
    pub fn new(id: egui::Id, filesystem: T, root_name: String) -> Self {
        let mut arena = indextree::Arena::new();
        let root_node_id = arena.new_node(Entry::Dir {
            name: "".to_string(),
            initialized: false,
            selected: Some(false),
            expanded: true,
        });
        Self {
            arena,
            id,
            filesystem,
            root_name,
            root_node_id,
        }
    }

    pub fn filesystem(&self) -> &T {
        &self.filesystem
    }

    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> luminol_filesystem::Result<()> {
        self.render_subtree(ui, self.id, self.root_node_id, &self.root_name.to_string())
    }

    fn render_subtree(
        &mut self,
        ui: &mut egui::Ui,
        id: egui::Id,
        node_id: indextree::NodeId,
        name: &str,
    ) -> luminol_filesystem::Result<()> {
        match self.arena[node_id].get_mut() {
            Entry::Dir {
                initialized: initialized @ false,
                ..
            } => {
                *initialized = true;

                let mut ancestors = node_id
                    .ancestors(&self.arena)
                    .filter(|p| !self.arena[*p].get().name().is_empty())
                    .collect_vec();
                ancestors.reverse();
                let path = ancestors
                    .into_iter()
                    .map(|n| self.arena[n].get().name())
                    .join("/");

                let mut subentries = self.filesystem.read_dir(path)?;
                subentries.sort_unstable_by(|a, b| {
                    let path_a = a.path.iter().last().unwrap();
                    let path_b = b.path.iter().last().unwrap();
                    path_a.partial_cmp(path_b).unwrap()
                });

                for subentry in subentries {
                    let subentry_name = subentry.path.iter().last().unwrap().to_string();
                    if subentry.metadata.is_file {
                        node_id.append_value(
                            Entry::File {
                                name: subentry_name,
                                selected: false,
                            },
                            &mut self.arena,
                        );
                    } else {
                        node_id.append_value(
                            Entry::Dir {
                                name: subentry_name,
                                initialized: false,
                                selected: Some(false),
                                expanded: false,
                            },
                            &mut self.arena,
                        );
                    }
                }
            }
            _ => {}
        }

        match self.arena[node_id].get_mut() {
            Entry::File { name, selected } => {
                ui.add(egui::SelectableLabel::new(*selected, name.to_string()));
            }
            Entry::Dir {
                selected, expanded, ..
            } => {
                let id = id.with(name);

                let header = egui::collapsing_header::CollapsingState::load_with_default_open(
                    ui.ctx(),
                    id,
                    *expanded,
                );

                *expanded = header.openness(ui.ctx()) >= 1.;

                let response = header
                    .show_header(ui, |ui| {
                        ui.add(egui::SelectableLabel::new(
                            selected.is_some_and(|s| s),
                            format!(
                                "{}{}",
                                match *selected {
                                    Some(true) => "▣   ",
                                    Some(false) => "☐   ",
                                    None => "⊟   ",
                                },
                                name
                            ),
                        ));
                    })
                    .body::<luminol_filesystem::Result<()>>(|ui| {
                        for node_id in node_id.children(&self.arena).collect_vec() {
                            self.render_subtree(
                                ui,
                                id,
                                node_id,
                                &self.arena[node_id].get().name().to_string(),
                            )?;
                        }
                        Ok(())
                    });

                if let Some(o) = response.2 {
                    o.inner?;
                }
            }
        }

        Ok(())
    }
}
