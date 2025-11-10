use crate::structs::objects::{object_block::ObjectBlock, triangle::Triangle};

#[allow(dead_code)]
pub fn object_block_builder() -> ObjectBlock {
    let mut object_block = ObjectBlock::default();

    object_block.objects[0] = Triangle {
        vert1: [-1.0, 1.0, 5.0, 0.0],
        vert2: [-1.0, -1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 0.0],
    }
    .into();
    object_block.objects[1] = Triangle {
        vert1: [-1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, 1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 1.0],
    }
    .into();
    object_block.objects[2] = Triangle {
        vert1: [1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, -1.0, 5.0, 0.0],
        vert3: [1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 2.0],
    }
    .into();
    object_block.objects[3] = Triangle {
        vert1: [1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, 1.0, 7.0, 0.0],
        vert3: [1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 3.0],
    }
    .into();
    object_block.objects[4] = Triangle {
        vert1: [1.0, 1.0, 7.0, 0.0],
        vert2: [-1.0, 1.0, 7.0, 0.0],
        vert3: [-1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 4.0],
    }
    .into();
    object_block.objects[5] = Triangle {
        vert1: [1.0, 1.0, 7.0, 0.0],
        vert2: [1.0, -1.0, 7.0, 0.0],
        vert3: [-1.0, -1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 5.0],
    }
    .into();
    object_block.objects[6] = Triangle {
        vert1: [-1.0, 1.0, 7.0, 0.0],
        vert2: [-1.0, 1.0, 5.0, 0.0],
        vert3: [-1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 6.0],
    }
    .into();
    object_block.objects[7] = Triangle {
        vert1: [-1.0, 1.0, 7.0, 0.0],
        vert2: [-1.0, -1.0, 7.0, 0.0],
        vert3: [-1.0, -1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 7.0],
    }
    .into();
    object_block.objects[8] = Triangle {
        vert1: [-1.0, 1.0, 7.0, 0.0],
        vert2: [1.0, 1.0, 7.0, 0.0],
        vert3: [-1.0, 1.0, 5.0, 0.0],
        data: [2.0, 0.0, 0.0, 2.0],
    }
    .into();
    object_block.objects[9] = Triangle {
        vert1: [-1.0, 1.0, 5.0, 0.0],
        vert2: [1.0, 1.0, 5.0, 0.0],
        vert3: [1.0, 1.0, 7.0, 0.0],
        data: [2.0, 0.0, 0.0, 7.0],
    }
    .into();

    object_block.objects_length = 10.0;

    object_block
}
