use egui::Context;
use super::super::super::app::WindowMain;

pub fn bar(gui: &mut WindowMain, ctx: &Context) {
    egui::TopBottomPanel::top("Top")
    .exact_height(gui.bar_top_height)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                ui.add_enabled_ui(gui.save_available, |ui| {
                    if ui.button("Save").clicked() && gui.is_popup_open() == false {
                        // Do some saving
                        ui.close_menu();
                        gui.popups.save_confirmation = true;
                    };
                });
                ui.add_enabled_ui(false, |ui| {
                    ui.menu_button("Open recent..", |_ui| {
    
                    });
                });

                ui.separator();

                if ui.button("Quit").clicked() {
                    gui.popups.quit = !gui.popups.quit;
                    ui.close_menu();
                }
            });
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    ui.close_menu();

                };

                if ui.button("Redo").clicked() {
                    ui.close_menu();
                    
                };
                
                ui.separator();

                if ui.button("Options").clicked() {
                    gui.popups.options = !gui.popups.options;
                    ui.close_menu();
                };
            });
            ui.menu_button("Preset", |ui| {
                if ui.button("Save as preset").clicked() {
                    ui.close_menu();

                };

                ui.separator();

                if ui.button("Preset Manager").clicked() && gui.is_popup_open() == false {
                    gui.popups.preset_manager = !gui.popups.preset_manager;
                    ui.close_menu();
                }
            });
        });
    });
}