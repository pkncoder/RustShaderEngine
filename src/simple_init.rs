use glium::backend::{
    glutin::{glutin::surface::WindowSurface, Display, SimpleWindowBuilder},
    winit::event_loop::EventLoop,
    winit::window::Window,
};

use imgui::Context;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::WinitPlatform;

// use glium::backend::glutin::glutin::surface::WindowSurface;
pub fn init_app() -> (
    EventLoop<()>,
    Window,
    Display<WindowSurface>,
    WinitPlatform,
    Context,
    Renderer,
) {
    let event_loop = EventLoop::builder().build().expect("event loop building");
    let (window, display) = SimpleWindowBuilder::new()
        .with_title("Rust Shader Engine")
        .build(&event_loop);

    let (winit_platform, mut imgui_context) = imgui_init(&window);

    let imgui_renderer = imgui_glium_renderer::Renderer::new(&mut imgui_context, &display)
        .expect("Failed to initialize imgui_renderer");

    (
        event_loop,
        window,
        display,
        winit_platform,
        imgui_context,
        imgui_renderer,
    )
}

pub fn imgui_init(window: &Window) -> (WinitPlatform, Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = imgui_winit_support::WinitPlatform::new(&mut imgui_context);

    let dpi_mode = imgui_winit_support::HiDpiMode::Default;

    winit_platform.attach_window(imgui_context.io_mut(), window, dpi_mode);

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    (winit_platform, imgui_context)
}
