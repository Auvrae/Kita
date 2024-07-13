use super::super::app::{WindowMain, Options, Theme};
use super::super::mods::ModsOrder;
use whoami::username;
use std::{fs, io};
use json::{self, object};

const PATH_CONFIG_WINDOWS: [&str; 2] = ["C:/Users/", "/AppData/Local/kita"];
const PATH_CONFIG_UNIX: [&str; 2] = ["/home/", "/.config/kita"];
const PATH_CONFIG_DARWIN: [&str; 2] = ["/Users/", "/.config/kita"];

// General config
pub fn read_config() -> Options {
    let config: Option<json::JsonValue>;
    #[cfg(target_os = "linux")]
    {
        let dir = fs::read_dir(format!("{}{}{}", PATH_CONFIG_UNIX[0], username(), PATH_CONFIG_UNIX[1]));
        match dir {
            Ok(kita) => {
                let mut found: bool = false;
                for item in kita {
                    if let Some(i) = item.ok() {
                        if i.file_name() == "config.json" {
                            found = true;
                        };
                    };
                };
                if found == true {
                    let config_file: Vec<u8> = fs::read(
                        format!("{}{}{}/config.json", PATH_CONFIG_UNIX[0], username(), PATH_CONFIG_UNIX[1])
                    ).unwrap();
                    config = Some(json::parse(String::from_utf8(config_file).unwrap().as_str()).unwrap());
                } else {
                    config = None;
                }
            }, 
            Err(err) => {
                config = None;
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        let dir = fs::read_dir(format!("{}{}{}", PATH_CONFIG_WINDOWS[0], username(), PATH_CONFIG_WINDOWS[1]));
        match dir {
            Ok(kita) => {
                let mut found: bool = false;
                for item in kita {
                    if let Some(i) = item.ok() {
                        if i.file_name() == "config.json" {
                            found = true;
                        };
                    };
                };
                if found == true {
                    let config_file: Vec<u8> = fs::read(
                        format!("{}{}{}/config.json", PATH_CONFIG_WINDOWS[0], username(), PATH_CONFIG_WINDOWS[1])
                    ).unwrap();
                    config = Some(json::parse(String::from_utf8(config_file).unwrap().as_str()).unwrap());
                } else {
                    config = None;
                }
            }, 
            Err(err) => {
                config = None;
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        let dir = fs::read_dir(format!("{}{}{}", PATH_CONFIG_DARWIN[0], username(), PATH_CONFIG_DARWIN[1]));
        match dir {
            Ok(kita) => {
                let mut found: bool = false;
                for item in kita {
                    if let Some(i) = item.ok() {
                        if i.file_name() == "config.json" {
                            found = true;
                        };
                    };
                };
                if found == true {
                    let config_file: Vec<u8> = fs::read(
                        format!("{}{}{}/config.json", PATH_CONFIG_DARWIN[0], username(), PATH_CONFIG_DARWIN[1])
                    ).unwrap();
                    config = Some(json::parse(String::from_utf8(config_file).unwrap().as_str()).unwrap());
                } else {
                    config = None;
                }
            }, 
            Err(err) => {
                config = None;
            }
        }
    }
    if config.is_some() {
        return serialize_config(config.unwrap())
    } else {
        return Options::default();
    }
}

pub fn write_config(options: Options) -> Result<(), String> {
    let path: String;
    #[cfg(target_os = "linux")]
    {
        path = format!("{}{}{}", PATH_CONFIG_UNIX[0], username(), PATH_CONFIG_UNIX[1]);
    }
    #[cfg(target_os = "windows")]
    {
        path = format!("{}{}{}", PATH_CONFIG_WINDOWS[0], username(), PATH_CONFIG_WINDOWS[1]);
    }
    #[cfg(target_os = "macos")]
    {
        path = format!("{}{}{}", PATH_CONFIG_DARWIN[0], username(), PATH_CONFIG_DARWIN[1]);
    }

    let config = deserialize_config(options);

    if fs::read_dir(&path).is_ok() {
        // Write the file
        match fs::write(format!("{}/config.json", path), json::stringify_pretty(config, 2)) {
            Ok(()) => {
                return Ok(());
            },
            Err(error) => {
                return Err(error.to_string());
            }
        };
    } else {
        // Path doesn't exist, attempt to create it.
        match fs::create_dir_all(&path) {
            Ok(()) => {
                // Write the file
                match fs::write(format!("{}/config.json", path), json::stringify_pretty(config, 2)) {
                    Ok(()) => {
                        return Ok(());
                    },
                    Err(error) => {
                        return Err(error.to_string());
                    }
                };
            },
            Err(error) => {
                return Err(error.to_string());
            }
        };
    }
}

pub fn serialize_config(config: json::JsonValue) -> Options {
    let mut options = Options::default();
    
    // Gui Scale
    if !config["gui_scale"].is_null() {
        options.gui_scale = config["gui_scale"].as_f32().unwrap();
    }
    
    // Modifiers Order
    if config["modifier_order"].is_array() {
        let total = config["modifier_order"].len();
        if total >= 1 {
            options.modifier_order.clear();
            for i in 0..total {
                match config["modifier_order"][i].clone().as_str().unwrap() {
                    "add" => {
                        if !options.modifier_order.contains(&ModsOrder::Add) {
                            options.modifier_order.push(ModsOrder::Add);
                        }
                    },
                    "case" => {
                        if !options.modifier_order.contains(&ModsOrder::Case) {
                            options.modifier_order.push(ModsOrder::Case);
                        }
                    },
                    "date" => {
                        if !options.modifier_order.contains(&ModsOrder::Date) {
                            options.modifier_order.push(ModsOrder::Date);
                        }
                    },
                    "ext" => {
                        if !options.modifier_order.contains(&ModsOrder::Ext) {
                            options.modifier_order.push(ModsOrder::Ext);
                        }
                    },
                    "hash" => {
                        if !options.modifier_order.contains(&ModsOrder::Hash) {
                            options.modifier_order.push(ModsOrder::Hash);
                        }
                    },
                    "movecopy" => {
                        if !options.modifier_order.contains(&ModsOrder::MoveCopy) {
                            options.modifier_order.push(ModsOrder::MoveCopy);
                        }
                    },
                    "name" => {
                        if !options.modifier_order.contains(&ModsOrder::Name) {
                            options.modifier_order.push(ModsOrder::Name);
                        }
                    },
                    "number" => {
                        if !options.modifier_order.contains(&ModsOrder::Number) {
                            options.modifier_order.push(ModsOrder::Number);
                        }
                    },
                    "regex" => {
                        if !options.modifier_order.contains(&ModsOrder::Regex) {
                            options.modifier_order.push(ModsOrder::Regex);
                        }
                    },
                    "remove" => {
                        if !options.modifier_order.contains(&ModsOrder::Remove) {
                            options.modifier_order.push(ModsOrder::Remove);
                        }
                    },
                    "replace" => {
                        if !options.modifier_order.contains(&ModsOrder::Replace) {
                            options.modifier_order.push(ModsOrder::Replace);
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    // General Settings
    match &config["general"]["theme"] {
        json::JsonValue::Short(string) => {
            if *string == String::from("dark") {
                options.general.theme = Theme::Dark;
                options.general.theme_name = String::from("Dark");
            } else if *string == String::from("light") {
                options.general.theme = Theme::Light;
                options.general.theme_name = String::from("Light");
            }
        },
        _ => {}
    }
    
    // Appearance Settings


    // File Browser Section Settings
    if !config["filebrowser"]["multi_select"].is_null() {
        options.file_browser.multi_select = config["filebrowser"]["multi_select"].as_bool().unwrap();
    }

    // File Selector Section Settings
    if !config["fileselector"]["stripped_column"].is_null() {
        options.file_selection.stripped_column = config["fileselector"]["stripped_column"].as_bool().unwrap();
    }
    if !config["fileselector"]["list_folders"].is_null() {
        options.file_selection.list_folders = config["fileselector"]["list_folders"].as_bool().unwrap();
    }
    if !config["fileselector"]["always_show_extra_row"].is_null() {
        options.file_selection.always_show_extra_row = config["fileselector"]["always_show_extra_row"].as_bool().unwrap();
    }

    // File Modifier Section Settings
    if !config["filemodifiers"]["sub_modifier_maximum"].is_null() {
        options.file_modifiers.sub_modifier_maximum = config["filemodifiers"]["sub_modifier_maximum"].as_u8().unwrap();

    }
    if config["filemodifiers"]["modifiers_enabled"].is_array() {
        let total = config["filemodifiers"]["modifiers_enabled"].len();
        if total >= 1 {
            for i in 0..=total {
                match config["filemodifiers"]["modifiers_enabled"][i].clone().as_str().unwrap() {
                    "add" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Add) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Add);
                        }
                    },
                    "case" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Case) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Case);
                        }
                    },
                    "date" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Date) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Date);
                        }
                    },
                    "ext" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Ext) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Ext);
                        }
                    },
                    "hash" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Hash) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Hash);
                        }
                    },
                    "movecopy" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::MoveCopy) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::MoveCopy);
                        }
                    },
                    "name" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Name) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Name);
                        }
                    },
                    "number" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Number) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Number);
                        }
                    },
                    "regex" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Regex) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Regex);
                        }
                    },
                    "remove" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Remove) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Remove);
                        }
                    },
                    "replace" => {
                        if !options.file_modifiers.modifiers_enabled.contains(&ModsOrder::Replace) {
                            options.file_modifiers.modifiers_enabled.push(ModsOrder::Replace);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    
    // Saving Settings

    // Presets Settings

    // Experimental Settings

    options
}

pub fn deserialize_config(config: Options) -> json::JsonValue {
    let mut jconfig = object! {
        gui_scale: config.gui_scale,
        modifier_order: [],
        general: {
            theme: "",
        },
        appearance: {

        },
        filebrowser: {
            multi_select: config.file_browser.multi_select
        },
        fileselector: {
            stripped_column: config.file_selection.stripped_column,
            list_folders: config.file_selection.list_folders,
            always_show_extra_row: config.file_selection.always_show_extra_row
        },
        filemodifiers: {
            sub_modifier_maximum: config.file_modifiers.sub_modifier_maximum,
            modifiers_enabled: []
        },
        saving: {

        },
        presets: {

        },
        experimental: {

        }
    };

    // Add Modifiers in order
    for modifier in config.modifier_order {
        match modifier {
            ModsOrder::Add => {
                jconfig["modifier_order"].push("add").unwrap();
            },
            ModsOrder::Case => {
                jconfig["modifier_order"].push("case").unwrap();
            },
            ModsOrder::Date => {
                jconfig["modifier_order"].push("date").unwrap();
            },
            ModsOrder::Ext => {
                jconfig["modifier_order"].push("ext").unwrap();
            },
            ModsOrder::Hash => {
                jconfig["modifier_order"].push("hash").unwrap();
            },
            ModsOrder::MoveCopy => {
                jconfig["modifier_order"].push("movecopy").unwrap();
            },
            ModsOrder::Name => {
                jconfig["modifier_order"].push("name").unwrap();
            },
            ModsOrder::Number => {
                jconfig["modifier_order"].push("number").unwrap();
            },
            ModsOrder::Regex => {
                jconfig["modifier_order"].push("regex").unwrap();
            },
            ModsOrder::Remove => {
                jconfig["modifier_order"].push("remove").unwrap();
            },
            ModsOrder::Replace => {
                jconfig["modifier_order"].push("replace").unwrap();
            }
        }
    };

    // Set Theme
    match config.general.theme {
        Theme::Dark => {
            jconfig["general"]["theme"] = "dark".into();
        },
        Theme::Light => {
            jconfig["general"]["theme"] = "light".into();
        }
    };

    // Add Enabled Modifiers
    for modifier in config.file_modifiers.modifiers_enabled {
        match modifier {
            ModsOrder::Add => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("add").unwrap();
            },
            ModsOrder::Case => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("case").unwrap();
            },
            ModsOrder::Date => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("date").unwrap();
            },
            ModsOrder::Ext => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("ext").unwrap();
            },
            ModsOrder::Hash => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("hash").unwrap();
            },
            ModsOrder::MoveCopy => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("movecopy").unwrap();
            },
            ModsOrder::Name => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("name").unwrap();
            },
            ModsOrder::Number => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("number").unwrap();
            },
            ModsOrder::Regex => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("regex").unwrap();
            },
            ModsOrder::Remove => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("remove").unwrap();
            },
            ModsOrder::Replace => {
                jconfig["filemodifiers"]["modifiers_enabled"].push("replace").unwrap();
            }
        }
    };

    return jconfig;
}


// Presets
pub fn read_presets() {

}

pub fn write_presets() {

}

pub fn serialize_presets() {

}

pub fn deserialize_presets() {

}

