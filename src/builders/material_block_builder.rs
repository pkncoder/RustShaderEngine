use crate::structs::materials::{material::Material, material_block::MaterialBlock};

#[allow(dead_code)]
pub fn material_block_builder() -> MaterialBlock {
    let mut material_block = MaterialBlock::default();

    material_block.materials[0] = Material {
        color: [1.0, 0.0, 0.0, 0.0],
    };
    material_block.materials[1] = Material {
        color: [0.0, 0.0, 1.0, 0.0],
    };
    material_block.materials[2] = Material {
        color: [0.0, 1.0, 0.0, 0.0],
    };
    material_block.materials[3] = Material {
        color: [0.0, 1.0, 1.0, 0.0],
    };

    material_block.materials_length = 3.0;

    material_block
}
