#![allow(non_snake_case)]

use crate::shader::Shader;

use glium::Surface;

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

use glium::uniforms::Uniforms;
use glium::DrawParameters;

use glium::Program;

use glium::index::IndicesSource;
use glium::vertex::MultiVerticesSource;

pub struct SimpleFrame {
    pub clrRed: f32,
    pub clrBlue: f32,
    pub clrGreen: f32,
    pub clrAlpha: f32,

    pub linkedShader: Option<Shader>
}

impl SimpleFrame {

    pub fn build() -> SimpleFrame {
        SimpleFrame {
            clrRed: 0.0,
            clrGreen: 0.0,
            clrBlue: 0.0,
            clrAlpha: 0.0,

            linkedShader: None
        }
    }

    pub fn setClearColor(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.clrRed = red;
        self.clrGreen = green;
        self.clrBlue = blue;
        self.clrAlpha = alpha;
    }

    pub fn linkShader(&mut self, shader: Shader) {
        self.linkedShader = Some(shader);
    }

    #[allow(dead_code)]
    pub fn draw<
        'a,
        'b,
        V: MultiVerticesSource<'a>, 
        I: Into<IndicesSource<'b>>,
        U: Uniforms
    >(
        &mut self,
        display: &Display<WindowSurface>,
        vertexBuffer: V,
        indexBuffer: I,
        program: &Program, 
        uniforms: &U, 
        drawParams: &DrawParameters<'_>
    ) {
        let mut target = display.draw();
        target.clear_color(self.clrRed, self.clrGreen, self.clrBlue, self.clrAlpha);
        target.draw(vertexBuffer, indexBuffer, program, uniforms, drawParams).unwrap();

        target.finish().unwrap();
    }

    pub fn linkedDraw<
        'a,
        'b,
        V: MultiVerticesSource<'a>, 
        I: Into<IndicesSource<'b>>,
        U: Uniforms
    >(
        &mut self,
        display: &Display<WindowSurface>,
        vertexBuffer: V,
        indexBuffer: I,
        uniforms: &U, 
        drawParams: &DrawParameters<'_>
    ) {
        let mut target = display.draw();
        target.clear_color(self.clrRed, self.clrGreen, self.clrBlue, self.clrAlpha);

        target.draw(vertexBuffer, indexBuffer, &self.linkedShader.as_ref().expect("Shader needs to be linked first.").program, uniforms, drawParams).unwrap();

        target.finish().unwrap();
    }
}
