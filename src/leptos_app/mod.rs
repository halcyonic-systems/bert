mod components;
#[allow(clippy::option_map_unit_fn)]
mod details;
mod simulation;
mod tree;
mod use_file_dialog;

use crate::bevy_app::data_model::complexity_calculator::calculate_simonian_complexity;
use crate::bevy_app::data_model::validate::{classify_openness, validate, Severity, ValidationIssue};
use crate::bevy_app::{
    init_bevy_app, DetachMarkerLabelEvent, ExternalEntityFilter, ExternalEntityQuery,
    InteractionQuery, InterfaceQuery, IsSameAsIdQuery, SelectedHighlightHelperAdded,
    SelectionFilter, SubSystemFilter, SubSystemQuery, SystemElement, SystemQuery,
};
use crate::leptos_app::components::{
    AppMode, ChatPanel, ControlsMenu, LandingScreen, ModelBrowser, Palette, Toast, ValidationPanel,
};
use crate::leptos_app::details::Details;
use crate::leptos_app::simulation::SimPanel;
use crate::LoadFileEvent;
use bevy::prelude::With;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::{
    message_b2l, message_l2b, signal_synced, single_query_signal, BevyCanvas,
};
use leptos_meta::*;
use use_file_dialog::use_file_dialog_with_options;

use crate::events::{
    CancelModeEvent, DeselectAllEvent, ModeChangeEvent, PaletteClickEvent, SaveSuccessEvent,
    TreeEvent, TriggerEvent, ZoomEvent,
};
use crate::leptos_app::tree::Tree;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Unified LoadFileEvent system for both Ctrl+L file dialog and Model Browser
    let (load_file_writer, load_file_event_receiver) = message_l2b::<LoadFileEvent>();

    // Zoom event system for JavaScript to Bevy communication
    let (zoom_event_writer, zoom_event_receiver) = message_l2b::<ZoomEvent>();

    // Deselect event system for close button functionality
    let (deselect_event_writer, deselect_event_receiver) = message_l2b::<DeselectAllEvent>();

    // Palette click event system for Leptos palette panel
    let (palette_click_writer, palette_click_receiver) = message_l2b::<PaletteClickEvent>();

    // Cancel mode event system for ESC key bypass
    let (cancel_mode_writer, cancel_mode_receiver) = message_l2b::<CancelModeEvent>();

    // Save success event system for user feedback
    let (_save_success_event_writer, save_success_event_receiver) =
        message_l2b::<SaveSuccessEvent>();

    // Toast notification state
    let (toast_visible, set_toast_visible) = signal(false);
    let (toast_message, set_toast_message) = signal(String::new());

    // Complexity counter - tracks system complexity when models are loaded
    let (complexity, set_complexity) = signal(0.0f64);

    // Pre-load validation state
    let (validation_issues, set_validation_issues) = signal(None::<Vec<ValidationIssue>>);
    let (pending_load, set_pending_load) = signal(None::<LoadFileEvent>);

    // Set up file dialog with the shared event writer
    let file_dialog_signal = RwSignal::new(None::<crate::leptos_app::use_file_dialog::UseFile>);
    let _ =
        use_file_dialog_with_options(crate::leptos_app::use_file_dialog::UseFileDialogOptions {
            initial_file: file_dialog_signal,
            extensions: vec!["application/json".to_string()].into(),
            additional_behavior: move || {
                // Check if Tauri is available for desktop file dialogs
                let tauri_exists = leptos_use::js! {
                    "__TAURI__" in &window()
                };

                if tauri_exists {
                    use leptos::task::spawn_local;
                    use serde::{Deserialize, Serialize};
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
                                let file_data =
                                    invoke::<crate::leptos_app::use_file_dialog::UseFile>(
                                        "load_file",
                                        &Args { pb: path },
                                    )
                                    .await;
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

    let (loaded_model_name, set_loaded_model_name) = signal(String::new());
    let (loaded_file_path, set_loaded_file_path) = signal(None::<String>);
    let (model_json_context, set_model_json_context) = signal(None::<String>);
    let (app_mode, set_app_mode) = signal(AppMode::Landing);

    // Connect file dialog signal to LoadFileEvent stream with validation
    Effect::new({
        let load_file_writer = load_file_writer.clone();
        let set_complexity_inner = set_complexity;
        let set_loaded_model_name = set_loaded_model_name;
        let set_loaded_file_path = set_loaded_file_path;
        let set_model_json_context = set_model_json_context;
        let set_app_mode = set_app_mode;
        let set_toast_message = set_toast_message;
        let set_toast_visible = set_toast_visible;
        move |_| {
            if let Some(crate::leptos_app::use_file_dialog::UseFile { path, data }) =
                file_dialog_signal.get()
            {
                if let Ok(json_value) = serde_json::from_slice::<serde_json::Value>(&data) {
                    let pre_result =
                        crate::bevy_app::data_model::validate::validate_json_structure(&json_value);
                    if pre_result.has_errors() {
                        set_pending_load.set(None);
                        set_validation_issues.set(Some(pre_result.issues));
                        return;
                    }
                }

                match serde_json::from_slice::<crate::bevy_app::data_model::WorldModel>(&data) {
                    Ok(world_model) => {
                        let complexity_result = calculate_simonian_complexity(&world_model);
                        set_complexity_inner.set(complexity_result.total_complexity);

                        if let Ok(json_str) = String::from_utf8(data.clone()) {
                            set_model_json_context.set(Some(json_str));
                        }

                        if path.starts_with("generated:") || path.starts_with("template:") {
                            set_loaded_file_path.set(None);
                        } else {
                            set_loaded_file_path.set(Some(path.clone()));
                        }

                        let mn = path.strip_prefix("template:").unwrap_or(&path);
                        let mn = mn.strip_suffix(".json").unwrap_or(mn);
                        let mn = mn.rsplit('/').next().unwrap_or(mn);
                        set_loaded_model_name.set(mn.to_string());
                        set_app_mode.set(AppMode::Editing);

                        // Non-blocking open/closed-with-respect-to-mass classification.
                        set_toast_message.set(classify_openness(&world_model));
                        set_toast_visible.set(true);

                        let result = validate(&world_model);
                        if result.is_clean() {
                            load_file_writer
                                .send(LoadFileEvent {
                                    file_path: path,
                                    data,
                                })
                                .ok();
                        } else {
                            set_pending_load.set(Some(LoadFileEvent {
                                file_path: path,
                                data,
                            }));
                            set_validation_issues.set(Some(result.issues));
                        }
                    }
                    Err(e) => {
                        set_pending_load.set(None);
                        set_validation_issues.set(Some(vec![ValidationIssue {
                            severity: Severity::Error,
                            location: "root".to_string(),
                            message: format!("JSON parse error: {e}"),
                            suggestion: Some("Check that the file is valid BERT JSON.".to_string()),
                        }]));
                    }
                }
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

    // Resource signal for spatial detail panel mode
    let (spatial_mode, spatial_mode_duplex) =
        signal_synced(crate::bevy_app::components::SpatialDetailPanelMode::default());

    let (detach_event_sender, detach_event_receiver) = message_l2b::<DetachMarkerLabelEvent>();

    let (tree_event_receiver, tree_event_sender) = message_b2l::<TreeEvent>();
    let (save_success_receiver, save_success_bevy_sender) = message_b2l::<SaveSuccessEvent>();
    let (mode_change_receiver, mode_change_sender) = message_b2l::<ModeChangeEvent>();
    let (trigger_event_sender, trigger_event_receiver) = message_l2b::<TriggerEvent>();

    // Mode indicator signal for bottom toolbar
    let (mode_text, set_mode_text) = signal(String::new());

    // Handle save success events from Bevy
    Effect::new(move |_| {
        if let Some(event) = save_success_receiver.read().as_ref() {
            set_toast_message.set(event.message.clone());
            set_toast_visible.set(true);
        }
    });

    // Handle mode change events from Bevy
    Effect::new(move |_| {
        if let Some(event) = mode_change_receiver.read().as_ref() {
            set_mode_text.set(event.mode_text.clone());
        }
    });

    // Global ESC key handler - works regardless of focus
    {
        let cancel_mode_writer = cancel_mode_writer;
        Effect::new(move |_| {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;

            let cancel_writer = cancel_mode_writer;
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                if event.key() == "Escape" {
                    leptos::logging::log!("🌐 Global ESC detected - cancelling mode");
                    cancel_writer.send(CancelModeEvent).ok();
                }
            }) as Box<dyn FnMut(_)>);

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();

            // Leak the closure to keep it alive (simple approach for app lifetime)
            closure.forget();
        });
    }

    let (tree_visible, set_tree_visible) = signal(false);
    let (controls_visible, set_controls_visible) = signal(false);
    let (model_browser_visible, set_model_browser_visible) = signal(false);
    let (sim_panel_visible, set_sim_panel_visible) = signal(false);
    let (chat_visible, set_chat_visible) = signal(false);
    let (active_run, set_active_run) = signal(None::<simulation::types::RunInfo>);
    let (_sim_results, set_sim_results) = signal(None::<simulation::types::SimulationResults>);

    let tauri_available = leptos_use::js! {
        "__TAURI__" in &window()
    };

    let (ollama_available, set_ollama_available) = signal(false);
    let (recent_models, set_recent_models) = signal(Vec::<(String, String)>::new());

    if tauri_available {
        use leptos::task::spawn_local;
        use tauri_sys::core::invoke;

        spawn_local({
            let set_ollama = set_ollama_available;
            async move {
                let status = invoke::<bool>("check_ollama_status", ()).await;
                set_ollama.set(status);
            }
        });

        spawn_local({
            let set_recent = set_recent_models;
            async move {
                #[derive(serde::Deserialize)]
                struct LocalModelInfo {
                    name: String,
                    path: String,
                    #[allow(dead_code)]
                    modified: u64,
                }
                let models = invoke::<Vec<LocalModelInfo>>("list_local_models", ()).await;
                set_recent.set(models.into_iter().map(|m| (m.name, m.path)).collect());
            }
        });
    }

    let is_editing = Memo::new(move |_| app_mode.get() == AppMode::Editing);
    let is_landing = Memo::new(move |_| app_mode.get() == AppMode::Landing);

    let load_writer_describe = load_file_writer.clone();
    let load_writer_scratch = load_file_writer.clone();
    let load_writer_recent = load_file_writer.clone();

    view! {
        // Landing screen — hidden while model browser is open
        <Show when=move || is_landing.get() && !model_browser_visible.get()>
            <LandingScreen
                on_describe=Callback::new({
                    let load_file_writer = load_writer_describe.clone();
                    let set_complexity = set_complexity;
                    let set_app_mode = set_app_mode;
                    let set_chat_visible = set_chat_visible;
                    let set_loaded_model_name = set_loaded_model_name;
                    let set_loaded_file_path = set_loaded_file_path;
                    let set_model_json_context = set_model_json_context;
                    let set_toast_message = set_toast_message;
                    let set_toast_visible = set_toast_visible;
                    let set_validation_issues = set_validation_issues;
                    let set_pending_load = set_pending_load;
                    move |description: String| {
                        set_app_mode.set(AppMode::Describing);
                        let load_writer = load_file_writer.clone();
                        leptos::task::spawn_local({
                            let desc = description.clone();
                            let set_app_mode = set_app_mode;
                            let set_chat_visible = set_chat_visible;
                            let set_loaded_model_name = set_loaded_model_name;
                            let set_loaded_file_path = set_loaded_file_path;
                            let set_model_json_context = set_model_json_context;
                            let set_toast_message = set_toast_message;
                            let set_toast_visible = set_toast_visible;
                            let set_complexity = set_complexity;
                            let set_validation_issues = set_validation_issues;
                            let set_pending_load = set_pending_load;
                            async move {
                                #[derive(serde::Serialize)]
                                struct GenArgs { conversation: String }
                                #[derive(serde::Deserialize)]
                                struct GenResponse { json_data: String }

                                let result = tauri_sys::core::invoke::<GenResponse>(
                                    "generate_model_from_conversation",
                                    &GenArgs { conversation: desc },
                                ).await;

                                let json_data = result.json_data;
                                match serde_json::from_str::<crate::bevy_app::data_model::WorldModel>(&json_data) {
                                    Ok(world_model) => {
                                        let complexity_result = calculate_simonian_complexity(&world_model);
                                        set_complexity.set(complexity_result.total_complexity);

                                        let model_name = world_model.environment.info.name.clone();
                                        let file_slug = model_name.to_lowercase().replace(' ', "-");

                                        set_model_json_context.set(Some(json_data.clone()));
                                        set_loaded_file_path.set(None);
                                        set_loaded_model_name.set(model_name);
                                        set_app_mode.set(AppMode::Editing);
                                        set_chat_visible.set(true);

                                        let event = LoadFileEvent {
                                            file_path: format!("generated:{file_slug}.json"),
                                            data: json_data.into_bytes(),
                                        };

                                        let result = validate(&world_model);
                                        if result.is_clean() {
                                            load_writer.send(event).ok();
                                        } else {
                                            set_pending_load.set(Some(event));
                                            set_validation_issues.set(Some(result.issues));
                                        }
                                    }
                                    Err(e) => {
                                        set_app_mode.set(AppMode::Landing);
                                        set_toast_message.set(format!("Generation failed: {e}"));
                                        set_toast_visible.set(true);
                                    }
                                }
                            }
                        });
                    }
                })
                on_scratch=Callback::new({
                    let load_file_writer = load_writer_scratch.clone();
                    let set_app_mode = set_app_mode;
                    let set_complexity = set_complexity;
                    let set_loaded_model_name = set_loaded_model_name;
                    let set_loaded_file_path = set_loaded_file_path;
                    move |_| {
                        let blank = include_str!("../../assets/models/examples/blank.json");
                        if let Ok(world_model) = serde_json::from_str::<crate::bevy_app::data_model::WorldModel>(blank) {
                            let complexity_result = calculate_simonian_complexity(&world_model);
                            set_complexity.set(complexity_result.total_complexity);
                            set_loaded_model_name.set("New System".to_string());
                            set_loaded_file_path.set(None);
                            set_app_mode.set(AppMode::Editing);
                            load_file_writer.send(LoadFileEvent {
                                file_path: "template:blank.json".to_string(),
                                data: blank.as_bytes().to_vec(),
                            }).ok();
                        }
                    }
                })
                on_browse=Callback::new(move |_| {
                    set_model_browser_visible.set(true);
                })
                on_open_file=Callback::new(move |_| {
                    file_dialog_signal.set(None);
                    // Trigger the Tauri file picker
                    if tauri_available {
                        use leptos::task::spawn_local;
                        use tauri_sys::core::invoke;
                        spawn_local({
                            let file_signal = file_dialog_signal;
                            async move {
                                let file_path = invoke::<Option<std::path::PathBuf>>("pick_file", ()).await;
                                if let Some(path) = file_path {
                                    #[derive(serde::Serialize)]
                                    struct Args { pb: std::path::PathBuf }
                                    let file_data = invoke::<crate::leptos_app::use_file_dialog::UseFile>(
                                        "load_file", &Args { pb: path },
                                    ).await;
                                    file_signal.update(|file| { *file = Some(file_data); });
                                }
                            }
                        });
                    }
                })
                on_open_recent=Callback::new({
                    let load_file_writer = load_writer_recent.clone();
                    let set_app_mode = set_app_mode;
                    let set_complexity = set_complexity;
                    let set_loaded_model_name = set_loaded_model_name;
                    let set_loaded_file_path = set_loaded_file_path;
                    let set_model_json_context = set_model_json_context;
                    let set_validation_issues = set_validation_issues;
                    let set_pending_load = set_pending_load;
                    move |path: String| {
                        leptos::task::spawn_local({
                            let load_writer = load_file_writer.clone();
                            let path = path.clone();
                            let set_app_mode = set_app_mode;
                            let set_complexity = set_complexity;
                            let set_loaded_model_name = set_loaded_model_name;
                            let set_loaded_file_path = set_loaded_file_path;
                            let set_model_json_context = set_model_json_context;
                            let set_validation_issues = set_validation_issues;
                            let set_pending_load = set_pending_load;
                            async move {
                                #[derive(serde::Serialize)]
                                struct Args { pb: std::path::PathBuf }
                                #[derive(serde::Deserialize)]
                                struct FileData { data: Vec<u8>, path: String }
                                let file_data = tauri_sys::core::invoke::<FileData>(
                                    "load_file", &Args { pb: std::path::PathBuf::from(&path) },
                                ).await;
                                let event = LoadFileEvent { data: file_data.data, file_path: file_data.path };
                                if let Ok(world_model) = serde_json::from_slice::<crate::bevy_app::data_model::WorldModel>(&event.data) {
                                    let complexity_result = calculate_simonian_complexity(&world_model);
                                    set_complexity.set(complexity_result.total_complexity);
                                    if let Ok(json_str) = String::from_utf8(event.data.clone()) {
                                        set_model_json_context.set(Some(json_str));
                                    }
                                    set_loaded_file_path.set(Some(path));
                                    set_loaded_model_name.set(world_model.environment.info.name.clone());
                                    set_app_mode.set(AppMode::Editing);
                                    let result = validate(&world_model);
                                    if result.is_clean() {
                                        load_writer.send(event).ok();
                                    } else {
                                        set_pending_load.set(Some(event));
                                        set_validation_issues.set(Some(result.issues));
                                    }
                                }
                            }
                        });
                    }
                })
                tauri_available=tauri_available
                ollama_available=Signal::derive(move || ollama_available.get())
                recent_models=Signal::derive(move || recent_models.get())
            />
        </Show>

        // Describing overlay — loading state during generation
        <Show when=move || app_mode.get() == AppMode::Describing>
            <div class="absolute inset-0 z-40 flex items-center justify-center bg-gray-50/80 backdrop-blur-sm">
                <div class="bg-white rounded-2xl shadow-2xl p-12 text-center">
                    <div class="animate-spin w-8 h-8 border-4 border-blue-600 border-t-transparent rounded-full mx-auto mb-4"></div>
                    <p class="text-gray-600 font-medium">"Generating your model..."</p>
                    <p class="text-sm text-gray-400 mt-1">"This takes a few seconds"</p>
                </div>
            </div>
        </Show>

        // Toolbar (tree hidden) — Editing mode only
        <Show when=move || is_editing.get() && !tree_visible.get()>
            {
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
                        {if tauri_available { Some(view! {
                            <button
                                class="px-4 py-2 rounded-lg bg-blue-50 text-blue-700 shadow-md hover:shadow-lg transition-shadow"
                                on:click=move |_| {
                                    set_chat_visible.set(true);
                                }
                            >
                                {"Chat"}
                            </button>
                        }) } else { None }}
                        {if tauri_available { Some(view! {
                            <button
                                class="px-4 py-2 rounded-lg bg-blue-50 text-blue-700 shadow-md hover:shadow-lg transition-shadow"
                                on:click=move |_| {
                                    set_sim_panel_visible.set(true);
                                }
                            >
                                {"Simulate"}
                            </button>
                        }) } else { None }}
                    </div>
                }
            }
        </Show>

        // Toolbar (tree visible) — Editing mode only
        <Show when=move || is_editing.get() && tree_visible.get()>
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
                {if tauri_available { Some(view! {
                    <button
                        class="px-4 py-2 rounded-lg bg-blue-50 text-blue-700 shadow-md hover:shadow-lg transition-shadow"
                        on:click=move |_| {
                            set_sim_panel_visible.set(true);
                        }
                    >
                        {"Simulate"}
                    </button>
                }) } else { None }}
            </div>
        </Show>

        // Complexity counter — Editing mode only
        <Show when=move || is_editing.get()>
            <div class="absolute top-4 right-1/3 z-20 bg-white px-3 py-2 rounded-lg shadow-md">
                <div class="text-sm text-gray-600">{"System Complexity"}</div>
                <div class="text-lg font-mono text-blue-600">
                    {move || format!("{:.1}", complexity.get())}
                </div>
            </div>
        </Show>

        <Palette
            on_element_click=Callback::new({
                let palette_click_writer = palette_click_writer;
                move |element_type| {
                    leptos::logging::log!("🎨 Palette clicked: {:?}", element_type);
                    palette_click_writer.send(PaletteClickEvent { element_type }).ok();
                }
            })
            mode_text=Signal::derive(move || mode_text.get())
        />
        <Tree visible=tree_visible event_receiver=tree_event_receiver />
        <ControlsMenu
            visible=controls_visible
            on_close=Callback::new(move |_| set_controls_visible.set(false))
        />
        <ModelBrowser
            visible=model_browser_visible
            on_close=Callback::new(move |_| set_model_browser_visible.set(false))
            on_load=Callback::new({
                let load_file_writer = load_file_writer.clone();
                let set_complexity = set_complexity;
                let set_app_mode = set_app_mode;
                let set_loaded_file_path = set_loaded_file_path;
                move |event: LoadFileEvent| {
                    match serde_json::from_slice::<crate::bevy_app::data_model::WorldModel>(&event.data) {
                        Ok(world_model) => {
                            let complexity_result = calculate_simonian_complexity(&world_model);
                            set_complexity.set(complexity_result.total_complexity);

                            if let Ok(json_str) = String::from_utf8(event.data.clone()) {
                                set_model_json_context.set(Some(json_str));
                            }

                            if event.file_path.starts_with("generated:") || event.file_path.starts_with("template:") {
                                set_loaded_file_path.set(None);
                            } else {
                                set_loaded_file_path.set(Some(event.file_path.clone()));
                            }

                            set_app_mode.set(AppMode::Editing);

                            let result = validate(&world_model);
                            if result.is_clean() {
                                load_file_writer.send(event).ok();
                            } else {
                                set_pending_load.set(Some(event));
                                set_validation_issues.set(Some(result.issues));
                            }
                        }
                        Err(e) => {
                            set_pending_load.set(None);
                            set_validation_issues.set(Some(vec![ValidationIssue {
                                severity: Severity::Error,
                                location: "root".to_string(),
                                message: format!("JSON parse error: {e}"),
                                suggestion: Some("Check that the file is valid BERT JSON.".to_string()),
                            }]));
                        }
                    }
                    set_model_browser_visible.set(false);
                }
            })
        />
        <ValidationPanel
            issues=Signal::derive(move || validation_issues.get())
            on_continue=Callback::new({
                let load_file_writer = load_file_writer.clone();
                move |_| {
                    if let Some(event) = pending_load.get() {
                        load_file_writer.send(event).ok();
                    }
                    set_validation_issues.set(None);
                    set_pending_load.set(None);
                }
            })
            on_dismiss=Callback::new(move |_| {
                set_validation_issues.set(None);
                set_pending_load.set(None);
            })
        />
        <ChatPanel
            visible=Signal::derive(move || chat_visible.get())
            on_close=Callback::new(move |_| {
                set_chat_visible.set(false);
                if app_mode.get() == AppMode::Creating {
                    set_app_mode.set(AppMode::Landing);
                }
            })
            model_context=Signal::derive(move || model_json_context.get())
            app_mode=Signal::derive(move || app_mode.get())
            on_model_generated=Callback::new({
                let load_file_writer = load_file_writer.clone();
                let set_complexity = set_complexity;
                let set_app_mode = set_app_mode;
                let set_loaded_file_path = set_loaded_file_path;
                move |json_data: Vec<u8>| {
                    match serde_json::from_slice::<crate::bevy_app::data_model::WorldModel>(&json_data) {
                        Ok(world_model) => {
                            let complexity_result = calculate_simonian_complexity(&world_model);
                            set_complexity.set(complexity_result.total_complexity);

                            let model_name = world_model.environment.info.name.clone();
                            let file_slug = model_name.to_lowercase().replace(' ', "-");

                            if let Ok(json_str) = String::from_utf8(json_data.clone()) {
                                set_model_json_context.set(Some(json_str));
                            }
                            set_loaded_file_path.set(None);
                            set_loaded_model_name.set(model_name);
                            set_app_mode.set(AppMode::Editing);
                            set_chat_visible.set(false);

                            let event = LoadFileEvent {
                                file_path: format!("generated:{file_slug}.json"),
                                data: json_data,
                            };

                            let result = validate(&world_model);
                            if result.is_clean() {
                                load_file_writer.send(event).ok();
                            } else {
                                set_pending_load.set(Some(event));
                                set_validation_issues.set(Some(result.issues));
                            }
                        }
                        Err(e) => {
                            leptos::logging::log!("Generated model parse error: {}", e);
                        }
                    }
                }
            })
        />
        <SimPanel
            visible=sim_panel_visible
            on_close=Callback::new(move |_| set_sim_panel_visible.set(false))
            on_launch=Callback::new(move |run_info: simulation::types::RunInfo| {
                set_active_run.set(Some(run_info));
            })
            on_results=Callback::new(move |res: simulation::types::SimulationResults| {
                set_sim_results.set(Some(res));
            })
            active_run=Signal::derive(move || active_run.get())
            model_name=Signal::derive(move || loaded_model_name.get())
            json_path=Signal::derive(move || loaded_file_path.get())
            model_json=Signal::derive(move || model_json_context.get())
        />
        <div class="h-screen"
             tabindex="0"
             id="bevy-container"
             on:click=move |_| {
                 leptos::logging::log!("🎯 Container clicked - should be focused for key events");
             }
             on:keydown={
                 let cancel_mode_writer = cancel_mode_writer;
                 move |ev| {
                     // Log all key presses for debugging
                     leptos::logging::log!("Key pressed: {} (code: {})", ev.key(), ev.code());

                     // Handle zoom keys directly in JavaScript to bypass Bevy keyboard focus issues
                     if ev.key() == "=" || ev.key() == "+" || ev.code() == "NumpadAdd" {
                         ev.prevent_default();
                         leptos::logging::log!("🔍 ZOOM IN detected via JavaScript - sending to Bevy");
                         zoom_event_writer.send(ZoomEvent::ZoomIn).ok();
                     } else if ev.key() == "-" || ev.key() == "_" || ev.code() == "NumpadSubtract" {
                         ev.prevent_default();
                         leptos::logging::log!("🔍 ZOOM OUT detected via JavaScript - sending to Bevy");
                         zoom_event_writer.send(ZoomEvent::ZoomOut).ok();
                     } else if ev.key() == "Escape" {
                         ev.prevent_default();
                         leptos::logging::log!("❌ ESC detected via JavaScript - cancelling mode");
                         cancel_mode_writer.send(CancelModeEvent).ok();
                     }
                 }
             }>
            <BevyCanvas init=move || {
                init_bevy_app(
                    selected_details_query,
                    interface_details_query,
                    interaction_details_query,
                    external_entity_details_query,
                    system_details_query,
                    sub_system_details_query,
                    is_same_as_id_query,
                    spatial_mode_duplex,
                    detach_event_receiver,
                    load_file_event_receiver,
                    tree_event_sender,
                    zoom_event_receiver,
                    deselect_event_receiver,
                    trigger_event_receiver,
                    palette_click_receiver,
                    save_success_event_receiver,
                    save_success_bevy_sender,
                    mode_change_sender,
                    cancel_mode_receiver,
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
            spatial_mode
            detach_event_sender
            deselect_event_sender=deselect_event_writer
        />
        <Toast
            message=Signal::derive(move || toast_message.get())
            visible=Signal::derive(move || toast_visible.get())
            on_hide=Callback::new(move |_| set_toast_visible.set(false))
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
