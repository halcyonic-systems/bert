mod components;
mod details;

use crate::bevy_app::init_bevy_app;
use crate::bevy_app::{
    components::System, ElementDescription, ExternalEntity, Flow, Interface,
    SelectedHighlightHelperAdded, SystemElement, SystemEnvironment,
};
use crate::leptos_app::details::Details;
use crate::{ParentState, Subsystem};
use bevy::prelude::{Name, With};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::{single_query_signal, BevyCanvas};
use leptos_meta::*;

pub type InterfaceQuery = (Name, ElementDescription, Interface);
pub type InteractionQuery = (Name, ElementDescription, Flow);
pub type ExternalEntityQuery = (Name, ElementDescription, ExternalEntity);
pub type SystemQuery = (Name, ElementDescription, System, SystemEnvironment);
pub type SubSystemQuery = (Name, ElementDescription, System, ParentState);

pub type SelectionFilter = With<SelectedHighlightHelperAdded>;
pub type SubSystemFilter = With<Subsystem>;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

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

    view! {
        <div class="h-screen">
            <BevyCanvas init=move || {
                init_bevy_app(
                    selected_details_query,
                    interface_details_query,
                    interaction_details_query,
                    external_entity_details_query,
                    system_details_query,
                    sub_system_details_query,
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
