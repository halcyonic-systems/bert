use std::cell::RefCell;
use std::rc::{Rc, Weak};

const NODE_GAP: f64 = 20.0;

#[derive(Clone)]
pub struct InputSvgSystem {
    pub label: String,
    pub level: i32,
    pub parent_label: String,
    pub children: Vec<InputSvgSystem>,
}

#[derive(Clone)]
pub struct SvgSystem {
    pub x: f64,
    pub label: String,
    pub level: i32,
    pub ancestor: Option<Weak<RefCell<SvgSystem>>>,
    pub children: Vec<Rc<RefCell<SvgSystem>>>,
    pub parent: Option<Weak<RefCell<SvgSystem>>>,
    pub thread: Option<Weak<RefCell<SvgSystem>>>,
    pub offset_modifier: f64,
    pub change: f64,
    pub shift: f64,
    pub sibling_order_number: usize,
}

impl PartialEq for SvgSystem {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level && self.label == other.label
    }
}

impl SvgSystem {
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get_node_y(&self) -> f64 {
        100.0 + self.level as f64 * 70.0
    }
    pub fn get_node_width(&self) -> f64 {
        match self.level {
            0 => 90.0,
            1 => 75.0,
            2 => 60.0,
            3 => 45.0,
            4 => 35.0,
            _ => 25.0,
        }
    }
    pub fn get_node_height(&self) -> f64 {
        match self.level {
            0 => 35.0,
            1 => 30.0,
            2 => 25.0,
            3 => 20.0,
            4 => 15.0,
            _ => 10.0,
        }
    }

    pub fn get_node_font_size(&self) -> &'static str {
        match self.level {
            0 => "1.1rem",
            1 => "0.9rem",
            2 => "0.8rem",
            3 => "0.7rem",
            4 => "0.6rem",
            _ => "0.5rem",
        }
    }

    pub fn next_left(&self) -> Option<Rc<RefCell<SvgSystem>>> {
        if !self.is_leaf() {
            Some(Rc::clone(&self.children[0]))
        } else {
            self.thread.as_ref().map(|t| t.upgrade()).flatten()
        }
    }

    pub fn next_right(&self) -> Option<Rc<RefCell<SvgSystem>>> {
        if !self.is_leaf() {
            Some(Rc::clone(&self.children[self.children.len() - 1]))
        } else {
            self.thread.as_ref().map(|t| t.upgrade()).flatten()
        }
    }
    pub fn left_sibling(&self) -> Option<Rc<RefCell<SvgSystem>>> {
        let mut left_sibling = None;
        if let Some(parent) = &self.parent {
            if let Some(parent) = parent.upgrade() {
                for sibling in &parent.borrow().children {
                    if &*sibling.borrow() == self {
                        return left_sibling;
                    } else {
                        left_sibling = Some(Rc::clone(sibling));
                    }
                }
            }
        }
        left_sibling
    }

    pub fn is_leftmost_sibling(&self) -> bool {
        if let Some(parent) = &self.parent {
            if let Some(parent) = parent.upgrade() {
                return &*parent.borrow().children[0].borrow() == self;
            }
        }
        false
    }

    pub fn leftmost_sibling(&self) -> Option<Rc<RefCell<SvgSystem>>> {
        if let Some(parent) = &self.parent {
            if let Some(parent) = parent.upgrade() {
                let leftmost_sibling = &parent.borrow().children[0];
                if &*leftmost_sibling.borrow() != self {
                    return Some(Rc::clone(&parent.borrow().children[0]));
                }
            }
        }
        None
    }
}

pub fn buchheim(node: &Rc<RefCell<SvgSystem>>, initial_offset: f64) {
    first_walk(Rc::clone(node));
    debug_assert!(node.borrow().offset_modifier == 0.0);
    second_walk(Rc::clone(node), initial_offset);
}

fn first_walk(node: Rc<RefCell<SvgSystem>>) {
    let is_leaf = node.borrow().is_leaf();
    if is_leaf {
        let x = if let Some(left_sibling) = node.borrow().left_sibling() {
            left_sibling.borrow().x + node.borrow().get_node_width() + NODE_GAP
        } else {
            0.0
        };

        let mut node = node.borrow_mut();
        node.x = x;

    } else {
        let mut default_ancestor = Rc::clone(&node.borrow().children[0]);
        let children = node.borrow().children.clone();
        for child in children {
            first_walk(Rc::clone(&child));
            default_ancestor = apportion(Rc::clone(&child), default_ancestor);
        }

        execute_shifts(Rc::clone(&node));

        let midpoint = {
            let children = &node.borrow().children;
            let first_child = children[0].borrow();
            let last_child = children[children.len() - 1].borrow();
            let node_width = node.borrow().get_node_width();
            let child_width = first_child.get_node_width();
            let offset = (child_width - node_width) * 0.5;
            (first_child.x + last_child.x) * 0.5 + offset
        };

        let left_sibling = node.borrow().left_sibling();
        let (x, offset_modifier) = if let Some(left_sibling) = left_sibling {
            let x = left_sibling.borrow().x + node.borrow().get_node_width() + NODE_GAP;
            (x, x - midpoint)
        } else {
            (midpoint, node.borrow().offset_modifier)
        };

        let mut node = node.borrow_mut();
        node.x = x;
        node.offset_modifier = offset_modifier;
    }
}

fn apportion(
    node: Rc<RefCell<SvgSystem>>,
    default_ancestor: Rc<RefCell<SvgSystem>>,
) -> Rc<RefCell<SvgSystem>> {
    let has_left_sibling = node.borrow().left_sibling().is_some();
    if has_left_sibling {
        let mut node_ir = Rc::clone(&node);
        let mut node_or = Rc::clone(&node);
        let mut node_il = Rc::clone(&node.borrow().left_sibling().expect("left sibling exists"));
        let mut node_ol = node
            .borrow()
            .leftmost_sibling()
            .expect("leftmost sibling exists");
        let mut offset_ir = node.borrow().offset_modifier;
        let mut offset_or = node.borrow().offset_modifier;
        let mut offset_il = node_il.borrow().offset_modifier;
        let mut offset_ol = node_ol.borrow().offset_modifier;

        loop {
            let next_right_of_node_il = node_il.borrow().next_right();
            let next_left_of_node_ir = node_ir.borrow().next_left();

            if let (Some(next_node_il), Some(next_node_ir)) = (next_right_of_node_il, next_left_of_node_ir) {
                node_il = next_node_il;
                node_ir = next_node_ir;
            } else {
                break;
            }

            {
                let next_node_ol = node_ol.borrow().next_left().unwrap();
                node_ol = next_node_ol;
            }

            {
                let next_node_or = node_or.borrow().next_right().unwrap();
                node_or = next_node_or;
                node_or.borrow_mut().ancestor = Some(Rc::downgrade(&node));
            }

            let shift =
                (node_il.borrow().x + offset_il) - (node_ir.borrow().x + offset_ir) + node.borrow().get_node_width() + NODE_GAP;
            if shift > 0.0 {
                let ancestor = get_ancestor(&node_il, &node, &default_ancestor);

                // start move subtree
                let ancestor_sibling_order_number = ancestor.borrow().sibling_order_number;
                let node_sibling_order_number = node.borrow().sibling_order_number;
                let subtrees = node_sibling_order_number - ancestor_sibling_order_number;
                let shift_by_subtrees = shift / subtrees as f64;

                {
                    let mut node = node.borrow_mut();
                    node.change -= shift_by_subtrees;
                    node.shift += shift;
                    node.x += shift;
                    node.offset_modifier += shift;
                }

                ancestor.borrow_mut().change += shift_by_subtrees;
                // end move subtree

                offset_ir += shift;
                offset_or += shift;
            }

            offset_il += node_il.borrow().offset_modifier;
            offset_ir += node_ir.borrow().offset_modifier;
            offset_ol += node_ol.borrow().offset_modifier;
            offset_or += node_or.borrow().offset_modifier;
        }

        {
            let node_il = node_il.borrow();
            if let Some(next_node_il) = node_il.next_right() {
                let has_no_right = node_or.borrow().next_right().is_none();
                if has_no_right {
                    let mut node_or = node_or.borrow_mut();
                    node_or.thread = Some(Rc::downgrade(&next_node_il));
                    node_or.offset_modifier += offset_il - offset_or;

                    return Rc::clone(&default_ancestor);
                }
            }
        }

        {
            let node_ir = node_ir.borrow();
            if let Some(next_node_ir) = node_ir.next_left() {
                let has_no_left = node_ol.borrow().next_left().is_none();
                if has_no_left {
                    let mut node_ol = node_ol.borrow_mut();
                    node_ol.thread = Some(Rc::downgrade(&next_node_ir));
                    node_ol.offset_modifier += offset_ir - offset_ol;

                    return Rc::clone(&node);
                }
            }
        }
    }

    Rc::clone(&default_ancestor)
}

fn move_subtree(wl: Rc<RefCell<SvgSystem>>, wr: Rc<RefCell<SvgSystem>>, shift: f64) {
    let sibling_order_number_wl = wl.borrow().sibling_order_number;
    let sibling_order_number_wr = wr.borrow().sibling_order_number;
    let subtrees = sibling_order_number_wr - sibling_order_number_wl;
    let shift_by_subtrees = shift / subtrees as f64;

    let mut wr = wr.borrow_mut();
    wr.change -= shift_by_subtrees;
    wr.shift += shift;
    wr.x += shift;
    wr.offset_modifier += shift;

    wl.borrow_mut().change += shift_by_subtrees;
}

fn execute_shifts(node: Rc<RefCell<SvgSystem>>) {
    let mut shift = 0.0;
    let mut change = 0.0;
    for child in node.borrow().children.iter().rev() {
        let mut child = child.borrow_mut();
        child.x += shift;
        child.offset_modifier += shift;
        change += child.change;
        shift += child.shift + change;
    }
}

pub fn get_ancestor(
    vil: &Rc<RefCell<SvgSystem>>,
    v: &Rc<RefCell<SvgSystem>>,
    default_ancestor: &Rc<RefCell<SvgSystem>>,
) -> Rc<RefCell<SvgSystem>> {
    if let (Some(parent), Some(ancestor)) = (&v.borrow().parent, &vil.borrow().ancestor) {
        if let (Some(parent), Some(ancestor)) = (parent.upgrade(), ancestor.upgrade()) {
            if parent
                .borrow()
                .children
                .iter()
                .find(|c| *c.borrow() == *ancestor.borrow())
                .is_some()
            {
                return Rc::clone(&ancestor);
            }
        }
    }

    Rc::clone(default_ancestor)
}

fn second_walk(node: Rc<RefCell<SvgSystem>>, m: f64) {
    let offset_modifier = node.borrow().offset_modifier;
    let children = node.borrow().children.clone();

    for child in children {
        second_walk(Rc::clone(&child), m + offset_modifier);
    }

    node.borrow_mut().x += m;
}

pub fn get_node_row_width(children_len: usize, child_width: f64) -> f64 {
    if children_len == 0 {
        return 0.0;
    }
    let nodes_width = child_width * children_len as f64;
    let gap = NODE_GAP * (children_len - 1) as f64;
    nodes_width + gap
}

pub fn get_max_tree_width(system_vec: &Vec<InputSvgSystem>) -> f64 {
    let widest_level = &system_vec
        .iter()
        .max_by_key(|sys| sys.level)
        .unwrap()
        .level;

    let wides_level_count = &system_vec
        .iter()
        .filter(|sys| sys.level == *widest_level)
        .count();

    let dummy_svg_sys = SvgSystem {
        label: "".to_string(),
        level: widest_level.clone(),
        thread: None,
        x: 0.0,
        offset_modifier: 0.0,
        change: 0.0,
        shift: 0.0,
        parent: None,
        children: vec![],
        ancestor: None,
        sibling_order_number: 0,
    };

    get_node_row_width(*wides_level_count, dummy_svg_sys.get_node_width())
}