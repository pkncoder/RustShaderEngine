use glium::uniforms::{UniformBuffer, Uniforms};

use glium::backend::glutin::glutin::surface::WindowSurface;
use glium::backend::glutin::Display;

use crate::structs::uniforms::uniform_object::UniformObject;
use crate::structs::{
    materials::material_block::MaterialBlock, objects::object_block::ObjectBlock,
};

pub struct Buffers {
    // object_buffer: UniformBuffer<ObjectBlock>,
    material_buffer: UniformBuffer<MaterialBlock>,
}

impl Buffers {
    pub fn build(display: &Display<WindowSurface>) -> Buffers {
        println!("ObjectBlock size: {}", std::mem::size_of::<ObjectBlock>());
        println!(
            "UniformObject size: {}",
            std::mem::size_of::<UniformObject>()
        );
        println!(
            "MaterialBlock size: {}",
            std::mem::size_of::<MaterialBlock>()
        );

        let default_obj_block = ObjectBlock::default();
        let default_mat_block = MaterialBlock::default();

        // let buffer = UniformBuffer::new(display, default_obj_block)
        //     .expect("Failed to create object uniform buffer");
        let mbuffer = UniformBuffer::new(display, default_mat_block)
            .expect("Failed to create material uniform buffer");

        let buffers = Buffers {
            // object_buffer: buffer,
            material_buffer: mbuffer,
        };

        // println!("Object Buffer Size: {}", buffers.object_buffer.get_size());
        println!(
            "Material Buffer Size: {}",
            buffers.material_buffer.get_size()
        );

        return buffers;
    }
}

pub fn get_buffers<'a>(
    buffers: &'a mut Buffers,
    object_data: &ObjectBlock,
    material_data: &MaterialBlock,
) -> impl Uniforms + 'a {
    // {
    //     let object_data_array = [UniformObject {
    //         location1: [0.0; 4],
    //         location2: [0.0; 4],
    //         location3: [0.0; 4],
    //         location4: [0.0; 4],
    //         data: [0.0; 4],
    //     }; 10];
    //     let mut mapping = buffers.object_buffer.map();
    //     mapping.objects = object_data_array;
    //     mapping.objects_length = object_data.objects_length;
    // }

    {
        let mut matping = buffers.material_buffer.map();
        matping.materials = material_data.materials;
        matping.materials_length = material_data.materials_length;
    }
    uniform! {
        // ObjectBlock: &buffers.object_buffer,
        MaterialBlock: &buffers.material_buffer
    }
}
