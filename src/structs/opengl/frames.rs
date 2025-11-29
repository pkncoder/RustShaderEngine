#![allow(dead_code)]

use std::rc::Rc;

use crate::structs::opengl::screen_mesh::ScreenMesh;
use crate::structs::opengl::shader::Shader;

use glium::{Frame, Surface, Texture2d};

use crate::Window;
use glium::backend::glutin::glutin::surface::WindowSurface;
use glium::backend::glutin::Display;

use glium::uniforms::Uniforms;
use glium::DrawParameters;

use imgui::Ui;

pub struct SimpleFrame {
    pub clr_red: f32,
    pub clr_blue: f32,
    pub clr_green: f32,
    pub clr_alpha: f32,

    pub texture: Rc<Texture2d>,
    pub fbo_width: usize,
    pub fbo_height: usize,

    pub linked_mesh: Option<ScreenMesh>,
    pub linked_shader: Option<Shader>,
}

impl SimpleFrame {
    pub fn build(display: &Display<WindowSurface>) -> SimpleFrame {
        let fbo_dimentions = display.get_framebuffer_dimensions();

        let fbo_width: usize = fbo_dimentions.0 as usize;
        let fbo_height: usize = fbo_dimentions.1 as usize;

        let texture: Rc<Texture2d> = Rc::new(
            glium::texture::Texture2d::empty(display, fbo_width as u32, fbo_height as u32).unwrap(),
        );

        SimpleFrame {
            clr_red: 1.0,
            clr_green: 0.4,
            clr_blue: 0.8,
            clr_alpha: 1.0,

            texture,
            fbo_width,
            fbo_height,

            linked_mesh: None,
            linked_shader: None,
        }
    }

    pub fn set_clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.clr_red = red;
        self.clr_green = green;
        self.clr_blue = blue;
        self.clr_alpha = alpha;
    }

    pub fn set_fbo_size(&mut self, display: &Display<WindowSurface>, width: usize, height: usize) {
        self.fbo_width = width;
        self.fbo_height = height;

        self.texture = Rc::new(
            glium::texture::Texture2d::empty(display, width as u32, height as u32).unwrap(),
        );
    }

    pub fn link_mesh(&mut self, mesh: ScreenMesh) {
        self.linked_mesh = Some(mesh);
    }

    pub fn link_shader(&mut self, shader: Shader) {
        self.linked_shader = Some(shader);
    }

    pub fn draw<U: Uniforms>(
        &mut self,
        display: &Display<WindowSurface>,
        uniforms: &U,
        draw_params: &DrawParameters<'_>,
        window: &Window,
        ui: &Ui,
        winit_platform: &mut imgui_winit_support::WinitPlatform,
    ) {
        let mut fbo = glium::framebuffer::SimpleFrameBuffer::new(display, &*self.texture).unwrap();

        fbo.clear_color(self.clr_red, self.clr_green, self.clr_blue, self.clr_alpha);

        fbo.draw(
            &self
                .linked_mesh
                .as_ref()
                .expect("Screen mesh needs to be linked first before vertex buffer use.")
                .vertex_buffer,
            self.linked_mesh
                .as_ref()
                .expect("Screen mesh needs to be linked first before index buffer use.")
                .indices,
            &self
                .linked_shader
                .as_ref()
                .expect("Shader needs to be linked first.")
                .program,
            uniforms,
            draw_params,
        )
        .unwrap();

        winit_platform.prepare_render(ui, window);
    }

    pub fn render_imgui(
        &mut self,
        imgui_context: &mut imgui::Context,
        renderer: &mut imgui_glium_renderer::Renderer,
        frame: &mut Frame,
    ) {
        let draw_data = imgui_context.render();
        renderer.render(frame, draw_data).unwrap();
    }

    // pub fn finish(&mut self, frame: &mut Frame) {
    //     frame.finish().take.unwrap();
    // }
}
