use crate::structs::{
    materials::material_block::MaterialBlock,
    node::Node,
    objects::{object::Object, object_block::ObjectBlock},
    scenes::scene_block::SceneBlock,
};

pub struct RenderData {
    pub scene_block: SceneBlock,
}

impl RenderData {
    pub fn build(scene_block: SceneBlock) -> RenderData {
        RenderData { scene_block }
    }

    pub fn build_node_tree(&self) -> Node {
        let mut top_node = Node::new("Objects".to_string(), true, None);

        for i in 0..(self.scene_block.object_block.objects_length as usize) {
            let object_type = self.scene_block.object_block.objects[i].get_object_type();

            top_node.children.push(Node::new(
                object_type.descriptor().to_string(),
                false,
                Some(i),
            ));
        }

        top_node
    }
}
