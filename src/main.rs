#![allow(non_snake_case)]

mod frames;
use frames::SimpleFrame;

mod shader;
use shader::Shader;

#[macro_use]
extern crate glium;

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #1")
        .build(&event_loop);


    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);
    let shape = vec![
        Vertex { position: [ 0.5, 0.5] },
        Vertex { position: [ 0.5,-0.5] },
        Vertex { position: [-0.5,-0.5] },
        Vertex { position: [-0.5, 0.5] }
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);


    let Shader = Shader::build(&display, "shaders/vertex.vert", "shaders/fragment.frag");


    let mut frame = SimpleFrame::build();
    frame.setClearColor(1.0, 0.4, 0.8, 1.0);
        
    // Now we wait until the program is closed
    #[allow(deprecated)]
    event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    frame.draw(
                        &display,
                        &vertex_buffer, 
                        &indices, 
                        &Shader.program, 
                        &glium::uniforms::EmptyUniforms, 
                        &Default::default()
                    );
                },
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                _ => (),
            },
            _ => (),
        };
    })
    .unwrap();
}
