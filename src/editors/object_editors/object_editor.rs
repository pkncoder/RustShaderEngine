use crate::editors::object_editors::box_editors::draw_box_editor;
use crate::editors::object_editors::mesh_editor::draw_mesh_editor;
use crate::editors::object_editors::sphere_editor::draw_sphere_editor;
use crate::editors::object_editors::triangle_editor::draw_triangle_editor;
use crate::enums::object::Object;
use crate::structs::node::Node;
use imgui::Ui;

pub fn draw_object_editor(
    ui: &Ui,
    selected_node: &mut Option<&mut Node>,
    object_data: &mut [Object],
) {
    ui.window("Object Editor")
        .size([300.0, 300.0], imgui::Condition::FirstUseEver)
        .position([20.0, 450.0], imgui::Condition::Appearing)
        .build(|| {
            if selected_node.is_none() {
                ui.text("Select a node to edit.");
                return;
            }

            let node = selected_node.as_ref().unwrap();
            if node.object_index.is_none() {
                ui.text("This node has no object attached.");
                return;
            }

            let index = node.object_index.unwrap();
            if index >= object_data.len() {
                ui.text("Object index out of range!");
            }

            let object = &mut object_data[index];

            match object {
                Object::Sphere(o) => {
                    draw_sphere_editor(ui, o);
                }
                Object::Box(o) => {
                    draw_box_editor(ui, o);
                }
                Object::Triangle(o) => {
                    draw_triangle_editor(ui, o);
                }
                Object::TriangleMesh(o) => {
                    draw_mesh_editor(ui, o);
                }
            }
        });
}
