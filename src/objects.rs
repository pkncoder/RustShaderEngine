use std::convert::From;

pub trait IntoUniform: From<Self> {}

impl<T> IntoUniform for T where T: From<T> {}

#[derive(Debug)]
pub enum ObjectType {
    Sphere(String),
    Box(String),
    Triangle(String),
}

impl From<f32> for ObjectType {
    fn from(value: f32) -> Self {
        match value {
            0.0 => ObjectType::Sphere("Sphere".to_string()),
            1.0 => ObjectType::Box("Box".to_string()),
            2.0 => ObjectType::Triangle("Triangle".to_string()),
            _ => ObjectType::Sphere("Unknown".to_string()),
        }
    }
}

impl ObjectType {
    pub fn descriptor(&self) -> &str {
        match self {
            ObjectType::Sphere(desc) => desc,
            ObjectType::Box(desc) => desc,
            ObjectType::Triangle(desc) => desc,
        }
    }
}

pub trait Object: IntoUniform {
    fn get_object_type(&self) -> ObjectType;
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub origin: [f32; 3],
    pub radius: f32,
    pub data: [f32; 4],
}

impl Object for Sphere {
    fn get_object_type(&self) -> ObjectType {
        self.data[0].into()
    }
}

impl From<Sphere> for UniformObject {
    fn from(sphere: Sphere) -> Self {
        UniformObject {
            location1: [sphere.origin[0], sphere.origin[1], sphere.origin[2], 0.0],
            location2: [sphere.radius, 0.0, 0.0, 0.0],
            location3: [0.0; 4],
            location4: [0.0; 4],
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

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub vert1: [f32; 4],
    pub vert2: [f32; 4],
    pub vert3: [f32; 4],
    pub data: [f32; 4],
}

impl Object for Triangle {
    fn get_object_type(&self) -> ObjectType {
        self.data[0].into()
    }
}

impl From<Triangle> for UniformObject {
    fn from(triangle: Triangle) -> Self {
        UniformObject {
            location1: triangle.vert1,
            location2: triangle.vert2,
            location3: triangle.vert3,
            location4: [0.0; 4],
            data: triangle.data,
        }
    }
}

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
                location1: [0.0; 4],
                location2: [0.0; 4],
                location3: [0.0; 4],
                location4: [0.0; 4],
                data: [0.0; 4],
            }; 10],
            objects_length: 0.0,
        }
    }
}

implement_uniform_block!(ObjectBlock, objects, objects_length);
