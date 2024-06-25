use super::super::super::app::{OptionsList, WindowMain, Theme};

pub fn window(gui: &mut WindowMain, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.horizontal(|ui| {
        // Left Nav Bar
        ui.vertical(|ui| {
            ui.set_width(gui.options.bar_size + 5.0);
            if ui.add_sized(
                egui::vec2(gui.options.bar_size, ui.available_height() + 5.0), 
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
                egui::vec2(gui.options.bar_size, ui.available_height() + 5.0), 
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
                egui::vec2(gui.options.bar_size, ui.available_height() + 5.0), 
                egui::widgets::SelectableLabel::new(gui.options.file_browser_selected, "Browser Section")
            ).clicked() {
                gui.options.sub_section_selected = OptionsList::FileBrowser;
                if someone_selected(gui) {
                    deselect_all(gui);
                };
                gui.options.file_browser_selected = true;
            }
            ui.separator();
            if ui.add_sized(
                egui::vec2(gui.options.bar_size, ui.available_height() + 5.0), 
                egui::widgets::SelectableLabel::new(gui.options.file_selection_selected, "Selector Section")
            ).clicked() {
                gui.options.sub_section_selected = OptionsList::FileSelector;
                if someone_selected(gui) {
                    deselect_all(gui);
                };
                gui.options.file_selection_selected = true;
            }
            ui.separator();
            if ui.add_sized(
                egui::vec2(gui.options.bar_size, ui.available_height() + 5.0), 
                egui::widgets::SelectableLabel::new(gui.options.file_modifiers_selected, "Modifiers Section")
            ).clicked() {
                gui.options.sub_section_selected = OptionsList::FileModifiers;
                if someone_selected(gui) {
                    deselect_all(gui);
                };
                gui.options.file_modifiers_selected = true;
            }
            ui.separator();
            if ui.add_sized(
                egui::vec2(gui.options.bar_size, ui.available_height() + 5.0), 
                egui::widgets::SelectableLabel::new(gui.options.saving_selected, "Saving")
            ).clicked() {
                gui.options.sub_section_selected = OptionsList::Saving;
                if someone_selected(gui) {
                    deselect_all(gui);
                };
                gui.options.saving_selected = true;
            }
            ui.separator();
            if ui.add_sized(
                egui::vec2(gui.options.bar_size, ui.available_height() + 5.0),
                 egui::widgets::SelectableLabel::new(gui.options.preview_selected, "Preview")
            ).clicked() {
                gui.options.sub_section_selected = OptionsList::Preview;
                if someone_selected(gui) {
                    deselect_all(gui);
                };
                gui.options.preview_selected = true;
            }
            ui.separator();
            if ui.add_sized(
                egui::vec2(gui.options.bar_size, ui.available_height() + 5.0), 
                egui::widgets::SelectableLabel::new(gui.options.preset_selected, "Presets")
            ).clicked() {
                gui.options.sub_section_selected = OptionsList::Presets;
                if someone_selected(gui) {
                    deselect_all(gui);
                };
                gui.options.preset_selected = true;
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

                    },
                    OptionsList::Appearance => {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Theme");
                                egui::ComboBox::new("General_Theme", "")
                                .selected_text(gui.options.general.theme_name.to_owned())
                                .show_ui(ui, |ui| {
                                    if ui.selectable_label(false, "Dark").clicked() {
                                        gui.options.general.theme = Theme::Dark;
                                        gui.options.general.theme_name = String::from("Dark");
                                    };
                                    if ui.selectable_label(false, "Light").clicked() {
                                        gui.options.general.theme = Theme::Light;
                                        gui.options.general.theme_name = String::from("Light");
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
                        ui.checkbox(&mut gui.options.file_selection.always_show_extra_row, "---> Row Always shown");
                    },
                    OptionsList::FileModifiers => {
    
                    },
                    OptionsList::Saving => {
    
                    },
                    OptionsList::Preview => {
    
                    },
                    OptionsList::Presets => {
    
                    }
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
    if gui.options.preview_selected { is_selected = true };
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
    gui.options.preview_selected = false;
    gui.options.preset_selected = false;
}