use serde::{Deserialize, Serialize};

use crate::structs::{
    objects::{
        box_object::BoxObject, sphere::Sphere, triangle::Triangle, triangle_mesh::TriangleMesh,
    },
    uniforms::uniform_object::UniformObject,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl From<Object> for Vec<UniformObject> {
    fn from(object: Object) -> Vec<UniformObject> {
        let mut vector: Vec<UniformObject> = vec![];

        match object {
            Object::Sphere(o) => {
                vector.push(Into::<UniformObject>::into(o));
                vector
            }
            Object::Box(o) => {
                vector.push(Into::<UniformObject>::into(o));
                vector
            }
            Object::Triangle(o) => {
                vector.push(Into::<UniformObject>::into(o));
                vector
            }
            Object::TriangleMesh(mut o) => {
                o.expand_mesh_from_asset_name();
                let triangle_vec: Vec<Triangle> = o.triangles;

                for triangle in triangle_vec {
                    vector.push(Into::<UniformObject>::into(triangle));
                }

                vector
            }
        }
    }
}
