#![allow(non_snake_case)]
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
use uniforms::{getUniforms, UniformStruct};

mod buffers;
use buffers::{getBuffers, Buffers};

mod objectNode;
use objectNode::{*};

fn main() {
    
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Rust Shader Engine")
        .build(&event_loop);

    let (mut winit_platform, mut imgui_context) = imgui_init(&window);

    // imgui_context.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

    let mut renderer = imgui_glium_renderer::Renderer::new(&mut imgui_context, &display)
        .expect("Failed to initialize renderer");

    let screenMesh = ScreenMesh::build(&display);
    let shader = Shader::build(&display, "shaders/vertex.vert", "shaders/fragment.frag", Some("./shaders/includes"));

    let mut frame = SimpleFrame::build();
    frame.setClearColor(1.0, 0.4, 0.8, 1.0);
    frame.linkMesh(screenMesh);
    frame.linkShader(shader);

    let mut last_frame = std::time::Instant::now();
    let mut uniforms = UniformStruct::build();

    let mut objectData = SphereBlock::default();
    {
        objectData.spheres[0] = Sphere {
            origin: [0.0, 0.0, 5.0, 1.0],
            data: [0.0, 0.0, 0.0, 0.0]
        };
        objectData.spheres[1] = Sphere {
            origin: [1.5, 1.5, 6.0, 0.7],
            data: [0.0, 0.0, 0.0, 1.0]
        };
        objectData.spheresLength = 2.0;
    };

    let mut node = Node::new("First".to_string(), None);
    node.children.push(Node::new("Sph1".to_string(), Some(objectData.spheres[0])));
    node.children.push(Node::new("Sph2".to_string(), Some(objectData.spheres[1])));

    #[allow(deprecated)]
    event_loop.run(move |event, window_target| match event {
        Event::NewEvents(_) => {
            let now = std::time::Instant::now();
            imgui_context.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }
        Event::AboutToWait => {
            winit_platform
                .prepare_frame(imgui_context.io_mut(), &window)
                .expect("Failed to prepare frame");
            window.request_redraw();
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {

            let ui = imgui_context.frame();

            ui.show_demo_window(&mut true);
            ui.window("Render Editor")
                .size([200.0, 100.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    ui.color_edit4("Sphere Origin", &mut objectData.spheres[0].origin);
                    ui.color_edit3("Ambient Color", &mut uniforms.ambientColor);
                    ui.slider("Ambient Amount", 0.0, 1.0, &mut uniforms.ambientPower);
                    ui.combo("Shading Model", &mut uniforms.shadingModel, &UniformStruct::SHADING_MODELS, |model| {
                        model.capitalize().into()
                    });
                }); 

            fill_node(&ui, &mut node);

            let mut buffers = Buffers::build(&display);
            
            let uniformBuffer = getUniforms(&display, &uniforms);
            let buffersBuffer = getBuffers(&mut buffers, &objectData); 

            let newUniformBuffer = append_uniforms!(uniformBuffer, buffersBuffer);

             frame.draw(
                &display,
                &newUniformBuffer,
                &Default::default(),
                &window,
                &ui,
                &mut winit_platform,
            );

            frame.renderImgui(&mut imgui_context, &mut renderer);

            frame.finish();
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => window_target.exit(),
        winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::Resized(new_size),
            ..
        } => {
            if new_size.width > 0 && new_size.height > 0 {
                display.resize((new_size.width, new_size.height));
            }
            winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
        }
        event => {
            winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
        }
    })
    .expect("EventLoop error");

}





