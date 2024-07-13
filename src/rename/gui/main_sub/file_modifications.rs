use std::sync::Arc;
use std::borrow::BorrowMut;
use std::time::Instant;

use super::super::super::util::threads;
use super::super::super::app::{WindowMain, DndDropLocation};
use super::super::super::debug::DebugStatType;
use super::super::super::mods::{Modifiers, ModsOrder, ModAdd, ModCase, ModExtension, ModDate, 
    ModHashing, ModMoveCopy, ModName, ModNumber, ModRegex, ModRemove, ModReplace, CaseMode, CaseExecptMode,
    DateFormatMode, DateMode, DateSeperator, ExtensionMode, HashSeperator, MoveCopyFromMode, MoveCopyToMode, NameMode, NumberMode, 
    NumberTypeMode, RemoveCropMode};

pub fn modifications(gui: &mut WindowMain, ui: &mut egui::Ui, _ctx: &egui::Context) {
    let start = Instant::now();
    gui.modifications_total = 0;
    ui.add_enabled_ui(gui.section_modifiers_enabled, |ui| {
        ui.vertical(|ui| {
            // Header
            ui.horizontal(|ui| {
                ui.strong("Modifiers").on_hover_text("Order -> Priority");
                
                ui.add_space(30.0);
                

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    let reset = ui.add_enabled(true, egui::Button::new("⟲".to_string()));
                    if reset.clicked() {
                        gui.modifiers = Modifiers::default();
                    };
                    reset.on_hover_text_at_pointer("Reset back to defaults.");
                    ui.add_enabled_ui(gui.modifiers_dnd_enabled, |ui| {
                        ui.toggle_value(gui.modifiers_reorder_enabled.borrow_mut(), "Rearrange");
                    });
                });
            });
    
            ui.separator();

            // Body
            ui.allocate_ui(egui::vec2(ui.available_width(), ui.available_height() - 80.0), |ui| {
                egui::ScrollArea::vertical()
                .id_source("Modifiers")
                .enable_scrolling(gui.modifiers.scroll_allowed)
                .show(ui, |ui| {
                    let mut from: Option<Arc<DndDropLocation>> = None;
                    let mut to: Option<DndDropLocation> = None;
                    for mod_index in 0..gui.options.modifier_order.len() {
                        let modifier = gui.options.modifier_order[mod_index];
                        if gui.modifiers_reorder_enabled && modifier != ModsOrder::Hash {
                            let loc = DndDropLocation {
                                row: mod_index
                            };
                            let response = ui.dnd_drag_source(egui::Id::new(("Modifer-", mod_index)), loc, |ui| {
                                fill_modification_container(gui, modifier, ui, true);
                            }).response;
                            if let (Some(_pointer), Some(_hovered_payload)) = (
                                ui.input(|i| i.pointer.interact_pos()),
                                response.dnd_hover_payload::<DndDropLocation>()) {
                                    let rect = response.rect;
                
                                    // Preview insertion:
                                    let insert_row_idx = {
                                        // We are dragged onto ourselves
                                        ui.painter()
                                            .rect_filled(
                                                rect, 
                                                egui::Rounding::default().at_most(4.0), 
                                                egui::Color32::from_black_alpha(120));
                                        mod_index
                                    };
                
                                    if let Some(dragged_payload) = response.dnd_release_payload() {
                                        // The user dropped onto this item.
                                        from = Some(dragged_payload);
                                        to = Some(DndDropLocation {
                                            row: insert_row_idx,
                                        });
                                    };
                            };
                        } else {
                            fill_modification_container(gui, modifier, ui, false);
                        };
                    };
                    if let (Some(from), Some(to)) = (from, to) {
                        let mut order = gui.options.modifier_order.clone();
                        let modifier = order.remove(from.row);
                        order.insert(to.row, modifier);
                        gui.options.modifier_order = order;
                    };
                });
    
            });

            ui.separator();

            // Save Box
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Save");
                });
                ui.add_enabled_ui(gui.save_available && !gui.modifiers_reorder_enabled, |ui| {
                    // Body
                    ui.vertical(|ui| {
                        ui.separator();
                        let button = ui.add_sized(
                            egui::vec2(ui.available_width(), 24.0), 
                            egui::widgets::Button::new("Save")
                        );
                        if button.clicked() {
                            gui.popups.save_confirmation = !gui.popups.save_confirmation
                        };
                    });
                });
            });
        });
    });
    // Update Stats
    gui.statistics.push(start.elapsed().as_micros() as u32, DebugStatType::GuiModifier);
}

fn fill_modification_container(gui: &mut WindowMain, modorder: ModsOrder, ui: &mut egui::Ui, headers_only: bool) {
    if gui.modifiers.allow_frame == false { return };
    match modorder {
        ModsOrder::Add => {
            let mut modadd = gui.modifiers.add.clone();
            let modreadd_enabled = gui.modifiers.add_enabled;
            let mut modreadd_sections: u8 = modadd.len().to_owned() as u8;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Add");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.add_enabled, "");
    
                            ui.separator();
    
                            if ui.small_button("➕").clicked() {
                                if modreadd_sections < gui.options.file_modifiers.sub_modifier_maximum {
                                    gui.modifiers.add.push(ModAdd::default());
                                    modadd.push(ModAdd::default());
                                    modreadd_sections += 1;
                                }
                            };
    
                            ui.separator();
    
                            if ui.small_button("➖").clicked() {
                                if modreadd_sections > 1 {
                                    gui.modifiers.add.pop().unwrap();
                                    modadd.pop().unwrap();
                                    modreadd_sections -= 1;
                                }
                            };
                        });
                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    for (mod_index, _) in modadd.iter().enumerate() {
                        ui.add_enabled_ui(modreadd_enabled, |ui| {
                            let mut add_raw = gui.modifiers.add.remove(mod_index);
                            let add = fill_modadd(gui, ui, &mut add_raw);
                            gui.modifiers.add.insert(mod_index, add.0);
                            gui.modifications_total += add.1;
                            if (modadd.len() != 1) && (mod_index != modadd.len() - 1) { ui.separator(); }; // Add Seperators in between the sections.
                        });
                    }
                });
            });
        },
        ModsOrder::Case =>{
            let mut modcase = gui.modifiers.case.clone();
            let modcase_enabled = gui.modifiers.case_enabled;
            let mut modcase_sections: u8 = modcase.len().to_owned() as u8;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Case");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.case_enabled, "");
    
                            ui.separator();
    
                            if ui.small_button("➕").clicked() {
                                if modcase_sections < gui.options.file_modifiers.sub_modifier_maximum {
                                    gui.modifiers.case.push(ModCase::default());
                                    modcase.push(ModCase::default());
                                    modcase_sections += 1;
                                }
                            };
    
                            ui.separator();
    
                            if ui.small_button("➖").clicked() {
                                if modcase_sections > 1 {
                                    gui.modifiers.case.pop().unwrap();
                                    modcase.pop().unwrap();
                                    modcase_sections -= 1;
                                }
                            };
                        });

                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    for (mod_index, _) in modcase.iter().enumerate() {
                        ui.add_enabled_ui(modcase_enabled, |ui| {
                            let mut replace_raw = gui.modifiers.case.remove(mod_index);
                            let replace = fill_modcase(gui, ui, &mut replace_raw, mod_index);
                            gui.modifiers.case.insert(mod_index, replace.0);
                            gui.modifications_total += replace.1;
                            if (modcase.len() != 1) && (mod_index != modcase.len() - 1) { ui.separator(); }; // Add Seperators in between the sections.
                        });
                    }
                });
            });
        },
        ModsOrder::Date => {
            let mut moddate = gui.modifiers.date.clone();
            let moddate_enabled = gui.modifiers.date_enabled;
            let mut moddate_sections: u8 = moddate.len().to_owned() as u8;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Date");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.date_enabled, "");
    
                            ui.separator();
    
                            if ui.small_button("➕").clicked() {
                                if moddate_sections < gui.options.file_modifiers.sub_modifier_maximum {
                                    gui.modifiers.date.push(ModDate::default());
                                    moddate.push(ModDate::default());
                                    moddate_sections += 1;
                                }
                            };
    
                            ui.separator();
    
                            if ui.small_button("➖").clicked() {
                                if moddate_sections > 1 {
                                    gui.modifiers.date.pop().unwrap();
                                    moddate.pop().unwrap();
                                    moddate_sections -= 1;
                                }
                            };
                        });
                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    for (mod_index, _) in moddate.iter().enumerate() {
                        ui.add_enabled_ui(moddate_enabled, |ui| {
                            let mut date_raw = gui.modifiers.date.remove(mod_index);
                            let date = fill_moddate(gui, ui, &mut date_raw, mod_index);
                            gui.modifiers.date.insert(mod_index, date.0);
                            gui.modifications_total += date.1;
                            if (moddate.len() != 1) && (mod_index != moddate.len() - 1) { ui.separator(); }; // Add Seperators in between the sections.
                        });
                    }
                });
            });
        },
        ModsOrder::Ext => {
            let mut modext = gui.modifiers.extension.clone();
            let modext_enabled = gui.modifiers.extension_enabled;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Extension");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.extension_enabled, "");
                        });
                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    ui.add_enabled_ui(modext_enabled, |ui| {
                        gui.modifications_total += fill_modextension(gui, ui, &mut modext);
                    });
                });
            });
            // Refill Modifiers
            gui.modifiers.extension = modext;
        },
        ModsOrder::Hash => {
            let mut modhash = gui.modifiers.hash.clone();
            let modhash_enabled = gui.modifiers.hash_enable;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Hash");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.hash_enable, "");
                        });
                    });
                });
                if gui.modifiers_reorder_enabled { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    ui.add_enabled_ui(modhash_enabled, |ui| {
                        gui.modifications_total += fill_modhash(gui, ui, &mut modhash);
                    });
                });
            });
            // Refill Modifiers
            gui.modifiers.hash = modhash;
        },
        ModsOrder::MoveCopy => {
            let mut modmovecopy = gui.modifiers.movecopy.clone();
            let modmovecopy_enabled = gui.modifiers.movecopy_enabled;
            let mut modmovecopy_sections: u8 = modmovecopy.len().to_owned() as u8;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Move / Copy");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.movecopy_enabled, "");
    
                            ui.separator();
                            if ui.small_button("➕").clicked() {
                                if modmovecopy_sections < gui.options.file_modifiers.sub_modifier_maximum {
                                    gui.modifiers.movecopy.push(ModMoveCopy::default());
                                    modmovecopy.push(ModMoveCopy::default());
                                    modmovecopy_sections += 1;
                                }
                            };
    
                            ui.separator();
    
                            if ui.small_button("➖").clicked() {
                                if modmovecopy_sections > 1 {
                                    gui.modifiers.movecopy.pop().unwrap();
                                    modmovecopy.pop().unwrap();
                                    modmovecopy_sections -= 1;
                                }
                            };
                        });
                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    for (mod_index, _) in modmovecopy.iter().enumerate() {
                        ui.add_enabled_ui(modmovecopy_enabled, |ui| {
                            let mut movecopy_raw = gui.modifiers.movecopy.remove(mod_index);
                            let movecopy = fill_modmovecopy(gui, ui, &mut movecopy_raw, mod_index);
                            gui.modifiers.movecopy.insert(mod_index, movecopy.0);
                            gui.modifications_total += movecopy.1;
                            if (modmovecopy.len() != 1) && (mod_index != modmovecopy.len() - 1) { ui.separator(); }; // Add Seperators in between the sections.
                        });
                    }
                });
            });
        },
        ModsOrder::Name => {
            let mut modname = gui.modifiers.name.clone();
            let modname_enabled = gui.modifiers.name_enabled;
            let mut modname_sections: u8 = modname.len().to_owned() as u8;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.name_enabled, "");
    
                            ui.separator();
    
                            if ui.small_button("➕").clicked() {
                                if modname_sections < gui.options.file_modifiers.sub_modifier_maximum {
                                    gui.modifiers.name.push(ModName::default());
                                    modname.push(ModName::default());
                                    modname_sections += 1;
                                }
                            };
    
                            ui.separator();
    
                            if ui.small_button("➖").clicked() {
                                if modname_sections > 1 {
                                    gui.modifiers.name.pop().unwrap();
                                    modname.pop().unwrap();
                                    modname_sections -= 1;
                                }
                            };
                        });
                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    for (mod_index, _) in modname.iter().enumerate() {
                        ui.add_enabled_ui(modname_enabled, |ui| {
                            let mut name_raw = gui.modifiers.name.remove(mod_index);
                            let name = fill_modname(gui, ui, &mut name_raw, mod_index);
                            gui.modifiers.name.insert(mod_index, name.0);
                            gui.modifications_total += name.1;
                            if (modname.len() != 1) && (mod_index != modname.len() - 1) { ui.separator(); }; // Add Seperators in between the sections.
                        });
                    }
                });
            });
        },
        ModsOrder::Number => {
            let mut modnumber = gui.modifiers.number.clone();
            let modnumber_enabled = gui.modifiers.number_enabled;
            let mut modnumber_sections: u8 = modnumber.len().to_owned() as u8;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Number");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.number_enabled, "");
    
                            ui.separator();
    
                            if ui.small_button("➕").clicked() {
                                if modnumber_sections < gui.options.file_modifiers.sub_modifier_maximum {
                                    gui.modifiers.number.push(ModNumber::default());
                                    modnumber.push(ModNumber::default());
                                    modnumber_sections += 1;
                                }
                            };
    
                            ui.separator();
    
                            if ui.small_button("➖").clicked() {
                                if modnumber_sections > 1 {
                                    gui.modifiers.number.pop().unwrap();
                                    modnumber.pop().unwrap();
                                    modnumber_sections -= 1;
                                }
                            };
                        });
                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    for (mod_index, _) in modnumber.iter().enumerate() {
                        ui.add_enabled_ui(modnumber_enabled, |ui| {
                            let mut number_raw = gui.modifiers.number.remove(mod_index);
                            let number = fill_modnumber(gui, ui, &mut number_raw, mod_index);
                            gui.modifiers.number.insert(mod_index, number.0);
                            gui.modifications_total += number.1;
                            if (modnumber.len() != 1) && (mod_index != modnumber.len() - 1) { ui.separator(); }; // Add Seperators in between the sections.
                        });
                    }
                });
            });
        },
        ModsOrder::Regex => {
            let mut modregex = gui.modifiers.regex.clone();
            let modregex_enabled = gui.modifiers.regex_enabled;
            let mut modregex_sections: u8 = modregex.len().to_owned() as u8;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Regex");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.regex_enabled, "");
    
                            ui.separator();
    
                            if ui.small_button("➕").clicked() {
                                if modregex_sections < gui.options.file_modifiers.sub_modifier_maximum {
                                    gui.modifiers.regex.push(ModRegex::default());
                                    modregex.push(ModRegex::default());
                                    modregex_sections += 1;
                                }
                            };
    
                            ui.separator();
    
                            if ui.small_button("➖").clicked() {
                                if modregex_sections > 1 {
                                    gui.modifiers.regex.pop().unwrap();
                                    modregex.pop().unwrap();
                                    modregex_sections -= 1;
                                }
                            };
                        });
                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    ui.add_enabled_ui(modregex_enabled, |ui| {
                        for (mod_index, _) in modregex.iter().enumerate() {
                            let mut regex_raw = gui.modifiers.regex.remove(mod_index);
                            let regex = fill_modregex(gui, ui, &mut regex_raw, mod_index);
                            gui.modifiers.regex.insert(mod_index, regex.0);
                            gui.modifications_total += regex.1;
                            if (modregex.len() != 1) && (mod_index != modregex.len() - 1) { ui.separator(); }; // Add Seperators in between the sections.
                        };
                    });
                });
            });
        },
        ModsOrder::Remove => {
            let mut modremove = gui.modifiers.remove.clone();
            let modremove_enabled = gui.modifiers.remove_enabled;
            let mut modremove_sections: u8 = modremove.len().to_owned() as u8;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Remove");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.remove_enabled, "");
    
                            ui.separator();
    
                            if ui.small_button("➕").clicked() {
                                if modremove_sections < gui.options.file_modifiers.sub_modifier_maximum {
                                    gui.modifiers.remove.push(ModRemove::default());
                                    modremove.push(ModRemove::default());
                                    modremove_sections += 1;
                                }
                            };
    
                            ui.separator();
    
                            if ui.small_button("➖").clicked() {
                                if modremove_sections > 1 {
                                    gui.modifiers.remove.pop().unwrap();
                                    modremove.pop().unwrap();
                                    modremove_sections -= 1;
                                }
                            };
                        });
                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    for (mod_index, _) in modremove.iter().enumerate() {
                        ui.add_enabled_ui(modremove_enabled, |ui| {
                            let mut remove_raw = gui.modifiers.remove.remove(mod_index);
                            let remove = fill_modremove(gui, ui, &mut remove_raw, mod_index);
                            gui.modifiers.remove.insert(mod_index, remove.0);
                            gui.modifications_total += remove.1;
                            if (modremove.len() != 1) && (mod_index != modremove.len() - 1) { ui.separator(); }; // Add Seperators in between the sections.
                        });
                    }
                });
            });
        },
        ModsOrder::Replace => {
            let mut modreplace = gui.modifiers.replace.clone();
            let modreplace_enabled = gui.modifiers.replace_enabled;
            let mut modreplace_sections: u8 = modreplace.len().to_owned() as u8;
            ui.group(|ui| {
                // Title Bar
                ui.horizontal(|ui| {
                    ui.label("Replace");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!headers_only, |ui| {
                            ui.checkbox(&mut gui.modifiers.replace_enabled, "");
    
                            ui.separator();
    
                            if ui.small_button("➕").clicked() {
                                if modreplace_sections < gui.options.file_modifiers.sub_modifier_maximum {
                                    gui.modifiers.replace.push(ModReplace::default());
                                    modreplace.push(ModReplace::default());
                                    modreplace_sections += 1;
                                }
                            };
    
                            ui.separator();
    
                            if ui.small_button("➖").clicked() {
                                if modreplace_sections > 1 {
                                    gui.modifiers.replace.pop().unwrap();
                                    modreplace.pop().unwrap();
                                    modreplace_sections -= 1;
                                }
                            };
                        });
                    });
                });
                if headers_only { return }; // Rearranging causes headers to collapse.
                // Body
                ui.vertical(|ui| {
                    ui.separator();
                    for (mod_index, _) in modreplace.iter().enumerate() {
                        ui.add_enabled_ui(modreplace_enabled, |ui| {
                            let mut replace_raw = gui.modifiers.replace.remove(mod_index);
                            let replace = fill_modreplace(gui, ui, &mut replace_raw, mod_index);
                            gui.modifiers.replace.insert(mod_index, replace.0);
                            gui.modifications_total += replace.1;
                            if (modreplace.len() != 1) && (mod_index != modreplace.len() - 1) { ui.separator(); }; // Add Seperators in between the sections.
                        });
                    }
                });
            });
        }
    };
}

fn fill_modadd(gui: &mut WindowMain, ui: &mut egui::Ui, add: &mut ModAdd) -> (ModAdd, u32) {
    let mut modifications: u32 = 0;
    ui.horizontal(|ui| {
        ui.label("Prefix");
        ui.add_sized(
            egui::vec2(ui.available_width(), ui.available_height()), 
            egui::widgets::text_edit::TextEdit::singleline(&mut add.prefix)
        );
    });
    ui.horizontal(|ui| {
        ui.label("Insert");
        ui.add_sized(
            egui::vec2(ui.available_width() - 145.0, ui.available_height()), 
            egui::widgets::text_edit::TextEdit::singleline(&mut add.insert)
        );
        ui.label("at");
        let drag = ui.add_enabled(true, 
            egui::DragValue::new(&mut add.insert_at)
            .range(-255..=255)
            .speed(0.05)
        );

        if drag.hovered() {
            gui.modifiers.drag_box_hovered = true;
        };

        if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
            add.insert_at += 1;
        } else if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
            if add.insert_at >= 1 {
                add.insert_at -= 1;
            }
        };
        if ui.small_button("➖").clicked() {
            add.insert_at -= 1;
        };

        ui.separator();

        if ui.small_button("➕").clicked() {
            add.insert_at += 1;
        };
    });
    ui.horizontal(|ui| {
        ui.label("Suffix");
        ui.add_sized(
            egui::vec2(ui.available_width(), ui.available_height()), 
            egui::widgets::text_edit::TextEdit::singleline(&mut add.suffix)
        );
    });
    // Fill modifications
    {
        if add.insert_at != 0 { modifications += 1 };
        if add.prefix.chars().count() >= 1 { modifications += 1 };
        if add.suffix.chars().count() >= 1 { modifications += 1 };
        if add.seperator_enabled == true { modifications += 1 };
    }
    return (add.to_owned(), modifications);
}

fn fill_modcase(gui: &mut WindowMain, ui: &mut egui::Ui, case: &mut ModCase, index: usize) -> (ModCase, u32) {
    let mut modifications: u32 = 0;
    ui.vertical(|ui| { 
        ui.horizontal(|ui| {
            ui.label("Mode");
            egui::ComboBox::new(format!("case-{}", index), "")
                .selected_text(case.mode_name.to_owned())
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, "Same").clicked() {
                        case.mode_name = String::from("Same");
                        case.mode = CaseMode::Same;
                    }
                    if ui.selectable_label(false, "Upper").clicked() {
                        case.mode_name = String::from("Upper");
                        case.mode = CaseMode::Upper;
                    }
                    if ui.selectable_label(false, "Lower").clicked() {
                        case.mode_name = String::from("Lower");
                        case.mode = CaseMode::Lower;
                    }
                    if ui.selectable_label(false, "Title").clicked() {
                        case.mode_name = String::from("Title");
                        case.mode = CaseMode::Title;
                    }
                    if ui.selectable_label(false, "UpperFirst").clicked() {
                        case.mode_name = String::from("UpperFirst");
                        case.mode = CaseMode::UpperFirst;
                    }
                });

                ui.separator();
                ui.add_enabled_ui(case.except_enabled, |ui| {
                    ui.label("Except");
                    egui::ComboBox::new(format!("case-except-{}", index), "")
                        .selected_text(case.except_mode_name.to_owned())
                        .show_ui(ui, |ui| {
                            if ui.selectable_label(false, "None").clicked() {
                                case.except_mode_name = String::from("None");
                                case.except_mode = CaseExecptMode::None;
                            }
                            if ui.selectable_label(false, "FromTo").clicked() {
                                case.except_mode_name = String::from("FromTo");
                                case.except_mode = CaseExecptMode::FromTo;
                            }
                            if ui.selectable_label(false, "Match").clicked() {
                                case.except_mode_name = String::from("Match");
                                case.except_mode = CaseExecptMode::Match;
                            }
                    });
                });
            });
        ui.horizontal(|ui| {
            ui.add_enabled_ui(case.widgets_enabled, |ui| {
                match case.except_mode {
                    CaseExecptMode::FromTo => {
                        ui.label("From");
                        let drag = ui.add_enabled(true, 
                            egui::DragValue::new(&mut case.except_from)
                            .range(0..=255)
                            .speed(0.05)
                            
                        );

                        if drag.hovered() {
                            gui.modifiers.drag_box_hovered = true;
                        };
                
                        if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                            case.except_from += 1;
                        } else if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                            if case.except_from >= 1 {
                                case.except_from -= 1;
                            };
                        };
                        if ui.small_button("➖").clicked() {
                            if case.except_from >= 1 {
                                case.except_from -= 1;
                            };
                        };
                
                        ui.separator();
                
                        if ui.small_button("➕").clicked() {
                            case.except_from += 1;
                        };

                        ui.label("To");
                        let drag = ui.add_enabled(true, 
                            egui::DragValue::new(&mut case.except_to)
                            .range(case.except_from..=255)
                            .speed(0.05)
                        );

                        if drag.hovered() {
                            gui.modifiers.drag_box_hovered = true;
                        };
                
                        if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                            case.except_to += 1;
                        } else if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                            if case.except_to >= 1 {
                                case.except_to -= 1;
                            };
                        };
                        if ui.small_button("➖").clicked() {
                            if case.except_to >= 1 {
                                case.except_to -= 1;
                            };
                        };
                
                        ui.separator();
                
                        if ui.small_button("➕").clicked() {
                            case.except_to += 1;
                        };
                    },
                    CaseExecptMode::Match => {
                        ui.label("Match");
                        ui.add_sized(
                            egui::vec2(ui.available_width(), ui.available_height()), 
                            egui::text_edit::TextEdit::singleline(&mut case.except)
                        );
                    },
                    _ => {}
                }
            });
        });
    });
    match case.except_mode {
        CaseExecptMode::None => {
            case.widgets_enabled = false;
        },
        _ => { case.widgets_enabled = true }
    }
    match case.mode {
        CaseMode::Title | CaseMode::UpperFirst | CaseMode::Same => {
            case.except_enabled = false;
        },
        _ => {
            case.except_enabled = true;
        }
    };
    // Fill modifications
    {
        if case.except_mode != CaseExecptMode::None { modifications += 1 };
        if case.except.chars().count() >= 1 { modifications += 1 };
        if case.except_from >= 1 { modifications += 1 };
        if case.except_to >= 1 { modifications += 1 };
        if case.mode != CaseMode::Same { modifications += 1 };
    }
    return (case.to_owned(), modifications);
}

fn fill_moddate(gui: &mut WindowMain, ui: &mut egui::Ui, date: &mut ModDate, index: usize) -> (ModDate, u32) {
    let mut modifications: u32 = 0;
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Mode");
            egui::ComboBox::new(format!("date-{}", index), "")
            .selected_text(date.mode_name.to_owned())
            .show_ui(ui, |ui| {
                if ui.selectable_label(false, "None").clicked() {
                    date.mode_name = String::from("None");
                    date.mode = DateMode::None;
                }
                if ui.selectable_label(false, "Prefix").clicked() {
                    date.mode_name = String::from("Prefix");
                    date.mode = DateMode::Prefix;
                }
                if ui.selectable_label(false, "Suffix").clicked() {
                    date.mode_name = String::from("Suffix");
                    date.mode = DateMode::Suffix;
                }
                if ui.selectable_label(false, "Insert").clicked() {
                    date.mode_name = String::from("Insert");
                    date.mode = DateMode::Insert;
                }
            });
            ui.add_enabled_ui(date.at_enabled, |ui| {
                ui.label("at");
                let drag = ui.add_enabled(true, 
                    egui::DragValue::new(&mut date.at_pos)
                    .range(-255..=255)
                    .speed(0.05)
                );

                if drag.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    date.at_pos += 1;
                } else if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if date.at_pos >= 1 {
                        date.at_pos -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    date.at_pos -= 1;
                };
        
                ui.separator();
        
                if ui.small_button("➕").clicked() {
                    date.at_pos += 1;
                };
            });
        });
        ui.horizontal(|ui| {
            ui.label("Format");
            ui.add_enabled_ui(date.widgets_enabled, |ui| {
                egui::ComboBox::new(format!("date_format-{}", index), "")
                .selected_text(date.format_name.to_owned())
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, "Y").clicked() {
                        date.format_name = String::from("Y");
                        date.format = DateFormatMode::Y;
                    }
                    if ui.selectable_label(false, "MY").clicked() {
                        date.format_name = String::from("MY");
                        date.format = DateFormatMode::MY;
                    }
                    if ui.selectable_label(false, "DMY").clicked() {
                        date.format_name = String::from("DMY");
                        date.format = DateFormatMode::DMY;
                    }
                    if ui.selectable_label(false, "DMYH").clicked() {
                        date.format_name = String::from("DMYH");
                        date.format = DateFormatMode::DMYH;
                    }
                    if ui.selectable_label(false, "DMYHM").clicked() {
                        date.format_name = String::from("DMYHM");
                        date.format = DateFormatMode::DMYHM;
                    }
                    if ui.selectable_label(false, "DMYHMS").clicked() {
                        date.format_name = String::from("DMYHMS");
                        date.format = DateFormatMode::DMYHMS;
                    }
                    if ui.selectable_label(false, "YM").clicked() {
                        date.format_name = String::from("YM");
                        date.format = DateFormatMode::YM;
                    }
                    if ui.selectable_label(false, "YMD").clicked() {
                        date.format_name = String::from("YMD");
                        date.format = DateFormatMode::YMD;
                    }
                    if ui.selectable_label(false, "YMDH").clicked() {
                        date.format_name = String::from("YMDH");
                        date.format = DateFormatMode::YMDH;
                    }
                    if ui.selectable_label(false, "YMDHM").clicked() {
                        date.format_name = String::from("YMDHM");
                        date.format = DateFormatMode::YMDHM;
                    }
                    if ui.selectable_label(false, "YMDHMS").clicked() {
                        date.format_name = String::from("YMDHMS");
                        date.format = DateFormatMode::YMDHMS;
                    }
                    if ui.selectable_label(false, "MDY").clicked() {
                        date.format_name = String::from("MDY");
                        date.format = DateFormatMode::MDY;
                    }
                    if ui.selectable_label(false, "MDYH").clicked() {
                        date.format_name = String::from("MDYH");
                        date.format = DateFormatMode::MDYH;
                    }
                    if ui.selectable_label(false, "MDYHM").clicked() {
                        date.format_name = String::from("MDYHM");
                        date.format = DateFormatMode::MDYHM;
                    }
                    if ui.selectable_label(false, "MDYHMS").clicked() {
                        date.format_name = String::from("MDYHMS");
                        date.format = DateFormatMode::MDYHMS;
                    }
                    if ui.selectable_label(false, "Custom").clicked() {
                        date.format_name = String::from("Custom");
                        date.format = DateFormatMode::Custom;
                    }
                });
            });
            ui.add_enabled_ui(date.widgets_enabled, |ui| {
                ui.label("Seperator");
                egui::ComboBox::new(format!("date_seperator-{}", index), "")
                .selected_text(date.seperator_name.to_owned())
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, "None").clicked() {
                        date.seperator = DateSeperator::None;
                        date.seperator_name = String::from("None");
                    }
                    if ui.selectable_label(false, "Space  ").clicked() {
                        date.seperator = DateSeperator::Space;
                        date.seperator_name = String::from("Space  ");
                    }
                    if ui.selectable_label(false, "Asterisk **").clicked() {
                        date.seperator = DateSeperator::Asterisk;
                        date.seperator_name = String::from("Asterisk **");
                    }
                    if ui.selectable_label(false, "Bracket []").clicked() {
                        date.seperator = DateSeperator::Bracket;
                        date.seperator_name = String::from("Bracket []");
                    }
                    ui.add_enabled_ui(!cfg!(windows), |ui| {
                        if ui.selectable_label(false, "Colan ::").clicked() {
                            date.seperator = DateSeperator::Colan;
                            date.seperator_name = String::from("Colan :");
                        }
                    });
                    if ui.selectable_label(false, "CurlyBracket {}").clicked() {
                        date.seperator = DateSeperator::CurlyBracket;
                        date.seperator_name = String::from("CurlyBracket {}");
                    }
                    ui.add_enabled_ui(!cfg!(windows), |ui| {
                        if ui.selectable_label(false, "Line ||").clicked() {
                            date.seperator = DateSeperator::Line;
                            date.seperator_name = String::from("Line ||");
                        }
                    });
                    if ui.selectable_label(false, "Minus --").clicked() {
                        date.seperator = DateSeperator::Minus;
                        date.seperator_name = String::from("Minus --");
                    }
                    if ui.selectable_label(false, "Parenthesis ()").clicked() {
                        date.seperator = DateSeperator::Parenthesis;
                        date.seperator_name = String::from("Parenthesis ()");
                    }
                    if ui.selectable_label(false, "Plus ++").clicked() {
                        date.seperator = DateSeperator::Plus;
                        date.seperator_name = String::from("Plus ++");
                    }
                    if ui.selectable_label(false, "Sign <>").clicked() {
                        date.seperator = DateSeperator::Sign;
                        date.seperator_name = String::from("Sign <>");
                    }
                    if ui.selectable_label(false, "Underscore __").clicked() {
                        date.seperator = DateSeperator::Underscore;
                        date.seperator_name = String::from("Underscore __");
                    }
                });
            });
        });
        ui.horizontal(|ui| {
            ui.add_enabled_ui(date.widgets_enabled, |ui| {
                ui.label("Seg. Date");
                egui::ComboBox::new(format!("date_seg_year-{}", index), "")
                .selected_text(date.segregator_year_name.to_owned())
                .show_ui(ui, |ui| {
                    ui.add_enabled_ui(date.widgets_enabled, |ui| {
                        if ui.selectable_label(false, "None").clicked() {
                            date.segregator_year = DateSeperator::None;
                            date.segregator_year_name = String::from("None");
                        }
                        if ui.selectable_label(false, "Space ").clicked() {
                            date.segregator_year = DateSeperator::Space;
                            date.segregator_year_name = String::from("Space ");
                        }
                        ui.add_enabled_ui(!cfg!(windows), |ui| {
                            if ui.selectable_label(false, "Colan :").clicked() {
                                date.segregator_year = DateSeperator::Colan;
                                date.segregator_year_name = String::from("Colan :");
                            }
                        });
                        ui.add_enabled_ui(!cfg!(windows), |ui| {
                            if ui.selectable_label(false, "Line |").clicked() {
                                date.segregator_year = DateSeperator::Line;
                                date.segregator_year_name = String::from("Line |");
                            }
                        });
                        if ui.selectable_label(false, "Minus -").clicked() {
                            date.segregator_year = DateSeperator::Minus;
                            date.segregator_year_name = String::from("Minus -");
                        }
                        if ui.selectable_label(false, "Plus +").clicked() {
                            date.segregator_year = DateSeperator::Plus;
                            date.segregator_year_name = String::from("Plus +");
                        }
                        if ui.selectable_label(false, "Underscore _").clicked() {
                            date.segregator_hour = DateSeperator::Underscore;
                            date.segregator_year_name = String::from("Underscore _");
                        }
                    });
                });
                ui.label("Seg. Time");
                egui::ComboBox::new(format!("date_seg_hour-{}", index), "")
                .selected_text(date.segregator_hour_name.to_owned())
                .show_ui(ui, |ui| {
                    ui.add_enabled_ui(date.widgets_enabled, |ui| {
                        if ui.selectable_label(false, "None").clicked() {
                            date.segregator_hour = DateSeperator::None;
                            date.segregator_hour_name = String::from("None");
                        }
                        if ui.selectable_label(false, "Space ").clicked() {
                            date.segregator_hour = DateSeperator::Space;
                            date.segregator_hour_name = String::from("Space ");
                        }
                        ui.add_enabled_ui(!cfg!(windows), |ui| {
                            if ui.selectable_label(false, "Colan :").clicked() {
                                date.segregator_hour = DateSeperator::Colan;
                                date.segregator_hour_name = String::from("Colan :");
                            }
                        });
                        ui.add_enabled_ui(!cfg!(windows), |ui| {
                            if ui.selectable_label(false, "Line |").clicked() {
                                date.segregator_hour = DateSeperator::Line;
                                date.segregator_hour_name = String::from("Line |");
                            }
                        });
                        if ui.selectable_label(false, "Minus -").clicked() {
                            date.segregator_hour = DateSeperator::Minus;
                            date.segregator_hour_name = String::from("Minus -");
                        }
                        if ui.selectable_label(false, "Plus +").clicked() {
                            date.segregator_hour = DateSeperator::Plus;
                            date.segregator_hour_name = String::from("Plus +");
                        }
                        if ui.selectable_label(false, "Underscore _").clicked() {
                            date.segregator_hour = DateSeperator::Underscore;
                            date.segregator_hour_name = String::from("Underscore _");
                        }
                    });
                });
            });
        });
        ui.horizontal(|ui| {
            ui.add_enabled_ui(date.widgets_enabled, |ui| {
                ui.label("Century").on_hover_text("1999 -> 99");
                ui.checkbox(&mut date.century, "");
            });
            ui.add_enabled_ui(date.custom_enabled, |ui| {
                ui.label("Custom");
                ui.add_sized(
                    egui::vec2(ui.available_width(), ui.available_height()), 
                    egui::text_edit::TextEdit::singleline(&mut date.custom
                )).on_hover_text(format!(
                    "Create a custom format using the following"
                ))
            });
        });
    });
    match date.mode {
        DateMode::None => {
            date.widgets_enabled = false;
            date.at_enabled = false;
        },
        DateMode::Insert => {
            date.widgets_enabled = true;
            date.at_enabled = true;
        }
        _ => { 
            date.widgets_enabled = true;
            date.at_enabled = false;
        }
    };
    match date.format {
        DateFormatMode::Custom => {
            date.custom_enabled = true;
        },
        _ => {
            date.custom_enabled = false;
        }
    };
    // Fill modifications
    {
        if date.segregator_hour_enabled == true { modifications += 1 };
        if date.segregator_year_enabled == true { modifications += 1 };
        if date.at_pos != 0 { modifications += 1 };
        if date.custom.chars().count() >= 1 { modifications += 1 };
        if date.mode != DateMode::None { modifications += 1 };
    }
    return (date.to_owned(), modifications);
}

fn fill_modextension(_gui: &mut WindowMain, ui: &mut egui::Ui, ext: &mut ModExtension) -> u32 {
    let mut modifications: u32 = 0;
    ui.horizontal(|ui| {ui.label("Mode");
        egui::ComboBox::new(format!("extension"), "")
        .selected_text(ext.mode_name.to_owned())
        .show_ui(ui, |ui| {
            if ui.selectable_label(false, "Same").clicked() {
                ext.mode_name = String::from("Same");
                ext.mode = ExtensionMode::Same;
            }
            if ui.selectable_label(false, "Upper").clicked() {
                ext.mode_name = String::from("Upper");
                ext.mode = ExtensionMode::Upper;
            }
            if ui.selectable_label(false, "Lower").clicked() {
                ext.mode_name = String::from("Lower");
                ext.mode = ExtensionMode::Lower;
            }
            if ui.selectable_label(false, "UpperFirst").clicked() {
                ext.mode_name = String::from("UpperFirst");
                ext.mode = ExtensionMode::UpperFirst;
            }
            if ui.selectable_label(false, "Fixed").clicked() {
                ext.mode_name = String::from("Fixed");
                ext.mode = ExtensionMode::Fixed;
            }
            if ui.selectable_label(false, "Extra").clicked() {
                ext.mode_name = String::from("Extra");
                ext.mode = ExtensionMode::Extra;
            }
            if ui.selectable_label(false, "Remove").clicked() {
                ext.mode_name = String::from("Remove");
                ext.mode = ExtensionMode::Remove;
            }
        });
    });
    ui.horizontal(|ui| {
        ui.add_enabled_ui(ext.widgets_enabled, |ui| {
            ui.label("Fixed");
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()), 
                egui::text_edit::TextEdit::singleline(&mut ext.fixed)
            );
        });
    });
    ui.horizontal(|ui| {
        ui.add_enabled_ui(ext.widgets_enabled, |ui| {
            ui.label("Extra");
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()), 
                egui::text_edit::TextEdit::singleline(&mut ext.extra)
            );
        });
    });
    match ext.mode {
        ExtensionMode::Same => {
            ext.widgets_enabled = false;
        },
        _ => { ext.widgets_enabled = true }
    };
    // Fill modifications
    {
        if ext.extra.chars().count() >= 1 { modifications += 1 };
        if ext.fixed.chars().count() >= 1 { modifications += 1 };
        if ext.mode != ExtensionMode::Same { modifications += 1 };
    }
    return modifications;
}

fn fill_modhash(_gui: &mut WindowMain, ui: &mut egui::Ui, hash: &mut ModHashing) -> u32 {
    let mut modifications: u32 = 0;
    ui.horizontal(|ui| {
        ui.label("Mode");
        egui::ComboBox::new(format!("hashmode"), "")
        .selected_text(hash.mode_name.to_owned())
        .show_ui(ui, |ui| {
            if ui.selectable_label(false, "None").clicked() {
                hash.mode_name = String::from("None");
                hash.mode = threads::HashMode::None;
            }
            if ui.selectable_label(false, "Prefix").clicked() {
                hash.mode_name = String::from("Prefix");
                hash.mode = threads::HashMode::Prefix;
            }
            if ui.selectable_label(false, "Suffix").clicked() {
                hash.mode_name = String::from("Suffix");
                hash.mode = threads::HashMode::Suffix;
            }
            if ui.selectable_label(false, "File").clicked() {
                hash.mode_name = String::from("File");
                hash.mode = threads::HashMode::File;
            }
        });
        ui.label("Algorithm");
        ui.add_enabled_ui(hash.widgets_enabled, |ui| {
            egui::ComboBox::new(format!("hashalgorithm"), "")
            .selected_text(hash.algorithm_name.to_owned())
            .show_ui(ui, |ui| {
                if ui.selectable_label(false, "CRC32").clicked() {
                    hash.algorithm_name = String::from("CRC32");
                    hash.algorithm = threads::HashType::CRC32;
                }
                if ui.selectable_label(false, "MD5").clicked() {
                    hash.algorithm_name = String::from("MD5");
                    hash.algorithm = threads::HashType::MD5;
                }
                if ui.selectable_label(false, "Sha1").clicked() {
                    hash.algorithm_name = String::from("Sha1");
                    hash.algorithm = threads::HashType::Sha1;
                }
                if ui.selectable_label(false, "Sha256").clicked() {
                    hash.algorithm_name = String::from("Sha256");
                    hash.algorithm = threads::HashType::Sha256;
                }
            });
        });
    });
    ui.horizontal(|ui| {
        ui.label("Seperator");
        egui::ComboBox::new(format!("hashaseperator"), "")
        .selected_text(hash.seperator_name.to_owned())
        .show_ui(ui, |ui| {
            if ui.selectable_label(false, "None").clicked() {
                hash.seperator_name = String::from("None");
                hash.seperator = HashSeperator::None;
            }
            if ui.selectable_label(false, "Asterisk **").clicked() {
                hash.seperator_name = String::from("Asterisk **");
                hash.seperator = HashSeperator::Asterisk;
            }
            if ui.selectable_label(false, "Bracket []").clicked() {
                hash.seperator_name = String::from("Bracket []");
                hash.seperator = HashSeperator::Bracket;
            }
            if ui.selectable_label(false, "Colan ::").clicked() {
                hash.seperator_name = String::from("Colan ::");
                hash.seperator = HashSeperator::Colan;
            }
            if ui.selectable_label(false, "CurlyBracket {}").clicked() {
                hash.seperator_name = String::from("CurlyBracket {}");
                hash.seperator = HashSeperator::CurlyBracket;
            }
            if ui.selectable_label(false, "Line ||").clicked() {
                hash.seperator_name = String::from("Line ||");
                hash.seperator = HashSeperator::Line;
            }
            if ui.selectable_label(false, "Minus --").clicked() {
                hash.seperator_name = String::from("Minus --");
                hash.seperator = HashSeperator::Minus;
            }
            if ui.selectable_label(false, "Parenthesis ()").clicked() {
                hash.seperator_name = String::from("Parenthesis ()");
                hash.seperator = HashSeperator::Parenthesis;
            }
            if ui.selectable_label(false, "Plus ++").clicked() {
                hash.seperator_name = String::from("Plus ++");
                hash.seperator = HashSeperator::Plus;
            }
            if ui.selectable_label(false, "Sign <>").clicked() {
                hash.seperator_name = String::from("Sign <>");
                hash.seperator = HashSeperator::Sign;
            }
            if ui.selectable_label(false, "Space  ").clicked() {
                hash.seperator_name = String::from("Space  ");
                hash.seperator = HashSeperator::Space;
            }
            if ui.selectable_label(false, "Underscore __").clicked() {
                hash.seperator_name = String::from("Underscore __");
                hash.seperator = HashSeperator::Underscore;
            }
        });
    });
    match hash.mode {
        threads::HashMode::None => {
            hash.widgets_enabled = false;
        },
        _ => { hash.widgets_enabled = true }
    };
    // Fill modification
    {
        if hash.mode != threads::HashMode::None { modifications += 1 };
    }
    return modifications;
}

fn fill_modmovecopy(gui: &mut WindowMain, ui: &mut egui::Ui, movecopy: &mut ModMoveCopy, index: usize) -> (ModMoveCopy, u32) {
    let mut modifications: u32 = 0;
    let mut seperator: String = movecopy.seperator.to_string();
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Mode");
            egui::ComboBox::new(format!("movecopyfrom-{}", index), "")
            .selected_text(movecopy.mode_from_name.to_owned())
            .show_ui(ui, |ui| {
                if ui.selectable_label(false, "None").clicked() {
                    movecopy.mode_from_name = String::from("None");
                    movecopy.mode_from = MoveCopyFromMode::None;
                }
                if ui.selectable_label(false, "Copy First N").clicked() {
                    movecopy.mode_from_name = String::from("Copy First N");
                    movecopy.mode_from = MoveCopyFromMode::CopyFirstN;
                }
                if ui.selectable_label(false, "Copy Last N").clicked() {
                    movecopy.mode_from_name = String::from("Copy Last N");
                    movecopy.mode_from = MoveCopyFromMode::CopyLastN;
                }
                if ui.selectable_label(false, "Move First N").clicked() {
                    movecopy.mode_from_name = String::from("Move First N");
                    movecopy.mode_from = MoveCopyFromMode::MoveFirstN;
                }
                if ui.selectable_label(false, "Move Last N").clicked() {
                    movecopy.mode_from_name = String::from("Move Last N");
                    movecopy.mode_from = MoveCopyFromMode::MoveLastN;
                }
            });
            ui.add_enabled_ui(movecopy.widgets_enabled, |ui| {
                ui.separator();
                ui.label("N");
                let drag = ui.add_enabled(true, 
                    egui::DragValue::new(&mut movecopy.letters_count)
                    .range(0..=255)
                    .speed(0.05)
                );

                if drag.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    movecopy.letters_count += 1;
                } else if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if movecopy.letters_count >= 1 {
                        movecopy.letters_count -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    if movecopy.letters_count >= 1 {
                        movecopy.letters_count -= 1;
                    }
                };
        
                ui.separator();
        
                if ui.small_button("➕").clicked() {
                    movecopy.letters_count += 1;
                };
            })
        });
        ui.horizontal(|ui| {
            ui.add_enabled_ui(movecopy.widgets_enabled, |ui| {
                ui.label("to");
                egui::ComboBox::new(format!("movecopyto-{}", index), "")
                .selected_text(movecopy.mode_to_name.to_owned())
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, "None").clicked() {
                        movecopy.mode_to_name = String::from("None");
                        movecopy.mode_to = MoveCopyToMode::None;
                    }
                    if ui.selectable_label(false, "To Start").clicked() {
                        movecopy.mode_to_name = String::from("To Start");
                        movecopy.mode_to = MoveCopyToMode::ToStart;
                    }
                    if ui.selectable_label(false, "To End").clicked() {
                        movecopy.mode_to_name = String::from("To End");
                        movecopy.mode_to = MoveCopyToMode::ToEnd;
                    }
                    if ui.selectable_label(false, "To Pos").clicked() {
                        movecopy.mode_to_name = String::from("To Pos");
                        movecopy.mode_to = MoveCopyToMode::ToPos;
                    }
                });
            });
            ui.add_enabled_ui(movecopy.widgets_enabled_two, |ui| {
                ui.separator();
                ui.label("pos");
                let drag = ui.add_enabled(true, 
                    egui::DragValue::new(&mut movecopy.mode_to_pos)
                    .range(0..=255)
                    .speed(0.05)
                );

                if drag.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    movecopy.mode_to_pos += 1;
                } else if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if movecopy.mode_to_pos >= 1 {
                        movecopy.mode_to_pos -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    if movecopy.mode_to_pos >= 1 {
                        movecopy.mode_to_pos -= 1;
                    }
                };
        
                ui.separator();
        
                if ui.small_button("➕").clicked() {
                    movecopy.mode_to_pos += 1;
                };
            });
        });
        ui.horizontal(|ui| {
            ui.add_enabled_ui(movecopy.widgets_enabled_two, |ui| {
                ui.label("Seperator Char");
                ui.checkbox(&mut movecopy.seperator_enabled, "");
                ui.add_enabled_ui(movecopy.seperator_enabled, |ui| {
                    let response = ui.add_sized(
                        egui::vec2(20.0, ui.available_height()), 
                        egui::text_edit::TextEdit::singleline(&mut seperator)
                        .char_limit(2)
                        .cursor_at_end(false)
                    );
                    if response.changed() {
                        if seperator.len() >= 1 {
                            let c = seperator.pop().unwrap();
                            movecopy.seperator = c;
                        }
                    }
                });
            });
        });
    });
    match movecopy.mode_from {
        MoveCopyFromMode::None => {
            movecopy.widgets_enabled = false;
        },
        _ => { movecopy.widgets_enabled = true }
    };
    match movecopy.mode_to {
        MoveCopyToMode::None => {
            movecopy.widgets_enabled_two = false;
        },
        MoveCopyToMode::ToPos => {
            movecopy.widgets_enabled_two = true;
        }
        _ => { movecopy.widgets_enabled_two = false }
    };
    // Fill modifications
    {
        if movecopy.mode_to != MoveCopyToMode::None { modifications += 1 };
        if movecopy.mode_from != MoveCopyFromMode::None { modifications += 1 };
        if movecopy.letters_count >= 1 { modifications += 1 };
        if movecopy.mode_to_pos >= 1 { modifications += 1 };
        if movecopy.seperator_enabled == true { modifications += 1 };
    }
    return (movecopy.to_owned(), modifications);
}

fn fill_modname(_gui: &mut WindowMain, ui: &mut egui::Ui, name: &mut ModName, index: usize) -> (ModName, u32) {
    let mut modifications: u32 = 0;
    ui.horizontal(|ui| {
        ui.label("Mode");
        egui::ComboBox::new(format!("name-{}", index), "")
        .selected_text(name.mode_name.to_owned())
        .show_ui(ui, |ui| {
            if ui.selectable_label(false, "Keep").clicked() {
                name.mode_name = String::from("Keep");
                name.mode = NameMode::Keep
            }
            if ui.selectable_label(false, "Remove").clicked() {
                name.mode_name = String::from("Remove");
                name.mode = NameMode::Remove
            }
            if ui.selectable_label(false, "Fixed").clicked() {
                name.mode_name = String::from("Fixed");
                name.mode = NameMode::Fixed
            }
            if ui.selectable_label(false, "Reverse").clicked() {
                name.mode_name = String::from("Reverse");
                name.mode = NameMode::Reverse
            }
        });
        ui.add_enabled_ui(name.widgets_enabled, |ui| {
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()), 
                egui::text_edit::TextEdit::singleline(&mut name.fixed
            ));
        });
    });
    match name.mode {
        NameMode::Keep => {
            name.widgets_enabled = false;
        },
        NameMode::Fixed => {
            name.widgets_enabled = true;
        },
        _ => { name.widgets_enabled = false }
    };
    // Fill modifications
    {
        if name.fixed.chars().count() >= 1 { modifications += 1 };
        if name.mode != NameMode::Keep { modifications += 1 };
    }
    return (name.to_owned(), modifications);
}

fn fill_modnumber(gui: &mut WindowMain, ui: &mut egui::Ui, number: &mut ModNumber, index: usize) -> (ModNumber, u32) {
    let mut modifications: u32 = 0;
    let mut seperator: String = number.seperator.to_string();
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Mode");
            egui::ComboBox::new(format!("number-{}", index), "")
            .selected_text(number.mode_name.to_owned())
            .show_ui(ui, |ui| {
                if ui.selectable_label(false, "None").clicked() {
                    number.mode_name = String::from("None");
                    number.mode = NumberMode::None
                }
                if ui.selectable_label(false, "Prefix").clicked() {
                    number.mode_name = String::from("Prefix");
                    number.mode = NumberMode::Prefix
                }
                if ui.selectable_label(false, "Suffix").clicked() {
                    number.mode_name = String::from("Suffix");
                    number.mode = NumberMode::Suffix
                }
                if ui.selectable_label(false, "Insert").clicked() {
                    number.mode_name = String::from("Insert");
                    number.mode = NumberMode::Insert
                }
                if ui.selectable_label(false, "Prefix+Suffix").clicked() {
                    number.mode_name = String::from("Prefix+Suffix");
                    number.mode = NumberMode::PrefixAndSuffix
                }
            });
            ui.separator();
            ui.add_enabled_ui(number.insert_enabled, |ui| {
                ui.label("at");
                let drag = ui.add_enabled(true, 
                    egui::DragValue::new(&mut number.insert_at)
                    .range(-255..=255)
                    .speed(0.05)
                );

                if drag.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    number.insert_at += 1;
                } else if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if number.insert_at >= 1 {
                        number.insert_at -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    number.insert_at -= 1;
                };
                ui.separator();
        
                if ui.small_button("➕").clicked() {
                    number.insert_at += 1;
                };
            });
        });
        ui.horizontal(|ui| {
            ui.add_enabled_ui(number.widgets_enabled, |ui| {
                ui.label("Starting");
                let drag = ui.add_enabled(true, 
                    egui::DragValue::new(&mut number.starting_num)
                    .range(0..=25565)
                    .speed(0.05)
                );

                if drag.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    number.starting_num += 1;
                } else if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if number.starting_num >= 1 {
                        number.starting_num -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    if number.starting_num >= 1 {
                        number.starting_num -= 1;
                    }
                };
                ui.separator();
        
                if ui.small_button("➕").clicked() {
                    number.starting_num += 1;
                };

                ui.label("Increment");
                let drag_increment = ui.add_enabled(true, 
                    egui::DragValue::new(&mut number.increment_num)
                    .range(1..=255)
                    .speed(0.05)
                );
                if drag_increment.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    number.increment_num += 1;
                } else if drag.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if number.increment_num >= 2 {
                        number.increment_num -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    if number.increment_num >= 2 {
                        number.increment_num -= 1;
                    }
                };
                ui.separator();
        
                if ui.small_button("➕").clicked() {
                    number.increment_num += 1;
                };
            });
        });
        ui.horizontal(|ui| {
            ui.add_enabled_ui(number.widgets_enabled, |ui| {
                ui.label("Padding");
                let drag_increment = ui.add_enabled(true, 
                    egui::DragValue::new(&mut number.padding)
                    .range(0..=255)
                    .speed(0.05)
                );

                if drag_increment.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag_increment.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    number.padding += 1;
                } else if drag_increment.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if number.padding >= 1 {
                        number.padding -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    if number.padding >= 1 {
                        number.padding -= 1;
                    }
                };
                ui.separator();
        
                if ui.small_button("➕").clicked() {
                    number.padding += 1;
                };

                ui.label("Seperator Char");
                ui.checkbox(&mut number.seperator_enabled, "");
                ui.add_enabled_ui(number.seperator_enabled, |ui| {
                    let response = ui.add_sized(
                        egui::vec2(20.0, ui.available_height()), 
                        egui::text_edit::TextEdit::singleline(&mut seperator)
                        .char_limit(2)
                        .cursor_at_end(false)
                    );
                    if response.changed() {
                        if seperator.len() >= 1 {
                            let c = seperator.pop().unwrap();
                            number.seperator = c;
                        };
                    };
                });
            });
        });
        ui.horizontal(|ui| {
            ui.add_enabled_ui(number.widgets_enabled, |ui| {
                ui.label("Type Mode");
                egui::ComboBox::new(format!("numbertype-{}", index), "")
                .selected_text(number.mode_type_name.to_owned())
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, "Base 2").clicked() {
                        number.mode_type_name = String::from("Base 2");
                        number.mode_type = NumberTypeMode::BaseTwo
                    }
                    if ui.selectable_label(false, "Base 8").clicked() {
                        number.mode_type_name = String::from("Base 8");
                        number.mode_type = NumberTypeMode::BaseEight
                    }
                    if ui.selectable_label(false, "Base 10").clicked() {
                        number.mode_type_name = String::from("Base Ten");
                        number.mode_type = NumberTypeMode::BaseTen
                    }
                    if ui.selectable_label(false, "Base 16").clicked() {
                        number.mode_type_name = String::from("Base 16");
                        number.mode_type = NumberTypeMode::BaseSixteen
                    }
                    if ui.selectable_label(false, "Roman Numeral").clicked() {
                        number.mode_type_name = String::from("Roman Numeral");
                        number.mode_type = NumberTypeMode::RomanNumeral
                    }
                    if ui.selectable_label(false, "a - z").clicked() {
                        number.mode_type_name = String::from("a - z");
                        number.mode_type = NumberTypeMode::AlphaLower
                    }
                    if ui.selectable_label(false, "A - Z").clicked() {
                        number.mode_type_name = String::from("A - Z");
                        number.mode_type = NumberTypeMode::AlphaUpper
                    }
                    if ui.selectable_label(false, "a - Z").clicked() {
                        number.mode_type_name = String::from("a - Z");
                        number.mode_type = NumberTypeMode::AlphaLowerToUpper
                    }
                });
            });
        });
    });
    match number.mode {
        NumberMode::None => {
            number.widgets_enabled = false;
            number.insert_enabled = false;
        },
        NumberMode::Insert => {
            number.widgets_enabled = true;
            number.insert_enabled = true;
        },
        _ => { 
            number.widgets_enabled = true; 
            number.insert_enabled = false; 
        }
    };
    // Fill modifications
    {
        if number.increment_num >= 2 { modifications += 1 };
        if number.insert_at != 0 { modifications += 1 };
        if number.mode != NumberMode::None { modifications += 1 };
        if number.starting_num != 1 { modifications += 1 };
    }
    return (number.to_owned(), modifications);
}

fn fill_modregex(_gui: &mut WindowMain, ui: &mut egui::Ui, regex: &mut ModRegex, _index: usize) -> (ModRegex, u32) {
    let mut modifications: u32 = 0;
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Exp");
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()),
                egui::text_edit::TextEdit::singleline(&mut regex.replace_match))
        });
        ui.horizontal(|ui| {
            ui.label("Replace");
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()),
                egui::text_edit::TextEdit::singleline(&mut regex.replace_with))
        });
    });
    // Fill modifications
    {
        if regex.replace_match.chars().count() >= 1 { modifications += 1 };
        if regex.replace_with.chars().count() >= 1 { modifications += 1 };
    }
    return (regex.to_owned(), modifications);
}

fn fill_modremove(gui: &mut WindowMain, ui: &mut egui::Ui, remove: &mut ModRemove, index: usize) -> (ModRemove, u32) {
    let mut modifications: u32 = 0;
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label("First");
                let drag_first = ui.add_enabled(true, 
                    egui::DragValue::new(&mut remove.first_n)
                    .range(0..=255)
                    .speed(0.05)
                );

                if drag_first.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag_first.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    remove.first_n += 1;
                } else if drag_first.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if remove.first_n >= 1 {
                        remove.first_n -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    if remove.first_n >= 1 {
                        remove.first_n -= 1;
                    }
                };
                ui.separator();

                if ui.small_button("➕").clicked() {
                    remove.first_n += 1;
                };
            });
            
            ui.group(|ui| {
                ui.label("Last");
                let drag_last = ui.add_enabled(true, 
                    egui::DragValue::new(&mut remove.last_n)
                    .range(0..=255)
                    .speed(0.05)
                );

                if drag_last.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag_last.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    remove.last_n += 1;
                } else if drag_last.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if remove.last_n >= 1 {
                        remove.last_n -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    if remove.last_n >= 1 {
                        remove.last_n -= 1;
                    }
                };
                ui.separator();

                if ui.small_button("➕").clicked() {
                    remove.last_n += 1;
                };
            });
        });
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label("From");
                let drag_from = ui.add_enabled(true, 
                    egui::DragValue::new(&mut remove.from_x)
                    .range(0..=25565)
                    .speed(0.05)
                );

                if drag_from.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag_from.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    remove.from_x += 1;
                } else if drag_from.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if remove.from_x >= 1 {
                        remove.from_x -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    if remove.from_x >= 1 {
                        remove.from_x -= 1;
                    }
                };
                ui.separator();

                if ui.small_button("➕").clicked() {
                    remove.from_x += 1;
                };
            });

            ui.group(|ui| {
                ui.label("To");
                let drag_to = ui.add_enabled(true, 
                    egui::DragValue::new(&mut remove.to_y)
                    .range(remove.from_x..=25565)
                    .speed(0.05)
                );

                if drag_to.hovered() {
                    gui.modifiers.drag_box_hovered = true;
                };
        
                if drag_to.hovered() && ui.input(|input| {input.raw_scroll_delta.y >= 1.0}){
                    remove.to_y += 1;
                } else if drag_to.hovered() && ui.input(|input| {input.raw_scroll_delta.y <= -1.0}) {
                    if remove.to_y >= 1 {
                        remove.to_y -= 1;
                    }
                };
                if ui.small_button("➖").clicked() {
                    if remove.to_y >= 1 {
                        remove.to_y -= 1;
                    }
                };
                ui.separator();

                if ui.small_button("➕").clicked() {
                    remove.to_y += 1;
                };
            })
        });
        ui.horizontal(|ui| {
            ui.label("Chars");
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()), 
                egui::text_edit::TextEdit::singleline(&mut remove.chars_comma_seperated)
            );
        });
        ui.horizontal(|ui| {
            ui.label("Words");
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()), 
                egui::text_edit::TextEdit::singleline(&mut remove.words_comma_seperated)
            );
        });
        ui.horizontal(|ui| {
            ui.label("Crop");
            egui::ComboBox::new(format!("remove_crop-{}", index), "")
            .selected_text(remove.crop_name.to_owned())
            .show_ui(ui, |ui| {
                if ui.selectable_label(false, "None").clicked() {
                    remove.crop_name = String::from("None");
                    remove.crop = RemoveCropMode::None
                }
                if ui.selectable_label(false, "Before").clicked() {
                    remove.crop_name = String::from("Before");
                    remove.crop = RemoveCropMode::Before
                }
                if ui.selectable_label(false, "After").clicked() {
                    remove.crop_name = String::from("After");
                    remove.crop = RemoveCropMode::After
                }
            });
            ui.add_enabled_ui(remove.crop_enabled, |ui| {
                ui.add_sized(
                    egui::vec2(ui.available_width(), ui.available_height()), 
                    egui::text_edit::TextEdit::singleline(&mut remove.crop_match)
                    .char_limit(255)
                );
            });
        });
        ui.horizontal(|ui| {
            ui.label("Digits");
            ui.checkbox(&mut remove.digits, "");

            ui.label("Dbl. Spaces");
            ui.checkbox(&mut remove.double_spaces, "");

            ui.label("Lead ..");
            ui.checkbox(&mut remove.leading_dots, "");

            ui.label("Symb.");
            ui.checkbox(&mut remove.symbols, "");

            ui.label("Trim");
            ui.checkbox(&mut remove.trim, "");
        });
    });
    match remove.crop {
        RemoveCropMode::None => {
            remove.crop_enabled = false;
        },
        _ => {
            remove.crop_enabled = true;
        }
    }
    // Fill modifications
    {
        if remove.digits == true { modifications += 1 };
        if remove.double_spaces == true { modifications += 1 };
        if remove.leading_dots == true { modifications += 1 };
        if remove.symbols == true { modifications += 1 };
        if remove.trim == true { modifications += 1 };
        if remove.crop_match.chars().count() >= 1 { modifications += 1};
        if remove.crop != RemoveCropMode::None { modifications += 1 };
        if remove.chars_comma_seperated.chars().count() >= 1 { modifications += 1 };
        if remove.words_comma_seperated.chars().count() >= 1 { modifications += 1 };
        if remove.to_y >= 1 { modifications += 1 };
        if remove.from_x >= 1 { modifications += 1 };
        if remove.first_n >= 1 { modifications += 1 };
        if remove.last_n >= 1 { modifications += 1 };
    }
    return (remove.to_owned(), modifications);
}

fn fill_modreplace(_gui: &mut WindowMain, ui: &mut egui::Ui, replace: &mut ModReplace, _index: usize) -> (ModReplace, u32) {
    let mut modifications: u32 = 0;
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Match");
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()), 
                egui::text_edit::TextEdit::singleline(&mut replace.replace_match)
            );
        });
        ui.horizontal(|ui| {
            ui.label("Replace");
            ui.add_sized(
                egui::vec2(ui.available_width(), ui.available_height()), 
                egui::text_edit::TextEdit::singleline(&mut replace.replace_with)
            );
        });
        ui.horizontal(|ui| {
            ui.label("First");
            ui.checkbox(&mut replace.first_occurance, "");
        });
    });
    // Fill modifications
    {
        if replace.first_occurance == true { modifications += 1 };
        if replace.replace_match.chars().count() >= 1 { modifications += 1};
        if replace.replace_with.chars().count() >= 1 { modifications += 1};
    }
    return (replace.to_owned(), modifications);
}