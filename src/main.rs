#![allow(unused_assignments)]

#[macro_use]
extern crate glium;

use glium::backend::{
    winit,
    winit::event::{Event, WindowEvent},
    winit::window::Window
};

use capitalize::Capitalize;

mod screenMesh;
use screenMesh::ScreenMesh;

mod structs;
use structs::{*};

mod shader;
use shader::Shader;

mod frames;
use frames::{SimpleFrame, imgui_init};

mod uniforms;
use uniforms::{get_uniforms, UniformStruct};

mod buffers;
use buffers::{get_buffers, Buffers};

mod object_node;
use object_node::{*};

fn main() {

    // Eventloop, window, and display building
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Rust Shader Engine")
        .build(&event_loop);

    // Winit initializion, and Imgui Context and Renderer init
    let (mut winit_platform, mut imgui_context) = imgui_init(&window);

    let mut renderer = imgui_glium_renderer::Renderer::new(&mut imgui_context, &display)
        .expect("Failed to initialize renderer");

    // Screen mesh and shader building
    let screen_mesh = ScreenMesh::build(&display);
    let shader = Shader::build(&display, "shaders/vertex.vert", "shaders/fragment.frag", Some("./shaders/includes"));

    // Frame building
    let mut frame = SimpleFrame::build();
    frame.setClearColor(1.0, 0.4, 0.8, 1.0);
    frame.linkMesh(screen_mesh);
    frame.linkShader(shader);

    // Uniform building
    let mut last_frame = std::time::Instant::now();
    let mut uniforms = UniformStruct::build();

    // Object building
    let mut object_data = SphereBlock::default();
    {
        object_data.spheres[0] = Sphere {
            origin: [0.0, 0.0, 5.0, 1.0],
            data: [0.0, 0.0, 0.0, 0.0]
        };
        object_data.spheres[1] = Sphere {
            origin: [1.5, 1.5, 6.0, 0.7],
            data: [0.0, 0.0, 0.0, 1.0]
        };
        object_data.spheres_length = 2.0;
    };

    // Node creation
    // TODO: Use objects
    let mut node = Node::new("First".to_string(), None);
    node.children.push(Node::new("Sph1".to_string(), Some(object_data.spheres[0])));
    node.children.push(Node::new("Sph2".to_string(), Some(object_data.spheres[1])));

    // Event loop
    #[allow(deprecated)]
    event_loop.run(move |event, window_target| match event {
        Event::NewEvents(_) => { // Every frame
            
            // Get frame time
            let now = std::time::Instant::now();
            imgui_context.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }
        Event::AboutToWait => { // Frame checking
            winit_platform
                .prepare_frame(imgui_context.io_mut(), &window)
                .expect("Failed to prepare frame");
            window.request_redraw();
        }
        Event::WindowEvent { // Window events (redraw)
            event: WindowEvent::RedrawRequested,
            ..
        } => { 

            // Create a new UI
            let ui = imgui_context.frame();

            // UI demo window
            ui.show_demo_window(&mut true);

            // UI renderer editor
            ui.window("Render Editor")
                .size([200.0, 100.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    ui.color_edit4("Sphere Origin", &mut object_data.spheres[0].origin);
                    ui.color_edit3("Ambient Color", &mut uniforms.ambient_color);
                    ui.slider("Ambient Amount", 0.0, 1.0, &mut uniforms.ambient_power);
                    ui.combo("Shading Model", &mut uniforms.shading_model, &UniformStruct::SHADING_MODELS, |model| {
                        model.capitalize().into()
                    });
                });

            // UI Node tree
            Node::fill_node(&ui, &mut node);

            // Buffers building
            let mut buffers = Buffers::build(&display);
            
            // Create the uniform buffer and the buffer's buffer
            let uniform_buffer = get_uniforms(&display, &uniforms);
            let buffers_buffer = get_buffers(&mut buffers, &object_data); 

            // Get a new uniform buffer from the two buffers
            let new_uniform_buffer = append_uniforms!(uniform_buffer, buffers_buffer);

            // Draw the frame
             frame.draw(
                &display,
                &new_uniform_buffer,
                &Default::default(),
                &window,
                &ui,
                &mut winit_platform,
            );

            // Draw the imgui frame
            frame.renderImgui(&mut imgui_context, &mut renderer);

            // Mark frame as complete
            frame.finish();
        }
        Event::WindowEvent { // Window event (quit)
            event: WindowEvent::CloseRequested,
            ..
        } => window_target.exit(),
        winit::event::Event::WindowEvent { // Window event (resize)
            event: winit::event::WindowEvent::Resized(new_size),
            ..
        } => {

            // Handel the new size
            if new_size.width > 0 && new_size.height > 0 {
                display.resize((new_size.width, new_size.height));
            }
            winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
        }
        event => { // Handel each event
            winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
        }
    })
    .expect("EventLoop error");

}





