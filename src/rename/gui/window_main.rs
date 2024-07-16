use egui;
use eframe;
use std::sync::{Arc, Mutex};

use super::popups;
use super::main_sub::{bar_bottom, bar_top, file_browser, file_selector, file_modifications};
use super::super::util::threads::{ThreadFunction, ThreadState, thread, ModifierThreadStorage};
use super::super::app::{WindowMain, Theme};
use super::super::debug::DebugStatType;

impl eframe::App for WindowMain {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // Frame Zero
        {
            if self.first_frame {
                // Set OS
                if self.no_refresh == false {
                    if cfg!(unix) {
                        self.operating_system = String::from("Unix");
                        self.file_browser.roots = self.read_directory(String::from("/"), true, vec![]);
                    } else if cfg!(windows) { // Do windows crap.
                        #[cfg(target_os="windows")]
                        {
                            self.operating_system = String::from("Windows");
                            self.file_browser.roots = self.read_directory(String::from("C:/"), true, vec![]);
                            let mut buffer: [u8; 256] = [0; 256];
                            unsafe { windows::Win32::Storage::FileSystem::GetLogicalDriveStringsA(Some(&mut buffer)); };
                            let mut b = vec![];
                            for i in buffer.to_vec() {
                                if i != 0 {
                                    b.push(i);
                                }
                            }
                            let chars = String::from_utf8(b).unwrap();
                            let letters: Vec<&str> = chars.split_terminator('\\').collect();
                            for drive_letter in letters {
                                self.file_mounts.push(format!("{}/", drive_letter.to_string().to_owned()))
                            };
                        }
                    };
                };
                thread(self, ThreadFunction::StringProcessing(16)); // Initialize String Processing Thread
                self.file_browser.allow_frame = true;
                self.file_selector.allow_frame = true;
                self.modifiers.allow_frame = true;
                self.options.gui_scale = {
                    if self.options.gui_scale == 0.0 {
                        ctx.pixels_per_point()
                    } else {
                        self.options.gui_scale
                    }
                }; // Store OS scale.
                self.no_refresh = false;
                self.first_frame = false;
            };
        }

        // Open Popup windows
        {
            // Options
            let mut options_open: bool = self.popups.options;
            egui::Window::new("Preferences")
                .id(self.popups.options_id)
                .default_pos(egui::pos2(self.window_size.x * 0.25, self.window_size.y * 0.25))
                .collapsible(false)
                .resizable(false)
                .open(&mut options_open)
                .show(ctx, |ui| {
                    popups::options::window(self, ui, ctx);
            });
            self.popups.options = options_open;

            // Save confirmation
            if self.popups.save_confirmation == true {
                egui::Window::new("Save Confirmation")
                .id(self.popups.save_confirmation_id)
                .default_pos(egui::pos2(self.window_size.x * 0.35, self.window_size.y * 0.35))
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .title_bar(true)
                .show(ctx, |ui| {
                    popups::save_confirmation::window(self, ui, ctx);
                });
            };

            // Hashing
            if self.popups.hashing {
                egui::Window::new("Hashing..")
                .id(self.popups.hashing_id)
                .default_pos(egui::pos2(self.window_size.x * 0.35, self.window_size.y * 0.35))
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .title_bar(true)
                .show(ctx, |ui| {
                    popups::hashing::window(self, ui, ctx);
                });
            };
            
            // Preset Manager
            if self.popups.preset_manager {
                egui::Window::new("Preset Manager")
                .id(self.popups.preset_manager_id)
                .default_pos(egui::pos2(self.window_size.x * 0.15, self.window_size.y * 0.15))
                .collapsible(false)
                .resizable(false)
                .movable(true)
                .title_bar(true)
                .show(ctx, |ui| {
                    popups::preset_manager::window(self, ui, ctx);
                });
            };
            // Saving
            if self.popups.saving {
                egui::Window::new("Saving..")
                .id(self.popups.saving_id)
                .default_pos(egui::pos2(self.window_size.x * 0.35, self.window_size.y * 0.35))
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .title_bar(true)
                .show(ctx, |ui| {
                    popups::saving::window(self, ui, ctx);
                });
            };

            // Quit
            if self.popups.quit {
                egui::Window::new("Quit Confirmation")
                .id(self.popups.preset_manager_id)
                .default_pos(egui::pos2(self.window_size.x * 0.35, self.window_size.y * 0.35))
                .collapsible(false)
                .resizable(false)
                .movable(true)
                .title_bar(true)
                .show(ctx, |ui| {
                    popups::quit::window(self, ui, ctx);
                });
            }


            // Save As Preset
            if self.popups.save_as_preset {
                egui::Window::new("Preset Confirmation")
                .id(self.popups.preset_manager_id)
                .default_pos(egui::pos2(self.window_size.x * 0.35, self.window_size.y * 0.35))
                .collapsible(false)
                .resizable(false)
                .movable(true)
                .title_bar(true)
                .show(ctx, |ui| {
                    popups::save_as_preset::window(self, ui, ctx);
                });
            }

            // Debug
            if self.popups.debug {
                egui::Window::new("DEBUG")
                .collapsible(true)
                .show(ctx, |ui| {
                    popups::debug::window(self, ui, ctx);
                });
            };
        }

        // Pre frame 
        {
            ctx.input(|state| { self.input_state = Some(state.clone()) }); // Update internal input state.
            self.window_size = ctx.available_rect().size();
            
            // Update CPU Usage
            let cpu_u = frame.info().cpu_usage.clone().take();
            if cpu_u.is_some() {
                self.cpu_usage = cpu_u.unwrap() * 1000.0
            };

            // Refill eddited files / folders
            let edits_thread = Arc::clone(&self.modifier_thread_storage.eddited_files);
            let errors_thread = Arc::clone(&self.modifier_thread_storage.errors);
            if edits_thread.lock().unwrap().is_some() && errors_thread.lock().unwrap().is_some() {
                let edits_thread = Arc::clone(&self.modifier_thread_storage.eddited_files);
                let edits = edits_thread.lock().unwrap().clone().unwrap();
                *edits_thread.lock().unwrap() = None;
                
                let errors_thread = Arc::clone(&self.modifier_thread_storage.errors);
                let errors = errors_thread.lock().unwrap().clone().unwrap();
                *errors_thread.lock().unwrap() = None;
                self.fill_selected_renamed(edits, errors);
            }

            // Get thread calculation time
            let thread_time = Arc::clone(&self.modifier_thread_storage.thread_calc_time);
            self.statistics.push(*thread_time.lock().unwrap(), DebugStatType::ThreadModifier);

            // Check if saving is available 
            {
                if self.file_selector.total_errored != 0 || self.file_selected_total == 0 || self.modifications_total == 0 {
                    self.save_available = false;
                } else { self.save_available = true };
            }

            // Check if any popups are open and disable ui elements
            if self.is_popup_open() {
                self.hide_all_elements();
            } else {
                self.show_all_elements();
            };

            // Set PPP (GUI Scale)
            if self.options.gui_scale_dragging == false {
                ctx.set_pixels_per_point(self.options.gui_scale);
            };

            // Set Theme
            match self.options.general.theme {
                Theme::Dark => {
                    ctx.set_visuals(egui::Visuals::dark());
                },
                Theme::Light => {
                    ctx.set_visuals(egui::Visuals::light());
                }
            };
        }

        // Panels 
        {
            // Top Bar
            bar_top::bar(self, ctx);

            // Bottom Bar
            bar_bottom::bar(self, ctx);
            
            // Start File Processing Thread
            if self.reset_processing == true {
                self.reset_processing = false;
                //self.file_browser = structs::FileBrowser::default();
                self.file_selector = file_selector::FileSelection::default();
                self.modifier_thread_storage = ModifierThreadStorage {
                    kill_sig_string_processor: Arc::new(Mutex::new(false)),
                    modifiers: Arc::new(Mutex::new(None)),
                    modifier_order: Arc::new(Mutex::new(None)),
                    eddited_files: Arc::new(Mutex::new(None)),
                    raw_files: Arc::new(Mutex::new(None)),
                    errors: Arc::new(Mutex::new(None)),
                    state: Arc::new(Mutex::new(ThreadState::None)),
                    thread_calc_time: Arc::new(Mutex::new(0))
                };
                self.first_frame = true;
                self.no_refresh = true;
                return;
            };

            // Center
            egui::CentralPanel::default()
            .show(ctx, |ui| {
                let inner_size = ui.available_size();
                let section_browser_width = ((800.0 * self.section_browser_percentage).floor() - 5.0) as u32;
                let section_options_width = ((800.0 * self.section_options_percentage).floor() - 5.0) as u32;
                let section_selector_width = (inner_size.x - 65.0) as u32 - (section_browser_width + section_options_width);

                ui.horizontal(|ui| { ui.group(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                        // Modifiers
                        ui.group(|ui| {
                            // Set UI Group Size
                            ui.set_height(inner_size.y - 24.0);
                            ui.set_max_height(inner_size.y - 24.0);
                            ui.set_width(section_options_width as f32);
                            ui.set_max_width(section_options_width as f32);

                            file_modifications::modifications(self, ui, ctx); // Fill Modifiers section
                        });
                        // File Selector
                        ui.group(|ui| {
                            if self.file_browser.collapsed == true {
                                // Set UI Group Size
                                ui.set_height(inner_size.y - 24.0);
                                ui.set_width((section_selector_width + section_browser_width) as f32 + 20.0);
                                ui.set_max_width(section_selector_width as f32);
                            } else {
                                // Set UI Group Size
                                ui.set_height(inner_size.y - 24.0);
                                ui.set_width(section_selector_width as f32);
                                ui.set_max_width(section_selector_width as f32);
                            }

                            file_selector::selector(self, ui, ctx); // Fill File Selector section
                        });
                        // File Browser
                        if self.file_browser.collapsed == false {
                            ui.group(|ui| {
                                // Set UI Group Size
                                ui.set_height(inner_size.y - 24.0);
                                ui.set_width(section_browser_width as f32);
                                ui.set_max_width(section_browser_width as f32);
                                file_browser::browser(self, ui, ctx); // Fill File Browser section
                            });
                        };
                    });
                })});
            });
        }

        // End Frame
        {
            let mods = Arc::clone(&self.modifier_thread_storage.modifiers);
            if mods.lock().unwrap().is_none() {
                mods.lock().unwrap().replace(self.modifiers.clone());
            };
    
            let mod_order = Arc::clone(&self.modifier_thread_storage.modifier_order);
            if mod_order.lock().unwrap().is_none() {
                mod_order.lock().unwrap().replace(self.options.modifier_order.clone());
            };
    
            let files = Arc::clone(&self.modifier_thread_storage.raw_files);
            if files.lock().unwrap().is_none() {
                files.lock().unwrap().replace(self.create_selected_vec());
            };
    
            let hashes = Arc::clone(&self.thread_storage.hashes);
            if hashes.lock().unwrap().len() != 0 {
                self.save(Some(hashes.lock().unwrap().to_owned()));
                *hashes.lock().unwrap() = vec![];
            };
    
            // Reset scroll for modifiers
            if self.modifiers.drag_box_hovered {
                self.modifiers.drag_box_hovered = false;
                self.modifiers.scroll_allowed = false;
            } else {
                self.modifiers.scroll_allowed = true;
            };
    
            // Update the clock
            self.local_time = chrono::Local::now();

            // Frame CPU usage
            if frame.info().cpu_usage.is_some() {
                self.statistics.push(
                    (frame.info().cpu_usage.unwrap() * 1000.0) as u32, 
                    DebugStatType::FrameTotal
                );
            };

            ctx.request_repaint();
        }
    }
}