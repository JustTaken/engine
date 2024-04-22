use crate::binding::dl;
use crate::binding::vulkan;

#[derive(Debug)]
pub enum LoadError {
    NoLibVulkan,
    NoFunction,
    InstanceFailed,
}

macro_rules! load_function {
    ($lib:ident, $name:ident) => {
        unsafe {
            let string = std::ffi::CString::new(&stringify!($name)[4..]).unwrap();
            let pointer = dl::dlsym($lib, string.as_ptr());
            let func = std::mem::transmute::<*const std::ffi::c_void, vulkan::$name>(pointer);

            if let Some(f) = func {
                Ok(f)
            } else {
                Err(LoadError::NoFunction)
            }
        }
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

pub fn test() -> Result<(), LoadError> {
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
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: std::ptr::null(),
        pNext: std::ptr::null(),
    };

    let null = std::ptr::null_mut();
    let vk_get_instance_proc_addr = load_function!(library, PFN_vkGetInstanceProcAddr)?;
    let vk_create_instance = instance_function!(vk_get_instance_proc_addr, null, PFN_vkCreateInstance)?;

    let instance: *mut vulkan::Instance = std::ptr::null_mut();

    if 0 != unsafe { vk_create_instance(&create_info as *const vulkan::InstanceCreateInfo, std::ptr::null(), instance) } {
        return Err(LoadError::InstanceFailed);
    }

    Ok(())
}
