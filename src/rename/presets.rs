use serde::{Deserialize, Serialize};
use super::mods::{Modifiers, ModsOrder};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Presets {
    pub sets: Vec<Preset>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    #[serde(default)]
    pub modifier_order: Vec<ModsOrder>,
    pub modifiers: Modifiers,
    pub include_files: bool,
    pub include_folders: bool,
    pub file_extension_filter: Vec<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PresetInclusion {
    Files,
    Folders,
}
