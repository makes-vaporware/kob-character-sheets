// leptosfmt ./**/*.rs
mod components;
mod types;

use components::AttributeDisplay;
use components::Notebook;
use components::SheetSettings;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use types::{AttributeType, CharacterState};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Modals
    let notebook_open = RwSignal::new(false);
    let settings_open = RwSignal::new(false);

    // Character state
    let character_state = RwSignal::new(CharacterState::default());

    view! {
        <div class="character-sheet">
            <div class="header-bar">
                <div class="name-group">
                    <p class="inline character-name bold">
                        {move || character_state.get().character_name.get()}
                    </p>
                    <p class="inline pronouns">
                        {move || format!("({})", character_state.get().pronouns.get())}
                    </p>
                </div>
                <div class="button-group">
                    <button on:click=move |_| notebook_open.set(true)>"📖"</button>
                    <button on:click=move |_| settings_open.set(true)>"⚙️"</button>
                </div>
            </div>

            // Stats
            <div class="stats-grid">
                <div>
                    <AttributeDisplay
                        attribute=AttributeType::Brains
                        character_state=character_state
                    />
                    <AttributeDisplay
                        attribute=AttributeType::Fight
                        character_state=character_state
                    />
                    <AttributeDisplay
                        attribute=AttributeType::Charm
                        character_state=character_state
                    />
                </div>
                <div>
                    <AttributeDisplay
                        attribute=AttributeType::Brawn
                        character_state=character_state
                    />
                    <AttributeDisplay
                        attribute=AttributeType::Flight
                        character_state=character_state
                    />
                    <AttributeDisplay
                        attribute=AttributeType::Grit
                        character_state=character_state
                    />
                </div>
            </div>
            <div>
                <p class="inline bold">"Damage:"</p>
                <div class="inline">
                    {(0..4)
                        .map(|i| {
                            view! {
                                <input
                                    type="checkbox"
                                    checked=move || character_state.get().damage.get()[i]
                                    on:change=move |ev| {
                                        let mut current_damage = character_state.get().damage.get();
                                        current_damage[i] = event_target_checked(&ev);
                                        character_state.get().damage.set(current_damage);
                                    }
                                />
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
                <p class="inline adversity-label bold">"Adversity Tokens:"</p>
                <input
                    type="number"
                    maxlength="3"
                    min="0"
                    max="999"
                    prop:value=move || character_state.get().adversity_tokens.get().to_string()
                    on:input=move |ev| {
                        if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                            character_state.get().adversity_tokens.set(value);
                        }
                    }
                    class="adversity-tokens-input"
                />
            </div>
        </div>
        <SheetSettings open=settings_open character_state=character_state />
        <Notebook open=notebook_open character_state=character_state />
    }
}
