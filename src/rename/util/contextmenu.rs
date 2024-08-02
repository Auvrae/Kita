//use std::fs;
#[cfg(target_os="windows")]
use winreg;

// Checks if the entries are already installed.
pub fn check_registry() -> Option<()> {
    #[cfg(target_os="windows")]
    {
        let khcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
        let background_path = std::path::Path::new("SOFTWARE\\Classes\\Directory\\Background\\shell").join("Kita");
        let folder_path = std::path::Path::new("SOFTWARE\\Classes\\Directory\\shell").join("Kita");
        match khcu.open_subkey(background_path) {
            Ok(_) => {},
            Err(_) => {
                return None;
            }
        }
        match khcu.open_subkey(folder_path) {
            Ok(_) => {},
            Err(_) => {
                return None;
            }
        }
        return Some(());
    }
    None
}

// Installs entries for the Windows File Explorer Context Menu
pub fn install_registry(kita_path: String) {
    let path_background: String = format!("\"{}\" -o \"%V\"", kita_path.to_owned());
    let path_folder: String = format!("\"{}\" -o \"%1\"", kita_path.to_owned());
    #[cfg(target_os="windows")]
    {
        // Background
        {
            //Computer\HKEY_CURRENT_USER\SOFTWARE\Classes\Directory\Background\shell
            let khcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
            let root_path = std::path::Path::new("SOFTWARE\\Classes\\Directory\\Background\\shell").join("Kita");
            let command_path = std::path::Path::new("SOFTWARE\\Classes\\Directory\\Background\\shell\\Kita").join("command");
            let (root_key, _disp) = khcu.create_subkey(&root_path).unwrap();
            let (command_key, _disp) = khcu.create_subkey(&command_path).unwrap();
            root_key.set_value("", &"Kita &Rename &Here").unwrap();
            command_key.set_value("", &path_background).unwrap();
        }
        // Folders
        {
            //Computer\HKEY_CURRENT_USER\SOFTWARE\Classes\Directory\shell
            let khcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
            let root_path = std::path::Path::new("SOFTWARE\\Classes\\Directory\\shell").join("Kita");
            let command_path = std::path::Path::new("SOFTWARE\\Classes\\Directory\\shell\\Kita").join("command");
            let (root_key, _disp) = khcu.create_subkey(&root_path).unwrap();
            let (command_key, _disp) = khcu.create_subkey(&command_path).unwrap();
            root_key.set_value("", &"Kita &Rename &Here").unwrap();
            command_key.set_value("", &path_folder).unwrap();
        }
    }
}

// Uninstalls all entries in the registry.
pub fn uninstall_registry() {
    #[cfg(target_os="windows")]
    {   
        // Background
        {
            let khcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
            let root_path = std::path::Path::new("SOFTWARE\\Classes\\Directory\\Background\\shell").join("Kita");
            khcu.delete_subkey_all(root_path).unwrap();
        }
        // Folders
        {
            let khcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
            let root_path = std::path::Path::new("SOFTWARE\\Classes\\Directory\\shell").join("Kita");
            khcu.delete_subkey_all(root_path).unwrap();
        }
    }
}