use super::super::app::WindowMain;
use super::super::presets::Preset;
use super::super::util::config;

use std::fs;

pub fn parse_arguments(app: &mut WindowMain, args: Vec<String>) -> CliResult {
    if args.len() >= 1 {
        let mut check_for_path: Vec<String> = vec![];
        let mut operator: CliArgType = CliArgType::None;
        let mut verified_path: Option<String> = None;
        let mut verified_preset: Option<Preset> = None;
        for arg in args {
            match arg.to_ascii_lowercase().as_str() {
                "-h" | "--help" => {
                    println!("Help... Not compelted");
                    std::process::exit(0);
                }
                "-p" => { // Apply Preset
                    operator = CliArgType::Operator(CliOperator::ApplyPreset)
                },
                "-op" => { // Open with Preset selected
                    operator = CliArgType::Operator(CliOperator::OpenPreset)
                },
                "-o" => { // Open to Path
                    operator = CliArgType::Operator(CliOperator::OpenPath)
                },
                _ => {
                    check_for_path.push(arg);
                }
            }
        }
        for argument in check_for_path {
            let test = verify_path(argument.clone());
            if test.is_some() {
                verified_path = Some(test.unwrap());
            };
            let preset = verfiy_preset(app, argument);
            if preset.is_some() {
                verified_preset = Some(preset.unwrap());
            };
        }
        match operator {
            CliArgType::None => {
                return CliResult::Continue;
            },
            CliArgType::Operator(CliOperator::ApplyPreset) => {
                let preset: Preset;
                if verified_preset.is_some() {
                    preset = verified_preset.unwrap();
                    app.modifiers = preset.modifiers.to_owned();
                    app.options.modifier_order.0 = preset.modifier_order.to_owned();
                    app.popups.save_as_preset_field_name = preset.name.to_owned();
                } else {
                    return CliResult::Error(String::from(format!("{}{}",
                        "Could not find that preset. Check to make sure you've used",
                        "the right name! If it has spaces, surround it in \"<Preset_Name>\""
                    )))
                };
                let path: String;
                if verified_path.is_some() {
                    path = verified_path.unwrap();
                    apply_preset(app, path, preset).unwrap();
                } else {
                    return CliResult::Error(format!("{}",
                        "No valid path given."
                    ))
                }
            },
            CliArgType::Operator(CliOperator::OpenPath) => {
                if verified_path.is_some() {
                    let path = verified_path.unwrap();
                    #[cfg(target_os="linux")] {
                        app.no_refresh = true;
                        app.file_browser.browse_to(path.clone()).unwrap();
                    }
                    #[cfg(target_os="windows")] {
                        let drive_letter = &path.clone()[0..3];
                        let mut drive_index: Option<usize> = None;
                        for (index, mount) in app.file_mounts.iter().enumerate() {
                            if *mount == drive_letter {
                                drive_index = Some(index);
                            }
                        }
                        if drive_index.is_some() {
                            app.no_refresh = true;
                            app.file_mounts_selected = drive_index.unwrap() as u8;
                            app.file_browser.browse_to(path.clone()).unwrap();
                        } else {
                            return CliResult::Error(String::from("Invalid path arguement."));
                        }
                    }

                    // Update Recently Open Vec
                    {
                        let mut was_recent: (usize, bool) = (0, false);
                        for (index, recent_path) in app.options.recently_opened.iter().enumerate() {
                            if recent_path == &path {
                                was_recent = (index, true);
                            }
                        }
                        if was_recent.1 == true {
                            let recent_path = app.options.recently_opened.remove(was_recent.0);
                            app.options.recently_opened.insert(0, recent_path);
                            
                        } else {
                            app.options.recently_opened.push(path);
                        }
                        config::write_config(app.options.clone()).unwrap();
                    }
                    return CliResult::Continue;
                } else {
                    return CliResult::Error(format!("{}",
                        "No valid path given."
                    ))
                };
            },
            CliArgType::Operator(CliOperator::OpenPreset) => {
                if verified_preset.is_some() && verified_path.is_some() {
                    let preset = verified_preset.unwrap();
                    let path = verified_path.unwrap();
                    app.modifiers = preset.modifiers.to_owned();
                    app.options.modifier_order.0 = preset.modifier_order.to_owned();
                    app.popups.save_as_preset_field_name = preset.name.to_owned();
                    #[cfg(target_os="linux")] {
                        app.no_refresh = true;
                        app.file_browser.browse_to(path.clone()).unwrap();
                    }
                    #[cfg(target_os="windows")] {
                        let drive_letter = &path.clone()[0..3];
                        println!("{}", drive_letter);
                        let mut drive_index: Option<usize> = None;
                        for (index, mount) in app.file_mounts.iter().enumerate() {
                            if *mount == drive_letter {
                                drive_index = Some(index);
                            }
                        }
                        if drive_index.is_some() {
                            app.no_refresh = true;
                            app.file_mounts_selected = drive_index.unwrap() as u8;
                            app.file_browser.browse_to(path.clone()).unwrap();
                        } else {
                            return CliResult::Error(String::from("Invalid path arguement."));
                        }
                    }

                    // Update Recently Open Vec
                    {
                        let mut was_recent: (usize, bool) = (0, false);
                        for (index, recent_path) in app.options.recently_opened.iter().enumerate() {
                            if recent_path == &path {
                                was_recent = (index, true);
                            }
                        }
                        if was_recent.1 == true {
                            let recent_path = app.options.recently_opened.remove(was_recent.0);
                            app.options.recently_opened.insert(0, recent_path);
                            
                        } else {
                            app.options.recently_opened.push(path);
                        }
                        config::write_config(app.options.clone()).unwrap();
                    }

                    return CliResult::Continue;
                } else {
                    return CliResult::Error(String::from(format!("{}{}",
                        "Could not find that preset. Check to make sure you've used",
                        "the right name! If it has spaces, surround it in \"<Preset_Name>\""
                    )))
                };
            }
        }
    } else {
        return CliResult::Continue
    }
    CliResult::Continue
}

fn apply_preset(app: &mut WindowMain, path: String, preset: Preset) -> Result<(), String> {
    

    Err(String::from("Not implemented yet.. hehe"))
}

fn verify_path(mut path: String) -> Option<String> {
    path = path.replacen("\\", "/", 254).to_string();
    if path.len() >= 3 && path.ends_with("/") {
        path = path[0..path.len() - 1].to_string();
    }
    match fs::read_dir(&path) {
        Ok(_) => {
            return Some(path);
        },
        Err(_) => {
            return None;    
        }
    };
}

fn verfiy_preset(app: &mut WindowMain, name: String) -> Option<Preset> {
    for preset in app.presets.sets.iter() {
        if preset.name == name {
            return Some(preset.to_owned());
        };
    };
    return None;
}

#[derive(Debug, Clone)]
pub enum CliResult {
    Continue,
    Stop,
    Error(String)
}

#[derive(Debug, Clone)]
enum CliArgType {
    None,
    Operator(CliOperator)
}

#[derive(Debug, Clone)]
enum CliOperator {
    OpenPath,
    OpenPreset,
    ApplyPreset
}