use std::fs;

use crate::structs::scenes::scene_block::SceneBlock;

pub fn scene_builder(scene_path: String) -> SceneBlock {
    let scene_block: SceneBlock =
        serde_json::from_str(fs::read_to_string(scene_path.as_str()).unwrap().as_str()).unwrap();

    // let object_block = object_block_builder();
    // let material_block = material_block_builder();

    /* How to write a scene block to a file */

    // let tempObjBlock = object_block_builder();
    // let tempMatBlock = material_block_builder();
    // let tempSceneBlock = SceneBlock {
    //     object_block: tempObjBlock,
    //     material_block: tempMatBlock,
    // };
    //
    // serde_json::to_writer_pretty(
    //     File::create("./scenes/cube_mesh.json").unwrap(),
    //     &tempSceneBlock,
    // );

    scene_block
}
