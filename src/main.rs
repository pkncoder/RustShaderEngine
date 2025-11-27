#[macro_use]
extern crate glium;

use glium::backend::winit::{
    self,
    event::{Event, WindowEvent},
    window::Window,
};

mod builders;
mod editors;
mod enums;
mod frames;
mod screen_mesh;
mod shader;
mod structs;

use structs::render::render_data::RenderData;

use structs::node::Node;

use editors::object_editors::object_editor::draw_object_editor;
use editors::renderer_editor::draw_renderer_editor;

use crate::structs::{
    imgui::imgui_data::ImGuiData,
    opengl::{opengl_configuration::OpenGLConfiguration, opengl_data::OpenGLData},
    render::render_data_configuration::RenderDataConfiguration,
    uniforms::uniform_data::UniformData,
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

    let render_data_configuration =
        RenderDataConfiguration::build("./scenes/ico_sphere.json".to_string());

    let mut render_data = RenderData::build(render_data_configuration);
    /* UNIFORMS */

    let mut uniforms = UniformData::build(&opengl_data.display, &mut render_data);

    // // Frametime
    let mut last_frame = std::time::Instant::now();

    // Build the render data and the top object tree node
    let mut top_object_tree_node = Node::build_node_tree(&render_data);

    let mut selected_node_index = None;

    /* Event loop */

    #[allow(deprecated)]
    opengl_data
        .event_loop
        .run(move |event, window_target| match event {
            Event::NewEvents(_) => {
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
                uniforms.time += 1.0;
                uniforms.frame_num += 1;

                // (Optional) If your object data changes, update the TBO
                if true {
                    // println!("Updating TBO...");
                    let object_vec: Vec<[f32; 4]> =
                        render_data.scene_block.object_block.get_object_vec();

                    uniforms.object_buffer.write(&object_vec);
                }
                // uniforms
                //     .material_buffer
                //     .write(&render_data.scene_block.material_block);

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

                let uniform_buffer = uniforms.get_uniforms();

                // --------------------------------------------------------
                // DRAW FRAME
                // --------------------------------------------------------
                opengl_data.frame.draw(
                    &opengl_data.display,
                    &uniform_buffer,
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

                // frame_num += 1.0;
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
