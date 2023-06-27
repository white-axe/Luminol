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
use crate::prelude::*;

pub struct Window {
    system: rpg::System,
}

impl Default for Window {
    fn default() -> Self {
        let system = state!().data_cache.system().clone();
        Self { system }
    }
}

impl super::Window for Window {
    fn id(&self) -> egui::Id {
        egui::Id::new("system_editor")
    }

    fn name(&self) -> String {
        "System".to_string()
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .id(self.id())
            .show(ctx, |ui| {
                let rpg::System {
                    magic_number,
                    party_members,
                    elements,
                    switches,
                    variables,
                    windowskin_name,
                    title_name,
                    gameover_name,
                    battle_transition,
                    title_bgm,
                    battle_bgm,
                    battle_end_me,
                    gameover_me,
                    cursor_se,
                    decision_se,
                    cancel_se,
                    buzzer_se,
                    equip_se,
                    shop_se,
                    save_se,
                    load_se,
                    battle_start_se,
                    escape_se,
                    actor_collapse_se,
                    enemy_collapse_se,
                    words,
                    test_battlers,
                    test_troop_id,
                    start_map_id,
                    start_x,
                    start_y,
                    battleback_name,
                    battler_name,
                    battler_hue,
                    edit_map_id,
                } = &mut self.system;

                egui::SidePanel::left("system_editor_left").show_inside(ui, |ui| {
                    ui.label("Initial Party");
                    ui.group(|ui| {
                        egui::ScrollArea::vertical()
                            .id_source("system_editor_party_members")
                            .show(ui, |ui| {
                                let actors = state!().data_cache.actors();
                                let available_ids = actors
                                    .iter()
                                    .filter(|a| !party_members.contains(&a.id))
                                    .map(|a| a.id)
                                    .collect_vec();
                                let mut del_index = None;
                                ui.columns(2, |columns| {
                                    for (index, id) in party_members.iter_mut().enumerate() {
                                        let actor_name = &actors[*id as usize - 1].name;
                                        egui::ComboBox::from_id_source(
                                            egui::Id::new("system_editor_party_combobox").with(*id),
                                        )
                                        .selected_text(format!("{id:0>3} : {actor_name}"))
                                        .show_ui(
                                            &mut columns[0],
                                            |ui| {
                                                for available_id in available_ids.iter().copied() {
                                                    let actor_name =
                                                        &actors[available_id as usize - 1].name;
                                                    ui.selectable_value(
                                                        id,
                                                        available_id,
                                                        format!(
                                                            "{available_id:0>3} : {actor_name}"
                                                        ),
                                                    );
                                                }
                                            },
                                        );
                                        if columns[1].button("-").clicked() {
                                            del_index = Some(index);
                                        }
                                    }
                                });
                                if let Some(index) = del_index {
                                    party_members.remove(index);
                                }
                                if let Some(&id) = available_ids.first() {
                                    if ui.button("+").clicked() {
                                        party_members.push(id);
                                    }
                                }
                            })
                    });
                    ui.label("Element names");
                    ui.group(|ui| {
                        egui::ScrollArea::vertical()
                            .id_source("system_editor_elements")
                            .show(ui, |ui| {
                                let mut del_index = None;
                                for (index, name) in elements.iter_mut().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{index:0>3} :"));
                                        ui.text_edit_singleline(name);
                                        if ui.button("-").clicked() {
                                            del_index = Some(index);
                                        }
                                    });
                                }
                                if let Some(index) = del_index {
                                    elements.remove(index);
                                }
                                if ui.button("+").clicked() {
                                    elements.push(String::new());
                                }
                            })
                    });
                });

                egui::SidePanel::right("system_editor_right").show_inside(ui, |ui| {
                    ui.label("Words");
                    ui.group(|ui| {
                        ui.columns(2, |columns| {
                            let rpg::Words {
                                gold,
                                hp,
                                sp,
                                str,
                                dex,
                                agi,
                                int,
                                atk,
                                pdef,
                                mdef,
                                weapon,
                                armor1,
                                armor2,
                                armor3,
                                armor4,
                                attack,
                                skill,
                                guard,
                                item,
                                equip,
                            } = words;

                            columns[0].label("Currency");
                            columns[0].text_edit_singleline(gold);

                            columns[1].label("Weapon");
                            columns[1].text_edit_singleline(weapon);

                            columns[0].label("HP");
                            columns[0].text_edit_singleline(hp);

                            columns[1].label("Shield");
                            columns[1].text_edit_singleline(armor1);

                            columns[0].label("SP");
                            columns[0].text_edit_singleline(sp);

                            columns[1].label("Helmet");
                            columns[1].text_edit_singleline(armor2);

                            columns[0].label("STR");
                            columns[0].text_edit_singleline(str);

                            columns[1].label("Body Armour");
                            columns[1].text_edit_singleline(armor3);

                            columns[0].label("DEX");
                            columns[0].text_edit_singleline(dex);

                            columns[1].label("Acessory");
                            columns[1].text_edit_singleline(armor4);

                            columns[0].label("AGI");
                            columns[0].text_edit_singleline(agi);

                            columns[1].label("Attack");
                            columns[1].text_edit_singleline(attack);

                            columns[0].label("INT");
                            columns[0].text_edit_singleline(int);

                            columns[1].label("Skill");
                            columns[1].text_edit_singleline(skill);

                            columns[0].label("ATK");
                            columns[0].text_edit_singleline(atk);

                            columns[1].label("Defend");
                            columns[1].text_edit_singleline(guard);

                            columns[0].label("PDEF");
                            columns[0].text_edit_singleline(pdef);

                            columns[1].label("Item");
                            columns[1].text_edit_singleline(item);

                            columns[0].label("MDEF");
                            columns[0].text_edit_singleline(mdef);

                            columns[1].label("Equip");
                            columns[1].text_edit_singleline(equip);
                        });
                    })
                });

                egui::CentralPanel::default().show_inside(ui, |ui| {});
            });
    }
}
