use imgui::Ui;

use crate::structs::objects::box_object::BoxObject;

pub fn draw_box_editor(ui: &Ui, object: &mut BoxObject) {
    ui.tree_node_config("Transform").build(|| {
        ui.slider("Origin X", -20.0, 20.0, &mut object.origin[0]);
        ui.slider("Origin Y", -20.0, 20.0, &mut object.origin[1]);
        ui.slider("Origin Z", -20.0, 20.0, &mut object.origin[2]);
    });

    ui.tree_node_config("Scale").build(|| {
        ui.slider("Cube Radius", 0.01, 20.0, &mut object.data[0]);
    });
}
