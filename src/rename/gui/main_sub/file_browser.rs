use std::time::Instant;

use super::super::super::app::WindowMain;
use super::super::super::debug::DebugStatType;
use super::super::main_sub::file_browser;

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
                                    gui.file_browser.roots = gui.read_directory(mount.to_owned(), true, vec![]);
                                    gui.file_browser.selected_children.clear();
                                }
                            };
                        });
                    }
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    let button = ui.button("⟲");
                    if button.clicked() {
                        #[cfg(target_os="windows")] {
                            if !gui.file_mounts.is_empty() {
                                gui.file_browser.roots = gui.read_directory(String::from(gui.file_mounts[0].clone()), true, vec![]);
                                gui.file_mounts_selected = 0;
                                gui.file_browser.selected_children.clear();
                            }
                        }
                        #[cfg(target_os="linux")] {
                            gui.file_browser.roots = gui.read_directory(String::from("/"), true, vec![]);
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
                let mut roots: Vec<file_browser::BrowserItem> = vec![];
                for mut root in gui.file_browser.roots.clone() {
                    fill_tree(gui, &mut root, ui, ctx);
                    roots.push(root);
                };
                gui.file_browser.roots = roots;
            });
        });
    });

    // Update Stats
    gui.statistics.push(start.elapsed().as_micros() as u32, DebugStatType::GuiBrowser);
}

fn fill_tree(gui: &mut WindowMain, item: &mut BrowserItem, ui: &mut egui::Ui, ctx: &egui::Context) {
    let mut header = egui::collapsing_header::CollapsingState::load_with_default_open(ctx, item.id, false);
    header.set_open(item.entered);
    let response = header.show_header(ui, |ui| {
        let title = ui.selectable_label(item.selected, item.title.clone());
        if item.selected && !gui.file_browser.selected_children.contains(&item.depth) {
            item.selected = false;
        };
        if title.clicked() && item.selected == false {
            select_and_travel(gui, item, ctx, false);
            if gui.options.file_browser.multi_select == true {
                gui.file_browser.selected_children.push(item.depth.to_owned());
                if item.root {
                    gui.file_browser.selected_children_paths.push(format!("{}{}", item.path, item.title));
                } else {
                    gui.file_browser.selected_children_paths.push(format!("{}/{}", item.path, item.title));
                };
            } else if gui.file_browser.selected_children.len() >= 1 {
                    gui.file_browser.selected_children.clear();
                    gui.file_browser.selected_children.push(item.depth.to_owned());
                    gui.file_browser.selected_children_paths.clear();
                    if item.root {
                        gui.file_browser.selected_children_paths.push(format!("{}{}", item.path, item.title));
                    } else {
                        gui.file_browser.selected_children_paths.push(format!("{}/{}", item.path, item.title));
                    };
            } else {
                gui.file_browser.selected_children.push(item.depth.to_owned());
                if item.root {
                    gui.file_browser.selected_children_paths.push(format!("{}{}", item.path, item.title));
                } else {
                    gui.file_browser.selected_children_paths.push(format!("{}/{}", item.path, item.title));
                };
            }
            item.selected = true;
        } else if title.clicked() && item.selected == true {
            if gui.options.file_browser.multi_select == true {
                for (index, depth) in gui.file_browser.selected_children.clone().iter().enumerate() {
                    if *depth == item.depth {
                        gui.file_browser.selected_children.remove(index);  
                        gui.file_browser.selected_children_paths.remove(index);
                        break;
                    }
                }
            } else {
                gui.file_browser.selected_children.clear();
                gui.file_browser.selected_children_paths.clear();
            }
            item.selected = false;
        }
    });
    let (mut header_res, _body_res, _) = response.body(|ui| {
        let mut children: Vec<BrowserItem> = vec![];
        let mut children_ids: Vec<egui::Id> = vec![];
        for mut child in item.children.clone() {
            fill_tree(gui, &mut child, ui, ctx);
            children_ids.push(child.id);
            children.push(child);
        }
        item.children = children;
        item.children_ids = children_ids;
    });
    if item.selected {
        header_res = header_res.highlight();
    };
    if header_res.clicked() {
        select_and_travel(gui, item, ctx, true);
    };
}   

fn select_and_travel(gui: &mut WindowMain, item: &mut BrowserItem, ctx: &egui::Context, collapsing: bool) {
    if item.entered == false {
        if collapsing == true {
            item.entered = true;
        };
        if item.root {
            if collapsing == true {
                item.children = gui.read_directory(format!("{}{}", item.path, item.title), false, item.depth.to_owned());
            };
        } else {
            if collapsing == true{
                item.children = gui.read_directory(format!("{}/{}", item.path, item.title), false, item.depth.to_owned());
            };
        };
    } else {
        if collapsing == true {
            item.entered = false;
            close_children(gui, item, ctx);
        };
    };
}

fn close_children(gui: &mut WindowMain, item: &mut BrowserItem, ctx: &egui::Context) {
    for mut child in item.children.clone() {
        close_children(gui, &mut child, ctx);
        match egui::collapsing_header::CollapsingState::load(ctx, child.id) {
            Some(child) => {
                child.remove(ctx);
            }, None => {}
        };
        for (index, depth) in gui.file_browser.selected_children.clone().iter().enumerate() {
            if *depth == child.depth {
                gui.file_browser.selected_children.remove(index);  
                gui.file_browser.selected_children_paths.remove(index);
                break;
            }
        }
    };
    item.children.clear();
    item.children_ids.clear();
}

#[derive(Default, Clone)]
pub struct FileBrowser {
    pub roots: Vec<BrowserItem>,
    pub selected_children_paths: Vec<String>,
    pub selected_children: Vec<Vec<u32>>,
    pub files_werent_modified: bool, // A bool that changes based on if anything has changed any of the files.
    pub allow_frame: bool,
}

#[derive(Clone, Debug)]
pub struct BrowserItem {
    pub title: String,
    pub id: egui::Id,
    pub path: String,
    pub entered: bool,
    pub selected: bool,
    pub root: bool,
    pub children: Vec<Self>,
    pub children_ids: Vec<egui::Id>,
    pub depth: Vec<u32>
}

impl BrowserItem {
    /// Digs through a BroswerItem to get to the deeply nested child. Returns a CLONE of the nested child.
    pub fn _dig_for_child(&mut self, child: BrowserItem, depth: &mut Vec<u32>) -> BrowserItem {
        let mut found: Self = child.clone();
        if depth.len() >= 1 {
            let index = depth.remove(0) as usize;
            found = self._dig_for_child(child.children[index].clone(), depth);
            println!("{}", index);
        }
        return found;
    }
}