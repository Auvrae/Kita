use super::super::super::util::threads::{ModifierThreadError, HashMode, HashType};
use super::super::super::mods::*;

use chrono;
use numerals;
use alpha_counter;
use regex::Regex;
use std::path::Path;

const FORBIDDEN_CHARS_WINDOWS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
const FORBIDDEN_CHARS_UNIX: [char; 2] = ['/', '\\'];
const FORBIDDEN_FILE_NAMES_WINDOWS: [&str; 22] = ["CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", 
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"];
const NONTITLE_WORDS: [&str; 24] = ["a", "an", "and", "as", "at", "but", "by", "en", "for", "from", "if", "in", 
    "nor", "of", "on", "or", "per", "the", "to", "up", "v", "vs", "viaare", "yet"];
const ACCENTED_CHARS: [char; 29] = ['á', 'à', 'â', 'ä', 'ã', 'å', 'æ', 'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô', 'ö', 'õ', 
    'ø', 'œ', 'ß', 'ú', 'ù', 'û', 'ü'];
const ACCENTED_CHARS_REPLACEMENT: [char; 29] = ['a', 'a', 'a', 'a', 'a', 'a', 'a', 'c', 'e', 'e', 'e', 'e', 'i', 'i', 'i', 'i', 'n', 'o', 'o', 'o',  
    'o', 'o', 'o', 'a', 'B', 'u', 'u', 'u', 'u'];
const SPECIAL_CHARS: [char; 29] = ['~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+', '=', '{', '}', '[', ']', '|', 
    ':', '\'', '<', '>', ',', '?', '/', '\\'];
const ALPHA_LOWER_UPPER: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n','o', 'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N','O', 'P', 'Q', 'R', 'S', 'T', 'U', 
    'V', 'W', 'X', 'Y', 'Z'];

pub fn process(_index: usize, modifiers: &mut Modifiers, file_names: Vec<(String, usize, Option<String>)>, order: Vec<ModsOrder>, is_folder: bool) -> (Vec<(String, usize, Option<String>)>, Vec<ModifierThreadError>) {
    let mut files: Vec<(String, usize, Option<String>)> = vec![];
    let mut errors: Vec<ModifierThreadError> = vec![];
    let mut duplicates: Vec<usize> = vec![];
    let mut files_over_length: Vec<(usize, u32)> = vec![];
    let mut invalid_chars: Vec<(usize, char)> = vec![];
    let mut invalid_names: Vec<(usize, String)> = vec![];
    for (index, filename_raw) in file_names.iter().enumerate() {
        let mut extension_raw = Path::new(&filename_raw.0).extension().unwrap_or_default().to_str().unwrap_or_default();
        if extension_raw.contains(' ') {
            // Clear the extension buffer if there's a space in it. False positive.
            extension_raw = "";
        };
        if filename_raw.0.chars().count() >= 2 && &utils::get_utf8_slice(&filename_raw.0, 0, 2, false) == &".." {
            extension_raw = "";
        };
        let file_index: usize = filename_raw.1;
        let file: String;
        let ext: String;
        if is_folder == true { // Don't try and find an extension in a folder name
            extension_raw = "";
        };
        if !extension_raw.is_empty() {
            let (f, e) = utils::split_uft8(&filename_raw.0, filename_raw.0.chars().count() - extension_raw.chars().count() - 1);
            file = f; ext = e;
        } else {
            file = filename_raw.0.clone();
            ext = String::new();
        };
        let mut file: String = file.to_string();
        let mut ext: String = ext.to_string();
        let file_hash: String = match &filename_raw.2 {
            Some(hash) => {
                hash.to_owned()
            },
            None => {
                String::new()
            }
        };
        for (_, modifier) in order.iter().enumerate() {
            match modifier {
                ModsOrder::Add => {
                    if !modifiers.add_enabled { continue };
                    for mode in modifiers.add.clone() {
                        let res = add(file.clone(), ext.clone(), mode);
                        file = res.0;
                        ext = res.1;
                    }
                },
                ModsOrder::Case => {
                    if !modifiers.case_enabled { continue };
                    for mode in modifiers.case.clone() {
                        let res = case(file.clone(), ext.clone(), mode);
                        file = res.0;
                        ext = res.1;
                    }
                },
                ModsOrder::Date => {
                    if !modifiers.date_enabled { continue };
                    for mode in modifiers.date.clone() {
                        let res = date(file.clone(), ext.clone(), mode);
                        file = res.0;
                        ext = res.1;
                    }
                },
                ModsOrder::Ext => {
                    if !modifiers.extension_enabled { continue };
                    let res = extension(file.clone(), ext.clone(), modifiers.extension.clone());
                    file = res.0;
                    ext = res.1;
                },
                ModsOrder::Hash => {
                    if !modifiers.hash_enable { continue };
                    if is_folder { continue };
                    let res = hash(file.clone(), ext.clone(), modifiers.hash.clone(), file_hash.clone());
                    file = res.0;
                    ext = res.1;
                },
                ModsOrder::MoveCopy => {
                    if !modifiers.movecopy_enabled { continue };
                    for mode in modifiers.movecopy.clone() {
                        let res = movecopy(file.clone(), ext.clone(), mode);
                        file = res.0;
                        ext = res.1;
                    }
                },
                ModsOrder::Name => {
                    if !modifiers.name_enabled { continue };
                    for mode in modifiers.name.clone() {
                        let res = name(file.clone(), ext.clone(), mode);
                        file = res.0;
                        ext = res.1;
                    }
                },
                ModsOrder::Number => {
                    if !modifiers.number_enabled { continue };
                    for mode in modifiers.number.clone() {
                        let res = number(file.clone(), ext.clone(), mode, index);
                        file = res.0;
                        ext = res.1;
                    }
                },
                ModsOrder::Regex => {
                    if !modifiers.regex_enabled { continue };
                    for mode in modifiers.regex.clone() {
                        let res = regex(file.clone(), ext.clone(), mode);
                        file = res.0;
                        ext = res.1;
                    }
                },
                ModsOrder::Remove => {
                    if !modifiers.remove_enabled { continue };
                    for mode in modifiers.remove.clone() {
                        let res = remove(file.clone(), ext.clone(), mode);
                        file = res.0;
                        ext = res.1;
                    }
                },
                ModsOrder::Replace => {
                    if !modifiers.replace_enabled { continue };
                    for mode in modifiers.replace.clone() {
                        let res = replace(file.clone(), ext.clone(), mode);
                        file = res.0;
                        ext = res.1;
                    }
                }
            }
        }
        
        // Find Reserved File names for windows.
        if cfg!(windows) {
            if FORBIDDEN_FILE_NAMES_WINDOWS.iter().any(|&e| *e == file) {
                invalid_names.push((index, file.clone()))
            }  
        }      

        files.push((format!("{}{}", file, ext), file_index, None));
    }
    // Check for FS errors..
    for (index, file) in files.clone().iter().enumerate() {
        if cfg!(windows) {
            let found = file.0.find(&FORBIDDEN_CHARS_WINDOWS);
            if found.is_some() {
                let char = found.unwrap();
                invalid_chars.push((index, file.0.clone().remove(char)));
            };
        } else if cfg!(unix) {
            let found = file.0.find(&FORBIDDEN_CHARS_UNIX);
            if found.is_some() {
                let char = found.unwrap();
                invalid_chars.push((index, file.0.clone().remove(char)));
            };
        }
        let file_name = &file.0;
        for (f_index, f) in files.iter().enumerate() {
            if f_index != index {
                if *file_name == f.0 {
                    duplicates.push(index);
                }
            }
        }

        if file_name.chars().count() >= 256 {
            files_over_length.push((file.1, file_name.chars().count() as u32))
        };

    };
    errors.push(ModifierThreadError::DuplicateFileName(duplicates));
    errors.push(ModifierThreadError::LengthLimitFileName(files_over_length));
    errors.push(ModifierThreadError::InvalidChar(invalid_chars));
    errors.push(ModifierThreadError::InvalidFileName(invalid_names));
    return (files, errors);
}

fn add(mut file: String, ext: String, modadd: ModAdd) -> (String, String) {
    if !modadd.prefix.is_empty() {
        file = format!("{}{}", modadd.prefix, file);
    };
    if !modadd.insert.is_empty() {
        let insert_at: u32;
        if modadd.insert_at.is_negative() {
            insert_at = (file.chars().count() as u32 + 1) - u32::try_from(modadd.insert_at * -1).unwrap().clamp(1, file.chars().count() as u32 + 1);
        } else {
            insert_at = u32::try_from(modadd.insert_at.clone().clamp(0, file.chars().count() as i32)).unwrap();
        }
        let (start, end) = utils::split_uft8(&file, insert_at as usize);
        file = format!("{}{}{}", start, modadd.insert.to_owned(), end);
    }
    if !modadd.suffix.is_empty() {
        file = format!("{}{}", file, modadd.suffix);
    };
    (file, ext)
}

fn case(mut file: String, ext: String, modcase: ModCase) -> (String, String) {
    match modcase.mode {
        CaseMode::Lower => {
            match modcase.except_mode {
                CaseExecptMode::FromTo => {
                    let clamped_from = modcase.except_from.clamp(0, file.chars().count() as u32);
                    let clamped_to = modcase.except_to.clamp(clamped_from, file.chars().count() as u32);

                    if modcase.except_to >= 1 {
                        //Determine the kept section.
                        let keep: String;
                        if clamped_from == 0 && clamped_to == 0 {
                            keep = "".to_string();
                        } else {
                            keep = utils::get_utf8_slice(&file, clamped_from as usize, clamped_to as usize, false);
                        };
                        if clamped_from == 0 && (clamped_to > clamped_from) {
                            file = format!(
                                "{}{}",
                                keep,
                                utils::get_utf8_slice(&file, clamped_to as usize, file.chars().count(), false).to_ascii_lowercase()
                            )
                        } else if clamped_from != clamped_to {
                            file = format!(
                                "{}{}{}",
                                utils::get_utf8_slice(&file, 0, clamped_from as usize, false).to_ascii_lowercase(),
                                keep,
                                utils::get_utf8_slice(&file, clamped_to as usize, file.chars().count(), false).to_ascii_lowercase()
                            );
                        } else {
                            file = file.to_lowercase();
                        }
                    } else {
                        file = file.to_lowercase();
                    };
                },
                CaseExecptMode::Match => {
                    match file.find(&modcase.except) {
                        Some(index) => {
                            let (start, _mid) = utils::split_uft8(&file, index);
                            let (_mid, end) = utils::split_uft8(&file, index + modcase.except.chars().count());
                            file = format!("{}{}{}", start.to_ascii_lowercase(), modcase.except, end.to_ascii_lowercase());
                        },
                        _ => { file = file.to_ascii_lowercase() } // Do nothing, couldn't find a match.
                    }
                },
                CaseExecptMode::None => {
                    file = file.to_ascii_lowercase();
                }
            }
        },
        CaseMode::Title => {
            let words_raw: Vec<&str> = file.split(' ').collect();
            let mut words: Vec<String> = vec![];
            let mut file_new: String = String::new();
            let mut first_word = true;
            for word_index in 0..words_raw.len() {
                let mut word = words_raw[word_index].to_string();
                if word.chars().count() != 0 {
                    if first_word == false && NONTITLE_WORDS.contains(&word.as_str()) {
                        word = word.to_ascii_lowercase();
                    } else if !word.starts_with(&['{', '[', '(']) && !word.ends_with(&['}', ']', ')']){
                        first_word = false;
                        if word.ends_with('.') {
                            first_word = true;
                        }
                        word = word.to_ascii_lowercase();
                        let start = word.remove(0);
                        word.insert(0, start.to_ascii_uppercase());
                    }
                    words.push(word);
                };
            }
            for word_index in 0..words.len() {
                if word_index == 0 {
                    file_new = words[0].clone();
                } else {
                    file_new = format!("{} {}", file_new, words[word_index]);
                };
            }
            file = file_new;
        },
        CaseMode::Upper => {
            match modcase.except_mode {
                CaseExecptMode::FromTo => {
                    let clamped_from = modcase.except_from.clamp(0, file.chars().count() as u32);
                    let clamped_to = modcase.except_to.clamp(clamped_from, file.chars().count() as u32);
                    if modcase.except_to >= 1 {
                        //Determine the kept section.
                        let keep: String;
                        if clamped_from == 0 && clamped_to == 0 {
                            keep = "".to_string();
                        } else {
                            keep = utils::get_utf8_slice(&file, clamped_from as usize, clamped_to as usize, false);
                        };
                        if clamped_from == 0 && (clamped_to > clamped_from) {
                            file = format!(
                                "{}{}",
                                keep,
                                utils::get_utf8_slice(&file, clamped_to as usize, file.chars().count(), false).to_ascii_uppercase()
                            )
                        } else if clamped_from != clamped_to {
                            file = format!(
                                "{}{}{}",
                                utils::get_utf8_slice(&file, 0, clamped_from as usize, false).to_ascii_uppercase(),
                                keep,
                                utils::get_utf8_slice(&file, clamped_to as usize, file.chars().count(), false).to_ascii_uppercase()
                            );
                        } else {
                            file = file.to_ascii_uppercase();
                        }
                    } else {
                        file = file.to_ascii_uppercase();
                    };
                },
                CaseExecptMode::Match => {
                    match file.find(&modcase.except) {
                        Some(index) => {
                            let (start, _mid) = utils::split_uft8(&file, index);
                            let (_mid, end) = utils::split_uft8(&file, index + modcase.except.chars().count());
                            file = format!("{}{}{}", start.to_ascii_uppercase(), modcase.except, end.to_ascii_uppercase());
                        },
                        _ => { file = file.to_ascii_uppercase() } // Do nothing, couldn't find a match.
                    }
                },
                CaseExecptMode::None => {
                    file = file.to_ascii_uppercase();
                }
            }
        }, 
        CaseMode::UpperFirst => {
            let words_raw: Vec<&str> = file.split(' ').collect();
            let mut words: Vec<String> = vec![];
            let mut file_new: String = String::new();
            if words_raw.len() != 0 {
                for word_index in 0..words_raw.len() {
                    let mut word = words_raw[word_index].to_string().to_ascii_lowercase();
                    if word.chars().count() != 0 {
                        let start = word.remove(0);
                        word.insert(0, start.to_ascii_uppercase());
                        words.push(word)
                    };
                }
                for word_index in 0..words.len() {
                    if word_index == 0 {
                        file_new = words[0].clone();
                    } else {
                        file_new = format!("{} {}", file_new, words[word_index]);
                    };
                }
            }
            file = file_new;
        },
        _ => {} // Same, do nothing
    };
    (file, ext)
}

fn date(mut file: String, ext: String, moddate: ModDate) -> (String, String) {
    let mut date_string: String = String::new();
    let mut date_year: Vec<String> = vec![];
    let mut date_hour: Vec<String> = vec![];
    let local_time: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let mut year = local_time.format("%Y").to_string();
    let month = local_time.format("%m").to_string();
    let day = local_time.format("%d").to_string();
    let hour = local_time.format("%H").to_string();
    let minute = local_time.format("%M").to_string();
    let second = local_time.format("%S").to_string();
    if moddate.century == false {
        let y = year.to_string();
        year = y[2..y.len()].to_string();
    };
    match moddate.format {
        DateFormatMode::MY => {
            date_year.push(month);
            date_year.push(year);
        },
        DateFormatMode::DMY => {
            date_year.push(day);
            date_year.push(month);
            date_year.push(year);
        },
        DateFormatMode::DMYH => {
            date_year.push(day);
            date_year.push(month);
            date_year.push(year);

            date_hour.push(hour);
        },
        DateFormatMode::DMYHM => {
            date_year.push(day);
            date_year.push(month);
            date_year.push(year);

            date_hour.push(hour);
            date_hour.push(minute);
        },
        DateFormatMode::DMYHMS => {
            date_year.push(day);
            date_year.push(month);
            date_year.push(year);

            date_hour.push(hour);
            date_hour.push(minute);
            date_hour.push(second);
        },
        DateFormatMode::MDY => {
            date_year.push(month);
            date_year.push(day);
            date_year.push(year);

        },
        DateFormatMode::MDYH => {
            date_year.push(month);
            date_year.push(day);
            date_year.push(year);

            date_hour.push(hour);
        },
        DateFormatMode::MDYHM => {
            date_year.push(month);
            date_year.push(day);
            date_year.push(year);

            date_hour.push(hour);
            date_hour.push(minute);
        },
        DateFormatMode::MDYHMS => {
            date_year.push(month);
            date_year.push(day);
            date_year.push(year);

            date_hour.push(hour);
            date_hour.push(minute);
            date_hour.push(second);

        },
        DateFormatMode::Y => {
            date_year.push(year);
        },
        DateFormatMode::YM => {
            date_year.push(year);
            date_year.push(month);
        },
        DateFormatMode::YMD => {
            date_year.push(year);
            date_year.push(month);
            date_year.push(day);
        },
        DateFormatMode::YMDH => {
            date_year.push(year);
            date_year.push(month);
            date_year.push(day);

            date_hour.push(hour);
        },
        DateFormatMode::YMDHM => {
            date_year.push(year);
            date_year.push(month);
            date_year.push(day);

            date_hour.push(hour);
            date_hour.push(minute);
        },
        DateFormatMode::YMDHMS => {
            date_year.push(year);
            date_year.push(month);
            date_year.push(day);

            date_hour.push(hour);
            date_hour.push(minute);
            date_hour.push(second);

        },
        DateFormatMode::Custom => {
            if !chrono::format::strftime::StrftimeItems::new(&moddate.custom).any(|i| matches!(i, chrono::format::Item::Error)) {
                date_string = chrono::Local::now().format(&moddate.custom).to_string();
                match moddate.mode {
                    DateMode::Prefix => {
                        file = format!("{}{}", date_string, file);
                    },
                    DateMode::Suffix => {
                        file = format!("{}{}", file, date_string);
                    },
                    DateMode::Insert => {
                        let insert_at: u32;
                        if moddate.at_pos.is_negative() {
                            insert_at = (file.chars().count() as u32 + 1) - u32::try_from(moddate.at_pos * -1).unwrap().clamp(1, file.chars().count() as u32 + 1);
                        } else {
                            insert_at = u32::try_from(moddate.at_pos.clone().clamp(0, file.chars().count() as i32)).unwrap();
                        }
                        let (start, end) = utils::split_uft8(&file, insert_at as usize);
                        file = format!("{}{}{}", start, date_string, end);
                    },
                    _ => {}
                }
            }
            return (file, ext);
        }
    };
    let seg_year_char: char = match moddate.segregator_year {
        DateSeperator::None => {
            '\0'
        },
        DateSeperator::Space => {
            ' '
        },
        DateSeperator::Colan => {
            ':'
        },
        DateSeperator::Line => {
            '|'
        },
        DateSeperator::Minus => {
            '-'
        },
        DateSeperator::Plus => {
            '+'
        }, 
        DateSeperator::Underscore => {
            '_'
        },
        _ => { '\0' }
    };
    let seg_hour_char: char = match moddate.segregator_hour {
        DateSeperator::None => {
            '\0'
        },
        DateSeperator::Space => {
            ' '
        },
        DateSeperator::Colan => {
            ':'
        },
        DateSeperator::Line => {
            '|'
        },
        DateSeperator::Minus => {
            '-'
        },
        DateSeperator::Plus => {
            '+'
        }, 
        DateSeperator::Underscore => {
            '_'
        },
        _ => { '\0' }
    };
    for (usize, year_mod) in date_year.iter().enumerate() {
        if date_year.len() == 1 {
            date_string = format!("{}{}", date_string, year_mod);
            continue;
        };
        
        if usize == date_year.len() -1 {
            date_string = format!("{}{}", date_string, year_mod);
        } else {
            date_string = format!("{}{}{}", date_string, year_mod, seg_year_char);
        }
    }

    if !date_hour.is_empty() {
        date_string = format!("{} ", date_string);
    }

    for (usize, hour_mod) in date_hour.iter().enumerate() {
        if date_hour.len() == 1 {
            date_string = format!("{}{}", date_string, hour_mod);
            continue;
        };
        
        if usize == date_hour.len() -1 {
            date_string = format!("{}{}", date_string, hour_mod);
        } else {
            date_string = format!("{}{}{}", date_string, hour_mod, seg_hour_char);
        }
    }
    match moddate.seperator {
        DateSeperator::Asterisk => {
            date_string = format!("*{}*", date_string)
        },
        DateSeperator::Bracket => {
            date_string = format!("[{}]", date_string)
        },
        DateSeperator::Colan => {
            date_string = format!(":{}:", date_string)
        },
        DateSeperator::CurlyBracket => {
            date_string = format!("{{{}}}", date_string)
        },
        DateSeperator::Line => {
            date_string = format!("|{}|", date_string)
        },
        DateSeperator::Minus => {
            date_string = format!("-{}-", date_string)
        },
        DateSeperator::Parenthesis => {
            date_string = format!("({})", date_string)
        },
        DateSeperator::Plus => {
            date_string = format!("+{}+", date_string)
        },
        DateSeperator::Sign => {
            date_string = format!("<{}>", date_string)
        },
        DateSeperator::Space => {
            date_string = format!(" {} ", date_string)
        },
        DateSeperator::Underscore => {
            date_string = format!("_{}_", date_string)
        },
        _ => { }
    };

    match moddate.mode {
        DateMode::Prefix => {
            file = format!("{} {}", date_string, file);
        },
        DateMode::Suffix => {
            file = format!("{} {}", file, date_string);
        },
        DateMode::Insert => {
            let insert_at: u32;
            if moddate.at_pos == 0 {
                file = format!("{} {}", date_string, file);
            } else if moddate.at_pos == file.chars().count() as i32 {
                file = format!("{} {}", file, date_string);
            } else {
                if moddate.at_pos.is_negative() {
                    insert_at = (file.chars().count() as u32 + 1) - u32::try_from(moddate.at_pos * -1).unwrap().clamp(1, file.chars().count() as u32 + 1);
                } else {
                    insert_at = u32::try_from(moddate.at_pos.clone().clamp(0, file.chars().count() as i32)).unwrap();
                }
                let (start, end) = utils::split_uft8(&file, insert_at as usize);
                file = format!("{} {} {}", start, date_string, end);
            }
        },
        _ => {}
    };

    return (file, ext);
}

fn extension(file: String, mut ext: String, modext: ModExtension) -> (String, String) {
    match modext.mode {
        ExtensionMode::Extra => {
            ext = format!("{}{}", ext, modext.extra);
        },
        ExtensionMode::Fixed => {
            ext = modext.fixed;
        },
        ExtensionMode::Lower => {
            ext = ext.to_ascii_lowercase();
        },
        ExtensionMode::Remove => {
            ext = String::new();
        },
        ExtensionMode::Upper => {
            ext = ext.to_ascii_uppercase();
        },
        ExtensionMode::UpperFirst => {
            let start = ext.remove(1);
            ext.insert(1, start.to_ascii_uppercase());
        },
        _ => {} // Same do nothing.
    }
    (file, ext)
}

fn hash(mut file: String, ext: String, modhash: ModHashing, file_hash: String) -> (String, String) {
    let mut fake_hash: String = match file_hash.len() != 0 {
        true => {
            file_hash
        },
        false => {
            match modhash.algorithm {
                HashType::CRC32 => {
                    String::from("xxCRC32x")
                },
                HashType::MD5 => {
                    String::from("xxxxxxxxxxxxxxxMD5xxxxxxxxxxxxxx")
                },
                HashType::Sha1 => {
                    String::from("xxxxxxxxxxxxxxxxxxxSHA1xxxxxxxxxxxxxxxxx")
                },
                HashType::Sha256 => {
                    String::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxSHA256xxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                }
            }
        }
    };

    match modhash.seperator {
        HashSeperator::Asterisk => {
            fake_hash = format!("*{}*", fake_hash);
        },
        HashSeperator::Bracket => {
            fake_hash = format!("[{}]", fake_hash);
        },
        HashSeperator::Colan => {
            fake_hash = format!(":{}:", fake_hash);
        },
        HashSeperator::CurlyBracket => {
            fake_hash = format!("{{{}}}", fake_hash);
        },
        HashSeperator::Line => {
            fake_hash = format!("|{}|", fake_hash);
        },
        HashSeperator::Minus => {
            fake_hash = format!("-{}-", fake_hash);
        },
        HashSeperator::None => {},
        HashSeperator::Parenthesis => {
            fake_hash = format!("({})", fake_hash);
        },
        HashSeperator::Plus => {
            fake_hash = format!("+{}+", fake_hash);
        },
        HashSeperator::Sign => {
            fake_hash = format!("<{}>", fake_hash);
        },
        HashSeperator::Space => {
            fake_hash = format!(" {} ", fake_hash);
        },
        HashSeperator::Underscore => {
            fake_hash = format!("_{}_", fake_hash);
        }
    }

    match modhash.mode {
        HashMode::Prefix => {
            file = format!("{} {}", fake_hash, file);
        },
        HashMode::Suffix => {
            file = format!("{} {}", file, fake_hash);
        },
        HashMode::File => {} // Do nothing here.
        HashMode::None => {} // Do nothing here.
    }
    (file, ext)
}

fn movecopy(mut file: String, ext: String, modmovecopy: ModMoveCopy) -> (String, String) {
    let clamped_from: u32 = modmovecopy.letters_count.clamp(0, file.chars().count() as u32);
    let clamped_to: u32 = modmovecopy.mode_to_pos.clamp(0, file.chars().count() as u32);
    match modmovecopy.mode_from {
        MoveCopyFromMode::CopyFirstN => {
            match modmovecopy.mode_to {
                MoveCopyToMode::ToStart => {}, // Do nothing
                MoveCopyToMode::ToEnd => {
                    if file.chars().count() != 0 && clamped_from != 0 {
                        let first_n = utils::get_utf8_slice(&file, 0, clamped_from as usize, false);
                        file = format!("{} {}", file, first_n);
                    };
                }, 
                MoveCopyToMode::ToPos => {
                    if file.chars().count() != 0 {
                        if clamped_from != 0 {
                            let first_n: String = utils::get_utf8_slice(&file, 0, clamped_from as usize, false);
                            let (start, end) = utils::split_uft8(&file, clamped_to as usize);
                            if modmovecopy.seperator_enabled == true {
                                file = format!("{}{}{}{}{}", start, modmovecopy.seperator, first_n, modmovecopy.seperator, end);
                            } else {
                                file = format!("{}{}{}", start, first_n, end);
                            }
                        };
                    };
                }, 
                MoveCopyToMode::None => {} // Do nothing
            };
        },
        MoveCopyFromMode::CopyLastN => {
            match modmovecopy.mode_to {
                MoveCopyToMode::ToStart => {
                    if file.chars().count() != 0 && clamped_from != 0 {
                        let last_n = utils::get_utf8_slice(&file, file.chars().count() - clamped_from as usize, file.chars().count(), false);
                        file = format!("{} {}", last_n, file);
                    };
                },
                MoveCopyToMode::ToEnd => {}, // Do nothing 
                MoveCopyToMode::ToPos => {
                    if file.chars().count() != 0 && clamped_from != 0 {
                        let last_n = utils::get_utf8_slice(&file, file.chars().count() - clamped_from as usize, file.chars().count(), false);
                        let (start, end) = utils::split_uft8(&file, clamped_to as usize);
                        if modmovecopy.seperator_enabled == true {
                            file = format!("{}{}{}{}{}", start, modmovecopy.seperator, last_n, modmovecopy.seperator, end);
                        } else {
                            file = format!("{}{}{}", start, last_n, end);
                        };
                    };
                }, 
                MoveCopyToMode::None => {} // Do nothing
            };

        },
        MoveCopyFromMode::MoveFirstN => {
            match modmovecopy.mode_to {
                MoveCopyToMode::ToStart => {}, // Do nothing
                MoveCopyToMode::ToEnd => {
                    if file.chars().count() != 0 && clamped_from != 0 {
                        let first_n = utils::get_utf8_slice(&file, 0, clamped_from as usize, false);
                        file = file.chars().skip(clamped_from as usize).collect();
                        file = format!("{} {}", file, first_n);
                    };
                }, 
                MoveCopyToMode::ToPos => {
                    if file.chars().count() != 0 && clamped_from != 0 && clamped_to != file.chars().count() as u32 {
                        if clamped_from != 0 {
                            let first_n: String = utils::get_utf8_slice(&file, 0, clamped_from as usize, false);
                            file = file.chars().skip(clamped_from as usize).collect();
                            let (start, end) = utils::split_uft8(&file, clamped_to as usize);
                            if modmovecopy.seperator_enabled == true {
                                file = format!("{}{}{}{}{}", start, modmovecopy.seperator, first_n, modmovecopy.seperator, end);
                            } else {
                                file = format!("{}{}{}", start, first_n, end);
                            }
                        };
                    };
                }, 
                MoveCopyToMode::None => {} // Do nothing
            };

        },
        MoveCopyFromMode::MoveLastN => {
            match modmovecopy.mode_to {
                MoveCopyToMode::ToStart => {
                    if file.chars().count() != 0 && clamped_from != 0 {
                        let last_n = utils::get_utf8_slice(&file, file.chars().count() - clamped_from as usize, file.chars().count(), false);
                        file = file.chars().take(file.chars().count() - clamped_from as usize).collect();
                        file = format!("{} {}", last_n, file);
                    };
                },
                MoveCopyToMode::ToEnd => {}, // Do nothing 
                MoveCopyToMode::ToPos => {
                    if file.chars().count() != 0 && clamped_from != 0 && clamped_to != file.chars().count() as u32 {
                        let last_n = utils::get_utf8_slice(&file, file.chars().count() - clamped_from as usize, file.chars().count(), false);
                        file = file.chars().take(file.chars().count() - clamped_from as usize).collect();
                        let (start, end) = utils::split_uft8(&file, clamped_to as usize);
                        if modmovecopy.seperator_enabled == true {
                            file = format!("{}{}{}{}{}", start, modmovecopy.seperator, last_n, modmovecopy.seperator, end);
                        } else {
                            file = format!("{}{}{}", start, last_n, end);
                        };
                    };
                }, 
                MoveCopyToMode::None => {} // Do nothing
            };
        },
        MoveCopyFromMode::None => {} // Do nothing
    };
    (file, ext)
}

fn name(mut file: String, ext: String, modname: ModName) -> (String, String) {
    match modname.mode {
        NameMode::Remove => {
            file = String::new();
        },
        NameMode::Reverse => {
            let mut reversedvec: Vec<(usize, String)> = vec![];
            for (usize, char) in file.to_owned().chars().enumerate() {
                reversedvec.push((usize, char.to_string()));
            }
            reversedvec.reverse();
            
            let mut reversed: String = "".to_string();
            for (_usize, mut char) in reversedvec {
                match char.as_str() {
                    ">" => {char = "<".to_string()},
                    "<" => {char = ">".to_string()},
                    "(" => {char = ")".to_string()},
                    ")" => {char = "(".to_string()},
                    "{" => {char = "}".to_string()},
                    "}" => {char = "{".to_string()},
                    "[" => {char = "]".to_string()},
                    "]" => {char = "[".to_string()},
                    "/" => {char = "\\".to_string()},
                    "\\" => {char = "/".to_string()},
                    _ => {}
                }
                reversed = reversed + &char;
            };
            file = reversed;
        },
        NameMode::Fixed => {
            file = modname.fixed;
        },
        NameMode::Keep => {
            // Don't do anything
        }
    }
    (file, ext)
}

fn number(mut file: String, ext: String, modnumber: ModNumber, file_index: usize) -> (String, String) {
    let mut num_string: String;
    match modnumber.mode_type {
        NumberTypeMode::AlphaLower => {
            if modnumber.starting_num == 0 {
                num_string = alpha_counter::AlphaCounter::lower(modnumber.starting_num as usize + file_index).to_string()
            } else {
                num_string = alpha_counter::AlphaCounter::lower(modnumber.starting_num as usize + file_index - 1).to_string()
            }
            if num_string.chars().count() < modnumber.padding as usize {
                let padding: u32 = modnumber.padding - num_string.chars().count() as u32;
                for _ in 0..padding {
                    num_string.insert(0, 'a');
                };
            };
        },
        NumberTypeMode::AlphaLowerToUpper => {
            let alpha_lower_upper: String = ALPHA_LOWER_UPPER.iter().collect();
            if modnumber.starting_num == 0 {
                num_string = alpha_counter::AlphaCounter::custom(modnumber.starting_num as usize + file_index, alpha_lower_upper.as_str()).to_string()
            } else {
                num_string = alpha_counter::AlphaCounter::custom(modnumber.starting_num as usize + file_index - 1, alpha_lower_upper.as_str()).to_string()
            }
            if num_string.chars().count() < modnumber.padding as usize {
                let padding: u32 = modnumber.padding - num_string.chars().count() as u32;
                for _ in 0..padding {
                    num_string.insert(0, 'a');
                };
            };
        },
        NumberTypeMode::AlphaUpper => {
            if modnumber.starting_num == 0 {
                num_string = alpha_counter::AlphaCounter::upper(modnumber.starting_num as usize + file_index).to_string()
            } else {
                num_string = alpha_counter::AlphaCounter::upper(modnumber.starting_num as usize + file_index - 1).to_string()
            }
            if num_string.chars().count() < modnumber.padding as usize {
                let padding: u32 = modnumber.padding - num_string.chars().count() as u32;
                for _ in 0..padding {
                    num_string.insert(0, 'A');
                };
            };
        },
        NumberTypeMode::BaseTwo => {
            num_string = format!("{:b}", (modnumber.starting_num + file_index as u32) * modnumber.increment_num);
            if num_string.chars().count() < modnumber.padding as usize {
                let padding: u32 = modnumber.padding - num_string.chars().count() as u32;
                for _ in 0..padding {
                    num_string.insert(0, '0');
                };
            };
        },
        NumberTypeMode::BaseEight => {
            num_string = format!("{:o}", (modnumber.starting_num + file_index as u32)  * modnumber.increment_num);
            if num_string.chars().count() < modnumber.padding as usize {
                let padding: u32 = modnumber.padding - num_string.chars().count() as u32;
                for _ in 0..padding {
                    num_string.insert(0, '0');
                };
            };
        },
        NumberTypeMode::BaseTen => {
            num_string = format!("{}", (modnumber.starting_num + file_index as u32) * modnumber.increment_num);
            if num_string.chars().count() < modnumber.padding as usize {
                let padding: u32 = modnumber.padding - num_string.chars().count() as u32;
                for _ in 0..padding {
                    num_string.insert(0, '0');
                };
            };
        },
        NumberTypeMode::BaseSixteen => {
            num_string = format!("{:x}", (modnumber.starting_num + file_index as u32) * modnumber.increment_num);
            if num_string.chars().count() < modnumber.padding as usize {
                let padding: u32 = modnumber.padding - num_string.chars().count() as u32;
                for _ in 0..padding {
                    num_string.insert(0, '0');
                };
            };
        },
        NumberTypeMode::RomanNumeral => {
            if modnumber.starting_num != 0 {
                num_string = format!("{:X}", numerals::roman::Roman::from((modnumber.starting_num + file_index as u32) as i16))
            } else {
                num_string = String::new()
            }
        }
    };

    match modnumber.mode {
        NumberMode::Prefix => {
            if modnumber.seperator_enabled == true {
                file = format!("{}{}{}", num_string, modnumber.seperator, file);
            } else {
                file = format!("{}{}", num_string, file);
            }
        },
        NumberMode::Suffix => {
            if modnumber.seperator_enabled == true {
                file = format!("{}{}{}", file, modnumber.seperator, num_string);
            } else {
                file = format!("{}{}", file, num_string);
            }
        },
        NumberMode::PrefixAndSuffix => {
            if modnumber.seperator_enabled == true {
                file = format!("{}{}{}{}{}", num_string, modnumber.seperator, file, modnumber.seperator, num_string);
            } else {
                file = format!("{}{}{}", num_string, file, num_string);
            }
        },
        NumberMode::Insert => {
            let insert_at: u32;
            if modnumber.insert_at.is_negative() {
                insert_at = (file.chars().count() as u32 + 1) - u32::try_from(modnumber.insert_at * -1).unwrap().clamp(1, file.chars().count() as u32 + 1);
            } else {
                insert_at = u32::try_from(modnumber.insert_at.clone().clamp(0, file.chars().count() as i32)).unwrap();
            }
            let (start, end) = utils::split_uft8(&file, insert_at as usize);
            if modnumber.seperator_enabled == true {
                file = format!("{}{}{}{}{}", start, modnumber.seperator, num_string, modnumber.seperator, end);
            } else {
                file = format!("{}{}{}", start, num_string, end);
            }
        },
        NumberMode::None => {} // Do nothing
    }

    (file, ext)
}

fn regex(mut file: String, ext: String, modregex: ModRegex) -> (String, String) {
    let reg = Regex::new(&modregex.replace_match);
    if reg.is_ok() {
        let reg = reg.unwrap();
        file = reg.replace(&file, modregex.replace_with).to_string();
    };
    (file, ext)
}

fn remove(mut file: String, ext: String, modremove: ModRemove) -> (String, String) {
    let clamped_first = modremove.first_n.clamp(0, file.chars().count() as u32);
        
    // Remove some chars from the beginning
    if file.chars().count() >= 1{
        file = file.chars().skip(clamped_first as usize).collect();

    }
    
    // Remove some chars from the end
    let clamped_last = modremove.last_n.clamp(0, file.chars().count() as u32);
    if file.chars().count() >= 1 {
        //file = file[0..file.len() - clamped_last as usize].to_string();
        file = file.chars().take(file.char_indices().count() - clamped_last as usize).collect();
    }

    // Remove a section
    let clamped_from = modremove.from_x.clamp(0, file.chars().count() as u32);
    let clamped_to = modremove.to_y.clamp(clamped_from, file.chars().count() as u32);
    if clamped_to >= 1 && file.chars().count() >= 1{
        let mut file_vec: Vec<char> = vec![];
        let mut file_chars: String = "".to_string();
        for character in file.chars() {
            file_vec.push(character);
        }
        for character in &file_vec[0..clamped_from as usize] {
            file_chars.push(character.to_owned());
        }
        for character in &file_vec[clamped_to as usize..file_vec.len()] {
            file_chars.push(character.to_owned());
        }
        file = file_chars;
        
    }

    // Remove Chars, comma seperated
    let char_vec: Vec<&str> = modremove.chars_comma_seperated.split(',').collect();
    for c in char_vec {
        if c.chars().count() > 1 {
            continue;
        };
        file = file.replacen(&c, "", 100);
    };

    // Remove Words, comma seperated
    let word_vec: Vec<&str> = modremove.words_comma_seperated.split(',').collect();
    for word in word_vec {
        file = file.replacen(word, "", 100);
    };

    // Remove Crop
    match modremove.crop {
        RemoveCropMode::Before => {
            if modremove.crop_match.chars().count() >= 1 {
                let first = file.find(&modremove.crop_match);
                if first.is_some() {
                    let index = first.unwrap();
                    let (_, end) = file.split_at(index);
                    file = end.to_string();
                };
            };
        },
        RemoveCropMode::After => {
            if modremove.crop_match.chars().count() >= 1 {
                let first = file.find(&modremove.crop_match);
                if first.is_some() {
                    let index = first.unwrap();
                    if index != modremove.crop_match.chars().count() || index != 0 {
                        let (start, _) = file.split_at(index + 1);
                        file = start.to_string();
                    }
                };
            }
        },
        _ => {}
    };

    // Remove digits
    if modremove.digits == true {
        file = file.replacen(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], "", 255);
    };

    // Remove double-spaces
    if modremove.double_spaces == true {
        file = file.replacen("  ", " ", 255);
    };

    // Replace Accented Chars with non-accented chars
    if modremove.accented_chars == true {
        for (index, c) in ACCENTED_CHARS.iter().enumerate() {
            file = file.replacen(*c, &ACCENTED_CHARS_REPLACEMENT[index].to_string(), 255);
        };
    };

    // Remove Symbols (Special characters)
    if modremove.symbols == true {
        for (_, c) in SPECIAL_CHARS.iter().enumerate() {
            file = file.replacen(*c, "", 255);
        }
    };

    // Remove all dots at the beginning
    if modremove.leading_dots == true {
        while file.starts_with(&['.']) {
            file.remove(0);
        }
    }

    // Trim whitespace
    if modremove.trim == true {
        file = file.trim().to_string();
    }

    (file, ext)
}

fn replace(mut file: String, ext: String, modreplace: ModReplace) -> (String, String) {
    if modreplace.first_occurance && modreplace.replace_match.len() != 0 {
        file = file.replacen(&modreplace.replace_match, &modreplace.replace_with, 1);
    } else if modreplace.replace_match.len() != 0 {
        file = file.replacen(&modreplace.replace_match, &modreplace.replace_with, 255);
    }
    (file, ext)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        let (prefix, _) = super::add(String::from("Hello world"), 
            String::from(".ext"), 
            super::ModAdd { 
                prefix: String::from("Test - "), 
                insert: String::from(""), 
                insert_at: 0, 
                suffix: String::from(""), 
                seperator: ' ', 
                seperator_enabled: false 
            });
        let (suffix, _) = super::add(String::from("Hello world"), 
            String::from(".ext"), 
            super::ModAdd { 
                prefix: String::from(""), 
                insert: String::from(""), 
                insert_at: 0, 
                suffix: String::from(" - Test"), 
                seperator: ' ', 
                seperator_enabled: false 
            });
        let (insert, _) = super::add(String::from("Hello world"), 
            String::from(".ext"), 
            super::ModAdd { 
                prefix: String::from(""), 
                insert: String::from("Test"), 
                insert_at: 5, 
                suffix: String::from(""), 
                seperator: ' ', 
                seperator_enabled: false 
            });
        assert_eq!(prefix, String::from("Test - Hello world"));
        assert_eq!(suffix, String::from("Hello world - Test"));
        assert_eq!(insert, String::from("HelloTest world"));
    }

    fn test_date() {
        
    }


}