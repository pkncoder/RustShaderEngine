use glium::uniforms::{UniformValue, Uniforms};

use glium::backend::glutin::glutin::surface::WindowSurface;
use glium::backend::glutin::Display;

use crate::objects::{Object, ObjectBlock};

use glium::program::ShaderStage;

use crate::object_tree::Node;

pub struct UniformStruct {
    pub ambient_color: [f32; 3],
    pub ambient_power: f32,
    pub shading_model: usize,
}

impl UniformStruct {
    pub fn build() -> UniformStruct {
        UniformStruct {
            ambient_color: [1.0, 1.0, 1.0],
            ambient_power: 0.2,
            shading_model: 0,
        }
    }

    pub const SHADING_MODELS: [&'static str; 2] = ["phong", "lambertion"];
}

pub fn get_uniforms(
    display: &Display<WindowSurface>,
    uniform_struct: &UniformStruct,
) -> impl Uniforms {
    let (width, height) = display.get_framebuffer_dimensions();

    let uniforms = uniform! {
        iResolution: [width as f32, height as f32],
        ambient: [uniform_struct.ambient_color[0], uniform_struct.ambient_color[1], uniform_struct.ambient_color[2], uniform_struct.ambient_power],
        modelColoring: (UniformStruct::SHADING_MODELS[uniform_struct.shading_model], ShaderStage::Fragment)
    };

    uniforms
}

pub struct RenderData {
    pub object_data: ObjectBlock,
}

impl RenderData {
    pub fn build(object_block: ObjectBlock) -> RenderData {
        RenderData {
            object_data: object_block,
        }
    }

    pub fn build_node_tree(&self) -> Node {
        let mut top_node = Node::new("Objects".to_string(), true, None);

        for i in 0..(self.object_data.objects_length as usize) {
            let object_type = self.object_data.objects[i].get_object_type();

            top_node.children.push(Node::new(
                object_type.descriptor().to_string(),
                false,
                Some(i),
            ));
        }

        top_node
    }
}

pub struct CombinedUniforms<U1, U2> {
    pub first: U1,
    pub second: U2,
}

impl<U1: Uniforms, U2: Uniforms> Uniforms for CombinedUniforms<U1, U2> {
    fn visit_values<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&str, UniformValue<'a>),
    {
        self.first.visit_values(|name, val| f(name, val));
        self.second.visit_values(|name, val| f(name, val));
    }
}

pub fn combine_uniforms<U1: Uniforms, U2: Uniforms>(u1: U1, u2: U2) -> CombinedUniforms<U1, U2> {
    CombinedUniforms {
        first: u1,
        second: u2,
    }
}

#[macro_export]
macro_rules! append_uniforms {
    ($a:expr, $b:expr) => {
        $crate::uniforms::combine_uniforms($a, $b)
    };
}
