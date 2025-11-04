use std::convert::From;

pub trait IntoUniform: From<Self> {}

impl<T> IntoUniform for T where T: From<T> {}

#[derive(Debug)]
pub enum ObjectType {
    Sphere(String),
    Box(String),
}

impl From<f32> for ObjectType {
    fn from(value: f32) -> Self {
        match value {
            0.0 => ObjectType::Sphere("Sphere".to_string()),
            1.0 => ObjectType::Box("Box".to_string()),
            _ => ObjectType::Sphere("Unknown".to_string()),
        }
    }
}

impl ObjectType {
    pub fn descriptor(&self) -> &str {
        match self {
            ObjectType::Sphere(desc) => desc,
            ObjectType::Box(desc) => desc,
        }
    }
}

pub trait Object: IntoUniform {
    fn get_object_type(&self) -> f32;
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub origin: [f32; 3],
    pub radius: f32,
    pub data: [f32; 4],
}

impl Object for Sphere {
    fn get_object_type(&self) -> f32 {
        self.data[0]
    }
}

impl From<Sphere> for UniformObject {
    fn from(sphere: Sphere) -> Self {
        UniformObject {
            origin: [
                sphere.origin[0],
                sphere.origin[1],
                sphere.origin[2],
                sphere.radius,
            ],
            data: sphere.data,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BoxObject {
    pub origin: [f32; 4],
    pub data: [f32; 4],
}

impl Object for BoxObject {
    fn get_object_type(&self) -> f32 {
        self.data[0]
    }
}

impl From<BoxObject> for UniformObject {
    fn from(box_obj: BoxObject) -> Self {
        UniformObject {
            origin: box_obj.origin,
            data: box_obj.data,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UniformObject {
    pub origin: [f32; 4],
    pub data: [f32; 4],
}
impl Object for UniformObject {
    fn get_object_type(&self) -> f32 {
        self.data[0]
    }
}

implement_uniform_block!(UniformObject, origin, data);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ObjectBlock {
    pub objects: [UniformObject; 10],
    pub objects_length: f32,
}

impl Default for ObjectBlock {
    fn default() -> Self {
        ObjectBlock {
            objects: [UniformObject {
                origin: [0.0; 4],
                data: [0.0; 4],
            }; 10],
            objects_length: 0.0,
        }
    }
}

implement_uniform_block!(ObjectBlock, objects, objects_length);
