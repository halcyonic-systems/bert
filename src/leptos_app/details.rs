use crate::bevy_app::components::{HcgsArchetype, SpatialDetailPanelMode};
use crate::bevy_app::data_model::Complexity;
use crate::bevy_app::smart_parameters::{ParameterValue, SmartParameter};
use crate::leptos_app::components::{
    Button, Divider, InputGroup, RadioGroup, SelectGroup, Slider, TextArea,
};
use crate::{
    DeselectAllEvent, DetachMarkerLabelEvent, ExternalEntityQuery, InteractionQuery,
    InteractionType, InteractionUsability, InterfaceQuery, IsSameAsIdQuery, SubSystemQuery,
    SubstanceType, SystemElement, SystemQuery,
};
use enum_iterator::all;
use leptos::prelude::*;
use leptos::tachys::html::property::IntoProperty;
use leptos::tachys::renderer::dom::Element;
use leptos::tachys::renderer::Rndr;
use leptos_bevy_canvas::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use wasm_bindgen::JsValue;

/// Panel mode for details view
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DetailsPanelMode {
    System,
    Boundary,
    Environment,
}

/// Complexity levels for systems (Simple → Adaptable → Evolveable)
/// Based on Mobus systems science: Evolveable implies Adaptable
#[derive(Copy, Clone, PartialEq, Eq, Default)]
enum ComplexityLevel {
    #[default]
    Simple,
    Adaptable,
    Evolveable,
}

impl std::fmt::Display for ComplexityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexityLevel::Simple => write!(f, "Simple"),
            ComplexityLevel::Adaptable => write!(f, "Adaptable"),
            ComplexityLevel::Evolveable => write!(f, "Evolveable"),
        }
    }
}

impl From<&Complexity> for ComplexityLevel {
    fn from(c: &Complexity) -> Self {
        match c {
            Complexity::Complex {
                adaptable: true,
                evolveable: true,
            } => ComplexityLevel::Evolveable,
            Complexity::Complex {
                adaptable: true,
                evolveable: false,
            } => ComplexityLevel::Adaptable,
            _ => ComplexityLevel::Simple,
        }
    }
}

impl From<ComplexityLevel> for Complexity {
    fn from(level: ComplexityLevel) -> Self {
        match level {
            ComplexityLevel::Simple => Complexity::Complex {
                adaptable: false,
                evolveable: false,
            },
            ComplexityLevel::Adaptable => Complexity::Complex {
                adaptable: true,
                evolveable: false,
            },
            ComplexityLevel::Evolveable => Complexity::Complex {
                adaptable: true,
                evolveable: true,
            },
        }
    }
}

#[derive(Copy, Clone, Default)]
struct DecimalWrapper(Decimal);

impl FromStr for DecimalWrapper {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut s = s.trim().to_string();

        if s.ends_with(".") || s.ends_with(",") {
            s.push('0');
            s.push('0');
        }
        Ok(DecimalWrapper(Decimal::from_str(&s)?))
    }
}

impl From<DecimalWrapper> for JsValue {
    fn from(value: DecimalWrapper) -> Self {
        value.0.to_string().into()
    }
}
impl IntoProperty for DecimalWrapper {
    type State = (Element, JsValue);
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element, key: &str) -> Self::State {
        let value = self.into();
        Rndr::set_property(el, key, &value);
        (el.clone(), value)
    }

    fn build(self, el: &Element, key: &str) -> Self::State {
        let value = self.into();
        Rndr::set_property(el, key, &value);
        (el.clone(), value)
    }

    fn rebuild(self, state: &mut Self::State, key: &str) {
        let (el, prev) = state;
        let value = self.into();
        Rndr::set_property(el, key, &value);
        *prev = value;
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }
}

#[component]
pub fn Details(
    selected: RwSignalSynced<Option<(SystemElement,)>>,
    interaction_details: RwSignalSynced<Option<InteractionQuery>>,
    interface_details: RwSignalSynced<Option<InterfaceQuery>>,
    external_entity_details: RwSignalSynced<Option<ExternalEntityQuery>>,
    system_details: RwSignalSynced<Option<SystemQuery>>,
    sub_system_details: RwSignalSynced<Option<SubSystemQuery>>,
    is_same_as_id: RwSignalSynced<Option<IsSameAsIdQuery>>,
    spatial_mode: RwSignalSynced<SpatialDetailPanelMode>,
    detach_event_sender: LeptosEventSender<DetachMarkerLabelEvent>,
    deselect_event_sender: LeptosEventSender<DeselectAllEvent>,
) -> impl IntoView {
    // Panel mode signal for simplified system details
    let _panel_mode = RwSignal::new(DetailsPanelMode::System);

    view! {
        <div
            class="relative z-10"
            aria-labelledby="slide-over-title"
            role="dialog"
            aria-modal="true"
        >

            <div class="overflow-hidden fixed">
                <div class="overflow-hidden absolute">
                    <div class="flex fixed inset-y-0 right-0 pl-10 max-w-full pointer-events-none">

                        <div
                            class="w-screen max-w-md transition duration-500 ease-in-out transform pointer-events-auto sm:duration-700"
                            class:translate-x-full=move || selected.get().is_none()
                            class:translate-x-0=move || selected.get().is_some()
                        >
                            <div class="flex overflow-y-scroll flex-col py-6 h-full bg-white shadow-xl">
                                <div class="px-4 sm:px-6">
                                    <div class="flex justify-between items-start">
                                        <div class="flex items-center space-x-2">
                                            <h2
                                                class="text-base font-semibold text-gray-900"
                                                id="slide-over-title"
                                            >
                                                Element Details
                                            </h2>
                                        </div>
                                        <div class="flex items-center ml-3 h-7">
                                            <button
                                                type="button"
                                                class="relative p-2 text-gray-400 bg-white rounded-md hover:text-gray-500 focus:ring-2 focus:ring-cyan-500 focus:ring-offset-2 focus:outline-none"
                                                on:click=move |_| {
                                                    // Clear selection to close the details panel
                                                    leptos::logging::log!("🔽 Close button clicked - clearing both Leptos and Bevy selection");

                                                    // Clear all Leptos selection signals
                                                    selected.update(|s| *s = None);
                                                    interaction_details.update(|s| *s = None);
                                                    interface_details.update(|s| *s = None);
                                                    external_entity_details.update(|s| *s = None);
                                                    system_details.update(|s| *s = None);
                                                    sub_system_details.update(|s| *s = None);
                                                    is_same_as_id.update(|s| *s = None);

                                                    // Also clear Bevy selection state so elements can be re-selected
                                                    deselect_event_sender.send(DeselectAllEvent).ok();
                                                }
                                            >
                                                <span class="sr-only">Close panel</span>
                                                <svg
                                                    class="w-6 h-6"
                                                    fill="none"
                                                    viewBox="0 0 24 24"
                                                    stroke-width="1.5"
                                                    stroke="currentColor"
                                                    aria-hidden="true"
                                                >
                                                    <path
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        d="M6 18 18 6M6 6l12 12"
                                                    />
                                                </svg>
                                            </button>
                                        </div>
                                    </div>
                                </div>
                                <div class="relative flex-1 px-4 mt-6 sm:px-6">
                                    <Show when=move || interaction_details.get().is_some()>
                                        <InteractionDetails interaction_query=interaction_details />
                                    </Show>
                                    <Show when=move || interface_details.get().is_some()>
                                        <InterfaceDetails interface_query=interface_details />
                                    </Show>
                                    <Show when=move || external_entity_details.get().is_some()>
                                        <ExternalEntityDetails
                                            external_entity_query=external_entity_details
                                            is_same_as_id_query=is_same_as_id
                                            detach_event_sender=detach_event_sender
                                        />
                                    </Show>
                                    <Show when=move || { system_details.get().is_some() }>
                                        <SystemDetails system_query=system_details spatial_mode=spatial_mode />
                                    </Show>
                                    <Show when=move || { sub_system_details.get().is_some() }>
                                        <SubSystemDetails sub_system_query=sub_system_details />
                                    </Show>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn InterfaceDetails(interface_query: RwSignalSynced<Option<InterfaceQuery>>) -> impl IntoView {
    let name = Signal::derive(move || {
        interface_query
            .read()
            .as_ref()
            .map(|(name, _, _)| name.to_string())
            .unwrap_or_default()
    });

    let description = Signal::derive(move || {
        interface_query
            .read()
            .as_ref()
            .map(|(_, el_desc, _)| el_desc.text.clone())
            .unwrap_or_default()
    });

    let protocol = Signal::derive(move || {
        interface_query
            .read()
            .as_ref()
            .map(|(_, _, interface)| interface.protocol.clone())
            .unwrap_or_default()
    });

    view! {
        <InputGroup
            id="name"
            label="Name"
            placeholder="Interface Name"
            value=name
            on_input=move |value: String| {
                interface_query.write().as_mut().map(|(name, _, _)| name.set(value));
            }
        />

        <TextArea
            id="description"
            label="Description"
            placeholder="Add a description"
            text=description
            on_input=move |value: String| {
                interface_query.write().as_mut().map(|(_, el_desc, _)| el_desc.text = value);
            }
        />

        <div class="mb-4">
            <label class="flex items-center mb-2">
                <span class="block text-sm font-medium leading-6 text-gray-900">Protocol (Optional)</span>
                <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm"
                      title="The rules for how substances cross this interface. Examples: 'HTTPS POST to /api/orders', 'phone call', 'batch file transfer', 'face-to-face meeting'. Can be as technical (code-level) or conceptual (business process) as needed for your analysis.">
                    ?
                </span>
            </label>
            <TextArea
                id="protocol"
                label=""
                placeholder="e.g., HTTPS API request, phone call, batch transfer, queue system..."
                text=protocol
                on_input=move |value: String| {
                    interface_query
                        .write()
                        .as_mut()
                        .map(|(_, _, interface)| interface.protocol = value);
                }
            />
        </div>
    }
}

#[component]
pub fn SimpleParameterInput(
    interaction_query: RwSignalSynced<Option<InteractionQuery>>,
) -> impl IntoView {
    // Get parameters from the interaction (using basic Parameter struct)
    let parameters = Memo::new(move |_| {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.parameters.clone())
            .unwrap_or_default()
    });

    // State for adding new parameter
    let (new_param_name, set_new_param_name) = signal(String::new());
    let (new_param_value, set_new_param_value) = signal(String::new());
    let (new_param_unit, set_new_param_unit) = signal(String::new());

    let add_parameter = move |_| {
        let name = new_param_name.get_untracked();
        let value = new_param_value.get_untracked();
        let unit = new_param_unit.get_untracked();

        if !name.is_empty() {
            let new_parameter = crate::Parameter {
                id: uuid::Uuid::new_v4(),
                name: name.clone(),
                value: value.clone(),
                unit: unit.clone(),
            };

            interaction_query
                .write()
                .as_mut()
                .map(|(_, _, interaction)| {
                    interaction.parameters.push(new_parameter);
                });

            // Reset form
            set_new_param_name.set(String::new());
            set_new_param_value.set(String::new());
            set_new_param_unit.set(String::new());
        }
    };

    view! {
        // Existing parameters list
        <Show when=move || !parameters.get().is_empty()>
            <div class="space-y-2 mb-3">
                {move || {
                    parameters.get().into_iter().map(|param| {
                        let param_id = param.id;
                        let initial_name = param.name.clone();
                        let initial_value = param.value.clone();
                        let initial_unit = param.unit.clone();
                        view! {
                            <div class="flex items-center gap-2 p-2 bg-gray-50 rounded">
                                <div class="flex-1 min-w-0 space-y-1">
                                    <input
                                        type="text"
                                        class="w-full px-2 py-0.5 text-sm font-medium text-gray-900 border border-gray-200 rounded focus:ring-cyan-500 focus:border-cyan-500"
                                        prop:value=initial_name
                                        on:change=move |ev| {
                                            let new_name = event_target_value(&ev);
                                            interaction_query.write().as_mut().map(|(_, _, interaction)| {
                                                if let Some(p) = interaction.parameters.iter_mut().find(|p| p.id == param_id) {
                                                    p.name = new_name;
                                                }
                                            });
                                        }
                                        placeholder="Name"
                                    />
                                    <div class="flex gap-1">
                                        <input
                                            type="text"
                                            class="flex-1 px-2 py-0.5 text-xs text-gray-600 border border-gray-200 rounded focus:ring-cyan-500 focus:border-cyan-500"
                                            prop:value=initial_value
                                            on:change=move |ev| {
                                                let new_value = event_target_value(&ev);
                                                interaction_query.write().as_mut().map(|(_, _, interaction)| {
                                                    if let Some(p) = interaction.parameters.iter_mut().find(|p| p.id == param_id) {
                                                        p.value = new_value;
                                                    }
                                                });
                                            }
                                            placeholder="Value"
                                        />
                                        <input
                                            type="text"
                                            class="w-16 px-2 py-0.5 text-xs text-gray-600 border border-gray-200 rounded focus:ring-cyan-500 focus:border-cyan-500"
                                            prop:value=initial_unit
                                            on:change=move |ev| {
                                                let new_unit = event_target_value(&ev);
                                                interaction_query.write().as_mut().map(|(_, _, interaction)| {
                                                    if let Some(p) = interaction.parameters.iter_mut().find(|p| p.id == param_id) {
                                                        p.unit = new_unit;
                                                    }
                                                });
                                            }
                                            placeholder="Unit"
                                        />
                                    </div>
                                </div>
                                <button
                                    type="button"
                                    on:click=move |_| {
                                        interaction_query.write().as_mut().map(|(_, _, interaction)| {
                                            interaction.parameters.retain(|p| p.id != param_id);
                                        });
                                    }
                                    class="p-1 text-gray-400 hover:text-red-600 rounded hover:bg-red-50 flex-shrink-0"
                                    title="Remove attribute"
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </Show>

        // Add new attribute form
        <div class="p-3 bg-blue-50 border border-blue-200 rounded">
            <div class="text-xs font-medium text-blue-900 mb-2">Add Attribute</div>
            <div class="space-y-2">
                <input
                    type="text"
                    placeholder="Name (e.g., temperature, efficiency)"
                    class="w-full px-2 py-1 text-sm border border-gray-300 rounded focus:ring-cyan-500 focus:border-cyan-500"
                    prop:value=new_param_name
                    on:input=move |ev| set_new_param_name.set(event_target_value(&ev))
                />
                <div class="flex gap-2">
                    <input
                        type="text"
                        placeholder="Value"
                        class="flex-1 px-2 py-1 text-sm border border-gray-300 rounded focus:ring-cyan-500 focus:border-cyan-500"
                        prop:value=new_param_value
                        on:input=move |ev| set_new_param_value.set(event_target_value(&ev))
                    />
                    <input
                        type="text"
                        placeholder="Unit"
                        class="w-20 px-2 py-1 text-sm border border-gray-300 rounded focus:ring-cyan-500 focus:border-cyan-500"
                        prop:value=new_param_unit
                        on:input=move |ev| set_new_param_unit.set(event_target_value(&ev))
                    />
                </div>
                <button
                    type="button"
                    on:click=add_parameter
                    class="w-full px-3 py-1.5 text-sm font-medium text-white bg-cyan-600 rounded hover:bg-cyan-700"
                >
                    Add Attribute
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn SmartParameterWidget(
    parameter: SmartParameter,
    on_update: impl Fn(SmartParameter) + 'static + Copy,
    on_delete: impl Fn(uuid::Uuid) + 'static + Copy,
) -> impl IntoView {
    let name = parameter.name.clone();
    match parameter.value {
        ParameterValue::Numeric { value, unit } => {
            let param_name = parameter.name.clone();
            let param_name_for_value = param_name.clone();
            let param_name_for_unit = param_name.clone();
            let unit_for_value = unit.clone();
            let value_for_unit = value.clone();
            view! {
                <div class="flex items-center space-x-3 p-3 border border-gray-200 rounded-lg bg-blue-50">
                    <div class="flex-1">
                        <label class="block text-sm font-medium text-gray-700">{name.clone()}</label>
                        <div class="flex mt-1 space-x-2">
                            <input
                                type="text"
                                class="flex-1 px-3 py-1 text-sm border border-gray-300 rounded focus:ring-blue-500 focus:border-blue-500"
                                placeholder="Value"
                                value=value.clone()
                                on:input=move |ev| {
                                    let new_value = event_target_value(&ev);
                                    let updated_param = SmartParameter {
                                        id: parameter.id,
                                        name: param_name_for_value.clone(),
                                        value: ParameterValue::Numeric {
                                            value: new_value,
                                            unit: unit_for_value.clone()
                                        },
                                    };
                                    on_update(updated_param);
                                }
                            />
                            <input
                                type="text"
                                class="w-20 px-3 py-1 text-sm border border-gray-300 rounded focus:ring-blue-500 focus:border-blue-500"
                                placeholder="Unit"
                                value=unit.clone()
                                on:input=move |ev| {
                                    let new_unit = event_target_value(&ev);
                                    let updated_param = SmartParameter {
                                        id: parameter.id,
                                        name: param_name_for_unit.clone(),
                                        value: ParameterValue::Numeric {
                                            value: value_for_unit.clone(),
                                            unit: new_unit
                                        },
                                    };
                                    on_update(updated_param);
                                }
                            />
                        </div>
                    </div>
                    <span class="px-2 py-1 text-xs text-blue-600 bg-blue-100 rounded-full">Numeric</span>
                    <button
                        type="button"
                        on:click=move |_| on_delete(parameter.id)
                        class="p-1 text-gray-400 hover:text-red-600 rounded-full hover:bg-red-50 transition-colors"
                        title="Delete parameter"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
            }.into_any()
        }

        ParameterValue::Ordinal { level, options } => {
            let param_name = parameter.name.clone();
            let opts_for_handler = options.clone();
            view! {
                <div class="flex items-center space-x-3 p-3 border border-gray-200 rounded-lg bg-green-50">
                    <div class="flex-1">
                        <label class="block text-sm font-medium text-gray-700">{name.clone()}</label>
                        <select
                            class="mt-1 w-full px-3 py-1 text-sm border border-gray-300 rounded focus:ring-green-500 focus:border-green-500"
                            on:change=move |ev| {
                                let new_level = event_target_value(&ev);
                                let updated_param = SmartParameter {
                                    id: parameter.id,
                                    name: param_name.clone(),
                                    value: ParameterValue::Ordinal {
                                        level: new_level,
                                        options: opts_for_handler.clone()
                                    },
                                };
                                on_update(updated_param);
                            }
                        >
                            {options.iter().map(|option| {
                                let selected = option == &level;
                                view! {
                                    <option value=option.clone() selected=selected>
                                        {option.clone()}
                                    </option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>
                    <span class="px-2 py-1 text-xs text-green-600 bg-green-100 rounded-full">Ordinal</span>
                    <button
                        type="button"
                        on:click=move |_| on_delete(parameter.id)
                        class="p-1 text-gray-400 hover:text-red-600 rounded-full hover:bg-red-50 transition-colors"
                        title="Delete parameter"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
            }.into_any()
        }

        ParameterValue::Categorical {
            value: cat_value,
            options,
        } => {
            let param_name = parameter.name.clone();
            let opts_for_handler = options.clone();
            view! {
                <div class="flex items-center space-x-3 p-3 border border-gray-200 rounded-lg bg-purple-50">
                    <div class="flex-1">
                        <label class="block text-sm font-medium text-gray-700">{name.clone()}</label>
                        <select
                            class="mt-1 w-full px-3 py-1 text-sm border border-gray-300 rounded focus:ring-purple-500 focus:border-purple-500"
                            on:change=move |ev| {
                                let new_value = event_target_value(&ev);
                                let updated_param = SmartParameter {
                                    id: parameter.id,
                                    name: param_name.clone(),
                                    value: ParameterValue::Categorical {
                                        value: new_value,
                                        options: opts_for_handler.clone()
                                    },
                                };
                                on_update(updated_param);
                            }
                        >
                            {options.iter().map(|option| {
                                let selected = option == &cat_value;
                                view! {
                                    <option value=option.clone() selected=selected>
                                        {option.clone()}
                                    </option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>
                    <span class="px-2 py-1 text-xs text-purple-600 bg-purple-100 rounded-full">Categorical</span>
                    <button
                        type="button"
                        on:click=move |_| on_delete(parameter.id)
                        class="p-1 text-gray-400 hover:text-red-600 rounded-full hover:bg-red-50 transition-colors"
                        title="Delete parameter"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
            }.into_any()
        }

        ParameterValue::Boolean {
            value: bool_value,
            true_label,
            false_label,
        } => {
            let param_name = parameter.name.clone();
            let param_name_for_true = param_name.clone();
            let param_name_for_false = param_name.clone();
            let t_label = true_label.clone();
            let f_label = false_label.clone();
            let t_label_for_true = t_label.clone();
            let f_label_for_true = f_label.clone();
            let t_label_for_false = t_label.clone();
            let f_label_for_false = f_label.clone();
            view! {
                <div class="flex items-center space-x-3 p-3 border border-gray-200 rounded-lg bg-orange-50">
                    <div class="flex-1">
                        <label class="block text-sm font-medium text-gray-700">{name.clone()}</label>
                        <div class="mt-1 flex items-center space-x-3">
                            <label class="flex items-center">
                                <input
                                    type="radio"
                                    name=format!("param_{}", name)
                                    checked=bool_value
                                    class="mr-2 text-orange-600 focus:ring-orange-500"
                                    on:change=move |_| {
                                        let updated_param = SmartParameter {
                                            id: parameter.id,
                                            name: param_name_for_true.clone(),
                                            value: ParameterValue::Boolean {
                                                value: true,
                                                true_label: t_label_for_true.clone(),
                                                false_label: f_label_for_true.clone()
                                            },
                                        };
                                        on_update(updated_param);
                                    }
                                />
                                <span class="text-sm">{true_label.clone()}</span>
                            </label>
                            <label class="flex items-center">
                                <input
                                    type="radio"
                                    name=format!("param_{}", name)
                                    checked=!bool_value
                                    class="mr-2 text-orange-600 focus:ring-orange-500"
                                    on:change=move |_| {
                                        let updated_param = SmartParameter {
                                            id: parameter.id,
                                            name: param_name_for_false.clone(),
                                            value: ParameterValue::Boolean {
                                                value: false,
                                                true_label: t_label_for_false.clone(),
                                                false_label: f_label_for_false.clone()
                                            },
                                        };
                                        on_update(updated_param);
                                    }
                                />
                                <span class="text-sm">{false_label.clone()}</span>
                            </label>
                        </div>
                    </div>
                    <span class="px-2 py-1 text-xs text-orange-600 bg-orange-100 rounded-full">Boolean</span>
                    <button
                        type="button"
                        on:click=move |_| on_delete(parameter.id)
                        class="p-1 text-gray-400 hover:text-red-600 rounded-full hover:bg-red-50 transition-colors"
                        title="Delete parameter"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
            }.into_any()
        }
    }
}

#[component]
pub fn InteractionDetails(
    interaction_query: RwSignalSynced<Option<InteractionQuery>>,
) -> impl IntoView {
    let name = Memo::new(move |_| {
        interaction_query
            .read()
            .as_ref()
            .map(|(name, _, _)| name.to_string())
            .unwrap_or_default()
    });

    let description = Memo::new(move |_| {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, el_desc, _)| el_desc.text.clone())
            .unwrap_or_default()
    });

    let usability = Memo::new(move |_| {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.usability)
    });

    let interaction_type = Memo::new(move |_| {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.interaction_type)
    });

    let substance_type = Memo::new(move |_| {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.substance_type)
    });

    // Hidden dynamic fields for v0.2.0 - too complex for structural analysis focus
    // let substance_sub_type = Memo::new(move |_| interaction_query
    //     .read()
    //     .as_ref()
    //     .map(|(_, _, interaction)| interaction.substance_sub_type.clone())
    //     .unwrap_or_default());

    // let substance_unit = Memo::new(move |_| interaction_query
    //     .read()
    //     .as_ref()
    //     .map(|(_, _, interaction)| interaction.unit.clone())
    //     .unwrap_or_default());

    // let substance_amount = Memo::new(move |_| interaction_query
    //     .read()
    //     .as_ref()
    //     .map(|(_, _, interaction)| DecimalWrapper(interaction.amount))
    //     .unwrap_or_default());

    // let parameters = Memo::new(move |_| interaction_query
    //     .read()
    //     .as_ref()
    //     .map(|(_, _, interaction)| interaction.parameters.clone())
    //     .unwrap_or(Vec::new()));

    let usability_types = all::<InteractionUsability>().collect::<Vec<_>>();
    let interaction_types = all::<InteractionType>().collect::<Vec<_>>();
    let substance_types = all::<SubstanceType>().collect::<Vec<_>>();

    view! {
        <InputGroup
            id="name"
            label="Name"
            placeholder="Interaction Name"
            value=name
            on_input=move |value: String| {
                interaction_query.write().as_mut().map(|(name, _, _)| name.set(value));
            }
        />

        <TextArea
            id="description"
            label="Description"
            placeholder="Add a description"
            text=description
            on_input=move |value: String| {
                interaction_query.write().as_mut().map(|(_, el_desc, _)| el_desc.text = value);
            }
        />

        <div class="mb-4">
            <label class="flex items-center mb-2">
                <span class="block text-sm font-medium leading-6 text-gray-900">Interaction Type</span>
                <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm"
                      title="Flow: gradual movement, Force: immediate influence">
                    ?
                </span>
            </label>
            <SelectGroup
                id="interaction-type"
                label=""
                options=interaction_types
                selected_option=interaction_type
                on_change=move |value| {
                    interaction_query
                        .write()
                        .as_mut()
                        .map(|(_, _, interaction)| value.map(|ty| interaction.interaction_type = ty));
                }
            />
        </div>

        <RadioGroup
            id="substance-type"
            label="Substance Type"
            options=substance_types
            selected=substance_type
            on_change=move |value| {
                interaction_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interaction)| interaction.substance_type = value);
            }
            tooltip="The fundamental category of what flows between system elements"
            option_descriptions=vec![
                "Power, work capacity, or transformative potential (electrical, thermal, mechanical)",
                "Physical matter or tangible resources (raw materials, fluids, objects)",
                "Data, signals, or symbolic content (information, commands, feedback)",
            ]
        />

        <RadioGroup
            id="interaction-usability"
            label="Usability"
            options=usability_types
            selected=usability
            on_change=move |value| {
                interaction_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interaction)| interaction.usability = value);
            }
            tooltip="Classification based on utility and direction"
            option_descriptions=vec![
                "Useful input: enhances system capabilities or provides needed materials/energy",
                "Harmful input: degrades system performance or introduces unwanted effects",
                "Useful output: fulfills the system's intended purpose or provides value",
                "Harmful output: unwanted byproducts or system inefficiency",
            ]
        />

        // Hidden dynamic fields for v0.2.0 - focus on structural analysis
        // <InputGroup
        //     id="substance-sub-type"
        //     label="Substance Sub Type"
        //     placeholder=""
        //     value=substance_sub_type
        //     on_input=move |value: String| {
        //         interaction_query
        //             .write()
        //             .as_mut()
        //             .map(|(_, _, interaction)| interaction.substance_sub_type = value);
        //     }
        // />

        // <InputGroup
        //     id="substance-unit"
        //     label="Substance Unit"
        //     placeholder=""
        //     value=substance_unit
        //     on_input=move |value: String| {
        //         interaction_query
        //             .write()
        //             .as_mut()
        //             .map(|(_, _, interaction)| interaction.unit = value);
        //     }
        // />

        // <InputGroup
        //     id="substance-amount"
        //     label="Substance Amount"
        //     placeholder=""
        //     type_="text"
        //     value=substance_amount
        //     on_input=move |value: DecimalWrapper| {
        //         interaction_query
        //             .write()
        //             .as_mut()
        //             .map(|(_, _, interaction)| interaction.amount = value.0);
        //     }
        // />

        // Parameters section - too dynamic for v0.2.0 structural analysis focus
        // <Divider name="Parameters" />

        // <button
        //     type="button"
        //     on:click=move |_| {
        //         interaction_query
        //             .write()
        //             .as_mut()
        //             .map(|(_, _, interaction)| interaction.parameters.push(Parameter::default()));
        //     }
        //     class="py-1.5 px-3 w-full text-sm font-semibold text-white rounded-full shadow-sm hover:bg-cyan-800 bg-cyan-950 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-cyan-900"
        // >
        //     Add
        // </button>

        // <div class="grid grid-cols-7 gap-x-4 mt-3">
        //     <Show when=move || { parameters.get().len() > 0 }>
        //         <div class="flex col-span-2 justify-self-center text-center item-center">Name</div>
        //         <div class="flex col-span-2 justify-self-center text-center item-center">Value</div>
        //         <div class="flex col-span-2 justify-self-center text-center item-center">Unit</div>
        //         <div class="flex justify-self-center item-center"></div>
        //     </Show>
        //     <For
        //         each=move || parameters.get()
        //         key=|param| param.id.clone()
        //         let(Parameter { id, name, value, unit })
        //     >
        //         <InputGroup
        //             id="name"
        //             placeholder="Name"
        //             value=name
        //             label_class="self-center"
        //             input_class="ml-2"
        //             on_input=move |val| {
        //                 interaction_query
        //                     .write()
        //                     .as_mut()
        //                     .map(|(_, _, interaction)| {
        //                         let mut param = interaction
        //                             .parameters
        //                             .iter_mut()
        //                             .find(|param| param.id == id)
        //                             .expect("id to exist in parameters");
        //                         param.name = val;
        //                     });
        //             }
        //             {..}
        //             class="flex col-span-2 justify-self-center item-center"
        //         />
        //         <InputGroup
        //             id="value"
        //             placeholder="Value"
        //             value=value
        //             label_class="self-center"
        //             input_class="ml-2"
        //             on_input=move |val| {
        //                 interaction_query
        //                     .write()
        //                     .as_mut()
        //                     .map(|(_, _, interaction)| {
        //                         let mut param = interaction
        //                             .parameters
        //                             .iter_mut()
        //                             .find(|param| param.id == id)
        //                             .expect("id to exist in parameters");
        //                         param.value = val;
        //                     });
        //             }
        //             {..}
        //             class="flex col-span-2 justify-self-center item-center"
        //         />
        //         <InputGroup
        //             id="value"
        //             placeholder="Unit"
        //             value=unit
        //             label_class="self-center"
        //             input_class="ml-2"
        //             on_input=move |val| {
        //                 interaction_query
        //                     .write()
        //                     .as_mut()
        //                     .map(|(_, _, interaction)| {
        //                         let mut param = interaction
        //                             .parameters
        //                             .iter_mut()
        //                             .find(|param| param.id == id)
        //                             .expect("id to exist in parameters");
        //                         param.unit = val;
        //                     });
        //             }
        //             {..}
        //             class="flex col-span-2 justify-self-center item-center"
        //         />
        //         <button
        //             type="button"
        //             on:click=move |_| {
        //                 interaction_query
        //                     .write()
        //                     .as_mut()
        //                     .map(|(_, _, interaction)| {
        //                         interaction.parameters.retain(|param| param.id != id);
        //                     });
        //             }
        //             class="justify-self-center self-center text-sm font-semibold text-white rounded-full shadow-sm hover:bg-cyan-800 w-fit bg-cyan-950 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-cyan-900"
        //         >
        //             <svg
        //                 class="w-5 rotate-45"
        //                 viewBox="0 0 20 20"
        //                 fill="currentColor"
        //                 aria-hidden="true"
        //                 data-slot="icon"
        //             >
        //                 <path d="M10.75 4.75a.75.75 0 0 0-1.5 0v4.5h-4.5a.75.75 0 0 0 0 1.5h4.5v4.5a.75.75 0 0 0 1.5 0v-4.5h4.5a.75.75 0 0 0 0-1.5h-4.5v-4.5Z" />
        //             </svg>
        //         </button>
        //     </For>
        // </div>

        // Attributes section - metadata for documentation and future simulation
        <div class="mt-6">
            <div class="mb-4">
                <h3 class="flex items-center text-lg font-medium text-gray-900">
                    <span>Attributes</span>
                    <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" title="Custom attributes for documenting flow characteristics. Stored as metadata for analysis and future simulation.">
                        "?"
                    </span>
                </h3>
                <p class="text-xs text-gray-500 mt-1">Document flow properties (e.g., latency, throughput, protocol)</p>
            </div>

            <div class="space-y-4">
                <SimpleParameterInput
                    interaction_query=interaction_query
                />
            </div>
        </div>
    }
}

#[component]
pub fn ExternalEntityDetails(
    external_entity_query: RwSignalSynced<Option<ExternalEntityQuery>>,
    is_same_as_id_query: RwSignalSynced<Option<IsSameAsIdQuery>>,
    detach_event_sender: LeptosEventSender<DetachMarkerLabelEvent>,
) -> impl IntoView {
    let name = Signal::derive(move || {
        external_entity_query
            .read()
            .as_ref()
            .map(|(name, _, _)| name.to_string())
            .unwrap_or_default()
    });

    let description = Signal::derive(move || {
        external_entity_query
            .read()
            .as_ref()
            .map(|(_, description, _)| description.text.clone())
            .unwrap_or_default()
    });

    let _equivalence = Signal::derive(move || {
        external_entity_query
            .read()
            .as_ref()
            .map(|(_, _, external_entity)| external_entity.equivalence.clone())
            .unwrap_or_default()
    });

    let _model = Signal::derive(move || {
        external_entity_query
            .read()
            .as_ref()
            .map(|(_, _, external_entity)| external_entity.model.clone())
            .unwrap_or_default()
    });

    let is_same_as_id = Signal::derive(move || {
        is_same_as_id_query
            .read()
            .as_ref()
            .map(|(is_same_as_id,)| *is_same_as_id)
    });

    view! {
        <InputGroup
            id="external-entity-name"
            label="Name"
            placeholder="External Entity Name"
            value=name
            on_input=move |value: String| {
                external_entity_query.write().as_mut().map(|(name, _, _)| name.set(value));
            }
        />

        <TextArea
            id="external-entity-description"
            label="Description"
            placeholder="Add a description"
            text=description
            on_input=move |value: String| {
                external_entity_query
                    .write()
                    .as_mut()
                    .map(|(_, element_description, _)| element_description.text = value);
            }
        />

        // REMOVED for Phase 3C UX improvements - Equivalence field was confusing and unimplemented
        // The actual equivalence feature uses IsSameAsId component via multi-select + Detach button (below)
        // ExternalEntity.equivalence data field kept for backward compatibility but not shown in UI
        // <InputGroup
        //     id="equivalence"
        //     label="Equivalence"
        //     value=equivalence
        //     on_input=move |value: String| {
        //         external_entity_query
        //             .write()
        //             .as_mut()
        //             .map(|(_, _, external_entity)| external_entity.equivalence = value);
        //     }
        // />

        // HIDDEN for v0.2.0 "Structural Analysis Mode" - Model field is for dynamic behavior modeling
        // This represents minimal behavioral models of source/sink entities (e.g., "seasonal demand", "steady supplier")
        // Preserved for v0.3.0 dynamic modeling features
        // <InputGroup
        //     id="model"
        //     label="Model"
        //     value=model
        //     on_input=move |value: String| {
        //         external_entity_query
        //             .write()
        //             .as_mut()
        //             .map(|(_, _, external_entity)| external_entity.model = value);
        //     }
        // />

        <div class="mt-4">
            <Show when=move || is_same_as_id.get().is_some()>
                <Button
                    text=Signal::derive(move || {
                        format!("Detach ({})", *is_same_as_id.get().unwrap_or_default())
                    })
                    on_click=move || {
                        detach_event_sender.send(DetachMarkerLabelEvent).ok();
                    }
                />
            </Show>
        </div>
    }
}

#[component]
pub fn SystemDetails(
    system_query: RwSignalSynced<Option<SystemQuery>>,
    spatial_mode: RwSignalSynced<SpatialDetailPanelMode>,
) -> impl IntoView {
    // Use Memo::new instead of Signal::derive to prevent disposal issues
    let system_name = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(name, _, _, _)| name.to_string())
            .unwrap_or_default()
    });

    let system_description = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, description, _, _)| description.text.clone())
            .unwrap_or_default()
    });

    let system_complexity_level = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| ComplexityLevel::from(&system.complexity))
    });

    let _system_equivalence = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.equivalence.clone())
            .unwrap_or_default()
    });

    let boundary_name = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.name.clone())
            .unwrap_or_default()
    });

    let boundary_porosity = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.porosity as f64)
            .unwrap_or_default()
    });

    let environment_name = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, _, system_env)| system_env.name.clone())
            .unwrap_or_default()
    });

    let environment_description = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, _, system_env)| system_env.description.clone())
            .unwrap_or_default()
    });

    let environment_milieu = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, _, system_env)| system_env.milieu.clone())
            .unwrap_or_default()
    });

    let _system_time_unit = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.time_unit.clone())
            .unwrap_or_default()
    });

    // H (History/Memory) from Mobus 8-tuple - enabled for structural completeness
    let _system_history = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.history.clone())
            .unwrap_or_default()
    });

    let _system_transformation = Memo::new(move |_| {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.transformation.clone())
            .unwrap_or_default()
    });

    view! {
        // System Mode Content
        <div class:hidden=move || !matches!(spatial_mode.get(), SpatialDetailPanelMode::System)>
            <h3 class="text-lg font-semibold text-gray-900 mb-4">System</h3>

            <InputGroup
                id="system-name"
                label="Name"
                placeholder="External Entity Name"
                value=system_name
                on_input=move |value: String| {
                    system_query.write().as_mut().map(|(name, _, _, _)| name.set(value));
                }
            />

            <TextArea
                id="system-description"
                label="Description"
                placeholder="Add a description"
                text=system_description
                on_input=move |value: String| {
                    system_query
                        .write()
                        .as_mut()
                        .map(|(_, description, _, _)| description.text = value);
                }
            />

            // HIDDEN: Time Unit - emergent property, should be derived from flow time units
            // See session: 2025-12-24/bert-ui-cleanup.md
            // <div class="mb-4">
            //     <label class="flex items-center mb-2">
            //         <span class="block text-sm font-medium leading-6 text-gray-900">Time Unit</span>
            //         <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm"
            //               title="Time scale for system dynamics (e.g., seconds for reactions, years for ecosystems)">
            //             ?
            //         </span>
            //     </label>
            //     <InputGroup
            //         id="system-time-unit"
            //         placeholder="e.g., Second, Minute, Year"
            //         value=system_time_unit
            //         on_input=move |value: String| {
            //             system_query.write().as_mut().map(|(_, _, system, _)| system.time_unit = value);
            //         }
            //     />
            // </div>

            <RadioGroup
                id="system-complexity"
                label="Complexity"
                options=vec![ComplexityLevel::Simple, ComplexityLevel::Adaptable, ComplexityLevel::Evolveable]
                selected=system_complexity_level
                tooltip="System behavior type: Simple (predictable), Adaptable (responds to environment), Evolveable (can fundamentally change structure)"
                on_change=move |level: ComplexityLevel| {
                    system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, system, _)| system.complexity = Complexity::from(level));
                }
            />

            // HIDDEN: Equivalence - requires formal treatment (isomorphism/homomorphism), confuses users as text field
            // See session: 2025-12-24/bert-ui-cleanup.md
            // <div class="mb-4">
            //     <label class="flex items-center mb-2">
            //         <span class="block text-sm font-medium leading-6 text-gray-900">Equivalence</span>
            //         <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm"
            //               title="Component type - what kind of thing this is based on its essential function (e.g., in a cell: 'Ribosome', 'Mitochondria'; in a company: 'Sales Team', 'Finance Department')">
            //             ?
            //         </span>
            //     </label>
            //     <InputGroup
            //         id="system-equivalence"
            //         placeholder="Equivalence"
            //         value=system_equivalence
            //         on_input=move |value: String| {
            //             system_query.write().as_mut().map(|(_, _, system, _)| system.equivalence = value);
            //         }
            //     />
            // </div>

            // HIDDEN: History (H) - Mobus 8-tuple H should emerge from stocks/state variables, not manual entry
            // See session: 2025-12-24/bert-ui-cleanup.md
            // <Divider />
            // <div class="space-y-2">
            //     <label class="flex items-center mb-2">
            //         <span class="block text-sm font-medium leading-6 text-gray-900">History (H)</span>
            //         <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm"
            //               title="Memory/state variables placeholder (under research - will point to stack of system state images per Mobus formalization)">
            //             ?
            //         </span>
            //     </label>
            //     <p class="text-xs text-gray-500 mb-2">
            //         "Placeholder for system memory - future implementation will track stack of state images over time per history_implementation.txt"
            //     </p>
            //     <TextArea
            //         id="system-history"
            //         label=""
            //         placeholder="Placeholder: Will contain pointer to state history stack (under research)"
            //         text=system_history
            //         on_input=move |value: String| {
            //             system_query.write().as_mut().map(|(_, _, system, _)| system.history = value);
            //         }
            //     />
            // </div>

            // <div class="mb-4">
            //     <label class="flex items-center mb-2">
            //         <span class="block text-sm font-medium leading-6 text-gray-900">Transformation</span>
            //         <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm"
            //               title="Rules for system state transitions (requires behavioral modeling)">
            //             ?
            //         </span>
            //     </label>
            //     <InputGroup
            //         id="system-transformation"
            //         placeholder="Transformation"
            //         value=_system_transformation
            //         on_input=move |value: String| {
            //             system_query.write().as_mut().map(|(_, _, system, _)| system.transformation = value);
            //         }
            //     />
            // </div>
        </div>

        // Boundary Mode Content
        <div class:hidden=move || !matches!(spatial_mode.get(), SpatialDetailPanelMode::Boundary)>
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Boundary</h3>

            <InputGroup
                id="boundary-name"
                label="Name"
                value=boundary_name
                on_input=move |value| {
                    system_query.write().as_mut().map(|(_, _, system, _)| system.boundary.name = value);
                }
            />

            <TextArea
                id="boundary-description"
                label="Description"
                placeholder="Describe what defines this system's boundary—physical enclosure, organizational limits, legal jurisdiction, or conceptual scope."
                tooltip="The boundary separates the system from its environment. It determines what is inside (components, subsystems) vs outside (sources, sinks). Boundaries can be physical, organizational, or conceptual."
                text=Memo::new(move |_| system_query
                    .read()
                    .as_ref()
                    .map(|(_, _, system, _)| system.boundary.description.clone())
                    .unwrap_or_default())
                on_input=move |value: String| {
                    system_query.write().as_mut().map(|(_, _, system, _)| system.boundary.description = value);
                }
            />

            <Slider
                id="boundary-porosity"
                label="Porosity"
                tooltip="How permeable the boundary is to substances, energy, and messages (0.0 = impermeable, 1.0 = completely open)"
                value=boundary_porosity
                step=0.01
                on_input=move |value: f64| {
                    system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, system, _)| system.boundary.porosity = value as f32);
                }
            />

            <Slider
                id="boundary-perceptive-fuzziness"
                label="Perceptive Fuzziness"
                tooltip="How difficult it is to determine exactly where the boundary lies (0.0 = sharp/clear boundary, 1.0 = very fuzzy/gradual boundary)"
                value=Memo::new(move |_| system_query
                    .read()
                    .as_ref()
                    .map(|(_, _, system, _)| system.boundary.perceptive_fuzziness as f64)
                    .unwrap_or_default())
                step=0.01
                on_input=move |value: f64| {
                    system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, system, _)| system.boundary.perceptive_fuzziness = value as f32);
                }
            />
        </div>

        // Environment Mode Content
        <div class:hidden=move || !matches!(spatial_mode.get(), SpatialDetailPanelMode::Environment)>
            <h3 class="text-lg font-semibold text-gray-900 mb-4">Environment</h3>

            <InputGroup
                id="environment-name"
                label="Name"
                value=environment_name
                on_input=move |value: String| {
                    system_query.write().as_mut().map(|(_, _, _, system_env)| system_env.name = value);
                }
            />

            <TextArea
                id="environment-description"
                label="Description"
                placeholder="Describe the environment surrounding this system—external entities (sources/sinks) and ambient conditions (milieu) that influence but are not controlled by the system."
                tooltip="The environment E = (O, M) consists of: O = external objects (sources providing inputs, sinks receiving outputs) and M = milieu (ambient properties like temperature, pressure, market conditions that 'bathe' the system)."
                text=environment_description
                on_input=move |value: String| {
                    system_query.write().as_mut().map(|(_, _, _, system_env)| system_env.description = value);
                }
            />

            <Divider />

            <div class="space-y-3">
                <h4 class="text-sm font-semibold text-gray-700">
                    Milieu Properties (M)
                    <span class="text-xs font-normal text-gray-500 ml-2">"(Part of Environment: E = (O, M))"</span>
                </h4>
                <p class="text-xs text-gray-500">
                    "Ambient properties that surround or 'bathe' the system but don't flow through discrete interfaces. Examples: Temperature, Humidity, Salinity, pH, Pressure."
                </p>

                <div class:hidden=move || !environment_milieu.get().is_empty()>
                    <div class="text-sm text-gray-500 italic py-2">
                        No milieu properties defined. Click + to add.
                    </div>
                </div>

                <div class:hidden=move || environment_milieu.get().is_empty()>
                    <div class="space-y-3">
                        <For
                            each=move || environment_milieu.get().into_iter().enumerate()
                            key=|(idx, _)| *idx
                            children=move |(idx, (name, value))| {
                                let property_name = name.clone();
                                let property_value = value.clone();
                                view! {
                                    <div class="bg-gray-50 p-3 rounded space-y-2">
                                        <div class="flex items-center gap-2">
                                            <input
                                                type="text"
                                                class="flex-1 px-2 py-1 text-sm border border-gray-300 rounded"
                                                placeholder="Property name (e.g., Temperature)"
                                                value=property_name.clone()
                                                on:input=move |ev| {
                                                    let new_name = event_target_value(&ev);
                                                    system_query.write().as_mut().map(|(_, _, _, system_env)| {
                                                        if let Some(prop) = system_env.milieu.get_mut(idx) {
                                                            prop.0 = new_name;
                                                        }
                                                    });
                                                }
                                            />
                                            <button
                                                class="text-red-600 hover:text-red-800 text-xs px-2"
                                                on:click=move |_| {
                                                    system_query.write().as_mut().map(|(_, _, _, system_env)| {
                                                        system_env.milieu.remove(idx);
                                                    });
                                                }
                                            >
                                                Remove
                                            </button>
                                        </div>
                                        <input
                                            type="text"
                                            class="w-full px-2 py-1 text-sm border border-gray-300 rounded"
                                            placeholder="Value with unit (e.g., 25°C, 60%, 7.4 pH)"
                                            value=property_value.clone()
                                            on:input=move |ev| {
                                                let new_value = event_target_value(&ev);
                                                system_query.write().as_mut().map(|(_, _, _, system_env)| {
                                                    if let Some(prop) = system_env.milieu.get_mut(idx) {
                                                        prop.1 = new_value;
                                                    }
                                                });
                                            }
                                        />
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>

                <button
                    class="w-full px-3 py-2 text-sm font-medium text-blue-700 bg-blue-50 rounded-md hover:bg-blue-100"
                    on:click=move |_| {
                        system_query.write().as_mut().map(|(_, _, _, system_env)| {
                            system_env.milieu.push(("Property".to_string(), "Value".to_string()));
                        });
                    }
                >
                    + Add Milieu Property
                </button>
            </div>

        </div>
    }
}

#[component]
pub fn SubSystemDetails(sub_system_query: RwSignalSynced<Option<SubSystemQuery>>) -> impl IntoView {
    // Use Memo::new pattern for consistency with SystemDetails (prevents potential lifecycle issues)
    let name = Memo::new(move |_| {
        sub_system_query
            .read()
            .as_ref()
            .map(|(name, _, _, _)| name.to_string())
            .unwrap_or_default()
    });

    let description = Memo::new(move |_| {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, description, _, _)| description.text.clone())
            .unwrap_or_default()
    });

    let _complexity_types = vec![
        Complexity::Complex {
            adaptable: false,
            evolveable: false,
        },
        Complexity::Multiset(0),
        Complexity::Atomic,
    ];

    let _complexity = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.complexity)
    });

    let subsystem_complexity_level = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| ComplexityLevel::from(&system.complexity))
    });

    let subsystem_archetype = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.archetype)
    });

    let _membership = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.membership as f64)
            .unwrap_or_default()
    });

    let _equivalence = Memo::new(move |_| {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.equivalence.clone())
            .unwrap_or_default()
    });

    let _time_unit = Memo::new(move |_| {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.time_unit.clone())
            .unwrap_or_default()
    });

    // Dynamic fields - will be hidden for v0.2.0 "Structural Analysis" focus
    let _history = Memo::new(move |_| {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.history.clone())
            .unwrap_or_default()
    });

    let _transformation = Memo::new(move |_| {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.transformation.clone())
            .unwrap_or_default()
    });

    let boundary_name = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.name.clone())
            .unwrap_or_default()
    });

    let boundary_description = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.description.clone())
            .unwrap_or_default()
    });

    let boundary_porosity = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.porosity as f64)
            .unwrap_or_default()
    });

    let perceptive_fuzziness = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.boundary.perceptive_fuzziness as f64)
            .unwrap_or_default()
    });

    let parent_system_name = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, _, parent_state)| parent_state.name.clone())
            .unwrap_or_default()
    });

    let parent_system_description = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, _, parent_state)| parent_state.description.clone())
            .unwrap_or_default()
    });

    view! {
        <InputGroup
            id="system-name"
            label="Name"
            placeholder="External Entity Name"
            value=name
            on_input=move |value: String| {
                sub_system_query.write().as_mut().map(|(name, _, _, _)| name.set(value));
            }
        />

        <TextArea
            id="system-description"
            label="Description"
            placeholder="Describe this subsystem's role and function within its parent system."
            tooltip="A component with sufficient complexity to warrant further deconstruction into its own subsystems. What does it do? What inputs does it transform into outputs?"
            text=description
            on_input=move |value: String| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, description, _, _)| description.text = value);
            }
        />

        // HIDDEN for v0.2.0 - Time Unit should be derived from flow time units, not specified upfront
        // <div class="mb-4">
        //     <label class="flex items-center mb-2">
        //         <span class="block text-sm font-medium leading-6 text-gray-900">Time Unit</span>
        //         <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm"
        //               title="Time scale for system dynamics (e.g., seconds for reactions, years for ecosystems)">
        //             ?
        //         </span>
        //     </label>
        //     <InputGroup
        //         id="system-time-unit"
        //         placeholder="e.g., Second, Minute, Year"
        //         value=time_unit
        //         on_input=move |value: String| {
        //             sub_system_query.write().as_mut().map(|(_, _, system, _)| system.time_unit = value);
        //         }
        //     />
        // </div>

        <RadioGroup
            id="subsystem-complexity-level"
            label="Complexity"
            options=vec![ComplexityLevel::Simple, ComplexityLevel::Adaptable, ComplexityLevel::Evolveable]
            selected=subsystem_complexity_level
            on_change=move |level: ComplexityLevel| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, system, _)| system.complexity = Complexity::from(level));
            }
            tooltip="System behavior type: Simple (predictable), Adaptable (responds to environment), Evolveable (can fundamentally change structure)"
        />

        <RadioGroup
            id="subsystem-archetype"
            label="Archetype"
            options=vec![HcgsArchetype::Unspecified, HcgsArchetype::Governance, HcgsArchetype::Economy, HcgsArchetype::Agent]
            selected=subsystem_archetype
            on_change=move |archetype: HcgsArchetype| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, system, _)| system.archetype = archetype);
            }
            tooltip="HCGS archetype classification (Mobus 2022): Governance (coordination/control), Economy (production/exchange), Agent (autonomous actors)"
        />

        // HIDDEN for v0.2.0 - Multiset/Atomic complexity types deferred
        // <Show when=move || {
        //     sub_system_query
        //         .read()
        //         .as_ref()
        //         .map(|(_, _, system, _)| system.complexity.is_multiset())
        //         .unwrap_or_default()
        // }>
        //     <Slider
        //         id="system-membership"
        //         label="Member Autonomy"
        //         tooltip="How much individual freedom system members have to act independently (0.0 = tightly controlled/coordinated, 1.0 = completely autonomous)"
        //         step=0.01
        //         value=membership
        //         on_input=move |value: f64| {
        //             sub_system_query
        //                 .write()
        //                 .as_mut()
        //                 .map(|(_, _, system, _)| system.membership = value as f32);
        //         }
        //     />
        // </Show>

        // HIDDEN for v0.2.0 - Equivalence requires formal isomorphism/homomorphism treatment
        // <div class="mb-4">
        //     <label class="flex items-center mb-2">
        //         <span class="block text-sm font-medium leading-6 text-gray-900">Equivalence</span>
        //         <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm"
        //               title="Component type - what kind of thing this is based on its essential function (e.g., in a cell: 'Ribosome', 'Mitochondria'; in a company: 'Sales Team', 'Finance Department')">
        //             ?
        //         </span>
        //     </label>
        //     <InputGroup
        //         id="system-equivalence"
        //         placeholder="Equivalence"
        //         value=equivalence
        //         on_input=move |value: String| {
        //             sub_system_query
        //                 .write()
        //                 .as_mut()
        //                 .map(|(_, _, system, _)| system.equivalence = value);
        //         }
        //     />
        // </div>


        // HIDDEN for v0.2.0 "Structural Analysis Mode" - History is dynamic modeling feature
        // <InputGroup
        //     id="system-history"
        //     label="History"
        //     value=history
        //     on_input=move |value: String| {
        //         sub_system_query.write().as_mut().map(|(_, _, system, _)| system.history = value);
        //     }
        // />

        // HIDDEN for v0.2.0 "Structural Analysis Mode" - Transformation is dynamic modeling feature
        // <InputGroup
        //     id="transformation"
        //     label="Transformation"
        //     value=transformation
        //     on_input=move |value: String| {
        //         sub_system_query
        //             .write()
        //             .as_mut()
        //             .map(|(_, _, system, _)| system.transformation = value);
        //     }
        // />

        <Divider name="Boundary" />

        <InputGroup
            id="boundary-name"
            label="Name"
            value=boundary_name
            on_input=move |value| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, system, _)| system.boundary.name = value);
            }
        />

        <TextArea
            id="boundary-description"
            label="Description"
            placeholder="Describe what defines this subsystem's boundary within its parent system."
            tooltip="The boundary separates this subsystem from sibling subsystems and the parent system's internal space. It determines what is inside vs outside this component."
            text=boundary_description
            on_input=move |value: String| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, system, _)| system.boundary.description = value);
            }
        />

        <Slider
            id="boundary-porosity"
            label="Porosity"
            tooltip="How permeable the boundary is to substances, energy, and messages (0.0 = impermeable, 1.0 = completely open)"
            value=boundary_porosity
            step=0.01
            on_input=move |value: f64| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, system, _)| system.boundary.porosity = value as f32);
            }
        />

        <Slider
            id="boundary-perceptive-fuzziness"
            label="Perceptive Fuzziness"
            tooltip="How difficult it is to determine exactly where the boundary lies (0.0 = sharp/clear boundary, 1.0 = very fuzzy/gradual boundary)"
            value=perceptive_fuzziness
            step=0.01
            on_input=move |value: f64| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, system, _)| system.boundary.perceptive_fuzziness = value as f32);
            }
        />

        <Divider name="Parent System" />

        <InputGroup
            id="parent-system-name"
            label="Name"
            value=parent_system_name
            disabled=true
            on_input=move |value| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, _, parent_state)| parent_state.name = value);
            }
        />

        <TextArea
            id="parent-system-description"
            label="Description"
            text=parent_system_description
            disabled=true
            on_input=move |value| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, _, parent_state)| parent_state.description = value);
            }
        />
    }
}
