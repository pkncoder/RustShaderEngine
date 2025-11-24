use serde::{Deserialize, Serialize};

use crate::structs::uniforms::uniform_object::UniformObject;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoxObject {
    pub origin: [f32; 4],
    pub data: [f32; 4],
}

impl From<BoxObject> for UniformObject {
    fn from(box_obj: BoxObject) -> Self {
        UniformObject {
            location1: box_obj.origin,
            location2: [box_obj.origin[3], 0.0, 0.0, 0.0],
            location3: [0.0; 4],
            location4: [0.0; 4],
            data: box_obj.data,
        }
    }
}
