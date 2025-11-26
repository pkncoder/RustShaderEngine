use std::path::Path;

use rand::Rng;

use crate::{
    enums::object::Object,
    structs::objects::{object_block::ObjectBlock, triangle::Triangle},
};

#[allow(dead_code)]
#[allow(clippy::vec_init_then_push)]
pub fn object_block_builder() -> Vec<Triangle> {
    // let mut object_block = ObjectBlock::default();

    // object_block
    //     .objects
    //     .push(Object::TriangleMesh(TriangleMesh::new_mesh_with_file(
    //         "./assets/cube_mesh.json".to_string(),
    //     )));
    // TODO: WHATS WRONG WDYM "calls `push` immediately" ??????
    let mut vec: Vec<Triangle> = Vec::new();

    vec.push(Triangle {
        vert1: [-1.0, 1.0, 5.0, 0.0],
        vert2: [-1.0, -1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 0.0],
    });
    vec.push(Triangle {
        vert1: [-1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, 1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 1.0],
    });
    vec.push(Triangle {
        vert1: [1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, -1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 2.0],
    });
    vec.push(Triangle {
        vert1: [1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, 1.0, 7.0, 0.0],
        vert3: [1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 3.0],
    });
    vec.push(Triangle {
        vert1: [1.0, 1.0, 7.0, 0.0],
        vert2: [-1.0, 1.0, 7.0, 0.0],
        vert3: [-1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 4.0],
    });
    vec.push(Triangle {
        vert1: [1.0, 1.0, 7.0, 0.0],
        vert2: [1.0, -1.0, 7.0, 0.0],
        vert3: [-1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 5.0],
    });
    vec.push(Triangle {
        vert1: [-1.0, 1.0, 7.0, 0.0],
        vert2: [-1.0, 1.0, 5.0, 0.0],
        vert3: [-1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 6.0],
    });
    vec.push(Triangle {
        vert1: [-1.0, 1.0, 7.0, 0.0],
        vert2: [-1.0, -1.0, 7.0, 0.0],
        vert3: [-1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 7.0],
    });
    vec.push(Triangle {
        vert1: [-1.0, 1.0, 7.0, 0.0],
        vert2: [1.0, 1.0, 7.0, 0.0],
        vert3: [-1.0, 1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 2.0],
    });
    vec.push(Triangle {
        vert1: [-1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, 1.0, 5.0, 0.0],
        vert3: [1.0, 1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 7.0],
    });
    vec.push(Triangle {
        vert1: [-1.0, -1.0, 5.0, 0.0],
        vert2: [-1.0, -1.0, 7.0, 0.0],
        vert3: [1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 1.0],
    });
    vec.push(Triangle {
        vert1: [-1.0, -1.0, 5.0, 0.0],
        vert2: [1.0, -1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 6.0],
    });

    vec
}

pub fn get_mesh_from_obj(obj_path_name: &str) -> ObjectBlock {
    let mut vec: Vec<Object> = Vec::new();

    let load_options = tobj::LoadOptions {
        triangulate: true, // Crucial: ensures all faces are triangles
        ..Default::default()
    };

    let obj_file = Path::new(obj_path_name);
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

            vec.push(Object::Triangle(Triangle {
                vert1: v1,
                vert2: v2,
                vert3: v3,
                data,
            }));

            println!("  Triangle {}: V1: {:?}, V2: {:?}, V3: {:?}", t, v1, v2, v3);

            // You can similarly access normals and texture coordinates (UVs) if they exist
            // (check if mesh.normals or mesh.texcoords are not empty).
        }
    }

    ObjectBlock {
        objects: vec,
        objects_length: 20.0,
        _padding: [0.0, 0.0, 0.0],
    }
}
