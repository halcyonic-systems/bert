use std::path::PathBuf;

use leptos::{ev::keydown, prelude::*, task::spawn_local};
use leptos_bevy_canvas::prelude::*;
use leptos_use::use_event_listener;
use serde::{Deserialize, Serialize};
use tauri_sys::core::invoke;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, FileReader, HtmlInputElement};

use crate::LoadFileEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseFile {
    pub path: String,
    pub data: Vec<u8>,
}

impl UseFile {
    pub fn new(path: String, data: Vec<u8>) -> Self {
        Self { path, data }
    }
}

pub struct UseFileDialogResult<C>
where
    C: Fn() + Clone + Send + Sync,
{
    pub file: Signal<Option<UseFile>>,
    pub cleanup: C,
}

pub struct UseFileDialogOptions<C>
where
    C: Fn() -> bool + Clone + Send + Sync + 'static,
{
    pub initial_file: RwSignal<Option<UseFile>>,
    pub extensions: Signal<Vec<String>>,
    pub additional_behavior: C,
}

// impl Default for UseFileDialogOptions {
//     fn default() -> Self {
//         Self {
//             initial_file: RwSignal::new(None),
//             extensions: vec![].into(),
//             additional_behavior: || true,
//         }
//     }
// }

pub fn use_file_dialog() -> UseFileDialogResult<impl Fn() + Clone + Send + Sync> {
    use_file_dialog_with_options(UseFileDialogOptions {
        initial_file: RwSignal::new(None),
        extensions: vec![].into(),
        additional_behavior: || true,
    })
}

pub fn use_file_dialog_with_options<C>(
    options: UseFileDialogOptions<C>,
) -> UseFileDialogResult<impl Fn() + Clone + Send + Sync>
where
    C: Fn() -> bool + Clone + Send + Sync + 'static,
{
    let UseFileDialogOptions {
        initial_file,
        extensions,
        additional_behavior,
    } = options;

    // Create a signal to store the file data as Vec<u8>
    let (file_data, set_file_data) = signal::<Option<UseFile>>(None);

    let cleanup = use_event_listener(window(), keydown, move |ev: web_sys::KeyboardEvent| {
        // Check if Ctrl + L was pressed
        let modifier = {
            #[cfg(target_os = "macos")]
            {
                ev.meta_key()
            }
            #[cfg(not(target_os = "macos"))]
            {
                ev.ctrl_key()
            }
        };
        if modifier && ev.key() == "l" {
            // Prevent the default behavior and stop propagation
            ev.prevent_default();
            ev.stop_propagation();

            // Call additional behavior if provided
            if additional_behavior() {
                // Create the file input element dynamically
                let document = web_sys::window().unwrap().document().unwrap();
                let input = document.create_element("input").unwrap();
                let input = input.dyn_into::<HtmlInputElement>().unwrap();

                // Set input attributes
                input.set_type("file");
                input.set_accept(&extensions.get_untracked().join(","));
                input.set_multiple(false);
                input.set_hidden(true);

                // Add an event listener for file selection
                let set_file_data = set_file_data.clone();
                let on_change = Closure::wrap(Box::new(move |ev: Event| {
                    let input = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                    if let Some(file) = input.files().and_then(|files| files.get(0)) {
                        // Create a FileReader instance
                        let reader = FileReader::new().unwrap();
                        let name = file.name().to_string();

                        // Set up the onload event handler for the FileReader
                        let set_file_data = set_file_data.clone();
                        let on_load = Closure::wrap(Box::new(move |ev: Event| {
                            let target = ev.target().unwrap().dyn_into::<FileReader>().unwrap();
                            if let Ok(result) = target.result() {
                                if result.is_instance_of::<js_sys::ArrayBuffer>() {
                                    // Convert ArrayBuffer to Vec<u8>
                                    let array_buffer =
                                        result.dyn_into::<js_sys::ArrayBuffer>().unwrap();
                                    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                                    let mut buffer = vec![0; uint8_array.length() as usize];
                                    uint8_array.copy_to(&mut buffer);
                                    set_file_data.set(Some(UseFile::new(name.clone(), buffer)));
                                }
                            }
                        })
                            as Box<dyn FnMut(_)>);

                        reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
                        on_load.forget(); // Prevent the closure from being garbage collected

                        // Read the file as an ArrayBuffer
                        reader.read_as_array_buffer(&file).unwrap();
                    }
                }) as Box<dyn FnMut(_)>);

                input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
                on_change.forget(); // Prevent the closure from being garbage collected

                // Append the input to the body and trigger the file dialog
                document.body().unwrap().append_child(&input).unwrap();
                input.click(); // Programmatically trigger the file dialog

                // Clean up the input element after use
                let _ = document.body().unwrap().remove_child(&input);
            }
        }
    });

    Effect::new(move || {
        if let Some(file_data) = file_data.get() {
            initial_file.update(|file| {
                *file = Some(file_data.clone());
            });
        }
    });

    UseFileDialogResult {
        file: Signal::derive(move || file_data.get()),
        cleanup,
    }
}

pub fn generate_file_loader() -> BevyMessageReceiver<LoadFileEvent> {
    let file = RwSignal::new(None::<UseFile>);

    let check_if_tauri_available = move || {
        let tauri_exists = leptos_use::js! {
            "__TAURI__" in &window()
        };

        if tauri_exists {
            #[derive(Serialize, Deserialize, Debug, Clone)]
            struct Args {
                pb: PathBuf,
            }
            spawn_local({
                async move {
                    let file_path = invoke::<Option<PathBuf>>("pick_file", ()).await;

                    if let Some(path) = file_path {
                        let file_data = invoke::<UseFile>("load_file", &Args { pb: path }).await;
                        file.update(|file| {
                            *file = Some(file_data);
                        });
                    }
                }
            });
        }

        !tauri_exists
    };

    let _ = use_file_dialog_with_options(UseFileDialogOptions {
        initial_file: file,
        extensions: vec!["application/json".to_string()].into(),
        additional_behavior: check_if_tauri_available,
    });

    let (load_file_event_sender, load_file_event_receiver) = message_l2b::<LoadFileEvent>();

    Effect::new(move |_| {
        if let Some(UseFile { path, data }) = file.get() {
            load_file_event_sender
                .send(LoadFileEvent {
                    file_path: path.clone(),
                    data: data.clone(),
                })
                .ok();
        }
    });

    load_file_event_receiver
}
