use super::super::super::app::WindowMain;


pub fn window(_gui: &mut WindowMain, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.set_min_size(egui::Vec2::new(960.0, 500.0));
    ui.set_max_size(egui::Vec2::new(960.0, 500.0));
    ui.vertical( |ui| {
        ui.add_space(7.0);
        ui.group(|ui| {
            ui.set_min_size(egui::Vec2::new(955.0, 495.0));
            ui.set_max_size(egui::Vec2::new(955.0, 495.0));

        });
    });
}