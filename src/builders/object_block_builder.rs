use crate::structs::objects::{
    box_object::BoxObject, object_block::ObjectBlock, sphere::Sphere, triangle::Triangle,
};

pub fn object_block_builder() -> ObjectBlock {
    let mut object_block = ObjectBlock::default();

    object_block.objects[0] = Sphere {
        origin: [0.0, 0.0, 5.0],
        radius: 1.0,
        data: [0.0, 0.0, 0.0, 0.0],
    }
    .into();
    object_block.objects[1] = Sphere {
        origin: [1.5, 1.5, 6.0],
        radius: 0.7,
        data: [0.0, 0.0, 0.0, 1.0],
    }
    .into();
    object_block.objects[2] = BoxObject {
        origin: [0.0, -2.0, 5.5, 0.5],
        data: [1.0, 0.0, 0.0, 2.0],
    }
    .into();
    object_block.objects[3] = Triangle {
        vert1: [2.0, 2.0, 5.0, 0.0],
        vert2: [3.5, 1.0, 4.0, 0.0],
        vert3: [3.0, -2.0, 5.5, 0.0],
        data: [2.0, 0.0, 0.0, 3.0],
    }
    .into();
    object_block.objects_length = 4.0;

    object_block
}
