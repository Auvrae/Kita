use super::mods::{Modifiers, ModsOrder};
use super::presets::Presets;
use super::util::{dir, threads::{ThreadState, ModifierThreadError, ModifierThreadStorage, 
    ThreadFunction, ThreadStorage, Endianness, thread, SaveType}, processing::file_processing::process};
use super::gui::main_sub::file_browser::{FileBrowser, MapFolder};
use super::gui::main_sub::file_selector::FileSelection;
use super::app;
use super::debug::DebugStats;

use utils;
use std::borrow::{Borrow, BorrowMut};
use std::collections::BTreeMap;
use std::sync;
use serde::{Deserialize, Serialize};

// Main Window
pub struct WindowMain {
    pub path_executable: String,
    pub window_size: egui::Vec2,
    pub cpu_usage: f32,
    pub operating_system: String,
    pub first_frame: bool,
    pub no_refresh: bool,
    pub fonts: Vec<String>,
    pub input_state: Option<egui::InputState>,
    pub options: Options,
    pub presets: Presets,
    pub local_time: chrono::DateTime<chrono::Local>,
    pub reset_processing: bool,
    pub reset_ui: bool,
    pub save_available: bool,

    pub edits: Edits,

    pub bar_top_height: f32,
    pub bar_top_enabled: bool,
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
    pub modifications_total: u32,
    pub modifiers: Modifiers,

    pub modifier_thread_storage: ModifierThreadStorage,
    pub thread_storage: ThreadStorage,

    pub statistics: DebugStats
}

impl Default for WindowMain {
    fn default() -> Self {
        Self {
            path_executable: String::new(),
            window_size: egui::Vec2::new(0.0, 0.0),
            cpu_usage: f32::default(),
            operating_system: String::new(),
            first_frame: true,
            no_refresh: false,
            fonts: vec![],
            input_state: None,
            options: Options::default(),
            presets: Presets::default(),
            local_time: chrono::Local::now(),
            reset_processing: false,
            reset_ui: false,
            save_available: false,

            edits: Edits{
                redo: None,
                undo: None
            },

            bar_top_height: 28.0,
            bar_top_enabled: true,
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
                save_as_preset: false,
                save_as_preset_id: egui::Id::from(utils::create_random_string(8)),
                save_as_preset_field: String::new(),
                save_as_preset_field_name: String::new(),
                about: false,
                about_id: egui::Id::from(utils::create_random_string(8)),
                debug: false,
                debug_id: egui::Id::from(utils::create_random_string(8)),
                debug_plot_id: egui::Id::from(utils::create_random_string(8))
            },

            file_mounts: vec![],
            file_mounts_selected: 0,
            file_browser: FileBrowser::default(),

            file_selector: FileSelection {
                allow_frame: true,
                ..Default::default()
            },
            file_selected_total: 0,

            modifiers_reorder_enabled: false,
            modifiers_dnd_enabled: true,
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
        if pops.save_as_preset { open = true };

        open
    }

    pub fn show_all_elements(&mut self) {
        self.section_browser_enabled = true;
        self.section_modifiers_enabled = true;
        self.section_selector_enabled = true;
        self.bar_top_enabled = true;
    }

    pub fn hide_all_elements(&mut self) {
        self.section_browser_enabled = false;
        self.section_modifiers_enabled = false;
        self.section_selector_enabled = false;
        self.bar_top_enabled = false;
    }
    
    pub fn read_directory(path: String, root: bool) -> Vec<MapFolder> {
        let mut items: Vec<MapFolder> = vec![];
        match dir::get_folder(path.to_owned(), false) {
            Ok(folder) => {
                for index in 0..folder.list_folders.len() {
                    let item = &folder.list_folders[index];
                    let full_path: String;
                    if root == true {
                        full_path = format!("{}{}", path.to_owned(), item.name.to_owned())
                    } else {
                        full_path = format!("{}/{}", path.to_owned(), item.name.to_owned());
                    }
                    items.push(MapFolder {
                        title: item.name.to_owned(),
                        full_path: full_path,
                        parent: path.to_owned(),
                        root: root,
                        entered: false,
                        selected: false,
                        ctx_id: utils::create_random_string(8).into()
                    });
                };
            },
            Err(error) => {
                println!("{:?}", error);
            }
        };
        return items;
    }

    pub fn get_windows_drive_letters(&mut self) {
        #[cfg(target_os="windows")]
        {
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
                self.file_mounts.push(format!("{}/", drive_letter.to_owned()));
            };
            
            self.file_browser.root = "C:/".into();
            for mount in self.file_mounts.iter().enumerate() {
                if mount.1 == "C:/" {
                    self.file_mounts_selected = mount.0 as u8;
                };
            };
        }
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
        //Check and leave if something changed.
        if renamed.len() != self.file_selector.folders.len() {
            return;
        };
        for (index, folder) in renamed.iter().enumerate() {
            // Check and leave if something changed.
            {
                if folder.0.len() != self.file_selector.folders[index].list_folders.len() {
                    return;
                }
                if folder.1.len() != self.file_selector.folders[index].list_files.len() {
                    return;
                }
            }

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
            let folders_edits = process(index, &mut self.modifiers, folders.to_owned(), self.options.modifier_order.0.clone(), true);
            let files_edits = process(index, &mut self.modifiers, files.to_owned(), self.options.modifier_order.0.clone(), false);
            completed_edits.push((folders_edits.0, files_edits.0));
            completed_errors.push((folders_edits.1, files_edits.1));
        };

        self.fill_selected_renamed(completed_edits, completed_errors);

        for (folder_index, folder) in self.file_selector.folders.iter().enumerate() {
            for (fold_index, item) in folder.selected_folders.iter().enumerate() {
                if *item == true {
                    edit.items.push(dir::EdittedItem {
                        name_original: self.file_selector.folders[folder_index].list_folders[fold_index].name.to_owned(),
                        name_edited: self.file_selector.folders[folder_index].list_folders[fold_index].name_modified.to_owned(),
                        path_original: self.file_selector.folders[folder_index].list_folders[fold_index].path.to_owned(),
                        path_edited: format!("{}/{}", self.file_selector.folders[folder_index].list_folders[fold_index].path_plain, 
                            self.file_selector.folders[folder_index].list_folders[fold_index].name_modified)
                    });
                }
            };
            for (file_index, item) in folder.selected_files.iter().enumerate() {
                if *item == true {
                    edit.items.push(dir::EdittedItem {
                        name_original: self.file_selector.folders[folder_index].list_files[file_index].name.to_owned(),
                        name_edited: self.file_selector.folders[folder_index].list_files[file_index].name_modified.to_owned(),
                        path_original: self.file_selector.folders[folder_index].list_files[file_index].path.to_owned(),
                        path_edited: format!("{}/{}", self.file_selector.folders[folder_index].list_files[file_index].path_plain, 
                            self.file_selector.folders[folder_index].list_files[file_index].name_modified)
                    });
                }
            };
        };
        
        self.popups.saving = true;
        *self.thread_storage.progress.lock().unwrap() = 0.00;
        edit.tag = format!("{} files.", edit.items.len());
        self.edits.undo = Some(edit.clone());
        self.edits.redo = None;

        thread(self, ThreadFunction::SaveUndoRedo(edit, SaveType::Save, self.options.saving.io_operation_waittime));
        //self.file_browser.allow_frame = false;
        self.file_selector.allow_frame = false;
        self.modifiers.allow_frame = false;
    }

    pub fn undo(&mut self) {
        let edit = self.edits.undo.take().unwrap();
        self.popups.saving = true;
        *self.thread_storage.progress.lock().unwrap() = 0.00;
        self.edits.undo = None;
        self.edits.redo = Some(edit.clone());


        *self.modifier_thread_storage.kill_sig_string_processor.lock().unwrap() = true;
        while *self.modifier_thread_storage.state.lock().unwrap() != ThreadState::Dead {}

        thread(self, ThreadFunction::SaveUndoRedo(edit, SaveType::Undo, self.options.saving.io_operation_waittime));
        //self.file_browser.allow_frame = false;
        self.file_selector.allow_frame = false;
        self.modifiers.allow_frame = false;
    }

    pub fn redo(&mut self) {
        let edit = self.edits.redo.take().unwrap();
        self.popups.saving = true;
        *self.thread_storage.progress.lock().unwrap() = 0.00;
        self.edits.undo = Some(edit.clone());
        self.edits.redo = None;


        *self.modifier_thread_storage.kill_sig_string_processor.lock().unwrap() = true;
        while *self.modifier_thread_storage.state.lock().unwrap() != ThreadState::Dead {}

        thread(self, ThreadFunction::SaveUndoRedo(edit, SaveType::Redo, self.options.saving.io_operation_waittime));
        //self.file_browser.allow_frame = false;
        self.file_selector.allow_frame = false;
        self.modifiers.allow_frame = false;
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
    pub save_as_preset: bool,
    pub save_as_preset_id: egui::Id,
    pub save_as_preset_field: String,
    pub save_as_preset_field_name: String,
    pub about: bool,
    pub about_id: egui::Id,
    pub debug: bool,
    pub debug_id: egui::Id,
    pub debug_plot_id: egui::Id
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Options {
    #[serde(skip)]
    pub sub_section_selected: OptionsList,
    #[serde(default)]
    pub gui_scale: f32,
    #[serde(skip)]
    pub gui_scale_dragging: bool,
    #[serde(skip)]
    pub windows_context_menu_installed: bool,

    #[serde(default)]
    pub modifier_order: ModifierOrder,

    #[serde(default)]
    pub general: OptionsGeneral,
    #[serde(skip)]
    pub general_selected: bool,
    #[serde(default)]
    pub appearance: OptionsAppearance,
    #[serde(skip)]
    pub appearance_selected: bool,
    #[serde(default)]
    pub file_browser: OptionsFileBrowser,
    #[serde(skip)]
    pub file_browser_selected: bool,
    #[serde(default)]
    pub file_selection: OptionsFileSelection,
    #[serde(skip)]
    pub file_selection_selected: bool,
    #[serde(default)]
    pub file_modifiers: OptionsFileModifiers,
    #[serde(skip)]
    pub file_modifiers_selected: bool,
    #[serde(default)]
    pub saving: OptionsSaving,
    #[serde(skip)]
    pub saving_selected: bool,
    #[serde(default)]
    pub preset: OptionsPresets,
    #[serde(skip)]
    pub preset_selected: bool,

    #[serde(default)]
    pub recently_opened: Vec<String>,
    #[serde(skip)]
    pub recently_opened_menu_opened: Option<usize>
}

impl Default for Options {
    fn default() -> Self {
        Self {
            sub_section_selected: OptionsList::General,
            gui_scale: 0.0,
            gui_scale_dragging: false,
            windows_context_menu_installed: false,

            modifier_order: ModifierOrder {0: vec![
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
            ]},

            general: OptionsGeneral {

            },
            general_selected: true,
            appearance: OptionsAppearance {
                theme: Theme::Dark,
                theme_name: String::from("Dark")
            },
            appearance_selected: false,
            file_browser: OptionsFileBrowser {
                ..Default::default()
            },
            file_browser_selected: false,
            file_selection: OptionsFileSelection {
                stripped_column: true,
                ..Default::default()
            },
            file_selection_selected: false,
            file_modifiers: OptionsFileModifiers {
                sub_modifier_maximum: 3,
                ..Default::default()
            },
            file_modifiers_selected: false,
            saving: OptionsSaving {
                io_operation_waittime: 2,
                ..Default::default()
            },
            saving_selected: false,
            preset: OptionsPresets {
                ..Default::default()
            },
            preset_selected: false,

            recently_opened: vec![],
            recently_opened_menu_opened: None
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsGeneral {

}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsAppearance {
    #[serde(default)]
    pub theme: Theme,
    #[serde(default)]
    pub theme_name: String

}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsFileBrowser {
    #[serde(default)]
    pub multi_select: bool
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsFileSelection {
    #[serde(default)]
    pub stripped_column: bool,
    #[serde(default)]
    pub list_folders: bool,
    #[serde(default)]
    pub always_show_extra_row: bool
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsFileModifiers {
    #[serde(default)]
    pub sub_modifier_maximum: u8,
    #[serde(default)]
    pub modifiers_enabled: Vec<ModsOrder>
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsSaving {
    #[serde(default)]
    pub io_operation_waittime: u8
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsPresets {

}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionsExperimental {

}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum OptionsList {
    #[default]
    General,
    Appearance,
    FileBrowser,
    FileSelector,
    FileModifiers,
    Saving,
    Presets
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Dark,
    Light
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edits {
    pub redo: Option<dir::Edit>,
    pub undo: Option<dir::Edit>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModifierOrder(pub Vec<ModsOrder>);

impl Default for ModifierOrder {
    fn default() -> Self {
        Self { 0: vec![
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
        ]}
    }
}