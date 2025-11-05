use crate::structs::{node::Node, object::Object, object_block::ObjectBlock};

pub struct RenderData {
    pub object_data: ObjectBlock,
}

impl RenderData {
    pub fn build(object_block: ObjectBlock) -> RenderData {
        RenderData {
            object_data: object_block,
        }
    }

    pub fn build_node_tree(&self) -> Node {
        let mut top_node = Node::new("Objects".to_string(), true, None);

        for i in 0..(self.object_data.objects_length as usize) {
            let object_type = self.object_data.objects[i].get_object_type();

            top_node.children.push(Node::new(
                object_type.descriptor().to_string(),
                false,
                Some(i),
            ));
        }

        top_node
    }
}
