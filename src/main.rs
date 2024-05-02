use engine::renderer::wayland;
use engine::renderer::vulkan;
use engine::font;

pub fn main() {
    let window = wayland::init("Engine name", 1920, 1080).unwrap();
    let instance = vulkan::instance(&window.extensions).unwrap();
    let surface = vulkan::surface(&instance, window.display, window.surface).unwrap();
    let device = vulkan::device(&instance, surface).unwrap();
    let graphics_pipeline = vulkan::graphics_pipeline(&device, &instance).unwrap();
    let font = font::init("assets/fonts/font.ttf", &[
        b'Q', b's', b'*', b';', b'e', b'^', b'%', b'c', b'b', b'a', b'd', b'e', b'f', b'g', b'h', b'?',
        b'i', b'j', b'k', b'l', b'm', b'n', b'p', b'q', b'r', b's', b't', b'u', b'v', b'x', b'y', b'z',
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'K', b'I', b'J', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'X', b'Y', b'Z'
    ], 100).unwrap();

    let mut swapchain = vulkan::swapchain(
        &device,
        &graphics_pipeline,
        font,
        window.width,
        window.height
    ).unwrap();

    while window.running {
        if let Err(e) = vulkan::draw_frame(&device, &swapchain) {
            if let vulkan::DrawError::HasToRecreate = e {
                vulkan::recreate_swapchain(&device, &mut swapchain, &graphics_pipeline, window.width, window.height).unwrap();
            } else {
                break;
            }
        }

        wayland::update(&window);
    }

    vulkan::shutdown_swapchain(&device, &swapchain);
    vulkan::shutdown_graphics_pipeline(&device, &graphics_pipeline);
    vulkan::shutdown_device(&device);
    vulkan::shutdown_surface(&instance, surface);
    vulkan::shutdown_instance(&instance);
    wayland::shutdown(&window);
}
