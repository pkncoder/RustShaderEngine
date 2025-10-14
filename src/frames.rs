#![allow(dead_code)]

use crate::screen_mesh::ScreenMesh;
use crate::shader::Shader;

use glium::{Frame, Surface};

use glium::backend::glutin::Display;
use crate::Window;
use glium::backend::glutin::glutin::surface::WindowSurface;

use glium::uniforms::Uniforms;
use glium::DrawParameters;


use imgui::Ui;

pub struct SimpleFrame {
    pub clr_red: f32,
    pub clr_blue: f32,
    pub clr_green: f32,
    pub clr_alpha: f32,

    pub target: Option<Frame>,

    pub linked_mesh: Option<ScreenMesh>,
    pub linked_shader: Option<Shader>
}

impl SimpleFrame {

    pub fn build() -> SimpleFrame {
        SimpleFrame {
            clr_red: 1.0,
            clr_green: 0.4,
            clr_blue: 0.8,
            clr_alpha: 1.0,

            target: None,

            linked_mesh: None,
            linked_shader: None
        }
    }

    pub fn set_clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.clr_red = red;
        self.clr_green = green;
        self.clr_blue = blue;
        self.clr_alpha = alpha;
    }

    pub fn link_mesh(&mut self, mesh: ScreenMesh) {
        self.linked_mesh = Some(mesh);
    }

    pub fn link_shader(&mut self, shader: Shader) {
        self.linked_shader = Some(shader);
    }

    pub fn draw<
        U: Uniforms
    >(
        &mut self,
        display: &Display<WindowSurface>, 
        uniforms: &U, 
        draw_params: &DrawParameters<'_>,
        window: &Window,
        ui: &Ui,
        winit_platform: &mut imgui_winit_support::WinitPlatform,
    ) {

        let mut target = display.draw();
        target.clear_color(self.clr_red, self.clr_green, self.clr_blue, self.clr_alpha);

        target.draw(
            &self.linked_mesh.as_ref().expect("Screen mesh needs to be linked first before vertex buffer use.").vertex_buffer,  
            &self.linked_mesh.as_ref().expect("Screen mesh needs to be linked first before index buffer use.").indices, 
            &self.linked_shader.as_ref().expect("Shader needs to be linked first.").program, 
            uniforms, 
            draw_params
        ).unwrap();

        winit_platform.prepare_render(ui, window);

        self.target = Some(target);
    }

    pub fn render_imgui(
        &mut self,
        imgui_context: &mut imgui::Context,
        renderer: &mut imgui_glium_renderer::Renderer
    ) {
        if let Some(mut target) = self.target.take() {

            let draw_data = imgui_context.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed.");

            self.target = Some(target);
        } else {
            println!("Please use linkedDraw() before rendering imgui");
        }

    }

    pub fn finish(&mut self) {
        if let Some(target) = self.target.take() {
            target.finish().unwrap()
        } else {
            println!("Please render before finishing");
        }

    }
}


