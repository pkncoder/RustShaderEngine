#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Sphere(String),
    Box(String),
    Triangle(String),
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
