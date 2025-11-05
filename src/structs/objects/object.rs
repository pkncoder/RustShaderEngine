use crate::enums::object_type::ObjectType;
use std::convert::From;

pub trait IntoUniform: From<Self> {}

impl<T> IntoUniform for T where T: From<T> {}

pub trait Object: IntoUniform {
    fn get_object_type(&self) -> ObjectType;
}
