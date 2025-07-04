use crate::app::FileConfiguration;
use leptos::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Race {
    pub name: String,
    pub date: String,
    pub linked_files: Vec<String>,
}

#[component]
pub fn ConfigurationScreen() -> impl IntoView {
    let state = use_context::<FileConfiguration>().expect("AppState missing");

    let races: RwSignal<Vec<Race>> = RwSignal::new(Vec::new());
    let added_files = state.lazy_files;

    let race_name = RwSignal::new(String::new());
    let race_date = RwSignal::new(String::new());

    let add_race = move |_| {
        let name = race_name.get().trim().to_string();
        let date = race_date.get().trim().to_string();

        if !name.is_empty() && !date.is_empty() {
            races.update(|list| {
                list.push(Race {
                    name,
                    date,
                    linked_files: vec![],
                });
            });
            race_name.set(String::new());
            race_date.set(String::new());
        }
    };

    view! {
        <h2>"🏁 Race configuration"</h2>
                        <div class="config-screen">


                            <div class="new-race-form">
                                <input
                                    class="styled-input"
                                    type="text"
                                    placeholder="Race name"
                                    prop:value=race_name
                                    on:input=move |ev| race_name.set(event_target_value(&ev))
                                />
                                <input
                                    class="styled-input"
                                    type="date"
                                    prop:value=race_date
                                    on:input=move |ev| race_date.set(event_target_value(&ev))
                                />
                                <button class="menu-button" on:click=add_race>"Add Race"</button>
                            </div>

                            <div class="race-list">
                                <For each=move || races.get() key=|r| r.name.clone() children=move |race| {
                // Clone needed values for safe reuse
                let race_name = race.name.clone();
                let race_date = race.date.clone();
                let linked_files = RwSignal::new(race.linked_files.clone());

                // Clone for view display
                let race_name_for_display = race_name.clone();
                let race_date_for_display = race_date.clone();

                // Clone for closure 1
                let race_name_for_add = race_name.clone();
                let add_file = move |ev: leptos::ev::Event| {
                    let file = event_target_value(&ev);
                    if !file.is_empty() && !linked_files.get().contains(&file) {
                        linked_files.update(|f| f.push(file.clone()));
                        races.update(|all| {
                            if let Some(r) = all.iter_mut().find(|r| r.name == race_name_for_add) {
                                r.linked_files = linked_files.get();
                            }
                        });
                    }
                };

                // Clone for closure 2
                let race_name_for_delete = race_name.clone();
                let delete_race = move |_| {
                    races.update(|all| {
                        all.retain(|r| r.name != race_name_for_delete);
                    });
                };

                view! {
                    <div class="race-card">
                        <h3>{race_name_for_display} " (" {race_date_for_display} ")"</h3>

                        <ul>
                            <For each=move || linked_files.get() key=|f| f.clone() children=move |file| {
                                view! { <li>{file}</li> }
                            } />
                        </ul>

                        <select on:change=add_file>
                            <option value="">"-- Link a file --"</option>
                            <For each=move || added_files.get() key=|f| f.clone() children=move |f| {
        let file = f.clone(); // clone once
        let file1 = f.clone();
        view! {
            <option value={file1.clone()}>{file}</option>
        }
    } />

                        </select>

                        <button class="delete-button" on:click=delete_race>"⛔ Remove Race"</button>
                    </div>
                }
            } />


                            </div>
                        </div>
                    }
}

/*
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

use thiserror::Error;

use crate::app::DirectoryArgs;

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

async fn load_file_content(directory: &str) -> Result<Vec<Vec<String>>, ConfigurationError> {
    let args = serde_wasm_bindgen::to_value(&DirectoryArgs { directory })
        .map_err(ConfigurationError::SerdeWasm)?;

    let new_content: JsValue = invoke("read_csv_file", args).await;

    let result = from_value::<Vec<Vec<String>>>(new_content).unwrap();
    Ok(result)
}

#[component]
pub fn ConfigurationScreen() -> impl IntoView {
    //let state = use_context::<FileConfiguration>().expect("AppState not found");

    let file_content: RwSignal<Vec<Vec<String>>> = RwSignal::new(Vec::new());

    //let selected_file: RwSignal<String> =
    //    RwSignal::new(String::from(state.added_files.get()[0].clone()));
    let selected_file: RwSignal<String> = RwSignal::new(String::from(
        r"C:/Users/TheoOdendaal/source/repos/no-eagles\bloemhof 1 jo.csv",
    ));

    let table_rows = Memo::new(move |_| {
        file_content
            .get()
            .iter()
            .map(|row| {
                view! {
                    <tr>
                        {row.iter().map(|cell| view! {
                            <td>{cell.clone()}</td>
                        }).collect::<Vec<_>>()}
                    </tr>
                }
            })
            .collect::<Vec<_>>()
    });

    let update_file_content = move |ev: MouseEvent| {
        spawn_local(async move {
            ev.prevent_default();
            let new_content = load_file_content(&selected_file.get()).await.unwrap();
            file_content.set(new_content);
        })
    };

    view! {
        <h2>"Configuration"</h2>

        <button type="submit" on:click=update_file_content>"Load"</button>
        <table class="file-table">
            <tbody>
                {move || table_rows.get()}
            </tbody>
        </table>

    }
}
*/
