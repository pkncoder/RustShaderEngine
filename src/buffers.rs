use glium::uniforms::{Uniforms, UniformBuffer};

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

use crate::structs::SphereBlock;
use crate::uniforms::UniformStruct;

pub fn getBuffers(display: &Display<WindowSurface>, uniformStruct: &UniformStruct) -> impl Uniforms {

    let mut sphereBuffer = UniformBuffer::<SphereBlock>::empty_immutable(display).unwrap();
    sphereBuffer.map().spheres[0] = uniformStruct.sphere;
    sphereBuffer.map().spheresLength = 1.0;

    // TODO: **AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA**
    let sphere_buffer: &'static mut UniformBuffer<SphereBlock> = Box::leak(Box::new(sphereBuffer));

    let buffers = uniform! {
        SphereBlock: &*sphere_buffer
    };

    return buffers;
}

