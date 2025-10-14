#![allow(unused_assignments)]

#[macro_use]
extern crate glium;

use glium::backend::{
    winit,
    winit::event::{Event, WindowEvent},
    winit::window::Window
};

use capitalize::Capitalize;

mod simple_init;
use simple_init::init_app;

mod screen_mesh;
use screen_mesh::ScreenMesh;

mod structs;
use structs::{*};

mod shader;
use shader::Shader;

mod frames;
use frames::SimpleFrame;

mod uniforms;
use uniforms::{get_uniforms, UniformStruct};

mod buffers;
use buffers::{get_buffers, Buffers};

mod object_node;
use object_node::Node;

fn main() {


    /* Initializations and building */

    // Winit / Glium / Imgui initialization
    let (event_loop, window, display, mut winit_platform, mut imgui_context, mut imgui_renderer) = init_app();

    // Frame, screen mesh, and shader building
    let mut frame = SimpleFrame::build();
    let screen_mesh = ScreenMesh::build(&display);
    let shader = Shader::build(&display, "shaders/vertex.vert", "shaders/fragment.frag", Some("./shaders/includes"));

    // Linking the screen mesh and shader
    frame.link_mesh(screen_mesh);
    frame.link_shader(shader);


    /* UNIFORMS */

    // Build the uniforms
    let mut uniforms = UniformStruct::build();

    // Frametime
    let mut last_frame = std::time::Instant::now();

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
    let mut node = Node::new("First".to_string(), None);
    node.children.push(Node::new("Sph1".to_string(), Some(object_data.spheres[0])));
    node.children.push(Node::new("Sph2".to_string(), Some(object_data.spheres[1])));


    /* Event loop */

    #[allow(deprecated)]
    event_loop.run(move |event, window_target| match event {
        Event::NewEvents(_) => { // Every frame
            
            // Get frame time
            let now = std::time::Instant::now();
            imgui_context.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }
        Event::AboutToWait => { // Frame error checking
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

            // UI imgui_renderer editor
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

            // UI object tree
            Node::draw_selectable_tree(&ui, &mut node);

            // Buffers building
            let mut buffers = Buffers::build(&display);
            
            // Create the uniform buffer and the buffer's buffer
            let uniform_buffer = get_uniforms(&display, &uniforms);
            let buffers_buffer = get_buffers(&mut buffers, &object_data); 

            // Append the two uniforms together to get the new uniform buffer
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

            // Render imgui ontop of the frame
            frame.render_imgui(&mut imgui_context, &mut imgui_renderer);

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





