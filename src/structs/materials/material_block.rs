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

impl MaterialBlock {
    pub fn get_material_vec(&mut self) -> Vec<[f32; 4]> {
        let mut vec = Vec::new();

        self.materials_length = 0.0;

        for material in &self.materials {
            // let uniform_object_vec: Vec<UniformObject> = object.clone().into();
            self.materials_length += 1.0;

            // for uniform_object in uniform_object_vec {
            //     for item in uniform_object.get_as_vec() {
            //         vec.push(item);
            //     }
            // }

            for item in material.color {
                vec.push(item);
            }
        }

        let material_vec: Vec<[f32; 4]> = vec
            .chunks_exact(4)
            .map(|c| [c[0], c[1], c[2], c[3]])
            .collect();

        material_vec
    }
}
