use crate::{
    builders::object_block_builder::get_block_from_scene_file,
    structs::scenes::scene_block::SceneBlock,
};

pub fn scene_builder(scene_path: String) -> SceneBlock {
    let scene_block = get_block_from_scene_file(scene_path.as_str());

    scene_block

    // serde_json::to_writer_pretty(
    //     File::create("./assets/cube_mesh.json").unwrap(),
    //     &tempObjBlock,
    // );

    // let scene_block: SceneBlock =
    //     serde_json::from_str(fs::read_to_string(scene_path.as_str()).unwrap().as_str()).unwrap();

    // let object_block = object_block_builder();
    // let material_block = material_block_builder();
}
