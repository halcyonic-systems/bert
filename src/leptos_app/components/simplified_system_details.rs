//! Simplified System Details UI Component
//!
//! Addresses the core UX problems identified:
//! 1. Separates system/boundary/environment into distinct panels
//! 2. Adds contextual help for advanced concepts  
//! 3. Progressive disclosure for complex DSA concepts
//! 4. Clear conceptual organization

use crate::leptos_app::components::{InputGroup, Checkbox, TextArea};
use crate::SystemQuery;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DetailsPanelMode {
    System,
    Boundary, 
    Environment,
}

#[component]
pub fn SimplifiedSystemDetails(
    system_query: RwSignalSynced<Option<SystemQuery>>,
    panel_mode: RwSignal<DetailsPanelMode>,
) -> impl IntoView {
    // System properties
    let name = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(name, _, _, _)| name.to_string())
            .unwrap_or_default()
    });
    let description = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, description, _, _)| description.text.clone())
            .unwrap_or_default()
    });
    let adaptable = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.complexity.is_adaptable())
            .unwrap_or_default()
    });
    let evolveable = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.complexity.is_evolveable())
            .unwrap_or_default()
    });
    let equivalence = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.equivalence.clone())
            .unwrap_or_default()
    });
    let time_unit = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.time_unit.clone())
            .unwrap_or_default()
    });
    let transformation = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.transformation.clone())
            .unwrap_or_default()
    });
    
    // Boundary properties
    let boundary_name = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.name.clone())
            .unwrap_or_default()
    });
    let boundary_description = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.description.clone())
            .unwrap_or_default()
    });
    let boundary_porosity = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.porosity as f64)
            .unwrap_or_default()
    });
    let perceptive_fuzziness = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.perceptive_fuzziness as f64)
            .unwrap_or_default()
    });
    
    // Environment properties  
    let environment_name = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, _, system_env)| system_env.name.clone())
            .unwrap_or_default()
    });
    let environment_description = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, _, system_env)| system_env.description.clone())
            .unwrap_or_default()
    });

    view! {
        // Panel Selection Buttons
        <div class="flex border-b border-gray-200 mb-4">
            <button
                type="button"
                class=move || {
                    let base = "px-4 py-2 text-sm font-medium border-b-2 transition-colors";
                    if panel_mode.get() == DetailsPanelMode::System {
                        format!("{} border-blue-500 text-blue-600", base)
                    } else {
                        format!("{} border-transparent text-gray-500 hover:text-gray-700", base)
                    }
                }
                on:click=move |_| panel_mode.set(DetailsPanelMode::System)
            >
                System
            </button>
            <button
                type="button"
                class=move || {
                    let base = "px-4 py-2 text-sm font-medium border-b-2 transition-colors";
                    if panel_mode.get() == DetailsPanelMode::Boundary {
                        format!("{} border-blue-500 text-blue-600", base)
                    } else {
                        format!("{} border-transparent text-gray-500 hover:text-gray-700", base)
                    }
                }
                on:click=move |_| panel_mode.set(DetailsPanelMode::Boundary)
            >
                Boundary
            </button>
            <button
                type="button"
                class=move || {
                    let base = "px-4 py-2 text-sm font-medium border-b-2 transition-colors";
                    if panel_mode.get() == DetailsPanelMode::Environment {
                        format!("{} border-blue-500 text-blue-600", base)
                    } else {
                        format!("{} border-transparent text-gray-500 hover:text-gray-700", base)
                    }
                }
                on:click=move |_| panel_mode.set(DetailsPanelMode::Environment)
            >
                Environment
            </button>
        </div>

        // System Panel
        <Show when=move || panel_mode.get() == DetailsPanelMode::System>
            <div class="space-y-4">
                <InputGroup
                    id="system-name"
                    label="Name"
                    placeholder="System Name"
                    value=name
                    on_input=move |value: String| {
                        system_query.write().as_mut().map(|(name, _, _, _)| name.set(value));
                    }
                />
                <TextArea
                    id="system-description"
                    label="Description"
                    placeholder="Add a description"
                    text=description
                    on_input=move |value: String| {
                        system_query
                            .write()
                            .as_mut()
                            .map(|(_, description, _, _)| description.text = value);
                    }
                />
                
                // Complexity with contextual help
                <div class="space-y-2">
                    <div class="flex items-center gap-2">
                        <label class="block font-medium text-gray-900 text-sm">Complexity</label>
                        <div class="relative group">
                            <button 
                                type="button"
                                class="text-gray-400 hover:text-blue-600 p-1 rounded-full hover:bg-blue-50 transition-colors"
                            >
                                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
                                </svg>
                            </button>
                            <div class="absolute left-6 top-0 invisible group-hover:visible bg-gray-800 text-white text-sm p-3 rounded-lg shadow-lg z-10 w-80 transition-opacity">
                                <div class="font-medium mb-1">Mobus on System Complexity:</div>
                                <div>
                                    "\"" <strong>"Adaptable"</strong> " systems can adjust their behavior in response to environmental changes. " <strong>"Evolveable"</strong> " systems can change their structure and create new behaviors.\""
                                </div>
                                <div class="absolute -left-1 top-3 w-2 h-2 bg-gray-800 rotate-45"></div>
                            </div>
                        </div>
                    </div>
                    <div class="flex gap-6">
                        <Checkbox
                            id="system-adaptable"
                            label="Adaptable"
                            checked=adaptable
                            on_toggle=move |value: bool| {
                                system_query
                                    .write()
                                    .as_mut()
                                    .map(|(_, _, system, _)| system.complexity.set_adaptable(value));
                            }
                        />
                        <Checkbox
                            id="system-evolveable"
                            label="Evolveable"
                            checked=evolveable
                            on_toggle=move |value: bool| {
                                system_query
                                    .write()
                                    .as_mut()
                                    .map(|(_, _, system, _)| system.complexity.set_evolveable(value));
                            }
                        />
                    </div>
                </div>

                // Advanced Properties (collapsed by default)
                <details class="border border-gray-200 rounded-lg">
                    <summary class="cursor-pointer p-3 font-medium text-gray-700 bg-gray-50 rounded-t-lg hover:bg-gray-100">
                        Advanced Properties
                    </summary>
                    <div class="p-3 space-y-4">
                        <div>
                            <div class="flex items-center gap-2 mb-2">
                                <label class="block font-medium text-gray-700 text-sm">Equivalence</label>
                                <div class="relative group">
                                    <button 
                                        type="button"
                                        class="text-gray-400 hover:text-blue-600 p-1 rounded-full hover:bg-blue-50 transition-colors"
                                    >
                                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
                                        </svg>
                                    </button>
                                    <div class="absolute left-6 top-0 invisible group-hover:visible bg-gray-800 text-white text-sm p-3 rounded-lg shadow-lg z-10 w-72 transition-opacity">
                                        <div class="font-medium mb-1">System Equivalence:</div>
                                        <div>
                                            "Mathematical relationship describing how this system relates to others. See " <strong>"Model Browser"</strong> " examples for conceptual approaches."
                                        </div>
                                        <div class="absolute -left-1 top-3 w-2 h-2 bg-gray-800 rotate-45"></div>
                                    </div>
                                </div>
                            </div>
                            <InputGroup
                                id="system-equivalence"
                                label=""
                                placeholder="Mathematical relationship (optional)"
                                value=equivalence
                                on_input=move |value: String| {
                                    system_query.write().as_mut().map(|(_, _, system, _)| system.equivalence = value);
                                }
                            />
                        </div>
                        
                        <div>
                            <div class="flex items-center gap-2 mb-2">
                                <label class="block font-medium text-gray-700 text-sm">Time Unit</label>
                                <div class="relative group">
                                    <button 
                                        type="button"
                                        class="text-gray-400 hover:text-blue-600 p-1 rounded-full hover:bg-blue-50 transition-colors"
                                    >
                                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
                                        </svg>
                                    </button>
                                    <div class="absolute left-6 top-0 invisible group-hover:visible bg-gray-800 text-white text-sm p-3 rounded-lg shadow-lg z-10 w-72 transition-opacity">
                                        <div class="font-medium mb-1">System Time Unit:</div>
                                        <div>
                                            "Fundamental time scale for system dynamics and modeling. " <strong>"Important"</strong> " for systems scientists building simulations."
                                        </div>
                                        <div class="absolute -left-1 top-3 w-2 h-2 bg-gray-800 rotate-45"></div>
                                    </div>
                                </div>
                            </div>
                            <InputGroup
                                id="system-time-unit"
                                label=""
                                placeholder="e.g., seconds, hours, years"
                                value=time_unit
                                on_input=move |value: String| {
                                    system_query.write().as_mut().map(|(_, _, system, _)| system.time_unit = value);
                                }
                            />
                        </div>

                        <div>
                            <div class="flex items-center gap-2 mb-2">
                                <label class="block font-medium text-gray-700 text-sm">Transformation Functions</label>
                                <div class="relative group">
                                    <button 
                                        type="button"
                                        class="text-gray-400 hover:text-blue-600 p-1 rounded-full hover:bg-blue-50 transition-colors"
                                    >
                                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
                                        </svg>
                                    </button>
                                    <div class="absolute left-6 top-0 invisible group-hover:visible bg-gray-800 text-white text-sm p-3 rounded-lg shadow-lg z-10 w-80 transition-opacity">
                                        <div class="font-medium mb-1">Transformation Functions:</div>
                                        <div>
                                            "Mathematical equations derived through measurement. In " <strong>"future versions"</strong> ", these will be visual equation builders rather than text."
                                        </div>
                                        <div class="absolute -left-1 top-3 w-2 h-2 bg-gray-800 rotate-45"></div>
                                    </div>
                                </div>
                            </div>
                            <TextArea
                                id="transformation"
                                label=""
                                placeholder="Mathematical equations (will become visual equation builder)"
                                text=transformation
                                on_input=move |value: String| {
                                    system_query
                                        .write()
                                        .as_mut()
                                        .map(|(_, _, system, _)| system.transformation = value);
                                }
                            />
                        </div>
                    </div>
                </details>
            </div>
        </Show>

        // Boundary Panel  
        <Show when=move || panel_mode.get() == DetailsPanelMode::Boundary>
            <div class="space-y-4">
                <div class="bg-blue-50 border border-blue-200 rounded-lg p-3 mb-4">
                    <p class="text-sm text-blue-800">
                        <strong>Boundary</strong> - The interface between system and environment. 
                        Contains all the interfaces that allow inputs and outputs to flow.
                    </p>
                </div>
                
                <InputGroup
                    id="boundary-name"
                    label="Name"
                    placeholder="Boundary Name"
                    value=boundary_name
                    on_input=move |value| {
                        system_query.write().as_mut().map(|(_, _, system, _)| system.boundary.name = value);
                    }
                />
                <TextArea
                    id="boundary-description"
                    label="Description"
                    placeholder="Describe the boundary"
                    text=boundary_description
                    on_input=move |value| {
                        system_query.write().as_mut().map(|(_, _, system, _)| system.boundary.description = value);
                    }
                />
                
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <div class="flex items-center gap-2 mb-2">
                            <label class="block font-medium text-gray-700 text-sm">Porosity</label>
                            <div class="relative group">
                                <button 
                                    type="button"
                                    class="text-gray-400 hover:text-blue-600 p-1 rounded-full hover:bg-blue-50 transition-colors"
                                >
                                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
                                    </svg>
                                </button>
                                <div class="absolute left-6 top-0 invisible group-hover:visible bg-gray-800 text-white text-sm p-3 rounded-lg shadow-lg z-10 w-64 transition-opacity">
                                    <div class="font-medium mb-1">Boundary Porosity:</div>
                                    <div>
                                        "How permeable the boundary is to flows. " <strong>"0"</strong> " = completely closed, " <strong>"1"</strong> " = completely open."
                                    </div>
                                    <div class="absolute -left-1 top-3 w-2 h-2 bg-gray-800 rotate-45"></div>
                                </div>
                            </div>
                        </div>
                        <input
                            type="range"
                            min="0"
                            max="1"
                            step="0.1"
                            value=boundary_porosity
                            class="w-full"
                            on:input=move |ev| {
                                let value = ev.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value().parse::<f32>().unwrap_or(0.0);
                                system_query.write().as_mut().map(|(_, _, system, _)| system.boundary.porosity = value);
                            }
                        />
                        <div class="text-xs text-gray-500 mt-1">{boundary_porosity}</div>
                    </div>
                    
                    <div>
                        <div class="flex items-center gap-2 mb-2">
                            <label class="block font-medium text-gray-700 text-sm">Perceptive Fuzziness</label>
                            <div class="relative group">
                                <button 
                                    type="button"
                                    class="text-gray-400 hover:text-blue-600 p-1 rounded-full hover:bg-blue-50 transition-colors"
                                >
                                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
                                    </svg>
                                </button>
                                <div class="absolute left-6 top-0 invisible group-hover:visible bg-gray-800 text-white text-sm p-3 rounded-lg shadow-lg z-10 w-64 transition-opacity">
                                    <div class="font-medium mb-1">Perceptive Fuzziness:</div>
                                    <div>
                                        "How clearly the boundary can be distinguished. " <strong>"0"</strong> " = sharp boundary, " <strong>"1"</strong> " = very fuzzy boundary."
                                    </div>
                                    <div class="absolute -left-1 top-3 w-2 h-2 bg-gray-800 rotate-45"></div>
                                </div>
                            </div>
                        </div>
                        <input
                            type="range"
                            min="0"
                            max="1"
                            step="0.1"
                            value=perceptive_fuzziness
                            class="w-full"
                            on:input=move |ev| {
                                let value = ev.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value().parse::<f32>().unwrap_or(0.0);
                                system_query.write().as_mut().map(|(_, _, system, _)| system.boundary.perceptive_fuzziness = value);
                            }
                        />
                        <div class="text-xs text-gray-500 mt-1">{perceptive_fuzziness}</div>
                    </div>
                </div>
                
                <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-3">
                    <p class="text-sm text-yellow-800">
                        <strong>Note:</strong> Interfaces within this boundary will be manageable through the interface creation tools.
                    </p>
                </div>
            </div>
        </Show>

        // Environment Panel
        <Show when=move || panel_mode.get() == DetailsPanelMode::Environment>
            <div class="space-y-4">
                <div class="bg-green-50 border border-green-200 rounded-lg p-3 mb-4">
                    <p class="text-sm text-green-800">
                        <strong>Environment</strong> - The external context surrounding this system. 
                        Contains sources, sinks, and other systems that interact with this one.
                    </p>
                </div>
                
                <InputGroup
                    id="environment-name"
                    label="Name"
                    placeholder="Environment Name"
                    value=environment_name
                    on_input=move |value| {
                        system_query.write().as_mut().map(|(_, _, _, env)| env.name = value);
                    }
                />
                <TextArea
                    id="environment-description"
                    label="Description"
                    placeholder="Describe the environment context"
                    text=environment_description
                    on_input=move |value| {
                        system_query.write().as_mut().map(|(_, _, _, env)| env.description = value);
                    }
                />
                
                <div class="bg-blue-50 border border-blue-200 rounded-lg p-3">
                    <p class="text-sm text-blue-800">
                        <strong>Note:</strong> Sources and sinks in this environment will be manageable through the external entity creation tools.
                    </p>
                </div>
            </div>
        </Show>
    }
}