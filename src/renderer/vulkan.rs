#![allow(non_camel_case_types, non_snake_case)]

use crate::binding::dl;
use crate::binding::vulkan;
use crate::binding::wayland;

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

pub struct InstanceDispatch {
    handle: *mut vulkan::Instance,
    vkDestroyInstance: vulkan::vkDestroyInstance,
    vkDestroySurfaceKHR: vulkan::vkDestroySurfaceKHR,
    vkCreateWaylandSurfaceKHR: vulkan::vkCreateWaylandSurfaceKHR,
    vkEnumeratePhysicalDevices: vulkan::vkEnumeratePhsysicalDevices,
    vkEnumerateDeviceExtensionProperties: vulkan::vkEnumerateDeviceExtensionProperties,
    vkGetPhysicalDeviceSurfaceFormatsKHR: vulkan::vkGetPhysicalDeviceSurfaceFormatsKHR,
    vkGetPhysicalDeviceSurfacePresentModesKHR: vulkan::vkGetPhysicalDeviceSurfacePresentModesKHR,
    vkGetPhysicalDeviceQueueFamilyProperties: vulkan::vkGetPhysicalDeviceQueueFamilyProperties,
    vkGetPhysicalDeviceMemoryProperties: vulkan::vkGetPhysicalDeviceMemoryProperties,
    vkGetPhysicalDeviceSurfaceSupportKHR: vulkan::vkGetPhysicalDeviceSurfaceSupportKHR,
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR: vulkan::vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
}

#[derive(Debug)]
pub enum LoadError {
    NoLibVulkan,
    NoFunction,
    InstanceFailed,
    SurfaceCreate,
    NoExtension,
    NoSuitableDevice,
}

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

fn make_version(major: u8, minor: u8) -> u32 {
    let lower = (minor as u32) << 12;
    let higher = (major as u32) << 22;
    higher | lower
}

pub fn dispatch(extensions: &[*const std::ffi::c_char]) -> Result<InstanceDispatch, LoadError> {
    let library = load_library("libvulkan.so")?;

    let api_name: *const std::ffi::c_char = b"Hello triangle\0".as_ptr().cast();

    let app_info = vulkan::ApplicationInfo {
        sType: vulkan::STRUCTURE_TYPE_APPLICATION_INFO,
        pApplicationName: api_name,
        pEngineName: api_name,
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

    for string in extensions.into_iter() {
        let _ = unsafe { std::ffi::CString::from_raw(*string as *mut i8) }; // Deinitialize previous strings allocated to create the VkInstance
    }

    Ok(InstanceDispatch {
        handle: ptr_instance,
        vkDestroyInstance: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkDestroyInstance)?,
        vkDestroySurfaceKHR: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkDestroySurfaceKHR)?,
        vkCreateWaylandSurfaceKHR: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkCreateWaylandSurfaceKHR)?,
        vkEnumeratePhysicalDevices: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkEnumeratePhysicalDevices)?,
        vkEnumerateDeviceExtensionProperties: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkEnumerateDeviceExtensionProperties)?,
        vkGetPhysicalDeviceSurfaceFormatsKHR: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)?,
        vkGetPhysicalDeviceSurfacePresentModesKHR: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkGetPhysicalDeviceSurfacePresentModesKHR)?,
        vkGetPhysicalDeviceQueueFamilyProperties: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkGetPhysicalDeviceQueueFamilyProperties)?,
        vkGetPhysicalDeviceMemoryProperties: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkGetPhysicalDeviceMemoryProperties)?,
        vkGetPhysicalDeviceSurfaceSupportKHR: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkGetPhysicalDeviceSurfaceSupportKHR)?,
        vkGetPhysicalDeviceSurfaceCapabilitiesKHR: instance_function!(vk_get_instance_proc_addr, ptr_instance, PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)?,

    })
}

pub fn surface(dispatch: &InstanceDispatch, display: *mut wayland::wl_display, surface: *mut wayland::wl_surface) -> Result<*mut vulkan::SurfaceKHR, LoadError> {
    let surface_info = vulkan::WaylandSurfaceCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
        display: display,
        surface: surface,
        flags: 0,
        pNext: std::ptr::null(),
    };

    let mut ptr_surface: *mut vulkan::SurfaceKHR = std::ptr::null_mut();

    if 0 != unsafe { (dispatch.vkCreateWaylandSurfaceKHR)(dispatch.handle, &surface_info as *const vulkan::WaylandSurfaceCreateInfo, std::ptr::null(), &mut ptr_surface as *mut *mut vulkan::SurfaceKHR) } {
        return Err(LoadError::SurfaceCreate);
    }

    Ok(ptr_surface)
}

pub fn device(dispatch: &InstanceDispatch, surface: *mut vulkan::SurfaceKHR) -> Result<vulkan::Device, LoadError> {
    let required_device_extension = unsafe { std::ffi::CStr::from_ptr(b"VK_KHR_swapchain\0".as_ptr().cast()) };
    let mut count: u32 = 0;
    unsafe { (dispatch.vkEnumeratePhysicalDevices)(dispatch.handle, &mut count as *mut u32, std::ptr::null_mut()) };
    let mut physical_devices: Vec::<*mut vulkan::PhysicalDevice> = vec![std::ptr::null_mut(); count as usize];

    unsafe { (dispatch.vkEnumeratePhysicalDevices)(dispatch.handle, &mut count as *mut u32, physical_devices.as_mut_ptr() as *mut *mut vulkan::PhysicalDevice) };

    let mut found_physical_device = false;
    for physical_device in physical_devices {
        let mut count: u32 = 0;
        unsafe { (dispatch.vkEnumerateDeviceExtensionProperties)(physical_device, std::ptr::null(), &mut count as *mut u32, std::ptr::null_mut()) };

        let mut extension_properties: Vec<vulkan::ExtensionProperties> = Vec::with_capacity(count as usize);
        unsafe { extension_properties.set_len(count as usize) };

        unsafe { (dispatch.vkEnumerateDeviceExtensionProperties)(physical_device, std::ptr::null(), &mut count as *mut u32, extension_properties.as_mut_ptr() as *mut vulkan::ExtensionProperties) };

        let mut flag = false;
        for extension in extension_properties {
            let propertie = unsafe { std::ffi::CStr::from_ptr(extension.extensionName.as_ptr()) };
            if propertie == required_device_extension {
                flag = true;
                break;
            }
        }

        if !flag {
            continue;
        }

        let mut count: u32 = 0;
        unsafe { (dispatch.vkGetPhysicalDeviceQueueFamilyProperties)(physical_device, &mut count as *mut u32, std::ptr::null_mut()) };

        let mut family_properties: Vec<vulkan::QueueFamilyProperties> = Vec::with_capacity(count as usize);
        unsafe { family_properties.set_len(count as usize) };
        unsafe { (dispatch.vkGetPhysicalDeviceQueueFamilyProperties)(physical_device, &mut count as *mut u32, family_properties.as_mut_ptr() as *mut vulkan::QueueFamilyProperties) };

        let mut families: [u32; 4] = [0; 4];
        for (i, properties) in family_properties.iter().enumerate() {
            let i = i as u32;
            let mut family_flag: u32 = 0;
            unsafe { (dispatch.vkGetPhysicalDeviceSurfaceSupportKHR)(physical_device, i, surface, &mut family_flag as *mut u32) };

            if properties.queueFlags & vulkan::QUEUE_GRAPHICS_BIT != 0 {
                families[0] = i;
            } if family_flag == 1 {
                families[1] = i;
            }if properties.queueFlags & vulkan::QUEUE_COMPUTE_BIT != 0 {
                families[2] = i;
            } if properties.queueFlags & vulkan::QUEUE_TRANSFER_BIT != 0 {
                families[3] = i;
            }
        }

        found_physical_device = true;
    }

    if !found_physical_device {
        return Err(LoadError::NoSuitableDevice);
    }

    Err(LoadError::NoFunction)
}

pub fn shutdown_surface(dispatch: &InstanceDispatch, surface: *mut vulkan::SurfaceKHR) {
    unsafe {
        (dispatch.vkDestroySurfaceKHR)(dispatch.handle, surface, std::ptr::null());
    };
}

pub fn shutdown_instance(dispatch: &InstanceDispatch) {
    unsafe {
        (dispatch.vkDestroyInstance)(dispatch.handle, std::ptr::null());
    };
}
