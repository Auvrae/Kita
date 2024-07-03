use super::mods::{Modifiers, ModsOrder};

#[derive(Clone, Debug)]
pub struct Presets {
    pub sets: Vec<Preset>
}

#[derive(Clone, Debug)]
pub struct Preset {
    pub name: String,
    pub modifier_order: ModsOrder,
    pub modifiers: Modifiers
}

