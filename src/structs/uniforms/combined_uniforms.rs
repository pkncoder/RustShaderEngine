#![allow(dead_code)]
use glium::uniforms::{UniformValue, Uniforms};

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
        $crate::structs::uniforms::combined_uniforms::combine_uniforms($a, $b)
    };
}
