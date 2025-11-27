use crate::{enums::object::Object, structs::uniforms::uniform_object::UniformObject};
use serde::{Deserialize, Serialize};

#[repr(C, align(16))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectBlock {
    pub objects: Vec<Object>,
    pub objects_length: f32,
    pub _padding: [f32; 3],
}

impl Default for ObjectBlock {
    fn default() -> Self {
        ObjectBlock {
            objects: vec![],
            objects_length: 0.0,
            _padding: [0.0; 3],
        }
    }
}

// TODO: Get rid of get_object_vec being mutable
// TODO: Get rid of objects_length (mostly in the scene json files)
impl ObjectBlock {
    pub fn get_object_vec(&mut self) -> Vec<[f32; 4]> {
        let mut vec = Vec::new();

        self.objects_length = 0.0;

        for object in &self.objects {
            let uniform_object_vec: Vec<UniformObject> = object.clone().into();
            self.objects_length += uniform_object_vec.len() as f32;

            for uniform_object in uniform_object_vec {
                for item in uniform_object.get_as_vec() {
                    vec.push(item);
                }
            }
        }

        let object_vec: Vec<[f32; 4]> = vec
            .chunks_exact(4)
            .map(|c| [c[0], c[1], c[2], c[3]])
            .collect();

        object_vec
    }
}
