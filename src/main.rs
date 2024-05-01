use engine::renderer::wayland;
use engine::renderer::vulkan;
use engine::font;

pub fn main() {
    let window = wayland::init("Engine name", 1920, 1080).unwrap();
    let instance = vulkan::instance(&window.extensions).unwrap();
    let surface = vulkan::surface(&instance, window.display, window.surface).unwrap();
    let device = vulkan::device(&instance, surface).unwrap();
    let graphics_pipeline = vulkan::graphics_pipeline(&device, &instance).unwrap();
    let font = font::init("assets/fonts/font.ttf", &[b'a', b'b', b'c', b'd', b'e', b'f']).unwrap();

    let mut swapchain = vulkan::swapchain(
        &device,
        &graphics_pipeline,
        font,
        window.width,
        window.height
    ).unwrap();

    for i in 0..swapchain.command_buffers.len() {
        vulkan::record_command_buffer(&device, &swapchain, &graphics_pipeline, i as u32);
    }

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
