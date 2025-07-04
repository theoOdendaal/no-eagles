use leptos::ev::{KeyboardEvent, SubmitEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

use thiserror::Error;

use crate::app::AppState;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("Serde wasm bindgen error: {0}")]
    SerdeWasm(#[from] serde_wasm_bindgen::Error),
}

#[derive(Serialize, Deserialize)]
struct DirectoryArgs<'a> {
    directory: &'a str,
}

async fn fetch_current_directory() -> Result<String, ImportError> {
    /*
    let dir = invoke("get_current_directory", JsValue::NULL).await;
    let value = from_value::<String>(dir)?;
    Ok(value)
    */
    // FIXME, im just manually setting the directory for now. In the future, the home directory should be changed in "Configuration".
    Ok("C:/Users/TheoOdendaal/source/repos/no-eagles".to_string())
}

async fn fetch_eligible_files(directory: &str) -> Result<Vec<String>, ImportError> {
    let args = serde_wasm_bindgen::to_value(&DirectoryArgs { directory }).unwrap();
    let new_available_files: JsValue = invoke("get_list_of_files", args).await;
    let value = from_value::<Vec<String>>(new_available_files)?;
    Ok(value)
}

#[component]
pub fn ImportScreen() -> impl IntoView {
    let state = use_context::<AppState>().expect("Could not retrieve state.");

    // Signals
    // Directory within which files a looked for.
    let current_directory = RwSignal::new(String::from("Loading..."));

    // Files selected by the user.
    // Retrieve added files from state
    let added_files = state.added_files;

    // List of all eligible files.
    let eligible_files: RwSignal<Vec<String>> = RwSignal::new(vec!["Loading...".to_string()]);

    // User input
    let user_input_value: RwSignal<String> = RwSignal::new(String::new());

    spawn_local(async move {
        // Determine current directory.
        let dir = fetch_current_directory()
            .await
            .unwrap_or_else(|_| "Failed to retrieve current directory.".to_string());
        current_directory.set(dir);

        // Available files.
        let directory = &current_directory.get_untracked();
        let files = fetch_eligible_files(directory)
            .await
            .expect("Unable to retrieve files.");
        eligible_files.set(files);
    });

    // File name input box.
    let update_user_input_value = move |ev| {
        let v = event_target_value(&ev);
        user_input_value.set(v);
    };

    // File suggestions
    let filtered_files = Signal::derive(move || {
        let query = user_input_value.get().to_lowercase();
        eligible_files
            .get()
            .iter()
            .filter(|name| name.to_lowercase().contains(&query))
            .cloned()
            .collect::<Vec<_>>()
    });

    let tab_autocomplete = move |ev: KeyboardEvent| {
        if ev.key() == "Tab" {
            ev.prevent_default();
            ev.stop_propagation();
            if let Some(first_suggestion) = filtered_files.get().first() {
                user_input_value.set(first_suggestion.to_string())
            }
        }
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let current_input = user_input_value.get();
        let trimmed_input = current_input.trim().to_string();
        if trimmed_input.is_empty() {
            return;
        }

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&DirectoryArgs {
                directory: &trimmed_input,
            })
            .unwrap();

            let exists = invoke("validate_file_exists", args).await;

            if let Ok(value) = from_value::<bool>(exists) {
                if value {
                    let mut files = added_files.get_untracked();
                    if !files.contains(&trimmed_input) {
                        files.push(trimmed_input.clone());
                        added_files.set(files);
                    }
                    user_input_value.set(String::new());
                }
            }
        });
    };

    view! {
        <div class="import-wrapper">
            //<!-- Left side: form and suggestions -->
            <div class="import-left">
                <h2>"Import"</h2>
                <p>{ move || current_directory.get().to_string() }</p>

                <form on:submit=on_submit>
                    <input
                        style="width: 400px;"
                        placeholder="Enter relative path ..."
                        prop:value=user_input_value
                        on:input=update_user_input_value
                        on:keydown=tab_autocomplete
                    />
                    <button class="right-panel-button" type="submit">"Import"</button>
                </form>

                <div class="suggestions-box">
                    {move || {
                        filtered_files.get().iter().take(10).map(|item| {
                            view! {
                                <div class="suggestion-item">{item.clone()}</div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>

            //<!-- Right side: list of imported files -->
            <div class="import-right">
                <h3>"Imported Files"</h3>
                <ul>
                    {move || added_files.get().iter().map(|file| {
                        view! { <li>{file.clone()}</li> }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>

        </div>
    }
}
