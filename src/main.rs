#[macro_use]
extern crate glium;

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
mod structs;

use structs::renderer_data::RenderData;
use structs::uniforms::uniform_struct::UniformStruct;

use structs::node::Node;

use editors::object_editors::object_editor::draw_object_editor;
use editors::renderer_editor::draw_renderer_editor;

use crate::{
    builders::scene_builder::scene_builder,
    structs::{
        imgui::imgui_data::ImGuiData,
        opengl::{opengl_configuration::OpenGLConfiguration, opengl_data::OpenGLData},
        uniforms::combined_uniforms::combine_uniforms,
    },
};

fn main() {
    /* Initializations and building */

    let opengl_configuration = OpenGLConfiguration::build(
        "shaders/vertex.vert".to_string(),
        "shaders/fragment.frag".to_string(),
        Some("./shaders/includes".to_string()),
    );

    let mut opengl_data = OpenGLData::build(opengl_configuration);
    let mut imgui_data = ImGuiData::build(&opengl_data.window, &opengl_data.display);

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

    // Build the render data and the top object tree node
    let mut render_data = RenderData::build(scene_block);
    let mut top_object_tree_node = Node::build_node_tree(&render_data);

    let mut selected_node_index = None;

    let object_texture_buffer;

    {
        let object_vec: Vec<[f32; 4]> = object_block
            .get_object_vec()
            .chunks_exact(4)
            .map(|c| [c[0], c[1], c[2], c[3]])
            .collect();

        object_texture_buffer =
            BufferTexture::new(&opengl_data.display, &object_vec, BufferTextureType::Float)
                .unwrap();
    }

    let material_block = scene_block.material_block;

    let material_buffer = UniformBuffer::new(&opengl_data.display, material_block).unwrap();

    /* Event loop */

    #[allow(deprecated)]
    opengl_data
        .event_loop
        .run(move |event, window_target| match event {
            Event::NewEvents(_) => {
                // Every frame

                // Get frame time
                let now = std::time::Instant::now();
                imgui_data
                    .imgui_context
                    .io_mut()
                    .update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::AboutToWait => {
                // Frame error checking
                imgui_data
                    .winit_platform
                    .prepare_frame(imgui_data.imgui_context.io_mut(), &opengl_data.window)
                    .expect("Failed to prepare frame");
                opengl_data.window.request_redraw();
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
                    let object_vec: Vec<[f32; 4]> = render_data
                        .scene_block
                        .object_block
                        .get_object_vec()
                        .chunks_exact(4)
                        .map(|c| [c[0], c[1], c[2], c[3]])
                        .collect();
                    object_texture_buffer.write(&object_vec);
                }

                // Create UI frame
                let ui = imgui_data.imgui_context.frame();

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

                let nuniform_buffer = uniforms.get_uniforms(&opengl_data.display);
                material_buffer.write(&material_block);

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
                opengl_data.frame.draw(
                    &opengl_data.display,
                    &new_uniform_buffer,
                    &Default::default(),
                    &opengl_data.window,
                    ui,
                    &mut imgui_data.winit_platform,
                );

                // Render imgui overlay
                opengl_data.frame.render_imgui(
                    &mut imgui_data.imgui_context,
                    &mut imgui_data.imgui_renderer,
                );

                // Finish frame
                opengl_data.frame.finish();

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
                    opengl_data
                        .display
                        .resize((new_size.width, new_size.height));
                }
                imgui_data.winit_platform.handle_event(
                    imgui_data.imgui_context.io_mut(),
                    &opengl_data.window,
                    &event,
                );
            }
            event => {
                // Handel each event
                imgui_data.winit_platform.handle_event(
                    imgui_data.imgui_context.io_mut(),
                    &opengl_data.window,
                    &event,
                );
            }
        })
        .expect("EventLoop error");
}
