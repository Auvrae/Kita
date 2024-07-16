use std::time::Instant;

use super::super::super::app::WindowMain;
use super::super::super::util::dir::{Folder, get_folder};
use super::super::super::debug::DebugStatType;

pub fn selector(gui: &mut WindowMain, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.add_enabled_ui(gui.section_selector_enabled, |ui| {
        let start = Instant::now();
        // Update Files if necessary. 
        if gui.file_browser.selected_children_paths != gui.file_selector.previously_selected_folders {
            gui.file_selector.previously_selected_folders = gui.file_browser.selected_children_paths.to_owned(); // Copy new paths.
            gui.file_selector.folders.clear(); // Clean local folders.
            gui.file_selector.last_selected_folder.clear();
            gui.file_selector.last_selected_file.clear();
            for path in gui.file_browser.selected_children_paths.to_owned() {
                match get_folder(path, false) {
                    Ok(folder) => { 
                        gui.file_selector.folders.push(folder); 
                        gui.file_selector.last_selected_folder.push(0);
                        gui.file_selector.last_selected_file.push(0);
                    },
                    Err(err) => { 
                        // Do something about it.
                        gui.file_selector.folders.push(Folder {
                            errored: Some(true),
                            errored_message: Some(String::from(err.to_string())),
                            ..Default::default()
                        }); 
                        gui.file_selector.last_selected_folder.push(0);
                        gui.file_selector.last_selected_file.push(0);
                    }
                };
            };
        };
        
        // Gui
        ui.vertical(|ui| {
            ui.vertical(|ui| {
                // Header
                ui.horizontal(|ui| {
                    if gui.file_browser.collapsed == true {
                        let collapse_button = ui.button("➡");
                        if collapse_button.clicked() {
                            gui.file_browser.collapsed = !gui.file_browser.collapsed;
                        };
                        if collapse_button.hovered() {
                            collapse_button.on_hover_text("Restore file browser");
                        };
                    }
                    ui.strong("File Selection");
                });
                ui.separator();
                let space = ui.add_enabled_ui(true, |ui| {
                    let available_width = ui.available_width();
                    let available_height = ui.available_height();
                    ui.set_min_height(ui.available_height());
                    egui_extras::TableBuilder::new(ui)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::TOP))
                    .column(egui_extras::Column::auto().at_most(available_width - 5.0).clip(true))
                    .min_scrolled_height(available_height)
                    .striped(gui.options.file_selection.stripped_column)
                    .body(|mut body| {
                        if gui.file_selector.allow_frame == false { return };
                        for (index, folder) in gui.file_selector.folders.clone() .iter().enumerate(){
                            let tables = fill_table(gui, &mut body, folder, index, available_width);
                            gui.file_selector.folders[index].selected_folders = tables[0].to_owned();
                            gui.file_selector.folders[index].selected_files = tables[1].to_owned();
                        }
                    });
                });
                if space.response.hovered() && gui.input_state.is_some() {
                    let state = gui.input_state.clone().unwrap();
                    if state.modifiers.ctrl && state.key_pressed(egui::Key::A) {
                        for (i, folder) in gui.file_selector.folders.clone().iter().enumerate() {
                            for (u, _) in folder.selected_folders.iter().enumerate() {
                                gui.file_selector.folders[i].selected_folders[u] = true;
                            }
                            for (u, _) in folder.selected_files.iter().enumerate() {
                                gui.file_selector.folders[i].selected_files[u] = true;

                            }
                        }
                    } else if space.response.hovered() && state.pointer.button_double_clicked(egui::PointerButton::Primary){
                        for (i, folder) in gui.file_selector.folders.clone().iter().enumerate() {
                            for (u, _) in folder.selected_folders.iter().enumerate() {
                                gui.file_selector.folders[i].selected_folders[u] = false;
                            }
                            for (u, _) in folder.selected_files.iter().enumerate() {
                                gui.file_selector.folders[i].selected_files[u] = false;

                            }
                        }
                    }
                };
            });
        });


        // Update Stats
        gui.statistics.push(start.elapsed().as_micros() as u32, DebugStatType::GuiSelector);
    });
}

fn fill_table(
    gui: &mut WindowMain, 
    body: &mut egui_extras::TableBody, 
    folder: &Folder, 
    folder_index: usize, 
    width_available: f32
) -> Vec<Vec<bool>> {
    let mut selected_folders: Vec<bool> = folder.selected_folders.clone();
    let mut selected_files: Vec<bool> = folder.selected_files.clone();
    gui.file_selector.total_errored = 0;
    if folder.errored.is_some() {
        let error = folder.errored_message.clone().unwrap();
        body.row(16.0, |mut ui| {
            ui.col(|ui| {
                ui.strong(gui.file_browser.selected_children_paths[folder_index].to_owned());
            });
        });
        body.row(16.0, |mut ui| {
            ui.col(|ui| {
                ui.colored_label(egui::Color32::RED, error);
            });
        });
        return vec![selected_folders, selected_files];
    };
    
    body.row(16.0, |mut ui| {
        ui.col(|ui| {
            ui.strong(folder.path.to_owned());
        });
    });

    if gui.options.file_selection.list_folders == true {
        for (index, item) in folder.list_folders.iter().enumerate() {
            let mut selected = false;
            body.row(16.0, |mut ui| {
                ui.col(|ui| {
                    ui.set_width(width_available);
                    if ui.toggle_value(&mut selected_folders[index], format!("🗁 {}", item.name.to_owned())).clicked() {
                        selected = true;
                    } else {
                        selected = false;
                    }
                    if gui.input_state.is_some() {
                        let state = gui.input_state.clone().unwrap();
                        if selected == true && state.modifiers.shift && !state.modifiers.ctrl {
                            let last_selected = gui.file_selector.last_selected_folder[folder_index];
                            if index > last_selected{
                                let select = index - last_selected;
                                for i in 0..=select {
                                    selected_folders[last_selected+i] = true;
                                };
                            }
                            else if index < last_selected {
                                for i in index..=last_selected {
                                    selected_folders[i] = true;
                                };
                            };
                        } else if selected == true && !state.modifiers.shift && !state.modifiers.ctrl {
                            for i in 0..selected_folders.len() {
                                selected_folders[i] = false;
                            };
                            selected_folders[index] = true;
                        };
                    };
                    if selected == true {
                        gui.file_selector.last_selected_folder[folder_index] = index;
                    }
                    ui.end_row();
                });
            });
            if selected_folders[index] == true {
                body.row(16.0, |mut ui| {
                    ui.col(|ui| {
                        ui.set_width(width_available);
                        if item.errored == true {
                            gui.file_selector.total_errored += 1;
                            ui.colored_label(
                                egui::Color32::RED, format!("---> {}", 
                                item.name_modified.to_owned()))
                            .on_hover_text(item.error.to_owned());
                        } else {
                            ui.label(format!("---> {}", item.name_modified.to_owned()));
                        }
                    });
                });
            } else if gui.options.file_selection.always_show_extra_row == true {
                body.row(16.0, |mut ui| {
                    ui.col(|ui| {
                        ui.set_width(width_available);
                        ui.label(format!("---> "));
                    });
                });
            };
        };
    };

    for (index, file) in folder.list_files.iter().enumerate() {
        body.row(16.0, |mut ui| {
            ui.col(|ui| {
                ui.set_width(width_available);
                let selected = ui.toggle_value(&mut selected_files[index], format!("{}", file.name.to_owned())).clicked();
                if gui.input_state.is_some() {
                    let state = gui.input_state.clone().unwrap();
                    if selected == true && state.modifiers.shift && !state.modifiers.ctrl {
                        let last_selected = gui.file_selector.last_selected_file[folder_index];
                        if index > last_selected{
                            let select = index - last_selected;
                            for i in 0..=select {
                                selected_files[last_selected+i] = true;
                            };
                        }
                        else if index < last_selected {
                            for i in index..=last_selected {
                                selected_files[i] = true;
                            };
                        };
                    } else if selected && !state.modifiers.shift && !state.modifiers.ctrl {
                        for i in 0..selected_files.len() {
                            selected_files[i] = false;
                        };
                        selected_files[index] = true;
                    };
                };
                if selected == true {
                    gui.file_selector.last_selected_file[folder_index] = index;
                };
                ui.end_row();
            });
        });
        if selected_files[index] == true {
            body.row(16.0, |mut ui| {
                ui.col(|ui| {
                    ui.set_width(width_available);
                    if file.errored == true {
                        gui.file_selector.total_errored += 1;
                        ui.colored_label(
                            egui::Color32::RED, format!("---> {}", 
                            file.name_modified.to_owned()))
                        .on_hover_text(file.error.to_owned());
                    } else {
                        ui.label(format!("---> {}", file.name_modified.to_owned()));
                    }
                });
            });
        } else if gui.options.file_selection.always_show_extra_row == true {
            body.row(16.0, |mut ui| {
                ui.col(|ui| {
                    ui.set_width(width_available);
                    ui.label(format!("---> "));
                });
            });
        };
    };

    return vec![selected_folders, selected_files];
}

#[derive(Default, Clone)]
pub struct FileSelection {
    pub folders: Vec<Folder>,
    pub previously_selected_folders: Vec<String>,
    pub last_selected_folder: Vec<usize>,
    pub last_selected_file: Vec<usize>,
    pub selected_folder_paths: Vec<(String, usize, usize)>,
    pub selected_file_paths: Vec<(String, usize, usize)>,
    pub total_errored: u32,
    pub allow_frame: bool
}