use crate::structs::uniforms::uniform_object::UniformObject;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ObjectBlock {
    pub objects: [UniformObject; 10],
    pub objects_length: f32,
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
        }
    }
}

implement_uniform_block!(ObjectBlock, objects, objects_length);
