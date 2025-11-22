use crate::{
    builders::scene_builder::scene_builder,
    structs::{
        render::render_data_configuration::RenderDataConfiguration, scenes::scene_block::SceneBlock,
    },
};

pub struct RenderData {
    pub scene_block: SceneBlock,
}

impl RenderData {
    pub fn build(configuration: RenderDataConfiguration) -> RenderData {
        RenderData {
            scene_block: scene_builder(configuration.scene_path),
        }
    }
}
