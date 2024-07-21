use std::fs;
use std::io;
use serde::{Deserialize, Serialize};

pub fn get_folder(path: String, _ignore_hidden: bool) -> io::Result<Folder> {
    let mut f = Folder {
        path: String::new(),
        list_files: vec![],
        list_folders: vec![],
        selected_files: vec![],
        selected_folders: vec![],
        errored: None,
        errored_message: None
    };
    match fs::read_dir(path.to_owned()) {
        Ok(directory) => {
            for (_, item) in directory.into_iter().enumerate() {
                if let Some(i) = item.ok() {
                    let ftype = i.file_type().unwrap();
                    if ftype.is_dir() {
                        f.list_folders.push(
                            FolderItem {
                                name: String::from(i.file_name().to_str().unwrap()),
                                name_modified: String::from(i.file_name().to_str().unwrap()),
                                path: String::from(i.path().to_str().unwrap()),
                                path_plain: path.to_owned(),
                                hash: String::new(),
                                error: String::new(),
                                errored: false
                            }
                        );
                        f.selected_folders.push(false);
                    } else if ftype.is_file() {
                        f.list_files.push(
                            FolderItem {
                                name: String::from(i.file_name().to_str().unwrap()),
                                name_modified: String::from(i.file_name().to_str().unwrap()),
                                path: String::from(i.path().to_str().unwrap()),
                                path_plain: path.to_owned(),
                                hash: String::new(),
                                error: String::new(),
                                errored: false
                            }
                        );
                        f.selected_files.push(false);
                    };
                };
            };
            f.errored = None;
            f.errored_message = None;
            f.path = path;
            f.list_files.sort();
            f.list_folders.sort();
            return Ok(f)
        },
        Err(err) => {
            return Err(err);
        }
    };
}

pub fn _read_folder(path: String) -> io::Result<Vec<String>> {
    let mut files: Vec<String> = vec![];
    match fs::read_dir(path.to_owned()) {
        Ok(directory) => {
            for (_, item) in directory.into_iter().enumerate() {
                if let Some(i) = item.ok() {
                    let ftype = i.file_type().unwrap();if ftype.is_file() {
                        files.push(String::from(i.file_name().to_str().unwrap()));
                    };
                };
            };
            return Ok(files);
        },
        Err(err) => {
            return Err(err);
        }
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Folder {
    pub path: String,
    pub errored: Option<bool>,
    pub errored_message: Option<String>,
    pub list_files: Vec<FolderItem>,
    pub selected_files: Vec<bool>,
    pub list_folders: Vec<FolderItem>,
    pub selected_folders: Vec<bool>
}

/// Path excludes file / folder name
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct FolderItem {
    pub name: String,
    pub name_modified: String,
    pub path: String,
    pub path_plain: String,
    pub hash: String,
    pub errored: bool,
    pub error: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EdittedItem {
    pub name_original: String,
    pub name_edited: String,
    pub path_original: String,
    pub path_edited: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edit {
    pub tag: String,
    pub items: Vec<EdittedItem>,
    pub edits: u32,
}