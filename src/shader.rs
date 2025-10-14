use std::fs;

use glium::Program;

use glsl_include::Context;

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

pub struct Shader {
    pub program: Program
}

impl Shader {
    pub fn build<'a>(display: &Display<WindowSurface>, vertex_shader_path: &str, fragment_shader_path: &str, includes_directory: Option<&str>) -> Shader {
    
        let vertex_shader = fs::read_to_string(vertex_shader_path).expect("Should have been able to read the file");
        let fragment_shader = fs::read_to_string(fragment_shader_path).expect("Should have been able to read the file");

        let mut final_fragment_shader = fragment_shader.clone();
        if includes_directory.is_some() {
            final_fragment_shader = Self::expand_shader_with_includes(&fragment_shader, includes_directory.expect("Error in unwrapping include directory string."));
        }

        Shader {
            program: glium::Program::from_source(display, &vertex_shader, &final_fragment_shader, None).unwrap()
        }
    }

    pub fn expand_shader_with_includes<'a> (shader: &'a str, includes_directory: &'a str) -> String {
        let files = fs::read_dir(includes_directory).unwrap();

        let mut expanded_shader = Context::new();

        for file in files {
            let path = file.expect("Error in reading file path.").path();
            if !path.is_dir() {
                let path_str = path.to_str().expect("Error in getting file path string");
                expanded_shader.include(path.file_name().expect("Failed to get file name.").to_str().expect("Failed to get string from OSStr"), &fs::read_to_string(path_str).expect(&format!("Include not found: {}", &path_str)));
            }
        }

        return expanded_shader.expand(shader).expect("Failed to expand shader.");
    }
}
