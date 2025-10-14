use crate::structs::Sphere;
use imgui::{Ui, TreeNodeFlags, MouseButton};

#[derive(PartialEq)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,

    pub sphere: Option<Sphere>,

    pub is_selected: bool
}

impl Node {
    pub fn new(name: String, sphere: Option<Sphere>) -> Self {
        Node {
            name: name,
            children: vec![],
            sphere: sphere,
            is_selected: false
        }
    }

    pub fn fill_node(ui: &Ui, node: &mut Node) {
        let mut flags = TreeNodeFlags::OPEN_ON_ARROW
            | TreeNodeFlags::SPAN_AVAIL_WIDTH
            | TreeNodeFlags::NO_TREE_PUSH_ON_OPEN;

        if node.is_selected {
            flags |= TreeNodeFlags::SELECTED;
        }

        let is_open = ui
            .tree_node_config(&node.name)
            .flags(flags)
            .build(|| {});

        let rect_min = ui.item_rect_min();

        let style = ui.clone_style();
        let frame_height = style.frame_padding[1] * 3.0 + ui.text_line_height();
        let arrow_region_width = frame_height;

        let mouse_pos = ui.io().mouse_pos;

        let clicked_label_only = ui.is_item_clicked_with_button(MouseButton::Left)
            && mouse_pos[0] > rect_min[0] + arrow_region_width;

        if clicked_label_only {
            node.is_selected = !node.is_selected;
        }

        if is_open.is_some() {
            ui.indent();
            for child in &mut node.children {
                Self::fill_node(ui, child);
            }
            ui.unindent();
        }
    }
}
