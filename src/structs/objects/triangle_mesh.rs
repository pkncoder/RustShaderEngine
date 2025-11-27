use std::{fs, path::Path};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::structs::objects::triangle::Triangle;

// TODO: Figure out if I want asset_name or asset_file_name or something else

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriangleMesh {
    pub obj_file: String,

    #[serde(skip)]
    pub triangles: Vec<Triangle>,

    #[serde(skip)]
    pub triangle_count: f32,
}

impl TriangleMesh {
    pub fn build(obj_file: String, triangles: Vec<Triangle>, triangle_count: f32) -> TriangleMesh {
        TriangleMesh {
            obj_file,

            triangles,
            triangle_count,
        }
    }

    pub fn new_empty_mesh() -> TriangleMesh {
        TriangleMesh {
            obj_file: "".to_string(),

            triangles: Vec::new(),
            triangle_count: 0.0,
        }
    }

    pub fn new_mesh_with_file(asset_file_name: String) -> TriangleMesh {
        TriangleMesh {
            obj_file: asset_file_name,
            triangles: Vec::new(),
            triangle_count: 0.0,
        }
    }

    pub fn new_from_file(file_path: &str) -> TriangleMesh {
        let triangles: Vec<Triangle> =
            serde_json::from_str(fs::read_to_string(file_path).unwrap().as_str()).unwrap();
        let triangle_count = triangles.len() as f32;

        TriangleMesh {
            obj_file: file_path.to_string(),

            triangles,
            triangle_count,
        }
    }

    pub fn build_triangle_vec(&self) -> Vec<Triangle> {
        let mut vec: Vec<Triangle> = Vec::new();

        let load_options = tobj::LoadOptions {
            triangulate: true, // Crucial: ensures all faces are triangles
            ..Default::default()
        };

        let obj_file = Path::new(self.obj_file.as_str());
        let (models, _) = tobj::load_obj(obj_file, &load_options).expect("Failed to load OBJ file");

        println!("Number of models: {}", models.len());

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            println!("\nModel[{}].name = '{}'", i, m.name);
            println!("Model[{}].triangle count = {}", i, mesh.indices.len() / 3);

            // Ensure the mesh has positions and indices
            if mesh.positions.is_empty() || mesh.indices.is_empty() {
                println!("Model[{}] is missing position or index data", i);
                continue;
            }

            // Iterate over the indices in groups of 3 to get each triangle
            for t in 0..mesh.indices.len() / 3 {
                // Get the indices for the three vertices of the current triangle
                let i1 = mesh.indices[t * 3] as usize;
                let i2 = mesh.indices[t * 3 + 1] as usize;
                let i3 = mesh.indices[t * 3 + 2] as usize;

                // Access the vertex positions using the indices
                // Positions are stored as a flat Vec<f32> in the order [x, y, z, x, y, z, ...]
                let v1 = [
                    mesh.positions[i1 * 3],
                    mesh.positions[i1 * 3 + 1],
                    mesh.positions[i1 * 3 + 2],
                    0.0,
                ];
                let v2 = [
                    mesh.positions[i2 * 3],
                    mesh.positions[i2 * 3 + 1],
                    mesh.positions[i2 * 3 + 2],
                    0.0,
                ];
                let v3 = [
                    mesh.positions[i3 * 3],
                    mesh.positions[i3 * 3 + 1],
                    mesh.positions[i3 * 3 + 2],
                    0.0,
                ];
                let mut rng = rand::rng();
                let random_color = rng.random_range(0..8);
                let data = [2.0, 0.0, 0.0, random_color as f32];

                vec.push(Triangle {
                    vert1: v1,
                    vert2: v2,
                    vert3: v3,
                    data,
                });

                println!("  Triangle {}: V1: {:?}, V2: {:?}, V3: {:?}", t, v1, v2, v3);

                // You can similarly access normals and texture coordinates (UVs) if they exist
                // (check if mesh.normals or mesh.texcoords are not empty).
            }
        }

        vec
    }

    pub fn expand_mesh_from_asset_name(&mut self) {
        let triangles: Vec<Triangle> = self.build_triangle_vec();
        let triangle_count = triangles.len() as f32;

        self.triangles = triangles;
        self.triangle_count = triangle_count;
    }
}
