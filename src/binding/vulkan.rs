#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports)]

use crate::binding::wayland;

use std::ffi::c_void as void;

pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;

pub const STRUCTURE_TYPE_APPLICATION_INFO: u32 = 0;
pub const STRUCTURE_TYPE_INSTANCE_CREATE_INFO: u32 = 1;
pub const STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: u32 = 2;
pub const STRUCTURE_TYPE_DEVICE_CREATE_INFO: u32 = 3;
pub const STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: u32 = 16;
pub const STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: u32 = 18;
pub const STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: u32 = 19;
pub const STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: u32 = 20;
pub const STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: u32 = 22;
pub const STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: u32 = 23;
pub const STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: u32 = 24;
pub const STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: u32 = 25;
pub const STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: u32 = 26;
pub const STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: u32 = 27;
pub const STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: u32 = 32;
pub const STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: u32 = 30;
pub const STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: u32 = 33;
pub const STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: u32 = 38;
pub const STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: u32 = 1000006000;
pub const STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: u32 = 15;
pub const STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: u32 = 28;
pub const STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: u32 = 1000001000;
pub const STRUCTURE_TYPE_IMAGE_CREATE_INFO: u32 = 14;
pub const STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: u32 = 5;
pub const STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: u32 = 37;
pub const STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: u32 = 39;
pub const STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: u32 = 40;
pub const STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: u32 = 42;
pub const STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: u32 = 43;
pub const STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: u32 = 9;
pub const STRUCTURE_TYPE_FENCE_CREATE_INFO: u32 = 8;
pub const STRUCTURE_TYPE_SUBMIT_INFO: u32 = 4;
pub const STRUCTURE_TYPE_PRESENT_INFO_KHR: u32 = 1000001001;
pub const STRUCTURE_TYPE_BUFFER_CREATE_INFO: u32 = 12;
pub const STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: u32 = 45;
pub const STRUCTURE_TYPE_SAMPLER_CREATE_INFO: u32 = 31;
pub const STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: u32 = 34;
pub const STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: u32 = 35;

pub const QUEUE_FAMILY_IGNORED: u32 = 0;
pub const IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: u32 = 7;

pub const FILTER_LINEAR: u32 = 1;

pub const SAMPLER_ADDRESS_MODE_REPEAT: u32 = 0;
pub const SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: u32 = 3;

pub const BORDER_COLOR_INT_OPAQUE_BLACK: u32 = 3;
pub const COMPARE_OP_ALWAYS: u32 = 7;
pub const DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: u32 = 1;
pub const SAMPLER_MIPMAP_MODE_LINEAR: u32 = 1;

pub const SUBOPTIMAL_KHR: i32 = 1000001003;
pub const OUT_OF_DATE_KHR: i32 = -1000001004;

pub const ACCESS_TRANSFER_READ_BIT: u32 = 2048;
pub const ACCESS_TRANSFER_WRITE_BIT: u32 = 4096;

pub const PIPELINE_STAGE_TOP_OF_PIPE_BIT: u32 = 1;
pub const PIPELINE_STAGE_TRANSFER_BIT: u32 = 4096;
pub const IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: u32 = 5;
pub const PIPELINE_STAGE_FRAGMENT_SHADER_BIT: u32 = 128;
pub const ACCESS_SHADER_READ_BIT: u32 = 32;

pub const MEMORY_PROPERTY_HOST_VISIBLE_BIT: u32 = 2;
pub const MEMORY_PROPERTY_HOST_COHERENT_BIT: u32 = 4;
pub const MEMORY_PROPERTY_DEVICE_LOCAL_BIT: u32 = 1;

pub const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: u32 = 2;
pub const COMMAND_BUFFER_LEVEL_PRIMARY: u32 = 0;
pub const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: u32 = 1;

pub const FENCE_CREATE_SIGNALED_BIT: u32 = 1;
pub const SUBPASS_CONTENTS_INLINE: u32 = 0;

pub const IMAGE_USAGE_TRANSFER_SRC_BIT: u32 = 1;
pub const IMAGE_USAGE_TRANSFER_DST_BIT: u32 = 2;
pub const BUFFER_USAGE_TRANSFER_DST_BIT: u32 = 2;
pub const BUFFER_USAGE_INDEX_BUFFER_BIT: u32 = 64;
pub const INDEX_TYPE_UINT16: u32 = 0;
pub const IMAGE_USAGE_SAMPLED_BIT: u32 = 4;
pub const IMAGE_USAGE_COLOR_ATTACHMENT_BIT: u32 = 16;
pub const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: u32 = 32;

pub const IMAGE_TILING_OPTIMAL: u32 = 0;
pub const IMAGE_ASPECT_DEPTH_BIT: u32 = 2;
pub const IMAGE_TYPE_2D: u32 = 1;

pub const IMAGE_VIEW_TYPE_2D: u32 = 1;
pub const IMAGE_ASPECT_COLOR_BIT: u32 = 1;
pub const COMPOSITE_ALPHA_OPAQUE_BIT_KHR: u32 = 1;
pub const COMPONENT_SWIZZLE_IDENTITY: u32 = 0;

pub const SHARING_MODE_EXCLUSIVE: u32 = 0;
pub const SHARING_MODE_CONCURRENT: u32 = 1;

pub const BUFFER_USAGE_TRANSFER_SRC_BIT: u32 = 1;
pub const BUFFER_USAGE_UNIFORM_BUFFER_BIT: u32 = 16;
pub const BUFFER_USAGE_VERTEX_BUFFER_BIT: u32 = 128;

pub const PRESENT_MODE_FIFO_KHR: u32 = 2;

pub const DESCRIPTOR_TYPE_UNIFORM_BUFFER: u32 = 6;
pub const DEPENDENCY_BY_REGION_BIT: u32 = 1;

pub const BLEND_FACTOR_ZERO: u32 = 0;
pub const BLEND_FACTOR_ONE: u32 = 1;
pub const BLEND_OP_ADD: u32 = 0;

pub const COMPARE_OP_LESS: u32 = 1;
pub const LOGIC_OP_COPY: u32 = 3;

pub const COLOR_COMPONENT_R_BIT: u32 = 1;
pub const COLOR_COMPONENT_G_BIT: u32 = 2;
pub const COLOR_COMPONENT_B_BIT: u32 = 4;
pub const COLOR_COMPONENT_A_BIT: u32 = 8;

pub const SAMPLE_COUNT_1_BIT: u32 = 1;
pub const PIPELINE_BIND_POINT_GRAPHICS: u32 = 0;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: u32 = 3;
pub const COLOR_SPACE_SRGB_NONLINEAR_KHR: u32 = 0;
pub const FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: u32 = 512;
pub const VERTEX_INPUT_RATE: u32 = 0;

pub const ATTACHMENT_LOAD_OP_CLEAR: u32 = 1;
pub const ATTACHMENT_LOAD_OP_DONT_CARE: u32 = 2;
pub const ATTACHMENT_STORE_OP_STORE: u32 = 0;
pub const ATTACHMENT_STORE_OP_DONT_CARE: u32 = 1;

pub const IMAGE_LAYOUT_UNDEFINED: u32 = 0;
pub const IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: u32 = 2;
pub const IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: u32 = 3;
pub const IMAGE_LAYOUT_PRESENT_SRC_KHR: u32 = 1000001002;

pub const SUBPASS_EXTERNAL: u32 = 0;
pub const PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: u32 = 1024;
pub const PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: u32 = 256;
pub const ACCESS_COLOR_ATTACHMENT_WRITE_BIT: u32 = 256;
pub const ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: u32 = 1024;

pub const POLYGON_MODE_FILL: u32 = 0;
pub const FRONT_FACE_CLOCKWISE: u32 = 1;
pub const FRONT_FACE_COUNTER_CLOCKWISE: u32 = 0;
pub const CULL_MODE_BACK_BIT: u32 = 2;

pub const R8_UNORM: u32 = 9;
pub const R8G8_UNORM: u32 = 16;
pub const R8G8B8A8_SRGB: u32 = 43;
pub const R32G32B32_SFLOAT: u32 = 106;
pub const R32G32_SFLOAT: u32 = 103;

pub const D32_SFLOAT: u32 = 126;
pub const D24_UNORM_S8_UINT: u32 = 129;
pub const D32_SFLOAT_S8_UINT: u32 = 130;

pub const DYNAMIC_STATE_VIEWPORT: u32 = 0;
pub const DYNAMIC_STATE_SCISSOR: u32 = 1;

pub const SHADER_STAGE_VERTEX_BIT: u32 = 1;
pub const SHADER_STAGE_FRAGMENT_BIT: u32 = 16;

pub const QUEUE_GRAPHICS_BIT: u32 = 1;
pub const QUEUE_COMPUTE_BIT: u32 = 2;
pub const QUEUE_TRANSFER_BIT: u32 = 4;

pub const PHYSICAL_DEVICE_TYPE_OTHER: u32= 0;
pub const PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: u32= 1;
pub const PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: u32= 2;
pub const PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: u32= 3;
pub const PHYSICAL_DEVICE_TYPE_CPU: u32= 4;

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
pub struct PushConstantRange {
    pub stageFlags: u32,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
pub struct PipelineLayoutCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const *mut DescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const PushConstantRange,
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
pub struct SurfaceKHR {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct PipelineLayout {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct DescriptorSetLayout {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct PipelineCache {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct Image {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct Pipeline {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct SwapchainKHR {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct SwapchainCreateInfoKHR {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub surface: *mut SurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: u32,
    pub imageColorSpace: u32,
    pub imageExtent: Extent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: u32,
    pub imageSharingMode: u32,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: u32,
    pub compositeAlpha: u32,
    pub presentMode: u32,
    pub clipped: u32,
    pub oldSwapchain: *mut SwapchainKHR,
}

#[repr(C)]
pub struct PipelineShaderStageCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub stage: u32,
    pub module: *mut ShaderModule,
    pub pName: *const i8,
    pub pSpecializationInfo: *const SpecializationInfo,
}

#[repr(C)]
pub struct PipelineTessellationStateCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub patchControlPoints: u32,
}

#[repr(C)]
pub struct SubmitInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const *mut Semaphore,
    pub pWaitDstStageMask: *const u32,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const *mut CommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const *mut Semaphore,
}

#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub stageCount: u32,
    pub pStages: *const PipelineShaderStageCreateInfo,
    pub pVertexInputState: *const PipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const PipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const PipelineTessellationStateCreateInfo,
    pub pViewportState: *const PipelineViewportStateCreateInfo,
    pub pRasterizationState: *const PipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const PipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const PipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const PipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const PipelineDynamicStateCreateInfo,
    pub layout: *mut PipelineLayout,
    pub renderPass: *mut RenderPass,
    pub subpass: u32,
    pub basePipelineHandle: *mut Pipeline,
    pub basePipelineIndex: i32,
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
pub struct PhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: u64,
    pub sparseAddressSpaceSize: u64,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3usize],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3usize],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2usize],
    pub viewportBoundsRange: [f32; 2usize],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: usize,
    pub minTexelBufferOffsetAlignment: u64,
    pub minUniformBufferOffsetAlignment: u64,
    pub minStorageBufferOffsetAlignment: u64,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: u32,
    pub framebufferDepthSampleCounts: u32,
    pub framebufferStencilSampleCounts: u32,
    pub framebufferNoAttachmentsSampleCounts: u32,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: u32,
    pub sampledImageIntegerSampleCounts: u32,
    pub sampledImageDepthSampleCounts: u32,
    pub sampledImageStencilSampleCounts: u32,
    pub storageImageSampleCounts: u32,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: u32,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2],
    pub lineWidthRange: [f32; 2],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: u32,
    pub standardSampleLocations: u32,
    pub optimalBufferCopyOffsetAlignment: u64,
    pub optimalBufferCopyRowPitchAlignment: u64,
    pub nonCoherentAtomSize: u64,
}

#[repr(C)]
pub struct ShaderModule {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct SpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: usize,
}

#[repr(C)]
pub struct Framebuffer {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct FramebufferCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub renderPass: *mut RenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const *mut ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[repr(C)]
pub struct SpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const SpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const void,
}

#[repr(C)]
pub struct ShaderModuleCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub codeSize: usize,
    pub pCode: *const u32,
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
pub struct PhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: u32,
    pub residencyStandard2DMultisampleBlockShape: u32,
    pub residencyStandard3DBlockShape: u32,
    pub residencyAlignedMipSize: u32,
    pub residencyNonResidentStrict: u32,
}

#[repr(C)]
pub struct PhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: u32,
    pub deviceName: [i8; 256],
    pub pipelineCacheUUID: [u8; 16],
    pub limits: PhysicalDeviceLimits,
    pub sparseProperties: PhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct FormatProperties {
    pub linearTilingFeatures: u32,
    pub optimalTilingFeatures: u32,
    pub bufferFeatures: u32,
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
pub struct PipelineDynamicStateCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const u32,
}

#[repr(C)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: u32,
}

#[repr(C)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: u32,
    pub offset: u32,
}

#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VertexInputAttributeDescription,
}

#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub topology: u32,
    pub primitiveRestartEnable: u32,
}

#[repr(C)]
pub struct BufferCopy {
    pub srcOffset: u64,
    pub dstOffset: u64,
    pub size: u64,
}

#[repr(C)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub viewportCount: u32,
    pub pViewports: *const Viewport,
    pub scissorCount: u32,
    pub pScissors: *const Rect2D,
}

#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub depthClampEnable: u32,
    pub rasterizerDiscardEnable: u32,
    pub polygonMode: u32,
    pub cullMode: u32,
    pub frontFace: u32,
    pub depthBiasEnable: u32,
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
    pub lineWidth: f32,
}

#[repr(C)]
pub struct StencilOpState {
    pub failOp: u32,
    pub passOp: u32,
    pub depthFailOp: u32,
    pub compareOp: u32,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}

#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub depthTestEnable: u32,
    pub depthWriteEnable: u32,
    pub depthCompareOp: u32,
    pub depthBoundsTestEnable: u32,
    pub stencilTestEnable: u32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub minDepthBounds: f32,
    pub maxDepthBounds: f32,
}

#[repr(C)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: u32,
}

#[repr(C)]
pub struct SubpassDescription {
    pub flags: u32,
    pub pipelineBindPoint: u32,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const AttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const AttachmentReference,
    pub pResolveAttachments: *const AttachmentReference,
    pub pDepthStencilAttachment: *const AttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

#[repr(C)]
pub struct SubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: u32,
    pub dstStageMask: u32,
    pub srcAccessMask: u32,
    pub dstAccessMask: u32,
    pub dependencyFlags: u32,
}

#[repr(C)]
pub struct AttachmentDescription {
    pub flags: u32,
    pub format: u32,
    pub samples: u32,
    pub loadOp: u32,
    pub storeOp: u32,
    pub stencilLoadOp: u32,
    pub stencilStoreOp: u32,
    pub initialLayout: u32,
    pub finalLayout: u32,
}

#[repr(C)]
pub struct RenderPassCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub attachmentCount: u32,
    pub pAttachments: *const AttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const SubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const SubpassDependency,
}

#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub rasterizationSamples: u32,
    pub sampleShadingEnable: u32,
    pub minSampleShading: f32,
    pub pSampleMask: *const u32,
    pub alphaToCoverageEnable: u32,
    pub alphaToOneEnable: u32,
}

#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
    pub blendEnable: u32,
    pub srcColorBlendFactor: u32,
    pub dstColorBlendFactor: u32,
    pub colorBlendOp: u32,
    pub srcAlphaBlendFactor: u32,
    pub dstAlphaBlendFactor: u32,
    pub alphaBlendOp: u32,
    pub colorWriteMask: u32,
}

#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub logicOpEnable: u32,
    pub logicOp: u32,
    pub attachmentCount: u32,
    pub pAttachments: *const PipelineColorBlendAttachmentState,
    pub blendConstants: [f32; 4],
}

#[repr(C)]
pub struct Sampler {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: u32,
    pub descriptorCount: u32,
    pub stageFlags: u32,
    pub pImmutableSamplers: *const *mut Sampler,
}

#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub bindingCount: u32,
    pub pBindings: *const DescriptorSetLayoutBinding,
}

#[repr(C)]
pub struct DescriptorPoolSize {
    pub type_: u32,
    pub descriptorCount: u32,
}

#[repr(C)]
pub struct DescriptorPool {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct DescriptorSet {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct RenderPass {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct DescriptorPoolCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const DescriptorPoolSize,
}

#[repr(C)]
pub struct DescriptorSetAllocateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub descriptorPool: *mut DescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const *mut DescriptorSetLayout,
}

#[repr(C)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub minDepth: f32,
    pub maxDepth: f32,
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
pub struct Queue {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ExtensionProperties {
    pub extensionName: [std::os::raw::c_char; 256],
    pub specVersion: u32,
}

#[repr(C)]
pub struct MemoryType {
    pub propertyFlags: u32,
    pub heapIndex: u32,
}

#[repr(C)]
pub struct MemoryHeap {
    pub size: u64,
    pub flags: u32,
}

#[repr(C)]
pub struct ImageCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub imageType: u32,
    pub format: u32,
    pub extent: Extent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: u32,
    pub tiling: u32,
    pub usage: u32,
    pub sharingMode: u32,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: u32,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [MemoryType; 32],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [MemoryHeap; 16],
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

#[repr(C)]
pub struct ComponentMapping {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

#[repr(C)]
pub struct ImageSubresourceRange {
    pub aspectMask: u32,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
pub struct ImageViewCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub image: *mut Image,
    pub viewType: u32,
    pub format: u32,
    pub components: ComponentMapping,
    pub subresourceRange: ImageSubresourceRange,
}

#[repr(C)]
pub struct ImageView {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct MemoryRequirements {
    pub size: u64,
    pub alignment: u64,
    pub memoryTypeBits: u32,
}

#[repr(C)]
pub struct MemoryAllocateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub allocationSize: u64,
    pub memoryTypeIndex: u32,
}

#[repr(C)]
pub struct DeviceMemory {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct CommandPoolCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub queueFamilyIndex: u32,
}

#[repr(C)]
pub struct Buffer {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct CommandBufferAllocateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub commandPool: *mut CommandPool,
    pub level: u32,
    pub commandBufferCount: u32,
}

#[repr(C)]
pub struct CommandBuffer {
    _unused: [u8; 0],
}

pub struct CommandPool {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depthStencil: ClearDepthStencilValue,
}

#[repr(C)]
pub struct RenderPassBeginInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub renderPass: *mut RenderPass,
    pub framebuffer: *mut Framebuffer,
    pub renderArea: Rect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const ClearValue,
}

#[repr(C)]
pub struct CommandBufferInheritanceInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub renderPass: *mut RenderPass,
    pub subpass: u32,
    pub framebuffer: *mut Framebuffer,
    pub occlusionQueryEnable: u32,
    pub queryFlags: u32,
    pub pipelineStatistics: u32,
}

#[repr(C)]
pub struct CommandBufferBeginInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub pInheritanceInfo: *const CommandBufferInheritanceInfo,
}

#[repr(C)]
pub struct SemaphoreCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
}

#[repr(C)]
pub struct Fence {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct Semaphore {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct FenceCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
}

#[repr(C)]
pub struct AcquireNextImageInfoKHR {
    pub sType: u32,
    pub pNext: *const void,
    pub swapchain: *mut SwapchainKHR,
    pub timeout: u64,
    pub semaphore: *mut Semaphore,
    pub fence: *mut Fence,
    pub deviceMask: u32,
}

#[repr(C)]
pub struct PresentInfoKHR {
    pub sType: u32,
    pub pNext: *const void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const *mut Semaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const *mut SwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut *mut u32,
}

#[repr(C)]
pub struct BufferCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub size: u64,
    pub usage: u32,
    pub sharingMode: u32,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

#[repr(C)]
pub struct MemoryBarrier {
    pub sType: u32,
    pub pNext: *const void,
    pub srcAccessMask: u32,
    pub dstAccessMask: u32,
}

#[repr(C)]
pub struct BufferMemoryBarrier {
    pub sType: u32,
    pub pNext: *const void,
    pub srcAccessMask: u32,
    pub dstAccessMask: u32,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: *mut Buffer,
    pub offset: u64,
    pub size: u64,
}

#[repr(C)]
pub struct ImageMemoryBarrier {
    pub sType: u32,
    pub pNext: *const void,
    pub srcAccessMask: u32,
    pub dstAccessMask: u32,
    pub oldLayout: u32,
    pub newLayout: u32,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: *mut Image,
    pub subresourceRange: ImageSubresourceRange,
}

#[repr(C)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
pub struct BufferImageCopy {
    pub bufferOffset: u64,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: ImageSubresourceLayers,
    pub imageOffset: Offset3D,
    pub imageExtent: Extent3D,
}

#[repr(C)]
pub struct SamplerCreateInfo {
    pub sType: u32,
    pub pNext: *const void,
    pub flags: u32,
    pub magFilter: u32,
    pub minFilter: u32,
    pub mipmapMode: u32,
    pub addressModeU: u32,
    pub addressModeV: u32,
    pub addressModeW: u32,
    pub mipLodBias: f32,
    pub anisotropyEnable: u32,
    pub maxAnisotropy: f32,
    pub compareEnable: u32,
    pub compareOp: u32,
    pub minLod: f32,
    pub maxLod: f32,
    pub borderColor: u32,
    pub unnormalizedCoordinates: u32,
}

#[repr(C)]
pub struct WriteDescriptorSet {
    pub sType: u32,
    pub pNext: *const void,
    pub dstSet: *mut DescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: u32,
    pub pImageInfo: *const DescriptorImageInfo,
    pub pBufferInfo: *const DescriptorBufferInfo,
    pub pTexelBufferView: *const BufferView,
}

#[repr(C)]
pub struct BufferView {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct DescriptorBufferInfo {
    pub buffer: *mut Buffer,
    pub offset: u64,
    pub range: u64,
}

#[repr(C)]
pub struct DescriptorImageInfo {
    pub sampler: *mut Sampler,
    pub imageView: *mut ImageView,
    pub imageLayout: u32,
}

#[repr(C)]
pub struct CopyDescriptorSet {
    pub sType: u32,
    pub pNext: *const void,
    pub srcSet: *mut DescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: *mut DescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

#[repr(C)]
pub struct CopyBufferToImageInfo2 {
    pub sType: u32,
    pub pNext: *const void,
    pub srcBuffer: *mut Buffer,
    pub dstImage: *mut Image,
    pub dstImageLayout: u32,
    pub regionCount: u32,
    pub pRegions: *const BufferImageCopy2,
}

#[repr(C)]
pub struct BufferImageCopy2 {
    pub sType: u32,
    pub pNext: *const void,
    pub bufferOffset: u64,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: ImageSubresourceLayers,
    pub imageOffset: Offset3D,
    pub imageExtent: Extent3D,
}

#[repr(C)]
pub struct ImageSubresourceLayers {
    pub aspectMask: u32,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
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
    pDevice: *mut *mut Device,
) -> i32;
pub type PFN_vkCreateDevice = Option<vkCreateDevice>;
pub type vkCreateInstance = unsafe extern "C" fn(
    pCreateInfo: *const InstanceCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pInstance: *mut *mut Instance,
) -> i32;
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
pub type PFN_vkDestroySurfaceKHR = Option<vkDestroySurfaceKHR>;
pub type vkCreateWaylandSurfaceKHR = unsafe extern "C" fn(
    instance: *mut Instance,
    pCreateInfo: *const WaylandSurfaceCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pSurface: *mut *mut SurfaceKHR,
) -> i32;
pub type PFN_vkCreateWaylandSurfaceKHR = Option<vkCreateWaylandSurfaceKHR>;
pub type vkEnumeratePhsysicalDevices = unsafe extern "C" fn(
    instance: *mut Instance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut *mut PhysicalDevice,
) -> i32;
pub type PFN_vkEnumeratePhysicalDevices = Option<vkEnumeratePhsysicalDevices>;
pub type vkEnumerateDeviceExtensionProperties = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    pLayerName: *const i8,
    pPropertyCount: *mut u32,
    pProperties: *mut ExtensionProperties,
) -> i32;
pub type PFN_vkEnumerateDeviceExtensionProperties = Option<vkEnumerateDeviceExtensionProperties>;
pub type vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    surface: *mut SurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut SurfaceFormatKHR,
) -> i32;
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = Option<vkGetPhysicalDeviceSurfaceFormatsKHR>;
pub type vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut QueueFamilyProperties,
);
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = Option<vkGetPhysicalDeviceQueueFamilyProperties>;
pub type vkGetPhysicalDeviceMemoryProperties = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    pMemoryProperties: *mut PhysicalDeviceMemoryProperties,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties = Option<vkGetPhysicalDeviceMemoryProperties>;
pub type vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    queueFamilyIndex: u32,
    surface: *mut SurfaceKHR,
    pSupported: *mut u32,
) -> i32;
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = Option<vkGetPhysicalDeviceSurfaceSupportKHR>;
pub type vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    surface: *mut SurfaceKHR,
    pSurfaceCapabilities: *mut SurfaceCapabilitiesKHR,
) -> i32;
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = Option<vkGetPhysicalDeviceSurfaceCapabilitiesKHR>;
pub type vkGetPhysicalDeviceFeatures = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    pFeatures: *mut PhysicalDeviceFeatures,
);
pub type PFN_vkGetPhysicalDeviceFeatures = Option<vkGetPhysicalDeviceFeatures>;
pub type vkGetPhysicalDeviceProperties = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    pProperties: *mut PhysicalDeviceProperties,
);
pub type PFN_vkGetPhysicalDeviceProperties = Option<vkGetPhysicalDeviceProperties>;
pub type vkGetDeviceQueue = unsafe extern "C" fn(
    device: *mut Device,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: *mut *mut Queue,
);
pub type PFN_vkGetDeviceQueue = Option<vkGetDeviceQueue>;
pub type vkDestroyDevice = unsafe extern "C" fn(
    device: *mut Device,
    pAllocator: *const AllocationCallbacks
);
pub type PFN_vkDestroyDevice = Option<vkDestroyDevice>;
pub type vkCreateShaderModule = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const ShaderModuleCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pShaderModule: *mut *mut ShaderModule,
) -> i32;
pub type PFN_vkCreateShaderModule = Option<vkCreateShaderModule>;
pub type vkCreateDescriptorSetLayout = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const DescriptorSetLayoutCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pSetLayout: *mut *mut DescriptorSetLayout,
) -> i32;
pub type PFN_vkCreateDescriptorSetLayout = Option<vkCreateDescriptorSetLayout>;
pub type vkCreatePipelineLayout = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const PipelineLayoutCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pPipelineLayout: *mut *mut PipelineLayout,
) -> i32;
pub type PFN_vkCreatePipelineLayout = Option<vkCreatePipelineLayout>;
pub type vkCreateDescriptorPool = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const DescriptorPoolCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pDescriptorPool: *mut *mut DescriptorPool,
) -> i32;
pub type PFN_vkCreateDescriptorPool = Option<vkCreateDescriptorPool>;
pub type vkAllocateDescriptorSets = unsafe extern "C" fn(
    device: *mut Device,
    pAllocateInfo: *const DescriptorSetAllocateInfo,
    pDescriptorSets: *mut *mut DescriptorSet,
) -> i32;
pub type PFN_vkAllocateDescriptorSets = Option<vkAllocateDescriptorSets>;
pub type vkGetPhysicalDeviceFormatProperties = unsafe extern "C" fn(
    physicalDevice: *mut PhysicalDevice,
    format: u32,
    pFormatProperties: *mut FormatProperties,
);
pub type PFN_vkGetPhysicalDeviceFormatProperties = Option<vkGetPhysicalDeviceFormatProperties>;
pub type vkCreateRenderPass = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const RenderPassCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pRenderPass: *mut *mut RenderPass,
) -> i32;
pub type PFN_vkCreateRenderPass = Option<vkCreateRenderPass>;
pub type vkCreateGraphicsPipelines = unsafe extern "C" fn(
    device: *mut Device,
    pipelineCache: *mut PipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const GraphicsPipelineCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pPipelines: *mut *mut Pipeline,
) -> i32;
pub type PFN_vkCreateGraphicsPipelines = Option<vkCreateGraphicsPipelines>;
pub type vkDestroyShaderModule = unsafe extern "C" fn(
    device: *mut Device,
    shaderModule: *mut ShaderModule,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyShaderModule = Option<vkDestroyShaderModule>;
pub type vkDestroyPipelineLayout = unsafe extern "C" fn(
    device: *mut Device,
    pipelineLayout: *mut PipelineLayout,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyPipelineLayout = Option<vkDestroyPipelineLayout>;
pub type vkDestroyDescriptorPool = unsafe extern "C" fn(
    device: *mut Device,
    descriptorPool: *mut DescriptorPool,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyDescriptorPool = Option<vkDestroyDescriptorPool>;
pub type vkDestroyDescriptorSetLayout = unsafe extern "C" fn(
    device: *mut Device,
    descriptorSetLayout: *mut DescriptorSetLayout,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyDescriptorSetLayout = Option<vkDestroyDescriptorSetLayout>;
pub type vkDestroyRenderPass = unsafe extern "C" fn(
    device: *mut Device,
    renderPass: *mut RenderPass,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyRenderPass = Option<vkDestroyRenderPass>;
pub type vkDestroyPipeline = unsafe extern "C" fn(
    device: *mut Device,
    pipeline: *mut Pipeline,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyPipeline = Option<vkDestroyPipeline>;
pub type vkGetSwapchainImagesKHR = unsafe extern "C" fn(
    device: *mut Device,
    swapchain: *mut SwapchainKHR,
    pSwapchainImageCount: *mut u32,
    pSwapchainImages: *mut *mut Image,
) -> i32;
pub type PFN_vkGetSwapchainImagesKHR = Option<vkGetSwapchainImagesKHR>;
pub type vkCreateSwapchainKHR = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const SwapchainCreateInfoKHR,
    pAllocator: *const AllocationCallbacks,
    pSwapchain: *mut *mut SwapchainKHR,
) -> i32;
pub type PFN_vkCreateSwapchainKHR = Option<vkCreateSwapchainKHR>;
pub type vkDestroySwapchainKHR = unsafe extern "C" fn(
    device: *mut Device,
    swapchain: *mut SwapchainKHR,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroySwapchainKHR = Option<vkDestroySwapchainKHR>;
pub type vkCreateImageView = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const ImageViewCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pView: *mut *mut ImageView,
) -> i32;
pub type PFN_vkCreateImageView = Option<vkCreateImageView>;
pub type vkDestroyImageView = unsafe extern "C" fn(
    device: *mut Device,
    imageView: *mut ImageView,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyImageView = Option<vkDestroyImageView>;
pub type vkCreateImage = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const ImageCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pImage: *mut *mut Image,
) -> i32;
pub type PFN_vkCreateImage = Option<vkCreateImage>;
pub type vkGetImageMemoryRequirements = unsafe extern "C" fn(
    device: *mut Device,
    image: *mut Image,
    pMemoryRequirements: *mut MemoryRequirements,
);
pub type PFN_vkGetImageMemoryRequirements = Option<vkGetImageMemoryRequirements>;
pub type vkAllocateMemory = unsafe extern "C" fn(
    device: *mut Device,
    pAllocateInfo: *const MemoryAllocateInfo,
    pAllocator: *const AllocationCallbacks,
    pMemory: *mut *mut DeviceMemory,
) -> i32;
pub type PFN_vkAllocateMemory = Option<vkAllocateMemory>;
pub type vkDestroyImage = unsafe extern "C" fn(
    device: *mut Device,
    image: *mut Image,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyImage = Option<vkDestroyImage>;
pub type vkFreeMemory = unsafe extern "C" fn(
    device: *mut Device,
    memory: *mut DeviceMemory,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkFreeMemory = Option<vkFreeMemory>;
pub type vkBindImageMemory = unsafe extern "C" fn(
    device: *mut Device,
    image: *mut Image,
    memory: *mut DeviceMemory,
    memoryOffset: u64,
) -> i32;
pub type PFN_vkBindImageMemory = Option<vkBindImageMemory>;
pub type vkCreateFramebuffer = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const FramebufferCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pFramebuffer: *mut *mut Framebuffer,
) -> i32;
pub type PFN_vkCreateFramebuffer = Option<vkCreateFramebuffer>;
pub type vkDestroyFramebuffer = unsafe extern "C" fn(
    device: *mut Device,
    framebuffer: *mut Framebuffer,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyFramebuffer = Option<vkDestroyFramebuffer>;
pub type vkCreateCommandPool = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const CommandPoolCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pCommandPool: *mut *mut CommandPool,
) -> i32;
pub type PFN_vkCreateCommandPool = Option<vkCreateCommandPool>;
pub type vkAllocateCommandBuffers = unsafe extern "C" fn(
    device: *mut Device,
    pAllocateInfo: *const CommandBufferAllocateInfo,
    pCommandBuffers: *mut *mut CommandBuffer,
) -> i32;
pub type PFN_vkAllocateCommandBuffers = Option<vkAllocateCommandBuffers>;
pub type vkDestroyCommandPool = unsafe extern "C" fn(
    device: *mut Device,
    commandPool: *mut CommandPool,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyCommandPool = Option<vkDestroyCommandPool>;
pub type vkBeginCommandBuffer = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    pBeginInfo: *const CommandBufferBeginInfo,
) -> i32;
pub type PFN_vkBeginCommandBuffer = Option<vkBeginCommandBuffer>;
pub type vkCmdBeginRenderPass = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    pRenderPassBegin: *const RenderPassBeginInfo,
    contents: u32,
);
pub type PFN_vkCmdBeginRenderPass = Option<vkCmdBeginRenderPass>;

pub type vkCmdBindPipeline = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    pipelineBindPoint: u32,
    pipeline: *mut Pipeline,
);
pub type PFN_vkCmdBindPipeline = Option<vkCmdBindPipeline>;
pub type vkCmdSetViewport = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: *const Viewport,
);
pub type PFN_vkCmdSetViewport = Option<vkCmdSetViewport>;
pub type vkCmdSetScissor = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: *const Rect2D,
);
pub type PFN_vkCmdSetScissor = Option<vkCmdSetScissor>;
pub type vkCmdDraw = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
);
pub type PFN_vkCmdDraw = Option<vkCmdDraw>;
pub type vkCmdEndRenderPass = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer
);
pub type PFN_vkCmdEndRenderPass = Option<vkCmdEndRenderPass>;
pub type vkEndCommandBuffer = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer
) -> i32;
pub type PFN_vkEndCommandBuffer = Option<vkEndCommandBuffer>;

pub type vkCreateSemaphore = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const SemaphoreCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pSemaphore: *mut *mut Semaphore,
) -> i32;
pub type PFN_vkCreateSemaphore = Option<vkCreateSemaphore>;
pub type vkDestroySemaphore = unsafe extern "C" fn(
    device: *mut Device,
    semaphore: *mut Semaphore,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroySemaphore = Option<vkDestroySemaphore>;
pub type vkCreateFence = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const FenceCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pFence: *mut *mut Fence,
) -> i32;
pub type PFN_vkCreateFence = Option<vkCreateFence>;
pub type vkDestroyFence = unsafe extern "C" fn(
    device: *mut Device,
    fence: *mut Fence,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyFence = Option<vkDestroyFence>;
pub type vkWaitForFences = unsafe extern "C" fn(
    device: *mut Device,
    fenceCount: u32,
    pFences: *const *mut Fence,
    waitAll: u32,
    timeout: u64,
) -> i32;
pub type PFN_vkWaitForFences = Option<vkWaitForFences>;
pub type vkAcquireNextImageKHR = unsafe extern "C" fn(
    device: *mut Device,
    swapchain: *mut SwapchainKHR,
    timeout: u64,
    semaphore: *mut Semaphore,
    fence: *mut Fence,
    pImageIndex: *mut u32,
) -> i32;
pub type PFN_vkAcquireNextImageKHR = Option<vkAcquireNextImageKHR>;
pub type vkQueueSubmit = unsafe extern "C" fn(
    queue: *mut Queue,
    submitCount: u32,
    pSubmits: *const SubmitInfo,
    fence: *mut Fence,
) -> i32;
pub type PFN_vkQueueSubmit = Option<vkQueueSubmit>;
pub type vkQueuePresentKHR = unsafe extern "C" fn(
    queue: *mut Queue,
    pPresentInfo: *const PresentInfoKHR
) -> i32;

pub type PFN_vkQueuePresentKHR = Option<vkQueuePresentKHR>;
pub type vkCreateBuffer = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const BufferCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pBuffer: *mut *mut Buffer,
) -> i32;
pub type PFN_vkCreateBuffer = Option<vkCreateBuffer>;
pub type vkDestroyBuffer = unsafe extern "C" fn(
    device: *mut Device,
    buffer: *mut Buffer,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyBuffer = Option<vkDestroyBuffer>;
pub type vkGetBufferMemoryRequirements = unsafe extern "C" fn(
    device: *mut Device,
    buffer: *mut Buffer,
    pMemoryRequirements: *mut MemoryRequirements,
);
pub type PFN_vkGetBufferMemoryRequirements = Option<vkGetBufferMemoryRequirements>;
pub type vkBindBufferMemory = unsafe extern "C" fn(
    device: *mut Device,
    buffer: *mut Buffer,
    memory: *mut DeviceMemory,
    memoryOffset: u64,
) -> i32;
pub type PFN_vkBindBufferMemory = Option<vkBindBufferMemory>;
pub type vkMapMemory = unsafe extern "C" fn(
    device: *mut Device,
    memory: *mut DeviceMemory,
    offset: u64,
    size: u64,
    flags: u32,
    ppData: *mut *mut void,
) -> i32;
pub type PFN_vkMapMemory = Option<vkMapMemory>;
pub type vkUnmapMemory = unsafe extern "C" fn(
    device: *mut Device,
    memory: *mut DeviceMemory
);
pub type PFN_vkUnmapMemory = Option<vkUnmapMemory>;
pub type vkCmdBindVertexBuffers = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const *mut Buffer,
    pOffsets: *const u64,
);
pub type PFN_vkCmdBindVertexBuffers = Option<vkCmdBindVertexBuffers>;
pub type vkResetFences = unsafe extern "C" fn(
    device: *mut Device,
    fenceCount: u32,
    pFences: *const *mut Fence
) -> i32;
pub type PFN_vkResetFences = Option<vkResetFences>;
pub type vkCmdPipelineBarrier = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    srcStageMask: u32,
    dstStageMask: u32,
    dependencyFlags: u32,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const *mut MemoryBarrier,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const *mut BufferMemoryBarrier,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const ImageMemoryBarrier,
);
pub type PFN_vkCmdPipelineBarrier = ::std::option::Option<vkCmdPipelineBarrier>;
pub type vkFreeCommandBuffers = unsafe extern "C" fn(
    device: *mut Device,
    commandPool: *mut CommandPool,
    commandBufferCount: u32,
    pCommandBuffers: *const *mut CommandBuffer,
);
pub type PFN_vkFreeCommandBuffers = ::std::option::Option<vkFreeCommandBuffers>;
pub type vkQueueWaitIdle = unsafe extern "C" fn(
    queue: *mut Queue,
) -> u32;
pub type PFN_vkQueueWaitIdle = ::std::option::Option<vkQueueWaitIdle>;
pub type vkCmdCopyBufferToImage = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    srcBuffer: *mut Buffer,
    dstImage: *mut Image,
    dstImageLayout: u32,
    regionCount: u32,
    pRegions: *const BufferImageCopy,
);
pub type PFN_vkCmdCopyBufferToImage = ::std::option::Option<vkCmdCopyBufferToImage>;
pub type vkCreateSampler = unsafe extern "C" fn(
    device: *mut Device,
    pCreateInfo: *const SamplerCreateInfo,
    pAllocator: *const AllocationCallbacks,
    pSampler: *mut *mut Sampler,
) -> u32;
pub type PFN_vkCreateSampler = ::std::option::Option<vkCreateSampler>;
pub type vkUpdateDescriptorSets = unsafe extern "C" fn(
    device: *mut Device,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const WriteDescriptorSet,
    descriptorCopyCount: u32,
    pDescriptorCopies: *const CopyDescriptorSet,
);
pub type PFN_vkUpdateDescriptorSets = ::std::option::Option<vkUpdateDescriptorSets>;
pub type vkDestroySampler =unsafe extern "C" fn(
    device: *mut Device,
    sampler: *mut Sampler,
    pAllocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroySampler = ::std::option::Option<vkDestroySampler>;
pub type vkCmdBindDescriptorSets = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    pipelineBindPoint: u32,
    layout: *mut PipelineLayout,
    firstSet: u32,
    descriptorSetCount: u32,
    pDescriptorSets: *const *mut DescriptorSet,
    dynamicOffsetCount: u32,
    pDynamicOffsets: *const u32,
);
pub type PFN_vkCmdBindDescriptorSets = ::std::option::Option<vkCmdBindDescriptorSets>;
pub type vkCmdCopyBuffer = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    srcBuffer: *mut Buffer,
    dstBuffer: *mut Buffer,
    regionCount: u32,
    pRegions: *const BufferCopy,
);
pub type PFN_vkCmdCopyBuffer = ::std::option::Option<vkCmdCopyBuffer>;
pub type vkCmdBindIndexBuffer = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    buffer: *mut Buffer,
    offset: u64,
    indexType: u32,
);
pub type PFN_vkCmdBindIndexBuffer = ::std::option::Option<vkCmdBindIndexBuffer>;
pub type vkCmdDrawIndexed = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    indexCount: u32,
    instanceCount: u32,
    firstIndex: u32,
    vertexOffset: i32,
    firstInstance: u32,
);
pub type PFN_vkCmdDrawIndexed = ::std::option::Option<vkCmdDrawIndexed>;
pub type vkCmdPushConstants = unsafe extern "C" fn(
    commandBuffer: *mut CommandBuffer,
    layout: *mut PipelineLayout,
    stageFlags: u32,
    offset: u32,
    size: u32,
    pValues: *const ::std::os::raw::c_void,
);
pub type PFN_vkCmdPushConstants = ::std::option::Option<vkCmdPushConstants>;
