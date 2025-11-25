use std::fs::{self, File};

use crate::{
    builders::{
        material_block_builder::material_block_builder, object_block_builder::object_block_builder,
    },
    structs::scenes::scene_block::SceneBlock,
};

pub fn scene_builder(scene_path: String) -> SceneBlock {
    // let tempObjBlock = object_block_builder();
    // let tempMatBlock = material_block_builder();
    // let tempSceneBlock = SceneBlock {
    //     object_block: tempObjBlock,
    //     material_block: tempMatBlock,
    // };

    // serde_json::to_writer_pretty(
    //     File::create("./assets/cube_mesh.json").unwrap(),
    //     &tempObjBlock,
    // );

    let scene_block: SceneBlock =
        serde_json::from_str(fs::read_to_string(scene_path.as_str()).unwrap().as_str()).unwrap();

    // let object_block = object_block_builder();
    // let material_block = material_block_builder();

    /* How to write a scene block to a file */

    scene_block
}
