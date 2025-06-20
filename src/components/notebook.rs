use crate::types::CharacterState;
use leptos::prelude::*;

#[component]
pub fn Notebook(open: RwSignal<bool>, character_state: RwSignal<CharacterState>) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            <div class="notebook-modal">
                <div class="header-bar">
                    <p class="bold">"Notebook"</p>
                    <button on:click=move |_| open.set(false)>"X"</button>
                </div>
                <div class="section">
                    <label for="backstory" class="label">
                        "Backstory"
                    </label>
                    <textarea
                        id="backstory"
                        rows="10"
                        cols="50"
                        class="textarea"
                        prop:value=move || character_state.get().backstory.get()
                        on:input=move |ev| {
                            character_state.get().backstory.set(event_target_value(&ev))
                        }
                    />
                </div>
                <div>
                    <label for="notes" class="label">
                        "Notes"
                    </label>
                    <textarea
                        id="notes"
                        rows="10"
                        cols="50"
                        class="textarea"
                        prop:value=move || character_state.get().notes.get()
                        on:input=move |ev| character_state.get().notes.set(event_target_value(&ev))
                    />
                </div>
            </div>
        </Show>
    }
}
