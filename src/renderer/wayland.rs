use crate::binding::wayland;

pub struct Core {
    pub display: *mut wayland::wl_display,
    pub surface: *mut wayland::wl_surface,
    pub extensions: [*const std::ffi::c_char; 2],
    pub running: bool,
    pub width: u32,
    pub height: u32,
    pub keys_pressed: [u8; 4],
    pub keys_count: u8,

    registry: *mut wayland::wl_registry,
    compositor: *mut wayland::wl_compositor,
    seat: *mut wayland::wl_seat,
    seat_listener: wayland::wl_seat_listener,
    keyboard: *mut wayland::wl_keyboard,
    keyboard_listener: wayland::wl_keyboard_listener,
    xdg_shell: *mut wayland::xdg_wm_base,
    xdg_surface: *mut wayland::xdg_surface,
    xdg_toplevel: *mut wayland::xdg_toplevel,
    registry_listener: wayland::wl_registry_listener,
    shell_listener: wayland::xdg_wm_base_listener,
    shell_surface_listener: wayland::xdg_surface_listener,
    toplevel_listener: wayland::xdg_toplevel_listener,
}

#[derive(Debug)]
pub enum WaylandError {
    CouldNotAddListener,
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
        if (core.keys_count as usize) < core.keys_pressed.len() {
            core.keys_pressed[core.keys_count as usize] = code;
            core.keys_count += 1;
        }
    } else if state == 0 {
        for i in 0..core.keys_count {
            if core.keys_pressed[i as usize] == code {
                core.keys_pressed[i as usize] = core.keys_pressed[core.keys_count as usize - 1];
                core.keys_count -= 1;
                break;
            }
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

pub fn init(name: &str, width: u32, height: u32) -> Result<Box<Core>, WaylandError> {
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
        running: true,
        keys_pressed: [0; 4],
        keys_count: 0,
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

pub fn update(core: &Core) {
    unsafe {
        wayland::wl_proxy_marshal_flags(core.surface as *mut wayland::wl_proxy, wayland::WL_SURFACE_COMMIT, std::ptr::null(), wayland::wl_proxy_get_version(core.surface as *mut wayland::wl_proxy), 0);
        wayland::wl_display_roundtrip(core.display);
    };
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
