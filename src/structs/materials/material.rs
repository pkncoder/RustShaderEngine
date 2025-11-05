#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: [f32; 4],
}
implement_uniform_block!(Material, color);
