use std::collections::HashMap;

use crate::types::{AttributeType, DiceType};
use serde::{Deserialize, Serialize};

// Update CharacterData to use HashMap instead of Vec
#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterData {
    pub character_name: String,
    pub pronouns: String,
    pub backstory: String,
    pub notes: String,
    pub selected_dice: HashMap<AttributeType, Option<DiceType>>,
    pub damage: Vec<bool>,
    pub adversity_tokens: u32,
    pub allow_die_reuse: bool,
}
