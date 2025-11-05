use crate::enums::object_type::ObjectType;
use crate::structs::objects::object::Object;
use crate::structs::uniforms::uniform_object::UniformObject;

#[derive(Copy, Clone, Debug)]
pub struct BoxObject {
    pub origin: [f32; 4],
    pub data: [f32; 4],
}

impl Object for BoxObject {
    fn get_object_type(&self) -> ObjectType {
        self.data[0].into()
    }
}

impl From<BoxObject> for UniformObject {
    fn from(box_obj: BoxObject) -> Self {
        UniformObject {
            location1: box_obj.origin,
            location2: [box_obj.origin[3], 0.0, 0.0, 0.0],
            location3: [0.0; 4],
            location4: [0.0; 4],
            data: box_obj.data,
        }
    }
}
