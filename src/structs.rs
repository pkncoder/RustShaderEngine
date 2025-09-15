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
    pub materialsLength: f32
}
implement_uniform_block!(MaterialBlock, materials, materialsLength);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub origin: [f32; 4],
    pub data: [f32; 4]
}
implement_uniform_block!(Sphere, origin, data);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SphereBlock {
    pub spheres: [Sphere; 10],
    pub spheresLength: f32
}
implement_uniform_block!(SphereBlock, spheres, spheresLength);
