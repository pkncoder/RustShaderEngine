use std::fs;

use serde::{Deserialize, Serialize};

use crate::structs::{objects::triangle::Triangle, uniforms::uniform_object::UniformObject};

// TODO: Figure out if I want asset_name or asset_file_name or something else

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriangleMesh {
    pub asset_name: String,

    #[serde(skip)]
    pub triangles: Vec<Triangle>,

    #[serde(skip)]
    pub triangle_count: f32,
}

impl TriangleMesh {
    pub fn build(
        asset_name: String,
        triangles: Vec<Triangle>,
        triangle_count: f32,
    ) -> TriangleMesh {
        TriangleMesh {
            asset_name,

            triangles,
            triangle_count,
        }
    }

    pub fn new_empty_mesh() -> TriangleMesh {
        TriangleMesh {
            asset_name: "".to_string(),

            triangles: Vec::new(),
            triangle_count: 0.0,
        }
    }

    pub fn new_mesh_with_file(asset_file_name: String) -> TriangleMesh {
        TriangleMesh {
            asset_name: asset_file_name,
            triangles: Vec::new(),
            triangle_count: 0.0,
        }
    }

    pub fn new_from_file(file_path: &str) -> TriangleMesh {
        let triangles: Vec<Triangle> =
            serde_json::from_str(fs::read_to_string(file_path).unwrap().as_str()).unwrap();
        let triangle_count = triangles.len() as f32;

        TriangleMesh {
            asset_name: file_path.to_string(),

            triangles,
            triangle_count,
        }
    }

    pub fn expand_mesh_from_asset_name(&mut self) {
        let triangles: Vec<Triangle> = serde_json::from_str(
            fs::read_to_string(self.asset_name.clone())
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let triangle_count = triangles.len() as f32;

        self.triangles = triangles;
        self.triangle_count = triangle_count;
    }
}
