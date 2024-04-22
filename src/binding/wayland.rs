pub const WL_DISPLAY_GET_REGISTRY: u32 = 1;
pub const WL_REGISTRY_BIND: u32 = 0;
pub const WL_COMPOSITOR_CREATE_SURFACE: u32 = 0;
pub const WL_SURFACE_COMMIT: u32 = 6;
pub const XDG_WM_BASE_GET_XDG_SURFACE: u32 = 2;
pub const XDG_WM_BASE_PONG: u32 = 3;
pub const XDG_SURFACE_GET_TOPLEVEL: u32 = 1;
pub const XDG_SURFACE_ACK_CONFIGURE: u32 = 4;
pub const XDG_TOPLEVEL_SET_TITLE: u32 = 2;
pub const XDG_TOPLEVEL_SET_APP_ID: u32 = 3;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_message {
    pub name: *const ::std::os::raw::c_char,
    pub signature: *const ::std::os::raw::c_char,
    pub types: *mut *const wl_interface,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_interface {
    pub name: *const ::std::os::raw::c_char,
    pub version: ::std::os::raw::c_int,
    pub method_count: ::std::os::raw::c_int,
    pub methods: *const wl_message,
    pub event_count: ::std::os::raw::c_int,
    pub events: *const wl_message,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_array {
    pub size: usize,
    pub alloc: usize,
    pub data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_proxy {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_display {
    _unused: [u8; 0],
}
extern "C" {
    pub fn wl_proxy_marshal_flags(
        proxy: *mut wl_proxy,
        opcode: u32,
        interface: *const wl_interface,
        version: u32,
        flags: u32,
        ...
    ) -> *mut wl_proxy;
}
extern "C" {
    pub fn wl_proxy_add_listener(
        proxy: *mut wl_proxy,
        implementation: *mut ::std::option::Option<unsafe extern "C" fn()>,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wl_proxy_get_version(proxy: *mut wl_proxy) -> u32;
}
extern "C" {
    pub fn wl_display_connect(name: *const ::std::os::raw::c_char) -> *mut wl_display;
}
extern "C" {
    pub fn wl_display_roundtrip(display: *mut wl_display) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_compositor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_registry {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_seat {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_surface {
    _unused: [u8; 0],
}
extern "C" {
    pub static wl_registry_interface: wl_interface;
}
extern "C" {
    pub static wl_compositor_interface: wl_interface;
}
extern "C" {
    pub static wl_surface_interface: wl_interface;
}
extern "C" {
    pub static wl_seat_interface: wl_interface;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_registry_listener {
    pub global: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_registry: *mut wl_registry,
            name: u32,
            interface: *const ::std::os::raw::c_char,
            version: u32,
        ),
    >,
    pub global_remove: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_registry: *mut wl_registry,
            name: u32,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdg_surface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdg_toplevel {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdg_wm_base {
    _unused: [u8; 0],
}
extern "C" {
    pub static xdg_wm_base_interface: wl_interface;
}
extern "C" {
    pub static xdg_surface_interface: wl_interface;
}
extern "C" {
    pub static xdg_toplevel_interface: wl_interface;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdg_wm_base_listener {
    pub ping: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            xdg_wm_base: *mut xdg_wm_base,
            serial: u32,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdg_surface_listener {
    pub configure: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            xdg_surface: *mut xdg_surface,
            serial: u32,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdg_toplevel_listener {
    pub configure: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            xdg_toplevel: *mut xdg_toplevel,
            width: i32,
            height: i32,
            states: *mut wl_array,
        ),
    >,
    pub close: ::std::option::Option<
        unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, xdg_toplevel: *mut xdg_toplevel),
    >,
    pub configure_bounds: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            xdg_toplevel: *mut xdg_toplevel,
            width: i32,
            height: i32,
        ),
    >,
    pub wm_capabilities: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            xdg_toplevel: *mut xdg_toplevel,
            capabilities: *mut wl_array,
        ),
    >,
}
