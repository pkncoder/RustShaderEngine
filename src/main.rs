#![allow(unused_assignments)]

#[macro_use]
extern crate glium;

use glium::backend::{
    winit,
    winit::event::{Event, WindowEvent},
    winit::window::Window,
};

mod buffers;
mod builders;
mod editors;
mod enums;
mod frames;
mod screen_mesh;
mod shader;
mod simple_init;
mod structs;

use frames::SimpleFrame;
use simple_init::init_app;

use screen_mesh::ScreenMesh;
use shader::Shader;

use structs::renderer_data::RenderData;
use structs::uniforms::uniform_struct::UniformStruct;

use structs::node::Node;

use buffers::{get_buffers, Buffers};

use editors::object_editors::object_editor::draw_object_editor;
use editors::renderer_editor::draw_renderer_editor;

use crate::builders::scene_builder::scene_builder;

fn main() {
    /* Initializations and building */

    // Winit / Glium / Imgui initialization
    let (event_loop, window, display, mut winit_platform, mut imgui_context, mut imgui_renderer) =
        init_app();

    // Frame, screen mesh, and shader building
    let mut frame = SimpleFrame::build();
    let screen_mesh = ScreenMesh::build(&display);
    let shader = Shader::build(
        &display,
        "shaders/vertex.vert",
        "shaders/fragment.frag",
        Some("./shaders/includes"),
    );

    // Linking the screen mesh and shader
    frame.link_mesh(screen_mesh);
    frame.link_shader(shader);

    /* UNIFORMS */

    // Build the uniforms
    let mut uniforms = UniformStruct::build();

    // Frametime
    let mut last_frame = std::time::Instant::now();

    // let tempObjBlock = object_block_builder();
    // let tempMatBlock = material_block_builder();
    // let tempSceneBlock = SceneBlock {
    //     object_block: tempObjBlock,
    //     material_block: tempMatBlock,
    // };
    //
    // serde_json::to_writer_pretty(
    //     File::create("./scenes/objects.json").unwrap(),
    //     &tempSceneBlock,
    // );

    // Object building
    let scene_block = scene_builder();
    let object_block = scene_block.object_block;
    let material_block = scene_block.material_block;

    // Build the render data and the top object tree node
    let mut render_data = RenderData::build(object_block, material_block);
    let mut top_object_tree_node = render_data.build_node_tree();

    let mut selected_node_index = None;

    /* Event loop */

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| match event {
            Event::NewEvents(_) => {
                // Every frame

                // Get frame time
                let now = std::time::Instant::now();
                imgui_context.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::AboutToWait => {
                // Frame error checking
                winit_platform
                    .prepare_frame(imgui_context.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw();
            }
            Event::WindowEvent {
                // Window events (redraw)
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Create a new UI
                let ui = imgui_context.frame();

                // UI demo window
                // ui.show_demo_window(&mut true);

                // UI imgui_renderer editor
                draw_renderer_editor(ui, &mut uniforms);

                // UI object tree
                selected_node_index = Node::draw_selectable_tree(
                    ui,
                    &mut top_object_tree_node,
                    &mut selected_node_index,
                );
                let mut selected_node = top_object_tree_node.get_node_from_id(selected_node_index);

                // Object Editor
                draw_object_editor(ui, &mut selected_node, &mut render_data.object_data.objects);

                // Buffers building
                let mut buffers = Buffers::build(&display);

                // Create the uniform buffer and the buffer's buffer
                let uniform_buffer = uniforms.get_uniforms(&display);
                let buffers_buffer = get_buffers(
                    &mut buffers,
                    &render_data.object_data,
                    &render_data.material_data,
                );

                // Append the two uniforms together to get the new uniform buffer
                let new_uniform_buffer = append_uniforms!(uniform_buffer, buffers_buffer);

                // Draw the frame
                frame.draw(
                    &display,
                    &new_uniform_buffer,
                    &Default::default(),
                    &window,
                    ui,
                    &mut winit_platform,
                );

                // Render imgui ontop of the frame
                frame.render_imgui(&mut imgui_context, &mut imgui_renderer);

                // Mark frame as complete
                frame.finish();
            }
            Event::WindowEvent {
                // Window event (quit)
                event: WindowEvent::CloseRequested,
                ..
            } => {
                window_target.exit();
            }
            winit::event::Event::WindowEvent {
                // Window event (resize)
                event: winit::event::WindowEvent::Resized(new_size),
                ..
            } => {
                // Handel the new size
                if new_size.width > 0 && new_size.height > 0 {
                    display.resize((new_size.width, new_size.height));
                }
                winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
            }
            event => {
                // Handel each event
                winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
            }
        })
        .expect("EventLoop error");
}
