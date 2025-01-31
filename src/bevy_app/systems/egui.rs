//! Holds all Egui systems that define the app UI.
//! The user can edit data that is associated with different selectable System Elements.
//! The forms are built using Egui, an immediate mode rust UI library.
//! This file contains systems that query component data associated with different elements, defines the form UI, & processes input.
//! The macros defined in this file are used to improve the code's readability by reducing the verbosity of egui's api.
//! This feature heavily uses "system piping".
use crate::bevy_app::components::*;
use crate::bevy_app::data_model::Complexity;
use crate::bevy_app::plugins::mouse_interaction::PickSelection;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_egui::egui::{Checkbox, ComboBox, DragValue, Margin, Slider, Ui, Visuals};
use bevy_egui::{egui, EguiContexts};
use rust_decimal::Decimal;

macro_rules! h_wrap {
    ($ui:expr, $body:expr) => {
        $ui.horizontal($body);
    };
}

macro_rules! vc_wrap {
    ($ui:expr, $body:expr) => {
        $ui.vertical_centered($body);
    };
}

macro_rules! h_label {
    ($ui:expr, $data:expr) => {
        $ui.horizontal(|ui| {
            ui.label($data);
        });
    };
}

macro_rules! vc_label {
    ($ui:expr, $data:expr) => {
        $ui.vertical_centered(|ui| {
            ui.label($data);
        });
    };
}

macro_rules! vcj_label {
    ($ui:expr, $data:expr) => {
        $ui.vertical_centered_justified(|ui| {
            ui.label($data);
        });
    };
}

macro_rules! vcj_text_edit {
    ($ui:expr, $data:expr, $multi_line_flag:expr) => {
        $ui.vertical_centered_justified(|ui| {
            if $multi_line_flag {
                ui.text_edit_multiline($data);
            } else {
                ui.text_edit_singleline($data);
            }
        });
    };
}

fn interface_egui(ui: &mut Ui, interface: &mut Interface) {
    h_label!(ui, "Protocol φ");
    vcj_text_edit!(ui, &mut interface.protocol, true);
}

fn interaction_egui(ui: &mut Ui, flow: &mut Flow) {
    h_label!(ui, "Interaction Usability");
    ui.horizontal(|ui| {
        ComboBox::from_label("")
            .selected_text(format!("{:?}", flow.usability))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(
                    &mut flow.usability,
                    InteractionUsability::Product,
                    "Product",
                );
                ui.selectable_value(&mut flow.usability, InteractionUsability::Waste, "Waste");
                ui.selectable_value(
                    &mut flow.usability,
                    InteractionUsability::Resource,
                    "Resource",
                );
                ui.selectable_value(
                    &mut flow.usability,
                    InteractionUsability::Disruption,
                    "Disruption",
                );
            });
    });

    flow_egui(ui, flow);
}

fn flow_egui(ui: &mut Ui, flow: &mut Flow) {
    h_label!(ui, "Interaction Type");
    h_wrap!(ui, |ui| {
        ComboBox::from_label("       ")
            .selected_text(format!("{:?}", flow.interaction_type))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut flow.interaction_type, InteractionType::Flow, "Flow");
                ui.selectable_value(&mut flow.interaction_type, InteractionType::Force, "Force");
            });
    });

    h_label!(ui, "Substance Type");
    ui.horizontal(|ui| {
        ComboBox::from_label(" ")
            .selected_text(format!("{:?}", flow.substance_type))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut flow.substance_type, SubstanceType::Energy, "Energy");
                ui.selectable_value(
                    &mut flow.substance_type,
                    SubstanceType::Material,
                    "Material",
                );
                ui.selectable_value(&mut flow.substance_type, SubstanceType::Message, "Message");
            });
    });

    h_label!(ui, "Substance Sub Type");
    vcj_text_edit!(ui, &mut flow.substance_sub_type, false);

    h_label!(ui, "Substance Unit");
    vcj_text_edit!(ui, &mut flow.unit, false);

    // TODO : allow empty strings
    let mut amount_string = flow.amount.to_string();
    h_label!(ui, "Substance Amount");
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut amount_string);
        only_valid_positive_decimal(&mut amount_string, &mut flow.amount);
    });

    ui.separator();
    vcj_label!(ui, "Parameters");
    parameters_list_egui(ui, flow);
}

fn parameters_list_egui(ui: &mut Ui, flow: &mut Flow) {
    egui::Grid::new("Parameters List")
        .striped(true)
        .show(ui, |ui| {
            if flow.parameters.is_empty() {
                if ui.button("Add Parameter").clicked() {
                    flow.parameters.push(Parameter::default());
                }
                return;
            }
            if ui.button("Add").clicked() {
                flow.parameters.push(Parameter::default());
            }
            let min_size = egui::Vec2::new(100.0, 20.0);
            ui.label("Name");
            ui.label("Value");
            ui.end_row();
            for idx in 0..flow.parameters.len() {
                if ui.button("Delete").clicked() {
                    flow.parameters.remove(idx);
                    return;
                }
                ui.add(
                    egui::TextEdit::singleline(&mut flow.parameters[idx].name)
                        .hint_text("Name...")
                        .min_size(min_size),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut flow.parameters[idx].value)
                        .hint_text("Value..")
                        .min_size(min_size),
                );
                ui.end_row();
            }
        });
}

pub fn only_valid_positive_decimal(s: &mut String, decimal: &mut Decimal) {
    if let Ok(value) = Decimal::from_str_exact(&s) {
        *decimal = value;
    } else {
        *s = decimal.to_string();
    }
}

fn system_of_interest_egui(
    ui: &mut Ui,
    system: &mut crate::bevy_app::components::System,
    system_environment: &mut SystemEnvironment,
) {
    h_label!(ui, "Complexity");
    match &mut system.complexity {
        Complexity::Complex {
            ref mut adaptable,
            ref mut evolveable,
        } => {
            h_wrap!(ui, |ui| {
                h_label!(ui, "Adaptable");
                ui.add(Checkbox::without_text(adaptable));

                h_label!(ui, "Evolveable");
                ui.add(Checkbox::without_text(evolveable));
            });
        }
        _ => panic!("System of Intest can only be complex"),
    }

    h_label!(ui, "Equivalence");
    vcj_text_edit!(ui, &mut system.equivalence, false);

    h_label!(ui, "Time Unit");
    vcj_text_edit!(ui, &mut system.time_unit, false);

    h_label!(ui, "History");
    vcj_text_edit!(ui, &mut system.history, false);

    h_label!(ui, "Transformation");
    vcj_text_edit!(ui, &mut system.transformation, false);

    ui.separator();
    boundary_egui(ui, system);

    ui.separator();
    mut_environment_egui(ui, system_environment);
}

fn boundary_egui(ui: &mut Ui, system: &mut crate::bevy_app::components::System) {
    vc_label!(ui, "Boundary");

    h_label!(ui, "Name");
    vcj_text_edit!(ui, &mut system.boundary.name, false);

    h_label!(ui, "Description");
    vcj_text_edit!(ui, &mut system.boundary.description, true);

    h_wrap!(ui, |ui| {
        ui.label("Porosity");
        ui.add(Slider::new(&mut system.boundary.porosity, 0.0..=1.0).drag_value_speed(1.0));
    });
    h_wrap!(ui, |ui| {
        ui.label("Perceptive Fuzziness");
        ui.add(
            Slider::new(&mut system.boundary.perceptive_fuzziness, 0.0..=1.0).drag_value_speed(1.0),
        );
    });
}

fn mut_environment_egui(ui: &mut Ui, system_environment: &mut SystemEnvironment) {
    vc_label!(ui, "Environment");

    h_label!(ui, "Name");
    vcj_text_edit!(ui, &mut system_environment.name, false);

    h_label!(ui, "Description");
    vcj_text_edit!(ui, &mut system_environment.description, true);
}

fn subsystem_egui(
    ui: &mut Ui,
    system: &mut crate::bevy_app::components::System,
    parent_system_info: &(String, String),
) {
    complexity_egui(ui, system);

    h_label!(ui, "Member Autonomy");
    ui.add(Slider::new(&mut system.membership, 0.0..=1.0).drag_value_speed(1.0));

    h_label!(ui, "Equivalence");
    vcj_text_edit!(ui, &mut system.equivalence, false);

    h_label!(ui, "Time Unit");
    vcj_text_edit!(ui, &mut system.time_unit, false);

    h_label!(ui, "History");
    vcj_text_edit!(ui, &mut system.history, false);

    h_label!(ui, "Transformation");
    vcj_text_edit!(ui, &mut system.transformation, false);

    ui.separator();
    boundary_egui(ui, system);

    ui.separator();
    vcj_label!(ui, "Parent System");

    h_wrap!(ui, |ui| {
        ui.label("Name: ");
        ui.label(&parent_system_info.0);
    });
    h_label!(ui, "Description:");
    vc_wrap!(ui, |ui| {
        ui.label(&parent_system_info.1);
    });
}

fn complexity_egui(ui: &mut Ui, system: &mut crate::bevy_app::components::System) {
    h_label!(ui, "Complexity Type");
    ComboBox::from_label("   ")
        .selected_text(system.complexity.to_string())
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            let complexity = system.complexity.clone();
            if matches!(complexity, Complexity::Complex { .. }) {
                ui.selectable_value(&mut system.complexity, complexity.clone(), "Complex");
            } else {
                ui.selectable_value(
                    &mut system.complexity,
                    Complexity::Complex {
                        adaptable: false,
                        evolveable: false,
                    },
                    "Complex",
                );
            }

            if matches!(complexity, Complexity::Multiset(_)) {
                ui.selectable_value(&mut system.complexity, complexity.clone(), "Multiset");
            } else {
                ui.selectable_value(&mut system.complexity, Complexity::Multiset(1), "Multiset");
            }

            ui.selectable_value(&mut system.complexity, Complexity::Atomic, "Atomic");
        });
    match &mut system.complexity {
        Complexity::Complex {
            ref mut adaptable,
            ref mut evolveable,
        } => {
            h_wrap!(ui, |ui| {
                h_label!(ui, "Adaptable");
                ui.add(Checkbox::without_text(adaptable));

                h_label!(ui, "Evolveable");
                ui.add(Checkbox::without_text(evolveable));
            });
        }
        Complexity::Multiset(ref mut count) => {
            h_wrap!(ui, |ui| {
                h_label!(ui, "System Instances");
                ui.add(DragValue::new(count).speed(1.));
            });
        }
        Complexity::Atomic => {}
    }
}

fn external_entity_egui(ui: &mut Ui, external_entity: &mut ExternalEntity) {
    h_label!(ui, "Equivalence");
    vcj_text_edit!(ui, &mut external_entity.equivalence, false);
    h_label!(ui, "Model");
    vcj_text_edit!(ui, &mut external_entity.model, false);
}

/// Gets all the data associated with selectable system elements.
/// Based on the System Element, it pipes relevant component data to different functions that control the UI for that System Element Type.
pub fn egui_selected_context(
    mut egui_contexts: EguiContexts,
    mut selectable_query: Query<(
        Entity,
        &PickSelection,
        &SystemElement,
        &mut Name,
        &mut ElementDescription,
    )>,
    mut interface_query: Query<&mut Interface>,
    mut flow_query: Query<&mut Flow>,
    mut system_environment_query: Query<&mut SystemEnvironment>,
    mut system_query: Query<&mut crate::bevy_app::components::System>,
    mut external_entity_query: Query<&mut ExternalEntity>,
    subsystem_query: Query<&crate::bevy_app::components::Subsystem>,
) {
    let mut count = 0;
    for (_, selection, _, _, _) in &mut selectable_query {
        if selection.is_selected {
            count += 1;
        }
    }
    // Prevents showing the side panel if more than 1 elements are selected.
    if count > 1 {
        return;
    }

    // TODO: This is inefficient and needs to be refactored.
    let mut info_hm = HashMap::<Entity, (String, String)>::new();
    for (entity, _, _, name, description) in &selectable_query {
        info_hm.insert(entity, (name.to_string(), description.text.clone()));
    }

    // Finds the currently selected System Element, defines the side panel layout, &
    // pipes the component data to an element type specific function that further defines the UI.
    for (entity, selection, system_element, mut name, mut description) in &mut selectable_query {
        if !selection.is_selected {
            continue;
        }
        egui_contexts.ctx_mut().set_visuals(Visuals::light());
        egui_contexts.ctx_mut().style_mut(|style| {
            style.spacing.window_margin = Margin {
                left: 10.0,
                right: 10.0,
                top: 10.0,
                bottom: 10.0,
            };
            style.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
        });
        egui::SidePanel::right(system_element.to_string())
            .default_width(300.0)
            .show(egui_contexts.ctx_mut(), |ui| {
                vc_wrap!(ui, |ui| {
                    ui.heading("Element Details");
                });
                egui::ScrollArea::both()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        h_label!(ui, "Name");
                        ui.vertical_centered_justified(|ui| {
                            name.mutate(|name| {
                                ui.text_edit_singleline(name);
                            });
                        });
                        h_label!(ui, "Description");
                        vcj_text_edit!(ui, &mut description.text, true);

                        match system_element {
                            SystemElement::Interface => interface_egui(
                                ui,
                                &mut interface_query
                                    .get_mut(entity)
                                    .expect("Interface not found"),
                            ),
                            SystemElement::System => {
                                let mut system =
                                    system_query.get_mut(entity).expect("System not found");

                                if let Ok(mut sys_env) = system_environment_query.get_mut(entity) {
                                    system_of_interest_egui(ui, &mut system, &mut sys_env)
                                } else {
                                    let subsystem = subsystem_query
                                        .get(entity)
                                        .expect("Subsystem should exist");

                                    let parent_info =
                                        info_hm.get(&subsystem.parent_system).unwrap();

                                    subsystem_egui(ui, &mut system, parent_info);
                                }
                            }
                            SystemElement::Interaction => interaction_egui(
                                ui,
                                &mut flow_query.get_mut(entity).expect("Interaction not found"),
                            ),
                            SystemElement::ExternalEntity => external_entity_egui(
                                ui,
                                &mut external_entity_query
                                    .get_mut(entity)
                                    .expect("External Entity not found"),
                            ),
                        };
                    });
            });
    }
}

/// When the user is interacting with EGUI, prevent the user input from effecting the diagram.
pub fn absorb_egui_inputs(
    mut contexts: EguiContexts,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut mouse_wheel: ResMut<Events<MouseWheel>>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();
    if ctx.wants_pointer_input() || ctx.is_pointer_over_area() || ctx.wants_keyboard_input() {
        // let modifiers = [
        //     KeyCode::SuperLeft,
        //     KeyCode::SuperRight,
        //     KeyCode::ControlLeft,
        //     KeyCode::ControlRight,
        //     KeyCode::AltLeft,
        //     KeyCode::AltRight,
        //     KeyCode::ShiftLeft,
        //     KeyCode::ShiftRight,
        // ];
        //
        // let pressed = modifiers.map(|key| keyboard.pressed(key).then_some(key));

        mouse.reset_all();
        mouse_wheel.clear();
        keyboard.reset_all();

        // for key in pressed.into_iter().flatten() {
        //     keyboard.press(key);
        // }
    }
}
