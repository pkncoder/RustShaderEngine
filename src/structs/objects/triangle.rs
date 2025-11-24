use serde::{Deserialize, Serialize};

use crate::structs::uniforms::uniform_object::UniformObject;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    pub vert1: [f32; 4],
    pub vert2: [f32; 4],
    pub vert3: [f32; 4],
    pub data: [f32; 4],
}

impl From<Triangle> for UniformObject {
    fn from(triangle: Triangle) -> Self {
        UniformObject {
            location1: triangle.vert1,
            location2: triangle.vert2,
            location3: triangle.vert3,
            location4: [0.0; 4],
            data: triangle.data,
        }
    }
}
