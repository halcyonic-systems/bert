use crate::components::{
    ElementDescription, Inflow, InflowUsability, Interface, Outflow, OutflowUsability,
    SystemElement,
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

fn outflow_egui(ui: &mut Ui, outflow: &mut Outflow) {
    ui.horizontal(|ui| {
        ui.label("Usability");
        ComboBox::from_label("")
            .selected_text(format!("{:?}", outflow.usability))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut outflow.usability, OutflowUsability::Product, "Product");
                ui.selectable_value(&mut outflow.usability, OutflowUsability::Waste, "Waste");
            });
    });

    ui.horizontal(|ui| {
        ui.label("Substance Type");
        ComboBox::from_label(" ")
            .selected_text(format!("{:?}", outflow.substance_type))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut outflow.substance_type, SubstanceType::Energy, "Energy");
                ui.selectable_value(
                    &mut outflow.substance_type,
                    SubstanceType::Material,
                    "Material",
                );
                ui.selectable_value(
                    &mut outflow.substance_type,
                    SubstanceType::Message,
                    "Message",
                );
            });
    });
}

fn inflow_egui(ui: &mut Ui, inflow: &mut Inflow) {
    ui.horizontal(|ui| {
        ui.label("Usability");
        ComboBox::from_label("")
            .selected_text(format!("{:?}", inflow.usability))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut inflow.usability, InflowUsability::Resource, "Resource");
                ui.selectable_value(
                    &mut inflow.usability,
                    InflowUsability::Disruption,
                    "Disruption",
                );
            });
    });
    ui.horizontal(|ui| {
        ui.label("Substance Type");
        ComboBox::from_label(" ")
            .selected_text(format!("{:?}", inflow.substance_type))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut inflow.substance_type, SubstanceType::Energy, "Energy");
                ui.selectable_value(
                    &mut inflow.substance_type,
                    SubstanceType::Material,
                    "Material",
                );
                ui.selectable_value(
                    &mut inflow.substance_type,
                    SubstanceType::Message,
                    "Message",
                );
            });
    });
}

pub fn egui_selected_context(
    mut egui_contexts: EguiContexts,
    mut selectables: Query<(
        Entity,
        &PickSelection,
        &SystemElement,
        &mut Name,
        &mut ElementDescription,
    )>,
    mut interfaces: Query<&mut Interface>,
    mut outflows: Query<&mut Outflow>,
    mut inflows: Query<&mut Inflow>,
) {
    for (entity, selectable, system_element, mut name, mut description) in &mut selectables {
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
                                    &mut *interfaces.get_mut(entity).expect("Interface not found"),
                                );
                            }
                            SystemElement::System => {
                                // TODO: implement
                            }
                            SystemElement::Inflow => inflow_egui(
                                ui,
                                &mut *inflows.get_mut(entity).expect("Inflow not found"),
                            ),
                            SystemElement::Outflow => outflow_egui(
                                ui,
                                &mut *outflows.get_mut(entity).expect("Outflow not found"),
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
