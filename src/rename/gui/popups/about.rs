use egui::Widget;
use super::super::super::util::threads;
use super::super::super::app::WindowMain;

pub fn window(gui: &mut WindowMain, ui: &mut egui::Ui, ctx: &egui::Context) {
    ui.set_min_size(egui::Vec2::new(350.0, 150.0));
    ui.set_max_size(egui::Vec2::new(350.0, 150.0));
    ui.vertical(|ui| {
        ui.group(|ui| {
            ui.set_min_size(egui::Vec2::new(345.0, 145.0));
            ui.set_max_size(egui::Vec2::new(345.0, 145.0));
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                        let mut header_text = egui::text::LayoutJob::default();
                        let heading_text = format!("{}{}",
                            "Kita is a Renaming Utility written in Rust with the aim to bring",
                            "a comprehensive renaming utility to Linux.\n\n\n\n",
                        );
                        header_text.append(&heading_text, 0.0, egui::TextFormat {
                            valign: egui::Align::Min,
                            ..Default::default()
                        });
                        ui.label(header_text);

                        ui.label(format!("{}{}", 
                            "Author: Auvrae\n",
                            "Beta Testers: Winter, Cheatfreak"
                        ));

                        ui.separator();
                        
                        ui.label("Kita Rename Utility");
                        
                        let link = egui::Hyperlink::new("https://github.com/Auvrae/Kita").open_in_new_tab(true);
                        ui.add(link);
                    });
                });
            });
        })
    });
}