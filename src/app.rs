use leptos::logging::log;
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

#[component]
pub fn App() -> impl IntoView {
    let current_screen = RwSignal::new("home".to_string());
    view! {
        <div class="app-container">
            <div class="left-panel">
                <button class="menu-button" on:click=move |_| current_screen.set("home".to_string())>"Home"</button>
                <button class="menu-button" on:click=move |_| current_screen.set("configuration".to_string())>"Configuration"</button>
                <button class="menu-button" on:click=move |_| current_screen.set("import".to_string())>"Import"</button>
            </div>

            <div class="right-panel">
                {
                    move || {
                        match current_screen.get().as_str() {
                            "home" => view! { <HomeScreen /> }.into_any(),
                            "configuration" => view! { <ConfigurationScreen /> }.into_any(),
                            "import" => view! { <ImportScreen /> }.into_any(),
                            _ => view! { <NotFoundScreen /> }.into_any(),
                        }
                    }
                }
            </div>
        </div>
    }
}

#[component]
pub fn HomeScreen() -> impl IntoView {
    view! {
    <div>
        <h2>"Home"</h2>
    </div> }
}

#[component]
pub fn ConfigurationScreen() -> impl IntoView {
    view! { <div><h2>"Configuration"</h2></div> }
}

#[derive(Serialize, Deserialize)]
struct FileImportArgs<'a> {
    file_name: &'a str,
}

#[component]
pub fn ImportScreen() -> impl IntoView {
    /*
    let file_name = RwSignal::new(String::new());
    let csv_data: RwSignal<Vec<Vec<String>>> = RwSignal::new(Vec::new());
    let error_msg: RwSignal<Option<String>> = RwSignal::new(None);
    log!("Testing 123");

    let update_file_name = move |ev| {
        let v = event_target_value(&ev);
        log!("Input value: {v}");
        file_name.set(v);
    };

    let read_file = move |ev: SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            let untracked_file_name = file_name.get_untracked();
            if untracked_file_name.is_empty() {
                return;
            }

            let args = match serde_wasm_bindgen::to_value(&FileImportArgs {
                file_name: &untracked_file_name,
            }) {
                Ok(val) => val,
                Err(err) => {
                    error_msg.set(Some(format!("Error serializing arguments: {err}")));
                    return;
                }
            };

            let result: JsValue = invoke("read_csv_file", args).await;
            match from_value::<Vec<Vec<String>>>(result) {
                Ok(data) => {
                    csv_data.set(data);
                    error_msg.set(None);
                }
                Err(err) => {
                    error_msg.set(Some(format!("Error parsing result: {err}")));
                }
            }
        });
    };
    */

    // Display the current directory below the heading.
    let current_directory = RwSignal::new(String::from("Loading..."));
    spawn_local(async move {
        let new_directory = invoke("get_current_directory", JsValue::NULL)
            .await
            .as_string()
            .unwrap();
        current_directory.set(new_directory);
    });

    // File name input box.
    let file_name: RwSignal<String> = RwSignal::new(String::new());

    let update_file_name = move |ev| {
        let v = event_target_value(&ev);
        file_name.set(v);
    };

    // Import file button.

    view! {
        //<main>
            <div>
                <h2>"Import"</h2>
                <p> { move || current_directory.get().to_string() } </p>
            </div>

            <form>
                <input style="width: 300px;" placeholder="Enter relative path ..." on:input=update_file_name/>
                <button class="right-panel-button" type="submit">"Import"</button>
                <p> { move || file_name.get() } </p>
            </form>
        //</main>
    }
    /*
    <form class="row" on:submit=read_file>
        <input
            id="import-file_path"
            placeholder="Enter full CSV file path..."
            on:input=update_file_name
        />
        <button type="submit">"Import"</button>
    </form>
    <p>{move || error_msg.get()} </p>
    */
    //<Show when=move || error_msg.get().is_some()>
    //    <p class="error-message">{move || error_msg.get().unwrap_or_default()}</p>
    //</Show>

    /*
    <table class="mt-4 border-collapse border border-gray-400">
        <tbody>
            { move || csv_data.get().iter().map(|row| {
                view! {
                    <tr>
                        { row.iter().map(|cell| {
                            view! { <td class="border border-gray-300 p-1">{cell.to_string()}</td> }
                        }).collect::<Vec<_>>() }
                    </tr>
                }
            }).collect::<Vec<_>>() }
        </tbody>
    </table>
    */
}

#[component]
pub fn NotFoundScreen() -> impl IntoView {
    view! { <div><h2>"Not found"</h2></div> }
}

/*
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>
        </main>
    }
}
*/
