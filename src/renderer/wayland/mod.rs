use crate::binding::wayland;
pub mod buffer;

pub struct Core {
    pub display: *mut wayland::wl_display,
    pub surface: *mut wayland::wl_surface,

    pub extensions: [*const i8; 2],
    pub running: bool,
    pub width: u32,
    pub height: u32,

    pub changed: bool,
    pub buffers: Vec<buffer::Buffer>,
    pub main_buffer_index: u32,

    completion_lines: Vec<buffer::Line>,

    command_mode: bool,
    command: Vec<u8>,

    scale: f32,
    x_ratio: f32,
    window_ratio: f32,

    chars_per_row: u32,
    chars_per_coloumn: u32,

    key_delay: std::time::Duration,
    key_rate: std::time::Duration,

    last_inserted_char: u8,
    last_function: Option<fn(&mut Core)>,
    last_fetch_rate: std::time::Instant,
    last_fetch_delay: std::time::Instant,
    alt_modifier: bool,
    shift_modifier: bool,
    control_modifier: bool,

    registry: *mut wayland::wl_registry,
    compositor: *mut wayland::wl_compositor,
    seat: *mut wayland::wl_seat,
    keyboard: *mut wayland::wl_keyboard,
    xdg_shell: *mut wayland::xdg_wm_base,
    xdg_surface: *mut wayland::xdg_surface,
    xdg_toplevel: *mut wayland::xdg_toplevel,

    seat_listener: wayland::wl_seat_listener,
    keyboard_listener: wayland::wl_keyboard_listener,
    registry_listener: wayland::wl_registry_listener,
    shell_listener: wayland::xdg_wm_base_listener,
    shell_surface_listener: wayland::xdg_surface_listener,
    toplevel_listener: wayland::xdg_toplevel_listener,
}

#[derive(Debug)]
pub enum WaylandError {
    CouldNotAddListener,
    NotAscci,
}

pub fn set_unchanged(core: &mut Core) {
    core.changed = false;
}

pub fn update(core: &mut Core) {
    unsafe {
        wayland::wl_proxy_marshal_flags(core.surface as *mut wayland::wl_proxy, wayland::WL_SURFACE_COMMIT, std::ptr::null(), wayland::wl_proxy_get_version(core.surface as *mut wayland::wl_proxy), 0);
        wayland::wl_display_roundtrip(core.display);
    };

    if let Some(f) = core.last_function {
        if core.last_fetch_delay.elapsed() >= core.key_delay {
            if core.last_fetch_rate.elapsed() >= core.key_rate {
                f(core);
                core.last_fetch_rate = std::time::Instant::now();
            }
        }
    }
}

fn page_down(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[buffer.main_cursor_index as usize].position;
    let len = buffer.lines.len() as u32;

    if buffer.offset.y + core.chars_per_row - 1 >= len {
        buffer.offset.y = len - core.chars_per_coloumn;
    } else {
        buffer.offset.y += core.chars_per_coloumn - 1;
    }

    if position.y < buffer.offset.y {
        position.y = buffer.offset.y;
    }

    position.x = 0;
    buffer::update_mode_line_right(core);
    buffer::update_chars(core);
    core.changed = true;
}

fn page_up(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[buffer.main_cursor_index as usize].position;

    if buffer.offset.y as i32 - core.chars_per_coloumn as i32 - 1 < 0 {
        buffer.offset.y = 0;
    } else {
        buffer.offset.y -= core.chars_per_coloumn - 1;
    }

    if position.y > buffer.offset.y + core.chars_per_coloumn - 1 {
        position.y = core.chars_per_coloumn + buffer.offset.y - 1;
    }

    position.x = 0;
    buffer::update_mode_line_right(core);
    buffer::update_chars(core);
    core.changed = true;
}

fn prev_line(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::prev_line(core, i);
    }

    buffer::update_mode_line_right(core);
    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn next_line(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::next_line(core, i);
    }

    buffer::update_mode_line_right(core);
    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn prev_char(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::prev_char(core, i);
    }

    buffer::update_mode_line_right(core);
    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn next_char(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::next_char(core, i);
    }

    buffer::update_mode_line_right(core);
    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn end_of_line(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::end_of_line(core, i);
    }

    buffer::update_mode_line_right(core);
    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn start_of_line(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::start_of_line(core, i);
    }

    buffer::update_mode_line_right(core);
    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn delete_char_at(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::delete_char_at(core, i);
    }

    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn delete_to_line_end(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::delete_to_line_end(core, i);
    }

    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn insert_char_at_current_position(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::insert_char_at(core, i);
    }

    buffer::update_mode_line_right(core);
    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn delete_to_command_end(core: &mut Core) {
    if core.buffers[core.main_buffer_index as usize].cursors[1].position.x < core.command.len() as u32 + 1 {
        core.command.drain(core.buffers[core.main_buffer_index as usize].cursors[1].position.x as usize - 1..);
        buffer::update_chars(core);
        core.changed = true;
    }
}

fn insert_new_line(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::insert_new_line(core, i);
    }

    buffer::update_mode_line_right(core);
    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn delete_prev_char(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    for i in 0..buffer.cursors.len() {
        buffer::delete_prev_char(core, i);
    }

    buffer::update_mode_line_right(core);
    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn insert_command_char(core: &mut Core) {
    let c = core.last_inserted_char;
    if c == b'\t' {
        let completions: Vec<Vec<u8>> = core.buffers.iter().map(|b| b.file_name.clone().unwrap()).collect();
        buffer::show_completion_box(core, completions);
    } else {
        core.command.insert(core.buffers[core.main_buffer_index as usize].cursors[1].position.x as usize - 1, c);
        core.buffers[core.main_buffer_index as usize].cursors[1].position.x += 1;
        buffer::update_chars(core);
    }

    core.changed = true;
}

fn delete_prev_command_char(core: &mut Core) {
    if let Some(_) = core.command.pop() {
        core.buffers[core.main_buffer_index as usize].cursors[1].position.x -= 1;
        buffer::update_chars(core);
        core.changed = true;
    }
}

fn prev_command_char(core: &mut Core) {
    if core.buffers[core.main_buffer_index as usize].cursors[1].position.x > 0 {
        core.buffers[core.main_buffer_index as usize].cursors[1].position.x -= 1;
        buffer::update_chars(core);
        core.changed = true;
    };
}

fn next_command_char(core: &mut Core) {
    if core.buffers[core.main_buffer_index as usize].cursors[1].position.x < core.command.len() as u32 + 1 {
        core.buffers[core.main_buffer_index as usize].cursors[1].position.x += 1;
        buffer::update_chars(core);
        core.changed = true;
    }
}

fn end_of_command_line(core: &mut Core) {
    if core.buffers[core.main_buffer_index as usize].cursors[1].position.x < core.command.len() as u32 + 1 {
        core.buffers[core.main_buffer_index as usize].cursors[1].position.x = core.command.len() as u32 + 1;
        buffer::update_chars(core);
        core.changed = true;
    }
}

fn start_of_command_line(core: &mut Core) {
    if core.buffers[core.main_buffer_index as usize].cursors[1].position.x > 1 {
        core.buffers[core.main_buffer_index as usize].cursors[1].position.x = 1;
        buffer::update_chars(core);
        core.changed = true;
    }
}

fn delete_command_char_at(core: &mut Core) {
    if core.command.len() > 0 {
        core.command.remove(core.buffers[core.main_buffer_index as usize].cursors[1].position.x as usize - 1);
        buffer::update_chars(core);
        core.changed = true;
    }
}

fn active_command_mode(core: &mut Core) {
    core.command_mode = true;
    core.buffers[core.main_buffer_index as usize].cursors.resize(
        1,
        buffer::Cursor {
            selection: None,
            position: buffer::Position {
                x: 0,
                y: 0,
            }
        }
    );

    let offset = core.buffers[core.main_buffer_index as usize].offset.y;
    core.buffers[core.main_buffer_index as usize].cursors.push(
        buffer::Cursor {
            selection: None,
            position: buffer::Position {
                x: 1,
                y: core.chars_per_coloumn + offset,
            }
        }
    );

    buffer::update_chars(core);
    core.changed = true;
}

fn deactive_command_mode(core: &mut Core) {
    core.command_mode = false;
    core.buffers[core.main_buffer_index as usize].cursors.pop();
}

fn execute_command(core: &mut Core) {
    buffer::execute_command(core);
    buffer::update_chars(core);
    core.changed = true;
}

fn save_buffer(core: &mut Core) {
    buffer::save_buffer(core);
}

const SHIFT_BIT: u8 = 0x01;
const CAPSLOCK_BIT: u8 = 0x02;
const CONTROL_BIT: u8 = 0x04;
const ALT_BIT: u8 = 0x08;

unsafe extern "C" fn keymap(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: i32, _: u32) {}
unsafe extern "C" fn enter(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: *mut wayland::wl_surface, _: *mut wayland::wl_array) {}
unsafe extern "C" fn leave(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: *mut wayland::wl_surface,) {}
unsafe extern "C" fn modifiers(data: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, depressed: u32, _: u32, locked: u32, _: u32) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    let pressed = depressed as u8 | locked as u8;

    core.control_modifier = pressed & CONTROL_BIT > 0;
    core.shift_modifier = pressed & (SHIFT_BIT | CAPSLOCK_BIT) > 0;
    core.alt_modifier = pressed & ALT_BIT > 0;
}

unsafe extern "C" fn repeat_info(data: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, rate: i32, delay: i32) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    core.key_delay = std::time::Duration::from_millis(delay as u64);
    core.key_rate = std::time::Duration::from_millis(rate as u64);
}

unsafe extern "C" fn key(data: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: u32, id: u32, state: u32) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    let code = id as u8;

    core.last_fetch_delay = std::time::Instant::now();
    core.last_function = None;

    if state == 1 {
        if let Ok(b) = try_ascci(code) {
            let c = if core.shift_modifier {
                b[1]
            } else {
                b[0]
            };

            if core.control_modifier {
                if core.command_mode {
                    match c {
                        b'b' => core.last_function = Some(prev_command_char),
                        b'f' => core.last_function = Some(next_command_char),
                        b'e' => core.last_function = Some(end_of_command_line),
                        b'a' => core.last_function = Some(start_of_command_line),
                        b'd' => core.last_function = Some(delete_command_char_at),
                        b'k' => core.last_function = Some(delete_to_command_end),
                        _ => {},
                    }
                } else {
                    match c {
                        b'p' => core.last_function = Some(prev_line),
                        b'n' => core.last_function = Some(next_line),
                        b'b' => core.last_function = Some(prev_char),
                        b'f' => core.last_function = Some(next_char),
                        b'e' => core.last_function = Some(end_of_line),
                        b'a' => core.last_function = Some(start_of_line),
                        b'd' => core.last_function = Some(delete_char_at),
                        b'k' => core.last_function = Some(delete_to_line_end),
                        b's' => core.last_function = Some(save_buffer),
                        b'v' => core.last_function = Some(page_down),
                        _ => {},
                    }
                }

            } else if core.alt_modifier {
                if core.command_mode {
                    return;
                }

                match c {
                    // b'f' => core.last_function = Some(next_word),
                    b'v' => core.last_function = Some(page_up),
                    b'x' => core.last_function = Some(active_command_mode),
                    _ => {},
                }
            } else {
                core.last_inserted_char = c;
                if core.command_mode {
                    core.last_function = Some(insert_command_char);
                } else {
                    core.last_function = Some(insert_char_at_current_position);
                }
            }
        } else if code == ENTER {
            if core.command_mode {
                deactive_command_mode(core);
                execute_command(core);
                return;
            }

            core.last_function = Some(insert_new_line);
        } else if code == BACKSPACE {
            if core.command_mode {
                core.last_function = Some(delete_prev_command_char);
            } else {
                core.last_function = Some(delete_prev_char);
            }
        } else if code == ESC {
            if core.command_mode {
                deactive_command_mode(core);
                core.command.clear();
                buffer::update_chars(core);
                core.changed = true;
            }
        }

        if let Some(f) = core.last_function {
            f(core)
        }
    }
}

unsafe extern "C" fn seat_name(_: *mut std::ffi::c_void, _: *mut wayland::wl_seat, _: *const i8) {}
unsafe extern "C" fn capabilities(data: *mut std::ffi::c_void, seat: *mut wayland::wl_seat, capability: u32) {
    let mut core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);

    if capability != 0 && wayland::WL_SEAT_CAPABILITY_KEYBOARD != 0 {
        core.keyboard = wayland::wl_proxy_marshal_flags(seat as *mut wayland::wl_proxy, wayland::WL_SEAT_GET_KEYBOARD, &wayland::wl_keyboard_interface, wayland::wl_proxy_get_version(seat as *mut wayland::wl_proxy), 0, std::ptr::null::<std::ffi::c_void>()) as *mut wayland::wl_keyboard;
        wayland::wl_proxy_add_listener(core.keyboard as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::wl_keyboard_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.keyboard_listener), std::mem::transmute::<&mut Core, *mut std::ffi::c_void>(&mut core));
    }
}

unsafe extern "C" fn shell_ping(_: *mut std::ffi::c_void, s: *mut wayland::xdg_wm_base, serial: u32) {
    wayland::wl_proxy_marshal_flags(s as *mut wayland::wl_proxy, wayland::XDG_WM_BASE_PONG, std::ptr::null(), wayland::wl_proxy_get_version(s as *mut wayland::wl_proxy), 0, serial);
}

unsafe extern "C" fn shell_surface_configure(_: *mut std::ffi::c_void, shell_surface: *mut wayland::xdg_surface, serial: u32) {
    wayland::wl_proxy_marshal_flags(shell_surface as *mut wayland::wl_proxy, wayland::XDG_SURFACE_ACK_CONFIGURE, std::ptr::null(), wayland::wl_proxy_get_version(shell_surface as *mut wayland::wl_proxy), 0, serial);
}

unsafe extern "C" fn toplevel_close(data: *mut std::ffi::c_void, _: *mut wayland::xdg_toplevel) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    core.running = false;
}

unsafe extern "C" fn toplevel_configure(data: *mut std::ffi::c_void, _: *mut wayland::xdg_toplevel, width: i32, height: i32, _: *mut wayland::wl_array) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);

    if width > 0 && height > 0 {
        core.width = width as u32;
        core.height = height as u32;
        core.changed = true;

        core.window_ratio = core.height as f32 / core.width as f32;
        core.chars_per_coloumn = (1.0 / core.scale) as u32 - 1;
        core.chars_per_row = (1.0 / (core.scale * core.x_ratio * core.window_ratio)) as u32;

        buffer::update_chars(core);
    }
}

unsafe extern "C" fn toplevel_configure_bounds(_: *mut std::ffi::c_void, _: *mut wayland::xdg_toplevel, _: i32, _: i32) {}
unsafe extern "C" fn toplevel_wm_capabilities(_: *mut std::ffi::c_void, _: *mut wayland::xdg_toplevel, _: *mut wayland::wl_array) {}
unsafe extern "C" fn remove_listener(_: *mut std::ffi::c_void, _: *mut wayland::wl_registry, _: u32) {}

unsafe extern "C" fn global_listener(data: *mut std::ffi::c_void, wl_registry: *mut wayland::wl_registry, name: u32, interface: *const std::ffi::c_char, _: u32) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    let interface_name = std::ffi::CStr::from_ptr(interface);

    if interface_name == std::ffi::CStr::from_ptr(wayland::wl_compositor_interface.name) {
        core.compositor = wayland::wl_proxy_marshal_flags(
            wl_registry as *mut wayland::wl_proxy,
            wayland::WL_REGISTRY_BIND,
            &wayland::wl_compositor_interface,
            4,
            0,
            name,
            wayland::wl_compositor_interface.name,
            4,
            std::ptr::null::<std::ffi::c_void>(),
        ) as *mut wayland::wl_compositor;
    } else if interface_name == std::ffi::CStr::from_ptr(wayland::xdg_wm_base_interface.name) {
        core.xdg_shell = wayland::wl_proxy_marshal_flags(
            wl_registry as *mut wayland::wl_proxy,
            wayland::WL_REGISTRY_BIND,
            &wayland::xdg_wm_base_interface,
            1,
            0,
            name,
            wayland::xdg_wm_base_interface.name,
            1,
            std::ptr::null::<std::ffi::c_void>(),
        ) as *mut wayland::xdg_wm_base;
    } else if interface_name == std::ffi::CStr::from_ptr(wayland::wl_seat_interface.name) {
        core.seat = wayland::wl_proxy_marshal_flags(
            wl_registry as *mut wayland::wl_proxy,
            wayland::WL_REGISTRY_BIND,
            &wayland::wl_seat_interface,
            1,
            0,
            name,
            wayland::wl_seat_interface.name,
            4,
            std::ptr::null::<std::ffi::c_void>(),
        ) as *mut wayland::wl_seat;
    }
}

pub fn init(
    name: &str,
    width: u32,
    height: u32,
    scale: f32,
    x_ratio: f32,
) -> Result<Box<Core>, WaylandError> {
    let window_ratio = height as f32 / width as f32;
    let chars_per_coloumn = (1.0 / scale) as u32 - 1;
    let chars_per_row = (1.0 / (scale * x_ratio * window_ratio)) as u32;

    let mut core = Box::new(Core {
        display: std::ptr::null_mut(),
        registry: std::ptr::null_mut(),
        surface: std::ptr::null_mut(),
        compositor: std::ptr::null_mut(),
        seat: std::ptr::null_mut(),
        keyboard: std::ptr::null_mut(),
        xdg_shell: std::ptr::null_mut(),
        xdg_surface: std::ptr::null_mut(),
        xdg_toplevel: std::ptr::null_mut(),
        extensions: [
            std::ffi::CString::new("VK_KHR_surface").unwrap().into_raw(),
            std::ffi::CString::new("VK_KHR_wayland_surface").unwrap().into_raw(),
        ],
        width,
        height,
        scale,
        x_ratio,
        window_ratio,
        running: true,
        changed: false,
        command_mode: false,
        command: Vec::new(),
        buffers: vec![buffer::buffer_from_file(chars_per_row, chars_per_coloumn, "src/renderer/wayland.rs").unwrap()],
        completion_lines: Vec::new(),
        main_buffer_index: 0,
        chars_per_row,
        chars_per_coloumn,
        key_rate: std::time::Duration::from_millis(20),
        key_delay: std::time::Duration::from_millis(200),
        last_function: None,
        last_inserted_char: b' ',
        last_fetch_delay: std::time::Instant::now(),
        last_fetch_rate: std::time::Instant::now(),
        alt_modifier: false,
        shift_modifier: false,
        control_modifier: false,
        registry_listener: wayland::wl_registry_listener {
            global: Some(global_listener),
            global_remove: Some(remove_listener),
        },
        seat_listener: wayland::wl_seat_listener {
            name: Some(seat_name),
            capabilities: Some(capabilities),
        },
        keyboard_listener: wayland::wl_keyboard_listener {
            keymap: Some(keymap),
            enter: Some(enter),
            leave: Some(leave),
            key: Some(key),
            modifiers: Some(modifiers),
            repeat_info: Some(repeat_info),
        },
        shell_listener: wayland::xdg_wm_base_listener {
            ping: Some(shell_ping)
        },
        shell_surface_listener: wayland::xdg_surface_listener {
            configure: Some(shell_surface_configure)
        },
        toplevel_listener: wayland::xdg_toplevel_listener {
            configure: Some(toplevel_configure),
            configure_bounds: Some(toplevel_configure_bounds),
            close: Some(toplevel_close),
            wm_capabilities: Some(toplevel_wm_capabilities),
        },
    });

    core.display = unsafe { wayland::wl_display_connect(std::ptr::null()) } as *mut wayland::wl_display;
    core.registry = unsafe { wayland::wl_proxy_marshal_flags(core.display as *mut wayland::wl_proxy, wayland::WL_DISPLAY_GET_REGISTRY, &wayland::wl_registry_interface, wayland::wl_proxy_get_version(core.display as *mut wayland::wl_proxy), 0) } as *mut wayland::wl_registry;

    let data = unsafe { std::mem::transmute::<&mut Core, *mut std::ffi::c_void>(&mut core) };
    let listener = unsafe { std::mem::transmute::<*mut wayland::wl_registry_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.registry_listener) };
    if 0 != unsafe { wayland::wl_proxy_add_listener(core.registry as *mut wayland::wl_proxy, listener, data) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    unsafe { wayland::wl_display_roundtrip(core.display) };

    core.surface = unsafe { wayland::wl_proxy_marshal_flags(core.compositor as *mut wayland::wl_proxy, wayland::WL_COMPOSITOR_CREATE_SURFACE, &wayland::wl_surface_interface, wayland::wl_proxy_get_version(core.compositor as *mut wayland::wl_proxy), 0, std::ptr::null::<std::ffi::c_void>()) } as *mut wayland::wl_surface;

    if 0 != unsafe { wayland::wl_proxy_add_listener(core.xdg_shell as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::xdg_wm_base_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.shell_listener), data) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    core.xdg_surface = unsafe { wayland::wl_proxy_marshal_flags(core.xdg_shell as *mut wayland::wl_proxy, wayland::XDG_WM_BASE_GET_XDG_SURFACE, &wayland::xdg_surface_interface, wayland::wl_proxy_get_version(core.xdg_shell as *mut wayland::wl_proxy), 0, data, core.surface) } as *mut wayland::xdg_surface;
    if 0 != unsafe { wayland::wl_proxy_add_listener(core.xdg_surface as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::xdg_surface_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.shell_surface_listener), data) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    core.xdg_toplevel = unsafe { wayland::wl_proxy_marshal_flags(core.xdg_surface as *mut wayland::wl_proxy, wayland::XDG_SURFACE_GET_TOPLEVEL, &wayland::xdg_toplevel_interface, wayland::wl_proxy_get_version(core.xdg_surface as *mut wayland::wl_proxy), 0, std::ptr::null_mut::<std::ffi::c_void>()) } as *mut wayland::xdg_toplevel;
    if 0 != unsafe { wayland::wl_proxy_add_listener(core.xdg_toplevel as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::xdg_toplevel_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.toplevel_listener), std::mem::transmute::<*mut Core, *mut std::ffi::c_void>(&mut *core)) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    if 0 != unsafe { wayland::wl_proxy_add_listener(core.seat as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::wl_seat_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.seat_listener), std::mem::transmute::<*mut Core, *mut std::ffi::c_void>(&mut *core)) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    let title = std::ffi::CString::new(name).unwrap();
    unsafe { wayland::wl_proxy_marshal_flags(core.xdg_toplevel as *mut wayland::wl_proxy, wayland::XDG_TOPLEVEL_SET_TITLE, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_toplevel as *mut wayland::wl_proxy), 0, title.as_ptr()) };
    unsafe { wayland::wl_proxy_marshal_flags(core.xdg_toplevel as *mut wayland::wl_proxy, wayland::XDG_TOPLEVEL_SET_APP_ID, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_toplevel as *mut wayland::wl_proxy), 0, title.as_ptr()) };
    unsafe { wayland::wl_proxy_marshal_flags(core.surface as *mut wayland::wl_proxy, wayland::WL_SURFACE_COMMIT, std::ptr::null(), wayland::wl_proxy_get_version(core.surface as *mut wayland::wl_proxy), 0) };
    unsafe { wayland::wl_display_roundtrip(core.display) };

    Ok(core)
}

pub fn shutdown(core: &Core) {
    unsafe {
        wayland::wl_proxy_marshal_flags(core.seat as *mut wayland::wl_proxy, wayland::WL_SEAT_RELEASE, std::ptr::null(), wayland::wl_proxy_get_version(core.seat as *mut wayland::wl_proxy), wayland::WL_MARSHAL_FLAG_DESTROY);
        wayland::wl_proxy_marshal_flags(core.xdg_toplevel as *mut wayland::wl_proxy, wayland::XDG_TOPLEVEL_DESTROY, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_toplevel as *mut wayland::wl_proxy), wayland::WL_MARSHAL_FLAG_DESTROY);
        wayland::wl_proxy_marshal_flags(core.xdg_surface as *mut wayland::wl_proxy, wayland::XDG_SURFACE_DESTROY, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_surface as *mut wayland::wl_proxy), wayland::WL_MARSHAL_FLAG_DESTROY);
        wayland::wl_proxy_marshal_flags(core.xdg_shell  as *mut wayland::wl_proxy, wayland::XDG_WM_BASE_DESTROY, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_shell as *mut wayland::wl_proxy), wayland::WL_MARSHAL_FLAG_DESTROY);
        wayland::wl_proxy_destroy(core.surface as *mut wayland::wl_proxy);
        wayland::wl_proxy_destroy(core.compositor as *mut wayland::wl_proxy);
        wayland::wl_proxy_destroy(core.registry as *mut wayland::wl_proxy);
        wayland::wl_display_disconnect(core.display);
    };
}

const ENTER: u8 = 28;
const BACKSPACE: u8 = 14;
const ESC: u8 = 1;

fn try_ascci(u: u8) -> Result<[u8; 2], WaylandError> {
    match u {
        2 => Ok([b'1', b'!']),
        3 => Ok([b'2', b'@']),
        4 => Ok([b'3', b'#']),
        5 => Ok([b'4', b'$']),
        6 => Ok([b'5', b'%']),
        7 => Ok([b'6', b'^']),
        8 => Ok([b'7', b'&']),
        9 => Ok([b'8', b'*']),
        10 => Ok([b'9', b'(']),
        11 => Ok([b'0', b')']),
        12 => Ok([b'-', b'_']),
        13 => Ok([b'=', b'+']),
        15 => Ok([b'\t', b'\t']),

        16 => Ok([b'q', b'Q']),
        17 => Ok([b'w', b'W']),
        18 => Ok([b'e', b'E']),
        19 => Ok([b'r', b'R']),
        20 => Ok([b't', b'T']),
        21 => Ok([b'y', b'Y']),
        22 => Ok([b'u', b'U']),
        23 => Ok([b'i', b'I']),
        24 => Ok([b'o', b'O']),
        25 => Ok([b'p', b'P']),

        26 => Ok([b'[', b'{']),
        27 => Ok([b']', b'}']),

        30 => Ok([b'a', b'A']),
        31 => Ok([b's', b'S']),
        32 => Ok([b'd', b'D']),
        33 => Ok([b'f', b'F']),
        34 => Ok([b'g', b'G']),
        35 => Ok([b'h', b'H']),
        36 => Ok([b'j', b'J']),
        37 => Ok([b'k', b'K']),
        38 => Ok([b'l', b'L']),

        39 => Ok([b';', b':']),
        40 => Ok([b'\'', b'"']),

        43 => Ok([b'\\', b'|']),

        44 => Ok([b'z', b'Z']),
        45 => Ok([b'x', b'X']),
        46 => Ok([b'c', b'C']),
        47 => Ok([b'v', b'V']),
        48 => Ok([b'b', b'B']),
        49 => Ok([b'n', b'N']),
        50 => Ok([b'm', b'M']),

        51 => Ok([b',', b'<']),
        52 => Ok([b'.', b'>']),
        53 => Ok([b'/', b'?']),

        57 => Ok([b' ', b' ']),

        _ => Err(WaylandError::NotAscci),
    }
}
