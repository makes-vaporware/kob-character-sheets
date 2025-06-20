use crate::types::{AttributeType, CharacterState, DiceType};
use leptos::prelude::*;
use std::collections::HashSet;

#[component]
pub fn AttributeSelector(
    attribute: AttributeType,
    character_state: RwSignal<CharacterState>,
) -> impl IntoView {
    // When updating attribute die
    let on_dice_change = move |e| {
        let value = event_target_value(&e);
        let new_dice = if value.is_empty() {
            None
        } else {
            DiceType::from_string(&value)
        };

        let state = character_state.get();
        state.selected_dice.update(|dice| {
            dice.insert(attribute, new_dice);
        });

        // Clear roll result when die updates
        state.roll_results.update(|results| {
            results.insert(attribute, None);
        });
    };

    let available_dice = Memo::new(move |_| {
        let state = character_state.get();
        let current_dice = state.selected_dice.get();
        let allow_reuse_val = state.allow_die_reuse.get();

        if allow_reuse_val {
            DiceType::all()
        } else {
            let used_dice: HashSet<DiceType> = current_dice
                .iter()
                .filter_map(|(&attr, &dice)| if attr != attribute { dice } else { None })
                .collect();
            DiceType::all()
                .into_iter()
                .filter(|dice| !used_dice.contains(dice))
                .collect()
        }
    });

    view! {
        <div class="attribute-row">
            <label class="attribute-label">{attribute.display_name()}</label>
            <select on:change=on_dice_change class="dice-select">
                <option value="">"Select"</option>
                <For
                    each=move || available_dice.get().into_iter()
                    key=|die| die.to_string()
                    children=move |die| {
                        view! {
                            <option
                                value=die.to_string()
                                prop:selected=move || {
                                    let state = character_state.get();
                                    let current_dice = state.selected_dice.get();
                                    current_dice[&attribute] == Some(die)
                                }
                            >
                                {die.to_string()}
                            </option>
                        }
                    }
                />
            </select>
        </div>
    }
}
