use crate::structs::uniforms::uniform_object::UniformObject;
use serde::{Deserialize, Serialize};

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectBlock {
    pub objects: [UniformObject; 10],
    pub objects_length: f32,
    pub _padding: [f32; 3],
}

impl Default for ObjectBlock {
    fn default() -> Self {
        ObjectBlock {
            objects: [UniformObject {
                location1: [0.0; 4],
                location2: [0.0; 4],
                location3: [0.0; 4],
                location4: [0.0; 4],
                data: [0.0; 4],
            }; 10],
            objects_length: 0.0,
            _padding: [0.0; 3],
        }
    }
}

impl ObjectBlock {
    pub fn get_object_vec(&self) -> Vec<f32> {
        let mut vec = Vec::new();

        for uniform_object in self.objects {
            vec.append(&mut uniform_object.get_as_vec());
        }

        vec
    }
}

implement_uniform_block!(ObjectBlock, objects, objects_length);
