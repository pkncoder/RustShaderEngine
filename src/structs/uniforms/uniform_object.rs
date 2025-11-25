use serde::{Deserialize, Serialize};

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct UniformObject {
    pub location1: [f32; 4],
    pub location2: [f32; 4],
    pub location3: [f32; 4],
    pub location4: [f32; 4],
    pub data: [f32; 4],
}

impl UniformObject {
    pub fn get_as_vec(&self) -> Vec<f32> {
        let mut vec = Vec::new();

        vec.append(&mut self.location1.to_vec());
        vec.append(&mut self.location2.to_vec());
        vec.append(&mut self.location3.to_vec());
        vec.append(&mut self.location4.to_vec());
        vec.append(&mut self.data.to_vec());

        vec
    }
}

implement_uniform_block!(
    UniformObject,
    location1,
    location2,
    location3,
    location4,
    data
);
