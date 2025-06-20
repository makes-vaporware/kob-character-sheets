use crate::types::{AttributeType, CharacterData, DiceType, RollResult};
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CharacterState {
    // Basic character info
    pub character_name: RwSignal<String>,
    pub pronouns: RwSignal<String>,

    // Notebook
    pub backstory: RwSignal<String>,
    pub notes: RwSignal<String>,

    // Attribute data
    pub selected_dice: RwSignal<HashMap<AttributeType, Option<DiceType>>>,
    pub roll_results: RwSignal<HashMap<AttributeType, Option<RollResult>>>,

    // Damage and adversity tokens
    pub damage: RwSignal<Vec<bool>>,
    pub adversity_tokens: RwSignal<u32>,

    // Die reuse setting
    pub allow_die_reuse: RwSignal<bool>,
}

impl Default for CharacterState {
    fn default() -> Self {
        Self {
            character_name: RwSignal::new("Jasper".to_string()),
            pronouns: RwSignal::new("he/him".to_string()),
            backstory: RwSignal::new(String::new()),
            notes: RwSignal::new(String::new()),
            selected_dice: RwSignal::new(HashMap::from_iter(
                AttributeType::all().into_iter().map(|attr| (attr, None)),
            )),
            roll_results: RwSignal::new(HashMap::from_iter(
                AttributeType::all().into_iter().map(|attr| (attr, None)),
            )),
            damage: RwSignal::new(vec![false; 4]),
            adversity_tokens: RwSignal::new(0u32),
            allow_die_reuse: RwSignal::new(false),
        }
    }
}

impl CharacterState {
    pub fn from_character_data(data: CharacterData) -> Self {
        let all_attributes = AttributeType::all();
        let mut selected_dice = data.selected_dice;
        let mut roll_results = HashMap::new();

        // Ensure all attributes are present
        for attr in all_attributes {
            selected_dice.entry(attr).or_insert(None);
            roll_results.insert(attr, None);
        }

        Self {
            character_name: RwSignal::new(data.character_name),
            pronouns: RwSignal::new(data.pronouns),
            backstory: RwSignal::new(data.backstory),
            notes: RwSignal::new(data.notes),
            selected_dice: RwSignal::new(selected_dice),
            roll_results: RwSignal::new(roll_results),
            damage: RwSignal::new(data.damage),
            adversity_tokens: RwSignal::new(data.adversity_tokens),
            allow_die_reuse: RwSignal::new(data.allow_die_reuse),
        }
    }

    pub fn to_character_data(&self) -> CharacterData {
        CharacterData {
            character_name: self.character_name.get(),
            pronouns: self.pronouns.get(),
            backstory: self.backstory.get(),
            notes: self.notes.get(),
            selected_dice: self.selected_dice.get(),
            damage: self.damage.get(),
            adversity_tokens: self.adversity_tokens.get(),
            allow_die_reuse: self.allow_die_reuse.get(),
        }
    }
}
