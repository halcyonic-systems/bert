mod components;
mod details;

mod tree;
mod use_file_dialog;

use crate::bevy_app::init_bevy_app;
use crate::bevy_app::{
    components::System, DetachMarkerLabelEvent, ElementDescription, ExternalEntity, Flow,
    Interface, IsSameAsId, SelectedHighlightHelperAdded, SystemElement, SystemEnvironment,
};
use crate::leptos_app::details::Details;
use crate::leptos_app::components::{ControlsMenu, ModelBrowser};
use crate::LoadFileEvent;
use crate::{ParentState, Subsystem};
use bevy::prelude::{Name, With};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::{event_l2b, single_query_signal, BevyCanvas};
use leptos_meta::*;
use use_file_dialog::use_file_dialog_with_options;

pub type InterfaceQuery = (Name, ElementDescription, Interface);
pub type InteractionQuery = (Name, ElementDescription, Flow);
pub type ExternalEntityQuery = (Name, ElementDescription, ExternalEntity);
pub type SystemQuery = (Name, ElementDescription, System, SystemEnvironment);
pub type SubSystemQuery = (Name, ElementDescription, System, ParentState);

pub type IsSameAsIdQuery = (IsSameAsId,);

pub type SelectionFilter = With<SelectedHighlightHelperAdded>;
pub type SubSystemFilter = With<Subsystem>;
pub type ExternalEntityFilter = With<ExternalEntity>;
use crate::events::{TreeEvent, TriggerEvent};
use crate::leptos_app::tree::Tree;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Unified LoadFileEvent system for both Ctrl+L file dialog and Model Browser
    let (load_file_writer, load_file_event_receiver) = event_l2b::<LoadFileEvent>();
    
    // Set up file dialog with the shared event writer
    let file_dialog_signal = RwSignal::new(None::<crate::leptos_app::use_file_dialog::UseFile>);
    let _ = use_file_dialog_with_options(crate::leptos_app::use_file_dialog::UseFileDialogOptions {
        initial_file: file_dialog_signal,
        extensions: vec!["application/json".to_string()].into(),
        additional_behavior: move || {
            // Check if Tauri is available for desktop file dialogs
            let tauri_exists = leptos_use::js! {
                "__TAURI__" in &window()
            };
            
            if tauri_exists {
                use leptos::task::spawn_local;
                use serde::{Serialize, Deserialize};
                use std::path::PathBuf;
                use tauri_sys::core::invoke;
                
                #[derive(Serialize, Deserialize, Debug, Clone)]
                struct Args {
                    pb: PathBuf,
                }
                
                spawn_local({
                    let file_signal = file_dialog_signal;
                    async move {
                        let file_path = invoke::<Option<PathBuf>>("pick_file", ()).await;
                        if let Some(path) = file_path {
                            let file_data = invoke::<crate::leptos_app::use_file_dialog::UseFile>("load_file", &Args { pb: path }).await;
                            file_signal.update(|file| {
                                *file = Some(file_data);
                            });
                        }
                    }
                });
            }
            
            !tauri_exists // Return true for web file dialog if Tauri not available
        },
    });
    
    // Connect file dialog signal to LoadFileEvent stream
    Effect::new({
        let load_file_writer = load_file_writer.clone();
        move |_| {
            if let Some(crate::leptos_app::use_file_dialog::UseFile { path, data }) = file_dialog_signal.get() {
                load_file_writer
                    .send(LoadFileEvent {
                        file_path: path,
                        data,
                    })
                    .ok();
            }
        }
    });

    let (selected_details, selected_details_query) =
        single_query_signal::<(SystemElement,), With<SelectedHighlightHelperAdded>>();

    let (interface_details, interface_details_query) =
        single_query_signal::<InterfaceQuery, SelectionFilter>();

    let (interaction_details, interaction_details_query) =
        single_query_signal::<InteractionQuery, SelectionFilter>();

    let (external_entity_details, external_entity_details_query) =
        single_query_signal::<ExternalEntityQuery, SelectionFilter>();

    let (system_details, system_details_query) =
        single_query_signal::<SystemQuery, SelectionFilter>();

    let (sub_system_details, sub_system_details_query) =
        single_query_signal::<SubSystemQuery, (SelectionFilter, SubSystemFilter)>();

    let (is_same_as_id, is_same_as_id_query) =
        single_query_signal::<IsSameAsIdQuery, (ExternalEntityFilter, SelectionFilter)>();

    let (detach_event_sender, detach_event_receiver) = event_l2b::<DetachMarkerLabelEvent>();

    let (tree_event_receiver, tree_event_sender) = event_b2l::<TreeEvent>();
    let (trigger_event_sender, trigger_event_receiver) = event_l2b::<TriggerEvent>();

    let (tree_visible, set_tree_visible) = signal(false);
    let (controls_visible, set_controls_visible) = signal(false);
    let (model_browser_visible, set_model_browser_visible) = signal(false);

    view! {
        <Show
            when=move || tree_visible.get()
            fallback=move || {
                let trigger_event_sender = trigger_event_sender.clone();
                view! {
                    <div class="absolute top-4 left-4 z-20 flex gap-2">
                        <button
                            class="px-4 py-2 rounded-lg bg-white shadow-md hover:shadow-lg transition-shadow"
                            on:click=move |_| {
                                trigger_event_sender.send(TriggerEvent::ShowTree).ok();
                                set_tree_visible.set(true);
                            }
                        >
                            {"Show Tree"}
                        </button>
                        <button
                            class="px-4 py-2 rounded-lg bg-white shadow-md hover:shadow-lg transition-shadow"
                            on:click=move |_| {
                                set_controls_visible.set(true);
                            }
                        >
                            {"Controls"}
                        </button>
                        <button
                            class="px-4 py-2 rounded-lg bg-white shadow-md hover:shadow-lg transition-shadow"
                            on:click=move |_| {
                                set_model_browser_visible.set(true);
                            }
                        >
                            {"Model Browser"}
                        </button>
                    </div>
                }
            }
        >
            <div class="absolute top-4 left-4 z-20 flex gap-2">
                <button
                    class="px-4 py-2 rounded-lg bg-white shadow-md hover:shadow-lg transition-shadow"
                    on:click=move |_| {
                        set_tree_visible.set(false);
                    }
                >
                    {"Hide Tree"}
                </button>
                <button
                    class="px-4 py-2 rounded-lg bg-white shadow-md hover:shadow-lg transition-shadow"
                    on:click=move |_| {
                        set_controls_visible.set(true);
                    }
                >
                    {"Controls"}
                </button>
                <button
                    class="px-4 py-2 rounded-lg bg-white shadow-md hover:shadow-lg transition-shadow"
                    on:click=move |_| {
                        set_model_browser_visible.set(true);
                    }
                >
                    {"Model Browser"}
                </button>
            </div>
        </Show>
        <Tree visible=tree_visible event_receiver=tree_event_receiver />
        <ControlsMenu 
            visible=controls_visible 
            on_close=Callback::new(move |_| set_controls_visible.set(false))
        />
        <ModelBrowser 
            visible=model_browser_visible
            on_close=Callback::new(move |_| set_model_browser_visible.set(false))
            on_load=Callback::new(move |event: LoadFileEvent| {
                // Send the load file event to Bevy
                leptos::logging::log!("Sending LoadFileEvent: {} with {} bytes", event.file_path, event.data.len());
                load_file_writer.send(event).ok();
                set_model_browser_visible.set(false);
            })
        />
        <div class="h-screen">
            <BevyCanvas init=move || {
                init_bevy_app(
                    selected_details_query,
                    interface_details_query,
                    interaction_details_query,
                    external_entity_details_query,
                    system_details_query,
                    sub_system_details_query,
                    is_same_as_id_query,
                    detach_event_receiver,
                    load_file_event_receiver,
                    tree_event_sender,
                    trigger_event_receiver,
                )
            } />
        </div>
        <Details
            selected=selected_details
            interaction_details
            interface_details
            external_entity_details
            system_details
            sub_system_details
            is_same_as_id
            detach_event_sender
        />
    }
}

// use leptos::task::spawn_local;
// use leptos::{ev::SubmitEvent, prelude::*};
// use serde::{Deserialize, Serialize};
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }
//
// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }
//
// #[component]
// pub fn App() -> impl IntoView {
//     let (name, set_name) = signal(String::new());
//     let (greet_msg, set_greet_msg) = signal(String::new());
//
//     let update_name = move |ev| {
//         let v = event_target_value(&ev);
//         set_name.set(v);
//     };
//
//     let greet = move |ev: SubmitEvent| {
//         ev.prevent_default();
//         spawn_local(async move {
//             let name = name.get_untracked();
//             if name.is_empty() {
//                 return;
//             }
//
//             let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
//             // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//             let new_msg = invoke("greet", args).await.as_string().unwrap();
//             set_greet_msg.set(new_msg);
//         });
//     };
//
//     view! {
//         <main class="container">
//             <h1>"Welcome to Tauri + Leptos"</h1>
//
//             <div class="row">
//                 <a href="https://tauri.app" target="_blank">
//                     <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
//                 </a>
//                 <a href="https://docs.rs/leptos/" target="_blank">
//                     <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
//                 </a>
//             </div>
//             <p>"Click on the Tauri and Leptos logos to learn more."</p>
//
//             <form class="row" on:submit=greet>
//                 <input
//                     id="greet-input"
//                     placeholder="Enter a name..."
//                     on:input=update_name
//                 />
//                 <button type="submit">"Greet"</button>
//             </form>
//             <p>{ move || greet_msg.get() }</p>
//         </main>
//     }
// }
