use crate::structs::objects::triangle::Triangle;

#[allow(dead_code)]
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
