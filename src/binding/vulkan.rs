#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports)]

use crate::binding::wayland;

use std::ffi::c_void as void;

pub const STRUCTURE_TYPE_APPLICATION_INFO: u32 = 0;
pub const STRUCTURE_TYPE_INSTANCE_CREATE_INFO: u32 = 1;
pub const STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: u32 = 1000006000;

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
pub struct Surface {
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

pub type PFN_vkVoidFunction = Option<unsafe extern "C" fn()>;

pub type PFN_vkReallocationFunction = Option<
    unsafe extern "C" fn(
        pUserData: *mut void,
        pOriginal: *mut void,
        size: usize,
        alignment: usize,
        allocationScope: u32,
    ) -> *mut void,
>;
pub type PFN_vkAllocationFunction = Option<
    unsafe extern "C" fn(
        pUserData: *mut void,
        size: usize,
        alignment: usize,
        allocationScope: u32,
    ) -> *mut void,
>;
pub type PFN_vkFreeFunction = Option<
    unsafe extern "C" fn(
        pUserData: *mut void,
        pMemory: *mut void,
    ),
>;
pub type PFN_vkInternalFreeNotification = Option<
    unsafe extern "C" fn(
        pUserData: *mut void,
        size: usize,
        allocationType: u32,
        allocationScope: u32,
    ),
>;
pub type PFN_vkInternalAllocationNotification = Option<
    unsafe extern "C" fn(
        pUserData: *mut void,
        size: usize,
        allocationType: u32,
        allocationScope: u32,
    ),
>;
pub type PFN_vkGetInstanceProcAddr = Option<
    unsafe extern "C" fn(
        instance: *mut Instance,
        pName: *const i8,
    ) -> PFN_vkVoidFunction,
>;
pub type PFN_vkGetDeviceProcAddr = Option<
    unsafe extern "C" fn(
        device: *mut Device,
        pName: *const i8,
    ) -> PFN_vkVoidFunction,
>;
pub type PFN_vkCreateDevice = Option<
    unsafe extern "C" fn(
        physicalDevice: *mut PhysicalDevice,
        pCreateInfo: *const DeviceCreateInfo,
        pAllocator: *const AllocationCallbacks,
        pDevice: *mut Device,
    ) -> u32,
>;
pub type PFN_vkCreateInstance = Option<
    unsafe extern "C" fn(
        pCreateInfo: *const InstanceCreateInfo,
        pAllocator: *const AllocationCallbacks,
        pInstance: *mut *mut Instance,
    ) -> u32,
>;
pub type PFN_vkDestroyInstance = Option<
    unsafe extern "C" fn(
        instance: *mut Instance,
        pAllocator: *const AllocationCallbacks
    ),
>;
pub type PFN_vkCreateWaylandSurfaceKHR = Option<
    unsafe extern "C" fn(
        instance: *mut Instance,
        pCreateInfo: *const WaylandSurfaceCreateInfo,
        pAllocator: *const AllocationCallbacks,
        pSurface: *mut *mut Surface,
    ) -> u32,
>;
