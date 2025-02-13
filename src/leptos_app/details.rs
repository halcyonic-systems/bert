use crate::bevy_app::data_model::Complexity;
use crate::leptos_app::components::{Checkbox, Divider, InputGroup, SelectGroup, Slider, TextArea};
use crate::{
    ExternalEntityQuery, InteractionQuery, InteractionType, InteractionUsability, InterfaceQuery,
    Parameter, SubSystemQuery, SubstanceType, SystemElement, SystemQuery,
};
use enum_iterator::all;
use leptos::prelude::*;
use leptos::tachys::html::property::IntoProperty;
use leptos::tachys::renderer::dom::Element;
use leptos::tachys::renderer::Rndr;
use leptos_bevy_canvas::prelude::RwSignalSynced;
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
) -> impl IntoView {
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
                                        <h2
                                            class="text-base font-semibold text-gray-900"
                                            id="slide-over-title"
                                        >
                                            Element Details
                                            <a
                                                download="test.png"
                                                href="assets/create-button/inflow.png"
                                            >
                                                Bruh
                                            </a>
                                        </h2>
                                        <div class="flex items-center ml-3 h-7">
                                            <button
                                                type="button"
                                                class="relative text-gray-400 bg-white rounded-md hover:text-gray-500 focus:ring-2 focus:ring-rose-500 focus:ring-offset-2 focus:outline-hidden"
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
                                <div class="relative flex-1 px-4 mt-6 sm:px-6">
                                    <Show when=move || interaction_details.get().is_some()>
                                        <InteractionDetails interaction_query=interaction_details />
                                    </Show>
                                    <Show when=move || interface_details.get().is_some()>
                                        <InterfaceDetails interface_query=interface_details />
                                    </Show>
                                    <Show when=move || external_entity_details.get().is_some()>
                                        <ExternalEntityDetails external_entity_query=external_entity_details />
                                    </Show>
                                    <Show when=move || { system_details.get().is_some() }>
                                        <SystemDetails system_query=system_details />
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
            class="py-1.5 px-3 w-full text-sm font-semibold text-white rounded-full shadow-sm hover:bg-rose-800 bg-rose-950 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-rose-900"
        >
            Add
        </button>

        <div class="grid grid-cols-9 gap-x-4">
            <For
                each=move || parameters.get()
                key=|param| param.id.clone()
                let(Parameter { id, name, value })
            >
                <InputGroup
                    id="name"
                    label="Name"
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
                    class="flex col-span-4 justify-self-center item-center"
                />
                <InputGroup
                    id="value"
                    label="Value"
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
                    class="flex col-span-4 justify-self-center item-center"
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
                    class="justify-self-center self-center p-1 text-sm font-semibold text-white rounded-full shadow-sm hover:bg-rose-800 size-fit bg-rose-950 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-rose-900"
                >
                    <svg
                        class="rotate-45 size-5"
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
            <label for="complexity" class="block font-medium text-gray-900 text-sm/6">
                Complexity
            </label>
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

        <InputGroup
            id="system-equivalence"
            label="Equivalence"
            value=equivalence
            on_input=move |value: String| {
                system_query.write().as_mut().map(|(_, _, system, _)| system.equivalence = value);
            }
        />

        <InputGroup
            id="system-time-unit"
            label="Time Unit"
            value=time_unit
            on_input=move |value: String| {
                system_query.write().as_mut().map(|(_, _, system, _)| system.time_unit = value);
            }
        />

        <InputGroup
            id="system-history"
            label="History"
            value=history
            on_input=move |value: String| {
                system_query.write().as_mut().map(|(_, _, system, _)| system.history = value);
            }
        />

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

        <Divider name="Boundary" />

        <InputGroup
            id="boundary-name"
            label="Name"
            value=boundary_name
            on_input=move |value| {
                system_query.write().as_mut().map(|(_, _, system, _)| system.boundary.name = value);
            }
        />

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

        <InputGroup
            id="environment-name"
            label="Name"
            value=environment_name
            on_input=move |value: String| {
                system_query.write().as_mut().map(|(_, _, _, system_env)| system_env.name = value);
            }
        />

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

        <InputGroup
            id="system-time-unit"
            label="Time Unit"
            value=time_unit
            on_input=move |value: String| {
                sub_system_query.write().as_mut().map(|(_, _, system, _)| system.time_unit = value);
            }
        />

        <InputGroup
            id="system-history"
            label="History"
            value=history
            on_input=move |value: String| {
                sub_system_query.write().as_mut().map(|(_, _, system, _)| system.history = value);
            }
        />

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

        <InputGroup
            id="parent-system-description"
            label="Description"
            value=parent_system_description
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
