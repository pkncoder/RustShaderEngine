#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: [f32; 4],
}
implement_uniform_block!(Material, color);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MaterialBlock {
    pub materials: [Material; 10],
    pub materials_length: f32,
}
implement_uniform_block!(MaterialBlock, materials, materials_length);
