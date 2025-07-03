use std::path::PathBuf;

use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ListFilesArgs<'a> {
    directory: &'a str,
}

#[derive(Serialize, Deserialize)]
struct ReadFileContentArgs<'a> {
    file_path: &'a str,
}

#[component]
pub fn ImportScreen() -> impl IntoView {
    let current_directory = RwSignal::new(String::from("Loading..."));
    let available_files: RwSignal<Vec<String>> =
        RwSignal::new(vec!["No files found...".to_string()]);

    spawn_local(async move {
        // Display the current directory below the heading.
        let new_directory = invoke("get_current_directory", JsValue::NULL)
            .await
            .as_string()
            .unwrap_or_else(|| "Failed to get directory.".to_string());
        current_directory.set(new_directory);
        // FIXME, im just manually setting the directory for now. In the future, the home directory should be changed in "Configuration".
        current_directory.set("C:/Users/TheoOdendaal/source/repos/no-eagles".to_string());

        // Available files.
        let directory = &current_directory.get_untracked();
        let args = serde_wasm_bindgen::to_value(&ListFilesArgs { directory }).unwrap();
        let new_available_files: JsValue = invoke("get_list_of_files", args).await;
        match from_value::<Vec<String>>(new_available_files) {
            Ok(data) => {
                available_files.set(data);
            }
            Err(e) => available_files.set(vec![e.to_string()]),
        }
    });

    // File name input box.
    let file_name: RwSignal<String> = RwSignal::new(String::new());

    let update_file_name = move |ev| {
        let v = event_target_value(&ev);
        file_name.set(v);
    };

    // File suggestions
    let filtered_files = Signal::derive(move || {
        let query = file_name.get().to_lowercase();
        available_files
            .get()
            .iter()
            .filter(|name| name.to_lowercase().contains(&query))
            .cloned()
            .collect::<Vec<_>>()
    });

    // Read current file_name.
    let file_content: RwSignal<Vec<Vec<String>>> = RwSignal::new(Vec::new());
    let selected_file: RwSignal<String> = RwSignal::new(String::new());
    let selected_file_exists: RwSignal<bool> = RwSignal::new(false);

    let update_file_content = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let file_path_buf = PathBuf::from(current_directory.get());
            let file_path_buf = file_path_buf.join(file_name.get());
            let file_path = &file_path_buf.to_string_lossy().to_string();
            selected_file.set(file_path.to_string());

            selected_file_exists.set(file_path_buf.exists());

            let args = serde_wasm_bindgen::to_value(&ReadFileContentArgs { file_path }).unwrap();
            let new_file_content: JsValue = invoke("read_csv_file", args).await;
            match from_value::<Vec<Vec<String>>>(new_file_content) {
                Ok(data) => {
                    file_content.set(data);
                }
                // FIXME, better error handling.
                Err(_) => file_content.set(Vec::new()),
            }
        });
    };

    view! {
            <div>
                <h2>"Import"</h2>
                <p> { move || current_directory.get().to_string() } </p>
            </div>


            <form on:submit=update_file_content>
                <input style="width: 300px;" placeholder="Enter relative path ..." on:input=update_file_name/>
                <button class="right-panel-button" type="submit">"Import"</button>
                <p> { move || file_name.get() } </p>
            </form>

            <p> {move || selected_file.get() } </p>
            <p> {move || selected_file_exists.get() } </p>

            <Show when=move || !file_name.get().is_empty()>
            <div class="suggestions-box">
                {move || {
                    filtered_files.get().iter().take(10).map(|item| {
                        view! {
                            <div class="suggestion-item">{item.clone()}</div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
            </Show>

            <div>
            {move || {
                    file_content.get().iter().map(|item| {
                        view! {
                            <div class="suggestion-item"> { item.join(" ").to_string()} </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

    }
}
