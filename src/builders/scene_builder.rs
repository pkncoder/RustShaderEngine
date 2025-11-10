use std::fs;

use crate::structs::scenes::scene_block::SceneBlock;

pub fn scene_builder() -> SceneBlock {
    let scene_block: SceneBlock = serde_json::from_str(
        fs::read_to_string("./scenes/cube_mesh.json")
            .unwrap()
            .as_str(),
    )
    .unwrap();

    // let object_block = object_block_builder();
    // let material_block = material_block_builder();

    scene_block
}
