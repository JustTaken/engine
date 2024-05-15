use engine::renderer::wayland;
use engine::renderer::vulkan;
use engine::font;

pub fn main() {
    let default_width = 1920;
    let default_height = 1080;
    let char_set = (32..127).collect::<Vec<u8>>();

    let font = font::init("assets/fonts/vic.ttf", &char_set, 100).unwrap();

    let mut window = wayland::init("Engine name", default_width, default_height, font.scale).unwrap();
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
        if let Err(_) = vulkan::draw_frame(
            &device,
            &mut swapchain,
            &graphics_pipeline,
            &mut window.unique_chars,
            &window.cursor,
            window.width,
            window.height
        ) {
            break;
        }

        if window.changed {
            vulkan::set_change(&mut swapchain);
            wayland::set_unchanged(&mut window);
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
