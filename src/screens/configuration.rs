use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
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
pub enum ConfigurationError {
    #[error("Serde wasm bindgen error: {0}")]
    SerdeWasm(#[from] serde_wasm_bindgen::Error),
}

#[derive(Serialize, Deserialize)]
struct DirectoryArgs<'a> {
    directory: &'a str,
}

async fn load_file_content(directory: &str) -> Result<Vec<Vec<String>>, ConfigurationError> {
    let args = serde_wasm_bindgen::to_value(&DirectoryArgs { directory })
        .map_err(ConfigurationError::SerdeWasm)?;

    let new_content: JsValue = invoke("read_csv_file", args).await;

    let result: Vec<Vec<String>> = from_value(new_content)?;
    Ok(result)
}

#[component]
pub fn ConfigurationScreen() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    // Optional: track the selected file
    let selected_file = RwSignal::new(String::new());
    let file_content: RwSignal<Vec<Vec<String>>> = RwSignal::new(Vec::new());

    let on_change = {
        move |ev| {
            let value = event_target_value(&ev);
            selected_file.set(value.clone());
        }
    };

    let update_file_content = {
        move |ev: SubmitEvent| {
            ev.prevent_default();

            let selected_file = selected_file.get();

            spawn_local(async move {
                match load_file_content(&selected_file).await {
                    Ok(data) => file_content.set(data),
                    Err(_) => file_content.set(vec![vec!["Failed to load file.".to_string()]]),
                }
            });
        }
    };

    view! {
        <div class="configuration-screen">
            <h2>"Configuration"</h2>

            <label for="file-select">"Choose a file:"</label>
            <select id="file-select" on:change=on_change>
                <option value="">"-- Select a file --"</option>
                {move || state.added_files.get().iter().map(|file| {
                    view! {
                        <option value={file.clone()}>{file.clone()}</option>
                    }
                }).collect::<Vec<_>>()}
            </select>

            <p>"Selected file: " {move || selected_file.get()}</p>

            <form on:submit=update_file_content>
                <button type="submit" class="menu-button">"Load"</button>
            </form>
        </div>

        <div class="table-container">
            {move || {
                let content = file_content.get();
                if content.is_empty() {
                    view! { <p>"No file loaded."</p> }.into_any()
                } else {
                    view! {
                        <table class="file-table">
                            <tbody>
                                {content.iter().map(|row| {
                                    view! {
                                        <tr>
                                            {row.iter().map(|cell| view! {
                                                <td>{cell.to_string()}</td>
                                            }).collect::<Vec<_>>()}
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    }.into_any()
                }
            }}
        </div>
    }
}
