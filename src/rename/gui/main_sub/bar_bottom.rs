use egui::Context;
use super::super::super::app::WindowMain;
use super::super::super::util::config;

pub fn bar(gui: &mut WindowMain, ctx: &Context) {
    egui::TopBottomPanel::bottom("Bottom")
    .exact_height(gui.bar_bottom_height)
    .show(ctx, |ui| {
        ui.vertical(|ui| {ui.add_space(2.0)});
        ui.horizontal(|ui| {
            ui.label(format!("v{}", env!("CARGO_PKG_VERSION").to_string()));

            ui.separator();
            
            if ui.button("Save Settings").clicked() {
                config::write_config(gui.options.clone()).unwrap();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                let year = gui.local_time.format("%Y");
                let month = gui.local_time.format("%m");
                let day = gui.local_time.format("%d");
                let hour = gui.local_time.format("%H");
                let minute = gui.local_time.format("%M");
                let second = gui.local_time.format("%S");
                ui.label(format!("{}-{}-{} {}:{}:{}", year, month, day, hour, minute, second));
            });
        });
    });
}