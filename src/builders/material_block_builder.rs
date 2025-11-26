use crate::structs::materials::{material::Material, material_block::MaterialBlock};

#[allow(dead_code)]
pub fn material_block_builder() -> MaterialBlock {
    let mut material_block = MaterialBlock::default();

    material_block.materials[0] = Material {
        color: [1.0, 0.0, 0.0, 0.0],
    };
    material_block.materials[1] = Material {
        color: [0.0, 1.0, 0.0, 0.0],
    };
    material_block.materials[2] = Material {
        color: [0.0, 0.0, 1.0, 0.0],
    };
    material_block.materials[3] = Material {
        color: [1.0, 1.0, 0.0, 0.0],
    };
    material_block.materials[4] = Material {
        color: [0.0, 1.0, 1.0, 0.0],
    };
    material_block.materials[5] = Material {
        color: [1.0, 0.0, 1.0, 0.0],
    };
    material_block.materials[6] = Material {
        color: [1.0, 1.0, 1.0, 0.0],
    };
    material_block.materials[7] = Material {
        color: [0.0, 0.0, 0.0, 0.0],
    };
    material_block.materials[8] = Material {
        color: [0.69020, 0.04314, 0.40784, 0.0],
    };

    material_block.materials_length = 9.0;

    material_block
}
