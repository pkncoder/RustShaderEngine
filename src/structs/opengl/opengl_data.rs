use glium::{
    backend::glutin::SimpleWindowBuilder,
    winit::{event_loop::EventLoop, window::Window},
    Display,
};
use glutin::surface::WindowSurface;

use crate::{
    frames::SimpleFrame, screen_mesh::ScreenMesh, shader::Shader,
    structs::opengl::opengl_configuration::OpenGLConfiguration,
};

pub struct OpenGLData {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub display: Display<WindowSurface>,
    pub frame: SimpleFrame,
}

impl OpenGLData {
    pub fn build(configuration: OpenGLConfiguration) -> OpenGLData {
        let event_loop = EventLoop::builder().build().expect("event loop building");
        let (window, display) = SimpleWindowBuilder::new()
            .with_title("Rust Shader Engine")
            .build(&event_loop);

        let mut frame = SimpleFrame::build();

        let shader = Shader::build(
            &display,
            &configuration.vertex_shader_path,
            &configuration.fragment_shader_path,
            configuration.shader_includes_directory.as_ref(),
        );
        frame.link_shader(shader);

        let screen_mesh = ScreenMesh::build(&display);
        frame.link_mesh(screen_mesh);

        OpenGLData {
            event_loop,
            window,
            display,
            frame,
        }
    }
}
