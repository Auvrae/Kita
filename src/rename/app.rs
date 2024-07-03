use super::mods::{Modifiers, ModsOrder};
use super::util::{dir, threads::{ThreadState, ModifierThreadError, ModifierThreadStorage, ThreadFunction, ThreadStorage, Endianness, thread}};
use super::util::processing::file_processing::process;
use super::gui::main_sub::file_browser::{FileBrowser, BrowserItem};
use super::gui::main_sub::file_selector::FileSelection;
use super::debug::DebugStats;

use utils;
use std::sync;

// Main Window
pub struct WindowMain {
    pub window_size: egui::Vec2,
    pub cpu_usage: f32,
    pub operating_system: String,
    pub cli_args: Vec<String>,
    pub first_frame: bool,
    pub no_refresh: bool,
    pub fonts: Vec<String>,
    pub input_state: Option<egui::InputState>,
    pub options: Options,
    pub local_time: chrono::DateTime<chrono::Local>,
    pub reset_processing: bool,
    pub reset_ui: bool,
    pub save_available: bool,

    pub bar_top_height: f32,
    pub bar_bottom_height: f32,

    pub section_browser_percentage: f32,
    pub section_options_percentage: f32,
    pub section_browser_enabled: bool,
    pub section_selector_enabled: bool,
    pub section_modifiers_enabled: bool,

    pub thread_dump_clock: std::time::Instant,

    pub popups: GuiPopUps,

    pub file_mounts: Vec<String>,
    pub file_mounts_selected: u8,
    pub file_browser: FileBrowser,

    pub file_selector: FileSelection,
    pub file_selected_total: u32,

    pub modifiers_reorder_enabled: bool,
    pub modifiers_dnd_enabled: bool,
    pub modifier_order: Vec<ModsOrder>,
    pub modifications_total: u32,
    pub modifiers: Modifiers,

    pub modifier_thread_storage: ModifierThreadStorage,
    pub thread_storage: ThreadStorage,

    pub statistics: DebugStats
}

impl Default for WindowMain {
    fn default() -> Self {
        Self {
            window_size: egui::Vec2::new(0.0, 0.0),
            cpu_usage: f32::default(),
            operating_system: String::new(),
            cli_args: vec![],
            first_frame: true,
            no_refresh: false,
            fonts: vec![],
            input_state: None,
            options: Options::default(),
            local_time: chrono::Local::now(),
            reset_processing: false,
            reset_ui: false,
            save_available: false,


            bar_top_height: 28.0,
            bar_bottom_height: 28.0,

            section_browser_percentage: 0.30,
            section_options_percentage: 0.47,
            section_browser_enabled: true,
            section_selector_enabled: true,
            section_modifiers_enabled: true,

            thread_dump_clock: std::time::Instant::now(),

            popups: GuiPopUps {
                options: false,
                options_id: egui::Id::from(utils::create_random_string(8)),
                save_confirmation: false,
                save_confirmation_id: egui::Id::from(utils::create_random_string(8)),
                saving: false,
                saving_id: egui::Id::from(utils::create_random_string(8)),
                hashing: false,
                hashing_id: egui::Id::from(utils::create_random_string(8)),
                error: false,
                error_id: egui::Id::from(utils::create_random_string(8)),
                preset_manager: false,
                preset_manager_id: egui::Id::from(utils::create_random_string(8)),
                quit: false,
                quit_id: egui::Id::from(utils::create_random_string(8)),
                debug: false,
                debug_id: egui::Id::from(utils::create_random_string(8)),
                debug_plot_id: egui::Id::from(utils::create_random_string(8))
            },

            file_mounts: vec![],
            file_mounts_selected: 0,
            file_browser: FileBrowser {
                allow_frame: true,
                files_werent_modified: true,
                collapsed: false,
                ..Default::default()
            },
            
            file_selector: FileSelection {
                allow_frame: true,
                ..Default::default()
            },
            file_selected_total: 0,

            modifiers_reorder_enabled: false,
            modifiers_dnd_enabled: true,
            modifier_order: vec![
                ModsOrder::Case,
                ModsOrder::Name,
                ModsOrder::Regex,
                ModsOrder::Remove,
                ModsOrder::MoveCopy,
                ModsOrder::Replace,
                ModsOrder::Add,
                ModsOrder::Date,
                ModsOrder::Number,
                ModsOrder::Ext,
                ModsOrder::Hash
            ],
            modifications_total: 0,
            modifiers: Modifiers::default(),

            modifier_thread_storage: ModifierThreadStorage {
                kill_sig_string_processor: sync::Arc::new(sync::Mutex::new(false)),
                modifiers: sync::Arc::new(sync::Mutex::new(None)),
                modifier_order: sync::Arc::new(sync::Mutex::new(None)),
                eddited_files: sync::Arc::new(sync::Mutex::new(None)),
                raw_files: sync::Arc::new(sync::Mutex::new(None)),
                errors: sync::Arc::new(sync::Mutex::new(None)),
                state: sync::Arc::new(sync::Mutex::new(ThreadState::None)),
                thread_calc_time: sync::Arc::new(sync::Mutex::new(0))
            },
            thread_storage: ThreadStorage::default(),

            statistics: DebugStats {
                frame_total_calc_time: vec![],
                gui_browser_calc_time: vec![],
                gui_modifier_calc_time: vec![],
                gui_selector_calc_time: vec![],
                thread_modifier_calc_time: vec![]
            }
        }
    }
}

impl WindowMain {
    pub fn is_popup_open(&self) -> bool {
        let pops = self.popups.clone();
        let mut open: bool = false;

        if pops.error { open = true };
        if pops.options { open = true };
        if pops.preset_manager { open = true };
        if pops.saving { open = true };
        if pops.save_confirmation { open = true };
        if pops.hashing { open = true };
        if pops.quit { open = true };

        open
    }

    pub fn show_all_elements(&mut self) {
        self.section_browser_enabled = true;
        self.section_modifiers_enabled = true;
        self.section_selector_enabled = true;
    }

    pub fn hide_all_elements(&mut self) {
        self.section_browser_enabled = false;
        self.section_modifiers_enabled = false;
        self.section_selector_enabled = false;
    }
    
    pub fn read_directory(&mut self, path: String, root: bool, depth: Vec<u32>) -> Vec<BrowserItem> {
        let mut items: Vec<BrowserItem> = vec![];
        match dir::get_folder(path.to_owned(), false) {
            Ok(folder) => {
                for index in 0..folder.list_folders.len() {
                    let mut digging = depth.clone();
                    digging.push(index as u32);
                    let item = &folder.list_folders[index];
                    items.push(BrowserItem {
                        title: item.name.to_owned(),
                        id: egui::Id::from(utils::create_random_string(8)),
                        path: path.to_owned(),
                        entered: false,
                        selected: false,
                        root: root.to_owned(),
                        children: vec![],
                        children_ids: vec![],
                        depth: digging
                    });
                };
            },
            Err(error) => {
                println!("{:?}", error);
            }
        };
        return items;
    }

    pub fn create_selected_vec(&mut self) -> Vec<(Vec<(String, usize, Option<String>)>, Vec<(String, usize, Option<String>)>)> {
        let mut selected: Vec<(Vec<(String, usize, Option<String>)>, Vec<(String, usize, Option<String>)>)> = vec![];
        let mut selected_folder_paths: Vec<(String, usize, usize)> = vec![];
        let mut selected_file_paths: Vec<(String, usize, usize)> = vec![];
        let mut selected_total: u32 = 0;
        for (index, folder) in self.file_selector.folders.clone().iter().enumerate() {
            let mut fold_selected: Vec<(String, usize, Option<String>)> = vec![];
            let mut file_selected: Vec<(String, usize, Option<String>)> = vec![];
            for (fold_index, fold) in folder.selected_folders.iter().enumerate() {
                if *fold == true {
                    fold_selected.push((folder.list_folders[fold_index].clone().name, fold_index, None));
                    selected_folder_paths.push((folder.list_folders[fold_index].path.clone(), index, fold_index));
                    selected_total += 1;
                }
            };

            for (file_index, file) in folder.selected_files.iter().enumerate() {
                if *file == true {
                    if folder.list_files[file_index].hash.len() != 0 {
                        file_selected.push((folder.list_files[file_index].clone().name, file_index, Some(folder.list_files[file_index].hash.clone())));
                        selected_file_paths.push((folder.list_files[file_index].path.clone(), index, file_index));
                        selected_total += 1;
                    } else {
                        file_selected.push((folder.list_files[file_index].clone().name, file_index, None));
                        selected_file_paths.push((folder.list_files[file_index].path.clone(), index, file_index));
                        selected_total += 1;
                    }
                };
            };
            selected.insert(index, (fold_selected, file_selected))
        };
        self.file_selected_total = selected_total;
        self.file_selector.selected_folder_paths = selected_folder_paths;
        self.file_selector.selected_file_paths = selected_file_paths;
        return selected;
    }

    pub fn fill_selected_renamed(&mut self, renamed: Vec<(Vec<(String, usize, Option<String>)>, Vec<(String, usize, Option<String>)>)>,  errors: Vec<(Vec<ModifierThreadError>, Vec<ModifierThreadError>)>) {
        self.file_browser.files_werent_modified = false;
        if renamed.len() != self.file_selector.folders.len() {
            return;
        };
        for (index, folder) in renamed.iter().enumerate() {
            for (_, fold) in folder.0.iter().enumerate() {
                self.file_selector.folders[index].list_folders[fold.1].name_modified = fold.0.to_owned();
            }
            for (_, file) in folder.1.iter().enumerate() {
                self.file_selector.folders[index].list_files[file.1].name_modified = file.0.to_owned();
            }
        };
        for(_, folders) in self.file_selector.folders.iter_mut().enumerate() {
            for (_, fold) in folders.list_folders.iter_mut().enumerate() {
                fold.error = String::new();
                fold.errored = false;
            }
        };
        for(_, folders) in self.file_selector.folders.iter_mut().enumerate() {
            for (_, file) in folders.list_files.iter_mut().enumerate() {
                file.error = String::new();
                file.errored = false;
            }
        };
        for (index, errors) in errors.iter().enumerate() {
            for (_, error) in errors.0.iter().enumerate() {
                match error {
                    ModifierThreadError::DuplicateFileName(duplicates) => {
                        for err in duplicates {
                            self.file_selector.folders[index].list_folders[*err].errored = true;
                            self.file_selector.folders[index].list_folders[*err].error = String::from("Duplicate name!");
                        }
                    },
                    ModifierThreadError::LengthLimitFileName(file_length) => {
                        for err in file_length {
                            self.file_selector.folders[index].list_folders[err.0.to_owned()].errored = true;
                            self.file_selector.folders[index].list_folders[err.0.to_owned()].error = 
                                String::from(format!("File length {} which is greater then the limit [255]", err.1));
                        }
                    },
                    ModifierThreadError::InvalidChar(invalidchars) => {
                        for err in invalidchars {
                            self.file_selector.folders[index].list_folders[err.0.to_owned()].errored = true;
                            self.file_selector.folders[index].list_folders[err.0.to_owned()].error = 
                                String::from(format!("Character {} invalid for names on this operating system!", err.1));
                        }
                    },
                    ModifierThreadError::InvalidFileName(invalidname) => {
                        for err in invalidname {
                            self.file_selector.folders[index].list_folders[err.0.to_owned()].errored = true;
                            self.file_selector.folders[index].list_folders[err.0.to_owned()].error = 
                                String::from(format!("File name is invalid, system reserved file name! - {}", err.1));
                        }
                    }
                };
            };
            for (_, error) in errors.1.iter().enumerate() {
                match error {
                    ModifierThreadError::DuplicateFileName(duplicates) => {
                        for err in duplicates {
                            self.file_selector.folders[index].list_files[*err].errored = true;
                            self.file_selector.folders[index].list_files[*err].error = String::from("Duplicate name!");
                        }
                    },
                    ModifierThreadError::LengthLimitFileName(file_length) => {
                        for err in file_length {
                            self.file_selector.folders[index].list_files[err.0.to_owned()].errored = true;
                            self.file_selector.folders[index].list_files[err.0.to_owned()].error = 
                                String::from(format!("File length {} which is greater then the limit [255]", err.1));
                        }
                    },
                    ModifierThreadError::InvalidChar(invalidchars) => {
                        for err in invalidchars {
                            self.file_selector.folders[index].list_files[err.0.to_owned()].errored = true;
                            self.file_selector.folders[index].list_files[err.0.to_owned()].error = 
                                String::from(format!("Character {} invalid for names on this operating system!", err.1));
                        }
                    },
                    ModifierThreadError::InvalidFileName(invalidname) => {
                        for err in invalidname {
                            self.file_selector.folders[index].list_files[err.0.to_owned()].errored = true;
                            self.file_selector.folders[index].list_files[err.0.to_owned()].error = 
                                String::from(format!("File name is invalid, system reserved file name! - {}", err.1));
                        }
                    }
                };
            };
        };
    }

    pub fn hash(&mut self) {
        *self.thread_storage.progress.lock().unwrap() = 0.00;
        thread(self, ThreadFunction::Hash(self.modifiers.hash.algorithm, self.file_selector.selected_file_paths.clone(), Endianness::BigEndian));
        self.popups.hashing = true;
    }

    pub fn save(&mut self, hashes: Option<Vec<(String, usize, usize)>>) {
        let mut edit: dir::Edit = dir::Edit {
            tag: String::new(),
            items: vec![],
            edits: self.modifications_total
        };
        if hashes.is_some() {
            let hashes = hashes.unwrap();
            for (_, hash) in hashes.iter().enumerate() {
                self.file_selector.folders[hash.1].list_files[hash.2].hash = hash.0.clone();
            };

        };
        *self.modifier_thread_storage.kill_sig_string_processor.lock().unwrap() = true;
        while *self.modifier_thread_storage.state.lock().unwrap() != ThreadState::Dead {}

        // Refresh all files and folders once before continuing
        let proto_files = self.create_selected_vec();
        let mut completed_edits: Vec<(Vec<(String, usize, Option<String>)>, Vec<(String, usize, Option<String>)>)> = vec![];
        let mut completed_errors: Vec<(Vec<ModifierThreadError>, Vec<ModifierThreadError>)> = vec![];
        for (index, (folders, files)) in proto_files.iter().enumerate() {
            let folders_edits = process(index, &mut self.modifiers, folders.to_owned(), self.modifier_order.clone(), true);
            let files_edits = process(index, &mut self.modifiers, files.to_owned(), self.modifier_order.clone(), false);
            completed_edits.push((folders_edits.0, files_edits.0));
            completed_errors.push((folders_edits.1, files_edits.1));
        };

        self.fill_selected_renamed(completed_edits, completed_errors);

        for (folder_index, folder) in self.file_selector.folders.iter().enumerate() {
            for (fold_index, _) in folder.selected_folders.iter().enumerate() {
                edit.items.push(dir::EdittedItem {
                    name_original: self.file_selector.folders[folder_index].list_folders[fold_index].name.to_owned(),
                    name_edited: self.file_selector.folders[folder_index].list_folders[fold_index].name_modified.to_owned(),
                    path_original: self.file_selector.folders[folder_index].list_folders[fold_index].path.to_owned(),
                    path_edited: format!("{}/{}", self.file_selector.folders[folder_index].list_folders[fold_index].path_plain, 
                        self.file_selector.folders[folder_index].list_folders[fold_index].name_modified)
                });
            };
            for (file_index, _) in folder.list_files.iter().enumerate() {
                edit.items.push(dir::EdittedItem {
                    name_original: self.file_selector.folders[folder_index].list_files[file_index].name.to_owned(),
                    name_edited: self.file_selector.folders[folder_index].list_files[file_index].name_modified.to_owned(),
                    path_original: self.file_selector.folders[folder_index].list_files[file_index].path.to_owned(),
                    path_edited: format!("{}/{}", self.file_selector.folders[folder_index].list_files[file_index].path_plain, 
                        self.file_selector.folders[folder_index].list_files[file_index].name_modified)
                });
            };
        };
        
        self.popups.saving = true;
        *self.thread_storage.progress.lock().unwrap() = 0.00;
        thread(self, ThreadFunction::SaveUndoRedo(edit));
    }

    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DndDropLocation {
    pub row: usize
}

#[derive(Debug, Clone)]
pub struct GuiPopUps{
    pub options: bool,
    pub options_id: egui::Id,
    pub save_confirmation: bool,
    pub save_confirmation_id: egui::Id,
    pub saving: bool,
    pub saving_id: egui::Id,
    pub hashing: bool,
    pub hashing_id: egui::Id,
    pub error: bool,
    pub error_id: egui::Id,
    pub preset_manager: bool,
    pub preset_manager_id: egui::Id,
    pub quit: bool,
    pub quit_id: egui::Id,
    pub debug: bool,
    pub debug_id: egui::Id,
    pub debug_plot_id: egui::Id
}

#[derive(Clone)]
pub struct Options {
    pub sub_section_selected: OptionsList,
    pub bar_size: f32,
    pub gui_scale: f32,
    pub gui_scale_dragging: bool,
    pub context_options: Option<egui::Options>,
    
    pub general: OptionsGeneral,
    pub general_selected: bool,
    pub appearance: OptionsAppearance,
    pub appearance_selected: bool,
    pub file_browser: OptionsFileBrowser,
    pub file_browser_selected: bool,
    pub file_selection: OptionsFileSelection,
    pub file_selection_selected: bool,
    pub file_modifiers: OptionsFileModifiers,
    pub file_modifiers_selected: bool,
    pub saving: OptionsSaving,
    pub saving_selected: bool,
    pub preview: OptionsPreview,
    pub preview_selected: bool,
    pub error: OptionsError,
    pub error_selected: bool,
    pub preset: OptionsPresets,
    pub preset_selected: bool
}

impl Default for Options {
    fn default() -> Self {
        Self {
            sub_section_selected: OptionsList::General,
            bar_size: 125.0,
            gui_scale: 0.0,
            gui_scale_dragging: false,
            context_options: None,

            general: OptionsGeneral {
                theme: Theme::Dark,
                theme_name: String::from("Dark")
            },
            general_selected: true,
            appearance: OptionsAppearance {
                ..Default::default()
            },
            appearance_selected: false,
            file_browser: OptionsFileBrowser {
                multi_select: true,
                ..Default::default()
            },
            file_browser_selected: false,
            file_selection: OptionsFileSelection {
                stripped_column: true,
                list_folders: true,
                ..Default::default()
            },
            file_selection_selected: false,
            file_modifiers: OptionsFileModifiers {
                sub_modifier_maximum: 5,
                ..Default::default()
            },
            file_modifiers_selected: false,
            saving: OptionsSaving {
                ..Default::default()
            },
            saving_selected: false,
            preview: OptionsPreview {
                ..Default::default()
            },
            preview_selected: false,
            error: OptionsError {
                ..Default::default()
            },
            error_selected: false,
            preset: OptionsPresets {
                ..Default::default()
            },
            preset_selected: false
        }
    }
}

#[derive(Clone)]
pub struct OptionsGeneral {
    pub theme: Theme,
    pub theme_name: String
}

#[derive(Default, Clone)]
pub struct OptionsAppearance {

}

#[derive(Default, Clone)]
pub struct OptionsFileBrowser {
    pub multi_select: bool
}

#[derive(Default, Clone)]
pub struct OptionsFileSelection {
    pub stripped_column: bool,
    pub list_folders: bool,
    pub always_show_extra_row: bool
}

#[derive(Default, Clone)]
pub struct OptionsFileModifiers {
    pub sub_modifier_maximum: u8,
    pub modifiers_enabled: Vec<ModsOrder>
}

#[derive(Default, Clone)]
pub struct OptionsSaving {

}

#[derive(Default, Clone)]
pub struct OptionsPreview {

}

#[derive(Default, Clone)]
pub struct OptionsError {

}

#[derive(Default, Clone)]
pub struct OptionsPresets {

}

#[derive(Default, Clone)]
pub struct OptionsExperimental {

}

#[derive(Clone, Debug)]
pub enum OptionsList {
    General,
    Appearance,
    FileBrowser,
    FileSelector,
    FileModifiers,
    Saving,
    Preview,
    Presets
}

#[derive(Clone, Debug)]
pub enum Theme {
    Dark,
    Light
}

