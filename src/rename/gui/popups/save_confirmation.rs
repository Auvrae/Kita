use super::super::super::app::WindowMain;
use super::super::super::util::threads;


pub fn window(gui: &mut WindowMain, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.set_min_size(egui::Vec2::new(300.0, 105.0));
    ui.set_max_size(egui::Vec2::new(300.0, 105.0));
    ui.vertical( |ui| {
        ui.add_space(7.0);
        ui.group(|ui| {
            ui.set_min_size(egui::Vec2::new(290.0, 140.0));
            ui.set_max_size(egui::Vec2::new(290.0, 140.0));
            let modifications_total = gui.modifications_total;
            let selected_total = gui.file_selected_total;
            ui.label(format!("You are about to write these changes to disk. \n\nYou have made {} change(s) to {} files \n\n\n Are you sure you want to save the changes?", modifications_total, selected_total));
            if gui.file_selector.total_errored != 0 {
                ui.colored_label(
                    egui::Color32::RED, 
                    format!("\n\nThere are {} unresolved errors in the file selection pane.", 
                        gui.file_selector.total_errored
                ));
            } else {
                ui.allocate_space(egui::vec2(280.0, 32.0));
            }
            ui.separator();
            ui.horizontal( |ui| {
                if ui.button("Cancel").clicked() {
                    // Cancel
                    gui.popups.save_confirmation = false;
                };
                ui.separator();
                ui.add_space(180.0);
                ui.separator();
                
                ui.add_enabled_ui(gui.save_available, |ui| {
                    if ui.button("Save").clicked() {
                        // Do some saving
                        gui.popups.save_confirmation = false;
                        if gui.modifiers.hash_enable && gui.modifiers.hash.mode != threads::HashMode::None {
                            gui.hash();
                        } else {
                            gui.save(None);
                        };
                    };
                });
            })
        });
    });
}