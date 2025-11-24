use crate::structs::objects::{
    box_object::BoxObject, sphere::Sphere, triangle::Triangle, triangle_mesh::TriangleMesh,
};

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Box(BoxObject),
    Triangle(Triangle),
    TriangleMesh(TriangleMesh),
}

impl Object {
    pub fn descriptor(&self) -> &str {
        match self {
            Object::Sphere(_) => "Sphere",
            Object::Box(_) => "Box",
            Object::Triangle(_) => "Triangle",
            Object::TriangleMesh(_) => "Triangle Mesh",
        }
    }
}
