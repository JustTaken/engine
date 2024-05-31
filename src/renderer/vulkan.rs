#![allow(non_camel_case_types, non_snake_case)]

use crate::binding::dl;
use crate::binding::vulkan;
use crate::binding::wayland;

use crate::font::TrueTypeFont;
use crate::renderer::wayland::UniqueChars;
use crate::renderer::wayland::Cursor;

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

macro_rules! device_function {
    ($proc:ident, $device:ident, $name:ident) => {
        unsafe {
            let string = std::ffi::CString::new(&stringify!($name)[4..]).unwrap();
            let pointer = $proc($device, string.as_ptr());
            let func = std::mem::transmute::<vulkan::PFN_vkVoidFunction, vulkan::$name>(pointer);

            if let Some(f) = func {
                Ok(f)
            } else {
                Err(LoadError::NoFunction)
            }
        }
    }
}

#[derive(Debug)]
pub enum DrawError {
    HasToRecreate,
    Fail,
}

#[derive(Debug)]
pub enum LoadError {
    NoLibVulkan,
    NoFunction,
    InstanceFailed,
    SurfaceCreate,
    NoExtension,
    NoSuitableDevice,
    NoSuchFile,
    ShaderModuleCreate,
    GraphicsPipelineFail,
    SwapchainDepthImage,
    SwapchainBuffer,
    SyncMemberFailed,
    BufferCreate,
    ImageFail,
    SwapchainHasToRecreate,
}

pub struct Instance {
    handle: *mut vulkan::Instance,
    vkDestroyInstance: vulkan::vkDestroyInstance,
    vkDestroySurfaceKHR: vulkan::vkDestroySurfaceKHR,
    vkCreateWaylandSurfaceKHR: vulkan::vkCreateWaylandSurfaceKHR,
    vkEnumeratePhysicalDevices: vulkan::vkEnumeratePhsysicalDevices,
    vkEnumerateDeviceExtensionProperties: vulkan::vkEnumerateDeviceExtensionProperties,
    vkGetPhysicalDeviceSurfaceFormatsKHR: vulkan::vkGetPhysicalDeviceSurfaceFormatsKHR,
    vkGetPhysicalDeviceQueueFamilyProperties: vulkan::vkGetPhysicalDeviceQueueFamilyProperties,
    vkGetPhysicalDeviceMemoryProperties: vulkan::vkGetPhysicalDeviceMemoryProperties,
    vkGetPhysicalDeviceSurfaceSupportKHR: vulkan::vkGetPhysicalDeviceSurfaceSupportKHR,
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR: vulkan::vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    vkGetPhysicalDeviceFeatures: vulkan::vkGetPhysicalDeviceFeatures,
    vkGetPhysicalDeviceProperties: vulkan::vkGetPhysicalDeviceProperties,
    vkCreateDevice: vulkan::vkCreateDevice,
    vkGetDeviceQueue: vulkan::vkGetDeviceQueue,
    vkGetDeviceProcAddr: vulkan::vkGetDeviceProcAddr,
    vkGetPhysicalDeviceFormatProperties: vulkan::vkGetPhysicalDeviceFormatProperties,
}

pub struct Device {
    handle: *mut vulkan::Device,
    surface: *mut vulkan::SurfaceKHR,
    physical_device: *mut vulkan::PhysicalDevice,
    queues: Vec<*mut vulkan::Queue>,
    families: [u32; 4],

    capabilities: vulkan::SurfaceCapabilitiesKHR,
    properties: vulkan::PhysicalDeviceMemoryProperties,
    vkDestroyDevice: vulkan::vkDestroyDevice,
    vkCreateShaderModule: vulkan::vkCreateShaderModule,
    vkCreateDescriptorSetLayout: vulkan::vkCreateDescriptorSetLayout,
    vkCreatePipelineLayout: vulkan::vkCreatePipelineLayout,
    vkCreateDescriptorPool: vulkan::vkCreateDescriptorPool,
    vkAllocateDescriptorSets: vulkan::vkAllocateDescriptorSets,
    vkCreateRenderPass: vulkan::vkCreateRenderPass,
    vkCreateGraphicsPipelines: vulkan::vkCreateGraphicsPipelines,
    vkDestroyShaderModule: vulkan::vkDestroyShaderModule,
    vkDestroyPipelineLayout: vulkan::vkDestroyPipelineLayout,
    vkDestroyDescriptorPool: vulkan::vkDestroyDescriptorPool,
    vkDestroyDescriptorSetLayout: vulkan::vkDestroyDescriptorSetLayout,
    vkDestroyRenderPass: vulkan::vkDestroyRenderPass,
    vkDestroyPipeline: vulkan::vkDestroyPipeline,
    vkGetSwapchainImagesKHR: vulkan::vkGetSwapchainImagesKHR,
    vkCreateSwapchainKHR: vulkan::vkCreateSwapchainKHR,
    vkDestroySwapchainKHR: vulkan::vkDestroySwapchainKHR,
    vkCreateImageView: vulkan::vkCreateImageView,
    vkDestroyImageView: vulkan::vkDestroyImageView,
    vkCreateImage: vulkan::vkCreateImage,
    vkGetImageMemoryRequirements: vulkan::vkGetImageMemoryRequirements,
    vkAllocateMemory: vulkan::vkAllocateMemory,
    vkDestroyImage: vulkan::vkDestroyImage,
    vkFreeMemory: vulkan::vkFreeMemory,
    vkBindImageMemory: vulkan::vkBindImageMemory,
    vkCreateFramebuffer: vulkan::vkCreateFramebuffer,
    vkDestroyFramebuffer: vulkan::vkDestroyFramebuffer,
    vkCreateCommandPool: vulkan::vkCreateCommandPool,
    vkAllocateCommandBuffers: vulkan::vkAllocateCommandBuffers,
    vkDestroyCommandPool: vulkan::vkDestroyCommandPool,
    vkBeginCommandBuffer: vulkan::vkBeginCommandBuffer,
    vkCmdBeginRenderPass: vulkan::vkCmdBeginRenderPass,
    vkCmdBindPipeline: vulkan::vkCmdBindPipeline,
    vkCmdSetViewport: vulkan::vkCmdSetViewport,
    vkCmdSetScissor: vulkan::vkCmdSetScissor,
    vkCmdEndRenderPass: vulkan::vkCmdEndRenderPass,
    vkEndCommandBuffer: vulkan::vkEndCommandBuffer,
    vkCreateSemaphore: vulkan::vkCreateSemaphore,
    vkDestroySemaphore: vulkan::vkDestroySemaphore,
    vkCreateFence: vulkan::vkCreateFence,
    vkDestroyFence: vulkan::vkDestroyFence,
    vkWaitForFences: vulkan::vkWaitForFences,
    vkAcquireNextImageKHR: vulkan::vkAcquireNextImageKHR,
    vkQueueSubmit: vulkan::vkQueueSubmit,
    vkQueuePresentKHR: vulkan::vkQueuePresentKHR,
    vkCreateBuffer: vulkan::vkCreateBuffer,
    vkDestroyBuffer: vulkan::vkDestroyBuffer,
    vkGetBufferMemoryRequirements: vulkan::vkGetBufferMemoryRequirements,
    vkBindBufferMemory: vulkan::vkBindBufferMemory,
    vkMapMemory: vulkan::vkMapMemory,
    vkUnmapMemory: vulkan::vkUnmapMemory,
    vkCmdBindVertexBuffers: vulkan::vkCmdBindVertexBuffers,
    vkResetFences: vulkan::vkResetFences,
    vkCmdPipelineBarrier: vulkan::vkCmdPipelineBarrier,
    vkFreeCommandBuffers: vulkan::vkFreeCommandBuffers,
    vkQueueWaitIdle: vulkan::vkQueueWaitIdle,
    vkCmdCopyBufferToImage: vulkan::vkCmdCopyBufferToImage,
    vkCreateSampler: vulkan::vkCreateSampler,
    vkUpdateDescriptorSets: vulkan::vkUpdateDescriptorSets,
    vkDestroySampler: vulkan::vkDestroySampler,
    vkCmdBindDescriptorSets: vulkan::vkCmdBindDescriptorSets,
    vkCmdCopyBuffer: vulkan::vkCmdCopyBuffer,
    vkCmdBindIndexBuffer: vulkan::vkCmdBindIndexBuffer,
    vkCmdDrawIndexed: vulkan::vkCmdDrawIndexed,
    vkCmdPushConstants: vulkan::vkCmdPushConstants,
    vkCmdExecuteCommands: vulkan::vkCmdExecuteCommands,
}

pub struct GraphicsPipeline {
    handle: *mut vulkan::Pipeline,
    layout: *mut vulkan::PipelineLayout,
    render_pass: *mut vulkan::RenderPass,

    global_descriptor_pool: *mut vulkan::DescriptorPool,
    global_descriptor_set_layout: *mut vulkan::DescriptorSetLayout,
    texture_descriptor_pool: *mut vulkan::DescriptorPool,
    texture_descriptor_set_layout: *mut vulkan::DescriptorSetLayout,

    surface_format: vulkan::SurfaceFormatKHR,
}

struct CommandBuffer {
    handle: *mut vulkan::CommandBuffer,
    secondary: [*mut vulkan::CommandBuffer; 2],
    is_text_updated: bool,
}

pub struct Swapchain {
    pub has_changed: bool,

    command_buffers: Vec<CommandBuffer>,
    handle: *mut vulkan::SwapchainKHR,
    image_views: Vec<*mut vulkan::ImageView>,

    texture_image: Image,
    texture_sampler: *mut vulkan::Sampler,
    texture_descriptor_set: *mut vulkan::DescriptorSet,

    cursor_texture_image: Image,
    cursor_texture_sampler: *mut vulkan::Sampler,
    cursor_texture_descriptor_set: *mut vulkan::DescriptorSet,

    uniform_descriptor_set: *mut vulkan::DescriptorSet,
    global_uniform_map: Vec<f32>,

    framebuffers: Vec<*mut vulkan::Framebuffer>,
    command_pool: *mut vulkan::CommandPool,
    extent: vulkan::Extent2D,

    global_uniform_buffer: Buffer,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    cursor_vertex_buffer: Buffer,

    image_available: *mut vulkan::Semaphore,
    render_finished: *mut vulkan::Semaphore,
    in_flight: *mut vulkan::Fence,
}

struct Buffer {
    handle: *mut vulkan::Buffer,
    memory: *mut vulkan::DeviceMemory,
}

struct Image {
    handle: *mut vulkan::Image,
    memory: *mut vulkan::DeviceMemory,
    view: *mut vulkan::ImageView,
}

fn loader_function(library: *const std::ffi::c_void) -> vulkan::PFN_vkGetInstanceProcAddr {
    unsafe {
        let string = std::ffi::CString::new("vkGetInstanceProcAddr").unwrap();
        let pointer = dl::dlsym(library, string.as_ptr());
        std::mem::transmute::<*const std::ffi::c_void, vulkan::PFN_vkGetInstanceProcAddr>(pointer)
    }
}

pub fn instance(extensions: &[*const std::ffi::c_char]) -> Result<Instance, LoadError> {
    let library = dl::load_library("libvulkan.so").map_err(|_| LoadError::NoLibVulkan)?;
    let layer_name: *const std::ffi::c_char = b"VK_LAYER_KHRONOS_validation\0".as_ptr().cast();
    let api_name: *const std::ffi::c_char = b"Hello triangle\0".as_ptr().cast();
    let version = ((1 as u32) << 22) | ((3 as u32) << 12);

    let app_info = vulkan::ApplicationInfo {
        sType: vulkan::STRUCTURE_TYPE_APPLICATION_INFO,
        pApplicationName: api_name,
        pEngineName: api_name,
        applicationVersion: version,
        engineVersion: version,
        apiVersion: version,
        pNext: std::ptr::null()
    };

    let create_info = vulkan::InstanceCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pApplicationInfo: &app_info as *const vulkan::ApplicationInfo,
        flags: 0,
        enabledLayerCount: 1,
        ppEnabledLayerNames: &layer_name as *const *const i8,
        enabledExtensionCount: extensions.len() as u32,
        ppEnabledExtensionNames: extensions.as_ptr() as *const *const i8,
        pNext: std::ptr::null(),
    };

    let vkGetInstanceProcAddr = loader_function(library).ok_or(LoadError::NoFunction)?;
    let null = std::ptr::null_mut();
    let vkCreateInstance = instance_function!(vkGetInstanceProcAddr, null, PFN_vkCreateInstance)?;

    let mut instance: *mut vulkan::Instance = std::ptr::null_mut();
    if 0 != unsafe { vkCreateInstance(&create_info as *const vulkan::InstanceCreateInfo, std::ptr::null(), &mut instance as *mut *mut vulkan::Instance) } {
        return Err(LoadError::InstanceFailed);
    }

    for string in extensions.into_iter() {
        let _ = unsafe { std::ffi::CString::from_raw(*string as *mut i8) };
    }

    Ok(Instance {
        handle: instance,
        vkDestroyInstance: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkDestroyInstance)?,
        vkDestroySurfaceKHR: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkDestroySurfaceKHR)?,
        vkCreateWaylandSurfaceKHR: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkCreateWaylandSurfaceKHR)?,
        vkEnumeratePhysicalDevices: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkEnumeratePhysicalDevices)?,
        vkEnumerateDeviceExtensionProperties: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkEnumerateDeviceExtensionProperties)?,
        vkGetPhysicalDeviceSurfaceFormatsKHR: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetPhysicalDeviceSurfaceFormatsKHR)?,
        vkGetPhysicalDeviceQueueFamilyProperties: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetPhysicalDeviceQueueFamilyProperties)?,
        vkGetPhysicalDeviceMemoryProperties: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetPhysicalDeviceMemoryProperties)?,
        vkGetPhysicalDeviceSurfaceSupportKHR: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetPhysicalDeviceSurfaceSupportKHR)?,
        vkGetPhysicalDeviceSurfaceCapabilitiesKHR: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR)?,
        vkGetPhysicalDeviceFeatures: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetPhysicalDeviceFeatures)?,
        vkGetPhysicalDeviceProperties: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetPhysicalDeviceProperties)?,
        vkCreateDevice: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkCreateDevice)?,
        vkGetDeviceQueue: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetDeviceQueue)?,
        vkGetDeviceProcAddr: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetDeviceProcAddr)?,
        vkGetPhysicalDeviceFormatProperties: instance_function!(vkGetInstanceProcAddr, instance, PFN_vkGetPhysicalDeviceFormatProperties)?,
    })
}

pub fn surface(dispatch: &Instance, display: *mut wayland::wl_display, surface: *mut wayland::wl_surface) -> Result<*mut vulkan::SurfaceKHR, LoadError> {
    let surface_info = vulkan::WaylandSurfaceCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
        display,
        surface,
        flags: 0,
        pNext: std::ptr::null(),
    };

    let mut ptr_surface: *mut vulkan::SurfaceKHR = std::ptr::null_mut();

    if 0 != unsafe { (dispatch.vkCreateWaylandSurfaceKHR)(dispatch.handle, &surface_info as *const vulkan::WaylandSurfaceCreateInfo, std::ptr::null(), &mut ptr_surface as *mut *mut vulkan::SurfaceKHR) } {
        return Err(LoadError::SurfaceCreate);
    }

    Ok(ptr_surface)
}

pub fn device(dispatch: &Instance, surface: *mut vulkan::SurfaceKHR) -> Result<Device, LoadError> {
    let mut count: u32 = 0;
    unsafe { (dispatch.vkEnumeratePhysicalDevices)(dispatch.handle, &mut count as *mut u32, std::ptr::null_mut()) };

    let mut physical_devices: Vec<*mut vulkan::PhysicalDevice> = Vec::with_capacity(count as usize);
    unsafe { (dispatch.vkEnumeratePhysicalDevices)(dispatch.handle, &mut count as *mut u32, physical_devices.as_mut_ptr() as *mut *mut vulkan::PhysicalDevice) };
    unsafe { physical_devices.set_len(count as usize) };

    let mut max_valuation: u32 = 0;
    let mut families_indices: [u32; 4] = [0; 4];
    let mut choosen_physical_device: *mut vulkan::PhysicalDevice = std::ptr::null_mut();

    let required_device_extension = unsafe { std::ffi::CStr::from_ptr(b"VK_KHR_swapchain\0".as_ptr().cast()) };
    for physical_device in physical_devices {
        let ans = avaliate_device(dispatch, surface, &required_device_extension, physical_device);

        if ans[0] > max_valuation {
            max_valuation = ans[0];
            choosen_physical_device = physical_device;
            families_indices.clone_from_slice(&ans[1..5]);
        }
    }

    if max_valuation == 0 {
        return Err(LoadError::NoSuitableDevice);
    }

    let mut families = Vec::from(&families_indices);
    families.dedup();
    let len = families.len();

    let mut queue_infos = Vec::with_capacity(len);

    for i in 0..len {
        queue_infos.push(vulkan::DeviceQueueCreateInfo {
            sType: vulkan::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            queueFamilyIndex: families[i],
            queueCount: 1,
            flags: 0,
            pNext: std::ptr::null(),
            pQueuePriorities: &(1.0 as f32) as *const f32,
        });
    }

    let mut features = std::mem::MaybeUninit::<vulkan::PhysicalDeviceFeatures>::uninit();
    unsafe { (dispatch.vkGetPhysicalDeviceFeatures)(choosen_physical_device, features.as_mut_ptr()) };

    let device_info = vulkan::DeviceCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        queueCreateInfoCount: len as u32,
        pQueueCreateInfos: queue_infos.as_ptr() as *const vulkan::DeviceQueueCreateInfo,
        pNext: std::ptr::null(),
        flags: 0,
        enabledLayerCount: 0,
        ppEnabledLayerNames: std::ptr::null(),
        ppEnabledExtensionNames: &required_device_extension.as_ptr() as *const *const i8,
        enabledExtensionCount: 1,
        pEnabledFeatures: features.as_ptr(),
    };

    let mut device: *mut vulkan::Device = std::ptr::null_mut();

    if 0 != unsafe { (dispatch.vkCreateDevice)(choosen_physical_device, &device_info as *const vulkan::DeviceCreateInfo, std::ptr::null(), &mut device as *mut *mut vulkan::Device) } {
        return Err(LoadError::NoSuitableDevice);
    }

    let mut queues: Vec<*mut vulkan::Queue> = vec![std::ptr::null_mut(); len];
    for i in 0..len {
        unsafe { (dispatch.vkGetDeviceQueue)(device, families[i], 0, &mut queues[i] as *mut *mut vulkan::Queue) } ;
    }

    let mut capabilities = std::mem::MaybeUninit::<vulkan::SurfaceCapabilitiesKHR>::uninit();
    unsafe { (dispatch.vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(choosen_physical_device, surface, capabilities.as_mut_ptr() as *mut vulkan::SurfaceCapabilitiesKHR) };
    let capabilities = unsafe { capabilities.assume_init() };

    let mut memory_properties = std::mem::MaybeUninit::<vulkan::PhysicalDeviceMemoryProperties>::uninit();
    unsafe { (dispatch.vkGetPhysicalDeviceMemoryProperties)(choosen_physical_device, memory_properties.as_mut_ptr() as *mut vulkan::PhysicalDeviceMemoryProperties) };
    let memory_properties = unsafe { memory_properties.assume_init() };

    let vkGetDeviceProcAddr = dispatch.vkGetDeviceProcAddr;

    Ok(Device {
        handle: device,
        surface,
        queues,
        physical_device: choosen_physical_device,
        families: families_indices,
        capabilities,

        properties: memory_properties,
        vkDestroyDevice: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyDevice)?,
        vkCreateShaderModule: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateShaderModule)?,
        vkCreateDescriptorSetLayout: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateDescriptorSetLayout)?,
        vkCreatePipelineLayout: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreatePipelineLayout)?,
        vkCreateDescriptorPool: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateDescriptorPool)?,
        vkAllocateDescriptorSets: device_function!(vkGetDeviceProcAddr, device, PFN_vkAllocateDescriptorSets)?,
        vkCreateRenderPass: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateRenderPass)?,
        vkCreateGraphicsPipelines: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateGraphicsPipelines)?,
        vkDestroyShaderModule: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyShaderModule)?,
        vkDestroyDescriptorPool: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyDescriptorPool)?,
        vkDestroyPipelineLayout: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyPipelineLayout)?,
        vkDestroyDescriptorSetLayout: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyDescriptorSetLayout)?,
        vkDestroyRenderPass: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyRenderPass)?,
        vkDestroyPipeline: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyPipeline)?,
        vkGetSwapchainImagesKHR: device_function!(vkGetDeviceProcAddr, device, PFN_vkGetSwapchainImagesKHR)?,
        vkCreateSwapchainKHR: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateSwapchainKHR)?,
        vkDestroySwapchainKHR: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroySwapchainKHR)?,
        vkCreateImageView: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateImageView)?,
        vkDestroyImageView: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyImageView)?,
        vkCreateImage: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateImage)?,
        vkGetImageMemoryRequirements: device_function!(vkGetDeviceProcAddr, device, PFN_vkGetImageMemoryRequirements)?,
        vkAllocateMemory: device_function!(vkGetDeviceProcAddr, device, PFN_vkAllocateMemory)?,
        vkDestroyImage: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyImage)?,
        vkFreeMemory: device_function!(vkGetDeviceProcAddr, device, PFN_vkFreeMemory)?,
        vkCreateFramebuffer: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateFramebuffer)?,
        vkDestroyFramebuffer: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyFramebuffer)?,
        vkCreateCommandPool: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateCommandPool)?,
        vkAllocateCommandBuffers: device_function!(vkGetDeviceProcAddr, device, PFN_vkAllocateCommandBuffers)?,
        vkDestroyCommandPool: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyCommandPool)?,
        vkBeginCommandBuffer: device_function!(vkGetDeviceProcAddr, device, PFN_vkBeginCommandBuffer)?,
        vkCmdBeginRenderPass: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdBeginRenderPass)?,
        vkCmdBindPipeline: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdBindPipeline)?,
        vkCmdSetViewport: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdSetViewport)?,
        vkCmdSetScissor: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdSetScissor)?,
        vkCmdEndRenderPass: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdEndRenderPass)?,
        vkEndCommandBuffer: device_function!(vkGetDeviceProcAddr, device, PFN_vkEndCommandBuffer)?,
        vkCreateSemaphore: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateSemaphore)?,
        vkDestroySemaphore: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroySemaphore)?,
        vkCreateFence: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateFence)?,
        vkDestroyFence: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyFence)?,
        vkWaitForFences: device_function!(vkGetDeviceProcAddr, device, PFN_vkWaitForFences)?,
        vkAcquireNextImageKHR: device_function!(vkGetDeviceProcAddr, device, PFN_vkAcquireNextImageKHR)?,
        vkQueueSubmit: device_function!(vkGetDeviceProcAddr, device, PFN_vkQueueSubmit)?,
        vkQueuePresentKHR: device_function!(vkGetDeviceProcAddr, device, PFN_vkQueuePresentKHR)?,
        vkCreateBuffer: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateBuffer)?,
        vkDestroyBuffer: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroyBuffer)?,
        vkGetBufferMemoryRequirements: device_function!(vkGetDeviceProcAddr, device, PFN_vkGetBufferMemoryRequirements)?,
        vkBindBufferMemory: device_function!(vkGetDeviceProcAddr, device, PFN_vkBindBufferMemory)?,
        vkMapMemory: device_function!(vkGetDeviceProcAddr, device, PFN_vkMapMemory)?,
        vkUnmapMemory: device_function!(vkGetDeviceProcAddr, device, PFN_vkUnmapMemory)?,
        vkCmdBindVertexBuffers: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdBindVertexBuffers)?,
        vkResetFences: device_function!(vkGetDeviceProcAddr, device, PFN_vkResetFences)?,
        vkBindImageMemory: device_function!(vkGetDeviceProcAddr, device, PFN_vkBindImageMemory)?,
        vkCmdPipelineBarrier: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdPipelineBarrier)?,
        vkFreeCommandBuffers: device_function!(vkGetDeviceProcAddr, device, PFN_vkFreeCommandBuffers)?,
        vkQueueWaitIdle: device_function!(vkGetDeviceProcAddr, device, PFN_vkQueueWaitIdle)?,
        vkCmdCopyBufferToImage: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdCopyBufferToImage)?,
        vkCreateSampler: device_function!(vkGetDeviceProcAddr, device, PFN_vkCreateSampler)?,
        vkUpdateDescriptorSets: device_function!(vkGetDeviceProcAddr, device, PFN_vkUpdateDescriptorSets)?,
        vkDestroySampler: device_function!(vkGetDeviceProcAddr, device, PFN_vkDestroySampler)?,
        vkCmdBindDescriptorSets: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdBindDescriptorSets)?,
        vkCmdCopyBuffer: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdCopyBuffer)?,
        vkCmdBindIndexBuffer: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdBindIndexBuffer)?,
        vkCmdDrawIndexed: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdDrawIndexed)?,
        vkCmdPushConstants: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdPushConstants)?,
        vkCmdExecuteCommands: device_function!(vkGetDeviceProcAddr, device, PFN_vkCmdExecuteCommands)?,
    })
}

fn avaliate_device(dispatch: &Instance, surface: *mut vulkan::SurfaceKHR, required_device_extension: &std::ffi::CStr, physical_device: *mut vulkan::PhysicalDevice) -> [u32; 5] {
    let mut ans: [u32; 5] = [0; 5];
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
        return [0; 5];
    }

    let mut count: u32 = 0;
    unsafe { (dispatch.vkGetPhysicalDeviceQueueFamilyProperties)(physical_device, &mut count as *mut u32, std::ptr::null_mut()) };

    let mut family_properties: Vec<vulkan::QueueFamilyProperties> = Vec::with_capacity(count as usize);

    unsafe { family_properties.set_len(count as usize) };
    unsafe { (dispatch.vkGetPhysicalDeviceQueueFamilyProperties)(physical_device, &mut count as *mut u32, family_properties.as_mut_ptr() as *mut vulkan::QueueFamilyProperties) };

    let mut families: [Option<u32>; 4] = [None; 4];
    for (i, properties) in family_properties.iter().enumerate() {
        let i = i as u32;
        let mut family_flag: u32 = 0;

        unsafe { (dispatch.vkGetPhysicalDeviceSurfaceSupportKHR)(physical_device, i.into(), surface, &mut family_flag as *mut u32) };

        if properties.queueFlags & vulkan::QUEUE_GRAPHICS_BIT != 0 && !families[0].is_some() {
            families[0] = Some(i);
        } if family_flag == vulkan::TRUE && !families[1].is_some() {
            families[1] = Some(i);
        } if properties.queueFlags & vulkan::QUEUE_COMPUTE_BIT != 0 && !families[2].is_some() {
            families[2] = Some(i);
        } if properties.queueFlags & vulkan::QUEUE_TRANSFER_BIT != 0 && !families[3].is_some() {
            families[3] = Some(i);
        }
    }

    for (i, family) in families.iter().enumerate() {
        if let Some(k) = family {
            ans[i] = *k;
        } else {
            return [0; 5];
        }
    }

    let mut features = std::mem::MaybeUninit::<vulkan::PhysicalDeviceFeatures>::uninit();
    let mut properties = std::mem::MaybeUninit::<vulkan::PhysicalDeviceProperties>::uninit();

    unsafe { (dispatch.vkGetPhysicalDeviceFeatures)(physical_device, features.as_mut_ptr()) };
    unsafe { (dispatch.vkGetPhysicalDeviceProperties)(physical_device, properties.as_mut_ptr()) };

    let features = unsafe { features.assume_init() };
    let properties = unsafe { properties.assume_init() };

    if features.geometryShader & features.samplerAnisotropy != 1 {
        return [0; 5];
    }

    ans[0] += match properties.deviceType {
        vulkan::PHYSICAL_DEVICE_TYPE_OTHER => 0,
        vulkan::PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => 3,
        vulkan::PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => 4,
        vulkan::PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => 2,
        vulkan::PHYSICAL_DEVICE_TYPE_CPU => 1,
        _ => return [0; 5],
    };

    ans
}

fn create_shader_module(device: &Device, path: &str) -> Result<*mut vulkan::ShaderModule, LoadError> {
   let code = std::fs::read(path).map_err(|_| LoadError::NoSuchFile)?;
    let info = vulkan::ShaderModuleCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        codeSize: code.len(),
        pCode: code.as_ptr() as *const u32,
    };

    let mut shader_module: *mut vulkan::ShaderModule = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateShaderModule)(device.handle, &info as *const vulkan::ShaderModuleCreateInfo, std::ptr::null(), &mut shader_module as *mut *mut vulkan::ShaderModule) } {
        return Err(LoadError::ShaderModuleCreate);
    }

    Ok(shader_module)
}

pub fn graphics_pipeline(device: &Device, instance: &Instance, width: u32, height: u32) -> Result<GraphicsPipeline, LoadError> {
    let vert_shader_module = create_shader_module(device, "assets/shader/vert.spv")?;
    let frag_shader_module = create_shader_module(device, "assets/shader/frag.spv")?;

    let shader_name = unsafe { std::ffi::CStr::from_ptr(b"main\0".as_ptr().cast()) };
    let shader_stage_infos: [vulkan::PipelineShaderStageCreateInfo; 2] = [
        vulkan::PipelineShaderStageCreateInfo {
            sType: vulkan::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            stage: vulkan::SHADER_STAGE_VERTEX_BIT,
            module: vert_shader_module,
            pName: shader_name.as_ptr(),
            pSpecializationInfo: std::ptr::null(),
        },
        vulkan::PipelineShaderStageCreateInfo {
            sType: vulkan::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            stage: vulkan::SHADER_STAGE_FRAGMENT_BIT,
            module: frag_shader_module,
            pName: shader_name.as_ptr(),
            pSpecializationInfo: std::ptr::null(),
        },
    ];

    let dynamic_states: [u32; 2] = [
        vulkan::DYNAMIC_STATE_VIEWPORT,
        vulkan::DYNAMIC_STATE_SCISSOR,
    ];

    let dynamic_states_info = vulkan::PipelineDynamicStateCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        dynamicStateCount: 2,
        pDynamicStates: dynamic_states.as_ptr() as *const u32,
    };

    let vertex_binding_description = vulkan::VertexInputBindingDescription {
        binding: 0,
        stride: std::mem::size_of::<f32>() as u32 * 2,
        inputRate: vulkan::VERTEX_INPUT_RATE,
    };

    let texture_coords_attribute_description = vulkan::VertexInputAttributeDescription {
        binding: 0,
        location: 0,
        format: vulkan::R32G32_SFLOAT,
        offset: 0,
    };

    let vertex_input_state_info = vulkan::PipelineVertexInputStateCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        vertexBindingDescriptionCount: 1,
        pVertexBindingDescriptions: &vertex_binding_description as *const vulkan::VertexInputBindingDescription,
        vertexAttributeDescriptionCount: 1,
        pVertexAttributeDescriptions: [texture_coords_attribute_description].as_ptr() as *const vulkan::VertexInputAttributeDescription,
    };

    let input_assembly_state_info = vulkan::PipelineInputAssemblyStateCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        topology: vulkan::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        primitiveRestartEnable: vulkan::FALSE,
        flags: 0,
        pNext: std::ptr::null(),
    };

    let viewport = vulkan::Viewport {
        x: 0.0,
        y: 0.0,
        width: width as f32,
        height: height as f32,
        minDepth: 0.0,
        maxDepth: 1.0,
    };

    let scissor = vulkan::Rect2D {
        offset: vulkan::Offset2D {
            x: 0,
            y: 0,
        },
        extent: vulkan::Extent2D {
            width,
            height,
        },
    };

    let viewport_state_info = vulkan::PipelineViewportStateCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        viewportCount: 1,
        pViewports: &viewport as *const vulkan::Viewport,
        scissorCount: 1,
        pScissors: &scissor as *const vulkan::Rect2D,
    };

    let rasterizer_state_info = vulkan::PipelineRasterizationStateCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        cullMode: vulkan::CULL_MODE_BACK_BIT,
        frontFace: vulkan::FRONT_FACE_CLOCKWISE,
        polygonMode: vulkan::POLYGON_MODE_FILL,
        depthBiasEnable: vulkan::FALSE,
        depthClampEnable: vulkan::FALSE,
        rasterizerDiscardEnable: vulkan::FALSE,
        lineWidth: 1.0,
        depthBiasClamp: 0.0,
        depthBiasConstantFactor: 0.0,
        depthBiasSlopeFactor: 0.0,
    };

    let multisampling_state_info = vulkan::PipelineMultisampleStateCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        rasterizationSamples: vulkan::SAMPLE_COUNT_1_BIT,
        sampleShadingEnable: vulkan::FALSE,
        pSampleMask: std::ptr::null(),
        alphaToOneEnable: vulkan::FALSE,
        alphaToCoverageEnable: vulkan::FALSE,
        minSampleShading: 1.0,
    };

    let color_blend_attachment = vulkan::PipelineColorBlendAttachmentState {
        blendEnable: vulkan::FALSE,
        colorWriteMask: vulkan::COLOR_COMPONENT_R_BIT | vulkan::COLOR_COMPONENT_G_BIT | vulkan::COLOR_COMPONENT_B_BIT | vulkan::COLOR_COMPONENT_A_BIT,
        srcColorBlendFactor: vulkan::BLEND_FACTOR_ONE,
        dstColorBlendFactor: vulkan::BLEND_FACTOR_ZERO,
        srcAlphaBlendFactor: vulkan::BLEND_FACTOR_ONE,
        dstAlphaBlendFactor: vulkan::BLEND_FACTOR_ZERO,
        colorBlendOp: vulkan::BLEND_OP_ADD,
        alphaBlendOp: vulkan::BLEND_OP_ADD,
    };

    let color_blend_state_info = vulkan::PipelineColorBlendStateCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        logicOpEnable: vulkan::FALSE,
        logicOp: vulkan::LOGIC_OP_COPY,
        blendConstants: [0.0, 0.0, 0.0, 0.0],
        attachmentCount: 1,
        pAttachments: &color_blend_attachment as *const vulkan::PipelineColorBlendAttachmentState,
    };

    let depth_stencil_state_info = vulkan::PipelineDepthStencilStateCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
        back: vulkan::StencilOpState { failOp: 0, passOp: 0, depthFailOp: 0, compareOp: 0, compareMask: 0, writeMask: 0, reference: 0, },
        front: vulkan::StencilOpState { failOp: 0, passOp: 0, depthFailOp: 0, compareOp: 0, compareMask: 0, writeMask: 0, reference: 0, },
        maxDepthBounds: 1.0,
        minDepthBounds: 0.0,
        depthCompareOp: vulkan::COMPARE_OP_LESS,
        depthTestEnable: vulkan::FALSE,
        depthWriteEnable: vulkan::FALSE,
        stencilTestEnable: vulkan::FALSE,
        depthBoundsTestEnable: vulkan::FALSE,
    };

    let global_binding = vulkan::DescriptorSetLayoutBinding {
        binding: 0,
        stageFlags: vulkan::SHADER_STAGE_VERTEX_BIT,
        descriptorType: vulkan::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
        descriptorCount: 1,
        pImmutableSamplers: std::ptr::null(),
    };

    let global_layout_info = vulkan::DescriptorSetLayoutCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
        bindingCount: 1,
        pBindings: &global_binding as *const vulkan::DescriptorSetLayoutBinding,
    };

    let mut global_descriptor_set_layout: *mut vulkan::DescriptorSetLayout = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateDescriptorSetLayout)(device.handle, &global_layout_info as *const vulkan::DescriptorSetLayoutCreateInfo, std::ptr::null(), &mut global_descriptor_set_layout as *mut *mut vulkan::DescriptorSetLayout) } {
        return Err(LoadError::GraphicsPipelineFail);
    }

    let texture_binding = vulkan::DescriptorSetLayoutBinding {
        binding: 0,
        stageFlags: vulkan::SHADER_STAGE_FRAGMENT_BIT,
        descriptorType: vulkan::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
        descriptorCount: 1,
        pImmutableSamplers: std::ptr::null(),
    };

    let texture_layout_info = vulkan::DescriptorSetLayoutCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
        bindingCount: 1,
        pBindings: &texture_binding as *const vulkan::DescriptorSetLayoutBinding,
    };

    let mut texture_descriptor_set_layout: *mut vulkan::DescriptorSetLayout = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateDescriptorSetLayout)(device.handle, &texture_layout_info as *const vulkan::DescriptorSetLayoutCreateInfo, std::ptr::null(), &mut texture_descriptor_set_layout as *mut *mut vulkan::DescriptorSetLayout) } {
        return Err(LoadError::GraphicsPipelineFail);
    }

    let push_constant = vulkan::PushConstantRange {
        stageFlags: vulkan::SHADER_STAGE_VERTEX_BIT,
        offset: 0,
        size: std::mem::size_of::<f32>() as u32 * 2,
    };

    let layout_info = vulkan::PipelineLayoutCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        pushConstantRangeCount: 1,
        pPushConstantRanges: &push_constant as *const vulkan::PushConstantRange,
        setLayoutCount: 2,
        pSetLayouts: [global_descriptor_set_layout, texture_descriptor_set_layout].as_ptr(),
    };

    let mut layout: *mut vulkan::PipelineLayout = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreatePipelineLayout)(device.handle, &layout_info as *const vulkan::PipelineLayoutCreateInfo, std::ptr::null(), &mut layout as *mut *mut vulkan::PipelineLayout) } {
        return Err(LoadError::GraphicsPipelineFail);
    }

    let global_pool_size = vulkan::DescriptorPoolSize {
        type_: vulkan::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
        descriptorCount: 16,
    };

    let global_pool_info = vulkan::DescriptorPoolCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
        poolSizeCount: 1,
        pPoolSizes: &global_pool_size,
        maxSets: 16,
    };

    let mut global_descriptor_pool: *mut vulkan::DescriptorPool = std::ptr::null_mut();
    unsafe { (device.vkCreateDescriptorPool)(device.handle, &global_pool_info as *const vulkan::DescriptorPoolCreateInfo, std::ptr::null(), &mut global_descriptor_pool as *mut *mut vulkan::DescriptorPool) };

    let texture_pool_size = vulkan::DescriptorPoolSize {
        type_: vulkan::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
        descriptorCount: 16,
    };

    let texture_pool_info = vulkan::DescriptorPoolCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
        poolSizeCount: 1,
        pPoolSizes: &texture_pool_size,
        maxSets: 16,
    };

    let mut texture_descriptor_pool: *mut vulkan::DescriptorPool = std::ptr::null_mut();
    unsafe { (device.vkCreateDescriptorPool)(device.handle, &texture_pool_info as *const vulkan::DescriptorPoolCreateInfo, std::ptr::null(), &mut texture_descriptor_pool as *mut *mut vulkan::DescriptorPool) };

    let mut count: u32 = 0;
    unsafe { (instance.vkGetPhysicalDeviceSurfaceFormatsKHR)(device.physical_device, device.surface, &mut count as *mut u32, std::ptr::null_mut()) };

    let mut surface_formats: Vec<vulkan::SurfaceFormatKHR> = Vec::with_capacity(count as usize);
    unsafe { surface_formats.set_len(count as usize) };
    unsafe { (instance.vkGetPhysicalDeviceSurfaceFormatsKHR)(device.physical_device, device.surface, &mut count as *mut u32, surface_formats.as_mut_ptr() as *mut vulkan::SurfaceFormatKHR) };

    let mut surface_format = vulkan::SurfaceFormatKHR {
        format: surface_formats[0].format,
        colorSpace: surface_formats[0].colorSpace,
    };

    for format in surface_formats.into_iter() {
        if format.format == vulkan::R8G8B8A8_SRGB && format.colorSpace == vulkan::COLOR_SPACE_SRGB_NONLINEAR_KHR {
            surface_format = format;
            break;
        }
    }

    let render_pass_attachments: [vulkan::AttachmentDescription; 1] = [
        vulkan::AttachmentDescription {
            format: surface_format.format,
            flags: 0,
            samples: vulkan::SAMPLE_COUNT_1_BIT,
            loadOp: vulkan::ATTACHMENT_LOAD_OP_CLEAR,
            storeOp: vulkan::ATTACHMENT_STORE_OP_STORE,
            finalLayout: vulkan::IMAGE_LAYOUT_PRESENT_SRC_KHR,
            initialLayout: vulkan::IMAGE_LAYOUT_UNDEFINED,
            stencilLoadOp: vulkan::ATTACHMENT_LOAD_OP_DONT_CARE,
            stencilStoreOp: vulkan::ATTACHMENT_STORE_OP_DONT_CARE,
        },
    ];

    let color_attachment = vulkan::AttachmentReference {
        attachment: 0,
        layout: vulkan::IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
    };

    let subpass = vulkan::SubpassDescription {
        pipelineBindPoint: vulkan::PIPELINE_BIND_POINT_GRAPHICS,
        colorAttachmentCount: 1,
        pColorAttachments: &color_attachment as *const vulkan::AttachmentReference,
        pDepthStencilAttachment: std::ptr::null(),//&stencil_attachment as *const vulkan::AttachmentReference,
        flags: 0,
        inputAttachmentCount: 0,
        pInputAttachments: std::ptr::null(),
        pResolveAttachments: std::ptr::null(),
        preserveAttachmentCount: 0,
        pPreserveAttachments: std::ptr::null(),
    };

    let dependencies = vulkan::SubpassDependency {
        srcSubpass: vulkan::SUBPASS_EXTERNAL,
        dstSubpass: 0,
        srcAccessMask: 0,
        srcStageMask: vulkan::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | vulkan::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT,
        dstStageMask: vulkan::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT | vulkan::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT,
        dstAccessMask: vulkan::ACCESS_COLOR_ATTACHMENT_WRITE_BIT | vulkan::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
        dependencyFlags: vulkan::DEPENDENCY_BY_REGION_BIT,
    };

    let render_pass_info = vulkan::RenderPassCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
        attachmentCount: render_pass_attachments.len() as u32,
        pAttachments: render_pass_attachments.as_ptr() as *const vulkan::AttachmentDescription,
        subpassCount: 1,
        pSubpasses: &subpass as *const vulkan::SubpassDescription,
        dependencyCount: 1,
        pDependencies: &dependencies as *const vulkan::SubpassDependency,
    };

    let mut render_pass: *mut vulkan::RenderPass = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateRenderPass)(device.handle, &render_pass_info as *const vulkan::RenderPassCreateInfo, std::ptr::null(), &mut render_pass as *mut *mut vulkan::RenderPass) } {
        return Err(LoadError::GraphicsPipelineFail);
    }

    let graphics_pipeline_info = vulkan::GraphicsPipelineCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        stageCount: shader_stage_infos.len() as u32,
        pStages: shader_stage_infos.as_ptr() as *const vulkan::PipelineShaderStageCreateInfo,
        pVertexInputState: &vertex_input_state_info as *const vulkan::PipelineVertexInputStateCreateInfo,
        pInputAssemblyState: &input_assembly_state_info as *const vulkan::PipelineInputAssemblyStateCreateInfo,
        pTessellationState: std::ptr::null(),
        pViewportState: &viewport_state_info as *const vulkan::PipelineViewportStateCreateInfo,
        pRasterizationState: &rasterizer_state_info as *const vulkan::PipelineRasterizationStateCreateInfo,
        pMultisampleState: &multisampling_state_info as *const vulkan::PipelineMultisampleStateCreateInfo,
        pDepthStencilState: &depth_stencil_state_info as *const vulkan::PipelineDepthStencilStateCreateInfo,
        pColorBlendState: &color_blend_state_info as *const vulkan::PipelineColorBlendStateCreateInfo,
        pDynamicState: &dynamic_states_info as *const vulkan::PipelineDynamicStateCreateInfo,
        layout,
        renderPass: render_pass,
        subpass: 0,
        basePipelineHandle: std::ptr::null_mut(),
        basePipelineIndex: 0,
    };

    let mut pipeline: *mut vulkan::Pipeline = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateGraphicsPipelines)(device.handle, std::ptr::null_mut(), 1, &graphics_pipeline_info as *const vulkan::GraphicsPipelineCreateInfo, std::ptr::null(), &mut pipeline as *mut *mut vulkan::Pipeline) } {
        return Err(LoadError::GraphicsPipelineFail);
    }

    unsafe { (device.vkDestroyShaderModule)(device.handle, vert_shader_module, std::ptr::null()) };
    unsafe { (device.vkDestroyShaderModule)(device.handle, frag_shader_module, std::ptr::null()) };

    Ok(GraphicsPipeline {
        handle: pipeline,
        render_pass,
        layout,
        surface_format,
        global_descriptor_pool,
        global_descriptor_set_layout,
        texture_descriptor_pool,
        texture_descriptor_set_layout,
    })
}

fn buffer<T>(device: &Device, usage: u32, properties: u32, len: usize) -> Result<Buffer, LoadError> {
    let buffer_info = vulkan::BufferCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        size: (std::mem::size_of::<T>() * len) as u64,
        usage,
        sharingMode: vulkan::SHARING_MODE_EXCLUSIVE,
        queueFamilyIndexCount: 0,
        pQueueFamilyIndices: std::ptr::null()
    };

    let mut handle: *mut vulkan::Buffer = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateBuffer)(device.handle, &buffer_info, std::ptr::null(), &mut handle as *mut *mut vulkan::Buffer) } {
    }

    let mut memory_requirements = std::mem::MaybeUninit::<vulkan::MemoryRequirements>::uninit();
    let mut memory_index: u32 = 0;

    unsafe { (device.vkGetBufferMemoryRequirements)(device.handle, handle, memory_requirements.as_mut_ptr() as *mut vulkan::MemoryRequirements) };
    let memory_requirements = unsafe { memory_requirements.assume_init() };

    for i in 0..device.properties.memoryTypeCount {
        if memory_requirements.memoryTypeBits & (1 as u32) << i != 0 && device.properties.memoryTypes[i as usize].propertyFlags & properties == properties {
            memory_index = i as u32;
            break;
        }
    }

    let alloc_info = vulkan::MemoryAllocateInfo {
        sType: vulkan::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        pNext: std::ptr::null(),
        allocationSize: memory_requirements.size,
        memoryTypeIndex: memory_index,
    };

    let mut memory: *mut vulkan::DeviceMemory = std::ptr::null_mut();
    if 0 != unsafe { (device.vkAllocateMemory)(device.handle, &alloc_info as *const vulkan::MemoryAllocateInfo, std::ptr::null(), &mut memory as *mut *mut vulkan::DeviceMemory) } {
        return Err(LoadError::BufferCreate);
    }

    unsafe { (device.vkBindBufferMemory)(device.handle, handle, memory, 0) };

    Ok(Buffer {
        handle,
        memory,
    })
}

fn create_sampler(device: &Device, adress_mode: u32) -> *mut vulkan::Sampler {
    let texture_sampler_info = vulkan::SamplerCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        magFilter: vulkan::FILTER_LINEAR,
        minFilter: vulkan::FILTER_LINEAR,
        addressModeU: adress_mode,
        addressModeV: adress_mode,
        addressModeW: adress_mode,
        anisotropyEnable: vulkan::FALSE,
        maxAnisotropy: 1.0,
        borderColor: vulkan::BORDER_COLOR_INT_OPAQUE_BLACK,
        unnormalizedCoordinates: vulkan::FALSE,
        compareEnable: vulkan::FALSE,
        compareOp: vulkan::COMPARE_OP_ALWAYS,
        mipmapMode: vulkan::SAMPLER_MIPMAP_MODE_LINEAR,
        mipLodBias: 0.0,
        minLod: 0.0,
        maxLod: 0.0,
    };

    let mut texture_sampler: *mut vulkan::Sampler = std::ptr::null_mut();
    unsafe { (device.vkCreateSampler)(device.handle, &texture_sampler_info as *const vulkan::SamplerCreateInfo, std::ptr::null(), &mut texture_sampler as *mut *mut vulkan::Sampler) };

    texture_sampler
}

fn allocate_descriptor_set(device: &Device, descriptor_pool: *mut vulkan::DescriptorPool, layout: *mut vulkan::DescriptorSetLayout) -> *mut vulkan::DescriptorSet {
    let info = vulkan::DescriptorSetAllocateInfo {
        sType: vulkan::STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
        pNext: std::ptr::null(),
        descriptorPool: descriptor_pool,
        descriptorSetCount: 1,
        pSetLayouts: &layout as *const *mut vulkan::DescriptorSetLayout,
    };

    let mut descriptor_set: *mut vulkan::DescriptorSet = std::ptr::null_mut();
    unsafe { (device.vkAllocateDescriptorSets)(device.handle, &info as *const vulkan::DescriptorSetAllocateInfo, &mut descriptor_set as *mut *mut vulkan::DescriptorSet) };

    descriptor_set
}

fn update_image_descriptor_set(device: &Device, image_view: *mut vulkan::ImageView, image_sampler: *mut vulkan::Sampler, descriptor_set: *mut vulkan::DescriptorSet) {
    let descriptor_image_info = vulkan::DescriptorImageInfo {
        imageLayout: vulkan::IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
        imageView: image_view,
        sampler: image_sampler,
    };

    let write_descriptor_set = vulkan::WriteDescriptorSet {
        sType: vulkan::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
        pNext: std::ptr::null(),
        dstSet: descriptor_set,
        dstBinding: 0,
        dstArrayElement: 0,
        descriptorCount: 1,
        descriptorType: vulkan::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
        pImageInfo: &descriptor_image_info as *const vulkan::DescriptorImageInfo,
        pBufferInfo: std::ptr::null(),
        pTexelBufferView: std::ptr::null(),
    };

    unsafe { (device.vkUpdateDescriptorSets)(device.handle, 1, &write_descriptor_set as *const vulkan::WriteDescriptorSet, 0, std::ptr::null()) };
}

fn copy_buffer_to_image(
    device: &Device,
    command_pool: *mut vulkan::CommandPool,
    image: *mut vulkan::Image,
    data: &[u8],
    width: u32,
    height: u32,
) -> Result<(), LoadError> {
    let buffer = buffer::<u8>(device, vulkan::BUFFER_USAGE_TRANSFER_SRC_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, data.len())?;

    let mut dst: *mut u8 = std::ptr::null_mut();
    unsafe { (device.vkMapMemory)(device.handle, buffer.memory, 0, data.len() as u64, 0, std::mem::transmute::<&mut *mut u8, *mut *mut std::ffi::c_void>(&mut dst)) };
    unsafe { std::ptr::copy(data.as_ptr(), dst, data.len()) };
    unsafe { (device.vkUnmapMemory)(device.handle, buffer.memory) };

    let barrier_command_buffer = begin_command_buffer(device, command_pool);

    let barrier = vulkan::ImageMemoryBarrier {
        sType: vulkan::STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
        pNext: std::ptr::null(),
        oldLayout: vulkan::IMAGE_LAYOUT_UNDEFINED,
        newLayout: vulkan::IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        srcQueueFamilyIndex: vulkan::QUEUE_FAMILY_IGNORED,
        dstQueueFamilyIndex: vulkan::QUEUE_FAMILY_IGNORED,
        image,
        subresourceRange: vulkan::ImageSubresourceRange {
            aspectMask: vulkan::IMAGE_ASPECT_COLOR_BIT,
            baseMipLevel: 0,
            levelCount: 1,
            baseArrayLayer: 0,
            layerCount: 1,
        },
        srcAccessMask: 0,
        dstAccessMask: vulkan::ACCESS_TRANSFER_WRITE_BIT,
    };

    unsafe { (device.vkCmdPipelineBarrier)(barrier_command_buffer, vulkan::PIPELINE_STAGE_TOP_OF_PIPE_BIT, vulkan::PIPELINE_STAGE_TRANSFER_BIT, 0, 0 as u32, std::ptr::null(), 0, std::ptr::null(), 1, &barrier) };
    let region = vulkan::BufferImageCopy {
        bufferOffset: 0,
        bufferRowLength: 0,
        bufferImageHeight: 0,
        imageSubresource: vulkan::ImageSubresourceLayers {
            aspectMask: vulkan::IMAGE_ASPECT_COLOR_BIT,
            mipLevel: 0,
            baseArrayLayer: 0,
            layerCount: 1,
        },
        imageOffset: vulkan::Offset3D {
            x: 0,
            y: 0,
            z: 0,
        },
        imageExtent: vulkan::Extent3D {
            width,
            height,
            depth: 1,
        },
    };

    unsafe { (device.vkCmdCopyBufferToImage)(barrier_command_buffer, buffer.handle, image, vulkan::IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &region as *const vulkan::BufferImageCopy) };
    end_command_buffer(device, command_pool, barrier_command_buffer);

    let second_barrier = vulkan::ImageMemoryBarrier {
        sType: vulkan::STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
        pNext: std::ptr::null(),
        oldLayout: vulkan::IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        newLayout: vulkan::IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
        srcQueueFamilyIndex: vulkan::QUEUE_FAMILY_IGNORED,
        dstQueueFamilyIndex: vulkan::QUEUE_FAMILY_IGNORED,
        image,
        subresourceRange: vulkan::ImageSubresourceRange {
            aspectMask: vulkan::IMAGE_ASPECT_COLOR_BIT,
            baseMipLevel: 0,
            levelCount: 1,
            baseArrayLayer: 0,
            layerCount: 1,
        },
        srcAccessMask: vulkan::ACCESS_TRANSFER_WRITE_BIT,
        dstAccessMask: vulkan::ACCESS_SHADER_READ_BIT,
    };

    let second_barrier_command_buffer = begin_command_buffer(device, command_pool);
    unsafe { (device.vkCmdPipelineBarrier)(second_barrier_command_buffer, vulkan::PIPELINE_STAGE_TRANSFER_BIT, vulkan::PIPELINE_STAGE_FRAGMENT_SHADER_BIT, 0, 0 as u32, std::ptr::null(), 0, std::ptr::null(), 1, &second_barrier) };
    end_command_buffer(device, command_pool, second_barrier_command_buffer);

    unsafe { (device.vkFreeMemory)(device.handle, buffer.memory, std::ptr::null()) };
    unsafe { (device.vkDestroyBuffer)(device.handle, buffer.handle, std::ptr::null()) };

    Ok(())
}

fn allocate_device_memory(device: &Device, requirements: vulkan::MemoryRequirements) -> *mut vulkan::DeviceMemory {
    let mut memory_index: u32 = 0;

    let property = vulkan::MEMORY_PROPERTY_DEVICE_LOCAL_BIT;
    for i in 0..device.properties.memoryTypeCount {
        if requirements.memoryTypeBits & (1 as u32) << i != 0 && device.properties.memoryTypes[i as usize].propertyFlags & property == property {
            memory_index = i as u32;
            break;
        }
    }

    let image_memory_allocate_info = vulkan::MemoryAllocateInfo {
        sType: vulkan::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        pNext: std::ptr::null(),
        allocationSize: requirements.size,
        memoryTypeIndex: memory_index,
    };

    let mut memory: *mut vulkan::DeviceMemory = std::ptr::null_mut();
    unsafe { (device.vkAllocateMemory)(device.handle, &image_memory_allocate_info as *const vulkan::MemoryAllocateInfo, std::ptr::null(), &mut memory as *mut *mut vulkan::DeviceMemory) };
    memory
}

fn create_image(
    device: &Device,
    format: u32,
    usage: u32,
    aspect: u32,
    width: u32,
    height: u32,
) -> Result<Image, LoadError> {
    let info = vulkan::ImageCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_IMAGE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        imageType: vulkan::IMAGE_TYPE_2D,
        extent: vulkan::Extent3D {
            width,
            height,
            depth: 1,
        },
        mipLevels: 1,
        arrayLayers: 1,
        format,
        tiling: vulkan::IMAGE_TILING_OPTIMAL,
        initialLayout: vulkan::IMAGE_LAYOUT_UNDEFINED,
        usage,
        sharingMode: vulkan::SHARING_MODE_EXCLUSIVE,
        samples: vulkan::SAMPLE_COUNT_1_BIT,
        queueFamilyIndexCount: 0,
        pQueueFamilyIndices: std::ptr::null(),
    };

    let mut image: *mut vulkan::Image = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateImage)(device.handle, &info as *const vulkan::ImageCreateInfo, std::ptr::null(), &mut image as *mut *mut vulkan::Image) } {
        return Err(LoadError::ImageFail);
    }

    let mut memory_requirements = std::mem::MaybeUninit::<vulkan::MemoryRequirements>::uninit();
    unsafe { (device.vkGetImageMemoryRequirements)(device.handle, image, memory_requirements.as_mut_ptr() as *mut vulkan::MemoryRequirements) };

    let memory_requirements = unsafe { memory_requirements.assume_init() };

    let image_memory = allocate_device_memory(device, memory_requirements);
    unsafe { (device.vkBindImageMemory)(device.handle, image, image_memory, 0) };

    let view = create_image_view(device, image, format, aspect);

    Ok(Image {
        handle: image,
        memory: image_memory,
        view,
    })
}

fn create_image_view(device: &Device, image: *mut vulkan::Image, format: u32, aspect: u32) -> *mut vulkan::ImageView {
    let info = vulkan::ImageViewCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
        image,
        viewType: vulkan::IMAGE_VIEW_TYPE_2D,
        format,
        subresourceRange: vulkan::ImageSubresourceRange {
            aspectMask: aspect,
            baseMipLevel: 0,
            levelCount: 1,
            baseArrayLayer: 0,
            layerCount: 1,
        },
        components: vulkan::ComponentMapping {
            r: vulkan::COMPONENT_SWIZZLE_IDENTITY,
            g: vulkan::COMPONENT_SWIZZLE_IDENTITY,
            b: vulkan::COMPONENT_SWIZZLE_IDENTITY,
            a: vulkan::COMPONENT_SWIZZLE_IDENTITY,
        },
    };

    let mut image_view: *mut vulkan::ImageView = std::ptr::null_mut();
    unsafe { (device.vkCreateImageView)(device.handle, &info as *const vulkan::ImageViewCreateInfo, std::ptr::null(), &mut image_view as *mut *mut vulkan::ImageView) };

    image_view
}

fn create_framebuffer(
    device: &Device,
    render_pass: *mut vulkan::RenderPass,
    images_view: &[*mut vulkan::ImageView],
    width: u32,
    height: u32
) -> *mut vulkan::Framebuffer {
    let info = vulkan::FramebufferCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
        renderPass: render_pass,
        attachmentCount: images_view.len() as u32,
        pAttachments: images_view.as_ptr() as *const *mut vulkan::ImageView,
        width,
        height,
        layers: 1,
    };

    let mut framebuffer: *mut vulkan::Framebuffer = std::ptr::null_mut();
    unsafe { (device.vkCreateFramebuffer)(device.handle, &info as *const vulkan::FramebufferCreateInfo, std::ptr::null(), &mut framebuffer as *mut *mut vulkan::Framebuffer) };

    framebuffer
}

pub fn swapchain(
    device: &Device,
    graphics_pipeline: &GraphicsPipeline,
    font: &TrueTypeFont,
    chars_len: usize,
    width: u32,
    height: u32,
) -> Result<Swapchain, LoadError> {
    let present_mode = vulkan::PRESENT_MODE_FIFO_KHR;
    let extent = vulkan::Extent2D {
        width,
        height,
    };

    let image_count = if device.capabilities.maxImageCount > 0 {
        std::cmp::min(device.capabilities.minImageCount + 1, device.capabilities.maxImageCount)
    } else {
        device.capabilities.minImageCount + 1
    };

    let (queue_family_index_len, sharing_mode): (u32, u32) = if device.families[0] == device.families[1] {
        (1, vulkan::SHARING_MODE_EXCLUSIVE)
    } else {
        (2, vulkan::SHARING_MODE_CONCURRENT)
    };

    let info = vulkan::SwapchainCreateInfoKHR {
        sType: vulkan::STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
        flags: 0,
        pNext: std::ptr::null(),
        surface: device.surface,
        minImageCount: image_count,
        imageFormat: graphics_pipeline.surface_format.format,
        imageColorSpace: graphics_pipeline.surface_format.colorSpace,
        imageExtent: vulkan::Extent2D {
            width: extent.width,
            height: extent.height,
        },
        imageSharingMode: sharing_mode,
        presentMode: present_mode,
        preTransform: device.capabilities.currentTransform,
        clipped: vulkan::TRUE,
        imageArrayLayers: 1,
        compositeAlpha: vulkan::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
        imageUsage: vulkan::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
        queueFamilyIndexCount: queue_family_index_len,
        pQueueFamilyIndices: device.families.as_ptr(),
        oldSwapchain: std::ptr::null_mut(),
    };

    let mut handle: *mut vulkan::SwapchainKHR = std::ptr::null_mut();
    unsafe { (device.vkCreateSwapchainKHR)(device.handle, &info as *const vulkan::SwapchainCreateInfoKHR, std::ptr::null(), &mut handle as *mut *mut vulkan::SwapchainKHR)};

    let mut count: u32 = 0;
    unsafe { (device.vkGetSwapchainImagesKHR)(device.handle, handle, &mut count as *mut u32, std::ptr::null_mut()) };
    let mut images: Vec<*mut vulkan::Image> = Vec::with_capacity(count as usize);
    unsafe { (device.vkGetSwapchainImagesKHR)(device.handle, handle, &mut count as *mut u32, images.as_mut_ptr() as *mut *mut vulkan::Image) };
    unsafe { images.set_len(count as usize) };

    let mut image_views: Vec<*mut vulkan::ImageView> = Vec::with_capacity(count as usize);

    for i in 0..count {
        image_views.push(
            create_image_view(
                device,
                images[i as usize],
                graphics_pipeline.surface_format.format,
                vulkan::IMAGE_ASPECT_COLOR_BIT
            )
        );
    }

    let mut framebuffers: Vec<*mut vulkan::Framebuffer> = Vec::with_capacity(count as usize);
    for i in 0..count {
        framebuffers.push(
            create_framebuffer(
                device,
                graphics_pipeline.render_pass,
                &[image_views[i as usize]],
                extent.width,
                extent.height
            )
        );
    }

    let command_pool_info = vulkan::CommandPoolCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
        flags: vulkan::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
        pNext: std::ptr::null(),
        queueFamilyIndex: device.families[0],
    };

    let mut command_pool: *mut vulkan::CommandPool = std::ptr::null_mut();
    unsafe { (device.vkCreateCommandPool)(device.handle, &command_pool_info as *const vulkan::CommandPoolCreateInfo, std::ptr::null(), &mut command_pool as *mut *mut vulkan::CommandPool) };

    let semaphore_info = vulkan::SemaphoreCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
    };

    let fence_info = vulkan::FenceCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_FENCE_CREATE_INFO,
        flags: vulkan::FENCE_CREATE_SIGNALED_BIT,
        pNext: std::ptr::null(),
    };

    let mut render_finished: *mut vulkan::Semaphore = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateSemaphore)(device.handle, &semaphore_info as *const vulkan::SemaphoreCreateInfo, std::ptr::null(), &mut render_finished as *mut *mut vulkan::Semaphore) } {
        return Err(LoadError::SyncMemberFailed);
    }

    let mut image_available: *mut vulkan::Semaphore = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateSemaphore)(device.handle, &semaphore_info as *const vulkan::SemaphoreCreateInfo, std::ptr::null(), &mut image_available as *mut *mut vulkan::Semaphore) } {
        return Err(LoadError::SyncMemberFailed);
    }

    let mut in_flight: *mut vulkan::Fence = std::ptr::null_mut();
    if 0 != unsafe { (device.vkCreateFence)(device.handle, &fence_info as *const vulkan::FenceCreateInfo, std::ptr::null(), &mut in_flight as *mut *mut vulkan::Fence) } {
        return Err(LoadError::SyncMemberFailed);
    }

    let mut global_uniform_dst: *mut f32 = std::ptr::null_mut();
    let ratio = height as f32 / width as f32;

    let global_uniform = [
        ratio,
        font.scale,
        -1.0,
        -1.0,
        font.x_ratio,
    ];

    let global_uniform_buffer = buffer::<f32>(device, vulkan::BUFFER_USAGE_UNIFORM_BUFFER_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, global_uniform.len())?;
    unsafe { (device.vkMapMemory)(device.handle, global_uniform_buffer.memory, 0, global_uniform.len() as u64, 0, std::mem::transmute::<&mut *mut f32, *mut *mut std::ffi::c_void>(&mut global_uniform_dst)) };
    unsafe { std::ptr::copy(global_uniform.as_ptr(), global_uniform_dst, global_uniform.len()) };
    let global_uniform_map = unsafe { Vec::from_raw_parts(global_uniform_dst, global_uniform.len(), 0) };

    let uniform_descriptor_set = allocate_descriptor_set(device, graphics_pipeline.global_descriptor_pool, graphics_pipeline.global_descriptor_set_layout);

    let global_uniform_descriptor_info = vulkan::DescriptorBufferInfo {
        buffer: global_uniform_buffer.handle,
        offset: 0,
        range: std::mem::size_of::<f32>() as u64 * global_uniform.len() as u64,
    };

    let global_uniform_write_descriptor_set = vulkan::WriteDescriptorSet {
        sType: vulkan::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
        pNext: std::ptr::null(),
        dstSet: uniform_descriptor_set,
        dstBinding: 0,
        dstArrayElement: 0,
        descriptorCount: 1,
        descriptorType: vulkan::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
        pImageInfo: std::ptr::null(),
        pBufferInfo: &global_uniform_descriptor_info as *const vulkan::DescriptorBufferInfo,
        pTexelBufferView: std::ptr::null(),
    };

    unsafe { (device.vkUpdateDescriptorSets)(device.handle, 1, &global_uniform_write_descriptor_set as *const vulkan::WriteDescriptorSet, 0, std::ptr::null()) };

    let vertex_staging_buffer = buffer::<[f32; 2]>(device, vulkan::BUFFER_USAGE_TRANSFER_SRC_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, 4 * chars_len)?;
    let mut vertex_data: *mut [f32; 2] = std::ptr::null_mut();

    unsafe { (device.vkMapMemory)(device.handle, vertex_staging_buffer.memory, 0, (chars_len * 4 * std::mem::size_of::<[f32; 2]>()) as u64, 0, std::mem::transmute::<&mut *mut [f32; 2], *mut *mut std::ffi::c_void>(&mut vertex_data)) };
    let vertex_buffer = buffer::<[f32; 2]>(device, vulkan::BUFFER_USAGE_TRANSFER_DST_BIT | vulkan::BUFFER_USAGE_VERTEX_BUFFER_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, 4 * chars_len)?;
    let mut instance_vertex_buffer = unsafe { Vec::from_raw_parts(vertex_data, chars_len * 4, 0) };
    let vertex_command_buffer = begin_command_buffer(device, command_pool);

    let height = font.line_height as f32 / font.height as f32;
    for (i, metric) in font.metrics.iter().enumerate() {
        let width = metric.width as f32 / font.width as f32;
        let x_offset = metric.x_offset as f32 / font.width as f32;
        let y_offset = metric.y_offset as f32 / font.height as f32;

        let index: usize = i as usize * 4;

        instance_vertex_buffer[index..(index + 4)].copy_from_slice(&[
            [x_offset, y_offset],
            [x_offset + width, y_offset],
            [x_offset, y_offset + height],
            [x_offset + width, y_offset + height],
        ]);

        let vertex_copy_info = vulkan::BufferCopy {
            srcOffset: (4 * i as usize * std::mem::size_of::<[f32; 2]>()) as u64,
            dstOffset: (4 * i as usize * std::mem::size_of::<[f32; 2]>()) as u64,
            size: (4 * std::mem::size_of::<[f32; 2]>()) as u64,
        };

        unsafe { (device.vkCmdCopyBuffer)(vertex_command_buffer, vertex_staging_buffer.handle, vertex_buffer.handle, 1, &vertex_copy_info as *const vulkan::BufferCopy) };
    }

    // let vertex_staging_buffer = buffer::<[f32; 2]>(device, vulkan::BUFFER_USAGE_TRANSFER_SRC_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, 4)?;
    // let mut vertex_data: *mut [f32; 2] = std::ptr::null_mut();
    // unsafe { (device.vkMapMemory)(device.handle, vertex_staging_buffer.memory, 0, (4 * std::mem::size_of::<[f32; 2]>()) as u64, 0, std::mem::transmute::<&mut *mut [f32; 2], *mut *mut std::ffi::c_void>(&mut vertex_data)) };

    // let vertex_buffer = buffer::<[f32; 2]>(device, vulkan::BUFFER_USAGE_TRANSFER_DST_BIT | vulkan::BUFFER_USAGE_VERTEX_BUFFER_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, 4)?;
    // let mut instance_vertex_buffer = unsafe { Vec::from_raw_parts(vertex_data, 4, 0) };

    // instance_vertex_buffer[0..4].copy_from_slice(&[
    //     [0.0, 1.0],
    //     [1.0, 1.0],
    //     [0.0, 0.0],
    //     [1.0, 0.0],
    // ]);
    // let vertex_copy_info = vulkan::BufferCopy {
    //     srcOffset: 0,
    //     dstOffset: 0,
    //     size: (4 * std::mem::size_of::<[f32; 2]>()) as u64,
    // };
    // let vertex_command_buffer = begin_command_buffer(device, command_pool);

    // unsafe { (device.vkCmdCopyBuffer)(vertex_command_buffer, vertex_staging_buffer.handle, vertex_buffer.handle, 1, &vertex_copy_info as *const vulkan::BufferCopy) };

    unsafe { (device.vkUnmapMemory)(device.handle, vertex_staging_buffer.memory) };

    let indices: [u16; 6] = [
        0, 1, 2, 1, 3, 2
    ];

    let index_staging_buffer = buffer::<u16>(device, vulkan::BUFFER_USAGE_TRANSFER_SRC_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, indices.len())?;
    let mut index_data: *mut u16 = std::ptr::null_mut();
    unsafe { (device.vkMapMemory)(device.handle, index_staging_buffer.memory, 0, (indices.len() * std::mem::size_of::<u16>()) as u64, 0, std::mem::transmute::<&mut *mut u16, *mut *mut std::ffi::c_void>(&mut index_data)) };
    unsafe { std::ptr::copy(indices.as_ptr(), index_data, indices.len()) };
    unsafe { (device.vkUnmapMemory)(device.handle, index_staging_buffer.memory) };

    let index_buffer = buffer::<u16>(device, vulkan::BUFFER_USAGE_TRANSFER_DST_BIT | vulkan::BUFFER_USAGE_INDEX_BUFFER_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, indices.len())?;
    let index_copy_info = vulkan::BufferCopy {
        srcOffset: 0,
        dstOffset: 0,
        size: (indices.len() * std::mem::size_of::<u16>()) as u64,
    };

    unsafe { (device.vkCmdCopyBuffer)(vertex_command_buffer, index_staging_buffer.handle, index_buffer.handle, 1, &index_copy_info as *const vulkan::BufferCopy) };

    let cursor_texture_coords: [[f32; 2]; 4] = [
        [0.0, 0.0],
        [1.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
    ];

    let cursor_vertex_staging_buffer = buffer::<[f32; 2]>(device, vulkan::BUFFER_USAGE_TRANSFER_SRC_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, cursor_texture_coords.len())?;
    let mut cursor_vertex_data: *mut [f32; 2] = std::ptr::null_mut();

    unsafe { (device.vkMapMemory)(device.handle, cursor_vertex_staging_buffer.memory, 0, (cursor_texture_coords.len() * std::mem::size_of::<[f32; 2]>()) as u64, 0, std::mem::transmute::<&mut *mut [f32; 2], *mut *mut std::ffi::c_void>(&mut cursor_vertex_data)) };


    unsafe { std::ptr::copy(cursor_texture_coords.as_ptr(), cursor_vertex_data, cursor_texture_coords.len()) };
    unsafe { (device.vkUnmapMemory)(device.handle, cursor_vertex_staging_buffer.memory) };

    let cursor_vertex_buffer = buffer::<[f32; 2]>(device, vulkan::BUFFER_USAGE_TRANSFER_DST_BIT | vulkan::BUFFER_USAGE_VERTEX_BUFFER_BIT, vulkan::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vulkan::MEMORY_PROPERTY_HOST_COHERENT_BIT, cursor_texture_coords.len())?;
    let cursor_vertex_copy_info = vulkan::BufferCopy {
        srcOffset: 0,
        dstOffset: 0,
        size: (cursor_texture_coords.len() * std::mem::size_of::<[f32; 2]>()) as u64,
    };

    unsafe { (device.vkCmdCopyBuffer)(vertex_command_buffer, cursor_vertex_staging_buffer.handle, cursor_vertex_buffer.handle, 1, &cursor_vertex_copy_info as *const vulkan::BufferCopy) };
    end_command_buffer(device, command_pool, vertex_command_buffer);

    unsafe { (device.vkFreeMemory)(device.handle, vertex_staging_buffer.memory, std::ptr::null()) };
    unsafe { (device.vkDestroyBuffer)(device.handle, vertex_staging_buffer.handle, std::ptr::null()) };
    unsafe { (device.vkFreeMemory)(device.handle, index_staging_buffer.memory, std::ptr::null()) };
    unsafe { (device.vkDestroyBuffer)(device.handle, index_staging_buffer.handle, std::ptr::null()) };
    unsafe { (device.vkFreeMemory)(device.handle, cursor_vertex_staging_buffer.memory, std::ptr::null()) };
    unsafe { (device.vkDestroyBuffer)(device.handle, cursor_vertex_staging_buffer.handle, std::ptr::null()) };

    let texture_image = create_image(
        device,
        vulkan::R8_UNORM,
        vulkan::IMAGE_USAGE_TRANSFER_DST_BIT | vulkan::IMAGE_USAGE_SAMPLED_BIT,
        vulkan::IMAGE_ASPECT_COLOR_BIT,
        font.width as u32,
        font.height as u32
    )?;
    copy_buffer_to_image(device, command_pool, texture_image.handle, &font.texture_atlas, font.width as u32, font.height as u32)?;

    let texture_sampler = create_sampler(device, vulkan::SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER);
    let texture_descriptor_set = allocate_descriptor_set(device, graphics_pipeline.global_descriptor_pool, graphics_pipeline.texture_descriptor_set_layout);
    update_image_descriptor_set(device, texture_image.view, texture_sampler, texture_descriptor_set);

    let cursor_texture_image = create_image(
        device,
        vulkan::R8G8_UNORM,
        vulkan::IMAGE_USAGE_TRANSFER_DST_BIT | vulkan::IMAGE_USAGE_SAMPLED_BIT,
        vulkan::IMAGE_ASPECT_COLOR_BIT,
        16,
        16,
    )?;

    let cursor_texture: [u8; 512] = [255; 512];
    copy_buffer_to_image(device, command_pool, cursor_texture_image.handle, &cursor_texture, 16, 16)?;
    let cursor_texture_sampler = create_sampler(device, vulkan::SAMPLER_ADDRESS_MODE_REPEAT);
    let cursor_texture_descriptor_set = allocate_descriptor_set(device, graphics_pipeline.global_descriptor_pool, graphics_pipeline.texture_descriptor_set_layout);
    update_image_descriptor_set(device, cursor_texture_image.view, cursor_texture_sampler, cursor_texture_descriptor_set);

    let command_buffer_info = vulkan::CommandBufferAllocateInfo {
        sType: vulkan::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        pNext: std::ptr::null(),
        commandPool: command_pool,
        level: vulkan::COMMAND_BUFFER_LEVEL_PRIMARY,
        commandBufferCount: count,
    };

    let mut primary_command_buffers: Vec<*mut vulkan::CommandBuffer> = Vec::with_capacity(count as usize);
    unsafe { (device.vkAllocateCommandBuffers)(device.handle, &command_buffer_info as *const vulkan::CommandBufferAllocateInfo, primary_command_buffers.as_mut_ptr() as *mut *mut vulkan::CommandBuffer) };
    unsafe { primary_command_buffers.set_len(count as usize) };

    let secondary_command_buffer_info = vulkan::CommandBufferAllocateInfo {
        sType: vulkan::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        pNext: std::ptr::null(),
        commandPool: command_pool,
        level: vulkan::COMMAND_BUFFER_LEVEL_SECONDARY,
        commandBufferCount: count * 2,
    };

    let mut secondary_command_buffers: Vec<[*mut vulkan::CommandBuffer; 2]> = Vec::with_capacity(count as usize);
    unsafe { (device.vkAllocateCommandBuffers)(device.handle, &secondary_command_buffer_info as *const vulkan::CommandBufferAllocateInfo, secondary_command_buffers.as_mut_ptr() as *mut *mut vulkan::CommandBuffer) };
    unsafe { secondary_command_buffers.set_len(count as usize) };

    let mut command_buffers = Vec::with_capacity(count as usize);
    for i in 0..count {
        command_buffers.push(CommandBuffer {
            handle: primary_command_buffers[i as usize],
            secondary: [secondary_command_buffers[i as usize][0], secondary_command_buffers[i as usize][1]],
            is_text_updated: false,
        });
    }

    Ok(Swapchain {
        handle,
        image_views,
        framebuffers,
        extent,

        texture_image,
        texture_sampler,
        texture_descriptor_set,

        cursor_texture_image,
        cursor_texture_sampler,
        cursor_texture_descriptor_set,

        uniform_descriptor_set,
        global_uniform_buffer,
        global_uniform_map,
        has_changed: true,

        command_pool,
        command_buffers,
        vertex_buffer,
        cursor_vertex_buffer,
        index_buffer,

        render_finished,
        image_available,
        in_flight,
    })
}

pub fn set_change(swapchain: &mut Swapchain) {
    swapchain.has_changed = true;

    for i in 0..swapchain.command_buffers.len() {
        swapchain.command_buffers[i].is_text_updated = false;
    }
}

fn begin_command_buffer(device: &Device, command_pool: *mut vulkan::CommandPool) -> *mut vulkan::CommandBuffer {
    let mut command_buffer: *mut vulkan::CommandBuffer = std::ptr::null_mut();
    let alloc_info = vulkan::CommandBufferAllocateInfo {
        sType: vulkan::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        pNext: std::ptr::null(),
        commandPool: command_pool,
        level: vulkan::COMMAND_BUFFER_LEVEL_PRIMARY,
        commandBufferCount: 1,
    };

    unsafe { (device.vkAllocateCommandBuffers)(device.handle, &alloc_info as *const vulkan::CommandBufferAllocateInfo, &mut command_buffer as *mut *mut vulkan::CommandBuffer) };
    let begin_info = vulkan::CommandBufferBeginInfo {
        sType: vulkan::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        flags: vulkan::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        pNext: std::ptr::null(),
        pInheritanceInfo: std::ptr::null(),
    };

    unsafe { (device.vkBeginCommandBuffer)(command_buffer, &begin_info as *const vulkan::CommandBufferBeginInfo) };

    command_buffer
}

fn end_command_buffer(device: &Device, command_pool: *mut vulkan::CommandPool, command_buffer: *mut vulkan::CommandBuffer) {
    unsafe { (device.vkEndCommandBuffer)(command_buffer) };
    let submit_info = vulkan::SubmitInfo {
        sType: vulkan::STRUCTURE_TYPE_SUBMIT_INFO,
        pNext: std::ptr::null(),
        commandBufferCount: 1,
        pCommandBuffers: &command_buffer as *const *mut vulkan::CommandBuffer,
        waitSemaphoreCount: 0,
        pWaitSemaphores: std::ptr::null(),
        pWaitDstStageMask: std::ptr::null(),
        signalSemaphoreCount: 0,
        pSignalSemaphores: std::ptr::null(),
    };

    unsafe { (device.vkQueueSubmit)(device.queues[0], 1, &submit_info as *const vulkan::SubmitInfo, std::ptr::null_mut()) };
    unsafe { (device.vkQueueWaitIdle)(device.queues[0]) };
    unsafe { (device.vkFreeCommandBuffers)(device.handle, command_pool, 1, &command_buffer as *const *mut vulkan::CommandBuffer) };
}

pub fn recreate_swapchain(device: &Device, swapchain: &mut Swapchain, graphics_pipeline: &GraphicsPipeline, width: u32, height: u32) -> Result<(), LoadError> {
    unsafe { (device.vkWaitForFences)(device.handle, 1, &swapchain.in_flight as *const *mut vulkan::Fence, vulkan::TRUE, 0xFFFFFF) };

    unsafe {
        let null = std::ptr::null();

        for image_view in swapchain.image_views.iter() {
            (device.vkDestroyImageView)(device.handle, *image_view, null);
        }

        for framebuffer in swapchain.framebuffers.iter() {
            (device.vkDestroyFramebuffer)(device.handle, *framebuffer, null);
        }

        (device.vkDestroySwapchainKHR)(device.handle, swapchain.handle, null);
    };

    let present_mode = vulkan::PRESENT_MODE_FIFO_KHR;
    swapchain.extent = if device.capabilities.currentExtent.width != 0xFFFFFFFF {
        vulkan::Extent2D {
            width: device.capabilities.currentExtent.width,
            height: device.capabilities.currentExtent.height,
        }
    } else {
        vulkan::Extent2D {
            width: width.clamp(device.capabilities.minImageExtent.width, device.capabilities.maxImageExtent.width),
            height: height.clamp(device.capabilities.minImageExtent.height, device.capabilities.maxImageExtent.height),
        }
    };

    let image_count = if device.capabilities.maxImageCount > 0 {
        std::cmp::min(device.capabilities.minImageCount + 1, device.capabilities.maxImageCount)
    } else {
        device.capabilities.minImageCount + 1
    };

    let (queue_family_index_len, sharing_mode): (u32, u32) = if device.families[0] == device.families[1] {
        (1, vulkan::SHARING_MODE_EXCLUSIVE)
    } else {
        (2, vulkan::SHARING_MODE_CONCURRENT)
    };

    let info = vulkan::SwapchainCreateInfoKHR {
        sType: vulkan::STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
        flags: 0,
        pNext: std::ptr::null(),
        surface: device.surface,
        minImageCount: image_count,
        imageFormat: graphics_pipeline.surface_format.format,
        imageColorSpace: graphics_pipeline.surface_format.colorSpace,
        imageExtent: vulkan::Extent2D {
            width: swapchain.extent.width,
            height: swapchain.extent.height,
        },
        imageSharingMode: sharing_mode,
        presentMode: present_mode,
        preTransform: device.capabilities.currentTransform,
        clipped: vulkan::TRUE,
        imageArrayLayers: 1,
        compositeAlpha: vulkan::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
        imageUsage: vulkan::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
        queueFamilyIndexCount: queue_family_index_len,
        pQueueFamilyIndices: device.families.as_ptr(),
        oldSwapchain: std::ptr::null_mut(),
    };

    unsafe { (device.vkCreateSwapchainKHR)(device.handle, &info as *const vulkan::SwapchainCreateInfoKHR, std::ptr::null(), &mut swapchain.handle as *mut *mut vulkan::SwapchainKHR)};

    let mut count: u32 = 0;
    unsafe { (device.vkGetSwapchainImagesKHR)(device.handle, swapchain.handle, &mut count as *mut u32, std::ptr::null_mut()) };
    let mut images: Vec<*mut vulkan::Image> = Vec::with_capacity(count as usize);
    unsafe { (device.vkGetSwapchainImagesKHR)(device.handle, swapchain.handle, &mut count as *mut u32, images.as_mut_ptr() as *mut *mut vulkan::Image) };
    unsafe { images.set_len(count as usize) };

    for i in 0..count {
        swapchain.image_views[i as usize] = create_image_view(device, images[i as usize], graphics_pipeline.surface_format.format, vulkan::IMAGE_ASPECT_COLOR_BIT);
    }

    for i in 0..count {
        swapchain.framebuffers[i as usize] = create_framebuffer(
            device,
            graphics_pipeline.render_pass,
            &[swapchain.image_views[i as usize]],
            swapchain.extent.width,
            swapchain.extent.height
        );
    }

    swapchain.global_uniform_map[0] = height as f32 / width as f32;

    unsafe { (device.vkDestroySemaphore)(device.handle, swapchain.image_available, std::ptr::null()) };
    let semaphore_info = vulkan::SemaphoreCreateInfo {
        sType: vulkan::STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
        flags: 0,
        pNext: std::ptr::null(),
    };

    if 0 != unsafe { (device.vkCreateSemaphore)(device.handle, &semaphore_info as *const vulkan::SemaphoreCreateInfo, std::ptr::null(), &mut swapchain.image_available as *mut *mut vulkan::Semaphore) } {
        return Err(LoadError::SyncMemberFailed);
    }

    Ok(())
}

fn record_text_secondary_command_buffer(
    device: &Device,
    command_buffer: *mut vulkan::CommandBuffer,
    vertex_buffer: *mut vulkan::Buffer,
    index_buffer: *mut vulkan::Buffer,
    uniform_descriptor_set: *mut vulkan::DescriptorSet,
    texture_descriptor_set: *mut vulkan::DescriptorSet,
    graphics_pipeline: &GraphicsPipeline,
    characters: &UniqueChars
) {
    unsafe { (device.vkCmdBindPipeline)(command_buffer, vulkan::PIPELINE_BIND_POINT_GRAPHICS, graphics_pipeline.handle) };
    unsafe { (device.vkCmdBindDescriptorSets)(command_buffer, vulkan::PIPELINE_BIND_POINT_GRAPHICS, graphics_pipeline.layout, 0, 2, [uniform_descriptor_set, texture_descriptor_set].as_ptr() as *const *mut vulkan::DescriptorSet, 0, std::ptr::null()) };
    unsafe { (device.vkCmdBindIndexBuffer)(command_buffer, index_buffer as *mut vulkan::Buffer, 0, vulkan::INDEX_TYPE_UINT16) };

    for (i, offset) in characters.offset.iter().enumerate() {
        if *offset == 255 || i == 0 {
            continue;
        }

        unsafe { (device.vkCmdBindVertexBuffers)(command_buffer, 0, 1, &vertex_buffer as *const *mut vulkan::Buffer, [(std::mem::size_of::<[f32; 2]>() * i) as u64 * 4].as_ptr()) };

        for pos in characters.positions[*offset as usize].iter() {
            unsafe { (device.vkCmdPushConstants)(command_buffer, graphics_pipeline.layout, vulkan::SHADER_STAGE_VERTEX_BIT, 0, std::mem::size_of::<f32>() as u32 * 2, std::mem::transmute::<*const f32, *const std::ffi::c_void>([pos[0] as f32 * 2.0 + 1.0, pos[1] as f32 * 2.0 + 1.0].as_ptr())) };
            unsafe { (device.vkCmdDrawIndexed)(command_buffer, 6, 1, 0, 0, 0) };
        }
    }
}

fn record_cursor_secondary_command_buffer(
    device: &Device,
    command_buffer: *mut vulkan::CommandBuffer,
    vertex_buffer: *mut vulkan::Buffer,
    index_buffer: *mut vulkan::Buffer,
    uniform_descriptor_set: *mut vulkan::DescriptorSet,
    texture_descriptor_set: *mut vulkan::DescriptorSet,
    graphics_pipeline: &GraphicsPipeline,
    cursor: &Cursor
) {
    unsafe { (device.vkCmdBindPipeline)(command_buffer, vulkan::PIPELINE_BIND_POINT_GRAPHICS, graphics_pipeline.handle) };
    unsafe { (device.vkCmdBindDescriptorSets)(command_buffer, vulkan::PIPELINE_BIND_POINT_GRAPHICS, graphics_pipeline.layout, 0, 2, [uniform_descriptor_set, texture_descriptor_set].as_ptr() as *const *mut vulkan::DescriptorSet, 0, std::ptr::null()) };
    unsafe { (device.vkCmdBindIndexBuffer)(command_buffer, index_buffer as *mut vulkan::Buffer, 0, vulkan::INDEX_TYPE_UINT16) };

    unsafe { (device.vkCmdBindVertexBuffers)(command_buffer, 0, 1, &vertex_buffer as *const *mut vulkan::Buffer, [0].as_ptr()) };
    unsafe { (device.vkCmdPushConstants)(command_buffer, graphics_pipeline.layout, vulkan::SHADER_STAGE_VERTEX_BIT, 0, std::mem::size_of::<f32>() as u32 * 2, std::mem::transmute::<*const f32, *const std::ffi::c_void>([cursor.xpos as f32 * 2.0 + 1.0, cursor.ypos as f32 * 2.0 + 1.0].as_ptr())) };
    unsafe { (device.vkCmdDrawIndexed)(command_buffer, 6, 1, 0, 0, 0) };
}

fn record_command_buffer(
    device: &Device,
    command_buffer: &mut CommandBuffer,
    framebuffer: *mut vulkan::Framebuffer,
    vertex_buffer: *mut vulkan::Buffer,
    cursor_vertex_buffer: *mut vulkan::Buffer,
    index_buffer: *mut vulkan::Buffer,
    uniform_descriptor_set: *mut vulkan::DescriptorSet,
    texture_descriptor_set: *mut vulkan::DescriptorSet,
    cursor_texture_descriptor_set: *mut vulkan::DescriptorSet,
    extent: &vulkan::Extent2D,
    graphics_pipeline: &GraphicsPipeline,
    characters: &mut UniqueChars,
    cursor: &Cursor,
) {
    let begin_info = vulkan::CommandBufferBeginInfo {
        sType: vulkan::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        pInheritanceInfo: std::ptr::null(),
    };

    unsafe { (device.vkBeginCommandBuffer)(command_buffer.handle, &begin_info as *const vulkan::CommandBufferBeginInfo) };

    let clear_values = [
        vulkan::ClearValue {
            color: vulkan::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        },
    ];

    let render_pass_info = vulkan::RenderPassBeginInfo {
        sType: vulkan::STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
        pNext: std::ptr::null(),
        renderPass: graphics_pipeline.render_pass,
        framebuffer,
        renderArea: vulkan::Rect2D {
            offset: vulkan::Offset2D {
                x: 0,
                y: 0,
            },
            extent: vulkan::Extent2D {
                width: extent.width,
                height: extent.height,
            },
        },
        clearValueCount: clear_values.len() as u32,
        pClearValues: clear_values.as_ptr() as *const vulkan::ClearValue
    };

    unsafe { (device.vkCmdBeginRenderPass)(command_buffer.handle, &render_pass_info as *const vulkan::RenderPassBeginInfo, vulkan::SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS) };

    let viewport = vulkan::Viewport {
        x: 0.0,
        y: 0.0,
        width: extent.width as f32,
        height: extent.height as f32,
        minDepth: 0.0,
        maxDepth: 1.0,
    };

    let scissor = vulkan::Rect2D {
        offset: vulkan::Offset2D {
            x: 0,
            y: 0,
        },
        extent: vulkan::Extent2D {
            width: extent.width,
            height: extent.height,
        }
    };

    let inheritance_info = vulkan::CommandBufferInheritanceInfo {
        sType: vulkan::STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
        renderPass: graphics_pipeline.render_pass,
        subpass: 0,
        framebuffer,
        pNext: std::ptr::null(),
        occlusionQueryEnable: 0,
        queryFlags: 0,
        pipelineStatistics: 0,
    };

    let secondary_command_buffer_begin_info = vulkan::CommandBufferBeginInfo {
        sType: vulkan::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        pNext: std::ptr::null(),
        flags: vulkan::COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT,
        pInheritanceInfo: &inheritance_info as *const vulkan::CommandBufferInheritanceInfo,
    };

    if !command_buffer.is_text_updated {
        unsafe { (device.vkBeginCommandBuffer)(command_buffer.secondary[0], &secondary_command_buffer_begin_info as *const vulkan::CommandBufferBeginInfo) };
        unsafe { (device.vkCmdSetViewport)(command_buffer.secondary[0], 0, 1, &viewport as *const vulkan::Viewport) };
        unsafe { (device.vkCmdSetScissor)(command_buffer.secondary[0], 0, 1, &scissor as *const vulkan::Rect2D) };

        record_text_secondary_command_buffer(
            device,
            command_buffer.secondary[0],
            vertex_buffer,
            index_buffer,
            uniform_descriptor_set,
            texture_descriptor_set,
            graphics_pipeline,
            characters
        );

        unsafe { (device.vkEndCommandBuffer)(command_buffer.secondary[0]) };

        command_buffer.is_text_updated = true;
    }

    unsafe { (device.vkBeginCommandBuffer)(command_buffer.secondary[1], &secondary_command_buffer_begin_info as *const vulkan::CommandBufferBeginInfo) };
    unsafe { (device.vkCmdSetViewport)(command_buffer.secondary[1], 0, 1, &viewport as *const vulkan::Viewport) };
    unsafe { (device.vkCmdSetScissor)(command_buffer.secondary[1], 0, 1, &scissor as *const vulkan::Rect2D) };

    record_cursor_secondary_command_buffer(
        device,
        command_buffer.secondary[1],
        cursor_vertex_buffer,
        index_buffer,
        uniform_descriptor_set,
        cursor_texture_descriptor_set,
        // texture_descriptor_set,
        graphics_pipeline,
        cursor,
    );

    unsafe { (device.vkEndCommandBuffer)(command_buffer.secondary[1]) };

    unsafe { (device.vkCmdExecuteCommands)(command_buffer.handle, 2, command_buffer.secondary.as_ptr() as *const *mut vulkan::CommandBuffer) };
    unsafe { (device.vkCmdEndRenderPass)(command_buffer.handle) };
    unsafe { (device.vkEndCommandBuffer)(command_buffer.handle) };
}

fn acquire_next_image(device: &Device, swapchain: &Swapchain) -> Result<u32, LoadError> {
    let mut image_index: u32 = 0;
    let result = unsafe { (device.vkAcquireNextImageKHR)(device.handle, swapchain.handle, 0xFFFFFF, swapchain.image_available, std::ptr::null_mut(), &mut image_index as *mut u32) };

    if result == vulkan::SUBOPTIMAL_KHR || result == vulkan::OUT_OF_DATE_KHR {
        return Err(LoadError::SwapchainHasToRecreate);
    } else if result != 0 {
        return Err(LoadError::ImageFail);
    }

    Ok(image_index)
}

pub fn draw_frame(
    device: &Device,
    swapchain: &mut Swapchain,
    graphics_pipeline: &GraphicsPipeline,
    characters: &mut UniqueChars,
    cursor: &Cursor,
    width: u32,
    height: u32
) -> Result<(), LoadError> {

    if swapchain.has_changed {
        unsafe { (device.vkWaitForFences)(device.handle, 1, &swapchain.in_flight as *const *mut vulkan::Fence, vulkan::TRUE, 0xFFFFFF) };
    } else {
        std::thread::sleep(std::time::Duration::from_millis(17));
        return Ok(())
    }

    let mut image_index = 0;
    let mut has_to_recreate = swapchain.extent.width != width || swapchain.extent.height != height;
    match acquire_next_image(device, swapchain) {
        Err(LoadError::SwapchainHasToRecreate) => has_to_recreate = true,
        Ok(index) => image_index = index,
        _ => return Err(LoadError::ImageFail),
    }

    if has_to_recreate {
        recreate_swapchain(device, swapchain, graphics_pipeline, width, height)?;
        image_index = acquire_next_image(device, swapchain)?;
    }

    record_command_buffer(
        device,
        &mut swapchain.command_buffers[image_index as usize],
        swapchain.framebuffers[image_index as usize],
        swapchain.vertex_buffer.handle,
        swapchain.cursor_vertex_buffer.handle,
        swapchain.index_buffer.handle,
        swapchain.uniform_descriptor_set,
        swapchain.texture_descriptor_set,
        swapchain.cursor_texture_descriptor_set,
        &swapchain.extent,
        graphics_pipeline,
        characters,
        cursor,
    );

    unsafe { (device.vkResetFences)(device.handle, 1, &swapchain.in_flight as *const *mut vulkan::Fence) };

    let submit_info = vulkan::SubmitInfo {
        sType: vulkan::STRUCTURE_TYPE_SUBMIT_INFO,
        pNext: std::ptr::null(),
        waitSemaphoreCount: 1,
        pWaitSemaphores: [swapchain.image_available].as_ptr() as *const *mut vulkan::Semaphore,
        pWaitDstStageMask: [vulkan::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT].as_ptr() as *const u32,
        commandBufferCount: 1,
        pCommandBuffers: &swapchain.command_buffers[image_index as usize].handle as *const *mut vulkan::CommandBuffer,
        signalSemaphoreCount: 1,
        pSignalSemaphores: [swapchain.render_finished].as_ptr() as *const *mut vulkan::Semaphore
    };

    unsafe { (device.vkQueueSubmit)(device.queues[0], 1, &submit_info as *const vulkan::SubmitInfo, swapchain.in_flight) };

    let present_info = vulkan::PresentInfoKHR {
        sType: vulkan::STRUCTURE_TYPE_PRESENT_INFO_KHR,
        pNext: std::ptr::null(),
        swapchainCount: 1,
        pSwapchains: [swapchain.handle].as_ptr() as *const *mut vulkan::SwapchainKHR,
        waitSemaphoreCount: 1,
        pWaitSemaphores: [swapchain.render_finished].as_ptr() as *const *mut vulkan::Semaphore,
        pImageIndices: &image_index as *const u32,
        pResults: std::ptr::null_mut(),
    };

    unsafe { (device.vkQueuePresentKHR)(device.queues[device.families[1] as usize], &present_info as *const vulkan::PresentInfoKHR) };

    swapchain.has_changed = false;
    Ok(())
}

pub fn shutdown_swapchain(device: &Device, swapchain: &Swapchain) {
    unsafe { (device.vkWaitForFences)(device.handle, 1, &swapchain.in_flight as *const *mut vulkan::Fence, vulkan::TRUE, 0xFFFFFF) };
    unsafe {
        let null = std::ptr::null();

        for image_view in swapchain.image_views.iter() {
            (device.vkDestroyImageView)(device.handle, *image_view, null);
        }

        for framebuffer in swapchain.framebuffers.iter() {
            (device.vkDestroyFramebuffer)(device.handle, *framebuffer, null);
        }

        (device.vkFreeMemory)(device.handle, swapchain.vertex_buffer.memory, null);
        (device.vkFreeMemory)(device.handle, swapchain.cursor_vertex_buffer.memory, null);
        (device.vkFreeMemory)(device.handle, swapchain.index_buffer.memory, null);
        (device.vkFreeMemory)(device.handle, swapchain.texture_image.memory, null);
        (device.vkFreeMemory)(device.handle, swapchain.cursor_texture_image.memory, null);
        (device.vkFreeMemory)(device.handle, swapchain.global_uniform_buffer.memory, null);
        (device.vkDestroyBuffer)(device.handle, swapchain.vertex_buffer.handle, null);
        (device.vkDestroyBuffer)(device.handle, swapchain.cursor_vertex_buffer.handle, null);
        (device.vkDestroyBuffer)(device.handle, swapchain.index_buffer.handle, null);
        (device.vkDestroyBuffer)(device.handle, swapchain.global_uniform_buffer.handle, null);
        (device.vkDestroySemaphore)(device.handle, swapchain.render_finished, null);
        (device.vkDestroySemaphore)(device.handle, swapchain.image_available, null);
        (device.vkDestroyFence)(device.handle, swapchain.in_flight, null);
        (device.vkDestroyCommandPool)(device.handle, swapchain.command_pool, null);
        (device.vkDestroyImageView)(device.handle, swapchain.texture_image.view, null);
        (device.vkDestroyImageView)(device.handle, swapchain.cursor_texture_image.view, null);
        (device.vkDestroyImage)(device.handle, swapchain.texture_image.handle, null);
        (device.vkDestroyImage)(device.handle, swapchain.cursor_texture_image.handle, null);
        (device.vkDestroySampler)(device.handle, swapchain.texture_sampler, null);
        (device.vkDestroySampler)(device.handle, swapchain.cursor_texture_sampler, null);
        (device.vkDestroySwapchainKHR)(device.handle, swapchain.handle, null);
    };
}

pub fn shutdown_graphics_pipeline(device: &Device, graphics_pipeline: &GraphicsPipeline) {
    unsafe {
        let null = std::ptr::null();

        (device.vkDestroyDescriptorSetLayout)(device.handle, graphics_pipeline.global_descriptor_set_layout, null);
        (device.vkDestroyDescriptorSetLayout)(device.handle, graphics_pipeline.texture_descriptor_set_layout, null);
        (device.vkDestroyDescriptorPool)(device.handle, graphics_pipeline.global_descriptor_pool, null);
        (device.vkDestroyDescriptorPool)(device.handle, graphics_pipeline.texture_descriptor_pool, null);
        (device.vkDestroyPipelineLayout)(device.handle, graphics_pipeline.layout, null);
        (device.vkDestroyRenderPass)(device.handle, graphics_pipeline.render_pass, null);
        (device.vkDestroyPipeline)(device.handle, graphics_pipeline.handle, null);
    };
}

pub fn shutdown_device(device: &Device) {
    unsafe {
        let null = std::ptr::null();

        (device.vkDestroyDevice)(device.handle, null);
    };
}

pub fn shutdown_surface(dispatch: &Instance, surface: *mut vulkan::SurfaceKHR) {
    unsafe {
        let null = std::ptr::null();

        (dispatch.vkDestroySurfaceKHR)(dispatch.handle, surface, null);
    };
}

pub fn shutdown_instance(dispatch: &Instance) {
    unsafe {
        let null = std::ptr::null();

        (dispatch.vkDestroyInstance)(dispatch.handle, null);
    };
}
