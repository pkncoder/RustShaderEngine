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
    pub materials_length: f32
}
implement_uniform_block!(MaterialBlock, materials, materials_length);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub origin: [f32; 4],
    pub data: [f32; 4]
}
implement_uniform_block!(Sphere, origin, data);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SphereBlock {
    pub spheres: [Sphere; 10],
    pub spheres_length: f32
}

impl Default for SphereBlock {
    fn default() -> Self {
        SphereBlock {
            spheres: [ Sphere {
                origin: [0.0; 4],
                data: [0.0; 4]
            }; 10],
            spheres_length: 0.0
        }
    }
}

implement_uniform_block!(SphereBlock, spheres, spheres_length);
