use imgui::Ui;

use crate::structs::uniforms::uniform_object::UniformObject;

pub fn draw_sphere_editor(ui: &Ui, object: &mut UniformObject) {
    ui.tree_node_config("Transform").build(|| {
        ui.slider("Origin X", -20.0, 20.0, &mut object.location1[0]);
        ui.slider("Origin Y", -20.0, 20.0, &mut object.location1[1]);
        ui.slider("Origin Z", -20.0, 20.0, &mut object.location1[2]);
    });

    ui.tree_node_config("Scale").build(|| {
        ui.slider("Sphere Radius", 0.01, 20.0, &mut object.location2[0]);
    });
}
