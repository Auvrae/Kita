use super::super::app::Options;
use super::super::presets::Presets;
use whoami::username;
use std::fs;

const PATH_CONFIG_WINDOWS: [&str; 2] = ["C:/Users/", "/AppData/Local/kita"];
const PATH_CONFIG_UNIX: [&str; 2] = ["/home/", "/.config/kita"];
const PATH_CONFIG_DARWIN: [&str; 2] = ["/Users/", "/.config/kita"];

// General config
pub fn read_config() -> Options {
    let config: Option<String>;
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
                    config = Some(String::from_utf8(config_file).unwrap());
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
                    config = Some(String::from_utf8(config_file).unwrap())
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
                    config = Some(String::from_utf8(config_file).unwrap())
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
        return serialize_config(config.unwrap());
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
        match fs::write(format!("{}/config.json", path), config) {
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
                    match fs::write(format!("{}/config.json", path), config) {
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

pub fn serialize_config(config: String) -> Options {
    let options: Options = serde_json::from_str(&config).unwrap();
    options
}

pub fn deserialize_config(config: Options) -> String {
    match serde_json::to_string_pretty(&config) {
        Ok(json_data) => json_data,
        Err(err) => panic!("{err}")
    }
}


// Presets
pub fn read_presets() -> Presets {
    let config: Option<String>;
    #[cfg(target_os = "linux")]
    {
        let dir = fs::read_dir(format!("{}{}{}", PATH_CONFIG_UNIX[0], username(), PATH_CONFIG_UNIX[1]));
        match dir {
            Ok(kita) => {
                let mut found: bool = false;
                for item in kita {
                    if let Some(i) = item.ok() {
                        if i.file_name() == "presets.json" {
                            found = true;
                        };
                    };
                };
                if found == true {
                    let config_file: Vec<u8> = fs::read(
                        format!("{}{}{}/presets.json", PATH_CONFIG_UNIX[0], username(), PATH_CONFIG_UNIX[1])
                    ).unwrap();
                    config = Some(String::from_utf8(config_file).unwrap());
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
                        if i.file_name() == "presets.json" {
                            found = true;
                        };
                    };
                };
                if found == true {
                    let config_file: Vec<u8> = fs::read(
                        format!("{}{}{}/presets.json", PATH_CONFIG_WINDOWS[0], username(), PATH_CONFIG_WINDOWS[1])
                    ).unwrap();
                    config = Some(String::from_utf8(config_file).unwrap())
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
                        if i.file_name() == "presets.json" {
                            found = true;
                        };
                    };
                };
                if found == true {
                    let config_file: Vec<u8> = fs::read(
                        format!("{}{}{}/presets.json", PATH_CONFIG_DARWIN[0], username(), PATH_CONFIG_DARWIN[1])
                    ).unwrap();
                    config = Some(String::from_utf8(config_file).unwrap())
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
        return serialize_presets(config.unwrap());
    } else {
        return Presets::default();
    }
}

pub fn write_presets(presets: Presets) -> Result<(), String> {
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

    let config = deserialize_presets(presets);

    if fs::read_dir(&path).is_ok() {
        // Write the file
        match fs::write(format!("{}/presets.json", path), config) {
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
                    match fs::write(format!("{}/presets.json", path), config) {
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

pub fn serialize_presets(config: String) -> Presets {
    let options: Presets = serde_json::from_str(&config).unwrap_or_default();
    options
}

pub fn deserialize_presets(config: Presets) -> String {
    match serde_json::to_string_pretty(&config) {
        Ok(json_data) => json_data,
        Err(err) => panic!("{err}")
    }
}

