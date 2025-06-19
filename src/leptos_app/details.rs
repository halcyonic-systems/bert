use crate::bevy_app::data_model::Complexity;
use crate::leptos_app::components::{
    Button, Checkbox, Divider, FieldTooltip, InputGroup, ResearchField, ResearchFieldProvider, ResearchFieldToggle, SelectGroup, Slider, TextArea,
};
use crate::{
    DetachMarkerLabelEvent, ExternalEntityQuery, InteractionQuery, InteractionType,
    InteractionUsability, InterfaceQuery, IsSameAsIdQuery, Parameter, SubSystemQuery,
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
    detach_event_sender: LeptosEventSender<DetachMarkerLabelEvent>,
) -> impl IntoView {
    // Panel width state: 0 = narrow (md), 1 = medium (lg), 2 = wide (xl)
    let (panel_width, set_panel_width) = signal(1);
    
    let width_class = move || match panel_width.get() {
        0 => "max-w-md",    // ~448px
        1 => "max-w-lg",    // ~512px  
        2 => "max-w-xl",    // ~576px
        _ => "max-w-lg",
    };
    
    let width_label = move || match panel_width.get() {
        0 => "Narrow",
        1 => "Medium", 
        2 => "Wide",
        _ => "Medium",
    };
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
                            class=move || format!("w-screen {} transition-all duration-300 ease-in-out transform pointer-events-auto", width_class())
                            class:translate-x-full=move || selected.get().is_none()
                            class:translate-x-0=move || selected.get().is_some()
                        >
                            <div class="flex overflow-y-scroll flex-col py-6 h-full bg-white shadow-xl">
                                <div class="px-4 sm:px-6">
                                    <div class="flex justify-between items-start w-full">
                                        <div class="flex items-center space-x-2">
                                            <h2
                                                class="text-base font-semibold text-gray-900"
                                                id="slide-over-title"
                                            >
                                                Element Details
                                            </h2>
                                            <div class="flex items-center space-x-1 px-2 py-1 bg-blue-50 rounded-md">
                                                <svg class="w-3 h-3 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                </svg>
                                                <span class="text-xs text-blue-600 font-medium">Editable</span>
                                            </div>
                                        </div>
                                        
                                        <div class="flex items-center space-x-2">
                                            // Panel width controls
                                            <div class="flex items-center space-x-1">
                                                <span class="text-xs text-gray-500 mr-1">{width_label}</span>
                                                <button
                                                    type="button"
                                                    class="p-1 text-gray-400 hover:text-gray-600 rounded"
                                                    on:click=move |_| {
                                                        let current = panel_width.get();
                                                        if current > 0 {
                                                            set_panel_width.set(current - 1);
                                                        }
                                                    }
                                                    disabled=move || panel_width.get() == 0
                                                >
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"></path>
                                                    </svg>
                                                </button>
                                                <button
                                                    type="button"
                                                    class="p-1 text-gray-400 hover:text-gray-600 rounded"
                                                    on:click=move |_| {
                                                        let current = panel_width.get();
                                                        if current < 2 {
                                                            set_panel_width.set(current + 1);
                                                        }
                                                    }
                                                    disabled=move || panel_width.get() == 2
                                                >
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                                                    </svg>
                                                </button>
                                            </div>
                                            
                                            <div class="flex items-center h-7">
                                            <button
                                                type="button"
                                                class="relative text-gray-400 bg-white rounded-md hover:text-gray-500 focus:ring-2 focus:ring-cyan-500 focus:ring-offset-2 focus:outline-hidden"
                                            >
                                                <span class="absolute -inset-2.5"></span>
                                                <span class="sr-only">Close panel</span>
                                                <svg
                                                    class="size-6"
                                                    fill="none"
                                                    viewBox="0 0 24 24"
                                                    stroke-width="1.5"
                                                    stroke="currentColor"
                                                    aria-hidden="true"
                                                    data-slot="icon"
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
                                </div>
                                <div class="relative flex-1 px-4 mt-6 sm:px-6">
                                    <ResearchFieldProvider>
                                        <ResearchFieldToggle />
                                        
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
                                            <SystemDetails system_query=system_details />
                                        </Show>
                                        <Show when=move || { sub_system_details.get().is_some() }>
                                            <SubSystemDetails sub_system_query=sub_system_details />
                                        </Show>
                                    </ResearchFieldProvider>
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

        <TextArea
            id="protocol"
            label="Protocol φ"
            placeholder="Create a protocol"
            text=protocol
            on_input=move |value: String| {
                interface_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interface)| interface.protocol = value);
            }
        />
    }
}

#[component]
pub fn InteractionDetails(
    interaction_query: RwSignalSynced<Option<InteractionQuery>>,
) -> impl IntoView {
    let name = Signal::derive(move || {
        interaction_query
            .read()
            .as_ref()
            .map(|(name, _, _)| name.to_string())
            .unwrap_or_default()
    });

    let description = Signal::derive(move || {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, el_desc, _)| el_desc.text.clone())
            .unwrap_or_default()
    });

    let usability = Signal::derive(move || {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.usability)
    });

    let interaction_type = Signal::derive(move || {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.interaction_type)
    });

    let substance_type = Signal::derive(move || {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.substance_type)
    });

    let substance_sub_type = Signal::derive(move || {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.substance_sub_type.clone())
            .unwrap_or_default()
    });

    let substance_unit = Signal::derive(move || {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.unit.clone())
            .unwrap_or_default()
    });

    let substance_amount = Signal::derive(move || {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| DecimalWrapper(interaction.amount))
            .unwrap_or_default()
    });

    let parameters = Signal::derive(move || {
        interaction_query
            .read()
            .as_ref()
            .map(|(_, _, interaction)| interaction.parameters.clone())
            .unwrap_or(Vec::new())
    });

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

        <SelectGroup
            id="interaction-usability"
            label="Interaction Usability"
            options=usability_types
            selected_option=usability
            on_change=move |value| {
                interaction_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interaction)| value.map(|u| interaction.usability = u));
            }
        />

        <SelectGroup
            id="interaction-type"
            label="Interaction Type"
            options=interaction_types
            selected_option=interaction_type
            on_change=move |value| {
                interaction_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interaction)| value.map(|ty| interaction.interaction_type = ty));
            }
        />

        <SelectGroup
            id="substance-type"
            label="Substance Type"
            options=substance_types
            selected_option=substance_type
            on_change=move |value| {
                interaction_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interaction)| value.map(|ty| interaction.substance_type = ty));
            }
        />

        <ResearchField>
            <InputGroup
                id="substance-sub-type"
                label="Substance Sub Type"
                placeholder=""
                value=substance_sub_type
                on_input=move |value: String| {
                    interaction_query
                        .write()
                        .as_mut()
                        .map(|(_, _, interaction)| interaction.substance_sub_type = value);
                }
            />
        </ResearchField>

        <InputGroup
            id="substance-unit"
            label="Substance Unit"
            placeholder=""
            value=substance_unit
            on_input=move |value: String| {
                interaction_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interaction)| interaction.unit = value);
            }
        />

        <InputGroup
            id="substance-amount"
            label="Substance Amount"
            placeholder=""
            type_="text"
            value=substance_amount
            on_input=move |value: DecimalWrapper| {
                interaction_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interaction)| interaction.amount = value.0);
            }
        />

        <Divider name="Parameters" />

        <button
            type="button"
            on:click=move |_| {
                interaction_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interaction)| interaction.parameters.push(Parameter::default()));
            }
            class="py-1.5 px-3 w-full text-sm font-semibold text-white rounded-full shadow-sm hover:bg-cyan-800 bg-cyan-950 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-cyan-900"
        >
            Add
        </button>

        <div class="grid grid-cols-7 gap-x-4 mt-3">
            <Show when=move || { parameters.get().len() > 0 }>
                <div class="flex col-span-2 justify-self-center text-center item-center">Name</div>
                <div class="flex col-span-2 justify-self-center text-center item-center">Value</div>
                <div class="flex col-span-2 justify-self-center text-center item-center">Unit</div>
                <div class="flex justify-self-center item-center"></div>
            </Show>
            <For
                each=move || parameters.get()
                key=|param| param.id.clone()
                let(Parameter { id, name, value, unit })
            >
                <InputGroup
                    id="name"
                    placeholder="Name"
                    value=name
                    label_class="self-center"
                    input_class="ml-2"
                    on_input=move |val| {
                        interaction_query
                            .write()
                            .as_mut()
                            .map(|(_, _, interaction)| {
                                let mut param = interaction
                                    .parameters
                                    .iter_mut()
                                    .find(|param| param.id == id)
                                    .expect("id to exist in parameters");
                                param.name = val;
                            });
                    }
                    {..}
                    class="flex col-span-2 justify-self-center item-center"
                />
                <InputGroup
                    id="value"
                    placeholder="Value"
                    value=value
                    label_class="self-center"
                    input_class="ml-2"
                    on_input=move |val| {
                        interaction_query
                            .write()
                            .as_mut()
                            .map(|(_, _, interaction)| {
                                let mut param = interaction
                                    .parameters
                                    .iter_mut()
                                    .find(|param| param.id == id)
                                    .expect("id to exist in parameters");
                                param.value = val;
                            });
                    }
                    {..}
                    class="flex col-span-2 justify-self-center item-center"
                />
                <InputGroup
                    id="value"
                    placeholder="Unit"
                    value=unit
                    label_class="self-center"
                    input_class="ml-2"
                    on_input=move |val| {
                        interaction_query
                            .write()
                            .as_mut()
                            .map(|(_, _, interaction)| {
                                let mut param = interaction
                                    .parameters
                                    .iter_mut()
                                    .find(|param| param.id == id)
                                    .expect("id to exist in parameters");
                                param.unit = val;
                            });
                    }
                    {..}
                    class="flex col-span-2 justify-self-center item-center"
                />
                <button
                    type="button"
                    on:click=move |_| {
                        interaction_query
                            .write()
                            .as_mut()
                            .map(|(_, _, interaction)| {
                                interaction.parameters.retain(|param| param.id != id);
                            });
                    }
                    class="justify-self-center self-center text-sm font-semibold text-white rounded-full shadow-sm hover:bg-cyan-800 w-fit bg-cyan-950 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-cyan-900"
                >
                    <svg
                        class="w-5 rotate-45"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        aria-hidden="true"
                        data-slot="icon"
                    >
                        <path d="M10.75 4.75a.75.75 0 0 0-1.5 0v4.5h-4.5a.75.75 0 0 0 0 1.5h4.5v4.5a.75.75 0 0 0 1.5 0v-4.5h4.5a.75.75 0 0 0 0-1.5h-4.5v-4.5Z" />
                    </svg>
                </button>
            </For>
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

    let equivalence = Signal::derive(move || {
        external_entity_query
            .read()
            .as_ref()
            .map(|(_, _, external_entity)| external_entity.equivalence.clone())
            .unwrap_or_default()
    });

    let model = Signal::derive(move || {
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

        <ResearchField>
            <InputGroup
                id="equivalence"
                label="Equivalence"
                value=equivalence
                on_input=move |value: String| {
                    external_entity_query
                        .write()
                        .as_mut()
                        .map(|(_, _, external_entity)| external_entity.equivalence = value);
                }
            />
        </ResearchField>

        <InputGroup
            id="model"
            label="Model"
            value=model
            on_input=move |value: String| {
                external_entity_query
                    .write()
                    .as_mut()
                    .map(|(_, _, external_entity)| external_entity.model = value);
            }
        />

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
pub fn SystemDetails(system_query: RwSignalSynced<Option<SystemQuery>>) -> impl IntoView {
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

    let history = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.history.clone())
            .unwrap_or_default()
    });

    let transformation = Signal::derive(move || {
        system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.transformation.clone())
            .unwrap_or_default()
    });

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
        <InputGroup
            id="system-name"
            label="Name"
            placeholder="External Entity Name"
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
        <div class="mb-2">
            <FieldTooltip field_id="complexity".to_string()>
                <label for="complexity" class="block font-medium text-gray-900 text-sm/6">
                    Complexity
                </label>
            </FieldTooltip>
        </div>
        <div class="flex justify-evenly">
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

        <ResearchField>
            <InputGroup
                id="system-equivalence"
                label="Equivalence"
                value=equivalence
                on_input=move |value: String| {
                    system_query.write().as_mut().map(|(_, _, system, _)| system.equivalence = value);
                }
            />
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="system-time-unit"
                label="Time Unit"
                value=time_unit
                on_input=move |value: String| {
                    system_query.write().as_mut().map(|(_, _, system, _)| system.time_unit = value);
                }
            />
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="system-history"
                label="History"
                value=history
                on_input=move |value: String| {
                    system_query.write().as_mut().map(|(_, _, system, _)| system.history = value);
                }
            />
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="transformation"
                label="Transformation"
                value=transformation
                on_input=move |value: String| {
                    system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, system, _)| system.transformation = value);
                }
            />
        </ResearchField>

        <Divider name="Boundary" />

        <ResearchField>
            <InputGroup
                id="boundary-name"
                label="Name"
                value=boundary_name
                on_input=move |value| {
                    system_query.write().as_mut().map(|(_, _, system, _)| system.boundary.name = value);
                }
            />
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="boundary-description"
                label="Description"
                value=boundary_description
                on_input=move |value| {
                    system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, system, _)| system.boundary.description = value);
                }
            />
        </ResearchField>

        <Slider
            id="boundary-porosity"
            label="Porosity"
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
            value=perceptive_fuzziness
            step=0.01
            on_input=move |value: f64| {
                system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, system, _)| system.boundary.perceptive_fuzziness = value as f32);
            }
        />

        <Divider name="Environment" />

        <ResearchField>
            <InputGroup
                id="environment-name"
                label="Name"
                value=environment_name
                on_input=move |value: String| {
                    system_query.write().as_mut().map(|(_, _, _, system_env)| system_env.name = value);
                }
            />
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="environment-description"
                label="Description"
                value=environment_description
                on_input=move |value: String| {
                    system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, _, system_env)| system_env.description = value);
                }
            />
        </ResearchField>
    }
}

#[component]
pub fn SubSystemDetails(sub_system_query: RwSignalSynced<Option<SubSystemQuery>>) -> impl IntoView {
    let name = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(name, _, _, _)| name.to_string())
            .unwrap_or_default()
    });

    let description = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, description, _, _)| description.text.clone())
            .unwrap_or_default()
    });

    let complexity_types = vec![
        Complexity::Complex {
            adaptable: false,
            evolveable: false,
        },
        Complexity::Multiset(0),
        Complexity::Atomic,
    ];

    let complexity = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.complexity)
    });

    let adaptable = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.complexity.is_adaptable())
            .unwrap_or_default()
    });

    let evolveable = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.complexity.is_evolveable())
            .unwrap_or_default()
    });

    let membership = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.membership as f64)
            .unwrap_or_default()
    });

    let equivalence = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.equivalence.clone())
            .unwrap_or_default()
    });

    let time_unit = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.time_unit.clone())
            .unwrap_or_default()
    });

    let history = Signal::derive(move || {
        sub_system_query
            .read()
            .as_ref()
            .map(|(_, _, system, _)| system.history.clone())
            .unwrap_or_default()
    });

    let transformation = Signal::derive(move || {
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
            placeholder="Add a description"
            text=description
            on_input=move |value: String| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, description, _, _)| description.text = value);
            }
        />

        <SelectGroup
            id="system-complexity"
            label="Complexity"
            options=complexity_types
            selected_option=complexity
            on_change=move |value| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, system, _)| system.complexity = value.unwrap_or_default());
            }
        />

        <Show when=move || {
            sub_system_query
                .read()
                .as_ref()
                .map(|(_, _, system, _)| system.complexity.is_complex())
                .unwrap_or_default()
        }>
            <div class="flex justify-evenly">
                <Checkbox
                    id="system-adaptable"
                    label="Adaptable"
                    checked=adaptable
                    on_toggle=move |value: bool| {
                        sub_system_query
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
                        sub_system_query
                            .write()
                            .as_mut()
                            .map(|(_, _, system, _)| system.complexity.set_evolveable(value));
                    }
                />
            </div>
        </Show>

        <ResearchField>
            <Show when=move || {
                sub_system_query
                    .read()
                    .as_ref()
                    .map(|(_, _, system, _)| system.complexity.is_multiset())
                    .unwrap_or_default()
            }>
                <Slider
                    id="system-membership"
                    label="Member Autonomy"
                    step=0.01
                    value=membership
                    on_input=move |value: f64| {
                        sub_system_query
                            .write()
                            .as_mut()
                            .map(|(_, _, system, _)| system.membership = value as f32);
                    }
                />
            </Show>
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="system-equivalence"
                label="Equivalence"
                value=equivalence
                on_input=move |value: String| {
                    sub_system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, system, _)| system.equivalence = value);
                }
            />
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="system-time-unit"
                label="Time Unit"
                value=time_unit
                on_input=move |value: String| {
                    sub_system_query.write().as_mut().map(|(_, _, system, _)| system.time_unit = value);
                }
            />
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="system-history"
                label="History"
                value=history
                on_input=move |value: String| {
                    sub_system_query.write().as_mut().map(|(_, _, system, _)| system.history = value);
                }
            />
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="transformation"
                label="Transformation"
                value=transformation
                on_input=move |value: String| {
                    sub_system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, system, _)| system.transformation = value);
                }
            />
        </ResearchField>

        <Divider name="Boundary" />

        <ResearchField>
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
        </ResearchField>

        <ResearchField>
            <InputGroup
                id="boundary-description"
                label="Description"
                value=boundary_description
                on_input=move |value| {
                    sub_system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, system, _)| system.boundary.description = value);
                }
            />
        </ResearchField>

        <Slider
            id="boundary-porosity"
            label="Porosity"
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
