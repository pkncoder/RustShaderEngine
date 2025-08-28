#![allow(non_snake_case)]

#[macro_use]
extern crate glium;

mod screenMesh;
use screenMesh::ScreenMesh;

mod shader;
use shader::Shader;

mod frames;
use frames::SimpleFrame;

mod uniforms;
use uniforms::getUniforms;

fn main() {
    
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Rust Shader Engine")
        .build(&event_loop);

    let screenMesh = ScreenMesh::build(&display);
    let shader = Shader::build(&display, "shaders/vertex.vert", "shaders/fragment.frag", Some("./shaders/includes"));

    let mut frame = SimpleFrame::build();
    frame.setClearColor(1.0, 0.4, 0.8, 1.0);
    frame.linkMesh(screenMesh);
    frame.linkShader(shader);

    let mut uniforms = getUniforms(&display);

    #[allow(deprecated)]
    event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                glium::winit::event::WindowEvent::RedrawRequested => {

                    frame.linkedDraw(
                        &display,
                        &uniforms,
                        &Default::default()
                    );
                },
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());

                    uniforms = getUniforms(&display);
                },
                _ => (),
            },
            _ => (),
        };
    })
    .unwrap();
}
