use glium::uniforms::Uniforms;

use glium::backend::glutin::glutin::surface::WindowSurface;
use glium::backend::glutin::Display;

use glium::program::ShaderStage;

#[repr(C, align(16))]
pub struct UniformStruct {
    pub ambient_color: [f32; 3],
    pub _padding1: f32,
    pub ambient_power: f32,
    pub shading_model: usize,
    pub time: f32,
    pub _padding2: f32,
}

impl UniformStruct {
    pub const SHADING_MODELS: [&'static str; 2] = ["phong", "lambertion"];

    pub fn build() -> UniformStruct {
        UniformStruct {
            ambient_color: [1.0, 1.0, 1.0],
            _padding1: 0.0,
            ambient_power: 0.2,
            shading_model: 0,
            time: 0.0,
            _padding2: 0.0,
        }
    }

    pub fn get_uniforms(
        &self,
        display: &Display<WindowSurface>,
        // object_block: &ObjectBlock,
        // object_texture_buffer: BufferTexture<[f32; 4]>,
    ) -> impl Uniforms {
        let (width, height) = display.get_framebuffer_dimensions();

        // let objects_length = object_block.objects_length;

        // objects_length = (object_vec.len() / (5 * 4)) as f32;
        // println!("{}", objects_length);

        let uniforms = uniform! {
            iResolution: [width as f32, height as f32],
            ambient: [self.ambient_color[0], self.ambient_color[1], self.ambient_color[2], self.ambient_power],
            modelColoring: (UniformStruct::SHADING_MODELS[self.shading_model], ShaderStage::Fragment),
            time: self.time,
            // objects: object_texture_buffer,
            // objects_length: 10,
        };

        uniforms
    }

    pub fn set_time(&mut self, new_time: f32) {
        self.time = new_time;
    }
}
