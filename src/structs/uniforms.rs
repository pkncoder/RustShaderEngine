use glium::uniforms::Uniforms;

use glium::backend::glutin::glutin::surface::WindowSurface;
use glium::backend::glutin::Display;

use glium::program::ShaderStage;

pub struct UniformStruct {
    pub ambient_color: [f32; 3],
    pub ambient_power: f32,
    pub shading_model: usize,
}

impl UniformStruct {
    pub const SHADING_MODELS: [&'static str; 2] = ["phong", "lambertion"];

    pub fn build() -> UniformStruct {
        UniformStruct {
            ambient_color: [1.0, 1.0, 1.0],
            ambient_power: 0.2,
            shading_model: 0,
        }
    }

    pub fn get_uniforms(&self, display: &Display<WindowSurface>) -> impl Uniforms {
        let (width, height) = display.get_framebuffer_dimensions();

        let uniforms = uniform! {
            iResolution: [width as f32, height as f32],
            ambient: [self.ambient_color[0], self.ambient_color[1], self.ambient_color[2], self.ambient_power],
            modelColoring: (UniformStruct::SHADING_MODELS[self.shading_model], ShaderStage::Fragment)
        };

        uniforms
    }
}
