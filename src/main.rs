#[macro_use]
extern crate glium;

use std::{collections::VecDeque, time::Duration};

use glium::{
    backend::winit::{
        self,
        event::{Event, WindowEvent},
        window::Window,
    },
    Surface,
};

mod builders;
mod editors;
mod enums;
mod structs;

use imgui_glium_renderer::Texture;
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
    imgui_data.imgui_context.io_mut().config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;

    let render_data_configuration =
        RenderDataConfiguration::build("./scenes/ico_sphere.json".to_string());

    let mut render_data = RenderData::build(render_data_configuration);
    /* UNIFORMS */

    let mut uniforms = UniformData::build(&opengl_data.display, &mut render_data);

    // Build the render data and the top object tree node
    let mut top_object_tree_node = Node::build_node_tree(&render_data);

    let mut selected_node_index = None;

    // // Frametime
    let mut last_frame: Option<Duration> = None;
    let mut fastest_frame = Duration::MAX;
    let mut slowest_frame = Duration::ZERO;
    let mut frametime_queue: VecDeque<Duration> = VecDeque::new();
    let frametime_queue_max_length = 60;

    /* Event loop */

    #[allow(deprecated)]
    opengl_data
        .event_loop
        .run(move |event, window_target| match event {
            Event::NewEvents(_) => {}
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
                let start_rendering = std::time::Instant::now();

                // Update uniforms per frame
                uniforms.time += 1.0;
                uniforms.frame_num += 1;

                // Create UI frame
                let ui = imgui_data.imgui_context.frame();
                ui.dockspace_over_main_viewport();

                if let Some(last_frame_time) = last_frame {
                    if last_frame_time < fastest_frame {
                        fastest_frame = last_frame_time;
                    }

                    if last_frame_time > slowest_frame {
                        slowest_frame = last_frame_time;
                    }
                }

                ui.window("Frametime Status (one behind)").build(|| {
                    ui.text(format!("Frame Time: {:?}", last_frame));
                    if !frametime_queue.is_empty() {
                        ui.text(format!(
                            "Average Frame Time: {:?}",
                            frametime_queue
                                .iter()
                                .sum::<Duration>()
                                .div_f32(frametime_queue.len() as f32),
                        ));
                    }
                    ui.text(format!("Fastest Frame Time: {:?}", fastest_frame));
                    ui.text(format!("Slowest Frame Time: {:?}", slowest_frame));
                    if ui.button("Reset") {
                        fastest_frame = Duration::MAX;
                        slowest_frame = Duration::ZERO;
                    }
                });

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

                let mut frame = opengl_data.display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 0.0);

                let imgui_tex = Texture {
                    texture: opengl_data.frame.texture.clone(), // or reference
                    sampler: Default::default(),
                };

                let tex_id = imgui_data.imgui_renderer.textures().insert(imgui_tex); // takes glium::texture::Texture2d

                let style_1 = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));

                let mut scaled_width = 0.0;
                let mut scaled_height = 0.0;

                ui.window("Viewport").title_bar(false).build(|| {
                    // 1. Get the available space in the current window
                    let content_region_avail = ui.content_region_avail();
                    let content_width = content_region_avail[0];
                    let content_height = content_region_avail[1];

                    // Calculate aspect ratios
                    let image_aspect_ratio = opengl_data.display.get_framebuffer_dimensions().0
                        as f32
                        / opengl_data.display.get_framebuffer_dimensions().1 as f32;
                    let content_aspect_ratio = content_width / content_height;

                    scaled_width = content_width;
                    scaled_height = content_height;
                    let mut x_padding = 0.0;
                    let mut y_padding = 0.0;

                    // 2. Calculate scaling factor to maintain aspect ratio
                    if content_aspect_ratio > image_aspect_ratio {
                        // Content region is wider, scale based on height
                        scaled_width = content_height * image_aspect_ratio;
                        x_padding = (content_width - scaled_width) / 2.0;
                    } else {
                        // Content region is taller or aspect ratios match, scale based on width
                        scaled_height = content_width / image_aspect_ratio;
                        y_padding = (content_height - scaled_height) / 2.0;
                    }

                    // 3. Set cursor position for centering (if needed) and render image
                    // Push the cursor position to center the image if there is padding
                    ui.set_cursor_pos([
                        ui.cursor_pos()[0] + x_padding,
                        ui.cursor_pos()[1] + y_padding,
                    ]);

                    imgui::Image::new(tex_id, [scaled_width, scaled_height]).build(ui);
                });

                style_1.pop();

                // Render imgui overlay
                opengl_data.frame.render_imgui(
                    &mut imgui_data.imgui_context,
                    &mut imgui_data.imgui_renderer,
                    &mut frame,
                );

                // Finish frame
                frame.finish().unwrap();

                let end_rendering = std::time::Instant::now();
                let frame_time = end_rendering - start_rendering;

                last_frame = Some(frame_time);
                frametime_queue.push_back(last_frame.unwrap());

                if frametime_queue.len() > frametime_queue_max_length {
                    frametime_queue.pop_front();
                }

                imgui_data.imgui_renderer.textures().remove(tex_id);

                if opengl_data.frame.fbo_width != scaled_width as usize
                    || opengl_data.frame.fbo_height != scaled_height as usize
                {
                    opengl_data.frame.set_fbo_size(
                        &opengl_data.display,
                        scaled_width as usize,
                        scaled_height as usize,
                    );
                }
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
