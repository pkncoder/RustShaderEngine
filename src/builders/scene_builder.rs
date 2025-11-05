use crate::{
    builders::{
        material_block_builder::material_block_builder, object_block_builder::object_block_builder,
    },
    structs::{materials::material_block::MaterialBlock, objects::object_block::ObjectBlock},
};

pub fn scene_builder() -> (ObjectBlock, MaterialBlock) {
    let object_block = object_block_builder();
    let material_block = material_block_builder();

    (object_block, material_block)
}
