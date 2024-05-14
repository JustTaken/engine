use crate::binding::wayland;

pub struct Core {
    pub display: *mut wayland::wl_display,
    pub surface: *mut wayland::wl_surface,

    pub extensions: [*const i8; 2],
    pub running: bool,
    pub width: u32,
    pub height: u32,

    pub unique_chars: UniqueChars,
    pub cursor: Cursor,
    pub changed: bool,

    scale: f32,
    chars_per_row: usize,
    chars_per_coloum: usize,

    shift_modifier: bool,
    control_modifier: bool,

    lines: Vec<Vec<u8>>,

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

pub struct Cursor {
    pub xpos: u8,
    pub ypos: u8,
    x_offset: u8,
    y_offset: u8,
}

pub struct UniqueChars {
    pub changed: bool,
    pub offset: [u8; 95],
    pub positions: Vec<Vec<[u8; 2]>>,
}

fn get_slice(line: &[u8], offset: u8, size: u8) -> &[u8] {
    let len = line.len();

    if len < offset as usize {
        &[]
    } else if len < size as usize {
        &line[offset as usize..len]
    } else {
        &line[offset as usize..size as usize]
    }
}

fn get_this_line_or_max(lines: &Vec<Vec<u8>>, i: usize) -> usize {
    let len = lines.len();

    if len < i {
        len
    } else {
        i
    }
}

fn remove_char(chars: &mut UniqueChars, c: u8, position: [u8; 2]) {
    let c = c as usize - 32;
    let offset = chars.offset[c] as usize;

    for i in 0..chars.positions[offset].len() {
        if chars.positions[offset][i][0] == position[0] && chars.positions[offset][i][1] == position[1] {
            chars.positions[offset].remove(i);
            break;
        }
    }

    chars.changed = true;
}

fn insert_char(chars: &mut UniqueChars, c: u8, position: [u8; 2]) {
    let c = c as usize - 32;

    if chars.offset[c] == 255 {
        chars.offset[c] = chars.positions.len() as u8;
        chars.positions.push(Vec::with_capacity(50));
    }

    chars.positions[chars.offset[c] as usize].push(position);

    chars.changed = true;
}

fn update_chars(core: &mut Core) {
    let max_line = get_this_line_or_max(&core.lines, core.cursor.y_offset as usize + core.chars_per_coloum);
    let lines = &core.lines[core.cursor.y_offset as usize..max_line];

    for i in 0..core.unique_chars.positions.len() {
        core.unique_chars.positions[i].clear();
    }

    for (i, line) in lines.iter().enumerate() {
        for (j, u) in get_slice(line, core.cursor.x_offset, core.chars_per_row as u8 + core.cursor.x_offset).iter().enumerate() {
            let c = *u as usize - 32;

            if core.unique_chars.offset[c] == 255 {
                core.unique_chars.offset[c] = core.unique_chars.positions.len() as u8;
                core.unique_chars.positions.push(Vec::with_capacity(50));
            }

            core.unique_chars.positions[core.unique_chars.offset[c] as usize].push([j as u8, i as u8]);
        }
    }

    core.unique_chars.changed = true;
}

fn delete_prev_char(core: &mut Core) {
    if core.cursor.xpos + core.cursor.x_offset == 0 {
        if core.cursor.ypos > 0 {
            core.cursor.ypos -= 1;
        } else if core.cursor.y_offset > 0 {
            core.cursor.y_offset -= 1;
        } else {
            return;
        }

        let len = core.lines[(core.cursor.ypos + core.cursor.y_offset) as usize].len();

        core.cursor.xpos = (len % core.chars_per_row) as u8;
        core.cursor.x_offset = len as u8 - core.cursor.xpos;
        update_chars(core);
    } else if core.cursor.xpos == 0 {
        core.cursor.x_offset -= 1;

        core.lines[(core.cursor.ypos + core.cursor.y_offset) as usize].remove(core.cursor.x_offset as usize);
    } else {
        core.cursor.xpos -= 1;

        let c = core.lines[(core.cursor.ypos + core.cursor.y_offset) as usize].remove((core.cursor.xpos + core.cursor.x_offset) as usize);
        remove_char(&mut core.unique_chars, c, [core.cursor.xpos, core.cursor.ypos]);
    }

    core.changed = true;
}

fn next_char(core: &mut Core) {
    let line = core.cursor.ypos + core.cursor.y_offset;
    let xpos = core.cursor.xpos + core.cursor.x_offset;

    if xpos as usize >= core.lines[line as usize].len() {
        if core.lines.len() <= line as usize + 1 {
            return;
        }

        if core.cursor.ypos as usize + 1 >= core.chars_per_coloum {
            core.cursor.y_offset += 1;
            core.cursor.xpos = 0;
            core.cursor.x_offset = 0;
        } else {
            core.cursor.xpos = 0;
            core.cursor.x_offset = 0;
            core.cursor.ypos += 1;
        }
    } else if core.cursor.xpos < core.chars_per_row as u8 {
        core.cursor.xpos += 1;
    } else {
        core.cursor.x_offset += 1;
    }

    core.changed = true;
    update_chars(core);
}

fn prev_char(core: &mut Core) {
    if core.cursor.xpos + core.cursor.x_offset == 0 {
        if core.cursor.ypos > 0 {
            core.cursor.ypos -= 1;
        } else if core.cursor.y_offset > 0 {
            core.cursor.y_offset -= 1;
        } else {
            return;
        }

        let len = core.lines[(core.cursor.ypos + core.cursor.y_offset) as usize].len();

        core.cursor.xpos = (len % core.chars_per_row) as u8;
        core.cursor.x_offset = len as u8 - core.cursor.xpos;

    } else if core.cursor.xpos == 0 {
        core.cursor.x_offset -= 1;
    } else {
        core.cursor.xpos -= 1;
    }

    core.changed = true;
    update_chars(core);
}

fn prev_line(core: &mut Core) {
    if core.cursor.ypos + core.cursor.y_offset > 0 {
        if core.cursor.ypos == 0 {
            core.cursor.y_offset -= 1;
        } else {
            core.cursor.ypos -= 1;
        }

        let xpos = core.cursor.xpos + core.cursor.x_offset;
        let ypos = core.cursor.ypos + core.cursor.y_offset;
        let xpos = this_pos_or_max(&core.lines[ypos as usize], xpos as usize);

        core.cursor.xpos = (xpos % core.chars_per_row) as u8;
        core.cursor.x_offset = (xpos - core.cursor.xpos as usize) as u8;

        core.changed = true;
        update_chars(core);
    }
}

fn this_pos_or_max(line: &[u8], x: usize) -> usize {
    let len = line.len();

    if len < x {
        len
    } else {
        x
    }
}

fn next_line(core: &mut Core) {
    let ypos = (core.cursor.ypos + core.cursor.y_offset + 1) as usize;
    if ypos < core.lines.len() {
        if core.cursor.ypos + 1 >= core.chars_per_coloum as u8 {
            core.cursor.y_offset += 1;
        } else {
            core.cursor.ypos += 1;
        }

        let xpos = core.cursor.xpos + core.cursor.x_offset;
        let xpos = this_pos_or_max(&core.lines[ypos], xpos as usize);

        core.cursor.xpos = (xpos % core.chars_per_row) as u8;
        core.cursor.x_offset = (xpos - core.cursor.xpos as usize) as u8;

        core.changed = true;
        update_chars(core);
    }
}

fn insert_new_line(core: &mut Core) {
    if core.cursor.ypos + 1 >= core.chars_per_coloum as u8 {
        core.cursor.y_offset += 1;
    } else {
        core.cursor.ypos += 1;
    }


    if core.lines.len() <= (core.cursor.ypos + core.cursor.y_offset) as usize {
        core.lines.push(Vec::with_capacity(255));
    }

    core.cursor.xpos = 0;
    core.cursor.x_offset = 0;

    core.changed = true;
    update_chars(core);
}

fn insert_char_at_current_position(core: &mut Core, c: u8) {
    core.lines[(core.cursor.ypos + core.cursor.y_offset) as usize].insert((core.cursor.xpos + core.cursor.x_offset) as usize, c);

    if core.cursor.xpos >= core.chars_per_row as u8 {
        core.cursor.x_offset += 1;
        update_chars(core);
    } else {
        insert_char(&mut core.unique_chars, c, [core.cursor.xpos, core.cursor.ypos]);
        core.cursor.xpos += 1;
    }

    core.changed = true;
}

pub fn set_unchanged(core: &mut Core) {
    core.changed = false;
    core.unique_chars.changed = false;
}

pub fn update(core: &Core) {
    unsafe {
        wayland::wl_proxy_marshal_flags(core.surface as *mut wayland::wl_proxy, wayland::WL_SURFACE_COMMIT, std::ptr::null(), wayland::wl_proxy_get_version(core.surface as *mut wayland::wl_proxy), 0);
        wayland::wl_display_roundtrip(core.display);
    };
}

unsafe extern "C" fn keymap(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: i32, _: u32) {}
unsafe extern "C" fn enter(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: *mut wayland::wl_surface, _: *mut wayland::wl_array) {}
unsafe extern "C" fn leave(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: *mut wayland::wl_surface,) {}
unsafe extern "C" fn modifiers(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: u32, _: u32, _: u32, _: u32) {}
unsafe extern "C" fn repeat_info(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: i32, _: i32) {}

unsafe extern "C" fn key(data: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: u32, id: u32, state: u32) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    let code = id as u8;

    if state == 1 {
        // let start = std::time::Instant::now();

        if let Ok(b) = try_ascci(code) {
            let c = if core.shift_modifier {
                b[1]
            } else {
                b[0]
            };

            if core.control_modifier {
                if c == b'p' {
                    prev_line(core);
                } else if c == b'n' {
                    next_line(core);
                } else if c == b'b' {
                    prev_char(core);
                } else if c == b'f' {
                    next_char(core);
                }
            } else {
                insert_char_at_current_position(core, c);
            }
        } else if code == ENTER {
            insert_new_line(core);
        } else if code == BACKSPACE {
            delete_prev_char(core);
        } else if code == SHIFT {
            core.shift_modifier = true;
        } else if code == CONTROL {
            core.control_modifier = true;
        }

        // let elapsed_time = start.elapsed();
        // println!("buffer modification run in {} ns", elapsed_time.as_nanos());
    } else if state == 0 {
        if code == SHIFT {
            core.shift_modifier = false;
        } else if code == CONTROL {
            core.control_modifier = false;
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

        core.chars_per_coloum = (2.0 / core.scale) as usize;
        core.chars_per_row = (2.0 / core.scale * core.width as f32 / core.height as f32) as usize - 1;

        // if chars_per_row != core.chars_per_row {
        //     core.chars_per_row = chars_per_row;
        //     for i in core.cursor_position_offset[1]..core.content.len() {
        //         core.lines_with_offset[i - core.cursor_position_offset[1]] = core.content[i].get_slice(core.cursor_position_offset[0], core.cursor_position_offset[0] + chars_per_row);
        //     }
        // }

        // if chars_per_coloum > core.lines_with_offset.len() {
        //     for i in chars_per_coloum..core.lines_with_offset.len() {
        //         core.lines_with_offset[i] = core.content[core.cursor_position_offset[1] + i].get_slice(core.cursor_position_offset[0], core.cursor_position_offset[0] + chars_per_row);
        //     }
        // } else {
        //     core.lines_with_offset.resize(chars_per_coloum, &[]);
        // }
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
            1,
            std::ptr::null::<std::ffi::c_void>(),
        ) as *mut wayland::wl_seat;
    }
}

pub fn init(
    name: &str,
    width: u32,
    height: u32,
    scale: f32,
) -> Result<Box<Core>, WaylandError> {
    let chars_per_coloum = (2.0 / scale) as usize;
    let chars_per_row = (2.0 / scale * width as f32 / height as f32) as usize;

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
        running: true,
        changed: false,
        chars_per_row,
        lines: vec![Vec::with_capacity(50)],
        chars_per_coloum,
        unique_chars: UniqueChars {
            changed: true,
            offset: [255; 95],
            positions: Vec::with_capacity(95),
        },
        cursor: Cursor {
            xpos: 0,
            ypos: 0,
            x_offset: 0,
            y_offset: 0,
        },
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
const SHIFT: u8 = 42;
const CONTROL: u8 = 58;
const BACKSPACE: u8 = 14;

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
