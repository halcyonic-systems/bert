use crate::components::{
    ElementDescription, Flow, InflowUsability, Interface, OutflowUsability, SystemElement,
    Usability,
};
use crate::plugins::mouse_interaction::PickSelection;
use crate::SubstanceType;
use bevy::prelude::*;
use bevy_egui::egui::{vec2, ComboBox, Margin, Ui, Visuals};
use bevy_egui::{egui, EguiContexts};

fn interface_egui(ui: &mut Ui, interface: &mut Interface) {
    let _ = ui;
    let _ = interface;
}

fn outflow_egui(ui: &mut Ui, flow: &mut Flow) {
    ui.horizontal(|ui| {
        ui.label("Usability");

        OutflowUsability::mutate(&mut flow.is_useful, |outflow_usability| {
            ComboBox::from_label("")
                .selected_text(format!("{:?}", outflow_usability))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(outflow_usability, OutflowUsability::Product, "Product");
                    ui.selectable_value(outflow_usability, OutflowUsability::Waste, "Waste");
                });
        });
    });

    flow_egui(ui, flow);
}

fn flow_egui(ui: &mut Ui, flow: &mut Flow) {
    ui.horizontal(|ui| {
        ui.label("Substance Type");
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
}

fn inflow_egui(ui: &mut Ui, flow: &mut Flow) {
    ui.horizontal(|ui| {
        ui.label("Usability");

        InflowUsability::mutate(&mut flow.is_useful, |inflow_usability| {
            ComboBox::from_label("")
                .selected_text(format!("{:?}", inflow_usability))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(inflow_usability, InflowUsability::Resource, "Resource");
                    ui.selectable_value(
                        inflow_usability,
                        InflowUsability::Disruption,
                        "Disruption",
                    );
                });
        });
    });

    flow_egui(ui, flow);
}

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
) {
    for (entity, selectable, system_element, mut name, mut description) in &mut selectable_query {
        if selectable.is_selected {
            egui_contexts.ctx_mut().set_visuals(Visuals::light());
            egui_contexts.ctx_mut().style_mut(|style| {
                style.spacing.window_margin = Margin {
                    left: 10.0,
                    right: 10.0,
                    top: 10.0,
                    bottom: 10.0,
                };
                style.spacing.item_spacing = vec2(10.0, 10.0);
            });
            egui::Window::new(&system_element.to_string()).show(egui_contexts.ctx_mut(), |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Name: ");

                            name.mutate(|name| {
                                ui.text_edit_singleline(name);
                            });
                        });
                        ui.horizontal(|ui| {
                            ui.label("Description: ");

                            ui.text_edit_multiline(&mut description.text);
                        });

                        match system_element {
                            SystemElement::Interface => {
                                interface_egui(
                                    ui,
                                    &mut interface_query.get_mut(entity).expect("Interface not found"),
                                );
                            }
                            SystemElement::System => {
                                // TODO: implement
                            }
                            SystemElement::Inflow => inflow_egui(
                                ui,
                                &mut flow_query.get_mut(entity).expect("Inflow not found"),
                            ),
                            SystemElement::Outflow => outflow_egui(
                                ui,
                                &mut flow_query.get_mut(entity).expect("Outflow not found"),
                            ),
                            SystemElement::ExternalEntity => {
                                // TODO: implement
                            }
                        };
                    });
            });
            return;
        }
    }
}
