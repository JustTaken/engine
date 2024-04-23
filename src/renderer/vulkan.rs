use crate::binding::dl;
use crate::binding::vulkan;
use crate::binding::wayland;

use std::mem::MaybeUninit;

pub struct VulkanObject {
    instance: *mut vulkan::Instance,
    surface: *mut vulkan::Surface,
}

#[derive(Debug)]
pub enum LoadError {
    NoLibVulkan,
    NoFunction,
    InstanceFailed,
    SurfaceCreate,
}

// macro_rules! load_function {
//     ($lib:ident, $name:ident) => {
//         unsafe {
//             let string = std::ffi::CString::new(&stringify!($name)[4..]).unwrap();
//             let pointer = dl::dlsym($lib, string.as_ptr());
//             let func = std::mem::transmute::<*const std::ffi::c_void, vulkan::$name>(pointer);

//             if let Some(f) = func {
//                 Ok(f)
//             } else {
//                 Err(LoadError::NoFunction)
//             }
//         }
//     }
// }

fn loader_function(library: *const std::ffi::c_void) -> vulkan::PFN_vkGetInstanceProcAddr {
    unsafe {
        let string = std::ffi::CString::new("vkGetInstanceProcAddr").unwrap();
        let pointer = dl::dlsym(library, string.as_ptr());
        std::mem::transmute::<*const std::ffi::c_void, vulkan::PFN_vkGetInstanceProcAddr>(pointer)
    }
}

fn load_library(library_name: &str) -> Result<*const std::ffi::c_void, LoadError>  {
    let vk_name = std::ffi::CString::new(library_name).unwrap();
    let library = unsafe { dl::dlopen(vk_name.as_ptr(), 1) };

    if library.is_null() {
        Err(LoadError::NoLibVulkan)
    } else {
        Ok(library)
    }
}

macro_rules! instance_function {
    ($proc:ident, $instance:ident, $name:ident) => {
        unsafe {
            let string = std::ffi::CString::new(&stringify!($name)[4..]).unwrap();
            let pointer = $proc($instance, string.as_ptr());
            let func = std::mem::transmute::<vulkan::PFN_vkVoidFunction, vulkan::$name>(pointer);

            if let Some(f) = func {
                Ok(f)
            } else {
                Err(LoadError::NoFunction)
            }
        }
    }
}

fn make_version(major: u8, minor: u8) -> u32 {
    let lower = (minor as u32) << 12;
    let higher = (major as u32) << 22;
    higher | lower
}

pub fn init(display: *mut wayland::wl_display, surface: *mut wayland::wl_surface, extensions: &[*const std::ffi::c_char]) -> Result<VulkanObject, LoadError> {
    let library = load_library("libvulkan.so")?;

    let api_name = std::ffi::CString::new("Hello triangle").unwrap();
    let engine_name = std::ffi::CString::new("Hello triangle").unwrap();

    let app_info = vulkan::ApplicationInfo {
        sType: vulkan::STRUCTURE_TYPE_APPLICATION_INFO,
        pApplicationName: api_name.as_ptr(),
        pEngineName: engine_name.as_ptr(),
        applicationVersion: make_version(1, 3),
        engineVersion: make_version(1, 3),
        apiVersion: make_version(1, 3),
        pNext: std::ptr::null()
    };

    let create_info = vulkan::InstanceCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pApplicationInfo: &app_info as *const vulkan::ApplicationInfo,
        flags: 0,
        enabledLayerCount: 0,
        ppEnabledLayerNames: std::ptr::null(),
        enabledExtensionCount: extensions.len() as u32,
        ppEnabledExtensionNames: extensions.as_ptr() as *const *const i8,
        pNext: std::ptr::null(),
    };

    let vk_get_instance_proc_addr = loader_function(library).ok_or(LoadError::NoFunction)?;
    let null = std::ptr::null_mut();
    let vk_create_instance = instance_function!(vk_get_instance_proc_addr, null, PFN_vkCreateInstance)?;

    let mut ptr_instance: *mut vulkan::Instance = std::ptr::null_mut();

    if 0 != unsafe { vk_create_instance(&create_info as *const vulkan::InstanceCreateInfo, std::ptr::null(), &mut ptr_instance as *mut *mut vulkan::Instance) } {
        return Err(LoadError::InstanceFailed);
    }

    let vk_create_wayland_surface_khr = instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkCreateWaylandSurfaceKHR)?;

    let surface_info = vulkan::WaylandSurfaceCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
        display: display,
        surface: surface,
        flags: 0,
        pNext: std::ptr::null(),
    };
    let mut ptr_surface: *mut vulkan::Surface = std::ptr::null_mut();

    if 0 != unsafe { vk_create_wayland_surface_khr(ptr_instance, &surface_info as *const vulkan::WaylandSurfaceCreateInfo, std::ptr::null(), &mut ptr_surface as *mut *mut vulkan::Surface) } {
        return Err(LoadError::SurfaceCreate);
    }

    Ok(VulkanObject {
        instance: ptr_instance,
        surface: ptr_surface
    })
}
