use glium::uniforms::{Uniforms, UniformBuffer};

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

use crate::structs::{*};

pub struct Buffers {
    sphere_buffer: UniformBuffer<SphereBlock>,
    material_buffer: UniformBuffer<MaterialBlock>
}

impl Buffers {

    pub fn build(display: &Display<WindowSurface>) -> Buffers {
        Buffers {
            sphere_buffer: UniformBuffer::<SphereBlock>::empty_immutable(display).unwrap(),
            material_buffer: UniformBuffer::<MaterialBlock>::empty_immutable(display).unwrap()
        }
    }}

pub fn get_buffers<'a>(buffers: &'a mut Buffers, object_data: &SphereBlock) -> impl Uniforms + 'a {
    {
        let mut mapping = buffers.sphere_buffer.map();
        mapping.spheres = object_data.spheres;
        mapping.spheres_length = object_data.spheres_length;
    }

    {
        let mut matping = buffers.material_buffer.map();
        matping.materials[0] = Material {
            color: [1.0, 0.0, 0.0, 0.0]
        };
        matping.materials[1] = Material {
            color: [0.0, 0.0, 1.0, 0.0]
        };
        matping.materials_length = 2.0;
    }

    let uniforms = uniform! {
        SphereBlock: &buffers.sphere_buffer,
        MaterialBlock: &buffers.material_buffer
    };

    return uniforms;
}

