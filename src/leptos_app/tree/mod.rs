mod models;
mod svg_node;

use self::models::*;
use self::svg_node::*;

use crate::data_model::{Id, WorldModel};
use crate::events::TreeEvent;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use leptos::{html::Div, logging, prelude::*};
use leptos_bevy_canvas::prelude::*;
use leptos_use::{use_element_size, UseElementSizeReturn};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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
            <div node_ref=tree_ref style="height: 100vh;">
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
            let ordered_systems = get_world_systems_ordered(&world_model);

            let mid_x = width * 0.5;
            let min_tree_width = get_min_tree_width(&ordered_systems);
            let x_for_mid_aligned_tree = mid_x - min_tree_width * 0.5;

            let root_node = create_node_tree(&ordered_systems, x_for_mid_aligned_tree);
            let root_x_mid = root_node.x + root_node.get_node_width() * 0.5;

            let svg_tree_view = draw_node_tree(root_node);
            let svg_tree_header_view = draw_node_tree_header(&world_model, root_x_mid);
            let svg_tree_aside_view =
                draw_node_tree_aside(&ordered_systems, x_for_mid_aligned_tree);

            view! {
                <svg width={width} height={height}>
                    {svg_tree_view}
                    {svg_tree_header_view}
                    {svg_tree_aside_view}
                </svg>
            }
        })
}

fn get_string_from_id(id: &Id) -> String {
    serde_json::to_string(&id).unwrap().replace("\"", "")
}

fn get_name_or_id(name: &String, id: &Id) -> String {
    if name.is_empty() || name == "System" || name == "Subsystem" {
        get_string_from_id(id)
    } else {
        name.clone()
    }
}

fn get_world_systems_ordered(world_model: &WorldModel) -> Vec<InputSystem> {
    let mut systems_vec = Vec::new();

    for system in &world_model.systems {
        let id = get_string_from_id(&system.info.id);

        let label = get_name_or_id(&system.info.name, &system.info.id);

        let mut parent_id = get_string_from_id(&system.parent);

        systems_vec.push(InputSystem {
            id,
            type_: system.complexity,
            label,
            level: system.info.level,
            parent_id,
            children: vec![],
        })
    }

    systems_vec.sort_by_key(|sys| (sys.level, sys.id.clone()));

    systems_vec
}

fn create_node_tree(systems_vec: &Vec<InputSystem>, midpoint: f64) -> SvgSystem {
    let mut node_map: HashMap<String, Rc<RefCell<SvgSystem>>> = HashMap::new();

    for system in systems_vec.iter() {
        let node = Rc::new(RefCell::new(SvgSystem {
            id: system.id.clone(),
            type_: system.type_,
            label: system.label.clone(),
            level: system.level,
            ..Default::default()
        }));
        node_map.insert(system.id.clone(), node);
    }

    for system in systems_vec.iter() {
        if let Some(node) = node_map.get(&system.id) {
            if !system.parent_id.is_empty() {
                if let Some(parent) = node_map.get(&system.parent_id) {
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
                type_={node.type_}
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

        views.push(
            view! {
                <SvgLine x1={children_mid_x} y1={y - 15.0} x2={children_mid_x} y2={y - 30.0} />
            }
            .into_any(),
        );

        if node.children.len() > 1 {
            let child_node_width = node.children[0].borrow().get_node_width();
            let children_width = node.get_children_width();

            let children_start_x =
                node.children.first().unwrap().borrow().x + child_node_width * 0.5;
            let children_end_x = node.children.last().unwrap().borrow().x + child_node_width * 0.5;

            views.push(view! {
                <SvgLine x1={children_start_x} y1={y - 15.0} x2={children_end_x} y2={y - 15.0} />
            }.into_any());
        }
    }

    for child in node.children {
        views.extend(draw_node_tree(Rc::into_inner(child).unwrap().into_inner()));
    }

    views
}

fn draw_node_tree_header(world_model: &WorldModel, midpoint: f64) -> Vec<AnyView> {
    let mut views = Vec::new();

    let sources_len = world_model.environment.sources.len();
    let sinks_len = world_model.environment.sinks.len();

    let free_y_space = 65.0;
    let svg_width = 12.0;
    let svg_height = 38.0;
    let svg_gap = 20.0;
    let y = free_y_space - svg_height;

    let colors = vec![
        "red", "gray", "green", "purple", "darkblue", "blue", "black",
    ];

    let mut last_source_x = 0.0;
    let mut last_sink_x = 0.0;

    for (i, source) in world_model.environment.sources.iter().enumerate() {
        let x = midpoint + ((i + 1) as f64 * svg_width + (i + 1) as f64 * svg_gap) * -1.0;

        views.push(
            view! {
                <SvgSinkOrSource
                    x={x}
                    y={y}
                    width=svg_width
                    height=svg_height
                    color=colors[i % colors.len()]
                />
            }
            .into_any(),
        );

        if i == sources_len - 1 {
            last_source_x = x;
        }
    }

    for (i, sink) in world_model.environment.sinks.iter().enumerate() {
        let x = (i + 1) as f64 * svg_width + (i + 1) as f64 * svg_gap + midpoint;

        views.push(
            view! {
                <SvgSinkOrSource
                    x={x}
                    y={y}
                    width={svg_width * -1.0}
                    height=svg_height
                    color=colors[(i + sources_len) % colors.len()]
                />
            }
            .into_any(),
        );

        if i == sinks_len - 1 {
            last_sink_x = x;
        }
    }

    if sources_len > 0 && sinks_len > 0 {
        let start_x = last_source_x + svg_width * 0.5;
        let end_x = last_sink_x - svg_width * 0.5;
        let y = y + svg_height + 15.0;

        views.push(view! {
            <SvgLine x1={start_x} y1={y} x2={end_x} y2={y} />
            <SvgLine x1={midpoint} y1={y} x2={midpoint} y2={y + 15.0} />
            <SvgText text="Sources".to_string() font_size="0.75rem" x={start_x + 21.0} y={y + 12.5} width=50.0 height=24.0 />
            <SvgText text="Sinks".to_string() font_size="0.75rem" x={end_x - 16.0} y={y + 12.5} width=50.0 height=24.0 />
        }.into_any());
    }

    views
}

fn draw_node_tree_aside(systems: &Vec<InputSystem>, tree_start_x: f64) -> Vec<AnyView> {
    let mut views = Vec::new();

    let mut levels_set = HashSet::new();
    for sys in systems {
        levels_set.insert(sys.level);
    }

    let levels_count = levels_set.len();
    let level_padding = 50.0;

    for level in 0..=levels_count {
        if level == 0 {
            views.push(view! {
                <SvgText text="Level -1".to_string() font_size="1rem" x={tree_start_x - level_padding} y={50.0} width=70.0 height=24.0 />
            }.into_any());
            continue;
        }

        let dummy = SvgSystem {
            level: (level - 1) as i32,
            ..Default::default()
        };

        let level_text = format!("Level {}", level - 1);

        let level_y = dummy.get_node_y() + dummy.get_node_height() * 0.5 + 5.0;

        views.push(view! {
            <SvgText text={level_text} font_size="1rem" x={tree_start_x - level_padding} y={level_y} width=70.0 height=24.0 />
        }.into_any());
    }

    views
}
