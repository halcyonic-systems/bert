use crate::bevy_app::data_model::Complexity;
use crate::leptos_app::components::{
    Button, Checkbox, Divider, InputGroup, SelectGroup, DetailsPanelMode, Slider, TextArea,
};
use crate::{
    DetachMarkerLabelEvent, DeselectAllEvent, ExternalEntityQuery, InteractionQuery, InteractionType,
    InteractionUsability, InterfaceQuery, IsSameAsIdQuery, SubSystemQuery,
    SubstanceType, SystemElement, SystemQuery,
};
use crate::bevy_app::components::SpatialDetailPanelMode;
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
                <span class="block text-sm font-medium leading-6 text-gray-900">Protocol</span>
                <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" 
                      title="Algorithm for letting flow across boundary in ordered fashion">
                    ?
                </span>
            </label>
            <TextArea
                id="protocol"
                label=""
                placeholder="Create a protocol"
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
pub fn InteractionDetails(
    interaction_query: RwSignalSynced<Option<InteractionQuery>>,
) -> impl IntoView {
    let name = Memo::new(move |_| interaction_query
        .read()
        .as_ref()
        .map(|(name, _, _)| name.to_string())
        .unwrap_or_default());

    let description = Memo::new(move |_| interaction_query
        .read()
        .as_ref()
        .map(|(_, el_desc, _)| el_desc.text.clone())
        .unwrap_or_default());

    let usability = Memo::new(move |_| interaction_query
        .read()
        .as_ref()
        .map(|(_, _, interaction)| interaction.usability));

    let interaction_type = Memo::new(move |_| interaction_query
        .read()
        .as_ref()
        .map(|(_, _, interaction)| interaction.interaction_type));

    let substance_type = Memo::new(move |_| interaction_query
        .read()
        .as_ref()
        .map(|(_, _, interaction)| interaction.substance_type));

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

        <div class="mb-4">
            <label class="flex items-center mb-2">
                <span class="block text-sm font-medium leading-6 text-gray-900">Substance Type</span>
                <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" 
                      title="Energy: power/work, Material: physical matter, Message: information">
                    ?
                </span>
            </label>
            <SelectGroup
                id="substance-type"
                label=""
                options=substance_types
                selected_option=substance_type
                on_change=move |value| {
                    interaction_query
                        .write()
                        .as_mut()
                        .map(|(_, _, interaction)| value.map(|ty| interaction.substance_type = ty));
                }
            />
        </div>

        <SelectGroup
            id="interaction-usability"
            label="Interaction Usability"
            tooltip="Type of interaction based on its utility: Resource (useful input), Disruption (harmful input), Product (useful output), or Waste (harmful output)"
            options=usability_types
            selected_option=usability
            on_change=move |value| {
                interaction_query
                    .write()
                    .as_mut()
                    .map(|(_, _, interaction)| value.map(|u| interaction.usability = u));
            }
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
    let system_name = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(name, _, _, _)| name.to_string())
        .unwrap_or_default());
    
    let system_description = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, description, _, _)| description.text.clone())
        .unwrap_or_default());
    
    let system_adaptable = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.complexity.is_adaptable())
        .unwrap_or_default());
    
    let system_evolveable = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.complexity.is_evolveable())
        .unwrap_or_default());
    
    let system_equivalence = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.equivalence.clone())
        .unwrap_or_default());
    
    let boundary_name = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.boundary.name.clone())
        .unwrap_or_default());
    
    let boundary_porosity = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.boundary.porosity as f64)
        .unwrap_or_default());
    
    let environment_name = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, _, system_env)| system_env.name.clone())
        .unwrap_or_default());
        
    let environment_description = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, _, system_env)| system_env.description.clone())
        .unwrap_or_default());
        
    let system_time_unit = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.time_unit.clone())
        .unwrap_or_default());
        
    // Dynamic fields - will be hidden for v0.2.0 "Structural Analysis" focus
    // Preserved for consistent pattern with SubSystemDetails and easier v0.3.0 restoration
    let _system_history = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.history.clone())
        .unwrap_or_default());

    let _system_transformation = Memo::new(move |_| system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.transformation.clone())
        .unwrap_or_default());

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
            
            <div class="mb-4">
                <label class="flex items-center mb-2">
                    <span class="block text-sm font-medium leading-6 text-gray-900">Time Unit</span>
                    <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" 
                          title="Time scale for system dynamics (e.g., seconds for reactions, years for ecosystems)">
                        ?
                    </span>
                </label>
                <InputGroup
                    id="system-time-unit"
                    placeholder="e.g., Second, Minute, Year"
                    value=system_time_unit
                    on_input=move |value: String| {
                        system_query.write().as_mut().map(|(_, _, system, _)| system.time_unit = value);
                    }
                />
            </div>
            
            <div class="mb-2">
                <label for="complexity" class="flex items-center font-medium text-gray-900 text-sm/6">
                    <span>Complexity</span>
                    <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" 
                          title="System behavior type: Simple (predictable), Adaptive (responds to environment), Evolveable (can fundamentally change structure)">
                        ?
                    </span>
                </label>
            </div>
            <div class="flex justify-evenly">
                <Checkbox
                    id="system-adaptable"
                    label="Adaptable"
                    checked=system_adaptable
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
                    checked=system_evolveable
                    on_toggle=move |value: bool| {
                        system_query
                            .write()
                            .as_mut()
                            .map(|(_, _, system, _)| system.complexity.set_evolveable(value));
                    }
                />
            </div>

            <div class="mb-4">
                <label class="flex items-center mb-2">
                    <span class="block text-sm font-medium leading-6 text-gray-900">Equivalence</span>
                    <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" 
                          title="Grouping for similar systems (e.g., 'Living Cell', 'Social Organization', 'Economic Market')">
                        ?
                    </span>
                </label>
                <InputGroup
                    id="system-equivalence"
                    placeholder="Equivalence"
                    value=system_equivalence
                    on_input=move |value: String| {
                        system_query.write().as_mut().map(|(_, _, system, _)| system.equivalence = value);
                    }
                />
            </div>
            
            // HIDDEN for v0.2.0 "Structural Analysis Mode" - Dynamic modeling features
            // Preserved for consistency with SubSystemDetails and easier v0.3.0 restoration
            // <div class="mb-4">
            //     <label class="flex items-center mb-2">
            //         <span class="block text-sm font-medium leading-6 text-gray-900">History</span>
            //         <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" 
            //               title="System state changes over time (requires temporal data collection)">
            //             ?
            //         </span>
            //     </label>
            //     <InputGroup
            //         id="system-history"
            //         placeholder="History"
            //         value=_system_history
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
                placeholder="Add a description"
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
                placeholder="Add a description"
                text=environment_description
                on_input=move |value: String| {
                    system_query.write().as_mut().map(|(_, _, _, system_env)| system_env.description = value);
                }
            />
            
        </div>
    }
}

#[component]
pub fn SubSystemDetails(sub_system_query: RwSignalSynced<Option<SubSystemQuery>>) -> impl IntoView {
    // Use Memo::new pattern for consistency with SystemDetails (prevents potential lifecycle issues)
    let name = Memo::new(move |_| sub_system_query
        .read()
        .as_ref()
        .map(|(name, _, _, _)| name.to_string())
        .unwrap_or_default());

    let description = Memo::new(move |_| sub_system_query
        .read()
        .as_ref()
        .map(|(_, description, _, _)| description.text.clone())
        .unwrap_or_default());

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

    let equivalence = Memo::new(move |_| sub_system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.equivalence.clone())
        .unwrap_or_default());

    let time_unit = Memo::new(move |_| sub_system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.time_unit.clone())
        .unwrap_or_default());

    // Dynamic fields - will be hidden for v0.2.0 "Structural Analysis" focus
    let _history = Memo::new(move |_| sub_system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.history.clone())
        .unwrap_or_default());

    let _transformation = Memo::new(move |_| sub_system_query
        .read()
        .as_ref()
        .map(|(_, _, system, _)| system.transformation.clone())
        .unwrap_or_default());

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

        <div class="mb-4">
            <label class="flex items-center mb-2">
                <span class="block text-sm font-medium leading-6 text-gray-900">Time Unit</span>
                <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" 
                      title="Time scale for system dynamics (e.g., seconds for reactions, years for ecosystems)">
                    ?
                </span>
            </label>
            <InputGroup
                id="system-time-unit"
                placeholder="e.g., Second, Minute, Year"
                value=time_unit
                on_input=move |value: String| {
                    sub_system_query.write().as_mut().map(|(_, _, system, _)| system.time_unit = value);
                }
            />
        </div>

        <div class="mb-4">
            <label class="flex items-center mb-2">
                <span class="block text-sm font-medium leading-6 text-gray-900">Complexity</span>
                <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" 
                      title="System behavior type: Simple (predictable), Adaptive (responds to environment), Evolveable (can fundamentally change structure)">
                    ?
                </span>
            </label>
            <SelectGroup
                id="system-complexity"
                label=""
                options=complexity_types
                selected_option=complexity
                on_change=move |value| {
                sub_system_query
                    .write()
                    .as_mut()
                    .map(|(_, _, system, _)| system.complexity = value.unwrap_or_default());
            }
        />
        </div>

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
                tooltip="How much individual freedom system members have to act independently (0.0 = tightly controlled/coordinated, 1.0 = completely autonomous)"
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

        <div class="mb-4">
            <label class="flex items-center mb-2">
                <span class="block text-sm font-medium leading-6 text-gray-900">Equivalence</span>
                <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" 
                      title="Grouping for similar systems (e.g., 'Living Cell', 'Social Organization', 'Economic Market')">
                    ?
                </span>
            </label>
            <InputGroup
                id="system-equivalence"
                placeholder="Equivalence"
                value=equivalence
                on_input=move |value: String| {
                    sub_system_query
                        .write()
                        .as_mut()
                        .map(|(_, _, system, _)| system.equivalence = value);
                }
            />
        </div>


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
