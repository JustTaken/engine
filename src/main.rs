use engine::renderer::wayland;
use engine::renderer::vulkan;

pub fn main() {
    let window = wayland::init("Engine name", 1920, 1080).unwrap();
    let dispatch = vulkan::dispatch(&window.extensions).unwrap();
    let surface = vulkan::surface(&dispatch, window.display, window.surface).unwrap();
    let _ = vulkan::device(&dispatch, surface).unwrap();

    wayland::shutdown(&window);
    vulkan::shutdown_surface(&dispatch, surface);
    vulkan::shutdown_instance(&dispatch);
}
