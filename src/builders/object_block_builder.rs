use crate::{
    enums::object::Object,
    structs::objects::{
        object_block::ObjectBlock, triangle::Triangle, triangle_mesh::TriangleMesh,
    },
};

#[allow(dead_code)]
pub fn object_block_builder() -> ObjectBlock {
    let mut object_block = ObjectBlock::default();

    object_block
        .objects
        .push(Object::TriangleMesh(TriangleMesh::new_mesh_with_file(
            "./assets/cube_mesh.json".to_string(),
        )));

    object_block
}
