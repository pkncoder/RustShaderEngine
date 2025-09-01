#![allow(non_snake_case)]

use crate::screenMesh::ScreenMesh;
use crate::shader::Shader;

use glium::{Frame, Surface};

use glium::backend::glutin::Display;
use crate::Window;
use glium::backend::glutin::glutin::surface::WindowSurface;

use glium::uniforms::Uniforms;
use glium::DrawParameters;


use imgui::Ui;

pub struct SimpleFrame {
    pub clrRed: f32,
    pub clrBlue: f32,
    pub clrGreen: f32,
    pub clrAlpha: f32,

    pub target: Option<Frame>,

    pub linkedMesh: Option<ScreenMesh>,
    pub linkedShader: Option<Shader>
}

impl SimpleFrame {

    pub fn build() -> SimpleFrame {
        SimpleFrame {
            clrRed: 0.0,
            clrGreen: 0.0,
            clrBlue: 0.0,
            clrAlpha: 0.0,

            target: None,

            linkedMesh: None,
            linkedShader: None
        }
    }

    pub fn setClearColor(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.clrRed = red;
        self.clrGreen = green;
        self.clrBlue = blue;
        self.clrAlpha = alpha;
    }

    pub fn linkMesh(&mut self, mesh: ScreenMesh) {
        self.linkedMesh = Some(mesh);
    }

    pub fn linkShader(&mut self, shader: Shader) {
        self.linkedShader = Some(shader);
    }

    pub fn draw<
        U: Uniforms
    >(
        &mut self,
        display: &Display<WindowSurface>, 
        uniforms: &U, 
        drawParams: &DrawParameters<'_>,
        window: &Window,
        ui: &Ui,
        winitPlatform: &mut imgui_winit_support::WinitPlatform,
    ) {

        let mut target = display.draw();
        target.clear_color(self.clrRed, self.clrGreen, self.clrBlue, self.clrAlpha);

        target.draw(
            &self.linkedMesh.as_ref().expect("Screen mesh needs to be linked first before vertex buffer use.").vertexBuffer,  
            &self.linkedMesh.as_ref().expect("Screen mesh needs to be linked first before index buffer use.").indices, 
            &self.linkedShader.as_ref().expect("Shader needs to be linked first.").program, 
            uniforms, 
            drawParams
        ).unwrap();

        winitPlatform.prepare_render(ui, window);

        self.target = Some(target);
    }

    pub fn renderImgui(
        &mut self,
        imguiContext: &mut imgui::Context,
        renderer: &mut imgui_glium_renderer::Renderer
    ) {
        if let Some(mut target) = self.target.take() {

            let drawData = imguiContext.render();
            renderer
                .render(&mut target, drawData)
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

pub fn imgui_init(window: &Window) -> (imgui_winit_support::WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = imgui_winit_support::WinitPlatform::new(&mut imgui_context);

    let dpi_mode = imgui_winit_support::HiDpiMode::Default;

    winit_platform.attach_window(imgui_context.io_mut(), window, dpi_mode);

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    (winit_platform, imgui_context)
}
