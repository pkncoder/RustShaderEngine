#[macro_use]
extern crate glium;

use std::fs::File;

use glium::{
    backend::winit::{
        self,
        event::{Event, WindowEvent},
        window::Window,
    },
    texture::buffer_texture::{BufferTexture, BufferTextureType},
    uniforms::UniformBuffer,
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

use buffers::Buffers;

use editors::object_editors::object_editor::draw_object_editor;
use editors::renderer_editor::draw_renderer_editor;

use crate::{
    builders::{
        material_block_builder::material_block_builder, object_block_builder::object_block_builder,
        scene_builder::scene_builder,
    },
    structs::{scenes::scene_block::SceneBlock, uniforms::combined_uniforms::combine_uniforms},
};

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
    let mut frame_num = 0.0;

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
    //     File::create("./scenes/cube_mesh.json").unwrap(),
    //     &tempSceneBlock,
    // );

    // Object building
    let scene_block = scene_builder();
    let object_block = scene_block.object_block;
    let material_block = scene_block.material_block;

    // Build the render data and the top object tree node
    let mut render_data = RenderData::build(scene_block);
    let mut top_object_tree_node = Node::build_node_tree(&render_data);

    let mut selected_node_index = None;

    let mut object_vec: Vec<[f32; 4]> = object_block
        .get_object_vec()
        .chunks_exact(4)
        .map(|c| [c[0], c[1], c[2], c[3]])
        .collect();

    let mut object_texture_buffer =
        BufferTexture::new(&display, &object_vec, BufferTextureType::Float).unwrap();

    let mut material_block = scene_block.material_block;

    let mut material_buffer = UniformBuffer::new(&display, material_block).unwrap();

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
                // Update uniforms per frame
                uniforms.set_time(frame_num);

                // (Optional) If your object data changes, update the TBO
                if true {
                    println!("Updating TBO...");
                    object_vec = render_data
                        .scene_block
                        .object_block
                        .get_object_vec()
                        .chunks_exact(4)
                        .map(|c| [c[0], c[1], c[2], c[3]])
                        .collect();
                    object_texture_buffer.write(&object_vec);
                }

                // Create UI frame
                let ui = imgui_context.frame();

                // Draw UI
                draw_renderer_editor(ui, &mut uniforms);

                selected_node_index = Node::draw_selectable_tree(
                    ui,
                    &mut top_object_tree_node,
                    &mut selected_node_index,
                );
                let mut selected_node = top_object_tree_node.get_node_from_id(selected_node_index);

                // Object editor modifies your CPU-side data
                draw_object_editor(
                    ui,
                    &mut selected_node,
                    &mut render_data.scene_block.object_block.objects,
                );

                let nuniform_buffer = uniforms.get_uniforms(&display);
                material_buffer = UniformBuffer::new(&display, material_block).unwrap();

                let new_uniform_buffer = combine_uniforms(
                    nuniform_buffer,
                    uniform! {
                        objects: &object_texture_buffer,
                        objects_length: 10.0f32,
                        MaterialBlock: &material_buffer,
                    },
                );

                // --------------------------------------------------------
                // DRAW FRAME
                // --------------------------------------------------------
                frame.draw(
                    &display,
                    &new_uniform_buffer,
                    &Default::default(),
                    &window,
                    ui,
                    &mut winit_platform,
                );

                // Render imgui overlay
                frame.render_imgui(&mut imgui_context, &mut imgui_renderer);

                // Finish frame
                frame.finish();

                frame_num += 1.0;
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
