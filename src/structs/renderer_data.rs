use crate::structs::scenes::scene_block::SceneBlock;

pub struct RenderData {
    pub scene_block: SceneBlock,
}

impl RenderData {
    pub fn build(scene_block: SceneBlock) -> RenderData {
        RenderData { scene_block }
    }
}
