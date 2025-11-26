use crate::{
    builders::{
        material_block_builder::material_block_builder, object_block_builder::get_mesh_from_obj,
    },
    structs::scenes::scene_block::SceneBlock,
};

pub fn scene_builder(obj_file_path: String) -> SceneBlock {
    let object_block = get_mesh_from_obj(obj_file_path.as_str());
    let material_block = material_block_builder();

    SceneBlock {
        object_block,
        material_block,
    }

    // serde_json::to_writer_pretty(
    //     File::create("./assets/cube_mesh.json").unwrap(),
    //     &tempObjBlock,
    // );

    // let scene_block: SceneBlock =
    //     serde_json::from_str(fs::read_to_string(scene_path.as_str()).unwrap().as_str()).unwrap();

    // let object_block = object_block_builder();
    // let material_block = material_block_builder();
}
