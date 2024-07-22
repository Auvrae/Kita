//use std::fs;
#[cfg(target_os="windows")]
use winreg;

fn main () -> Result<(), eframe::Error> {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // Remove the path argument. 

    // Do CLI stuff
    {
        
    }


    let options = eframe::NativeOptions {
        //renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder {
            app_id: Some(String::from("auv.bulk_rename.kita_debug")),
            inner_size: Some(egui::vec2(300.0, 150.0)), // Default size
            min_inner_size: Some(egui::vec2(300.0, 150.0)), // Minimum size
            titlebar_buttons_shown: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };
    
    pub struct App {
        pub label_value: String,
        pub label_two_value: String
    }

    impl Default for App {
        fn default() -> Self {
            Self {
                label_value: String::new(),
                label_two_value: String::new()
            }
        }
    }

    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("     ").clicked() {
                            #[cfg(target_os="windows")]
                            {
                                let khcu = winreg::RegKey::predef(winreg::enums::HKEY_CLASSES_ROOT);
                                let root_path = std::path::Path::new("Directory\\shell").join("Kita");
                                let command_path = std::path::Path::new("Directory\\shell\\Kita").join("command");
                                let (root_key, _disp) = khcu.create_subkey(&root_path).unwrap();
                                let (command_key, _disp) = khcu.create_subkey(&command_path).unwrap();
                                root_key.set_value("", &"Kita &Rename &Here").unwrap();
                                command_key.set_value("", &"\"C:\\Users\\Cheat\\kita-windows-x64.exe\" \"%1\" %*").unwrap();
                                self.label_value = String::from("Success")
                            }
                        };
                        ui.label("Install \"Kita Rename Here\" (Folders)");
                    });
                    ui.label(self.label_value.clone());
                    ui.horizontal(|ui| {
                        if ui.button("     ").clicked() {
                            #[cfg(target_os="windows")]
                            {
                                let khcu = winreg::RegKey::predef(winreg::enums::HKEY_CLASSES_ROOT);
                                let root_path = std::path::Path::new("Directory\\shell").join("Kita");
                                khcu.delete_subkey_all(root_path).unwrap();
                                self.label_two_value = String::from("Success")
                            }
                        };
                        ui.label("Uninstall \"Kita Rename Here\" (Folders)");
                    });
                    ui.label(self.label_two_value.clone());
                });

            });
        }
    }

    eframe::run_native(
        "Context Menu Tester",
        options,
        Box::new(|_cc| {
            Ok(Box::<App>::default())
        }),
    )
}