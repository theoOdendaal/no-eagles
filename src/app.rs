use leptos::prelude::*;

use serde::{Deserialize, Serialize};

use crate::screens;
use screens::configuration::ConfigurationScreen;
use screens::home::HomeScreen;
use screens::import::ImportScreen;

#[derive(Serialize, Deserialize)]
pub struct DirectoryArgs<'a> {
    pub directory: &'a str,
}

#[derive(Clone)]
pub struct FileConfiguration {
    pub lazy_files: RwSignal<Vec<String>>,
}

impl FileConfiguration {
    pub fn new() -> Self {
        Self {
            lazy_files: RwSignal::new(Vec::new()),
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
    let app_state = FileConfiguration::new();
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
