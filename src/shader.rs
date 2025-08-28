#![allow(non_snake_case)]

use std::fs;

use glium::Program;

use glsl_include::Context;

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

pub struct Shader {
    pub program: Program
}

impl Shader {
    pub fn build<'a>(display: &Display<WindowSurface>, vertexShaderPath: &str, fragmentShaderPath: &str, includesDirectory: Option<&str>) -> Shader {
    
        let vertexShader = fs::read_to_string(vertexShaderPath).expect("Should have been able to read the file");
        let fragmentShader = fs::read_to_string(fragmentShaderPath).expect("Should have been able to read the file");

        let mut finalFragmentShader = fragmentShader.clone();
        if includesDirectory.is_some() {
            finalFragmentShader = Self::expandShaderWithIncludes(&fragmentShader, includesDirectory.expect("Error in unwrapping include directory string."));
        }

        Shader {
            program: glium::Program::from_source(display, &vertexShader, &finalFragmentShader, None).unwrap()
        }
    }

    pub fn expandShaderWithIncludes<'a> (shader: &'a str, includesDirectory: &'a str) -> String {
        let files = fs::read_dir(includesDirectory).unwrap();

        let mut expandedShader = Context::new();

        for file in files {
            let path = file.expect("Error in reading file path.").path();
            if !path.is_dir() {
                let pathStr = path.to_str().expect("Error in getting file path string");
                expandedShader.include(path.file_name().expect("Failed to get file name.").to_str().expect("Failed to get string from OSStr"), &fs::read_to_string(pathStr).expect(&format!("Include not found: {}", &pathStr)));
            }
        }

        return expandedShader.expand(shader).expect("Failed to expand shader.");
    }
}
