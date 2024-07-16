use serde::{Deserialize, Serialize};
use super::mods::{Modifiers, ModsOrder};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Presets {
    pub sets: Vec<Preset>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub modifier_order: Vec<ModsOrder>,
    pub modifiers: Modifiers
}

