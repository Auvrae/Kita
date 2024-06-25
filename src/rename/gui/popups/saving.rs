use super::super::super::util::threads;
use super::super::super::app::WindowMain;

pub fn window(gui: &mut WindowMain, ui: &mut egui::Ui, ctx: &egui::Context) {
    match *gui.thread_storage.state.lock().unwrap() {
        threads::ThreadState::Completed => {
            gui.popups.saving = false;
            gui.reset_processing = true;
        },
        _ => {}
    };
    let progress = *gui.thread_storage.progress.lock().unwrap();
    ctx.request_repaint(); // Keep the bar moving
    ui.set_min_size(egui::Vec2::new(300.0, 40.0));
    ui.set_max_size(egui::Vec2::new(300.0, 40.0));
    ui.vertical(|ui| {
        ui.separator();
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.add(egui::ProgressBar::new((progress / 100.0) as f32));
                ui.separator();
                ui.add(egui::Label::new((progress as u32).to_string()));
            })
        })
    });
}