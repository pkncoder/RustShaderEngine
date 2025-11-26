pub struct RenderDataConfiguration {
    pub obj_file_path: String,
}

impl RenderDataConfiguration {
    pub fn build(obj_file_path: String) -> RenderDataConfiguration {
        RenderDataConfiguration { obj_file_path }
    }
}
