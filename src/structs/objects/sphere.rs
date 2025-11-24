use crate::structs::uniforms::uniform_object::UniformObject;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub origin: [f32; 3],
    pub radius: f32,
    pub data: [f32; 4],
}

impl From<Sphere> for UniformObject {
    fn from(sphere: Sphere) -> Self {
        UniformObject {
            location1: [sphere.origin[0], sphere.origin[1], sphere.origin[2], 0.0],
            location2: [sphere.radius, 0.0, 0.0, 0.0],
            location3: [0.0; 4],
            location4: [0.0; 4],
            data: sphere.data,
        }
    }
}
