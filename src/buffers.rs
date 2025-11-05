use glium::uniforms::{UniformBuffer, Uniforms};

use glium::backend::glutin::glutin::surface::WindowSurface;
use glium::backend::glutin::Display;

use crate::structs::{material_block::MaterialBlock, object_block::ObjectBlock};

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

pub fn get_buffers<'a>(
    buffers: &'a mut Buffers,
    object_data: &ObjectBlock,
    material_data: &MaterialBlock,
) -> impl Uniforms + 'a {
    {
        let mut mapping = buffers.object_buffer.map();
        mapping.objects = object_data.objects;
        mapping.objects_length = object_data.objects_length;
    }

    {
        let mut matping = buffers.material_buffer.map();
        matping.materials = material_data.materials;
        matping.materials_length = material_data.materials_length;
    }

    uniform! {
        ObjectBlock: &buffers.object_buffer,
        MaterialBlock: &buffers.material_buffer
    }
}
