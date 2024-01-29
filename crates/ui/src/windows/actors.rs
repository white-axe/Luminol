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

use luminol_components::UiExt;

#[derive(Default)]
pub struct Window {
    selected_actor_name: Option<String>,
    previous_actor: Option<usize>,

    shields: Vec<usize>,
    helmets: Vec<usize>,
    body_armors: Vec<usize>,
    accessories: Vec<usize>,

    view: luminol_components::DatabaseView,
}

impl Window {
    pub fn new() -> Self {
        Default::default()
    }
}

impl luminol_core::Window for Window {
    fn name(&self) -> String {
        if let Some(name) = &self.selected_actor_name {
            format!("Editing actor {:?}", name)
        } else {
            "Actor Editor".into()
        }
    }

    fn id(&self) -> egui::Id {
        egui::Id::new("actor_editor")
    }

    fn requires_filesystem(&self) -> bool {
        true
    }

    fn show(
        &mut self,
        ctx: &egui::Context,
        open: &mut bool,
        update_state: &mut luminol_core::UpdateState<'_>,
    ) {
        let mut actors = update_state.data.actors();
        let classes = update_state.data.classes();
        let weapons = update_state.data.weapons();
        let armors = update_state.data.armors();

        self.shields.clear();
        self.shields.extend(armors.data.iter().filter_map(|a| {
            matches!(a.kind, luminol_data::rpg::armor::Kind::Shield).then_some(a.id)
        }));
        self.helmets.clear();
        self.helmets.extend(armors.data.iter().filter_map(|a| {
            matches!(a.kind, luminol_data::rpg::armor::Kind::Helmet).then_some(a.id)
        }));
        self.body_armors.clear();
        self.body_armors.extend(armors.data.iter().filter_map(|a| {
            matches!(a.kind, luminol_data::rpg::armor::Kind::BodyArmor).then_some(a.id)
        }));
        self.accessories.clear();
        self.accessories.extend(armors.data.iter().filter_map(|a| {
            matches!(a.kind, luminol_data::rpg::armor::Kind::Accessory).then_some(a.id)
        }));

        let mut modified = false;

        self.selected_actor_name = None;

        let response = egui::Window::new(self.name())
            .id(self.id())
            .default_width(500.)
            .open(open)
            .show(ctx, |ui| {
                self.view.show(
                    ui,
                    "Actors",
                    update_state
                        .project_config
                        .as_ref()
                        .expect("project not loaded"),
                    &mut actors.data,
                    |actor| format!("{:0>3}: {}", actor.id, actor.name),
                    |ui, actor| {
                        self.selected_actor_name = Some(actor.name.clone());

                        modified |= ui
                            .add(luminol_components::Field::new(
                                "Name",
                                egui::TextEdit::singleline(&mut actor.name)
                                    .desired_width(f32::INFINITY),
                            ))
                            .changed();

                        ui.with_stripe(true, |ui| {
                            modified |= ui
                                .add(luminol_components::Field::new(
                                    "Class",
                                    luminol_components::OptionalIdComboBox::new(
                                        (actor.id, "class"),
                                        &mut actor.class_id,
                                        classes.data.len(),
                                        |id| {
                                            classes.data.get(id).map_or_else(
                                                || "".into(),
                                                |c| format!("{id:0>3}: {}", c.name),
                                            )
                                        },
                                    ),
                                ))
                                .changed();
                        });

                        ui.with_stripe(false, |ui| {
                            modified |= ui
                                .add(luminol_components::Field::new(
                                    "Starting Weapon",
                                    |ui: &mut egui::Ui| {
                                        egui::Frame::none()
                                            .show(ui, |ui| {
                                                ui.columns(2, |columns| {
                                                    columns[0].add(
                                                        luminol_components::OptionalIdComboBox::new(
                                                            (actor.id, "weapon_id"),
                                                            &mut actor.weapon_id,
                                                            weapons.data.len(),
                                                            |id| {
                                                                weapons.data.get(id).map_or_else(
                                                                    || "".into(),
                                                                    |w| {
                                                                        format!(
                                                                            "{id:0>3}: {}",
                                                                            w.name
                                                                        )
                                                                    },
                                                                )
                                                            },
                                                        ),
                                                    );
                                                    columns[1]
                                                        .checkbox(&mut actor.weapon_fix, "Fixed");
                                                });
                                            })
                                            .response
                                    },
                                ))
                                .changed();
                        });

                        ui.with_stripe(true, |ui| {
                            modified |= ui
                                .add(luminol_components::Field::new(
                                    "Starting Shield",
                                    |ui: &mut egui::Ui| {
                                        egui::Frame::none()
                                            .show(ui, |ui| {
                                                ui.columns(2, |columns| {
                                                    columns[0].add(
                                                        luminol_components::OptionalIdComboBox::new(
                                                            (actor.id, "armor1_id"),
                                                            &mut actor.armor1_id,
                                                            self.shields.len(),
                                                            |id| {
                                                                self.shields.get(id).map_or_else(
                                                                    || "".into(),
                                                                    |i| {
                                                                        format!(
                                                                            "{i:0>3}: {}",
                                                                            armors.data[*i].name,
                                                                        )
                                                                    },
                                                                )
                                                            },
                                                        ),
                                                    );
                                                    columns[1]
                                                        .checkbox(&mut actor.armor1_fix, "Fixed");
                                                });
                                            })
                                            .response
                                    },
                                ))
                                .changed();
                        });

                        ui.with_stripe(false, |ui| {
                            modified |= ui
                                .add(luminol_components::Field::new(
                                    "Starting Helmet",
                                    |ui: &mut egui::Ui| {
                                        egui::Frame::none()
                                            .show(ui, |ui| {
                                                ui.columns(2, |columns| {
                                                    columns[0].add(
                                                        luminol_components::OptionalIdComboBox::new(
                                                            (actor.id, "armor2_id"),
                                                            &mut actor.armor2_id,
                                                            self.helmets.len(),
                                                            |id| {
                                                                self.helmets.get(id).map_or_else(
                                                                    || "".into(),
                                                                    |i| {
                                                                        format!(
                                                                            "{i:0>3}: {}",
                                                                            armors.data[*i].name,
                                                                        )
                                                                    },
                                                                )
                                                            },
                                                        ),
                                                    );
                                                    columns[1]
                                                        .checkbox(&mut actor.armor2_fix, "Fixed");
                                                });
                                            })
                                            .response
                                    },
                                ))
                                .changed();
                        });

                        ui.with_stripe(true, |ui| {
                            modified |= ui
                                .add(luminol_components::Field::new(
                                    "Starting Body Armor",
                                    |ui: &mut egui::Ui| {
                                        egui::Frame::none()
                                            .show(ui, |ui| {
                                                ui.columns(2, |columns| {
                                                    columns[0].add(
                                                        luminol_components::OptionalIdComboBox::new(
                                                            (actor.id, "armor3_id"),
                                                            &mut actor.armor3_id,
                                                            self.body_armors.len(),
                                                            |id| {
                                                                self.body_armors
                                                                    .get(id)
                                                                    .map_or_else(
                                                                        || "".into(),
                                                                        |i| {
                                                                            format!(
                                                                                "{i:0>3}: {}",
                                                                                armors.data[*i]
                                                                                    .name,
                                                                            )
                                                                        },
                                                                    )
                                                            },
                                                        ),
                                                    );
                                                    columns[1]
                                                        .checkbox(&mut actor.armor3_fix, "Fixed");
                                                });
                                            })
                                            .response
                                    },
                                ))
                                .changed();
                        });

                        ui.with_stripe(false, |ui| {
                            modified |= ui
                                .add(luminol_components::Field::new(
                                    "Starting Accessory",
                                    |ui: &mut egui::Ui| {
                                        egui::Frame::none()
                                            .show(ui, |ui| {
                                                ui.columns(2, |columns| {
                                                    columns[0].add(
                                                        luminol_components::OptionalIdComboBox::new(
                                                            (actor.id, "armor4_id"),
                                                            &mut actor.armor4_id,
                                                            self.accessories.len(),
                                                            |id| {
                                                                self.accessories
                                                                    .get(id)
                                                                    .map_or_else(
                                                                        || "".into(),
                                                                        |i| {
                                                                            format!(
                                                                                "{i:0>3}: {}",
                                                                                armors.data[*i]
                                                                                    .name,
                                                                            )
                                                                        },
                                                                    )
                                                            },
                                                        ),
                                                    );
                                                    columns[1]
                                                        .checkbox(&mut actor.armor4_fix, "Fixed");
                                                });
                                            })
                                            .response
                                    },
                                ))
                                .changed();
                        });

                        ui.with_stripe(true, |ui| {
                            ui.columns(2, |columns| {
                                modified |= columns[0]
                                    .add(luminol_components::Field::new(
                                        "Initial Level",
                                        egui::DragValue::new(&mut actor.initial_level)
                                            .clamp_range(1..=99),
                                    ))
                                    .changed();

                                modified |= columns[1]
                                    .add(luminol_components::Field::new(
                                        "Final Level",
                                        egui::DragValue::new(&mut actor.final_level)
                                            .clamp_range(1..=99),
                                    ))
                                    .changed();
                            });
                        });

                        ui.with_stripe(false, |ui| {
                            ui.columns(2, |columns| {
                                modified |= columns[0]
                                    .add(luminol_components::Field::new(
                                        "EXP Curve Basis",
                                        egui::DragValue::new(&mut actor.exp_basis)
                                            .clamp_range(10..=50),
                                    ))
                                    .changed();

                                modified |= columns[1]
                                    .add(luminol_components::Field::new(
                                        "EXP Curve Inflation",
                                        egui::DragValue::new(&mut actor.exp_inflation)
                                            .clamp_range(10..=50),
                                    ))
                                    .changed();
                            });
                        });

                        self.previous_actor = Some(actor.id);
                    },
                )
            });

        if response.is_some_and(|ir| ir.inner.is_some_and(|ir| ir.inner.modified)) {
            modified = true;
        }

        if modified {
            update_state.modified.set(true);
            actors.modified = true;
        }
    }
}
