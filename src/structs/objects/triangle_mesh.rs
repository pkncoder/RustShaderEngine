use std::fs;

use serde::{Deserialize, Serialize};

use crate::structs::{objects::triangle::Triangle, uniforms::uniform_object::UniformObject};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriangleMesh {
    pub triangles: Vec<Triangle>,
    pub triangle_count: f32,
}

impl TriangleMesh {
    pub fn build(triangles: Vec<Triangle>, triangle_count: f32) -> TriangleMesh {
        TriangleMesh {
            triangles,
            triangle_count,
        }
    }

    pub fn new_empty_mesh() -> TriangleMesh {
        TriangleMesh {
            triangles: Vec::new(),
            triangle_count: 0.0,
        }
    }

    pub fn new_from_file(file_path: &str) -> TriangleMesh {
        let triangles: Vec<Triangle> =
            serde_json::from_str(fs::read_to_string(file_path).unwrap().as_str()).unwrap();
        let triangle_count = triangles.len() as f32;

        TriangleMesh {
            triangles,
            triangle_count,
        }
    }
}
