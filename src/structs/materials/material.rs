use serde::{Deserialize, Serialize};

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    pub color: [f32; 4],
}
implement_uniform_block!(Material, color);
