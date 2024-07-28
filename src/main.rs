#![windows_subsystem = "windows"]
#![allow(dead_code, unused_variables, unused_imports)]

mod rename;

use eframe::egui;
use egui_extras;
use rename::app::WindowMain;
use rename::util::config;
use rename::cli::parser;

#[cfg(target_os="windows")]
use winapi::um::wincon::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS}; 


fn main() -> Result<(), eframe::Error> {
    #[cfg(target_os="windows")]
    /*
        We need to free the console in case it's still attached. 
        Then reattach to make sure it connects to the terminal window correctly.
        This a janky fix for #![windows_subsystem = "windows"] to make sure the window 
        doesn't spawn a terminal window when opening.
    */
    unsafe {
        FreeConsole();
        AttachConsole(ATTACH_PARENT_PROCESS);
    }

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder {
            app_id: Some(String::from("auv.bulk_rename.kita")),
            inner_size: Some(egui::vec2(1280.0, 800.0)), // Default size
            min_inner_size: Some(egui::vec2(1280.0, 800.0)), // Minimum size
            //icon: Some(std::sync::Arc::new(eframe::icon_data::from_png_bytes(include_bytes!("icon.png")).unwrap())), // Icon! (Doesn't work on Linux though)
            minimize_button: Some(true),
            maximize_button: Some(true),
            titlebar_buttons_shown: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };
    let mut pre_options = config::read_config();

    // Prevert modifier_order from having extra elements.
    if pre_options.modifier_order.0.len() != 11 {
        pre_options.modifier_order = rename::app::ModifierOrder::default();
    }

    // Prevent modifier_order from having multiple of the same varient.
    {
        let mut found: Vec<u8> = vec![];
        for (index, varient) in rename::mods::ModsOrder::iterate_over_oneness().enumerate() {
            found.push(0);
            for (_, v) in pre_options.modifier_order.0.iter().enumerate() {
                if *v == varient {
                    found[index] += 1;
                };
            };
        };
        for count in found { 
            if count > 1 || count == 0 {
                pre_options.modifier_order = rename::app::ModifierOrder::default();
                break;
            };
        };
    }

    let pre_presets = config::read_presets();
    let mut main = WindowMain {
        options: pre_options,
        presets: pre_presets,
        ..Default::default()
    };
    
    // Do CLI Commands
    {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0); // Remove the path argument. 
        let cli = parser::parse_arguments(&mut main, args);
        match cli {
            parser::CliResult::Error(error) => {
                println!("Could not be completed: {}", error);
                std::process::exit(0);
            },
            parser::CliResult::Stop => {
               return Ok(());
            },
            _ => {}
        }
    }

    eframe::run_native(
        "Kita",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(main))
        }),
    )
}