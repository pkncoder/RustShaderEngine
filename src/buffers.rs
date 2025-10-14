use glium::uniforms::{Uniforms, UniformBuffer};

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

use crate::structs::{*};

pub struct Buffers {
    sphereBuffer: UniformBuffer<SphereBlock>,
    materialBuffer: UniformBuffer<MaterialBlock>
}

impl Buffers {

    pub fn build(display: &Display<WindowSurface>) -> Buffers {
        Buffers {
            sphereBuffer: UniformBuffer::<SphereBlock>::empty_immutable(display).unwrap(),
            materialBuffer: UniformBuffer::<MaterialBlock>::empty_immutable(display).unwrap()
        }
    }}

pub fn getBuffers<'a>(buffers: &'a mut Buffers, objectData: &SphereBlock) -> impl Uniforms + 'a {
    {
        let mut mapping = buffers.sphereBuffer.map();
        mapping.spheres = objectData.spheres;
        mapping.spheresLength = objectData.spheresLength;
    }

    {
        let mut matping = buffers.materialBuffer.map();
        matping.materials[0] = Material {
            color: [1.0, 0.0, 0.0, 0.0]
        };
        matping.materials[1] = Material {
            color: [0.0, 0.0, 1.0, 0.0]
        };
        matping.materialsLength = 2.0;
    }

    let uniforms = uniform! {
        SphereBlock: &buffers.sphereBuffer,
        MaterialBlock: &buffers.materialBuffer
    };

    return uniforms;
}

