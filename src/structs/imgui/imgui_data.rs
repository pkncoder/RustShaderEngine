use glium::{winit::window::Window, Display};
use glutin::surface::WindowSurface;
use imgui::Context;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::WinitPlatform;

pub struct ImGuiData {
    pub winit_platform: WinitPlatform,
    pub imgui_context: Context,
    pub imgui_renderer: Renderer,
}

impl ImGuiData {
    pub fn build(window: &Window, display: &Display<WindowSurface>) -> ImGuiData {
        let mut imgui_context = imgui::Context::create();
        imgui_context.set_ini_filename(None);

        let mut winit_platform = imgui_winit_support::WinitPlatform::new(&mut imgui_context);

        let dpi_mode = imgui_winit_support::HiDpiMode::Default;

        winit_platform.attach_window(imgui_context.io_mut(), window, dpi_mode);

        imgui_context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        let imgui_renderer = imgui_glium_renderer::Renderer::new(&mut imgui_context, display)
            .expect("Failed to initialize imgui_renderer");

        ImGuiData {
            winit_platform,
            imgui_context,
            imgui_renderer,
        }
    }
}
