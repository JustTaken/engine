use engine::renderer::wayland;
use engine::renderer::vulkan;

pub fn main() {
    let window = wayland::display("Engine name", 1920, 1080).unwrap();
    let _ = vulkan::init(window.display, window.surface, &window.extensions).unwrap();

    wayland::shutdown(&window);
}
