use glium::uniforms::{Uniforms, UniformValue};

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

    let uniforms = uniform! {
        iResolution: [width as f32, height as f32],
        ambient: [uniformStruct.ambientColor[0], uniformStruct.ambientColor[1], uniformStruct.ambientColor[2], uniformStruct.ambientPower],
        modelColoring: (UniformStruct::SHADING_MODELS[uniformStruct.shadingModel], ShaderStage::Fragment)
    };


    return uniforms;
}

pub struct CombinedUniforms<U1, U2> {
    pub first: U1,
    pub second: U2,
}

impl<U1: Uniforms, U2: Uniforms> Uniforms for CombinedUniforms<U1, U2> {
    fn visit_values<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&str, UniformValue<'a>)
    {
        self.first.visit_values(|name, val| f(name, val));
        self.second.visit_values(|name, val| f(name, val));
    }
}


pub fn combine_uniforms<U1: Uniforms, U2: Uniforms>(u1: U1, u2: U2) -> CombinedUniforms<U1, U2> {
    CombinedUniforms { first: u1, second: u2 }
}

#[macro_export]
macro_rules! append_uniforms {
    ($a:expr, $b:expr) => {
        $crate::uniforms::combine_uniforms($a, $b)
    };
}
