use crate::types::{AttributeType, CharacterState, RollResult};
use leptos::prelude::*;
use rand::Rng;

#[component]
pub fn AttributeDisplay(
    attribute: AttributeType,
    character_state: RwSignal<CharacterState>,
) -> impl IntoView {
    // Dice rolling function
    let roll_dice = move |_| {
        let state = character_state.get();
        if let Some(dice) = state.selected_dice.get().get(&attribute).and_then(|d| *d) {
            let mut rng = rand::thread_rng();
            let mut rolls = Vec::new();
            let max_value = dice.sides();

            // Dice explosion logic
            loop {
                let roll = rng.gen_range(1..=dice.sides());
                rolls.push(roll);

                if roll != max_value {
                    break;
                }
            }

            let total = rolls.iter().sum();

            state.roll_results.update(|results| {
                results.insert(attribute, Some(RollResult { rolls, total }));
            });
        }
    };

    view! {
        <div class="attribute-row">
            <label class="attribute-label">{attribute.display_name()}</label>
            <p>
                {move || {
                    let state = character_state.get();
                    state
                        .selected_dice
                        .get()
                        .get(&attribute)
                        .and_then(|d| d.map(|dice| dice.to_string()))
                        .unwrap_or_else(|| "Select an option!")
                }}
            </p>
            <button
                on:click=roll_dice
                disabled=move || {
                    let state = character_state.get();
                    state.selected_dice.get().get(&attribute).and_then(|d| *d).is_none()
                }
                class="roll-button"
            >
                "Roll"
            </button>
            {move || {
                let state = character_state.get();
                if let Some(Some(result)) = state.roll_results.get().get(&attribute) {
                    let calculation = if result.rolls.len() == 1 {
                        String::new()
                    } else {
                        let formatted_rolls: Vec<String> = result
                            .rolls
                            .iter()
                            .map(|&roll| roll.to_string())
                            .collect();
                        formatted_rolls.join(" + ")
                    };

                    view! {
                        <span class="result">
                            <b>"Result: "</b>{calculation}<b>{result.total}</b>
                        </span>
                    }.into_any()
                } else {
                    view! {
                        <span class="result"></span>
                    }.into_any()
                }
            }}
        </div>
    }
}
