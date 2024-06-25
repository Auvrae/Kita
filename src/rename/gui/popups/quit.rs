use super::super::super::app::WindowMain;


pub fn window(gui: &mut WindowMain, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.set_min_size(egui::Vec2::new(200.0, 50.0));
    ui.set_max_size(egui::Vec2::new(200.0, 50.0));
    ui.vertical( |ui| {
        ui.add_space(7.0);
        ui.group(|ui| {
            ui.set_min_size(egui::Vec2::new(195.0, 45.0));
            ui.set_max_size(egui::Vec2::new(195.0, 45.0));
            ui.horizontal(|ui| {
                ui.label("Are you sure you want to quit?");
            });
            
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    gui.popups.quit = false;
                };
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("Confirm").clicked() {
                        std::process::exit(0);
                    };
                });
            });
        });
    });
}