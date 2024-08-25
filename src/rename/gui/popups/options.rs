use super::super::super::super::config;
use super::super::super::util::contextmenu;
use super::super::super::app::{OptionsList, WindowMain, Theme};

pub fn window(gui: &mut WindowMain, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            // Left Nav Bar
            ui.vertical(|ui| {
                ui.set_width(125.0 + 5.0);
                if ui.add_sized(
                    egui::vec2(125.0, ui.available_height() + 5.0), 
                    egui::widgets::SelectableLabel::new(gui.options.general_selected, "General")
                ).clicked() {
                    gui.options.sub_section_selected = OptionsList::General;
                    if someone_selected(gui) {
                        deselect_all(gui);
                    };
                    gui.options.general_selected = true;
                }
                ui.separator();
                if ui.add_sized(
                    egui::vec2(125.0, ui.available_height() + 5.0), 
                    egui::widgets::SelectableLabel::new(gui.options.appearance_selected, "Appearance")
                ).clicked() {
                    gui.options.sub_section_selected = OptionsList::Appearance;
                    if someone_selected(gui) {
                        deselect_all(gui);
                    };
                    gui.options.appearance_selected = true;
                }
                ui.separator();
                if ui.add_sized(
                    egui::vec2(125.0, ui.available_height() + 5.0), 
                    egui::widgets::SelectableLabel::new(gui.options.file_browser_selected, "File Browser")
                ).clicked() {
                    gui.options.sub_section_selected = OptionsList::FileBrowser;
                    if someone_selected(gui) {
                        deselect_all(gui);
                    };
                    gui.options.file_browser_selected = true;
                }
                ui.separator();
                if ui.add_sized(
                    egui::vec2(125.0, ui.available_height() + 5.0), 
                    egui::widgets::SelectableLabel::new(gui.options.file_selection_selected, "File Picker")
                ).clicked() {
                    gui.options.sub_section_selected = OptionsList::FileSelector;
                    if someone_selected(gui) {
                        deselect_all(gui);
                    };
                    gui.options.file_selection_selected = true;
                }
                ui.separator();
                if ui.add_sized(
                    egui::vec2(125.0, ui.available_height() + 5.0), 
                    egui::widgets::SelectableLabel::new(gui.options.saving_selected, "Saving")
                ).clicked() {
                    gui.options.sub_section_selected = OptionsList::Saving;
                    if someone_selected(gui) {
                        deselect_all(gui);
                    };
                    gui.options.saving_selected = true;
                }
            });

            ui.separator();
            
            // Option Varient Section
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.set_height(ui.available_height());
                    ui.set_width(375.0);
                    // Implement each view for each catagory
                    match gui.options.sub_section_selected {
                        OptionsList::General => {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    #[cfg(target_os = "windows")]
                                    {
                                        ui.label("Windows File Explorer context menu");
                                        if ui.checkbox(&mut gui.options.windows_context_menu_installed, "").clicked() {
                                            if gui.options.windows_context_menu_installed == false {
                                                contextmenu::uninstall_registry();
                                            } else {
                                                contextmenu::install_registry(gui.path_executable.clone())
                                            }
                                        }
                                    }
                                });
                            });
                        },
                        OptionsList::Appearance => {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Theme");
                                    egui::ComboBox::new("General_Theme", "")
                                    .selected_text(gui.options.appearance.theme_name.to_owned())
                                    .show_ui(ui, |ui| {
                                        if ui.selectable_label(false, "Dark").clicked() {
                                            gui.options.appearance.theme = Theme::Dark;
                                            gui.options.appearance.theme_name = String::from("Dark");
                                        };
                                        if ui.selectable_label(false, "Light").clicked() {
                                            gui.options.appearance.theme = Theme::Light;
                                            gui.options.appearance.theme_name = String::from("Light");
                                        };
                                    });
                                    ui.label("UI Scale");
                                    let slider = ui.add(
                                        egui::Slider::new(&mut gui.options.gui_scale, std::ops::RangeInclusive::new(0.5, 1.5))
                                        .fixed_decimals(2)
                                        .clamp_to_range(true)
                                        .show_value(true)
                                        .logarithmic(false)
                                        .step_by(0.05)
                                    );
                                    if slider.dragged() {
                                        gui.options.gui_scale_dragging = true;
                                    } else {
                                        gui.options.gui_scale_dragging = false;
                                    }
                                    
                                });
                                ui.horizontal(|_ui| {
                                    
                                });
                            });
                        },
                        OptionsList::FileBrowser => {
                            ui.checkbox(&mut gui.options.file_browser.multi_select, "Multiple-Selections")
                            .on_hover_text("Allows for more than one folder to be added to the selection area at a time.");
                        },
                        OptionsList::FileSelector => {
                            ui.checkbox(&mut gui.options.file_selection.stripped_column, "Stripped Columns");
                            ui.checkbox(&mut gui.options.file_selection.always_show_extra_row, "Preview Row Always shown");
                            ui.checkbox(&mut gui.options.file_selection.list_folders, "Show folders");
                        },
                        OptionsList::FileModifiers => {
        
                        },
                        OptionsList::Saving => {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("IO Operation wait time");
                                    let slider = ui.add(
                                    egui::Slider::new(&mut gui.options.saving.io_operation_waittime, std::ops::RangeInclusive::new(0, 5))
                                    .fixed_decimals(0)
                                    .clamp_to_range(true)
                                    .show_value(true)
                                    .logarithmic(false)
                                    .step_by(1.0)
                                    );
                                    ui.label("ms");
                                });
                            });
                        },
                        OptionsList::Presets => {
        
                        }
                    }
                });
            });
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                if ui.button("Save").clicked() {
                    config::write_config(gui.options.clone()).unwrap();
                }
            });
        });
    });
}

fn someone_selected(gui: &mut WindowMain) -> bool {
    let mut is_selected: bool = false;
    if gui.options.general_selected { is_selected = true };
    if gui.options.appearance_selected { is_selected = true };
    if gui.options.file_browser_selected { is_selected = true };
    if gui.options.file_selection_selected { is_selected = true };
    if gui.options.file_modifiers_selected { is_selected = true };
    if gui.options.saving_selected { is_selected = true };
    if gui.options.preset_selected { is_selected = true };
    return is_selected;
}

fn deselect_all(gui: &mut WindowMain) {
    gui.options.general_selected = false;
    gui.options.appearance_selected = false;
    gui.options.file_browser_selected = false;
    gui.options.file_selection_selected = false;
    gui.options.file_modifiers_selected = false;
    gui.options.saving_selected = false;
    gui.options.preset_selected = false;
}