use crate::components::{Inflow, InflowUsability, Interface, Outflow, OutflowUsability, SystemElement};
use bevy::prelude::*;
use bevy_egui::egui::{ComboBox, Ui};
use bevy_egui::{egui, EguiContexts};
use bevy_mod_picking::prelude::*;

fn interface_egui(ui: &mut Ui, interface: &mut Interface) {
    ui.horizontal(|ui| {
        ui.label("Name: ");
        ui.text_edit_singleline(&mut interface.name);
    });
}

fn outflow_egui(ui: &mut Ui, outflow: &mut Outflow) {
    ui.horizontal(|ui| {
        ui.label("Name: ");
        ui.text_edit_singleline(&mut outflow.name);
    });

    ComboBox::from_label("Usability")
        .selected_text(format!("{:?}", outflow.usability))
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            ui.selectable_value(&mut outflow.usability, OutflowUsability::Product, "Product");
            ui.selectable_value(&mut outflow.usability, OutflowUsability::Waste, "Waste");
        });
}

fn inflow_egui(ui: &mut Ui, inflow: &mut Inflow) {
    ui.horizontal(|ui| {
        ui.label("Name: ");
        ui.text_edit_singleline(&mut inflow.name);
    });

    ComboBox::from_label("Usability")
        .selected_text(format!("{:?}", inflow.usability))
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            ui.selectable_value(&mut inflow.usability, InflowUsability::Resource, "Resource");
            ui.selectable_value(&mut inflow.usability, InflowUsability::Disruption, "Disruption");
        });
}

pub fn egui_selected_context(
    mut egui_contexts: EguiContexts,
    selectables: Query<(Entity, &PickSelection, &SystemElement)>,
    mut interfaces: Query<&mut Interface>,
    mut outflows: Query<&mut Outflow>,
    mut inflows: Query<&mut Inflow>,
) {
    for (entity, selectable, system_element) in &selectables {
        if selectable.is_selected {
            egui::Window::new(&system_element.to_string()).show(egui_contexts.ctx_mut(), |ui| {
                egui::ScrollArea::both().auto_shrink([false; 2]).show(
                    ui,
                    |ui| match system_element {
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
                    },
                );
            });
            return;
        }
    }
}
