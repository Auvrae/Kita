use super::super::super::app::WindowMain;
use super::super::super::presets::Preset;
use super::super::super::super::config;

pub fn window(gui: &mut WindowMain, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.set_min_size(egui::Vec2::new(200.0, 50.0));
    ui.set_max_size(egui::Vec2::new(200.0, 50.0));
    ui.vertical( |ui| {
        ui.add_space(7.0);
        ui.group(|ui| {
            ui.set_min_size(egui::Vec2::new(198.0, 72.0));
            ui.set_max_size(egui::Vec2::new(198.0, 72.0));
            ui.horizontal(|ui| {
                ui.label("Are you sure you want to save the current modifiers to a preset?\n");
            });

            ui.horizontal(|ui| {
                ui.label("Preset Name: ");
                let field = ui.add_sized(
                    egui::vec2(ui.available_width(), ui.available_height()), 
                    egui::text_edit::TextEdit::singleline(&mut gui.popups.save_as_preset_field)
                );

                field.on_hover_text("Preset name optional, will display \nthe time since EPOCH in milliseconds instead.")
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    gui.popups.save_as_preset = false;
                    gui.popups.save_as_preset_field = String::new();
                };
                let checked: bool;
                { // Check if the name is valid
                    if gui.popups.save_as_preset_field.contains(" ") {
                        checked = false;
                    } else {
                        checked = true;
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.add_enabled_ui(checked, |ui| {
                        if ui.button("Confirm").clicked() {
                            gui.presets.sets.push(Preset {
                                name: {
                                    if gui.popups.save_as_preset_field.len() >= 1 && checked {
                                        gui.popups.save_as_preset_field.clone()
                                    } else {
                                        String::from(gui.local_time.clone().timestamp_millis().to_string())
                                    }
                                },
                                modifier_order: gui.options.modifier_order.0.clone(),
                                modifiers: gui.modifiers.clone(),
                                file_extension_filter: vec![],
                                include_files: false,
                                include_folders: false
                            });
                            config::write_presets(gui.presets.to_owned()).unwrap(); 
                            gui.popups.save_as_preset = false; 
                            gui.popups.save_as_preset_field = String::new();
                        };
                    });
                });
            });
        });
    });
}