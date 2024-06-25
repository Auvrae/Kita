#![windows_subsystem = "windows"]
#![allow(dead_code, unused_variables)]

mod rename;

use eframe::egui;
use egui_extras;
use rename::app::WindowMain;

#[cfg(target_os="windows")]
use winapi::um::wincon::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS}; 


fn main() -> Result<(), eframe::Error> {
    #[cfg(target_os="windows")]
    unsafe {
        FreeConsole();
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // Remove the path argument. 

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder {
            app_id: Some(String::from("auv.bulk_rename.kita")),
            inner_size: Some(egui::vec2(1280.0, 800.0)),
            min_inner_size: Some(egui::vec2(1280.0, 800.0)),
            //icon: Some(std::sync::Arc::new(eframe::icon_data::from_png_bytes(include_bytes!("icon.png")).unwrap())), // Icon! (Doesn't work on Linux though)
            drag_and_drop: Some(true),
            minimize_button: Some(true),
            maximize_button: Some(true),
            titlebar_buttons_shown: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };
    let main = WindowMain {
        cli_args: args,
        ..Default::default()
    };

    eframe::run_native(
        "Kita",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(main)
        }),
    )
}