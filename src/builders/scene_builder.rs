use std::fs;

use crate::structs::scenes::scene_block::SceneBlock;

pub fn scene_builder(scene_path: String) -> SceneBlock {
    serde_json::from_str(fs::read_to_string(scene_path.as_str()).unwrap().as_str()).unwrap()
}
