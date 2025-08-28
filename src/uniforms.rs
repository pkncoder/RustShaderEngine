use glium::uniforms::Uniforms;

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

pub fn getUniforms(display: &Display<WindowSurface>) -> impl Uniforms {
    let (width, height) = display.get_framebuffer_dimensions();

    uniform! {
        iResolution: [width as f32, height as f32]
    }
}
