use glium::uniforms::Uniforms;

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

pub fn getUniforms(display: &Display<WindowSurface>) -> impl Uniforms {
    let (width, height) = display.get_framebuffer_dimensions();

    uniform! {
        iResolution: [width as f32, height as f32],
        ambient: [1.0 as f32, 1.0 as f32, 1.0 as f32, 0.2 as f32]
    }
}
