use super::super::app::{WindowMain, Options};
use whoami::username;
use std::{fs, io};
use json;

const PATH_CONFIG_WINDOWS: [&str; 2] = ["C:/Users/", "/AppData/Local/kita"];
const PATH_CONFIG_UNIX: [&str; 2] = ["/home/", "/.config/kita"];
const PATH_CONFIG_DARWIN: [&str; 2] = ["/Users/", "/.config/kita"];

// General config
pub fn read_config() -> Option<Options> {
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
                    return None
                }
            }, 
            Err(err) => {
                return None
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
                    return None
                }
            }, 
            Err(err) => {
                return None
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
                    return None
                }
            }, 
            Err(err) => {
                return None
            }
        }
    }
    if config.is_some() {
        serialize_config(config.unwrap());
    } else {
        return None
    }

    None
}

pub fn write_config(options: Options) {

}

pub fn serialize_config(config: json::JsonValue) { //-> Options {
    
}

pub fn deserialize_config(config: Options) { //-> json::JsonValue {
    
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

