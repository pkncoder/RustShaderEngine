use crate::{
    enums::object::Object,
    structs::objects::{object_block::ObjectBlock, triangle::Triangle},
};

#[allow(dead_code)]
pub fn object_block_builder() -> ObjectBlock {
    let mut object_block = ObjectBlock::default();

    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [-1.0, 1.0, 5.0, 0.0],
        vert2: [-1.0, -1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 0.0],
    }));
    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [-1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, 1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 1.0],
    }));
    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, -1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 2.0],
    }));
    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, 1.0, 7.0, 0.0],
        vert3: [1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 3.0],
    }));
    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [1.0, 1.0, 7.0, 0.0],
        vert2: [-1.0, 1.0, 7.0, 0.0],
        vert3: [-1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 4.0],
    }));
    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [1.0, 1.0, 7.0, 0.0],
        vert2: [1.0, -1.0, 7.0, 0.0],
        vert3: [-1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 5.0],
    }));
    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [-1.0, 1.0, 7.0, 0.0],
        vert2: [-1.0, 1.0, 5.0, 0.0],
        vert3: [-1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 6.0],
    }));
    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [-1.0, 1.0, 7.0, 0.0],
        vert2: [-1.0, -1.0, 7.0, 0.0],
        vert3: [-1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 7.0],
    }));
    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [-1.0, 1.0, 7.0, 0.0],
        vert2: [1.0, 1.0, 7.0, 0.0],
        vert3: [-1.0, 1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 2.0],
    }));
    object_block.objects.push(Object::Triangle(Triangle {
        vert1: [-1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, 1.0, 5.0, 0.0],
        vert3: [1.0, 1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 7.0],
    }));

    object_block.objects_length = 10.0;

    object_block
}
