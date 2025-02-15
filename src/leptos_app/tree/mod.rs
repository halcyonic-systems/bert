mod models;
use self::models::*;

use crate::events::TreeEvent;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use leptos::{html::Div, logging, prelude::*};
use leptos_bevy_canvas::prelude::*;
use leptos_use::{use_element_size, UseElementSizeReturn};
use std::collections::HashMap;

const NODE_GAP: f64 = 20.0;
const NODE_GROUP_GAP: f64 = 20.0;

pub fn recursive_system_child(
    parent: SvgSystem,
    mut sorted_system_info_vec: Vec<SvgSystem>,
) -> Vec<SvgSystem> {
    let mut children = Vec::new();

    for system in sorted_system_info_vec.clone() {
        if system.parent == parent.label {
            let mut child = system.clone();
            child.children = recursive_system_child(child.clone(), sorted_system_info_vec.clone());
            children.push(child);
        }
    }

    children
}

pub fn collect_levels_and_lengths(system: &SvgSystem) -> Vec<(i32, Vec<usize>)> {
    let mut levels_map: HashMap<i32, Vec<usize>> = HashMap::new();

    fn traverse(system: &SvgSystem, levels_map: &mut HashMap<i32, Vec<usize>>) {
        levels_map
            .entry(system.level)
            .or_insert_with(Vec::new)
            .push(system.children.len());
        for child in &system.children {
            traverse(child, levels_map);
        }
    }

    traverse(system, &mut levels_map);

    let mut levels_and_lengths: Vec<(i32, Vec<usize>)> = levels_map.into_iter().collect();
    levels_and_lengths.sort_by_key(|&(level, _)| level);
    levels_and_lengths
}

pub fn get_node_row_width(len: i32, level: i32) -> f64 {
    if len == 0 {
        return 0.0;
    }
    let dummy = SystemNode {
        level,
        ..Default::default()
    };
    let nodes_width = dummy.get_node_width() * len as f64;
    let gap = NODE_GAP * (len - 1) as f64;
    nodes_width + gap
}

pub fn get_svg_tree(
    system: SvgSystem,
    root_x: f64,
    structure: &Vec<(i32, Vec<usize>)>,
) -> Vec<AnyView> {
    let mut views = Vec::new();

    let root_node = SystemNode {
        label: system.label.clone(),
        level: system.level,
        x: root_x,
    };
    views.push(
        view! {
            <SvgNode node=root_node />
        }
        .into_any(),
    );

    let children_len = system.children.len();
    if children_len > 0 {
        views.append(&mut recursive_child_nodes(system, root_x, structure));
    }

    views
}

pub fn recursive_child_nodes(
    current_system: SvgSystem,
    root_x: f64,
    structure: &Vec<(i32, Vec<usize>)>,
) -> Vec<AnyView> {
    let mut views = Vec::new();
    let group_length = current_system.children.len();
    let group_width = get_node_row_width(group_length as i32, current_system.level + 1);
    let current_system_node_width = SystemNode {
        level: current_system.level,
        ..Default::default()
    }
    .get_node_width();

    let mut current_x = root_x - (group_width / 2.0);
    let mut first_child_x = 0.0;
    let mut last_child_x = 0.0;

    for (i, child) in current_system.children.into_iter().enumerate() {
        let mut child_node = SystemNode {
            label: child.label.clone(),
            level: child.level,
            x: current_x,
        };
        child_node.x += child_node.get_node_width() / 2.0;
        child_node.x += (current_system_node_width - child_node.get_node_width()) / 2.0;

        if i == 0 {
            first_child_x = child_node.x + child_node.get_node_width() / 2.0;
        }

        if i == group_length - 1 {
            last_child_x = child_node.x + child_node.get_node_width() / 2.0;
        }

        let children_len = child.children.len();
        if children_len > 0 {
            views.append(&mut recursive_child_nodes(
                child.clone(),
                child_node.x,
                structure,
            ));
        }

        current_x += child_node.get_node_width() + NODE_GAP;

        // let row_structure = &structure[child.level as usize].1;
        // let row_group_gap = if child.level > 1 {
        //     NODE_GROUP_GAP * (row_structure.len() - 1) as f64
        // } else {
        //     0.0
        // };
        //
        // let mut row_width = row_group_gap;
        // for len in row_structure {
        //     if *len == 0 {
        //         continue;
        //     }
        //     let group_width =
        //         dummy_node.get_node_width() * *len as f64 + NODE_GAP * (*len - 1) as f64;
        //     row_width += group_width
        // }
        //
        // let child_x = root_x - (row_width / 2.0)
        //     + (i as f64 * dummy_node.get_node_width())
        //     + (i as f64 * NODE_GAP);

        views.push(view! { <SvgNode node=child_node /> }.into_any());
    }

    if group_length > 0 {
        let middle_x = root_x + current_system_node_width / 2.0;
        let y = SystemNode {
            level: current_system.level + 1,
            ..Default::default()
        }
        .get_node_y();
        views.push(view! {
            <line x1={first_child_x} y1={y - 15.0} x2={last_child_x} y2={y - 15.0} stroke-width="2" stroke="black" />
            <line x1={middle_x} y1={y - 15.0} x2={middle_x} y2={y - 30.0} stroke-width="2" stroke="black" />
        }.into_any());
    }

    views
}

#[component]
pub fn SvgNode(node: SystemNode) -> impl IntoView {
    let y = node.get_node_y();
    let width = node.get_node_width();
    let height = node.get_node_height();
    let font_size = node.get_node_font_size();

    view! {
        <rect x={node.x} y={y} fill="none" stroke="steelblue" stroke-width="2" rx="5" ry="5" width={width} height={height}></rect>
        <text x={node.x + (width / 2.0)} y={y + height / 1.5} fill="#222" font-size={font_size} font-weight="bold" font-family="sans-serif" text-anchor="middle">
            {node.label}
        </text>
        <line x1={node.x + width / 2.0} y1={y} x2={node.x + width / 2.0} y2={y - 15.0} stroke-width="2" stroke="black" />
        <line x1={node.x + width / 2.0} y1={y + height} x2={node.x + width / 2.0} y2={y + height + 15.0} stroke-width="2" stroke="black" />
    }
}

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
                        let mut systems_vec = Vec::new();
                        event_receiver.read().as_ref().map(|TreeEvent { world_model }| {
                            for system in world_model.systems.iter() {
                                let id = serde_json::to_string(&system.info.id).unwrap().replace("\"", "");
                                let parent = serde_json::to_string(&system.parent).unwrap().replace("\"", "");
                                systems_vec.push(SvgSystem {
                                    label: id,
                                    level: system.info.level,
                                    parent,
                                    children: Vec::new()
                                })
                            }
                            systems_vec.sort_by_key(|sys| (sys.level, sys.label.clone()));

                            let mut svg_system = systems_vec.remove(0);
                            svg_system.children = recursive_system_child(svg_system.clone(), systems_vec);

                            let levels_and_lengths = collect_levels_and_lengths(&svg_system);
                            logging::log!("{:?}", levels_and_lengths);

                            let svg_tree_view = get_svg_tree(svg_system.clone(), width.get() / 2.0, &levels_and_lengths);

                            view! {
                                <svg width={width} height={height}>
                                    {svg_tree_view}
                                </svg>
                            }
                        })
                    }
                }
            </div>
        </Show>
    }
}
