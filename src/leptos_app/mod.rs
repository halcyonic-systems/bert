mod tree;

use crate::bevy_app::init_bevy_app;
use crate::events::{TreeEvent, TriggerEvent};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use crate::leptos_app::tree::Tree;

#[component]
pub fn App() -> impl IntoView {
    let (tree_event_receiver, tree_event_sender) = event_b2l::<TreeEvent>();
    let (trigger_event_sender, trigger_event_receiver) = event_l2b::<TriggerEvent>();

    let (tree_visible, set_tree_visible) = signal(false);

    view! {
        <h2>"Hello World"</h2>
        <button on:click=move |_| {
            trigger_event_sender.send(TriggerEvent::ShowTree).ok();
            tree_visible.set(true);
        }>{"Show Tree"}</button>
        <BevyCanvas
            init=move || { init_bevy_app(tree_event_sender) }

            {..}
            width="500"
            height="300"
        />

        <Tree visible=tree_visible event_receiver=tree_event_receiver />
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
