use crate::enums::object_type::ObjectType;
use crate::structs::objects::object::Object;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UniformObject {
    pub location1: [f32; 4],
    pub location2: [f32; 4],
    pub location3: [f32; 4],
    pub location4: [f32; 4],
    pub data: [f32; 4],
}
impl Object for UniformObject {
    fn get_object_type(&self) -> ObjectType {
        self.data[0].into()
    }
}

implement_uniform_block!(
    UniformObject,
    location1,
    location2,
    location3,
    location4,
    data
);
