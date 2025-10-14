use glium::uniforms::{Uniforms, UniformValue};

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

use glium::program::ShaderStage;

pub struct UniformStruct {
    pub ambient_color: [f32; 3],
    pub ambient_power: f32,
    pub shading_model: usize
}

impl UniformStruct {
    pub fn build() -> UniformStruct {
        UniformStruct {
            ambient_color: [1.0, 1.0, 1.0],
            ambient_power: 0.2,
            shading_model: 0 as usize
        }
    }

    pub const SHADING_MODELS: [&'static str; 2] = ["phong", "lambertion"];
}

pub fn get_uniforms(display: &Display<WindowSurface>, uniform_struct: &UniformStruct) -> impl Uniforms {
    let (width, height) = display.get_framebuffer_dimensions();

    let uniforms = uniform! {
        iResolution: [width as f32, height as f32],
        ambient: [uniform_struct.ambient_color[0], uniform_struct.ambient_color[1], uniform_struct.ambient_color[2], uniform_struct.ambient_power],
        modelColoring: (UniformStruct::SHADING_MODELS[uniform_struct.shading_model], ShaderStage::Fragment)
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
