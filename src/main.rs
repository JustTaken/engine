use engine::renderer::wayland;
use engine::renderer::vulkan;
use engine::font;

pub fn main() {
    let char_set = (32..127).collect::<Vec<u8>>();

    let mut window = wayland::init("Engine name", 1920, 1080, char_set.len()).unwrap();
    let instance = vulkan::instance(&window.extensions).unwrap();
    let surface = vulkan::surface(&instance, window.display, window.surface).unwrap();
    let device = vulkan::device(&instance, surface).unwrap();
    let graphics_pipeline = vulkan::graphics_pipeline(&device, &instance, window.width, window.height).unwrap();

    // let char_set = [
    //     b'\n', b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b'`', b'-', b'.', b'/', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';', b'<', b'=', b'<', b'?', b'@',
    //      b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
    //      b'[', b'\\', b']', b'^', b'_', b'`',
    //      b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
    //      b'{', b'|', b'}', b'~',
    // ];

    let font = font::init("assets/fonts/font.ttf", &char_set, 80).unwrap();
    let mut swapchain = vulkan::swapchain(
        &device,
        &graphics_pipeline,
        font,
        char_set.len(),
        window.width,
        window.height
    ).unwrap();

    while window.running {
        if let Err(e) = vulkan::draw_frame(&device, &mut swapchain, &graphics_pipeline, &window.lines_with_offset, window.width, window.height) {
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
