pub struct RenderDataConfiguration {
    pub scene_path: String,
}

impl RenderDataConfiguration {
    pub fn build(scene_path: String) -> RenderDataConfiguration {
        RenderDataConfiguration { scene_path }
    }
}
