use glium::{
    backend::glutin::SimpleWindowBuilder,
    winit::{event_loop::EventLoop, window::Window},
    Display, Frame,
};
use glutin::surface::WindowSurface;

use crate::{
    screen_mesh::ScreenMesh, shader::Shader,
    structs::opengl::opengl_configuration::OpenGLConfiguration,
};

pub struct OpenGLData {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub display: Display<WindowSurface>,
    pub frame: Option<Frame>,
    pub shader: Shader,
    pub screen_mesh: ScreenMesh,
}

impl OpenGLData {
    pub fn build(configuration: OpenGLConfiguration) -> OpenGLData {
        let event_loop = EventLoop::builder().build().expect("event loop building");
        let (window, display) = SimpleWindowBuilder::new()
            .with_title("Rust Shader Engine")
            .build(&event_loop);

        let shader = Shader::build(
            &display,
            &configuration.vertex_shader_path,
            &configuration.fragment_shader_path,
            configuration.shader_includes_directory.as_ref(),
        );

        let screen_mesh = ScreenMesh::build(&display);

        OpenGLData {
            event_loop,
            window,
            display,
            frame: None,
            shader,
            screen_mesh,
        }
    }
}
