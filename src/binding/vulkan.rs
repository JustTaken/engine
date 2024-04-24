#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports)]

use crate::binding::wayland;

use std::ffi::c_void as void;

pub const STRUCTURE_TYPE_APPLICATION_INFO: u32 = 0;
pub const STRUCTURE_TYPE_INSTANCE_CREATE_INFO: u32 = 1;
pub const STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: u32 = 1000006000;

pub const QUEUE_GRAPHICS_BIT: u32 = 1;
pub const QUEUE_COMPUTE_BIT: u32 = 2;
pub const QUEUE_TRANSFER_BIT: u32 = 4;

#[repr(C)]
pub struct ApplicationInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub pApplicationName: *const i8,
    pub applicationVersion: u32,
    pub pEngineName: *const i8,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[repr(C)]
pub struct WaylandSurfaceCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub display: *const wayland::wl_display,
    pub surface: *const wayland::wl_surface,
}

#[repr(C)]
pub struct AllocationCallbacks {
    pub pUserData: *mut void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}

#[repr(C)]
pub struct InstanceCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub pApplicationInfo: *const ApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const i8,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const i8,
}

#[repr(C)]
pub struct DeviceCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const DeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const i8,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const i8,
    pub pEnabledFeatures: *const PhysicalDeviceFeatures,
}

#[repr(C)]
pub struct SurfaceKHR {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct PhysicalDeviceFeatures {
    pub robustBufferAccess: u32,
    pub fullDrawIndexUint32: u32,
    pub imageCubeArray: u32,
    pub independentBlend: u32,
    pub geometryShader: u32,
    pub tessellationShader: u32,
    pub sampleRateShading: u32,
    pub dualSrcBlend: u32,
    pub logicOp: u32,
    pub multiDrawIndirect: u32,
    pub drawIndirectFirstInstance: u32,
    pub depthClamp: u32,
    pub depthBiasClamp: u32,
    pub fillModeNonSolid: u32,
    pub depthBounds: u32,
    pub wideLines: u32,
    pub largePoints: u32,
    pub alphaToOne: u32,
    pub multiViewport: u32,
    pub samplerAnisotropy: u32,
    pub textureCompressionETC2: u32,
    pub textureCompressionASTC_LDR: u32,
    pub textureCompressionBC: u32,
    pub occlusionQueryPrecise: u32,
    pub pipelineStatisticsQuery: u32,
    pub vertexPipelineStoresAndAtomics: u32,
    pub fragmentStoresAndAtomics: u32,
    pub shaderTessellationAndGeometryPointSize: u32,
    pub shaderImageGatherExtended: u32,
    pub shaderStorageImageExtendedFormats: u32,
    pub shaderStorageImageMultisample: u32,
    pub shaderStorageImageReadWithoutFormat: u32,
    pub shaderStorageImageWriteWithoutFormat: u32,
    pub shaderUniformBufferArrayDynamicIndexing: u32,
    pub shaderSampledImageArrayDynamicIndexing: u32,
    pub shaderStorageBufferArrayDynamicIndexing: u32,
    pub shaderStorageImageArrayDynamicIndexing: u32,
    pub shaderClipDistance: u32,
    pub shaderCullDistance: u32,
    pub shaderFloat64: u32,
    pub shaderInt64: u32,
    pub shaderInt16: u32,
    pub shaderResourceResidency: u32,
    pub shaderResourceMinLod: u32,
    pub sparseBinding: u32,
    pub sparseResidencyBuffer: u32,
    pub sparseResidencyImage2D: u32,
    pub sparseResidencyImage3D: u32,
    pub sparseResidency2Samples: u32,
    pub sparseResidency4Samples: u32,
    pub sparseResidency8Samples: u32,
    pub sparseResidency16Samples: u32,
    pub sparseResidencyAliased: u32,
    pub variableMultisampleRate: u32,
    pub inheritedQueries: u32,
}

#[repr(C)]
pub struct DeviceQueueCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const f32,
}

#[repr(C)]
pub struct Instance {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct PhysicalDevice {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct Device {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct ExtensionProperties {
    pub extensionName: [std::os::raw::c_char; 256],
    pub specVersion: u32,
}

#[repr(C)]
pub struct VkMemoryType {
    pub propertyFlags: u32,
    pub heapIndex: u32,
}

#[repr(C)]
pub struct VkMemoryHeap {
    pub size: u64,
    pub flags: u32,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; 32],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; 16],
}

#[repr(C)]
pub struct SurfaceFormatKHR {
    pub format: u32,
    pub colorSpace: u32,
}

#[repr(C)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
pub struct QueueFamilyProperties {
    pub queueFlags: u32,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: Extent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: Extent2D,
    pub minImageExtent: Extent2D,
    pub maxImageExtent: Extent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: u32,
    pub currentTransform: u32,
    pub supportedCompositeAlpha: u32,
    pub supportedUsageFlags: u32,
}

pub type PFN_vkVoidFunction = Option<unsafe extern "C" fn()>;

pub type vkReallocationFunction = unsafe extern "C" fn(
    pUserData: *mut void,
    pOriginal: *mut void,
    size: usize,
    alignment: usize,
    allocationScope: u32,
) -> *mut void;
pub type PFN_vkReallocationFunction = Option<vkReallocationFunction>;
pub type vkAllocationFunction = unsafe extern "C" fn(
    pUserData: *mut void,
    size: usize,
    alignment: usize,
    allocationScope: u32,
) -> *mut void;
pub type PFN_vkAllocationFunction = Option<vkAllocationFunction>;
pub type vkFreeFunction = unsafe extern "C" fn(
    pUserData: *mut void,
    pMemory: *mut void,
);
pub type PFN_vkFreeFunction = Option<vkFreeFunction>;
pub type vkInternalFreeNotification = unsafe extern "C" fn(
    pUserData: *mut void,
    size: usize,
    allocationType: u32,
    allocationScope: u32,
);
pub type PFN_vkInternalFreeNotification = Option<vkInternalFreeNotification>;
pub type vkInternalAllocationNotification = unsafe extern "C" fn(
    pUserData: *mut void,
    size: usize,
    allocationType: u32,
    allocationScope: u32,
);
pub type PFN_vkInternalAllocationNotification = Option<vkInternalAllocationNotification>;
pub type vkGetInstanceProcAddr = unsafe extern "C" fn(
    instance: *mut Instance,
    pName: *const i8,
) -> PFN_vkVoidFunction;
pub type PFN_vkGetInstanceProcAddr = Option<vkGetInstanceProcAddr>;
pub type vkGetDeviceProcAddr = unsafe extern "C" fn(
    device: *mut Device,
    pName: *const i8,
) -> PFN_vkVoidFunction;
pub type PFN_vkGetDeviceProcAddr = Option<vkGetDeviceProcAddr>;
pub type vkCreateDevice = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    pCreateInfo: *const DeviceCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pDevice: *mut Device,
) -> u32;
pub type PFN_vkCreateDevice = Option<vkCreateDevice>;
pub type vkCreateInstance = unsafe extern "C" fn(
    pCreateInfo: *const InstanceCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pInstance: *mut *mut Instance,
) -> u32;
pub type PFN_vkCreateInstance = Option<vkCreateInstance>;
pub type vkDestroyInstance = unsafe extern "C" fn(
    instance: *mut Instance,
    pAllocator: *const AllocationCallbacks
);
pub type PFN_vkDestroyInstance = Option<vkDestroyInstance>;
pub type vkDestroySurfaceKHR = unsafe extern "C" fn(
    instance: *mut Instance,
    surface: *mut SurfaceKHR,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroySurfaceKHR = std::option::Option<vkDestroySurfaceKHR>;
pub type vkCreateWaylandSurfaceKHR = unsafe extern "C" fn(
    instance: *mut Instance,
    pCreateInfo: *const WaylandSurfaceCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pSurface: *mut *mut SurfaceKHR,
) -> u32;
pub type PFN_vkCreateWaylandSurfaceKHR = Option<vkCreateWaylandSurfaceKHR>;
pub type vkEnumeratePhsysicalDevices = unsafe extern "C" fn(
    instance: *mut Instance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut *mut PhysicalDevice,
) -> u32;
pub type PFN_vkEnumeratePhysicalDevices = std::option::Option<vkEnumeratePhsysicalDevices>;
pub type vkEnumerateDeviceExtensionProperties = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    pLayerName: *const std::ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut ExtensionProperties,
) -> u32;
pub type PFN_vkEnumerateDeviceExtensionProperties = std::option::Option<vkEnumerateDeviceExtensionProperties>;
pub type vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    surface: *mut SurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut SurfaceFormatKHR,
) -> u32;
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = std::option::Option<vkGetPhysicalDeviceSurfaceFormatsKHR>;
pub type vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    surface: *mut SurfaceKHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut u32,
) -> u32;
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = std::option::Option<vkGetPhysicalDeviceSurfacePresentModesKHR>;
pub type vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut QueueFamilyProperties,
);
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = std::option::Option<vkGetPhysicalDeviceQueueFamilyProperties>;
pub type vkGetPhysicalDeviceMemoryProperties = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    pMemoryProperties: *mut PhysicalDeviceMemoryProperties,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties = std::option::Option<vkGetPhysicalDeviceMemoryProperties>;
pub type vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    queueFamilyIndex: u32,
    surface: *mut SurfaceKHR,
    pSupported: *mut u32,
) -> u32;
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = std::option::Option<vkGetPhysicalDeviceSurfaceSupportKHR>;
pub type vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    surface: *mut SurfaceKHR,
    pSurfaceCapabilities: *mut SurfaceCapabilitiesKHR,
) -> u32;
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = std::option::Option<vkGetPhysicalDeviceSurfaceCapabilitiesKHR>;
