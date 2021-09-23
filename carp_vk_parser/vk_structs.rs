#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBaseOutStructure
{
	pub sType: VkStructureType,
	pub pNext: * mut VkBaseOutStructure,
}
impl VkBaseOutStructure
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBaseInStructure
{
	pub sType: VkStructureType,
	pub pNext: * const VkBaseInStructure,
}
impl VkBaseInStructure
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset2D
{
	pub x: i32,
	pub y: i32,
}
impl VkOffset2D
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset3D
{
	pub x: i32,
	pub y: i32,
	pub z: i32,
}
impl VkOffset3D
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent2D
{
	pub width: u32,
	pub height: u32,
}
impl VkExtent2D
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent3D
{
	pub width: u32,
	pub height: u32,
	pub depth: u32,
}
impl VkExtent3D
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkViewport
{
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
	pub minDepth: f32,
	pub maxDepth: f32,
}
impl VkViewport
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRect2D
{
	pub offset: VkOffset2D,
	pub extent: VkExtent2D,
}
impl VkRect2D
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearRect
{
	pub rect: VkRect2D,
	pub baseArrayLayer: u32,
	pub layerCount: u32,
}
impl VkClearRect
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkComponentMapping
{
	pub r: VkComponentSwizzle,
	pub g: VkComponentSwizzle,
	pub b: VkComponentSwizzle,
	pub a: VkComponentSwizzle,
}
impl VkComponentMapping
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProperties
{
	pub apiVersion: u32,
	pub driverVersion: u32,
	pub vendorID: u32,
	pub deviceID: u32,
	pub deviceType: VkPhysicalDeviceType,
	pub deviceName: [c_uchar; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
	pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
	pub limits: VkPhysicalDeviceLimits,
	pub sparseProperties: VkPhysicalDeviceSparseProperties,
}
impl VkPhysicalDeviceProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtensionProperties
{
	pub extensionName: [c_uchar; VK_MAX_EXTENSION_NAME_SIZE],
	pub specVersion: u32,
}
impl VkExtensionProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkLayerProperties
{
	pub layerName: [c_uchar; VK_MAX_EXTENSION_NAME_SIZE],
	pub specVersion: u32,
	pub implementationVersion: u32,
	pub description: [c_uchar; VK_MAX_DESCRIPTION_SIZE],
}
impl VkLayerProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkApplicationInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub pApplicationName: * const c_uchar,
	pub applicationVersion: u32,
	pub pEngineName: * const c_uchar,
	pub engineVersion: u32,
	pub apiVersion: u32,
}
impl VkApplicationInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceQueueCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkDeviceQueueCreateFlagBits,
	pub queueFamilyIndex: u32,
	pub queueCount: u32,
	pub pQueuePriorities: * const f32,
}
impl VkDeviceQueueCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkDeviceCreateFlagBits,
	pub queueCreateInfoCount: u32,
	pub pQueueCreateInfos: * const VkDeviceQueueCreateInfo,
	pub enabledLayerCount: u32,
	pub ppEnabledLayerNames: * const * const c_uchar,
	pub enabledExtensionCount: u32,
	pub ppEnabledExtensionNames: * const * const c_uchar,
	pub pEnabledFeatures: * const VkPhysicalDeviceFeatures,
}
impl VkDeviceCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInstanceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkInstanceCreateFlagBits,
	pub pApplicationInfo: * const VkApplicationInfo,
	pub enabledLayerCount: u32,
	pub ppEnabledLayerNames: * const * const c_uchar,
	pub enabledExtensionCount: u32,
	pub ppEnabledExtensionNames: * const * const c_uchar,
}
impl VkInstanceCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueueFamilyProperties
{
	pub queueFlags: VkQueueFlagBits,
	pub queueCount: u32,
	pub timestampValidBits: u32,
	pub minImageTransferGranularity: VkExtent3D,
}
impl VkQueueFamilyProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties
{
	pub memoryTypeCount: u32,
	pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
	pub memoryHeapCount: u32,
	pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}
impl VkPhysicalDeviceMemoryProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub allocationSize: VkDeviceSize,
	pub memoryTypeIndex: u32,
}
impl VkMemoryAllocateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryRequirements
{
	pub size: VkDeviceSize,
	pub alignment: VkDeviceSize,
	pub memoryTypeBits: u32,
}
impl VkMemoryRequirements
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageFormatProperties
{
	pub aspectMask: VkImageAspectFlagBits,
	pub imageGranularity: VkExtent3D,
	pub flags: VkSparseImageFormatFlagBits,
}
impl VkSparseImageFormatProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryRequirements
{
	pub formatProperties: VkSparseImageFormatProperties,
	pub imageMipTailFirstLod: u32,
	pub imageMipTailSize: VkDeviceSize,
	pub imageMipTailOffset: VkDeviceSize,
	pub imageMipTailStride: VkDeviceSize,
}
impl VkSparseImageMemoryRequirements
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryType
{
	pub propertyFlags: VkMemoryPropertyFlagBits,
	pub heapIndex: u32,
}
impl VkMemoryType
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryHeap
{
	pub size: VkDeviceSize,
	pub flags: VkMemoryHeapFlagBits,
}
impl VkMemoryHeap
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMappedMemoryRange
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub memory: VkDeviceMemory,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
}
impl VkMappedMemoryRange
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFormatProperties
{
	pub linearTilingFeatures: VkFormatFeatureFlagBits,
	pub optimalTilingFeatures: VkFormatFeatureFlagBits,
	pub bufferFeatures: VkFormatFeatureFlagBits,
}
impl VkFormatProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatProperties
{
	pub maxExtent: VkExtent3D,
	pub maxMipLevels: u32,
	pub maxArrayLayers: u32,
	pub sampleCounts: VkSampleCountFlagBits,
	pub maxResourceSize: VkDeviceSize,
}
impl VkImageFormatProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorBufferInfo
{
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub range: VkDeviceSize,
}
impl VkDescriptorBufferInfo
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorImageInfo
{
	pub sampler: VkSampler,
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout,
}
impl VkDescriptorImageInfo
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWriteDescriptorSet
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub dstSet: VkDescriptorSet,
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
	pub descriptorType: VkDescriptorType,
	pub pImageInfo: * const VkDescriptorImageInfo,
	pub pBufferInfo: * const VkDescriptorBufferInfo,
	pub pTexelBufferView: * const VkBufferView,
}
impl VkWriteDescriptorSet
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyDescriptorSet
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub srcSet: VkDescriptorSet,
	pub srcBinding: u32,
	pub srcArrayElement: u32,
	pub dstSet: VkDescriptorSet,
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
}
impl VkCopyDescriptorSet
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkBufferCreateFlagBits,
	pub size: VkDeviceSize,
	pub usage: VkBufferUsageFlagBits,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: * const u32,
}
impl VkBufferCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferViewCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkBufferViewCreateFlagBits,
	pub buffer: VkBuffer,
	pub format: VkFormat,
	pub offset: VkDeviceSize,
	pub range: VkDeviceSize,
}
impl VkBufferViewCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresource
{
	pub aspectMask: VkImageAspectFlagBits,
	pub mipLevel: u32,
	pub arrayLayer: u32,
}
impl VkImageSubresource
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceLayers
{
	pub aspectMask: VkImageAspectFlagBits,
	pub mipLevel: u32,
	pub baseArrayLayer: u32,
	pub layerCount: u32,
}
impl VkImageSubresourceLayers
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceRange
{
	pub aspectMask: VkImageAspectFlagBits,
	pub baseMipLevel: u32,
	pub levelCount: u32,
	pub baseArrayLayer: u32,
	pub layerCount: u32,
}
impl VkImageSubresourceRange
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryBarrier
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub srcAccessMask: VkAccessFlagBits,
	pub dstAccessMask: VkAccessFlagBits,
}
impl VkMemoryBarrier
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_BARRIER;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferMemoryBarrier
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub srcAccessMask: VkAccessFlagBits,
	pub dstAccessMask: VkAccessFlagBits,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
}
impl VkBufferMemoryBarrier
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageMemoryBarrier
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub srcAccessMask: VkAccessFlagBits,
	pub dstAccessMask: VkAccessFlagBits,
	pub oldLayout: VkImageLayout,
	pub newLayout: VkImageLayout,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub image: VkImage,
	pub subresourceRange: VkImageSubresourceRange,
}
impl VkImageMemoryBarrier
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkImageCreateFlagBits,
	pub imageType: VkImageType,
	pub format: VkFormat,
	pub extent: VkExtent3D,
	pub mipLevels: u32,
	pub arrayLayers: u32,
	pub samples: VkSampleCountFlagBits,
	pub tiling: VkImageTiling,
	pub usage: VkImageUsageFlagBits,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: * const u32,
	pub initialLayout: VkImageLayout,
}
impl VkImageCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubresourceLayout
{
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub rowPitch: VkDeviceSize,
	pub arrayPitch: VkDeviceSize,
	pub depthPitch: VkDeviceSize,
}
impl VkSubresourceLayout
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageViewCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkImageViewCreateFlagBits,
	pub image: VkImage,
	pub viewType: VkImageViewType,
	pub format: VkFormat,
	pub components: VkComponentMapping,
	pub subresourceRange: VkImageSubresourceRange,
}
impl VkImageViewCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCopy
{
	pub srcOffset: VkDeviceSize,
	pub dstOffset: VkDeviceSize,
	pub size: VkDeviceSize,
}
impl VkBufferCopy
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseMemoryBind
{
	pub resourceOffset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub flags: VkSparseMemoryBindFlagBits,
}
impl VkSparseMemoryBind
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryBind
{
	pub subresource: VkImageSubresource,
	pub offset: VkOffset3D,
	pub extent: VkExtent3D,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub flags: VkSparseMemoryBindFlagBits,
}
impl VkSparseImageMemoryBind
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseBufferMemoryBindInfo
{
	pub buffer: VkBuffer,
	pub bindCount: u32,
	pub pBinds: * const VkSparseMemoryBind,
}
impl VkSparseBufferMemoryBindInfo
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageOpaqueMemoryBindInfo
{
	pub image: VkImage,
	pub bindCount: u32,
	pub pBinds: * const VkSparseMemoryBind,
}
impl VkSparseImageOpaqueMemoryBindInfo
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryBindInfo
{
	pub image: VkImage,
	pub bindCount: u32,
	pub pBinds: * const VkSparseImageMemoryBind,
}
impl VkSparseImageMemoryBindInfo
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindSparseInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: * const VkSemaphore,
	pub bufferBindCount: u32,
	pub pBufferBinds: * const VkSparseBufferMemoryBindInfo,
	pub imageOpaqueBindCount: u32,
	pub pImageOpaqueBinds: * const VkSparseImageOpaqueMemoryBindInfo,
	pub imageBindCount: u32,
	pub pImageBinds: * const VkSparseImageMemoryBindInfo,
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphores: * const VkSemaphore,
}
impl VkBindSparseInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_SPARSE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageCopy
{
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D,
}
impl VkImageCopy
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageBlit
{
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffsets: [VkOffset3D; 2],
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffsets: [VkOffset3D; 2],
}
impl VkImageBlit
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferImageCopy
{
	pub bufferOffset: VkDeviceSize,
	pub bufferRowLength: u32,
	pub bufferImageHeight: u32,
	pub imageSubresource: VkImageSubresourceLayers,
	pub imageOffset: VkOffset3D,
	pub imageExtent: VkExtent3D,
}
impl VkBufferImageCopy
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageResolve
{
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D,
}
impl VkImageResolve
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkShaderModuleCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkShaderModuleCreateFlagBits,
	pub codeSize: usize,
	pub pCode: * const u32,
}
impl VkShaderModuleCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutBinding
{
	pub binding: u32,
	pub descriptorType: VkDescriptorType,
	pub descriptorCount: u32,
	pub stageFlags: VkShaderStageFlagBits,
	pub pImmutableSamplers: * const VkSampler,
}
impl VkDescriptorSetLayoutBinding
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkDescriptorSetLayoutCreateFlagBits,
	pub bindingCount: u32,
	pub pBindings: * const VkDescriptorSetLayoutBinding,
}
impl VkDescriptorSetLayoutCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolSize
{
	pub type_type: VkDescriptorType,
	pub descriptorCount: u32,
}
impl VkDescriptorPoolSize
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkDescriptorPoolCreateFlagBits,
	pub maxSets: u32,
	pub poolSizeCount: u32,
	pub pPoolSizes: * const VkDescriptorPoolSize,
}
impl VkDescriptorPoolCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub descriptorPool: VkDescriptorPool,
	pub descriptorSetCount: u32,
	pub pSetLayouts: * const VkDescriptorSetLayout,
}
impl VkDescriptorSetAllocateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSpecializationMapEntry
{
	pub constantID: u32,
	pub offset: u32,
	pub size: usize,
}
impl VkSpecializationMapEntry
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSpecializationInfo
{
	pub mapEntryCount: u32,
	pub pMapEntries: * const VkSpecializationMapEntry,
	pub dataSize: usize,
	pub pData: * const c_void,
}
impl VkSpecializationInfo
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineShaderStageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineShaderStageCreateFlagBits,
	pub stage: VkShaderStageFlagBits,
	pub module: VkShaderModule,
	pub pName: * const c_uchar,
	pub pSpecializationInfo: * const VkSpecializationInfo,
}
impl VkPipelineShaderStageCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkComputePipelineCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineCreateFlagBits,
	pub stage: VkPipelineShaderStageCreateInfo,
	pub layout: VkPipelineLayout,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32,
}
impl VkComputePipelineCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputBindingDescription
{
	pub binding: u32,
	pub stride: u32,
	pub inputRate: VkVertexInputRate,
}
impl VkVertexInputBindingDescription
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputAttributeDescription
{
	pub location: u32,
	pub binding: u32,
	pub format: VkFormat,
	pub offset: u32,
}
impl VkVertexInputAttributeDescription
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineVertexInputStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineVertexInputStateCreateFlagBits,
	pub vertexBindingDescriptionCount: u32,
	pub pVertexBindingDescriptions: * const VkVertexInputBindingDescription,
	pub vertexAttributeDescriptionCount: u32,
	pub pVertexAttributeDescriptions: * const VkVertexInputAttributeDescription,
}
impl VkPipelineVertexInputStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineInputAssemblyStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineInputAssemblyStateCreateFlagBits,
	pub topology: VkPrimitiveTopology,
	pub primitiveRestartEnable: VkBool32,
}
impl VkPipelineInputAssemblyStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineTessellationStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineTessellationStateCreateFlagBits,
	pub patchControlPoints: u32,
}
impl VkPipelineTessellationStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineViewportStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineViewportStateCreateFlagBits,
	pub viewportCount: u32,
	pub pViewports: * const VkViewport,
	pub scissorCount: u32,
	pub pScissors: * const VkRect2D,
}
impl VkPipelineViewportStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineRasterizationStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineRasterizationStateCreateFlagBits,
	pub depthClampEnable: VkBool32,
	pub rasterizerDiscardEnable: VkBool32,
	pub polygonMode: VkPolygonMode,
	pub cullMode: VkCullModeFlagBits,
	pub frontFace: VkFrontFace,
	pub depthBiasEnable: VkBool32,
	pub depthBiasConstantFactor: f32,
	pub depthBiasClamp: f32,
	pub depthBiasSlopeFactor: f32,
	pub lineWidth: f32,
}
impl VkPipelineRasterizationStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineMultisampleStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineMultisampleStateCreateFlagBits,
	pub rasterizationSamples: VkSampleCountFlagBits,
	pub sampleShadingEnable: VkBool32,
	pub minSampleShading: f32,
	pub pSampleMask: * const VkSampleMask,
	pub alphaToCoverageEnable: VkBool32,
	pub alphaToOneEnable: VkBool32,
}
impl VkPipelineMultisampleStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineColorBlendAttachmentState
{
	pub blendEnable: VkBool32,
	pub srcColorBlendFactor: VkBlendFactor,
	pub dstColorBlendFactor: VkBlendFactor,
	pub colorBlendOp: VkBlendOp,
	pub srcAlphaBlendFactor: VkBlendFactor,
	pub dstAlphaBlendFactor: VkBlendFactor,
	pub alphaBlendOp: VkBlendOp,
	pub colorWriteMask: VkColorComponentFlagBits,
}
impl VkPipelineColorBlendAttachmentState
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineColorBlendStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineColorBlendStateCreateFlagBits,
	pub logicOpEnable: VkBool32,
	pub logicOp: VkLogicOp,
	pub attachmentCount: u32,
	pub pAttachments: * const VkPipelineColorBlendAttachmentState,
	pub blendConstants: [f32; 4],
}
impl VkPipelineColorBlendStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineDynamicStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineDynamicStateCreateFlagBits,
	pub dynamicStateCount: u32,
	pub pDynamicStates: * const VkDynamicState,
}
impl VkPipelineDynamicStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkStencilOpState
{
	pub failOp: VkStencilOp,
	pub passOp: VkStencilOp,
	pub depthFailOp: VkStencilOp,
	pub compareOp: VkCompareOp,
	pub compareMask: u32,
	pub writeMask: u32,
	pub reference: u32,
}
impl VkStencilOpState
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineDepthStencilStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineDepthStencilStateCreateFlagBits,
	pub depthTestEnable: VkBool32,
	pub depthWriteEnable: VkBool32,
	pub depthCompareOp: VkCompareOp,
	pub depthBoundsTestEnable: VkBool32,
	pub stencilTestEnable: VkBool32,
	pub front: VkStencilOpState,
	pub back: VkStencilOpState,
	pub minDepthBounds: f32,
	pub maxDepthBounds: f32,
}
impl VkPipelineDepthStencilStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGraphicsPipelineCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineCreateFlagBits,
	pub stageCount: u32,
	pub pStages: * const VkPipelineShaderStageCreateInfo,
	pub pVertexInputState: * const VkPipelineVertexInputStateCreateInfo,
	pub pInputAssemblyState: * const VkPipelineInputAssemblyStateCreateInfo,
	pub pTessellationState: * const VkPipelineTessellationStateCreateInfo,
	pub pViewportState: * const VkPipelineViewportStateCreateInfo,
	pub pRasterizationState: * const VkPipelineRasterizationStateCreateInfo,
	pub pMultisampleState: * const VkPipelineMultisampleStateCreateInfo,
	pub pDepthStencilState: * const VkPipelineDepthStencilStateCreateInfo,
	pub pColorBlendState: * const VkPipelineColorBlendStateCreateInfo,
	pub pDynamicState: * const VkPipelineDynamicStateCreateInfo,
	pub layout: VkPipelineLayout,
	pub renderPass: VkRenderPass,
	pub subpass: u32,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32,
}
impl VkGraphicsPipelineCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineCacheCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineCacheCreateFlagBits,
	pub initialDataSize: usize,
	pub pInitialData: * const c_void,
}
impl VkPipelineCacheCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineCacheHeaderVersionOne
{
	pub headerSize: u32,
	pub headerVersion: VkPipelineCacheHeaderVersion,
	pub vendorID: u32,
	pub deviceID: u32,
	pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
}
impl VkPipelineCacheHeaderVersionOne
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPushConstantRange
{
	pub stageFlags: VkShaderStageFlagBits,
	pub offset: u32,
	pub size: u32,
}
impl VkPushConstantRange
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineLayoutCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkPipelineLayoutCreateFlagBits,
	pub setLayoutCount: u32,
	pub pSetLayouts: * const VkDescriptorSetLayout,
	pub pushConstantRangeCount: u32,
	pub pPushConstantRanges: * const VkPushConstantRange,
}
impl VkPipelineLayoutCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkSamplerCreateFlagBits,
	pub magFilter: VkFilter,
	pub minFilter: VkFilter,
	pub mipmapMode: VkSamplerMipmapMode,
	pub addressModeU: VkSamplerAddressMode,
	pub addressModeV: VkSamplerAddressMode,
	pub addressModeW: VkSamplerAddressMode,
	pub mipLodBias: f32,
	pub anisotropyEnable: VkBool32,
	pub maxAnisotropy: f32,
	pub compareEnable: VkBool32,
	pub compareOp: VkCompareOp,
	pub minLod: f32,
	pub maxLod: f32,
	pub borderColor: VkBorderColor,
	pub unnormalizedCoordinates: VkBool32,
}
impl VkSamplerCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandPoolCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkCommandPoolCreateFlagBits,
	pub queueFamilyIndex: u32,
}
impl VkCommandPoolCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub commandPool: VkCommandPool,
	pub level: VkCommandBufferLevel,
	pub commandBufferCount: u32,
}
impl VkCommandBufferAllocateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferInheritanceInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub renderPass: VkRenderPass,
	pub subpass: u32,
	pub framebuffer: VkFramebuffer,
	pub occlusionQueryEnable: VkBool32,
	pub queryFlags: VkQueryControlFlagBits,
	pub pipelineStatistics: VkQueryPipelineStatisticFlagBits,
}
impl VkCommandBufferInheritanceInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkCommandBufferUsageFlagBits,
	pub pInheritanceInfo: * const VkCommandBufferInheritanceInfo,
}
impl VkCommandBufferBeginInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub renderPass: VkRenderPass,
	pub framebuffer: VkFramebuffer,
	pub renderArea: VkRect2D,
	pub clearValueCount: u32,
	pub pClearValues: * const VkClearValue,
}
impl VkRenderPassBeginInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearDepthStencilValue
{
	pub depth: f32,
	pub stencil: u32,
}
impl VkClearDepthStencilValue
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearAttachment
{
	pub aspectMask: VkImageAspectFlagBits,
	pub colorAttachment: u32,
	pub clearValue: VkClearValue,
}
impl VkClearAttachment
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentDescription
{
	pub flags: VkAttachmentDescriptionFlagBits,
	pub format: VkFormat,
	pub samples: VkSampleCountFlagBits,
	pub loadOp: VkAttachmentLoadOp,
	pub storeOp: VkAttachmentStoreOp,
	pub stencilLoadOp: VkAttachmentLoadOp,
	pub stencilStoreOp: VkAttachmentStoreOp,
	pub initialLayout: VkImageLayout,
	pub finalLayout: VkImageLayout,
}
impl VkAttachmentDescription
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReference
{
	pub attachment: u32,
	pub layout: VkImageLayout,
}
impl VkAttachmentReference
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDescription
{
	pub flags: VkSubpassDescriptionFlagBits,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub inputAttachmentCount: u32,
	pub pInputAttachments: * const VkAttachmentReference,
	pub colorAttachmentCount: u32,
	pub pColorAttachments: * const VkAttachmentReference,
	pub pResolveAttachments: * const VkAttachmentReference,
	pub pDepthStencilAttachment: * const VkAttachmentReference,
	pub preserveAttachmentCount: u32,
	pub pPreserveAttachments: * const u32,
}
impl VkSubpassDescription
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDependency
{
	pub srcSubpass: u32,
	pub dstSubpass: u32,
	pub srcStageMask: VkPipelineStageFlagBits,
	pub dstStageMask: VkPipelineStageFlagBits,
	pub srcAccessMask: VkAccessFlagBits,
	pub dstAccessMask: VkAccessFlagBits,
	pub dependencyFlags: VkDependencyFlagBits,
}
impl VkSubpassDependency
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkRenderPassCreateFlagBits,
	pub attachmentCount: u32,
	pub pAttachments: * const VkAttachmentDescription,
	pub subpassCount: u32,
	pub pSubpasses: * const VkSubpassDescription,
	pub dependencyCount: u32,
	pub pDependencies: * const VkSubpassDependency,
}
impl VkRenderPassCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkEventCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkEventCreateFlagBits,
}
impl VkEventCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EVENT_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFenceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkFenceCreateFlagBits,
}
impl VkFenceCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFeatures
{
	pub robustBufferAccess: VkBool32,
	pub fullDrawIndexUint32: VkBool32,
	pub imageCubeArray: VkBool32,
	pub independentBlend: VkBool32,
	pub geometryShader: VkBool32,
	pub tessellationShader: VkBool32,
	pub sampleRateShading: VkBool32,
	pub dualSrcBlend: VkBool32,
	pub logicOp: VkBool32,
	pub multiDrawIndirect: VkBool32,
	pub drawIndirectFirstInstance: VkBool32,
	pub depthClamp: VkBool32,
	pub depthBiasClamp: VkBool32,
	pub fillModeNonSolid: VkBool32,
	pub depthBounds: VkBool32,
	pub wideLines: VkBool32,
	pub largePoints: VkBool32,
	pub alphaToOne: VkBool32,
	pub multiViewport: VkBool32,
	pub samplerAnisotropy: VkBool32,
	pub textureCompressionETC2: VkBool32,
	pub textureCompressionASTC_LDR: VkBool32,
	pub textureCompressionBC: VkBool32,
	pub occlusionQueryPrecise: VkBool32,
	pub pipelineStatisticsQuery: VkBool32,
	pub vertexPipelineStoresAndAtomics: VkBool32,
	pub fragmentStoresAndAtomics: VkBool32,
	pub shaderTessellationAndGeometryPointSize: VkBool32,
	pub shaderImageGatherExtended: VkBool32,
	pub shaderStorageImageExtendedFormats: VkBool32,
	pub shaderStorageImageMultisample: VkBool32,
	pub shaderStorageImageReadWithoutFormat: VkBool32,
	pub shaderStorageImageWriteWithoutFormat: VkBool32,
	pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
	pub shaderSampledImageArrayDynamicIndexing: VkBool32,
	pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageImageArrayDynamicIndexing: VkBool32,
	pub shaderClipDistance: VkBool32,
	pub shaderCullDistance: VkBool32,
	pub shaderFloat64: VkBool32,
	pub shaderInt64: VkBool32,
	pub shaderInt16: VkBool32,
	pub shaderResourceResidency: VkBool32,
	pub shaderResourceMinLod: VkBool32,
	pub sparseBinding: VkBool32,
	pub sparseResidencyBuffer: VkBool32,
	pub sparseResidencyImage2D: VkBool32,
	pub sparseResidencyImage3D: VkBool32,
	pub sparseResidency2Samples: VkBool32,
	pub sparseResidency4Samples: VkBool32,
	pub sparseResidency8Samples: VkBool32,
	pub sparseResidency16Samples: VkBool32,
	pub sparseResidencyAliased: VkBool32,
	pub variableMultisampleRate: VkBool32,
	pub inheritedQueries: VkBool32,
}
impl VkPhysicalDeviceFeatures
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSparseProperties
{
	pub residencyStandard2DBlockShape: VkBool32,
	pub residencyStandard2DMultisampleBlockShape: VkBool32,
	pub residencyStandard3DBlockShape: VkBool32,
	pub residencyAlignedMipSize: VkBool32,
	pub residencyNonResidentStrict: VkBool32,
}
impl VkPhysicalDeviceSparseProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceLimits
{
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
	pub bufferImageGranularity: VkDeviceSize,
	pub sparseAddressSpaceSize: VkDeviceSize,
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
	pub maxComputeWorkGroupCount: [u32; 3],
	pub maxComputeWorkGroupInvocations: u32,
	pub maxComputeWorkGroupSize: [u32; 3],
	pub subPixelPrecisionBits: u32,
	pub subTexelPrecisionBits: u32,
	pub mipmapPrecisionBits: u32,
	pub maxDrawIndexedIndexValue: u32,
	pub maxDrawIndirectCount: u32,
	pub maxSamplerLodBias: f32,
	pub maxSamplerAnisotropy: f32,
	pub maxViewports: u32,
	pub maxViewportDimensions: [u32; 2],
	pub viewportBoundsRange: [f32; 2],
	pub viewportSubPixelBits: u32,
	pub minMemoryMapAlignment: usize,
	pub minTexelBufferOffsetAlignment: VkDeviceSize,
	pub minUniformBufferOffsetAlignment: VkDeviceSize,
	pub minStorageBufferOffsetAlignment: VkDeviceSize,
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
	pub framebufferColorSampleCounts: VkSampleCountFlagBits,
	pub framebufferDepthSampleCounts: VkSampleCountFlagBits,
	pub framebufferStencilSampleCounts: VkSampleCountFlagBits,
	pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlagBits,
	pub maxColorAttachments: u32,
	pub sampledImageColorSampleCounts: VkSampleCountFlagBits,
	pub sampledImageIntegerSampleCounts: VkSampleCountFlagBits,
	pub sampledImageDepthSampleCounts: VkSampleCountFlagBits,
	pub sampledImageStencilSampleCounts: VkSampleCountFlagBits,
	pub storageImageSampleCounts: VkSampleCountFlagBits,
	pub maxSampleMaskWords: u32,
	pub timestampComputeAndGraphics: VkBool32,
	pub timestampPeriod: f32,
	pub maxClipDistances: u32,
	pub maxCullDistances: u32,
	pub maxCombinedClipAndCullDistances: u32,
	pub discreteQueuePriorities: u32,
	pub pointSizeRange: [f32; 2],
	pub lineWidthRange: [f32; 2],
	pub pointSizeGranularity: f32,
	pub lineWidthGranularity: f32,
	pub strictLines: VkBool32,
	pub standardSampleLocations: VkBool32,
	pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
	pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
	pub nonCoherentAtomSize: VkDeviceSize,
}
impl VkPhysicalDeviceLimits
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkSemaphoreCreateFlagBits,
}
impl VkSemaphoreCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueryPoolCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkQueryPoolCreateFlagBits,
	pub queryType: VkQueryType,
	pub queryCount: u32,
	pub pipelineStatistics: VkQueryPipelineStatisticFlagBits,
}
impl VkQueryPoolCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkFramebufferCreateFlagBits,
	pub renderPass: VkRenderPass,
	pub attachmentCount: u32,
	pub pAttachments: * const VkImageView,
	pub width: u32,
	pub height: u32,
	pub layers: u32,
}
impl VkFramebufferCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawIndirectCommand
{
	pub vertexCount: u32,
	pub instanceCount: u32,
	pub firstVertex: u32,
	pub firstInstance: u32,
}
impl VkDrawIndirectCommand
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawIndexedIndirectCommand
{
	pub indexCount: u32,
	pub instanceCount: u32,
	pub firstIndex: u32,
	pub vertexOffset: i32,
	pub firstInstance: u32,
}
impl VkDrawIndexedIndirectCommand
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDispatchIndirectCommand
{
	pub x: u32,
	pub y: u32,
	pub z: u32,
}
impl VkDispatchIndirectCommand
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: * const VkSemaphore,
	pub pWaitDstStageMask: * const VkPipelineStageFlagBits,
	pub commandBufferCount: u32,
	pub pCommandBuffers: * const VkCommandBuffer,
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphores: * const VkSemaphore,
}
impl VkSubmitInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFeatures2
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub features: VkPhysicalDeviceFeatures,
}
impl VkPhysicalDeviceFeatures2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProperties2
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub properties: VkPhysicalDeviceProperties,
}
impl VkPhysicalDeviceProperties2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFormatProperties2
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub formatProperties: VkFormatProperties,
}
impl VkFormatProperties2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatProperties2
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub imageFormatProperties: VkImageFormatProperties,
}
impl VkImageFormatProperties2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceImageFormatInfo2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub format: VkFormat,
	pub type_type: VkImageType,
	pub tiling: VkImageTiling,
	pub usage: VkImageUsageFlagBits,
	pub flags: VkImageCreateFlagBits,
}
impl VkPhysicalDeviceImageFormatInfo2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueueFamilyProperties2
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub queueFamilyProperties: VkQueueFamilyProperties,
}
impl VkQueueFamilyProperties2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties2
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}
impl VkPhysicalDeviceMemoryProperties2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageFormatProperties2
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub properties: VkSparseImageFormatProperties,
}
impl VkSparseImageFormatProperties2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub format: VkFormat,
	pub type_type: VkImageType,
	pub samples: VkSampleCountFlagBits,
	pub usage: VkImageUsageFlagBits,
	pub tiling: VkImageTiling,
}
impl VkPhysicalDeviceSparseImageFormatInfo2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkConformanceVersion
{
	pub major: u8,
	pub minor: u8,
	pub subminor: u8,
	pub patch: u8,
}
impl VkConformanceVersion
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceDriverProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub driverID: VkDriverId,
	pub driverName: [c_uchar; VK_MAX_DRIVER_NAME_SIZE],
	pub driverInfo: [c_uchar; VK_MAX_DRIVER_INFO_SIZE],
	pub conformanceVersion: VkConformanceVersion,
}
impl VkPhysicalDeviceDriverProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVariablePointersFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub variablePointersStorageBuffer: VkBool32,
	pub variablePointers: VkBool32,
}
impl VkPhysicalDeviceVariablePointersFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVariablePointerFeatures
{
}
impl VkPhysicalDeviceVariablePointerFeatures
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalMemoryProperties
{
	pub externalMemoryFeatures: VkExternalMemoryFeatureFlagBits,
	pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagBits,
	pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagBits,
}
impl VkExternalMemoryProperties
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalImageFormatInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl VkPhysicalDeviceExternalImageFormatInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalImageFormatProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub externalMemoryProperties: VkExternalMemoryProperties,
}
impl VkExternalImageFormatProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalBufferInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkBufferCreateFlagBits,
	pub usage: VkBufferUsageFlagBits,
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl VkPhysicalDeviceExternalBufferInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalBufferProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub externalMemoryProperties: VkExternalMemoryProperties,
}
impl VkExternalBufferProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceIDProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub deviceUUID: [u8; VK_UUID_SIZE],
	pub driverUUID: [u8; VK_UUID_SIZE],
	pub deviceLUID: [u8; VK_LUID_SIZE],
	pub deviceNodeMask: u32,
	pub deviceLUIDValid: VkBool32,
}
impl VkPhysicalDeviceIDProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalMemoryImageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub handleTypes: VkExternalMemoryHandleTypeFlagBits,
}
impl VkExternalMemoryImageCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalMemoryBufferCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub handleTypes: VkExternalMemoryHandleTypeFlagBits,
}
impl VkExternalMemoryBufferCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportMemoryAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub handleTypes: VkExternalMemoryHandleTypeFlagBits,
}
impl VkExportMemoryAllocateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl VkPhysicalDeviceExternalSemaphoreInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalSemaphoreProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagBits,
	pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagBits,
	pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagBits,
}
impl VkExternalSemaphoreProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportSemaphoreCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub handleTypes: VkExternalSemaphoreHandleTypeFlagBits,
}
impl VkExportSemaphoreCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalFenceInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub handleType: VkExternalFenceHandleTypeFlagBits,
}
impl VkPhysicalDeviceExternalFenceInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalFenceProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlagBits,
	pub compatibleHandleTypes: VkExternalFenceHandleTypeFlagBits,
	pub externalFenceFeatures: VkExternalFenceFeatureFlagBits,
}
impl VkExternalFenceProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportFenceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub handleTypes: VkExternalFenceHandleTypeFlagBits,
}
impl VkExportFenceCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMultiviewFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub multiview: VkBool32,
	pub multiviewGeometryShader: VkBool32,
	pub multiviewTessellationShader: VkBool32,
}
impl VkPhysicalDeviceMultiviewFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMultiviewProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub maxMultiviewViewCount: u32,
	pub maxMultiviewInstanceIndex: u32,
}
impl VkPhysicalDeviceMultiviewProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassMultiviewCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub subpassCount: u32,
	pub pViewMasks: * const u32,
	pub dependencyCount: u32,
	pub pViewOffsets: * const i32,
	pub correlationMaskCount: u32,
	pub pCorrelationMasks: * const u32,
}
impl VkRenderPassMultiviewCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceGroupProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub physicalDeviceCount: u32,
	pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE],
	pub subsetAllocation: VkBool32,
}
impl VkPhysicalDeviceGroupProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryAllocateFlagsInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkMemoryAllocateFlagBits,
	pub deviceMask: u32,
}
impl VkMemoryAllocateFlagsInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindBufferMemoryInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub buffer: VkBuffer,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
}
impl VkBindBufferMemoryInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindBufferMemoryDeviceGroupInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub deviceIndexCount: u32,
	pub pDeviceIndices: * const u32,
}
impl VkBindBufferMemoryDeviceGroupInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindImageMemoryInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub image: VkImage,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
}
impl VkBindImageMemoryInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindImageMemoryDeviceGroupInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub deviceIndexCount: u32,
	pub pDeviceIndices: * const u32,
	pub splitInstanceBindRegionCount: u32,
	pub pSplitInstanceBindRegions: * const VkRect2D,
}
impl VkBindImageMemoryDeviceGroupInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupRenderPassBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub deviceMask: u32,
	pub deviceRenderAreaCount: u32,
	pub pDeviceRenderAreas: * const VkRect2D,
}
impl VkDeviceGroupRenderPassBeginInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupCommandBufferBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub deviceMask: u32,
}
impl VkDeviceGroupCommandBufferBeginInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphoreDeviceIndices: * const u32,
	pub commandBufferCount: u32,
	pub pCommandBufferDeviceMasks: * const u32,
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphoreDeviceIndices: * const u32,
}
impl VkDeviceGroupSubmitInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupBindSparseInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub resourceDeviceIndex: u32,
	pub memoryDeviceIndex: u32,
}
impl VkDeviceGroupBindSparseInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupDeviceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub physicalDeviceCount: u32,
	pub pPhysicalDevices: * const VkPhysicalDevice,
}
impl VkDeviceGroupDeviceCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorUpdateTemplateEntry
{
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
	pub descriptorType: VkDescriptorType,
	pub offset: usize,
	pub stride: usize,
}
impl VkDescriptorUpdateTemplateEntry
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorUpdateTemplateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkDescriptorUpdateTemplateCreateFlagBits,
	pub descriptorUpdateEntryCount: u32,
	pub pDescriptorUpdateEntries: * const VkDescriptorUpdateTemplateEntry,
	pub templateType: VkDescriptorUpdateTemplateType,
	pub descriptorSetLayout: VkDescriptorSetLayout,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub pipelineLayout: VkPipelineLayout,
	pub set: u32,
}
impl VkDescriptorUpdateTemplateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInputAttachmentAspectReference
{
	pub subpass: u32,
	pub inputAttachmentIndex: u32,
	pub aspectMask: VkImageAspectFlagBits,
}
impl VkInputAttachmentAspectReference
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub aspectReferenceCount: u32,
	pub pAspectReferences: * const VkInputAttachmentAspectReference,
}
impl VkRenderPassInputAttachmentAspectCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDevice16BitStorageFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub storageBuffer16BitAccess: VkBool32,
	pub uniformAndStorageBuffer16BitAccess: VkBool32,
	pub storagePushConstant16: VkBool32,
	pub storageInputOutput16: VkBool32,
}
impl VkPhysicalDevice16BitStorageFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSubgroupProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub subgroupSize: u32,
	pub supportedStages: VkShaderStageFlagBits,
	pub supportedOperations: VkSubgroupFeatureFlagBits,
	pub quadOperationsInAllStages: VkBool32,
}
impl VkPhysicalDeviceSubgroupProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub shaderSubgroupExtendedTypes: VkBool32,
}
impl VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferMemoryRequirementsInfo2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub buffer: VkBuffer,
}
impl VkBufferMemoryRequirementsInfo2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageMemoryRequirementsInfo2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub image: VkImage,
}
impl VkImageMemoryRequirementsInfo2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSparseMemoryRequirementsInfo2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub image: VkImage,
}
impl VkImageSparseMemoryRequirementsInfo2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryRequirements2
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub memoryRequirements: VkMemoryRequirements,
}
impl VkMemoryRequirements2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryRequirements2
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub memoryRequirements: VkSparseImageMemoryRequirements,
}
impl VkSparseImageMemoryRequirements2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDevicePointClippingProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub pointClippingBehavior: VkPointClippingBehavior,
}
impl VkPhysicalDevicePointClippingProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryDedicatedRequirements
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub prefersDedicatedAllocation: VkBool32,
	pub requiresDedicatedAllocation: VkBool32,
}
impl VkMemoryDedicatedRequirements
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryDedicatedAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub image: VkImage,
	pub buffer: VkBuffer,
}
impl VkMemoryDedicatedAllocateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageViewUsageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub usage: VkImageUsageFlagBits,
}
impl VkImageViewUsageCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub domainOrigin: VkTessellationDomainOrigin,
}
impl VkPipelineTessellationDomainOriginStateCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerYcbcrConversionInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub conversion: VkSamplerYcbcrConversion,
}
impl VkSamplerYcbcrConversionInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerYcbcrConversionCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub format: VkFormat,
	pub ycbcrModel: VkSamplerYcbcrModelConversion,
	pub ycbcrRange: VkSamplerYcbcrRange,
	pub components: VkComponentMapping,
	pub xChromaOffset: VkChromaLocation,
	pub yChromaOffset: VkChromaLocation,
	pub chromaFilter: VkFilter,
	pub forceExplicitReconstruction: VkBool32,
}
impl VkSamplerYcbcrConversionCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindImagePlaneMemoryInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub planeAspect: VkImageAspectFlagBits,
}
impl VkBindImagePlaneMemoryInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImagePlaneMemoryRequirementsInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub planeAspect: VkImageAspectFlagBits,
}
impl VkImagePlaneMemoryRequirementsInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub samplerYcbcrConversion: VkBool32,
}
impl VkPhysicalDeviceSamplerYcbcrConversionFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerYcbcrConversionImageFormatProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub combinedImageSamplerDescriptorCount: u32,
}
impl VkSamplerYcbcrConversionImageFormatProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkProtectedSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub protectedSubmit: VkBool32,
}
impl VkProtectedSubmitInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub protectedMemory: VkBool32,
}
impl VkPhysicalDeviceProtectedMemoryFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProtectedMemoryProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub protectedNoFault: VkBool32,
}
impl VkPhysicalDeviceProtectedMemoryProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceQueueInfo2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkDeviceQueueCreateFlagBits,
	pub queueFamilyIndex: u32,
	pub queueIndex: u32,
}
impl VkDeviceQueueInfo2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub filterMinmaxSingleComponentFormats: VkBool32,
	pub filterMinmaxImageComponentMapping: VkBool32,
}
impl VkPhysicalDeviceSamplerFilterMinmaxProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerReductionModeCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub reductionMode: VkSamplerReductionMode,
}
impl VkSamplerReductionModeCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatListCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub viewFormatCount: u32,
	pub pViewFormats: * const VkFormat,
}
impl VkImageFormatListCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMaintenance3Properties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub maxPerSetDescriptors: u32,
	pub maxMemoryAllocationSize: VkDeviceSize,
}
impl VkPhysicalDeviceMaintenance3Properties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutSupport
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub supported: VkBool32,
}
impl VkDescriptorSetLayoutSupport
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub shaderDrawParameters: VkBool32,
}
impl VkPhysicalDeviceShaderDrawParametersFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceShaderDrawParameterFeatures
{
}
impl VkPhysicalDeviceShaderDrawParameterFeatures
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub shaderFloat16: VkBool32,
	pub shaderInt8: VkBool32,
}
impl VkPhysicalDeviceShaderFloat16Int8Features
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFloatControlsProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
	pub roundingModeIndependence: VkShaderFloatControlsIndependence,
	pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
	pub shaderDenormPreserveFloat16: VkBool32,
	pub shaderDenormPreserveFloat32: VkBool32,
	pub shaderDenormPreserveFloat64: VkBool32,
	pub shaderDenormFlushToZeroFloat16: VkBool32,
	pub shaderDenormFlushToZeroFloat32: VkBool32,
	pub shaderDenormFlushToZeroFloat64: VkBool32,
	pub shaderRoundingModeRTEFloat16: VkBool32,
	pub shaderRoundingModeRTEFloat32: VkBool32,
	pub shaderRoundingModeRTEFloat64: VkBool32,
	pub shaderRoundingModeRTZFloat16: VkBool32,
	pub shaderRoundingModeRTZFloat32: VkBool32,
	pub shaderRoundingModeRTZFloat64: VkBool32,
}
impl VkPhysicalDeviceFloatControlsProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceHostQueryResetFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub hostQueryReset: VkBool32,
}
impl VkPhysicalDeviceHostQueryResetFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
	pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
	pub descriptorBindingPartiallyBound: VkBool32,
	pub descriptorBindingVariableDescriptorCount: VkBool32,
	pub runtimeDescriptorArray: VkBool32,
}
impl VkPhysicalDeviceDescriptorIndexingFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub maxUpdateAfterBindDescriptorsInAllPools: u32,
	pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
	pub robustBufferAccessUpdateAfterBind: VkBool32,
	pub quadDivergentImplicitLod: VkBool32,
	pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
	pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
	pub maxPerStageUpdateAfterBindResources: u32,
	pub maxDescriptorSetUpdateAfterBindSamplers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
	pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
	pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
}
impl VkPhysicalDeviceDescriptorIndexingProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub bindingCount: u32,
	pub pBindingFlags: * const VkDescriptorBindingFlagBits,
}
impl VkDescriptorSetLayoutBindingFlagsCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub descriptorSetCount: u32,
	pub pDescriptorCounts: * const u32,
}
impl VkDescriptorSetVariableDescriptorCountAllocateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub maxVariableDescriptorCount: u32,
}
impl VkDescriptorSetVariableDescriptorCountLayoutSupport
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentDescription2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkAttachmentDescriptionFlagBits,
	pub format: VkFormat,
	pub samples: VkSampleCountFlagBits,
	pub loadOp: VkAttachmentLoadOp,
	pub storeOp: VkAttachmentStoreOp,
	pub stencilLoadOp: VkAttachmentLoadOp,
	pub stencilStoreOp: VkAttachmentStoreOp,
	pub initialLayout: VkImageLayout,
	pub finalLayout: VkImageLayout,
}
impl VkAttachmentDescription2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReference2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub attachment: u32,
	pub layout: VkImageLayout,
	pub aspectMask: VkImageAspectFlagBits,
}
impl VkAttachmentReference2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDescription2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkSubpassDescriptionFlagBits,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub viewMask: u32,
	pub inputAttachmentCount: u32,
	pub pInputAttachments: * const VkAttachmentReference2,
	pub colorAttachmentCount: u32,
	pub pColorAttachments: * const VkAttachmentReference2,
	pub pResolveAttachments: * const VkAttachmentReference2,
	pub pDepthStencilAttachment: * const VkAttachmentReference2,
	pub preserveAttachmentCount: u32,
	pub pPreserveAttachments: * const u32,
}
impl VkSubpassDescription2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDependency2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub srcSubpass: u32,
	pub dstSubpass: u32,
	pub srcStageMask: VkPipelineStageFlagBits,
	pub dstStageMask: VkPipelineStageFlagBits,
	pub srcAccessMask: VkAccessFlagBits,
	pub dstAccessMask: VkAccessFlagBits,
	pub dependencyFlags: VkDependencyFlagBits,
	pub viewOffset: i32,
}
impl VkSubpassDependency2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassCreateInfo2
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkRenderPassCreateFlagBits,
	pub attachmentCount: u32,
	pub pAttachments: * const VkAttachmentDescription2,
	pub subpassCount: u32,
	pub pSubpasses: * const VkSubpassDescription2,
	pub dependencyCount: u32,
	pub pDependencies: * const VkSubpassDependency2,
	pub correlatedViewMaskCount: u32,
	pub pCorrelatedViewMasks: * const u32,
}
impl VkRenderPassCreateInfo2
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub contents: VkSubpassContents,
}
impl VkSubpassBeginInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassEndInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
}
impl VkSubpassEndInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_END_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub timelineSemaphore: VkBool32,
}
impl VkPhysicalDeviceTimelineSemaphoreFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub maxTimelineSemaphoreValueDifference: u64,
}
impl VkPhysicalDeviceTimelineSemaphoreProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreTypeCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub semaphoreType: VkSemaphoreType,
	pub initialValue: u64,
}
impl VkSemaphoreTypeCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkTimelineSemaphoreSubmitInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub waitSemaphoreValueCount: u32,
	pub pWaitSemaphoreValues: * const u64,
	pub signalSemaphoreValueCount: u32,
	pub pSignalSemaphoreValues: * const u64,
}
impl VkTimelineSemaphoreSubmitInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreWaitInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkSemaphoreWaitFlagBits,
	pub semaphoreCount: u32,
	pub pSemaphores: * const VkSemaphore,
	pub pValues: * const u64,
}
impl VkSemaphoreWaitInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreSignalInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub semaphore: VkSemaphore,
	pub value: u64,
}
impl VkSemaphoreSignalInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDevice8BitStorageFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub storageBuffer8BitAccess: VkBool32,
	pub uniformAndStorageBuffer8BitAccess: VkBool32,
	pub storagePushConstant8: VkBool32,
}
impl VkPhysicalDevice8BitStorageFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub vulkanMemoryModel: VkBool32,
	pub vulkanMemoryModelDeviceScope: VkBool32,
	pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
}
impl VkPhysicalDeviceVulkanMemoryModelFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub shaderBufferInt64Atomics: VkBool32,
	pub shaderSharedInt64Atomics: VkBool32,
}
impl VkPhysicalDeviceShaderAtomicInt64Features
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub supportedDepthResolveModes: VkResolveModeFlagBits,
	pub supportedStencilResolveModes: VkResolveModeFlagBits,
	pub independentResolveNone: VkBool32,
	pub independentResolve: VkBool32,
}
impl VkPhysicalDeviceDepthStencilResolveProperties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDescriptionDepthStencilResolve
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub depthResolveMode: VkResolveModeFlagBits,
	pub stencilResolveMode: VkResolveModeFlagBits,
	pub pDepthStencilResolveAttachment: * const VkAttachmentReference2,
}
impl VkSubpassDescriptionDepthStencilResolve
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageStencilUsageCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub stencilUsage: VkImageUsageFlagBits,
}
impl VkImageStencilUsageCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub scalarBlockLayout: VkBool32,
}
impl VkPhysicalDeviceScalarBlockLayoutFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub uniformBufferStandardLayout: VkBool32,
}
impl VkPhysicalDeviceUniformBufferStandardLayoutFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub bufferDeviceAddress: VkBool32,
	pub bufferDeviceAddressCaptureReplay: VkBool32,
	pub bufferDeviceAddressMultiDevice: VkBool32,
}
impl VkPhysicalDeviceBufferDeviceAddressFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferDeviceAddressInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub buffer: VkBuffer,
}
impl VkBufferDeviceAddressInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub opaqueCaptureAddress: u64,
}
impl VkBufferOpaqueCaptureAddressCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub imagelessFramebuffer: VkBool32,
}
impl VkPhysicalDeviceImagelessFramebufferFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferAttachmentsCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub attachmentImageInfoCount: u32,
	pub pAttachmentImageInfos: * const VkFramebufferAttachmentImageInfo,
}
impl VkFramebufferAttachmentsCreateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferAttachmentImageInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub flags: VkImageCreateFlagBits,
	pub usage: VkImageUsageFlagBits,
	pub width: u32,
	pub height: u32,
	pub layerCount: u32,
	pub viewFormatCount: u32,
	pub pViewFormats: * const VkFormat,
}
impl VkFramebufferAttachmentImageInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassAttachmentBeginInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub attachmentCount: u32,
	pub pAttachments: * const VkImageView,
}
impl VkRenderPassAttachmentBeginInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub separateDepthStencilLayouts: VkBool32,
}
impl VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReferenceStencilLayout
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub stencilLayout: VkImageLayout,
}
impl VkAttachmentReferenceStencilLayout
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentDescriptionStencilLayout
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub stencilInitialLayout: VkImageLayout,
	pub stencilFinalLayout: VkImageLayout,
}
impl VkAttachmentDescriptionStencilLayout
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub opaqueCaptureAddress: u64,
}
impl VkMemoryOpaqueCaptureAddressAllocateInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo
{
	pub sType: VkStructureType,
	pub pNext: * const c_void,
	pub memory: VkDeviceMemory,
}
impl VkDeviceMemoryOpaqueCaptureAddressInfo
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVulkan11Features
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub storageBuffer16BitAccess: VkBool32,
	pub uniformAndStorageBuffer16BitAccess: VkBool32,
	pub storagePushConstant16: VkBool32,
	pub storageInputOutput16: VkBool32,
	pub multiview: VkBool32,
	pub multiviewGeometryShader: VkBool32,
	pub multiviewTessellationShader: VkBool32,
	pub variablePointersStorageBuffer: VkBool32,
	pub variablePointers: VkBool32,
	pub protectedMemory: VkBool32,
	pub samplerYcbcrConversion: VkBool32,
	pub shaderDrawParameters: VkBool32,
}
impl VkPhysicalDeviceVulkan11Features
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVulkan11Properties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub deviceUUID: [u8; VK_UUID_SIZE],
	pub driverUUID: [u8; VK_UUID_SIZE],
	pub deviceLUID: [u8; VK_LUID_SIZE],
	pub deviceNodeMask: u32,
	pub deviceLUIDValid: VkBool32,
	pub subgroupSize: u32,
	pub subgroupSupportedStages: VkShaderStageFlagBits,
	pub subgroupSupportedOperations: VkSubgroupFeatureFlagBits,
	pub subgroupQuadOperationsInAllStages: VkBool32,
	pub pointClippingBehavior: VkPointClippingBehavior,
	pub maxMultiviewViewCount: u32,
	pub maxMultiviewInstanceIndex: u32,
	pub protectedNoFault: VkBool32,
	pub maxPerSetDescriptors: u32,
	pub maxMemoryAllocationSize: VkDeviceSize,
}
impl VkPhysicalDeviceVulkan11Properties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVulkan12Features
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub samplerMirrorClampToEdge: VkBool32,
	pub drawIndirectCount: VkBool32,
	pub storageBuffer8BitAccess: VkBool32,
	pub uniformAndStorageBuffer8BitAccess: VkBool32,
	pub storagePushConstant8: VkBool32,
	pub shaderBufferInt64Atomics: VkBool32,
	pub shaderSharedInt64Atomics: VkBool32,
	pub shaderFloat16: VkBool32,
	pub shaderInt8: VkBool32,
	pub descriptorIndexing: VkBool32,
	pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
	pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
	pub descriptorBindingPartiallyBound: VkBool32,
	pub descriptorBindingVariableDescriptorCount: VkBool32,
	pub runtimeDescriptorArray: VkBool32,
	pub samplerFilterMinmax: VkBool32,
	pub scalarBlockLayout: VkBool32,
	pub imagelessFramebuffer: VkBool32,
	pub uniformBufferStandardLayout: VkBool32,
	pub shaderSubgroupExtendedTypes: VkBool32,
	pub separateDepthStencilLayouts: VkBool32,
	pub hostQueryReset: VkBool32,
	pub timelineSemaphore: VkBool32,
	pub bufferDeviceAddress: VkBool32,
	pub bufferDeviceAddressCaptureReplay: VkBool32,
	pub bufferDeviceAddressMultiDevice: VkBool32,
	pub vulkanMemoryModel: VkBool32,
	pub vulkanMemoryModelDeviceScope: VkBool32,
	pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
	pub shaderOutputViewportIndex: VkBool32,
	pub shaderOutputLayer: VkBool32,
	pub subgroupBroadcastDynamicId: VkBool32,
}
impl VkPhysicalDeviceVulkan12Features
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVulkan12Properties
{
	pub sType: VkStructureType,
	pub pNext: * mut c_void,
	pub driverID: VkDriverId,
	pub driverName: [c_uchar; VK_MAX_DRIVER_NAME_SIZE],
	pub driverInfo: [c_uchar; VK_MAX_DRIVER_INFO_SIZE],
	pub conformanceVersion: VkConformanceVersion,
	pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
	pub roundingModeIndependence: VkShaderFloatControlsIndependence,
	pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
	pub shaderDenormPreserveFloat16: VkBool32,
	pub shaderDenormPreserveFloat32: VkBool32,
	pub shaderDenormPreserveFloat64: VkBool32,
	pub shaderDenormFlushToZeroFloat16: VkBool32,
	pub shaderDenormFlushToZeroFloat32: VkBool32,
	pub shaderDenormFlushToZeroFloat64: VkBool32,
	pub shaderRoundingModeRTEFloat16: VkBool32,
	pub shaderRoundingModeRTEFloat32: VkBool32,
	pub shaderRoundingModeRTEFloat64: VkBool32,
	pub shaderRoundingModeRTZFloat16: VkBool32,
	pub shaderRoundingModeRTZFloat32: VkBool32,
	pub shaderRoundingModeRTZFloat64: VkBool32,
	pub maxUpdateAfterBindDescriptorsInAllPools: u32,
	pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
	pub robustBufferAccessUpdateAfterBind: VkBool32,
	pub quadDivergentImplicitLod: VkBool32,
	pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
	pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
	pub maxPerStageUpdateAfterBindResources: u32,
	pub maxDescriptorSetUpdateAfterBindSamplers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
	pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
	pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
	pub supportedDepthResolveModes: VkResolveModeFlagBits,
	pub supportedStencilResolveModes: VkResolveModeFlagBits,
	pub independentResolveNone: VkBool32,
	pub independentResolve: VkBool32,
	pub filterMinmaxSingleComponentFormats: VkBool32,
	pub filterMinmaxImageComponentMapping: VkBool32,
	pub maxTimelineSemaphoreValueDifference: u64,
	pub framebufferIntegerColorSampleCounts: VkSampleCountFlagBits,
}
impl VkPhysicalDeviceVulkan12Properties
{
    pub fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES;

        return s;
    }
}

