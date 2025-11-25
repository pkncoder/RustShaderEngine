use imgui::Ui;

use crate::structs::objects::triangle_mesh::TriangleMesh;

pub fn draw_mesh_editor(ui: &Ui, object: &mut TriangleMesh) {
    // ui.tree_node_config("Transform").build(|| {
    //     ui.slider("Origin X", -20.0, 20.0, &mut object.location1[0]);
    //     ui.slider("Origin Y", -20.0, 20.0, &mut object.location1[1]);
    //     ui.slider("Origin Z", -20.0, 20.0, &mut object.location1[2]);
    // });
    //
    // ui.tree_node_config("Scale").build(|| {
    //     ui.slider("Cube Radius", 0.01, 20.0, &mut object.location2[0]);
    // });

    ui.text(format!("Cannot Edit Meshes Yet : ( {}", object.asset_name));
}
