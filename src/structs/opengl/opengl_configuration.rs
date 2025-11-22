pub struct OpenGLConfiguration {
    pub vertex_shader_path: String,
    pub fragment_shader_path: String,
    pub shader_includes_directory: Option<String>,
}

impl OpenGLConfiguration {
    pub fn build(
        vertex_shader_path: String,
        fragment_shader_path: String,
        shader_includes_directory: Option<String>,
    ) -> OpenGLConfiguration {
        OpenGLConfiguration {
            fragment_shader_path,
            vertex_shader_path,
            shader_includes_directory,
        }
    }
}
