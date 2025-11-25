use glium::{
    program::ShaderStage,
    texture::buffer_texture::{BufferTexture, BufferTextureType},
    uniforms::{UniformBuffer, Uniforms},
    Display,
};
use glutin::surface::WindowSurface;

use crate::structs::{materials::material_block::MaterialBlock, render::render_data::RenderData};

pub struct UniformData {
    pub screen_resolution: [f32; 2],

    pub time: f32,
    pub frame_num: u32,

    pub shading_model: usize,

    pub ambient_color: [f32; 3],
    pub ambient_power: f32,

    pub object_buffer: BufferTexture<[f32; 4]>,
    pub object_num: f32,

    pub material_buffer: UniformBuffer<MaterialBlock>,
}

impl UniformData {
    pub const SHADING_MODELS: [&'static str; 2] = ["phong", "lambertion"];

    pub fn build(display: &Display<WindowSurface>, render_data: &mut RenderData) -> UniformData {
        let (width, height) = display.get_framebuffer_dimensions();

        let object_buffer = BufferTexture::new(
            display,
            &render_data.scene_block.object_block.get_object_vec(),
            BufferTextureType::Float,
        )
        .unwrap();

        let object_num = render_data.scene_block.object_block.objects_length;

        let material_buffer =
            UniformBuffer::new(display, render_data.scene_block.material_block).unwrap();

        UniformData {
            screen_resolution: [width as f32, height as f32],

            time: 1.0,
            frame_num: 0,

            shading_model: 0,

            ambient_color: [1.0, 1.0, 1.0],
            ambient_power: 0.2,

            object_buffer,
            object_num,

            material_buffer,
        }
    }

    pub fn get_uniforms(&self) -> impl Uniforms + '_ {
        let uniforms = uniform! {
            iResolution: self.screen_resolution,
            ambient: [self.ambient_color[0], self.ambient_color[1], self.ambient_color[2], self.ambient_power],

            time: self.time,

            modelColoring: (UniformData::SHADING_MODELS[self.shading_model], ShaderStage::Fragment),


            objects: &self.object_buffer,
            objects_length: self.object_num,
            MaterialBlock: &self.material_buffer,
        };

        uniforms
    }
}
