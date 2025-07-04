use leptos::prelude::*;

use crate::screens;
use screens::configuration::ConfigurationScreen;
use screens::home::HomeScreen;
use screens::import::ImportScreen;

#[derive(Clone)]
pub struct AppState {
    pub added_files: RwSignal<Vec<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            added_files: RwSignal::new(Vec::new()),
        }
    }
}

#[derive(Clone)]
enum AppScreens {
    Home,
    Import,
    Configuration,
}

#[component]
pub fn App() -> impl IntoView {
    let app_state = AppState::new();
    provide_context(app_state);

    let current_screen = RwSignal::new(AppScreens::Home);
    view! {
        <div class="app-container">
            <div class="left-panel">
                <button class="menu-button" on:click=move |_| current_screen.set(AppScreens::Home)>"Home"</button>
                <button class="menu-button" on:click=move |_| current_screen.set(AppScreens::Import)>"Import"</button>
                <button class="menu-button" on:click=move |_| current_screen.set(AppScreens::Configuration)>"Configuration"</button>

            </div>

            <div class="right-panel">
                {
                    move || {
                        match current_screen.get() {
                            AppScreens::Home => view! { <HomeScreen /> }.into_any(),
                            AppScreens::Import => view! { <ImportScreen /> }.into_any(),
                            AppScreens::Configuration => view! { <ConfigurationScreen /> }.into_any(),

                        }
                    }
                }
            </div>
        </div>
    }
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
