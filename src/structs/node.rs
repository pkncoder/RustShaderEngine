use crate::structs::render::render_data::RenderData;
use imgui::{MouseButton, TreeNodeFlags, Ui};
use uuid::Uuid;

#[derive(PartialEq)]
pub struct Node {
    pub name: String,

    pub node_id: Uuid,
    pub default_open: bool,

    pub object_index: Option<usize>,

    pub children: Vec<Node>,
}

impl Node {
    pub fn new(name: String, default_open: bool, object_index: Option<usize>) -> Self {
        Node {
            name,

            node_id: Uuid::new_v4(),
            default_open,

            object_index,

            children: vec![],
        }
    }

    pub fn build_node_tree(renderer_data: &RenderData) -> Node {
        let mut top_node = Node::new("Objects".to_string(), true, None);

        for i in 0..(renderer_data.scene_block.object_block.objects_length as usize) {
            let object_type = "./src/structs/node.rs: unimplented get_object_method".to_string();

            top_node
                .children
                .push(Node::new(object_type, false, Some(i)));
        }

        top_node
    }

    pub fn draw_selectable_tree(
        ui: &Ui,
        node: &mut Node,
        selected_node_index: &mut Option<Uuid>,
    ) -> Option<Uuid> {
        let mut flags = TreeNodeFlags::OPEN_ON_ARROW
            | TreeNodeFlags::SPAN_AVAIL_WIDTH
            | TreeNodeFlags::NO_TREE_PUSH_ON_OPEN;

        if node.default_open {
            flags |= TreeNodeFlags::DEFAULT_OPEN;
        }

        if selected_node_index.is_some() && node.node_id == selected_node_index.unwrap() {
            flags |= TreeNodeFlags::SELECTED;
        }

        ui.window("Objects")
            .size([300.0, 300.0], imgui::Condition::FirstUseEver)
            .position([20.0, 130.0], imgui::Condition::Appearing)
            .build(|| {
                let label = format!("{}##{}", node.name, node.node_id);
                let is_open = ui.tree_node_config(label).flags(flags).build(|| {});

                let rect_min = ui.item_rect_min();

                let style = ui.clone_style();
                let frame_height = style.frame_padding[1] * 3.0 + ui.text_line_height();
                let arrow_region_width = frame_height;

                let mouse_pos = ui.io().mouse_pos;

                let clicked_label_only = ui.is_item_clicked_with_button(MouseButton::Left)
                    && mouse_pos[0] > rect_min[0] + arrow_region_width;

                if clicked_label_only {
                    if !selected_node_index.is_some()
                        || node.node_id != selected_node_index.unwrap()
                    {
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
            });

        *selected_node_index
    }

    pub fn get_node_from_id(&mut self, id: Option<Uuid>) -> Option<&mut Node> {
        // Check to make sure id exists
        id?;

        // Base case
        if self.node_id == id.unwrap() {
            return Some(self);
        }

        // Recursive search in children
        for child in &mut self.children {
            if let Some(found) = child.get_node_from_id(id) {
                return Some(found);
            }
        }

        // Not found
        None
    }
}
