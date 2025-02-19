mod models;
mod svg_node;

use self::models::*;
use self::svg_node::*;

use crate::events::TreeEvent;
use crate::data_model::WorldModel;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use leptos::{html::Div, logging, prelude::*};
use leptos_bevy_canvas::prelude::*;
use leptos_use::{use_element_size, UseElementSizeReturn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[component]
pub fn Tree(
    #[prop(into)] visible: Signal<bool>,
    event_receiver: LeptosEventReceiver<TreeEvent>,
) -> impl IntoView {
    let tree_ref = NodeRef::<Div>::new();
    let UseElementSizeReturn { width, height } = use_element_size(tree_ref);

    view! {
        <Show when=move || visible.get()>
            <h2>"Tree"</h2>
                <div node_ref=tree_ref style="padding: 1rem; height: 100vh;">
                {
                    let event_receiver = event_receiver.clone();
                    move || {
                        layout_tree(event_receiver.clone(), width.get(), height.get())
                    }
                }
            </div>
        </Show>
    }
}

fn layout_tree(
    event_receiver: LeptosEventReceiver<TreeEvent>,
    width: f64,
    height: f64,
) -> impl IntoView {
    event_receiver
        .read()
        .as_ref()
        .map(|TreeEvent { world_model }| {
            let ordered_world_systems = get_world_systems_ordered(&world_model);
            // Todo: create svg nodes and read width

            let max_tree_width = get_max_tree_width(&ordered_world_systems);
            let tree_mid = (width - max_tree_width) * 0.5;

            let root_node = create_node_tree(&ordered_world_systems, tree_mid);

            let svg_tree_view = draw_node_tree(root_node);

            view! {
                <svg width={width} height={height}>
                    {svg_tree_view}
                </svg>
            }
        })
}

fn get_world_systems_ordered(world_model: &WorldModel) -> Vec<InputSvgSystem> {
    let mut systems_vec = Vec::new();

    for system in world_model.systems.iter() {
        let id = serde_json::to_string(&system.info.id)
            .unwrap()
            .replace("\"", "");
        let mut parent = serde_json::to_string(&system.parent)
            .unwrap()
            .replace("\"", "");
        if parent == "E-1" {
            parent = "Root".to_string();
        }
        systems_vec.push(InputSvgSystem {
            label: id.clone(),
            level: system.info.level,
            parent_label: parent.clone(),
            children: vec![],
        })
    }
    systems_vec.sort_by_key(|sys| (sys.level, sys.label.clone()));

    systems_vec
}

fn create_node_tree(systems_vec: &Vec<InputSvgSystem>, midpoint: f64) -> SvgSystem {
    let mut node_map: HashMap<String, Rc<RefCell<SvgSystem>>> = HashMap::new();

    for system in systems_vec.iter() {
        let node = Rc::new(RefCell::new(SvgSystem {
            label: system.label.clone(),
            level: system.level,
            parent: None,
            children: vec![],
            x: 0.0,
            thread: None,
            ancestor: None,
            change: 0.0,
            shift: 0.0,
            offset_modifier: 0.0,
            sibling_order_number: 0,
        }));
        node_map.insert(system.label.clone(), node);
    }

    for system in systems_vec.iter() {
        if let Some(node) = node_map.get(&system.label) {
            if !system.parent_label.is_empty() {
                if let Some(parent) = node_map.get(&system.parent_label) {
                    node.borrow_mut().parent = Some(Rc::downgrade(parent));
                    node.borrow_mut().sibling_order_number = parent.borrow().children.len() + 1;
                    parent.borrow_mut().children.push(node.clone());
                }
            }
        }
    }

    let root = node_map
        .values()
        .find(|node| node.borrow().parent.is_none())
        .unwrap()
        .clone();

    drop(node_map);

    buchheim(&root, midpoint);

    Rc::into_inner(root).unwrap().into_inner()
}

fn draw_node_tree(node: SvgSystem) -> Vec<AnyView> {
    let mut views = Vec::new();

    views.push(
        view! {
            <SvgNode
                label={node.label.clone()}
                x={node.x}
                y={node.get_node_y()}
                width={node.get_node_width()}
                height={node.get_node_height()}
                font_size={node.get_node_font_size()}
            />
        }
        .into_any(),
    );

    if node.children.len() > 0 {
        let parent_x = node.x;
        let parent_node_width = node.get_node_width();
        let children_mid_x = parent_x + parent_node_width * 0.5;
        let y = node.children[0].borrow().get_node_y();

        views.push(view! {
            <line x1={children_mid_x} y1={y - 15.0} x2={children_mid_x} y2={y - 30.0} stroke-width="2" stroke="black" />
        }.into_any());

        if node.children.len() > 1 {
            let child_node_width = node.children[0].borrow().get_node_width();
            let children_width = get_node_row_width(node.children.len(), child_node_width);

            let children_start_x =
                node.children.first().unwrap().borrow().x + child_node_width * 0.5;
            let children_end_x = node.children.last().unwrap().borrow().x + child_node_width * 0.5;

            views.push(view! {
                <line x1={children_start_x} y1={y - 15.0} x2={children_end_x} y2={y - 15.0} stroke-width="2" stroke="black" />
                <line x1={children_mid_x} y1={y - 15.0} x2={children_mid_x} y2={y - 30.0} stroke-width="2" stroke="black" />
            }.into_any());
        }
    }

    for child in node.children {
        views.extend(draw_node_tree(Rc::into_inner(child).unwrap().into_inner()));
    }

    views
}
