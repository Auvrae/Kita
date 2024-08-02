use egui::Context;
use super::super::super::app::WindowMain;
use super::super::main_sub::file_selector::FileSelection;
use super::super::super::util::threads;

pub fn bar(gui: &mut WindowMain, ctx: &Context) {
    egui::TopBottomPanel::top("Top")
    .exact_height(gui.bar_top_height)
    .show(ctx, |ui| {
        ui.add_enabled_ui(gui.bar_top_enabled, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    ui.add_enabled_ui(gui.save_available, |ui| {
                        if ui.button("Save").clicked() && gui.is_popup_open() == false {
                            // Do some saving
                            ui.close_menu();
                            gui.popups.save_confirmation = true;
                        };
                    });
                    let recent_enabled: bool;
                    if gui.options.recently_opened.len() >= 1 {
                        recent_enabled = true;
                    } else {
                        recent_enabled = false;
                    }
                    ui.add_enabled_ui(recent_enabled, |ui| {
                        ui.menu_button("Open recent..", |ui| {
                            for (_, recent_path) in gui.options.recently_opened.iter().enumerate() {
                                if ui.selectable_label(false, recent_path.to_owned()).clicked() {
                                    #[cfg(target_os="windows")] 
                                    {
                                        let root: String = recent_path.clone().split_once("/").unwrap().0.to_string();
                                        for (index, mount) in gui.file_mounts.iter().enumerate() {
                                            if *mount == format!("{}/", root) {
                                                gui.file_mounts_selected = index as u8;
                                                break;
                                            }
                                        }
                                    }
                                    *gui.modifier_thread_storage.kill_sig_string_processor.lock().unwrap() = true;
                                    while *gui.modifier_thread_storage.state.lock().unwrap() != threads::ThreadState::Dead {}
                                    gui.reset_processing = true;
                                    gui.file_browser.selected_folders.clear();
                                    gui.file_browser.browse_to(recent_path.to_owned()).unwrap();
                                }
                            }
                        });
                    });
    
                    ui.separator();
    
                    if ui.button("Quit").clicked() {
                        gui.popups.quit = !gui.popups.quit;
                        ui.close_menu();
                    }
                });
                ui.menu_button("Edit", |ui| {
                    // Undo
                    if gui.edits.undo.is_some() {   
                        if ui.button("Undo").clicked() {
                            gui.undo();
                            ui.close_menu();
                        };
                    } else {
                        ui.add_enabled_ui(false, |ui: &mut egui::Ui| { let _ = ui.button("Undo"); });
                    };
    
                    // Redo 
                    if gui.edits.redo.is_some() {   
                        if ui.button("Redo").clicked() {
                            gui.redo();
                            ui.close_menu();
                        };
                    } else {
                        ui.add_enabled_ui(false, |ui: &mut egui::Ui| { let _ = ui.button("Redo"); });
                    };
    
                    ui.separator();
    
                    if ui.button("Options").clicked() {
                        gui.popups.options = !gui.popups.options;
                        ui.close_menu();
                    };
                });
                ui.menu_button("Preset", |ui| {
                    if ui.button("Save as preset").clicked() {
                        gui.popups.save_as_preset = !gui.popups.save_as_preset;
                        ui.close_menu();
    
                    };
    
                    ui.separator();
                    
                    ui.add_enabled_ui(false, |ui| {
                        if ui.button("Preset Manager").clicked() {
                            ui.close_menu();
                        };
                    });
                    /*
                    if ui.button("Preset Manager").clicked() && gui.is_popup_open() == false {
                        gui.popups.preset_manager = !gui.popups.preset_manager;
                        ui.close_menu();
                    }
                    */
                });
            });
        });
    });
}