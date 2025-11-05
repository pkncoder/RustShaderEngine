use crate::structs::node::Node;
use crate::structs::uniform_object::UniformObject;
use imgui::Ui;

pub fn draw_object_editor(
    ui: &Ui,
    selected_node: &mut Option<&mut Node>,
    object_data: &mut [UniformObject],
) {
    ui.window("Object Editor")
        .size([300.0, 300.0], imgui::Condition::FirstUseEver)
        .position([20.0, 450.0], imgui::Condition::Appearing)
        .build(|| {
            if let Some(node) = selected_node {
                if let Some(index) = node.object_index {
                    if index < object_data.len() {
                        let object = &mut object_data[index];

                        ui.slider("X", -10.0, 10.0, &mut object.location1[0]);
                        ui.slider("Y", -10.0, 10.0, &mut object.location1[1]);
                        ui.slider("Z", -10.0, 10.0, &mut object.location1[2]);
                        ui.slider("Radius", 0.1, 10.0, &mut object.location1[3]);
                    } else {
                        ui.text("Object index out of range!");
                    }
                } else {
                    ui.text("This node has no object attached.");
                }
            } else {
                ui.text("Select a node to edit.");
            }
        });
}
