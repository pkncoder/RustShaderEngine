#![allow(non_snake_case)]

use glium::backend::glutin::Display;
use glium::backend::glutin::glutin::surface::WindowSurface;

use glium::index::NoIndices;
use glium::VertexBuffer;


#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub struct ScreenMesh {
    pub vertexBuffer: VertexBuffer<Vertex>,
    pub indices: NoIndices
}

impl ScreenMesh {
    pub fn build(display: &Display<WindowSurface>) -> ScreenMesh {

        let shape = vec![
            Vertex { position: [ 1.0, 1.0] },
            Vertex { position: [ 1.0,-1.0] },
            Vertex { position: [-1.0,-1.0] },
            Vertex { position: [-1.0, 1.0] }
        ];

        ScreenMesh {
            vertexBuffer: glium::VertexBuffer::new(display, &shape).unwrap(),
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan)
        }
    }
}
