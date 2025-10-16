use imgui::Ui;
use crate::UniformStruct;
use capitalize::Capitalize;

pub fn draw_renderer_editor(ui: &Ui, uniforms: &mut UniformStruct) {
    ui.window("Render Editor")
        .size([200.0, 100.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.color_edit3("Ambient Color", &mut uniforms.ambient_color);
            ui.slider("Ambient Amount", 0.0, 1.0, &mut uniforms.ambient_power);
            ui.combo("Shading Model", &mut uniforms.shading_model, &UniformStruct::SHADING_MODELS, |model| {
                model.capitalize().into()
            });
        });
}
