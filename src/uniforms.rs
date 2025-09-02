use glium::uniforms::Uniforms;

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

use glium::program::ShaderStage;

pub struct UniformStruct {
    pub ambientColor: [f32; 3],
    pub ambientPower: f32,
    pub shadingModel: usize 
}

impl UniformStruct {
    pub fn build() -> UniformStruct {
        UniformStruct {
            ambientColor: [1.0, 1.0, 1.0],
            ambientPower: 0.2,
            shadingModel: 0 as usize
        }
    }

    pub const SHADING_MODELS: [&'static str; 2] = ["phong", "lambertion"];
}

pub fn getUniforms(display: &Display<WindowSurface>, uniformStruct: &UniformStruct) -> impl Uniforms {
    let (width, height) = display.get_framebuffer_dimensions();

    uniform! {
        iResolution: [width as f32, height as f32],
        ambient: [uniformStruct.ambientColor[0], uniformStruct.ambientColor[1], uniformStruct.ambientColor[2], uniformStruct.ambientPower],
        modelColoring: (UniformStruct::SHADING_MODELS[uniformStruct.shadingModel], ShaderStage::Fragment)
    }
}
