use engine::renderer::wayland;
use engine::renderer::vulkan;
use engine::font;

pub fn main() {
    let default_width = 1920;
    let default_height = 1080;
    let char_set = (32..127).collect::<Vec<u8>>();
    let font = font::init("assets/fonts/font.ttf", &char_set, 25).unwrap();

    let mut window = wayland::init("Engine name", default_width, default_height, font.scale, font.x_ratio).unwrap();
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
        let start = std::time::Instant::now();

        if let Err(_) = vulkan::draw_frame(
            &device,
            &mut swapchain,
            &graphics_pipeline,
            &window.buffer,
            window.width,
            window.height
        ) {
            break;
        }

        if window.changed {
            vulkan::set_change(&mut swapchain);
            wayland::set_unchanged(&mut window);
        }

        wayland::update(&mut window);

        let elapsed = start.elapsed();
        // println!("this function took {} ms", elapsed.as_millis());
    }

    vulkan::shutdown_swapchain(&device, &swapchain);
    vulkan::shutdown_graphics_pipeline(&device, &graphics_pipeline);
    vulkan::shutdown_device(&device);
    vulkan::shutdown_surface(&instance, surface);
    vulkan::shutdown_instance(&instance);
    wayland::shutdown(&window);
}
