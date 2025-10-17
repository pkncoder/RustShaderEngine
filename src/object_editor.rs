use crate::structs::Sphere;
use crate::Node;
use imgui::Ui;

pub fn draw_object_editor(
    ui: &Ui,
    selected_node: &mut Option<&mut Node>,
    object_data: &mut [Sphere],
) {
    ui.window("Object Editor")
        .size([300.0, 300.0], imgui::Condition::FirstUseEver)
        .build(|| {
            if let Some(node) = selected_node {
                if let Some(index) = node.sphere_index {
                    if index < object_data.len() {
                        let sphere = &mut object_data[index];

                        // ui.text(format!("Editing: {}", node.name));

                        ui.slider("X", -10.0, 10.0, &mut sphere.origin[0]);
                        ui.slider("Y", -10.0, 10.0, &mut sphere.origin[1]);
                        ui.slider("Z", -10.0, 10.0, &mut sphere.origin[2]);
                        ui.slider("Radius", 0.1, 10.0, &mut sphere.origin[3]);
                    } else {
                        ui.text("Sphere index out of range!");
                    }
                } else {
                    ui.text("This node has no sphere attached.");
                }
            } else {
                ui.text("Select a node to edit.");
            }
        });
}
