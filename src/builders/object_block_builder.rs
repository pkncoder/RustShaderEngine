use std::fs;

use crate::structs::scenes::scene_block::SceneBlock;

pub fn get_block_from_scene_file(scene_path: &str) -> SceneBlock {
    let scene_block: SceneBlock =
        serde_json::from_str(fs::read_to_string(scene_path).unwrap().as_str()).unwrap();

    // for i in 0..scene_block.object_block.objects.len() {
    //     let object = &scene_block.object_block.objects[i];
    //
    //     match object {
    //         Object::Sphere(_) => continue,
    //         Object::Box(_) => continue,
    //         Object::Triangle(_) => continue,
    //         Object::TriangleMesh(o) => {}
    //     }
    // }

    scene_block
}
