use leptos::ev::{KeyboardEvent, SubmitEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;

use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

use thiserror::Error;

use crate::app::{DirectoryArgs, FileConfiguration};

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

async fn fetch_current_directory() -> Result<String, ImportError> {
    /*
    let dir = invoke("get_current_directory", JsValue::NULL).await;
    let value = from_value::<String>(dir)?;
    Ok(value)
    */
    // FIXME, im just manually setting the directory for now. In the future, the home directory can be changed by the user.
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
    //Auto focus on the input box when screen is mounted.
    let input_ref: NodeRef<leptos::html::Input> = NodeRef::new();
    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            input.focus().unwrap();
        }
    });

    let state = use_context::<FileConfiguration>().expect("Could not retrieve state.");

    // Signals
    // Directory within which files a looked for.
    let current_directory = RwSignal::new(String::from("Loading..."));

    // Files selected by the user.
    // Retrieve added files from state
    let lazy_files = state.lazy_files;

    // List of all eligible files.
    let eligible_files: RwSignal<Vec<String>> = RwSignal::new(vec!["Loading...".to_string()]);

    // Index selected from suggestions
    let selected_index: RwSignal<Option<usize>> = RwSignal::new(None);

    // User input
    let user_input_value: RwSignal<String> = RwSignal::new(String::new());

    spawn_local(async move {
        // Determine current directory.
        let dir = fetch_current_directory()
            .await
            .unwrap_or_else(|_| "Failed to retrieve current directory.".to_string());
        current_directory.set(dir);

        // Available files.
        let directory = &current_directory.get();
        let files = fetch_eligible_files(directory)
            .await
            .expect("Unable to retrieve files.");
        eligible_files.set(files);
    });

    // File name input box.
    let update_user_input_value = move |ev| {
        let v = event_target_value(&ev);
        user_input_value.set(v);
        selected_index.set(None);
    };

    // File suggestions
    // FIXME Added take(10) here? Rather than in the show logic? Should speed-up?
    let filtered_files = Signal::derive(move || {
        let query = user_input_value.get().to_lowercase();
        eligible_files
            .get()
            .iter()
            .filter(|name| name.to_lowercase().contains(&query))
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
    });

    let keyboard_navigation = move |ev: KeyboardEvent| {
        let key = ev.key();

        let total_selections = filtered_files.get().len();
        let current_selection = selected_index.get();

        if key == "ArrowRight" {
            ev.prevent_default();
            ev.stop_propagation();

            if let Some(index) = current_selection {
                if let Some(selection) = filtered_files.get().get(index) {
                    user_input_value.set(selection.to_string())
                }
            }
        }

        if key == "ArrowDown" {
            ev.prevent_default();
            ev.stop_propagation();
            let new_index = Some(match current_selection {
                Some(i) => (i + 1) % total_selections,
                None => 0,
            });
            selected_index.set(new_index);
        }

        if key == "ArrowUp" {
            ev.prevent_default();
            ev.stop_propagation();
            let new_index = Some(match current_selection {
                Some(0) | None => total_selections.saturating_sub(1),
                Some(i) => (i - 1) % total_selections,
            });
            selected_index.set(new_index);
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
                    let mut files = lazy_files.get();
                    if !files.contains(&trimmed_input) {
                        files.push(trimmed_input.clone());
                        lazy_files.set(files);
                    }
                    user_input_value.set(String::new());
                }
            }
        });
    };

    view! {
        <div>
            //<!-- Left side: form and suggestions -->
            <div>
                <h2>"Import"</h2>
                <p>"Starting typing to update the suggestions."</p>
                <p>"Use the up and down arrows to navigate the suggestions, and the right arrow to select a suggestion."</p>

                <form on:submit=on_submit>
                    <input
                        class="styled-input"
                        node_ref=input_ref
                        placeholder="Enter file name ..."
                        prop:value=user_input_value
                        on:input=update_user_input_value
                        on:keydown=keyboard_navigation
                    />
                    <button class="menu-button" type="submit">"Load file"</button>
                </form>

                <div class="suggestions-box">

                    {move || {
                        let filtered = filtered_files.get();
                        let items = filtered.iter().enumerate().map(|(i, item)| {
                            let is_selected = selected_index.get() == Some(i);
                            view! {
                                <div class="suggestion-item" class:selected={is_selected}>{item.clone()}</div>
                            }
                        }).collect::<Vec<_>>();

                        if items.is_empty() {
                            view! {
                                <div class="suggestion-item" style="opacity: 0.5;">"No matches found."</div>
                            }.into_any()
                        } else {
                            view! { <>{items}</> }.into_any()
                        }
                    }}

                </div>
            </div>

            //<!-- Right side: list of imported files -->
            <div>
                <h3>"Imported Files"</h3>

                <div class="imported-file-list">
                    {move || lazy_files.get().iter().enumerate().map(|(index, file)| {
                        let file_name = file.clone();
                        let on_delete = {
                            move |_| {
                                let mut files = lazy_files.get();
                                    files.remove(index);
                                    lazy_files.set(files);
                            }
                        };

                        view! {
                            <div class="imported-file-item">
                                <span class="file-name">{file_name}</span>
                                <button class="delete-button" title="Remove file" on:click=on_delete> "remove" </button>
                                //"❌"
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
