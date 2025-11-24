use imgui::Ui;

use crate::structs::objects::triangle::Triangle;

pub fn draw_triangle_editor(ui: &Ui, object: &mut Triangle) {
    // Vertex 1
    ui.tree_node_config("Vertex One").build(|| {
        ui.slider("X", -20.0, 20.0, &mut object.vert1[0]);
        ui.slider("Y", -20.0, 20.0, &mut object.vert1[1]);
        ui.slider("Z", -20.0, 20.0, &mut object.vert1[2]);
    });

    // Vertex 2
    ui.tree_node_config("Vertex Two").build(|| {
        ui.slider("X", -20.0, 20.0, &mut object.vert2[0]);
        ui.slider("Y", -20.0, 20.0, &mut object.vert2[1]);
        ui.slider("Z", -20.0, 20.0, &mut object.vert2[2]);
    });

    // Vertex 3
    ui.tree_node_config("Vertex Three").build(|| {
        ui.slider("X", -20.0, 20.0, &mut object.vert3[0]);
        ui.slider("Y", -20.0, 20.0, &mut object.vert3[1]);
        ui.slider("Z", -20.0, 20.0, &mut object.vert3[2]);
    });
}
