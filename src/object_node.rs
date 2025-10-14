use crate::structs::Sphere;
use imgui::{Ui, TreeNodeFlags, MouseButton};
use uuid::Uuid;

#[derive(PartialEq)]
pub struct Node {
    pub name: String,
    pub node_id: Uuid,

    pub sphere: Option<Sphere>,

    pub children: Vec<Node>
}

impl Node {
    pub fn new(name: String, sphere: Option<Sphere>) -> Self {
        Node {
            name: name,
            node_id: Uuid::new_v4(),

            sphere: sphere,

            children: vec![]
        }
    }

    pub fn draw_selectable_tree(ui: &Ui, node: &mut Node, selected_node_index: &mut Option<Uuid>) -> Option<Uuid> {
        let mut flags = TreeNodeFlags::OPEN_ON_ARROW
            | TreeNodeFlags::SPAN_AVAIL_WIDTH
            | TreeNodeFlags::NO_TREE_PUSH_ON_OPEN;

        if selected_node_index.is_some() && node.node_id == selected_node_index.unwrap() {
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
            if !selected_node_index.is_some() || node.node_id != selected_node_index.unwrap() {
                *selected_node_index = Some(node.node_id);
            } else {
                *selected_node_index = None;
            }
        }

        if is_open.is_some() {
            ui.indent();
            for child in &mut node.children {
                Self::draw_selectable_tree(ui, child, selected_node_index);
            }
            ui.unindent();
        }

        return *selected_node_index;
    }
}
