use imgui::Ui;

use crate::structs::uniforms::uniform_object::UniformObject;

pub fn draw_triangle_editor(ui: &Ui, object: &mut UniformObject) {
    // Vertex 1
    ui.tree_node_config("Vertex One").build(|| {
        ui.slider("X", -20.0, 20.0, &mut object.location1[0]);
        ui.slider("Y", -20.0, 20.0, &mut object.location1[1]);
        ui.slider("Z", -20.0, 20.0, &mut object.location1[2]);
    });

    // Vertex 2
    ui.tree_node_config("Vertex Two").build(|| {
        ui.slider("X", -20.0, 20.0, &mut object.location2[0]);
        ui.slider("Y", -20.0, 20.0, &mut object.location2[1]);
        ui.slider("Z", -20.0, 20.0, &mut object.location2[2]);
    });

    // Vertex 3
    ui.tree_node_config("Vertex Three").build(|| {
        ui.slider("X", -20.0, 20.0, &mut object.location3[0]);
        ui.slider("Y", -20.0, 20.0, &mut object.location3[1]);
        ui.slider("Z", -20.0, 20.0, &mut object.location3[2]);
    });
}
