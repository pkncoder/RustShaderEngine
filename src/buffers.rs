use glium::uniforms::{UniformBuffer, Uniforms};

use glium::backend::glutin::glutin::surface::WindowSurface;
use glium::backend::glutin::Display;

use crate::materials::*;
use crate::objects::*;

pub struct Buffers {
    object_buffer: UniformBuffer<ObjectBlock>,
    material_buffer: UniformBuffer<MaterialBlock>,
}

impl Buffers {
    pub fn build(display: &Display<WindowSurface>) -> Buffers {
        Buffers {
            object_buffer: UniformBuffer::<ObjectBlock>::empty_immutable(display).unwrap(),
            material_buffer: UniformBuffer::<MaterialBlock>::empty_immutable(display).unwrap(),
        }
    }
}

pub fn get_buffers<'a>(buffers: &'a mut Buffers, object_data: &ObjectBlock) -> impl Uniforms + 'a {
    {
        let mut mapping = buffers.object_buffer.map();
        mapping.objects = object_data.objects;
        mapping.objects_length = object_data.objects_length;
    }

    {
        let mut matping = buffers.material_buffer.map();
        matping.materials[0] = Material {
            color: [1.0, 0.0, 0.0, 0.0],
        };
        matping.materials[1] = Material {
            color: [0.0, 0.0, 1.0, 0.0],
        };
        matping.materials[2] = Material {
            color: [0.0, 1.0, 0.0, 0.0],
        };
        matping.materials_length = 3.0;
    }

    uniform! {
        ObjectBlock: &buffers.object_buffer,
        MaterialBlock: &buffers.material_buffer
    }
}
