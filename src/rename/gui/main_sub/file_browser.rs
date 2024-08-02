use super::super::super::app::WindowMain;
use super::super::super::debug::DebugStatType;
use super::super::main_sub::file_browser;

use std::collections::BTreeMap;
use std::time::Instant;
use std::fs;
use utils;

pub fn browser(gui: &mut WindowMain, ui: &mut egui::Ui, ctx: &egui::Context) {
    let start = Instant::now();
    ui.add_enabled_ui(gui.section_browser_enabled, |ui| {
        ui.vertical(|ui| {
            // Header
            ui.horizontal(|ui| {
                ui.strong("Browser");
                #[cfg(target_os="windows")] {
                    if !gui.file_mounts.is_empty() {
                        ui.separator();
                        ui.label("Mounts");
                        egui::ComboBox::new(format!("Browser-Mounts"), "")
                        .width(40.0)
                        .selected_text(gui.file_mounts[gui.file_mounts_selected.clone() as usize].clone())
                        .show_ui(ui, |ui| {
                            for (index, mount) in gui.file_mounts.clone().iter().enumerate() {
                                if ui.selectable_label(false, mount).clicked() {
                                    gui.file_mounts_selected = index as u8;
                                    gui.file_browser.folder_map.clear();
                                    gui.file_browser.selected_folders.clear();
                                    gui.file_browser.root = gui.file_mounts[index].clone();
                                    for folder in WindowMain::read_directory(String::from(gui.file_mounts[index].clone()), true) {
                                        gui.file_browser.folder_map.insert(folder.full_path.to_owned(), folder);
                                    }
                                }
                            };
                        });
                    }
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    let collapse_button = ui.button("⬅");
                    if collapse_button.clicked() {
                        gui.file_browser.collapsed = !gui.file_browser.collapsed;
                    }
                    if collapse_button.hovered() {
                        collapse_button.on_hover_text("Collapse file browser");
                    }

                    let button = ui.button("⟲");
                    if button.clicked() {
                        #[cfg(target_os="windows")] {
                            gui.file_browser.folder_map.clear();
                            gui.file_browser.selected_folders.clear();
                            gui.file_mounts.clear();
                            gui.get_windows_drive_letters();
                            for folder in WindowMain::read_directory(gui.file_mounts[gui.file_mounts_selected.to_owned() as usize].to_owned(), true) {
                                gui.file_browser.folder_map.insert(folder.full_path.to_owned(), folder);
                            };
                        }
                        #[cfg(target_os="linux")] {
                            gui.file_browser.folder_map.clear();
                            gui.file_browser.selected_folders.clear();
                            for folder in WindowMain::read_directory(gui.file_browser.root.clone(), true) {
                                gui.file_browser.folder_map.insert(folder.full_path.to_owned(), folder);
                            }
                        }
                    };
                    if button.hovered() {
                        button.on_hover_text("Reset browser to root");
                    }
                });
            });
            if gui.file_browser.allow_frame == false { return };
            
            ui.separator();
    
            // Body
            egui::ScrollArea::both()
            .id_source("Broswer")
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(ui.available_height());
                #[cfg(target_os="windows")] {
                    let children = gui.file_browser.get_children_of(gui.file_mounts[gui.file_mounts_selected as usize].to_owned()).unwrap();
                    for child in children {
                        fill_tree(gui, child, ui, ctx);
                    }
                };
                #[cfg(target_os="linux")] {
                    let children = gui.file_browser.get_children_of(String::from("/")).unwrap();
                    for child in children {
                        fill_tree(gui, child, ui, ctx);
                    }
                };
            });
        });
    });

    // Update Stats
    gui.statistics.push(start.elapsed().as_micros() as u32, DebugStatType::GuiBrowser);
}

fn fill_tree(gui: &mut WindowMain, parent: String, ui: &mut egui::Ui, ctx: &egui::Context) {
    let (path, mut root) = gui.file_browser.folder_map.remove_entry(&parent).unwrap();
    let mut header = egui::collapsing_header::CollapsingState::load_with_default_open(ctx, root.ctx_id, false);
    header.set_open(root.entered);
    let response = header.show_header(ui, |ui| {
        let title = ui.selectable_label(root.selected, root.title.clone());
        if root.selected && !gui.file_browser.selected_folders.contains(&root.full_path) {
            root.selected = false;
        };
        if title.clicked() && root.selected == false {
            //select_and_travel(gui, root, ctx, false);
            if gui.options.file_browser.multi_select == true {
                if root.root {
                    gui.file_browser.selected_folders.push(format!("{}{}", root.parent, root.title));
                } else {
                    gui.file_browser.selected_folders.push(format!("{}/{}", root.parent, root.title));
                };
            } else if gui.file_browser.selected_folders.len() >= 1 {
                    gui.file_browser.selected_folders.clear();
                    if root.root {
                        gui.file_browser.selected_folders.push(format!("{}{}", root.parent, root.title));
                    } else {
                        gui.file_browser.selected_folders.push(format!("{}/{}", root.parent, root.title));
                    };
            } else {
                if root.root {
                    gui.file_browser.selected_folders.push(format!("{}{}", root.parent, root.title));
                } else {
                    gui.file_browser.selected_folders.push(format!("{}/{}", root.parent, root.title));
                };
            }
            root.selected = true;
        } else if title.clicked() && root.selected == true {
            if gui.options.file_browser.multi_select == true {
                for (index, path) in gui.file_browser.selected_folders.clone().iter().enumerate() {
                    if *path == root.full_path {
                        gui.file_browser.selected_folders.remove(index);
                        break;
                    }
                }
            } else {
                gui.file_browser.selected_folders.clear();
            }
            root.selected = false;
        }
    });
    let (mut header_res, _body_res, _) = response.body(|ui| {
        let children = gui.file_browser.get_children_of(parent);
        if children.is_some() {
            let children = children.unwrap();
            for child in children {
                fill_tree(gui, child, ui, ctx);
            };
        };
    });
    if root.selected {
        header_res = header_res.highlight();
    };
    if header_res.clicked() {
        select_and_travel(gui, &mut root, ctx, true);
    };
    gui.file_browser.folder_map.insert(path, root);
}

fn select_and_travel(gui: &mut WindowMain, item: &mut MapFolder, ctx: &egui::Context, collapsing: bool) {
    if item.entered == false {
        if collapsing == true {
            item.entered = true;
        };
        if item.root {
            if collapsing == true {
                let children = WindowMain::read_directory(format!("{}{}", item.parent, item.title), false);
                for child in children {
                    gui.file_browser.folder_map.insert(child.full_path.to_owned(), child);
                }
            };
        } else {
            if collapsing == true{
                let children = WindowMain::read_directory(format!("{}/{}", item.parent, item.title), false);
                for child in children {
                    gui.file_browser.folder_map.insert(child.full_path.to_owned(), child);
                }
            };
        };
    } else {
        if collapsing == true {
            item.entered = false;
            close_children(gui, item, ctx);
        };
    };
}

fn close_children(gui: &mut WindowMain, item: &mut MapFolder, ctx: &egui::Context) {
    let children = gui.file_browser.get_children_of(item.full_path.to_owned());
    if children.is_some() {
        let children = children.unwrap();
        for child in children {
            if gui.file_browser.selected_folders.contains(&child) {
                let mut match_index: Option<usize> = None;
                for (index, path) in gui.file_browser.selected_folders.iter().enumerate() {
                    if *path == child {
                        match_index = Some(index);
                        break;
                    }
                }
                if match_index.is_some() {
                    gui.file_browser.selected_folders.remove(match_index.unwrap());
                }
            }
            let mut parent = gui.file_browser.folder_map.remove(&child).unwrap();
            close_children(gui, &mut parent, ctx);
            gui.file_browser.folder_map.remove(&child);
        }
    }
}


#[derive(Default, Clone)]
pub struct FileBrowser {
    pub folder_map: BTreeMap<String, MapFolder>, // <Full Path, MapFolder>
    pub selected_folders: Vec<String>,
    pub root: String,
    pub allow_frame: bool,
    pub collapsed: bool
}

#[derive(Clone, Debug)]
pub struct MapFolder {
    pub title: String,
    pub full_path: String, // Also it's key
    pub parent: String, // Parent folder
    pub root: bool,
    pub entered: bool,
    pub selected: bool,
    pub ctx_id: egui::Id
}
impl Default for MapFolder {
    fn default() -> Self {
        Self {
            title: String::new(),
            full_path: String::new(),
            parent: String::new(),
            root: false,
            entered: false,
            selected: false,
            ctx_id: egui::Id::from(utils::create_random_string(8))
        }
    }
}
impl FileBrowser {
    pub fn browse_to(&mut self, path: String) -> Result<(), String> {
        let check = fs::read_dir(path.to_owned());
        match check { // Check if the path is valid.
            Err(err) => {
                return Err(err.to_string());
            },
            _ => {}
        }
        let path = path.replacen("\\", "/", 254);
        let path_original = path.clone();
        let path_split = path.split("/");
        let mut path_pieces: Vec<String> = vec![];
        
        { // Remove empty pieces
            for piece in path_split {
                if piece != "" {
                    path_pieces.push(piece.to_string());
                }
            }
        }


        let mut path: String;
        #[cfg(target_os="windows")] {
            path = format!("{}/", path_pieces.remove(0));
        }

        #[cfg(target_os="linux")] {
            path = String::from("/");
        }

        self.root = path.clone();

        { // Read the root
            let branches = WindowMain::read_directory(path.to_owned(), true);
            for (index, branch) in branches.iter().enumerate() {
                self.folder_map.insert(branch.full_path.to_owned(), branch.to_owned());
            };
        }

        let mut first: bool = true;
        for (index, piece) in path_pieces.iter().enumerate() {
            if first == true {
                first = false;
                path = format!("{}{}", path, piece);
            } else {
                path = format!("{}/{}", path, piece);
            }
            let branches = WindowMain::read_directory(path.to_owned(), false);
            for (index, branch) in branches.iter().enumerate() {
                self.folder_map.insert(branch.full_path.to_owned(), branch.to_owned());
            };

            match self.folder_map.get_mut(&path) {
                Some(root) => {
                    root.entered = true;
                },
                None => {}
            }
        };
        match self.folder_map.get_mut(&path_original) {
            Some(i) => {
                i.selected = true;
                self.selected_folders.push(path_original);
            }, 
            None => {}
        }
        Ok(())
    }

    fn get_children_of(&self, parent: String) -> Option<Vec<String>> {
        let mut children: Vec<String> = vec![];
        for folder in self.folder_map.values() {
            if folder.parent == parent {
                children.push(folder.full_path.clone());
            };
        };
        if children.len() >= 1 {
            Some(children)
        } else {
            None
        }
    }

}
