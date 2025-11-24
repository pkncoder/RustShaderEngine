use serde::{Deserialize, Serialize};

use crate::structs::{
    materials::material_block::MaterialBlock, objects::object_block::ObjectBlock,
};

#[repr(C)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneBlock {
    pub object_block: ObjectBlock,
    pub material_block: MaterialBlock,
}
