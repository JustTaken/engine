use engine::renderer::wayland;
use engine::renderer::vulkan;
use engine::font;

pub fn main() {
    let default_width = 1920;
    let default_height = 1080;
    let char_set = (32..127).collect::<Vec<u8>>();
    // let char_set = [b';'];

    let font = font::init("assets/fonts/font.ttf", &char_set, 80).unwrap();

    let mut window = wayland::init("Engine name", default_width, default_height, char_set.len(), font.scale).unwrap();
    let instance = vulkan::instance(&window.extensions).unwrap();
    let surface = vulkan::surface(&instance, window.display, window.surface).unwrap();
    let device = vulkan::device(&instance, surface).unwrap();
    let graphics_pipeline = vulkan::graphics_pipeline(&device, &instance, window.width, window.height).unwrap();

    let mut swapchain = vulkan::swapchain(
        &device,
        &graphics_pipeline,
        &font,
        char_set.len(),
        window.width,
        window.height
    ).unwrap();

    while window.running {
        if let Err(e) = vulkan::draw_frame(
            &device,
            &mut swapchain,
            &graphics_pipeline,
            &window.lines_with_offset,
            window.width,
            window.height
        ) {
            if let vulkan::DrawError::HasToRecreate = e {
                vulkan::recreate_swapchain(&device, &mut swapchain, &graphics_pipeline, window.width, window.height).unwrap();
            } else {
                break;
            }
        }

        if window.content_changed {
            vulkan::invalidate_command_buffers(&mut swapchain);
            window.content_changed = false;
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
