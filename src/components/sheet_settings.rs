use crate::components::AttributeSelector;
use crate::types::{AttributeType, CharacterData, CharacterState};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, Blob, FileReader, HtmlAnchorElement, HtmlInputElement, Url};

#[component]
pub fn SheetSettings(
    open: RwSignal<bool>,
    character_state: RwSignal<CharacterState>,
) -> impl IntoView {
    // State for JSON import/export
    let file_input_ref = NodeRef::<leptos::html::Input>::new();
    let show_load_confirm = RwSignal::new(false);
    let pending_data = RwSignal::new(None::<CharacterData>);

    // Function to collect current character data
    let collect_character_data =
        move || -> CharacterData { character_state.get().to_character_data() };

    // Function to apply character data
    let apply_character_data = move |data: CharacterData| {
        let new_state = CharacterState::from_character_data(data);
        character_state.set(new_state);
    };

    // Save JSON function
    let save_json = move |_| {
        let data = collect_character_data();
        if let Ok(json_string) = serde_json::to_string_pretty(&data) {
            if let Some(window) = window() {
                if let Ok(blob) =
                    Blob::new_with_str_sequence(&js_sys::Array::of1(&json_string.into()))
                {
                    if let Ok(url) = Url::create_object_url_with_blob(&blob) {
                        if let Some(document) = window.document() {
                            if let Ok(anchor) = document.create_element("a") {
                                let anchor: HtmlAnchorElement = anchor.unchecked_into();
                                anchor.set_href(&url);
                                anchor.set_download(&format!(
                                    "{}_character.json",
                                    data.character_name.replace(" ", "_")
                                ));
                                anchor.click();
                                let _ = Url::revoke_object_url(&url);
                            }
                        }
                    }
                }
            }
        }
    };

    // Load JSON function
    let load_json = move |_| {
        if let Some(input) = file_input_ref.get() {
            let input_element: HtmlInputElement = input.clone().unchecked_into();
            input_element.click();
        }
    };

    // Handle file selection
    let handle_file_change = move |_| {
        if let Some(input) = file_input_ref.get() {
            let input_element: HtmlInputElement = input.clone().unchecked_into();
            if let Some(files) = input_element.files() {
                if let Some(file) = files.get(0) {
                    let file_reader = FileReader::new().unwrap();
                    let file_reader_clone = file_reader.clone();

                    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        if let Ok(result) = file_reader_clone.result() {
                            if let Some(text) = result.as_string() {
                                if let Ok(data) = serde_json::from_str::<CharacterData>(&text) {
                                    pending_data.set(Some(data));
                                    show_load_confirm.set(true);
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    let _ = file_reader.read_as_text(&file);
                    onload.forget();
                }
            }
        }
    };

    // Confirm load function
    let confirm_load = move |_| {
        if let Some(data) = pending_data.get() {
            apply_character_data(data);
        }
        show_load_confirm.set(false);
        pending_data.set(None);
    };

    // Cancel load function
    let cancel_load = move |_| {
        show_load_confirm.set(false);
        pending_data.set(None);
    };

    view! {
        <Show when=move || open.get()>
            <div class="settings-modal">
                <div class="header-bar">
                    <p class="bold">"Settings"</p>
                    <button on:click=move |_| open.set(false)>"X"</button>
                </div>

                <div class="section">
                    <p>"Character:"</p>
                    <input
                        type="text"
                        prop:value=move || character_state.get().character_name.get()
                        on:input=move |ev| {
                            character_state.get().character_name.set(event_target_value(&ev))
                        }
                    />
                    <br />
                    <p>"Pronouns:"</p>
                    <input
                        type="text"
                        prop:value=move || character_state.get().pronouns.get()
                        on:input=move |ev| {
                            character_state.get().pronouns.set(event_target_value(&ev))
                        }
                    />
                </div>

                <div class="section">
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            checked=move || character_state.get().allow_die_reuse.get()
                            on:change=move |ev| {
                                character_state
                                    .get()
                                    .allow_die_reuse
                                    .set(event_target_checked(&ev));
                            }
                        />
                        <span>"Allow reusing the same die type on different stats"</span>
                    </label>
                </div>

                <div class="stats-grid">
                    <div>
                        <AttributeSelector
                            attribute=AttributeType::Brains
                            character_state=character_state
                        />
                        <AttributeSelector
                            attribute=AttributeType::Fight
                            character_state=character_state
                        />
                        <AttributeSelector
                            attribute=AttributeType::Charm
                            character_state=character_state
                        />
                    </div>
                    <div>
                        <AttributeSelector
                            attribute=AttributeType::Brawn
                            character_state=character_state
                        />
                        <AttributeSelector
                            attribute=AttributeType::Flight
                            character_state=character_state
                        />
                        <AttributeSelector
                            attribute=AttributeType::Grit
                            character_state=character_state
                        />
                    </div>
                </div>

                // Thin divider
                <div style="border-top: 1px solid #ccc; margin: 20px 0;"></div>

                // JSON Import/Export section
                <div class="section">
                    <p class="bold">"Character Data"</p>
                    <div style="display: flex; gap: 10px; margin-top: 10px;">
                        <button on:click=save_json>"Save JSON"</button>
                        <button on:click=load_json>"Load JSON"</button>
                    </div>

                    // Hidden file input
                    <input
                        type="file"
                        accept=".json"
                        style="display: none;"
                        node_ref=file_input_ref
                        on:change=handle_file_change
                    />
                </div>
            </div>
        </Show>

        // Load confirmation modal
        <Show when=move || show_load_confirm.get()>
            <div class="settings-modal" style="z-index: 1001;">
                <div class="header-bar">
                    <p class="bold">"Confirm Load"</p>
                </div>
                <div class="section">
                    <p>"This will replace all current character data. Are you sure?"</p>
                    <div style="display: flex; gap: 10px; margin-top: 15px;">
                        <button on:click=confirm_load>"Confirm"</button>
                        <button on:click=cancel_load>"Cancel"</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
