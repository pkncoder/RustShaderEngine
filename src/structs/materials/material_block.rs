use serde::{Deserialize, Serialize};

use crate::structs::materials::material::Material;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MaterialBlock {
    pub materials: [Material; 10],
    pub materials_length: f32,
    pub _padding: [f32; 3],
}

impl Default for MaterialBlock {
    fn default() -> Self {
        MaterialBlock {
            materials: [Material { color: [0.0; 4] }; 10],
            materials_length: 0.0,
            _padding: [0.0; 3],
        }
    }
}

implement_uniform_block!(MaterialBlock, materials, materials_length);
