#![allow(non_snake_case)]

use std::fs;

use glium::Program;

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

pub struct Shader {
    pub program: Program
}

impl Shader {
    pub fn build<'a>(display: &Display<WindowSurface>, vertexShaderPath: &str, fragmentShaderPath: &str) -> Shader {
    
        let vertexShader = fs::read_to_string(vertexShaderPath).expect("Should have been able to read the file");
        let fragmentShader = fs::read_to_string(fragmentShaderPath).expect("Should have been able to read the file");

        Shader {
            program: glium::Program::from_source(display, &vertexShader, &fragmentShader, None).unwrap()

        }
    }

}
