#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBaseOutStructure
{
	sType: VkStructureType,
	pNext: * const VkBaseOutStructure,
}
impl VkBaseOutStructure
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBaseInStructure
{
	sType: VkStructureType,
	pNext: * const VkBaseInStructure,
}
impl VkBaseInStructure
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset2D
{
	x: i32,
	y: i32,
}
impl VkOffset2D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset3D
{
	x: i32,
	y: i32,
	z: i32,
}
impl VkOffset3D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent2D
{
	width: u32,
	height: u32,
}
impl VkExtent2D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent3D
{
	width: u32,
	height: u32,
	depth: u32,
}
impl VkExtent3D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkViewport
{
	x: f32,
	y: f32,
	width: f32,
	height: f32,
	minDepth: f32,
	maxDepth: f32,
}
impl VkViewport
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRect2D
{
	offset: VkOffset2D,
	extent: VkExtent2D,
}
impl VkRect2D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearRect
{
	rect: VkRect2D,
	baseArrayLayer: u32,
	layerCount: u32,
}
impl VkClearRect
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkComponentMapping
{
	r: VkComponentSwizzle,
	g: VkComponentSwizzle,
	b: VkComponentSwizzle,
	a: VkComponentSwizzle,
}
impl VkComponentMapping
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProperties
{
	apiVersion: u32,
	driverVersion: u32,
	vendorID: u32,
	deviceID: u32,
	deviceType: VkPhysicalDeviceType,
	deviceName: c_char,
	pipelineCacheUUID: u8,
	limits: VkPhysicalDeviceLimits,
	sparseProperties: VkPhysicalDeviceSparseProperties,
}
impl VkPhysicalDeviceProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtensionProperties
{
	extensionName: c_char,
	specVersion: u32,
}
impl VkExtensionProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkLayerProperties
{
	layerName: c_char,
	specVersion: u32,
	implementationVersion: u32,
	description: c_char,
}
impl VkLayerProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkApplicationInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	pApplicationName: * const c_char,
	applicationVersion: u32,
	pEngineName: * const c_char,
	engineVersion: u32,
	apiVersion: u32,
}
impl VkApplicationInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAllocationCallbacks
{
	pUserData: * const c_void,
	pfnAllocation: PFN_vkAllocationFunction,
	pfnReallocation: PFN_vkReallocationFunction,
	pfnFree: PFN_vkFreeFunction,
	pfnInternalAllocation: PFN_vkInternalAllocationNotification,
	pfnInternalFree: PFN_vkInternalFreeNotification,
}
impl VkAllocationCallbacks
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceQueueCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDeviceQueueCreateFlagBits,
	queueFamilyIndex: u32,
	queueCount: u32,
	pQueuePriorities: * const f32,
}
impl VkDeviceQueueCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDeviceCreateFlagBits,
	queueCreateInfoCount: u32,
	pQueueCreateInfos: * const VkDeviceQueueCreateInfo,
	enabledLayerCount: u32,
	ppEnabledLayerNames: * const * const c_char,
	enabledExtensionCount: u32,
	ppEnabledExtensionNames: * const * const c_char,
	pEnabledFeatures: * const VkPhysicalDeviceFeatures,
}
impl VkDeviceCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkInstanceCreateFlagBits,
	pApplicationInfo: * const VkApplicationInfo,
	enabledLayerCount: u32,
	ppEnabledLayerNames: * const * const c_char,
	enabledExtensionCount: u32,
	ppEnabledExtensionNames: * const * const c_char,
}
impl VkInstanceCreateInfo
{
    fn new() -> Self
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
	queueFlags: VkQueueFlagBits,
	queueCount: u32,
	timestampValidBits: u32,
	minImageTransferGranularity: VkExtent3D,
}
impl VkQueueFamilyProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties
{
	memoryTypeCount: u32,
	memoryTypes: VkMemoryType,
	memoryHeapCount: u32,
	memoryHeaps: VkMemoryHeap,
}
impl VkPhysicalDeviceMemoryProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryAllocateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	allocationSize: VkDeviceSize,
	memoryTypeIndex: u32,
}
impl VkMemoryAllocateInfo
{
    fn new() -> Self
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
	size: VkDeviceSize,
	alignment: VkDeviceSize,
	memoryTypeBits: u32,
}
impl VkMemoryRequirements
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageFormatProperties
{
	aspectMask: VkImageAspectFlagBits,
	imageGranularity: VkExtent3D,
	flags: VkSparseImageFormatFlagBits,
}
impl VkSparseImageFormatProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryRequirements
{
	formatProperties: VkSparseImageFormatProperties,
	imageMipTailFirstLod: u32,
	imageMipTailSize: VkDeviceSize,
	imageMipTailOffset: VkDeviceSize,
	imageMipTailStride: VkDeviceSize,
}
impl VkSparseImageMemoryRequirements
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryType
{
	propertyFlags: VkMemoryPropertyFlagBits,
	heapIndex: u32,
}
impl VkMemoryType
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryHeap
{
	size: VkDeviceSize,
	flags: VkMemoryHeapFlagBits,
}
impl VkMemoryHeap
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMappedMemoryRange
{
	sType: VkStructureType,
	pNext: * const c_void,
	memory: VkDeviceMemory,
	offset: VkDeviceSize,
	size: VkDeviceSize,
}
impl VkMappedMemoryRange
{
    fn new() -> Self
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
	linearTilingFeatures: VkFormatFeatureFlagBits,
	optimalTilingFeatures: VkFormatFeatureFlagBits,
	bufferFeatures: VkFormatFeatureFlagBits,
}
impl VkFormatProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatProperties
{
	maxExtent: VkExtent3D,
	maxMipLevels: u32,
	maxArrayLayers: u32,
	sampleCounts: VkSampleCountFlagBits,
	maxResourceSize: VkDeviceSize,
}
impl VkImageFormatProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorBufferInfo
{
	buffer: VkBuffer,
	offset: VkDeviceSize,
	range: VkDeviceSize,
}
impl VkDescriptorBufferInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorImageInfo
{
	sampler: VkSampler,
	imageView: VkImageView,
	imageLayout: VkImageLayout,
}
impl VkDescriptorImageInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWriteDescriptorSet
{
	sType: VkStructureType,
	pNext: * const c_void,
	dstSet: VkDescriptorSet,
	dstBinding: u32,
	dstArrayElement: u32,
	descriptorCount: u32,
	descriptorType: VkDescriptorType,
	pImageInfo: * const VkDescriptorImageInfo,
	pBufferInfo: * const VkDescriptorBufferInfo,
	pTexelBufferView: * const VkBufferView,
}
impl VkWriteDescriptorSet
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	srcSet: VkDescriptorSet,
	srcBinding: u32,
	srcArrayElement: u32,
	dstSet: VkDescriptorSet,
	dstBinding: u32,
	dstArrayElement: u32,
	descriptorCount: u32,
}
impl VkCopyDescriptorSet
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkBufferCreateFlagBits,
	size: VkDeviceSize,
	usage: VkBufferUsageFlagBits,
	sharingMode: VkSharingMode,
	queueFamilyIndexCount: u32,
	pQueueFamilyIndices: * const u32,
}
impl VkBufferCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkBufferViewCreateFlagBits,
	buffer: VkBuffer,
	format: VkFormat,
	offset: VkDeviceSize,
	range: VkDeviceSize,
}
impl VkBufferViewCreateInfo
{
    fn new() -> Self
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
	aspectMask: VkImageAspectFlagBits,
	mipLevel: u32,
	arrayLayer: u32,
}
impl VkImageSubresource
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceLayers
{
	aspectMask: VkImageAspectFlagBits,
	mipLevel: u32,
	baseArrayLayer: u32,
	layerCount: u32,
}
impl VkImageSubresourceLayers
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceRange
{
	aspectMask: VkImageAspectFlagBits,
	baseMipLevel: u32,
	levelCount: u32,
	baseArrayLayer: u32,
	layerCount: u32,
}
impl VkImageSubresourceRange
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryBarrier
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcAccessMask: VkAccessFlagBits,
	dstAccessMask: VkAccessFlagBits,
}
impl VkMemoryBarrier
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	srcAccessMask: VkAccessFlagBits,
	dstAccessMask: VkAccessFlagBits,
	srcQueueFamilyIndex: u32,
	dstQueueFamilyIndex: u32,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	size: VkDeviceSize,
}
impl VkBufferMemoryBarrier
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	srcAccessMask: VkAccessFlagBits,
	dstAccessMask: VkAccessFlagBits,
	oldLayout: VkImageLayout,
	newLayout: VkImageLayout,
	srcQueueFamilyIndex: u32,
	dstQueueFamilyIndex: u32,
	image: VkImage,
	subresourceRange: VkImageSubresourceRange,
}
impl VkImageMemoryBarrier
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkImageCreateFlagBits,
	imageType: VkImageType,
	format: VkFormat,
	extent: VkExtent3D,
	mipLevels: u32,
	arrayLayers: u32,
	samples: VkSampleCountFlagBits,
	tiling: VkImageTiling,
	usage: VkImageUsageFlagBits,
	sharingMode: VkSharingMode,
	queueFamilyIndexCount: u32,
	pQueueFamilyIndices: * const u32,
	initialLayout: VkImageLayout,
}
impl VkImageCreateInfo
{
    fn new() -> Self
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
	offset: VkDeviceSize,
	size: VkDeviceSize,
	rowPitch: VkDeviceSize,
	arrayPitch: VkDeviceSize,
	depthPitch: VkDeviceSize,
}
impl VkSubresourceLayout
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageViewCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkImageViewCreateFlagBits,
	image: VkImage,
	viewType: VkImageViewType,
	format: VkFormat,
	components: VkComponentMapping,
	subresourceRange: VkImageSubresourceRange,
}
impl VkImageViewCreateInfo
{
    fn new() -> Self
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
	srcOffset: VkDeviceSize,
	dstOffset: VkDeviceSize,
	size: VkDeviceSize,
}
impl VkBufferCopy
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseMemoryBind
{
	resourceOffset: VkDeviceSize,
	size: VkDeviceSize,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize,
	flags: VkSparseMemoryBindFlagBits,
}
impl VkSparseMemoryBind
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryBind
{
	subresource: VkImageSubresource,
	offset: VkOffset3D,
	extent: VkExtent3D,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize,
	flags: VkSparseMemoryBindFlagBits,
}
impl VkSparseImageMemoryBind
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseBufferMemoryBindInfo
{
	buffer: VkBuffer,
	bindCount: u32,
	pBinds: * const VkSparseMemoryBind,
}
impl VkSparseBufferMemoryBindInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageOpaqueMemoryBindInfo
{
	image: VkImage,
	bindCount: u32,
	pBinds: * const VkSparseMemoryBind,
}
impl VkSparseImageOpaqueMemoryBindInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryBindInfo
{
	image: VkImage,
	bindCount: u32,
	pBinds: * const VkSparseImageMemoryBind,
}
impl VkSparseImageMemoryBindInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindSparseInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	waitSemaphoreCount: u32,
	pWaitSemaphores: * const VkSemaphore,
	bufferBindCount: u32,
	pBufferBinds: * const VkSparseBufferMemoryBindInfo,
	imageOpaqueBindCount: u32,
	pImageOpaqueBinds: * const VkSparseImageOpaqueMemoryBindInfo,
	imageBindCount: u32,
	pImageBinds: * const VkSparseImageMemoryBindInfo,
	signalSemaphoreCount: u32,
	pSignalSemaphores: * const VkSemaphore,
}
impl VkBindSparseInfo
{
    fn new() -> Self
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
	srcSubresource: VkImageSubresourceLayers,
	srcOffset: VkOffset3D,
	dstSubresource: VkImageSubresourceLayers,
	dstOffset: VkOffset3D,
	extent: VkExtent3D,
}
impl VkImageCopy
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageBlit
{
	srcSubresource: VkImageSubresourceLayers,
	srcOffsets: VkOffset3D,
	dstSubresource: VkImageSubresourceLayers,
	dstOffsets: VkOffset3D,
}
impl VkImageBlit
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferImageCopy
{
	bufferOffset: VkDeviceSize,
	bufferRowLength: u32,
	bufferImageHeight: u32,
	imageSubresource: VkImageSubresourceLayers,
	imageOffset: VkOffset3D,
	imageExtent: VkExtent3D,
}
impl VkBufferImageCopy
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageResolve
{
	srcSubresource: VkImageSubresourceLayers,
	srcOffset: VkOffset3D,
	dstSubresource: VkImageSubresourceLayers,
	dstOffset: VkOffset3D,
	extent: VkExtent3D,
}
impl VkImageResolve
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkShaderModuleCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkShaderModuleCreateFlagBits,
	codeSize: size_t,
	pCode: * const u32,
}
impl VkShaderModuleCreateInfo
{
    fn new() -> Self
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
	binding: u32,
	descriptorType: VkDescriptorType,
	descriptorCount: u32,
	stageFlags: VkShaderStageFlagBits,
	pImmutableSamplers: * const VkSampler,
}
impl VkDescriptorSetLayoutBinding
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDescriptorSetLayoutCreateFlagBits,
	bindingCount: u32,
	pBindings: * const VkDescriptorSetLayoutBinding,
}
impl VkDescriptorSetLayoutCreateInfo
{
    fn new() -> Self
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
	type: VkDescriptorType,
	descriptorCount: u32,
}
impl VkDescriptorPoolSize
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDescriptorPoolCreateFlagBits,
	maxSets: u32,
	poolSizeCount: u32,
	pPoolSizes: * const VkDescriptorPoolSize,
}
impl VkDescriptorPoolCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	descriptorPool: VkDescriptorPool,
	descriptorSetCount: u32,
	pSetLayouts: * const VkDescriptorSetLayout,
}
impl VkDescriptorSetAllocateInfo
{
    fn new() -> Self
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
	constantID: u32,
	offset: u32,
	size: size_t,
}
impl VkSpecializationMapEntry
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSpecializationInfo
{
	mapEntryCount: u32,
	pMapEntries: * const VkSpecializationMapEntry,
	dataSize: size_t,
	pData: * const c_void,
}
impl VkSpecializationInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineShaderStageCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineShaderStageCreateFlagBits,
	stage: VkShaderStageFlagBits,
	module: VkShaderModule,
	pName: * const c_char,
	pSpecializationInfo: * const VkSpecializationInfo,
}
impl VkPipelineShaderStageCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineCreateFlagBits,
	stage: VkPipelineShaderStageCreateInfo,
	layout: VkPipelineLayout,
	basePipelineHandle: VkPipeline,
	basePipelineIndex: i32,
}
impl VkComputePipelineCreateInfo
{
    fn new() -> Self
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
	binding: u32,
	stride: u32,
	inputRate: VkVertexInputRate,
}
impl VkVertexInputBindingDescription
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputAttributeDescription
{
	location: u32,
	binding: u32,
	format: VkFormat,
	offset: u32,
}
impl VkVertexInputAttributeDescription
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineVertexInputStateCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineVertexInputStateCreateFlagBits,
	vertexBindingDescriptionCount: u32,
	pVertexBindingDescriptions: * const VkVertexInputBindingDescription,
	vertexAttributeDescriptionCount: u32,
	pVertexAttributeDescriptions: * const VkVertexInputAttributeDescription,
}
impl VkPipelineVertexInputStateCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineInputAssemblyStateCreateFlagBits,
	topology: VkPrimitiveTopology,
	primitiveRestartEnable: VkBool32,
}
impl VkPipelineInputAssemblyStateCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineTessellationStateCreateFlagBits,
	patchControlPoints: u32,
}
impl VkPipelineTessellationStateCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineViewportStateCreateFlagBits,
	viewportCount: u32,
	pViewports: * const VkViewport,
	scissorCount: u32,
	pScissors: * const VkRect2D,
}
impl VkPipelineViewportStateCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineRasterizationStateCreateFlagBits,
	depthClampEnable: VkBool32,
	rasterizerDiscardEnable: VkBool32,
	polygonMode: VkPolygonMode,
	cullMode: VkCullModeFlagBits,
	frontFace: VkFrontFace,
	depthBiasEnable: VkBool32,
	depthBiasConstantFactor: f32,
	depthBiasClamp: f32,
	depthBiasSlopeFactor: f32,
	lineWidth: f32,
}
impl VkPipelineRasterizationStateCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineMultisampleStateCreateFlagBits,
	rasterizationSamples: VkSampleCountFlagBits,
	sampleShadingEnable: VkBool32,
	minSampleShading: f32,
	pSampleMask: * const VkSampleMask,
	alphaToCoverageEnable: VkBool32,
	alphaToOneEnable: VkBool32,
}
impl VkPipelineMultisampleStateCreateInfo
{
    fn new() -> Self
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
	blendEnable: VkBool32,
	srcColorBlendFactor: VkBlendFactor,
	dstColorBlendFactor: VkBlendFactor,
	colorBlendOp: VkBlendOp,
	srcAlphaBlendFactor: VkBlendFactor,
	dstAlphaBlendFactor: VkBlendFactor,
	alphaBlendOp: VkBlendOp,
	colorWriteMask: VkColorComponentFlagBits,
}
impl VkPipelineColorBlendAttachmentState
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineColorBlendStateCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineColorBlendStateCreateFlagBits,
	logicOpEnable: VkBool32,
	logicOp: VkLogicOp,
	attachmentCount: u32,
	pAttachments: * const VkPipelineColorBlendAttachmentState,
	blendConstants: f32,
}
impl VkPipelineColorBlendStateCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineDynamicStateCreateFlagBits,
	dynamicStateCount: u32,
	pDynamicStates: * const VkDynamicState,
}
impl VkPipelineDynamicStateCreateInfo
{
    fn new() -> Self
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
	failOp: VkStencilOp,
	passOp: VkStencilOp,
	depthFailOp: VkStencilOp,
	compareOp: VkCompareOp,
	compareMask: u32,
	writeMask: u32,
	reference: u32,
}
impl VkStencilOpState
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineDepthStencilStateCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineDepthStencilStateCreateFlagBits,
	depthTestEnable: VkBool32,
	depthWriteEnable: VkBool32,
	depthCompareOp: VkCompareOp,
	depthBoundsTestEnable: VkBool32,
	stencilTestEnable: VkBool32,
	front: VkStencilOpState,
	back: VkStencilOpState,
	minDepthBounds: f32,
	maxDepthBounds: f32,
}
impl VkPipelineDepthStencilStateCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineCreateFlagBits,
	stageCount: u32,
	pStages: * const VkPipelineShaderStageCreateInfo,
	pVertexInputState: * const VkPipelineVertexInputStateCreateInfo,
	pInputAssemblyState: * const VkPipelineInputAssemblyStateCreateInfo,
	pTessellationState: * const VkPipelineTessellationStateCreateInfo,
	pViewportState: * const VkPipelineViewportStateCreateInfo,
	pRasterizationState: * const VkPipelineRasterizationStateCreateInfo,
	pMultisampleState: * const VkPipelineMultisampleStateCreateInfo,
	pDepthStencilState: * const VkPipelineDepthStencilStateCreateInfo,
	pColorBlendState: * const VkPipelineColorBlendStateCreateInfo,
	pDynamicState: * const VkPipelineDynamicStateCreateInfo,
	layout: VkPipelineLayout,
	renderPass: VkRenderPass,
	subpass: u32,
	basePipelineHandle: VkPipeline,
	basePipelineIndex: i32,
}
impl VkGraphicsPipelineCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineCacheCreateFlagBits,
	initialDataSize: size_t,
	pInitialData: * const c_void,
}
impl VkPipelineCacheCreateInfo
{
    fn new() -> Self
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
	headerSize: u32,
	headerVersion: VkPipelineCacheHeaderVersion,
	vendorID: u32,
	deviceID: u32,
	pipelineCacheUUID: u8,
}
impl VkPipelineCacheHeaderVersionOne
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPushConstantRange
{
	stageFlags: VkShaderStageFlagBits,
	offset: u32,
	size: u32,
}
impl VkPushConstantRange
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineLayoutCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineLayoutCreateFlagBits,
	setLayoutCount: u32,
	pSetLayouts: * const VkDescriptorSetLayout,
	pushConstantRangeCount: u32,
	pPushConstantRanges: * const VkPushConstantRange,
}
impl VkPipelineLayoutCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkSamplerCreateFlagBits,
	magFilter: VkFilter,
	minFilter: VkFilter,
	mipmapMode: VkSamplerMipmapMode,
	addressModeU: VkSamplerAddressMode,
	addressModeV: VkSamplerAddressMode,
	addressModeW: VkSamplerAddressMode,
	mipLodBias: f32,
	anisotropyEnable: VkBool32,
	maxAnisotropy: f32,
	compareEnable: VkBool32,
	compareOp: VkCompareOp,
	minLod: f32,
	maxLod: f32,
	borderColor: VkBorderColor,
	unnormalizedCoordinates: VkBool32,
}
impl VkSamplerCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkCommandPoolCreateFlagBits,
	queueFamilyIndex: u32,
}
impl VkCommandPoolCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	commandPool: VkCommandPool,
	level: VkCommandBufferLevel,
	commandBufferCount: u32,
}
impl VkCommandBufferAllocateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	renderPass: VkRenderPass,
	subpass: u32,
	framebuffer: VkFramebuffer,
	occlusionQueryEnable: VkBool32,
	queryFlags: VkQueryControlFlagBits,
	pipelineStatistics: VkQueryPipelineStatisticFlagBits,
}
impl VkCommandBufferInheritanceInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkCommandBufferUsageFlagBits,
	pInheritanceInfo: * const VkCommandBufferInheritanceInfo,
}
impl VkCommandBufferBeginInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	renderPass: VkRenderPass,
	framebuffer: VkFramebuffer,
	renderArea: VkRect2D,
	clearValueCount: u32,
	pClearValues: * const VkClearValue,
}
impl VkRenderPassBeginInfo
{
    fn new() -> Self
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
	depth: f32,
	stencil: u32,
}
impl VkClearDepthStencilValue
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearAttachment
{
	aspectMask: VkImageAspectFlagBits,
	colorAttachment: u32,
	clearValue: VkClearValue,
}
impl VkClearAttachment
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentDescription
{
	flags: VkAttachmentDescriptionFlagBits,
	format: VkFormat,
	samples: VkSampleCountFlagBits,
	loadOp: VkAttachmentLoadOp,
	storeOp: VkAttachmentStoreOp,
	stencilLoadOp: VkAttachmentLoadOp,
	stencilStoreOp: VkAttachmentStoreOp,
	initialLayout: VkImageLayout,
	finalLayout: VkImageLayout,
}
impl VkAttachmentDescription
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReference
{
	attachment: u32,
	layout: VkImageLayout,
}
impl VkAttachmentReference
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDescription
{
	flags: VkSubpassDescriptionFlagBits,
	pipelineBindPoint: VkPipelineBindPoint,
	inputAttachmentCount: u32,
	pInputAttachments: * const VkAttachmentReference,
	colorAttachmentCount: u32,
	pColorAttachments: * const VkAttachmentReference,
	pResolveAttachments: * const VkAttachmentReference,
	pDepthStencilAttachment: * const VkAttachmentReference,
	preserveAttachmentCount: u32,
	pPreserveAttachments: * const u32,
}
impl VkSubpassDescription
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDependency
{
	srcSubpass: u32,
	dstSubpass: u32,
	srcStageMask: VkPipelineStageFlagBits,
	dstStageMask: VkPipelineStageFlagBits,
	srcAccessMask: VkAccessFlagBits,
	dstAccessMask: VkAccessFlagBits,
	dependencyFlags: VkDependencyFlagBits,
}
impl VkSubpassDependency
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkRenderPassCreateFlagBits,
	attachmentCount: u32,
	pAttachments: * const VkAttachmentDescription,
	subpassCount: u32,
	pSubpasses: * const VkSubpassDescription,
	dependencyCount: u32,
	pDependencies: * const VkSubpassDependency,
}
impl VkRenderPassCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkEventCreateFlagBits,
}
impl VkEventCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkFenceCreateFlagBits,
}
impl VkFenceCreateInfo
{
    fn new() -> Self
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
	robustBufferAccess: VkBool32,
	fullDrawIndexUint32: VkBool32,
	imageCubeArray: VkBool32,
	independentBlend: VkBool32,
	geometryShader: VkBool32,
	tessellationShader: VkBool32,
	sampleRateShading: VkBool32,
	dualSrcBlend: VkBool32,
	logicOp: VkBool32,
	multiDrawIndirect: VkBool32,
	drawIndirectFirstInstance: VkBool32,
	depthClamp: VkBool32,
	depthBiasClamp: VkBool32,
	fillModeNonSolid: VkBool32,
	depthBounds: VkBool32,
	wideLines: VkBool32,
	largePoints: VkBool32,
	alphaToOne: VkBool32,
	multiViewport: VkBool32,
	samplerAnisotropy: VkBool32,
	textureCompressionETC2: VkBool32,
	textureCompressionASTC_LDR: VkBool32,
	textureCompressionBC: VkBool32,
	occlusionQueryPrecise: VkBool32,
	pipelineStatisticsQuery: VkBool32,
	vertexPipelineStoresAndAtomics: VkBool32,
	fragmentStoresAndAtomics: VkBool32,
	shaderTessellationAndGeometryPointSize: VkBool32,
	shaderImageGatherExtended: VkBool32,
	shaderStorageImageExtendedFormats: VkBool32,
	shaderStorageImageMultisample: VkBool32,
	shaderStorageImageReadWithoutFormat: VkBool32,
	shaderStorageImageWriteWithoutFormat: VkBool32,
	shaderUniformBufferArrayDynamicIndexing: VkBool32,
	shaderSampledImageArrayDynamicIndexing: VkBool32,
	shaderStorageBufferArrayDynamicIndexing: VkBool32,
	shaderStorageImageArrayDynamicIndexing: VkBool32,
	shaderClipDistance: VkBool32,
	shaderCullDistance: VkBool32,
	shaderFloat64: VkBool32,
	shaderInt64: VkBool32,
	shaderInt16: VkBool32,
	shaderResourceResidency: VkBool32,
	shaderResourceMinLod: VkBool32,
	sparseBinding: VkBool32,
	sparseResidencyBuffer: VkBool32,
	sparseResidencyImage2D: VkBool32,
	sparseResidencyImage3D: VkBool32,
	sparseResidency2Samples: VkBool32,
	sparseResidency4Samples: VkBool32,
	sparseResidency8Samples: VkBool32,
	sparseResidency16Samples: VkBool32,
	sparseResidencyAliased: VkBool32,
	variableMultisampleRate: VkBool32,
	inheritedQueries: VkBool32,
}
impl VkPhysicalDeviceFeatures
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSparseProperties
{
	residencyStandard2DBlockShape: VkBool32,
	residencyStandard2DMultisampleBlockShape: VkBool32,
	residencyStandard3DBlockShape: VkBool32,
	residencyAlignedMipSize: VkBool32,
	residencyNonResidentStrict: VkBool32,
}
impl VkPhysicalDeviceSparseProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceLimits
{
	maxImageDimension1D: u32,
	maxImageDimension2D: u32,
	maxImageDimension3D: u32,
	maxImageDimensionCube: u32,
	maxImageArrayLayers: u32,
	maxTexelBufferElements: u32,
	maxUniformBufferRange: u32,
	maxStorageBufferRange: u32,
	maxPushConstantsSize: u32,
	maxMemoryAllocationCount: u32,
	maxSamplerAllocationCount: u32,
	bufferImageGranularity: VkDeviceSize,
	sparseAddressSpaceSize: VkDeviceSize,
	maxBoundDescriptorSets: u32,
	maxPerStageDescriptorSamplers: u32,
	maxPerStageDescriptorUniformBuffers: u32,
	maxPerStageDescriptorStorageBuffers: u32,
	maxPerStageDescriptorSampledImages: u32,
	maxPerStageDescriptorStorageImages: u32,
	maxPerStageDescriptorInputAttachments: u32,
	maxPerStageResources: u32,
	maxDescriptorSetSamplers: u32,
	maxDescriptorSetUniformBuffers: u32,
	maxDescriptorSetUniformBuffersDynamic: u32,
	maxDescriptorSetStorageBuffers: u32,
	maxDescriptorSetStorageBuffersDynamic: u32,
	maxDescriptorSetSampledImages: u32,
	maxDescriptorSetStorageImages: u32,
	maxDescriptorSetInputAttachments: u32,
	maxVertexInputAttributes: u32,
	maxVertexInputBindings: u32,
	maxVertexInputAttributeOffset: u32,
	maxVertexInputBindingStride: u32,
	maxVertexOutputComponents: u32,
	maxTessellationGenerationLevel: u32,
	maxTessellationPatchSize: u32,
	maxTessellationControlPerVertexInputComponents: u32,
	maxTessellationControlPerVertexOutputComponents: u32,
	maxTessellationControlPerPatchOutputComponents: u32,
	maxTessellationControlTotalOutputComponents: u32,
	maxTessellationEvaluationInputComponents: u32,
	maxTessellationEvaluationOutputComponents: u32,
	maxGeometryShaderInvocations: u32,
	maxGeometryInputComponents: u32,
	maxGeometryOutputComponents: u32,
	maxGeometryOutputVertices: u32,
	maxGeometryTotalOutputComponents: u32,
	maxFragmentInputComponents: u32,
	maxFragmentOutputAttachments: u32,
	maxFragmentDualSrcAttachments: u32,
	maxFragmentCombinedOutputResources: u32,
	maxComputeSharedMemorySize: u32,
	maxComputeWorkGroupCount: u32,
	maxComputeWorkGroupInvocations: u32,
	maxComputeWorkGroupSize: u32,
	subPixelPrecisionBits: u32,
	subTexelPrecisionBits: u32,
	mipmapPrecisionBits: u32,
	maxDrawIndexedIndexValue: u32,
	maxDrawIndirectCount: u32,
	maxSamplerLodBias: f32,
	maxSamplerAnisotropy: f32,
	maxViewports: u32,
	maxViewportDimensions: u32,
	viewportBoundsRange: f32,
	viewportSubPixelBits: u32,
	minMemoryMapAlignment: size_t,
	minTexelBufferOffsetAlignment: VkDeviceSize,
	minUniformBufferOffsetAlignment: VkDeviceSize,
	minStorageBufferOffsetAlignment: VkDeviceSize,
	minTexelOffset: i32,
	maxTexelOffset: u32,
	minTexelGatherOffset: i32,
	maxTexelGatherOffset: u32,
	minInterpolationOffset: f32,
	maxInterpolationOffset: f32,
	subPixelInterpolationOffsetBits: u32,
	maxFramebufferWidth: u32,
	maxFramebufferHeight: u32,
	maxFramebufferLayers: u32,
	framebufferColorSampleCounts: VkSampleCountFlagBits,
	framebufferDepthSampleCounts: VkSampleCountFlagBits,
	framebufferStencilSampleCounts: VkSampleCountFlagBits,
	framebufferNoAttachmentsSampleCounts: VkSampleCountFlagBits,
	maxColorAttachments: u32,
	sampledImageColorSampleCounts: VkSampleCountFlagBits,
	sampledImageIntegerSampleCounts: VkSampleCountFlagBits,
	sampledImageDepthSampleCounts: VkSampleCountFlagBits,
	sampledImageStencilSampleCounts: VkSampleCountFlagBits,
	storageImageSampleCounts: VkSampleCountFlagBits,
	maxSampleMaskWords: u32,
	timestampComputeAndGraphics: VkBool32,
	timestampPeriod: f32,
	maxClipDistances: u32,
	maxCullDistances: u32,
	maxCombinedClipAndCullDistances: u32,
	discreteQueuePriorities: u32,
	pointSizeRange: f32,
	lineWidthRange: f32,
	pointSizeGranularity: f32,
	lineWidthGranularity: f32,
	strictLines: VkBool32,
	standardSampleLocations: VkBool32,
	optimalBufferCopyOffsetAlignment: VkDeviceSize,
	optimalBufferCopyRowPitchAlignment: VkDeviceSize,
	nonCoherentAtomSize: VkDeviceSize,
}
impl VkPhysicalDeviceLimits
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkSemaphoreCreateFlagBits,
}
impl VkSemaphoreCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkQueryPoolCreateFlagBits,
	queryType: VkQueryType,
	queryCount: u32,
	pipelineStatistics: VkQueryPipelineStatisticFlagBits,
}
impl VkQueryPoolCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkFramebufferCreateFlagBits,
	renderPass: VkRenderPass,
	attachmentCount: u32,
	pAttachments: * const VkImageView,
	width: u32,
	height: u32,
	layers: u32,
}
impl VkFramebufferCreateInfo
{
    fn new() -> Self
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
	vertexCount: u32,
	instanceCount: u32,
	firstVertex: u32,
	firstInstance: u32,
}
impl VkDrawIndirectCommand
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawIndexedIndirectCommand
{
	indexCount: u32,
	instanceCount: u32,
	firstIndex: u32,
	vertexOffset: i32,
	firstInstance: u32,
}
impl VkDrawIndexedIndirectCommand
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDispatchIndirectCommand
{
	x: u32,
	y: u32,
	z: u32,
}
impl VkDispatchIndirectCommand
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMultiDrawInfoEXT
{
	firstVertex: u32,
	vertexCount: u32,
}
impl VkMultiDrawInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMultiDrawIndexedInfoEXT
{
	firstIndex: u32,
	indexCount: u32,
	vertexOffset: i32,
}
impl VkMultiDrawIndexedInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubmitInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	waitSemaphoreCount: u32,
	pWaitSemaphores: * const VkSemaphore,
	pWaitDstStageMask: * const VkPipelineStageFlagBits,
	commandBufferCount: u32,
	pCommandBuffers: * const VkCommandBuffer,
	signalSemaphoreCount: u32,
	pSignalSemaphores: * const VkSemaphore,
}
impl VkSubmitInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayPropertiesKHR
{
	display: VkDisplayKHR,
	displayName: * const c_char,
	physicalDimensions: VkExtent2D,
	physicalResolution: VkExtent2D,
	supportedTransforms: VkSurfaceTransformFlagBitsKHR,
	planeReorderPossible: VkBool32,
	persistentContent: VkBool32,
}
impl VkDisplayPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayPlanePropertiesKHR
{
	currentDisplay: VkDisplayKHR,
	currentStackIndex: u32,
}
impl VkDisplayPlanePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayModeParametersKHR
{
	visibleRegion: VkExtent2D,
	refreshRate: u32,
}
impl VkDisplayModeParametersKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayModePropertiesKHR
{
	displayMode: VkDisplayModeKHR,
	parameters: VkDisplayModeParametersKHR,
}
impl VkDisplayModePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayModeCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDisplayModeCreateFlagBitsKHR,
	parameters: VkDisplayModeParametersKHR,
}
impl VkDisplayModeCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayPlaneCapabilitiesKHR
{
	supportedAlpha: VkDisplayPlaneAlphaFlagBitsKHR,
	minSrcPosition: VkOffset2D,
	maxSrcPosition: VkOffset2D,
	minSrcExtent: VkExtent2D,
	maxSrcExtent: VkExtent2D,
	minDstPosition: VkOffset2D,
	maxDstPosition: VkOffset2D,
	minDstExtent: VkExtent2D,
	maxDstExtent: VkExtent2D,
}
impl VkDisplayPlaneCapabilitiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplaySurfaceCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDisplaySurfaceCreateFlagBitsKHR,
	displayMode: VkDisplayModeKHR,
	planeIndex: u32,
	planeStackIndex: u32,
	transform: VkSurfaceTransformFlagBitsKHR,
	globalAlpha: f32,
	alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
	imageExtent: VkExtent2D,
}
impl VkDisplaySurfaceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceCapabilitiesKHR
{
	minImageCount: u32,
	maxImageCount: u32,
	currentExtent: VkExtent2D,
	minImageExtent: VkExtent2D,
	maxImageExtent: VkExtent2D,
	maxImageArrayLayers: u32,
	supportedTransforms: VkSurfaceTransformFlagBitsKHR,
	currentTransform: VkSurfaceTransformFlagBitsKHR,
	supportedCompositeAlpha: VkCompositeAlphaFlagBitsKHR,
	supportedUsageFlags: VkImageUsageFlagBits,
}
impl VkSurfaceCapabilitiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAndroidSurfaceCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkAndroidSurfaceCreateFlagBitsKHR,
	window: * const ANativeWindow,
}
impl VkAndroidSurfaceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkViSurfaceCreateInfoNN
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkViSurfaceCreateFlagBitsNN,
	window: * const c_void,
}
impl VkViSurfaceCreateInfoNN
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWaylandSurfaceCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkWaylandSurfaceCreateFlagBitsKHR,
	display: * const wl_display,
	surface: * const wl_surface,
}
impl VkWaylandSurfaceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWin32SurfaceCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkWin32SurfaceCreateFlagBitsKHR,
	hinstance: HINSTANCE,
	hwnd: HWND,
}
impl VkWin32SurfaceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkXlibSurfaceCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkXlibSurfaceCreateFlagBitsKHR,
	dpy: * const Display,
	window: Window,
}
impl VkXlibSurfaceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkXcbSurfaceCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkXcbSurfaceCreateFlagBitsKHR,
	connection: * const xcb_connection_t,
	window: xcb_window_t,
}
impl VkXcbSurfaceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDirectFBSurfaceCreateInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDirectFBSurfaceCreateFlagBitsEXT,
	dfb: * const IDirectFB,
	surface: * const IDirectFBSurface,
}
impl VkDirectFBSurfaceCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkImagePipeSurfaceCreateFlagBitsFUCHSIA,
	imagePipeHandle: zx_handle_t,
}
impl VkImagePipeSurfaceCreateInfoFUCHSIA
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkStreamDescriptorSurfaceCreateInfoGGP
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkStreamDescriptorSurfaceCreateFlagBitsGGP,
	streamDescriptor: GgpStreamDescriptor,
}
impl VkStreamDescriptorSurfaceCreateInfoGGP
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkScreenSurfaceCreateInfoQNX
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkScreenSurfaceCreateFlagBitsQNX,
	context: * const _screen_context,
	window: * const _screen_window,
}
impl VkScreenSurfaceCreateInfoQNX
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceFormatKHR
{
	format: VkFormat,
	colorSpace: VkColorSpaceKHR,
}
impl VkSurfaceFormatKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSwapchainCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkSwapchainCreateFlagBitsKHR,
	surface: VkSurfaceKHR,
	minImageCount: u32,
	imageFormat: VkFormat,
	imageColorSpace: VkColorSpaceKHR,
	imageExtent: VkExtent2D,
	imageArrayLayers: u32,
	imageUsage: VkImageUsageFlagBits,
	imageSharingMode: VkSharingMode,
	queueFamilyIndexCount: u32,
	pQueueFamilyIndices: * const u32,
	preTransform: VkSurfaceTransformFlagBitsKHR,
	compositeAlpha: VkCompositeAlphaFlagBitsKHR,
	presentMode: VkPresentModeKHR,
	clipped: VkBool32,
	oldSwapchain: VkSwapchainKHR,
}
impl VkSwapchainCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPresentInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	waitSemaphoreCount: u32,
	pWaitSemaphores: * const VkSemaphore,
	swapchainCount: u32,
	pSwapchains: * const VkSwapchainKHR,
	pImageIndices: * const u32,
	pResults: * const VkResult,
}
impl VkPresentInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugMarkerObjectNameInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	objectType: VkDebugReportObjectTypeEXT,
	object: u64,
	pObjectName: * const c_char,
}
impl VkDebugMarkerObjectNameInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugMarkerObjectTagInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	objectType: VkDebugReportObjectTypeEXT,
	object: u64,
	tagName: u64,
	tagSize: size_t,
	pTag: * const c_void,
}
impl VkDebugMarkerObjectTagInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugMarkerMarkerInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	pMarkerName: * const c_char,
	color: f32,
}
impl VkDebugMarkerMarkerInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalImageFormatPropertiesNV
{
	imageFormatProperties: VkImageFormatProperties,
	externalMemoryFeatures: VkExternalMemoryFeatureFlagBitsNV,
	exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagBitsNV,
	compatibleHandleTypes: VkExternalMemoryHandleTypeFlagBitsNV,
}
impl VkExternalImageFormatPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPrivateDataSlotCreateInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPrivateDataSlotCreateFlagBitsEXT,
}
impl VkPrivateDataSlotCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGraphicsShaderGroupCreateInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	stageCount: u32,
	pStages: * const VkPipelineShaderStageCreateInfo,
	pVertexInputState: * const VkPipelineVertexInputStateCreateInfo,
	pTessellationState: * const VkPipelineTessellationStateCreateInfo,
}
impl VkGraphicsShaderGroupCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindShaderGroupIndirectCommandNV
{
	groupIndex: u32,
}
impl VkBindShaderGroupIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindIndexBufferIndirectCommandNV
{
	bufferAddress: VkDeviceAddress,
	size: u32,
	indexType: VkIndexType,
}
impl VkBindIndexBufferIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindVertexBufferIndirectCommandNV
{
	bufferAddress: VkDeviceAddress,
	size: u32,
	stride: u32,
}
impl VkBindVertexBufferIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSetStateFlagsIndirectCommandNV
{
	data: u32,
}
impl VkSetStateFlagsIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkIndirectCommandsStreamNV
{
	buffer: VkBuffer,
	offset: VkDeviceSize,
}
impl VkIndirectCommandsStreamNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkIndirectCommandsLayoutTokenNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	tokenType: VkIndirectCommandsTokenTypeNV,
	stream: u32,
	offset: u32,
	vertexBindingUnit: u32,
	vertexDynamicStride: VkBool32,
	pushconstantPipelineLayout: VkPipelineLayout,
	pushconstantShaderStageFlags: VkShaderStageFlagBits,
	pushconstantOffset: u32,
	pushconstantSize: u32,
	indirectStateFlags: VkIndirectStateFlagBitsNV,
	indexTypeCount: u32,
	pIndexTypes: * const VkIndexType,
	pIndexTypeValues: * const u32,
}
impl VkIndirectCommandsLayoutTokenNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkIndirectCommandsLayoutCreateInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkIndirectCommandsLayoutUsageFlagBitsNV,
	pipelineBindPoint: VkPipelineBindPoint,
	tokenCount: u32,
	pTokens: * const VkIndirectCommandsLayoutTokenNV,
	streamCount: u32,
	pStreamStrides: * const u32,
}
impl VkIndirectCommandsLayoutCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGeneratedCommandsInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	pipelineBindPoint: VkPipelineBindPoint,
	pipeline: VkPipeline,
	indirectCommandsLayout: VkIndirectCommandsLayoutNV,
	streamCount: u32,
	pStreams: * const VkIndirectCommandsStreamNV,
	sequencesCount: u32,
	preprocessBuffer: VkBuffer,
	preprocessOffset: VkDeviceSize,
	preprocessSize: VkDeviceSize,
	sequencesCountBuffer: VkBuffer,
	sequencesCountOffset: VkDeviceSize,
	sequencesIndexBuffer: VkBuffer,
	sequencesIndexOffset: VkDeviceSize,
}
impl VkGeneratedCommandsInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	pipelineBindPoint: VkPipelineBindPoint,
	pipeline: VkPipeline,
	indirectCommandsLayout: VkIndirectCommandsLayoutNV,
	maxSequencesCount: u32,
}
impl VkGeneratedCommandsMemoryRequirementsInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFeatures2KHR
{
}
impl VkPhysicalDeviceFeatures2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProperties2
{
	sType: VkStructureType,
	pNext: * const c_void,
	properties: VkPhysicalDeviceProperties,
}
impl VkPhysicalDeviceProperties2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProperties2KHR
{
}
impl VkPhysicalDeviceProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFormatProperties2
{
	sType: VkStructureType,
	pNext: * const c_void,
	formatProperties: VkFormatProperties,
}
impl VkFormatProperties2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFormatProperties2KHR
{
}
impl VkFormatProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatProperties2
{
	sType: VkStructureType,
	pNext: * const c_void,
	imageFormatProperties: VkImageFormatProperties,
}
impl VkImageFormatProperties2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatProperties2KHR
{
}
impl VkImageFormatProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceImageFormatInfo2
{
	sType: VkStructureType,
	pNext: * const c_void,
	format: VkFormat,
	type: VkImageType,
	tiling: VkImageTiling,
	usage: VkImageUsageFlagBits,
	flags: VkImageCreateFlagBits,
}
impl VkPhysicalDeviceImageFormatInfo2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceImageFormatInfo2KHR
{
}
impl VkPhysicalDeviceImageFormatInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueueFamilyProperties2
{
	sType: VkStructureType,
	pNext: * const c_void,
	queueFamilyProperties: VkQueueFamilyProperties,
}
impl VkQueueFamilyProperties2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueueFamilyProperties2KHR
{
}
impl VkQueueFamilyProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties2
{
	sType: VkStructureType,
	pNext: * const c_void,
	memoryProperties: VkPhysicalDeviceMemoryProperties,
}
impl VkPhysicalDeviceMemoryProperties2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties2KHR
{
}
impl VkPhysicalDeviceMemoryProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageFormatProperties2
{
	sType: VkStructureType,
	pNext: * const c_void,
	properties: VkSparseImageFormatProperties,
}
impl VkSparseImageFormatProperties2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageFormatProperties2KHR
{
}
impl VkSparseImageFormatProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2
{
	sType: VkStructureType,
	pNext: * const c_void,
	format: VkFormat,
	type: VkImageType,
	samples: VkSampleCountFlagBits,
	usage: VkImageUsageFlagBits,
	tiling: VkImageTiling,
}
impl VkPhysicalDeviceSparseImageFormatInfo2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR
{
}
impl VkPhysicalDeviceSparseImageFormatInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkConformanceVersion
{
	major: u8,
	minor: u8,
	subminor: u8,
	patch: u8,
}
impl VkConformanceVersion
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkConformanceVersionKHR
{
}
impl VkConformanceVersionKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceDriverPropertiesKHR
{
}
impl VkPhysicalDeviceDriverPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPresentRegionKHR
{
	rectangleCount: u32,
	pRectangles: * const VkRectLayerKHR,
}
impl VkPresentRegionKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRectLayerKHR
{
	offset: VkOffset2D,
	extent: VkExtent2D,
	layer: u32,
}
impl VkRectLayerKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVariablePointersFeaturesKHR
{
}
impl VkPhysicalDeviceVariablePointersFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVariablePointerFeaturesKHR
{
}
impl VkPhysicalDeviceVariablePointerFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
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
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalMemoryProperties
{
	externalMemoryFeatures: VkExternalMemoryFeatureFlagBits,
	exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagBits,
	compatibleHandleTypes: VkExternalMemoryHandleTypeFlagBits,
}
impl VkExternalMemoryProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalMemoryPropertiesKHR
{
}
impl VkExternalMemoryPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR
{
}
impl VkPhysicalDeviceExternalImageFormatInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalImageFormatPropertiesKHR
{
}
impl VkExternalImageFormatPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalBufferInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkBufferCreateFlagBits,
	usage: VkBufferUsageFlagBits,
	handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl VkPhysicalDeviceExternalBufferInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalBufferInfoKHR
{
}
impl VkPhysicalDeviceExternalBufferInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalBufferProperties
{
	sType: VkStructureType,
	pNext: * const c_void,
	externalMemoryProperties: VkExternalMemoryProperties,
}
impl VkExternalBufferProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalBufferPropertiesKHR
{
}
impl VkExternalBufferPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceIDPropertiesKHR
{
}
impl VkPhysicalDeviceIDPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalMemoryImageCreateInfoKHR
{
}
impl VkExternalMemoryImageCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalMemoryBufferCreateInfoKHR
{
}
impl VkExternalMemoryBufferCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportMemoryAllocateInfoKHR
{
}
impl VkExportMemoryAllocateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryZirconHandlePropertiesFUCHSIA
{
	sType: VkStructureType,
	pNext: * const c_void,
	memoryTypeBits: u32,
}
impl VkMemoryZirconHandlePropertiesFUCHSIA
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryGetZirconHandleInfoFUCHSIA
{
	sType: VkStructureType,
	pNext: * const c_void,
	memory: VkDeviceMemory,
	handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl VkMemoryGetZirconHandleInfoFUCHSIA
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryWin32HandlePropertiesKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	memoryTypeBits: u32,
}
impl VkMemoryWin32HandlePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryGetWin32HandleInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	memory: VkDeviceMemory,
	handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl VkMemoryGetWin32HandleInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryFdPropertiesKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	memoryTypeBits: u32,
}
impl VkMemoryFdPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryGetFdInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	memory: VkDeviceMemory,
	handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl VkMemoryGetFdInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl VkPhysicalDeviceExternalSemaphoreInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR
{
}
impl VkPhysicalDeviceExternalSemaphoreInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalSemaphoreProperties
{
	sType: VkStructureType,
	pNext: * const c_void,
	exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagBits,
	compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagBits,
	externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagBits,
}
impl VkExternalSemaphoreProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalSemaphorePropertiesKHR
{
}
impl VkExternalSemaphorePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportSemaphoreCreateInfoKHR
{
}
impl VkExportSemaphoreCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImportSemaphoreWin32HandleInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	semaphore: VkSemaphore,
	flags: VkSemaphoreImportFlagBits,
	handleType: VkExternalSemaphoreHandleTypeFlagBits,
	handle: HANDLE,
	name: LPCWSTR,
}
impl VkImportSemaphoreWin32HandleInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreGetWin32HandleInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	semaphore: VkSemaphore,
	handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl VkSemaphoreGetWin32HandleInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImportSemaphoreFdInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	semaphore: VkSemaphore,
	flags: VkSemaphoreImportFlagBits,
	handleType: VkExternalSemaphoreHandleTypeFlagBits,
	fd: int,
}
impl VkImportSemaphoreFdInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreGetFdInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	semaphore: VkSemaphore,
	handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl VkSemaphoreGetFdInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA
{
	sType: VkStructureType,
	pNext: * const c_void,
	semaphore: VkSemaphore,
	flags: VkSemaphoreImportFlagBits,
	handleType: VkExternalSemaphoreHandleTypeFlagBits,
	zirconHandle: zx_handle_t,
}
impl VkImportSemaphoreZirconHandleInfoFUCHSIA
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA
{
	sType: VkStructureType,
	pNext: * const c_void,
	semaphore: VkSemaphore,
	handleType: VkExternalSemaphoreHandleTypeFlagBits,
}
impl VkSemaphoreGetZirconHandleInfoFUCHSIA
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalFenceInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	handleType: VkExternalFenceHandleTypeFlagBits,
}
impl VkPhysicalDeviceExternalFenceInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceExternalFenceInfoKHR
{
}
impl VkPhysicalDeviceExternalFenceInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalFenceProperties
{
	sType: VkStructureType,
	pNext: * const c_void,
	exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlagBits,
	compatibleHandleTypes: VkExternalFenceHandleTypeFlagBits,
	externalFenceFeatures: VkExternalFenceFeatureFlagBits,
}
impl VkExternalFenceProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExternalFencePropertiesKHR
{
}
impl VkExternalFencePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportFenceCreateInfoKHR
{
}
impl VkExportFenceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImportFenceWin32HandleInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	fence: VkFence,
	flags: VkFenceImportFlagBits,
	handleType: VkExternalFenceHandleTypeFlagBits,
	handle: HANDLE,
	name: LPCWSTR,
}
impl VkImportFenceWin32HandleInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFenceGetWin32HandleInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	fence: VkFence,
	handleType: VkExternalFenceHandleTypeFlagBits,
}
impl VkFenceGetWin32HandleInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImportFenceFdInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	fence: VkFence,
	flags: VkFenceImportFlagBits,
	handleType: VkExternalFenceHandleTypeFlagBits,
	fd: int,
}
impl VkImportFenceFdInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFenceGetFdInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	fence: VkFence,
	handleType: VkExternalFenceHandleTypeFlagBits,
}
impl VkFenceGetFdInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMultiviewFeaturesKHR
{
}
impl VkPhysicalDeviceMultiviewFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMultiviewPropertiesKHR
{
}
impl VkPhysicalDeviceMultiviewPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassMultiviewCreateInfoKHR
{
}
impl VkRenderPassMultiviewCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceCapabilities2EXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	minImageCount: u32,
	maxImageCount: u32,
	currentExtent: VkExtent2D,
	minImageExtent: VkExtent2D,
	maxImageExtent: VkExtent2D,
	maxImageArrayLayers: u32,
	supportedTransforms: VkSurfaceTransformFlagBitsKHR,
	currentTransform: VkSurfaceTransformFlagBitsKHR,
	supportedCompositeAlpha: VkCompositeAlphaFlagBitsKHR,
	supportedUsageFlags: VkImageUsageFlagBits,
	supportedSurfaceCounters: VkSurfaceCounterFlagBitsEXT,
}
impl VkSurfaceCapabilities2EXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayPowerInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	powerState: VkDisplayPowerStateEXT,
}
impl VkDisplayPowerInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceEventInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	deviceEvent: VkDeviceEventTypeEXT,
}
impl VkDeviceEventInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayEventInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	displayEvent: VkDisplayEventTypeEXT,
}
impl VkDisplayEventInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceGroupProperties
{
	sType: VkStructureType,
	pNext: * const c_void,
	physicalDeviceCount: u32,
	physicalDevices: VkPhysicalDevice,
	subsetAllocation: VkBool32,
}
impl VkPhysicalDeviceGroupProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceGroupPropertiesKHR
{
}
impl VkPhysicalDeviceGroupPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryAllocateFlagsInfoKHR
{
}
impl VkMemoryAllocateFlagsInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindBufferMemoryInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	buffer: VkBuffer,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize,
}
impl VkBindBufferMemoryInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindBufferMemoryInfoKHR
{
}
impl VkBindBufferMemoryInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindBufferMemoryDeviceGroupInfoKHR
{
}
impl VkBindBufferMemoryDeviceGroupInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindImageMemoryInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	image: VkImage,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize,
}
impl VkBindImageMemoryInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindImageMemoryInfoKHR
{
}
impl VkBindImageMemoryInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindImageMemoryDeviceGroupInfoKHR
{
}
impl VkBindImageMemoryDeviceGroupInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupRenderPassBeginInfoKHR
{
}
impl VkDeviceGroupRenderPassBeginInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupCommandBufferBeginInfoKHR
{
}
impl VkDeviceGroupCommandBufferBeginInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupSubmitInfoKHR
{
}
impl VkDeviceGroupSubmitInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupBindSparseInfoKHR
{
}
impl VkDeviceGroupBindSparseInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupPresentCapabilitiesKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	presentMask: u32,
	modes: VkDeviceGroupPresentModeFlagBitsKHR,
}
impl VkDeviceGroupPresentCapabilitiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAcquireNextImageInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	swapchain: VkSwapchainKHR,
	timeout: u64,
	semaphore: VkSemaphore,
	fence: VkFence,
	deviceMask: u32,
}
impl VkAcquireNextImageInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceGroupDeviceCreateInfoKHR
{
}
impl VkDeviceGroupDeviceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorUpdateTemplateEntry
{
	dstBinding: u32,
	dstArrayElement: u32,
	descriptorCount: u32,
	descriptorType: VkDescriptorType,
	offset: size_t,
	stride: size_t,
}
impl VkDescriptorUpdateTemplateEntry
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorUpdateTemplateEntryKHR
{
}
impl VkDescriptorUpdateTemplateEntryKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorUpdateTemplateCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDescriptorUpdateTemplateCreateFlagBits,
	descriptorUpdateEntryCount: u32,
	pDescriptorUpdateEntries: * const VkDescriptorUpdateTemplateEntry,
	templateType: VkDescriptorUpdateTemplateType,
	descriptorSetLayout: VkDescriptorSetLayout,
	pipelineBindPoint: VkPipelineBindPoint,
	pipelineLayout: VkPipelineLayout,
	set: u32,
}
impl VkDescriptorUpdateTemplateCreateInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR
{
}
impl VkDescriptorUpdateTemplateCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkXYColorEXT
{
	x: f32,
	y: f32,
}
impl VkXYColorEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkHdrMetadataEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	displayPrimaryRed: VkXYColorEXT,
	displayPrimaryGreen: VkXYColorEXT,
	displayPrimaryBlue: VkXYColorEXT,
	whitePoint: VkXYColorEXT,
	maxLuminance: f32,
	minLuminance: f32,
	maxContentLightLevel: f32,
	maxFrameAverageLightLevel: f32,
}
impl VkHdrMetadataEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_HDR_METADATA_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRefreshCycleDurationGOOGLE
{
	refreshDuration: u64,
}
impl VkRefreshCycleDurationGOOGLE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPastPresentationTimingGOOGLE
{
	presentID: u32,
	desiredPresentTime: u64,
	actualPresentTime: u64,
	earliestPresentTime: u64,
	presentMargin: u64,
}
impl VkPastPresentationTimingGOOGLE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPresentTimeGOOGLE
{
	presentID: u32,
	desiredPresentTime: u64,
}
impl VkPresentTimeGOOGLE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkIOSSurfaceCreateInfoMVK
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkIOSSurfaceCreateFlagBitsMVK,
	pView: * const c_void,
}
impl VkIOSSurfaceCreateInfoMVK
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMacOSSurfaceCreateInfoMVK
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkMacOSSurfaceCreateFlagBitsMVK,
	pView: * const c_void,
}
impl VkMacOSSurfaceCreateInfoMVK
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMetalSurfaceCreateInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkMetalSurfaceCreateFlagBitsEXT,
	pLayer: * const CAMetalLayer,
}
impl VkMetalSurfaceCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkViewportWScalingNV
{
	xcoeff: f32,
	ycoeff: f32,
}
impl VkViewportWScalingNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkViewportSwizzleNV
{
	x: VkViewportCoordinateSwizzleNV,
	y: VkViewportCoordinateSwizzleNV,
	z: VkViewportCoordinateSwizzleNV,
	w: VkViewportCoordinateSwizzleNV,
}
impl VkViewportSwizzleNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInputAttachmentAspectReference
{
	subpass: u32,
	inputAttachmentIndex: u32,
	aspectMask: VkImageAspectFlagBits,
}
impl VkInputAttachmentAspectReference
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInputAttachmentAspectReferenceKHR
{
}
impl VkInputAttachmentAspectReferenceKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR
{
}
impl VkRenderPassInputAttachmentAspectCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	surface: VkSurfaceKHR,
}
impl VkPhysicalDeviceSurfaceInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceCapabilities2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	surfaceCapabilities: VkSurfaceCapabilitiesKHR,
}
impl VkSurfaceCapabilities2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceFormat2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	surfaceFormat: VkSurfaceFormatKHR,
}
impl VkSurfaceFormat2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayProperties2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	displayProperties: VkDisplayPropertiesKHR,
}
impl VkDisplayProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayPlaneProperties2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	displayPlaneProperties: VkDisplayPlanePropertiesKHR,
}
impl VkDisplayPlaneProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayModeProperties2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	displayModeProperties: VkDisplayModePropertiesKHR,
}
impl VkDisplayModeProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayPlaneInfo2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	mode: VkDisplayModeKHR,
	planeIndex: u32,
}
impl VkDisplayPlaneInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDisplayPlaneCapabilities2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	capabilities: VkDisplayPlaneCapabilitiesKHR,
}
impl VkDisplayPlaneCapabilities2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDevice16BitStorageFeaturesKHR
{
}
impl VkPhysicalDevice16BitStorageFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR
{
}
impl VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferMemoryRequirementsInfo2
{
	sType: VkStructureType,
	pNext: * const c_void,
	buffer: VkBuffer,
}
impl VkBufferMemoryRequirementsInfo2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferMemoryRequirementsInfo2KHR
{
}
impl VkBufferMemoryRequirementsInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageMemoryRequirementsInfo2
{
	sType: VkStructureType,
	pNext: * const c_void,
	image: VkImage,
}
impl VkImageMemoryRequirementsInfo2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageMemoryRequirementsInfo2KHR
{
}
impl VkImageMemoryRequirementsInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSparseMemoryRequirementsInfo2
{
	sType: VkStructureType,
	pNext: * const c_void,
	image: VkImage,
}
impl VkImageSparseMemoryRequirementsInfo2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSparseMemoryRequirementsInfo2KHR
{
}
impl VkImageSparseMemoryRequirementsInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryRequirements2
{
	sType: VkStructureType,
	pNext: * const c_void,
	memoryRequirements: VkMemoryRequirements,
}
impl VkMemoryRequirements2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryRequirements2KHR
{
}
impl VkMemoryRequirements2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryRequirements2
{
	sType: VkStructureType,
	pNext: * const c_void,
	memoryRequirements: VkSparseImageMemoryRequirements,
}
impl VkSparseImageMemoryRequirements2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryRequirements2KHR
{
}
impl VkSparseImageMemoryRequirements2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDevicePointClippingPropertiesKHR
{
}
impl VkPhysicalDevicePointClippingPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryDedicatedRequirementsKHR
{
}
impl VkMemoryDedicatedRequirementsKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryDedicatedAllocateInfoKHR
{
}
impl VkMemoryDedicatedAllocateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageViewUsageCreateInfoKHR
{
}
impl VkImageViewUsageCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR
{
}
impl VkPipelineTessellationDomainOriginStateCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerYcbcrConversionInfoKHR
{
}
impl VkSamplerYcbcrConversionInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerYcbcrConversionCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	format: VkFormat,
	ycbcrModel: VkSamplerYcbcrModelConversion,
	ycbcrRange: VkSamplerYcbcrRange,
	components: VkComponentMapping,
	xChromaOffset: VkChromaLocation,
	yChromaOffset: VkChromaLocation,
	chromaFilter: VkFilter,
	forceExplicitReconstruction: VkBool32,
}
impl VkSamplerYcbcrConversionCreateInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerYcbcrConversionCreateInfoKHR
{
}
impl VkSamplerYcbcrConversionCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindImagePlaneMemoryInfoKHR
{
}
impl VkBindImagePlaneMemoryInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImagePlaneMemoryRequirementsInfoKHR
{
}
impl VkImagePlaneMemoryRequirementsInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR
{
}
impl VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerYcbcrConversionImageFormatPropertiesKHR
{
}
impl VkSamplerYcbcrConversionImageFormatPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkConditionalRenderingBeginInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	flags: VkConditionalRenderingFlagBitsEXT,
}
impl VkConditionalRenderingBeginInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceQueueInfo2
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDeviceQueueCreateFlagBits,
	queueFamilyIndex: u32,
	queueIndex: u32,
}
impl VkDeviceQueueInfo2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT
{
}
impl VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSampleLocationEXT
{
	x: f32,
	y: f32,
}
impl VkSampleLocationEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentSampleLocationsEXT
{
	attachmentIndex: u32,
	sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
impl VkAttachmentSampleLocationsEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassSampleLocationsEXT
{
	subpassIndex: u32,
	sampleLocationsInfo: VkSampleLocationsInfoEXT,
}
impl VkSubpassSampleLocationsEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMultisamplePropertiesEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	maxSampleLocationGridSize: VkExtent2D,
}
impl VkMultisamplePropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerReductionModeCreateInfoEXT
{
}
impl VkSamplerReductionModeCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatListCreateInfoKHR
{
}
impl VkImageFormatListCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkValidationCacheCreateInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkValidationCacheCreateFlagBitsEXT,
	initialDataSize: size_t,
	pInitialData: * const c_void,
}
impl VkValidationCacheCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMaintenance3PropertiesKHR
{
}
impl VkPhysicalDeviceMaintenance3PropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutSupport
{
	sType: VkStructureType,
	pNext: * const c_void,
	supported: VkBool32,
}
impl VkDescriptorSetLayoutSupport
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutSupportKHR
{
}
impl VkDescriptorSetLayoutSupportKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
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
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceShaderFloat16Int8FeaturesKHR
{
}
impl VkPhysicalDeviceShaderFloat16Int8FeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFloat16Int8FeaturesKHR
{
}
impl VkPhysicalDeviceFloat16Int8FeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFloatControlsPropertiesKHR
{
}
impl VkPhysicalDeviceFloatControlsPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceHostQueryResetFeaturesEXT
{
}
impl VkPhysicalDeviceHostQueryResetFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkNativeBufferUsage2ANDROID
{
	consumer: u64,
	producer: u64,
}
impl VkNativeBufferUsage2ANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkNativeBufferANDROID
{
	sType: VkStructureType,
	pNext: * const c_void,
	handle: * const c_void,
	stride: int,
	format: int,
	usage: int,
	usage2: VkNativeBufferUsage2ANDROID,
}
impl VkNativeBufferANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_NATIVE_BUFFER_ANDROID;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSwapchainImageCreateInfoANDROID
{
	sType: VkStructureType,
	pNext: * const c_void,
	usage: VkSwapchainImageUsageFlagBitsANDROID,
}
impl VkSwapchainImageCreateInfoANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDevicePresentationPropertiesANDROID
{
	sType: VkStructureType,
	pNext: * const c_void,
	sharedImage: VkBool32,
}
impl VkPhysicalDevicePresentationPropertiesANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkShaderResourceUsageAMD
{
	numUsedVgprs: u32,
	numUsedSgprs: u32,
	ldsSizePerLocalWorkGroup: u32,
	ldsUsageSizeInBytes: size_t,
	scratchMemUsageInBytes: size_t,
}
impl VkShaderResourceUsageAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkShaderStatisticsInfoAMD
{
	shaderStageMask: VkShaderStageFlagBits,
	resourceUsage: VkShaderResourceUsageAMD,
	numPhysicalVgprs: u32,
	numPhysicalSgprs: u32,
	numAvailableVgprs: u32,
	numAvailableSgprs: u32,
	computeWorkGroupSize: u32,
}
impl VkShaderStatisticsInfoAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugUtilsObjectNameInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	objectType: VkObjectType,
	objectHandle: u64,
	pObjectName: * const c_char,
}
impl VkDebugUtilsObjectNameInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugUtilsObjectTagInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	objectType: VkObjectType,
	objectHandle: u64,
	tagName: u64,
	tagSize: size_t,
	pTag: * const c_void,
}
impl VkDebugUtilsObjectTagInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugUtilsLabelEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	pLabelName: * const c_char,
	color: f32,
}
impl VkDebugUtilsLabelEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugUtilsMessengerCallbackDataEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDebugUtilsMessengerCallbackDataFlagBitsEXT,
	pMessageIdName: * const c_char,
	messageIdNumber: i32,
	pMessage: * const c_char,
	queueLabelCount: u32,
	pQueueLabels: * const VkDebugUtilsLabelEXT,
	cmdBufLabelCount: u32,
	pCmdBufLabels: * const VkDebugUtilsLabelEXT,
	objectCount: u32,
	pObjects: * const VkDebugUtilsObjectNameInfoEXT,
}
impl VkDebugUtilsMessengerCallbackDataEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceMemoryReportCallbackDataEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkDeviceMemoryReportFlagBitsEXT,
	type: VkDeviceMemoryReportEventTypeEXT,
	memoryObjectId: u64,
	size: VkDeviceSize,
	objectType: VkObjectType,
	objectHandle: u64,
	heapIndex: u32,
}
impl VkDeviceMemoryReportCallbackDataEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryHostPointerPropertiesEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	memoryTypeBits: u32,
}
impl VkMemoryHostPointerPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCalibratedTimestampInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	timeDomain: VkTimeDomainEXT,
}
impl VkCalibratedTimestampInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceDescriptorIndexingFeaturesEXT
{
}
impl VkPhysicalDeviceDescriptorIndexingFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceDescriptorIndexingPropertiesEXT
{
}
impl VkPhysicalDeviceDescriptorIndexingPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfoEXT
{
}
impl VkDescriptorSetLayoutBindingFlagsCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfoEXT
{
}
impl VkDescriptorSetVariableDescriptorCountAllocateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupportEXT
{
}
impl VkDescriptorSetVariableDescriptorCountLayoutSupportEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentDescription2
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkAttachmentDescriptionFlagBits,
	format: VkFormat,
	samples: VkSampleCountFlagBits,
	loadOp: VkAttachmentLoadOp,
	storeOp: VkAttachmentStoreOp,
	stencilLoadOp: VkAttachmentLoadOp,
	stencilStoreOp: VkAttachmentStoreOp,
	initialLayout: VkImageLayout,
	finalLayout: VkImageLayout,
}
impl VkAttachmentDescription2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentDescription2KHR
{
}
impl VkAttachmentDescription2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReference2
{
	sType: VkStructureType,
	pNext: * const c_void,
	attachment: u32,
	layout: VkImageLayout,
	aspectMask: VkImageAspectFlagBits,
}
impl VkAttachmentReference2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReference2KHR
{
}
impl VkAttachmentReference2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDescription2
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkSubpassDescriptionFlagBits,
	pipelineBindPoint: VkPipelineBindPoint,
	viewMask: u32,
	inputAttachmentCount: u32,
	pInputAttachments: * const VkAttachmentReference2,
	colorAttachmentCount: u32,
	pColorAttachments: * const VkAttachmentReference2,
	pResolveAttachments: * const VkAttachmentReference2,
	pDepthStencilAttachment: * const VkAttachmentReference2,
	preserveAttachmentCount: u32,
	pPreserveAttachments: * const u32,
}
impl VkSubpassDescription2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDescription2KHR
{
}
impl VkSubpassDescription2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDependency2
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcSubpass: u32,
	dstSubpass: u32,
	srcStageMask: VkPipelineStageFlagBits,
	dstStageMask: VkPipelineStageFlagBits,
	srcAccessMask: VkAccessFlagBits,
	dstAccessMask: VkAccessFlagBits,
	dependencyFlags: VkDependencyFlagBits,
	viewOffset: i32,
}
impl VkSubpassDependency2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDependency2KHR
{
}
impl VkSubpassDependency2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassCreateInfo2
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkRenderPassCreateFlagBits,
	attachmentCount: u32,
	pAttachments: * const VkAttachmentDescription2,
	subpassCount: u32,
	pSubpasses: * const VkSubpassDescription2,
	dependencyCount: u32,
	pDependencies: * const VkSubpassDependency2,
	correlatedViewMaskCount: u32,
	pCorrelatedViewMasks: * const u32,
}
impl VkRenderPassCreateInfo2
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassCreateInfo2KHR
{
}
impl VkRenderPassCreateInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassBeginInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	contents: VkSubpassContents,
}
impl VkSubpassBeginInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassBeginInfoKHR
{
}
impl VkSubpassBeginInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassEndInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
}
impl VkSubpassEndInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_END_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassEndInfoKHR
{
}
impl VkSubpassEndInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeaturesKHR
{
}
impl VkPhysicalDeviceTimelineSemaphoreFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceTimelineSemaphorePropertiesKHR
{
}
impl VkPhysicalDeviceTimelineSemaphorePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreTypeCreateInfoKHR
{
}
impl VkSemaphoreTypeCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkTimelineSemaphoreSubmitInfoKHR
{
}
impl VkTimelineSemaphoreSubmitInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreWaitInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkSemaphoreWaitFlagBits,
	semaphoreCount: u32,
	pSemaphores: * const VkSemaphore,
	pValues: * const u64,
}
impl VkSemaphoreWaitInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreWaitInfoKHR
{
}
impl VkSemaphoreWaitInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreSignalInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	semaphore: VkSemaphore,
	value: u64,
}
impl VkSemaphoreSignalInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreSignalInfoKHR
{
}
impl VkSemaphoreSignalInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputBindingDivisorDescriptionEXT
{
	binding: u32,
	divisor: u32,
}
impl VkVertexInputBindingDivisorDescriptionEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAndroidHardwareBufferPropertiesANDROID
{
	sType: VkStructureType,
	pNext: * const c_void,
	allocationSize: VkDeviceSize,
	memoryTypeBits: u32,
}
impl VkAndroidHardwareBufferPropertiesANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID
{
	sType: VkStructureType,
	pNext: * const c_void,
	memory: VkDeviceMemory,
}
impl VkMemoryGetAndroidHardwareBufferInfoANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDevice8BitStorageFeaturesKHR
{
}
impl VkPhysicalDevice8BitStorageFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeaturesKHR
{
}
impl VkPhysicalDeviceVulkanMemoryModelFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceShaderAtomicInt64FeaturesKHR
{
}
impl VkPhysicalDeviceShaderAtomicInt64FeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCheckpointDataNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	stage: VkPipelineStageFlagBits,
	pCheckpointMarker: * const c_void,
}
impl VkCheckpointDataNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceDepthStencilResolvePropertiesKHR
{
}
impl VkPhysicalDeviceDepthStencilResolvePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDescriptionDepthStencilResolveKHR
{
}
impl VkSubpassDescriptionDepthStencilResolveKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkShadingRatePaletteNV
{
	shadingRatePaletteEntryCount: u32,
	pShadingRatePaletteEntries: * const VkShadingRatePaletteEntryNV,
}
impl VkShadingRatePaletteNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCoarseSampleLocationNV
{
	pixelX: u32,
	pixelY: u32,
	sample: u32,
}
impl VkCoarseSampleLocationNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCoarseSampleOrderCustomNV
{
	shadingRate: VkShadingRatePaletteEntryNV,
	sampleCount: u32,
	sampleLocationCount: u32,
	pSampleLocations: * const VkCoarseSampleLocationNV,
}
impl VkCoarseSampleOrderCustomNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawMeshTasksIndirectCommandNV
{
	taskCount: u32,
	firstTask: u32,
}
impl VkDrawMeshTasksIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRayTracingShaderGroupCreateInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	type: VkRayTracingShaderGroupTypeKHR,
	generalShader: u32,
	closestHitShader: u32,
	anyHitShader: u32,
	intersectionShader: u32,
}
impl VkRayTracingShaderGroupCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRayTracingShaderGroupCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	type: VkRayTracingShaderGroupTypeKHR,
	generalShader: u32,
	closestHitShader: u32,
	anyHitShader: u32,
	intersectionShader: u32,
	pShaderGroupCaptureReplayHandle: * const c_void,
}
impl VkRayTracingShaderGroupCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRayTracingPipelineCreateInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineCreateFlagBits,
	stageCount: u32,
	pStages: * const VkPipelineShaderStageCreateInfo,
	groupCount: u32,
	pGroups: * const VkRayTracingShaderGroupCreateInfoNV,
	maxRecursionDepth: u32,
	layout: VkPipelineLayout,
	basePipelineHandle: VkPipeline,
	basePipelineIndex: i32,
}
impl VkRayTracingPipelineCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRayTracingPipelineCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPipelineCreateFlagBits,
	stageCount: u32,
	pStages: * const VkPipelineShaderStageCreateInfo,
	groupCount: u32,
	pGroups: * const VkRayTracingShaderGroupCreateInfoKHR,
	maxPipelineRayRecursionDepth: u32,
	pLibraryInfo: * const VkPipelineLibraryCreateInfoKHR,
	pLibraryInterface: * const VkRayTracingPipelineInterfaceCreateInfoKHR,
	pDynamicState: * const VkPipelineDynamicStateCreateInfo,
	layout: VkPipelineLayout,
	basePipelineHandle: VkPipeline,
	basePipelineIndex: i32,
}
impl VkRayTracingPipelineCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGeometryTrianglesNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	vertexData: VkBuffer,
	vertexOffset: VkDeviceSize,
	vertexCount: u32,
	vertexStride: VkDeviceSize,
	vertexFormat: VkFormat,
	indexData: VkBuffer,
	indexOffset: VkDeviceSize,
	indexCount: u32,
	indexType: VkIndexType,
	transformData: VkBuffer,
	transformOffset: VkDeviceSize,
}
impl VkGeometryTrianglesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGeometryAABBNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	aabbData: VkBuffer,
	numAABBs: u32,
	stride: u32,
	offset: VkDeviceSize,
}
impl VkGeometryAABBNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGeometryDataNV
{
	triangles: VkGeometryTrianglesNV,
	aabbs: VkGeometryAABBNV,
}
impl VkGeometryDataNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGeometryNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	geometryType: VkGeometryTypeKHR,
	geometry: VkGeometryDataNV,
	flags: VkGeometryFlagBitsKHR,
}
impl VkGeometryNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_GEOMETRY_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	type: VkAccelerationStructureTypeNV,
	flags: VkBuildAccelerationStructureFlagBitsNV,
	instanceCount: u32,
	geometryCount: u32,
	pGeometries: * const VkGeometryNV,
}
impl VkAccelerationStructureInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureCreateInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	compactedSize: VkDeviceSize,
	info: VkAccelerationStructureInfoNV,
}
impl VkAccelerationStructureCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindAccelerationStructureMemoryInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	accelerationStructure: VkAccelerationStructureNV,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize,
	deviceIndexCount: u32,
	pDeviceIndices: * const u32,
}
impl VkBindAccelerationStructureMemoryInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	type: VkAccelerationStructureMemoryRequirementsTypeNV,
	accelerationStructure: VkAccelerationStructureNV,
}
impl VkAccelerationStructureMemoryRequirementsInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkStridedDeviceAddressRegionKHR
{
	deviceAddress: VkDeviceAddress,
	stride: VkDeviceSize,
	size: VkDeviceSize,
}
impl VkStridedDeviceAddressRegionKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkTraceRaysIndirectCommandKHR
{
	width: u32,
	height: u32,
	depth: u32,
}
impl VkTraceRaysIndirectCommandKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrmFormatModifierPropertiesEXT
{
	drmFormatModifier: u64,
	drmFormatModifierPlaneCount: u32,
	drmFormatModifierTilingFeatures: VkFormatFeatureFlagBits,
}
impl VkDrmFormatModifierPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageDrmFormatModifierPropertiesEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	drmFormatModifier: u64,
}
impl VkImageDrmFormatModifierPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageStencilUsageCreateInfoEXT
{
}
impl VkImageStencilUsageCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeaturesEXT
{
}
impl VkPhysicalDeviceScalarBlockLayoutFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR
{
}
impl VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesKHR
{
}
impl VkPhysicalDeviceBufferDeviceAddressFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceBufferAddressFeaturesEXT
{
}
impl VkPhysicalDeviceBufferAddressFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferDeviceAddressInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	buffer: VkBuffer,
}
impl VkBufferDeviceAddressInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferDeviceAddressInfoKHR
{
}
impl VkBufferDeviceAddressInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferDeviceAddressInfoEXT
{
}
impl VkBufferDeviceAddressInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferOpaqueCaptureAddressCreateInfoKHR
{
}
impl VkBufferOpaqueCaptureAddressCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceImagelessFramebufferFeaturesKHR
{
}
impl VkPhysicalDeviceImagelessFramebufferFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferAttachmentsCreateInfoKHR
{
}
impl VkFramebufferAttachmentsCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferAttachmentImageInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkImageCreateFlagBits,
	usage: VkImageUsageFlagBits,
	width: u32,
	height: u32,
	layerCount: u32,
	viewFormatCount: u32,
	pViewFormats: * const VkFormat,
}
impl VkFramebufferAttachmentImageInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferAttachmentImageInfoKHR
{
}
impl VkFramebufferAttachmentImageInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassAttachmentBeginInfoKHR
{
}
impl VkRenderPassAttachmentBeginInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCooperativeMatrixPropertiesNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	MSize: u32,
	NSize: u32,
	KSize: u32,
	AType: VkComponentTypeNV,
	BType: VkComponentTypeNV,
	CType: VkComponentTypeNV,
	DType: VkComponentTypeNV,
	scope: VkScopeNV,
}
impl VkCooperativeMatrixPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageViewHandleInfoNVX
{
	sType: VkStructureType,
	pNext: * const c_void,
	imageView: VkImageView,
	descriptorType: VkDescriptorType,
	sampler: VkSampler,
}
impl VkImageViewHandleInfoNVX
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageViewAddressPropertiesNVX
{
	sType: VkStructureType,
	pNext: * const c_void,
	deviceAddress: VkDeviceAddress,
	size: VkDeviceSize,
}
impl VkImageViewAddressPropertiesNVX
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineCreationFeedbackEXT
{
	flags: VkPipelineCreationFeedbackFlagBitsEXT,
	duration: u64,
}
impl VkPipelineCreationFeedbackEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPerformanceCounterKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	unit: VkPerformanceCounterUnitKHR,
	scope: VkPerformanceCounterScopeKHR,
	storage: VkPerformanceCounterStorageKHR,
	uuid: u8,
}
impl VkPerformanceCounterKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPerformanceCounterDescriptionKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkPerformanceCounterDescriptionFlagBitsKHR,
	name: c_char,
	category: c_char,
	description: c_char,
}
impl VkPerformanceCounterDescriptionKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAcquireProfilingLockInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkAcquireProfilingLockFlagBitsKHR,
	timeout: u64,
}
impl VkAcquireProfilingLockInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkHeadlessSurfaceCreateInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkHeadlessSurfaceCreateFlagBitsEXT,
}
impl VkHeadlessSurfaceCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferMixedSamplesCombinationNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	coverageReductionMode: VkCoverageReductionModeNV,
	rasterizationSamples: VkSampleCountFlagBits,
	depthStencilSamples: VkSampleCountFlagBits,
	colorSamples: VkSampleCountFlagBits,
}
impl VkFramebufferMixedSamplesCombinationNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPerformanceValueINTEL
{
	type: VkPerformanceValueTypeINTEL,
	data: VkPerformanceValueDataINTEL,
}
impl VkPerformanceValueINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInitializePerformanceApiInfoINTEL
{
	sType: VkStructureType,
	pNext: * const c_void,
	pUserData: * const c_void,
}
impl VkInitializePerformanceApiInfoINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueryPoolCreateInfoINTEL
{
}
impl VkQueryPoolCreateInfoINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPerformanceMarkerInfoINTEL
{
	sType: VkStructureType,
	pNext: * const c_void,
	marker: u64,
}
impl VkPerformanceMarkerInfoINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPerformanceStreamMarkerInfoINTEL
{
	sType: VkStructureType,
	pNext: * const c_void,
	marker: u32,
}
impl VkPerformanceStreamMarkerInfoINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPerformanceOverrideInfoINTEL
{
	sType: VkStructureType,
	pNext: * const c_void,
	type: VkPerformanceOverrideTypeINTEL,
	enable: VkBool32,
	parameter: u64,
}
impl VkPerformanceOverrideInfoINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPerformanceConfigurationAcquireInfoINTEL
{
	sType: VkStructureType,
	pNext: * const c_void,
	type: VkPerformanceConfigurationTypeINTEL,
}
impl VkPerformanceConfigurationAcquireInfoINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR
{
}
impl VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReferenceStencilLayoutKHR
{
}
impl VkAttachmentReferenceStencilLayoutKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentDescriptionStencilLayoutKHR
{
}
impl VkAttachmentDescriptionStencilLayoutKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	pipeline: VkPipeline,
}
impl VkPipelineInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineExecutablePropertiesKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	stages: VkShaderStageFlagBits,
	name: c_char,
	description: c_char,
	subgroupSize: u32,
}
impl VkPipelineExecutablePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineExecutableInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	pipeline: VkPipeline,
	executableIndex: u32,
}
impl VkPipelineExecutableInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineExecutableStatisticKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	name: c_char,
	description: c_char,
	format: VkPipelineExecutableStatisticFormatKHR,
	value: VkPipelineExecutableStatisticValueKHR,
}
impl VkPipelineExecutableStatisticKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineExecutableInternalRepresentationKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	name: c_char,
	description: c_char,
	isText: VkBool32,
	dataSize: size_t,
	pData: * const c_void,
}
impl VkPipelineExecutableInternalRepresentationKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfoKHR
{
}
impl VkMemoryOpaqueCaptureAddressAllocateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	memory: VkDeviceMemory,
}
impl VkDeviceMemoryOpaqueCaptureAddressInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfoKHR
{
}
impl VkDeviceMemoryOpaqueCaptureAddressInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceToolPropertiesEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	name: c_char,
	version: c_char,
	purposes: VkToolPurposeFlagBitsEXT,
	description: c_char,
	layer: c_char,
}
impl VkPhysicalDeviceToolPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureGeometryTrianglesDataKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	vertexFormat: VkFormat,
	vertexData: VkDeviceOrHostAddressConstKHR,
	vertexStride: VkDeviceSize,
	maxVertex: u32,
	indexType: VkIndexType,
	indexData: VkDeviceOrHostAddressConstKHR,
	transformData: VkDeviceOrHostAddressConstKHR,
}
impl VkAccelerationStructureGeometryTrianglesDataKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureGeometryAabbsDataKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	data: VkDeviceOrHostAddressConstKHR,
	stride: VkDeviceSize,
}
impl VkAccelerationStructureGeometryAabbsDataKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureGeometryInstancesDataKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	arrayOfPointers: VkBool32,
	data: VkDeviceOrHostAddressConstKHR,
}
impl VkAccelerationStructureGeometryInstancesDataKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureGeometryKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	geometryType: VkGeometryTypeKHR,
	geometry: VkAccelerationStructureGeometryDataKHR,
	flags: VkGeometryFlagBitsKHR,
}
impl VkAccelerationStructureGeometryKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	type: VkAccelerationStructureTypeKHR,
	flags: VkBuildAccelerationStructureFlagBitsKHR,
	mode: VkBuildAccelerationStructureModeKHR,
	srcAccelerationStructure: VkAccelerationStructureKHR,
	dstAccelerationStructure: VkAccelerationStructureKHR,
	geometryCount: u32,
	pGeometries: * const VkAccelerationStructureGeometryKHR,
	ppGeometries: * const * const VkAccelerationStructureGeometryKHR,
	scratchData: VkDeviceOrHostAddressKHR,
}
impl VkAccelerationStructureBuildGeometryInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureBuildRangeInfoKHR
{
	primitiveCount: u32,
	primitiveOffset: u32,
	firstVertex: u32,
	transformOffset: u32,
}
impl VkAccelerationStructureBuildRangeInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	createFlags: VkAccelerationStructureCreateFlagBitsKHR,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	size: VkDeviceSize,
	type: VkAccelerationStructureTypeKHR,
	deviceAddress: VkDeviceAddress,
}
impl VkAccelerationStructureCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAabbPositionsKHR
{
	minX: f32,
	minY: f32,
	minZ: f32,
	maxX: f32,
	maxY: f32,
	maxZ: f32,
}
impl VkAabbPositionsKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAabbPositionsNV
{
}
impl VkAabbPositionsNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkTransformMatrixKHR
{
	matrix: f32,
}
impl VkTransformMatrixKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkTransformMatrixNV
{
}
impl VkTransformMatrixNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureInstanceKHR
{
	transform: VkTransformMatrixKHR,
	instanceCustomIndex: u32,
	mask: u32,
	instanceShaderBindingTableRecordOffset: u32,
	flags: VkGeometryInstanceFlagBitsKHR,
	accelerationStructureReference: u64,
}
impl VkAccelerationStructureInstanceKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureInstanceNV
{
}
impl VkAccelerationStructureInstanceNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	accelerationStructure: VkAccelerationStructureKHR,
}
impl VkAccelerationStructureDeviceAddressInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureVersionInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	pVersionData: * const u8,
}
impl VkAccelerationStructureVersionInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyAccelerationStructureInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	src: VkAccelerationStructureKHR,
	dst: VkAccelerationStructureKHR,
	mode: VkCopyAccelerationStructureModeKHR,
}
impl VkCopyAccelerationStructureInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyAccelerationStructureToMemoryInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	src: VkAccelerationStructureKHR,
	dst: VkDeviceOrHostAddressKHR,
	mode: VkCopyAccelerationStructureModeKHR,
}
impl VkCopyAccelerationStructureToMemoryInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyMemoryToAccelerationStructureInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	src: VkDeviceOrHostAddressConstKHR,
	dst: VkAccelerationStructureKHR,
	mode: VkCopyAccelerationStructureModeKHR,
}
impl VkCopyMemoryToAccelerationStructureInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	maxPipelineRayPayloadSize: u32,
	maxPipelineRayHitAttributeSize: u32,
}
impl VkRayTracingPipelineInterfaceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineLibraryCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	libraryCount: u32,
	pLibraries: * const VkPipeline,
}
impl VkPipelineLibraryCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCopy2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcOffset: VkDeviceSize,
	dstOffset: VkDeviceSize,
	size: VkDeviceSize,
}
impl VkBufferCopy2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_COPY_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageCopy2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcSubresource: VkImageSubresourceLayers,
	srcOffset: VkOffset3D,
	dstSubresource: VkImageSubresourceLayers,
	dstOffset: VkOffset3D,
	extent: VkExtent3D,
}
impl VkImageCopy2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_COPY_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageBlit2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcSubresource: VkImageSubresourceLayers,
	srcOffsets: VkOffset3D,
	dstSubresource: VkImageSubresourceLayers,
	dstOffsets: VkOffset3D,
}
impl VkImageBlit2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_BLIT_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferImageCopy2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	bufferOffset: VkDeviceSize,
	bufferRowLength: u32,
	bufferImageHeight: u32,
	imageSubresource: VkImageSubresourceLayers,
	imageOffset: VkOffset3D,
	imageExtent: VkExtent3D,
}
impl VkBufferImageCopy2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageResolve2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcSubresource: VkImageSubresourceLayers,
	srcOffset: VkOffset3D,
	dstSubresource: VkImageSubresourceLayers,
	dstOffset: VkOffset3D,
	extent: VkExtent3D,
}
impl VkImageResolve2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyBufferInfo2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcBuffer: VkBuffer,
	dstBuffer: VkBuffer,
	regionCount: u32,
	pRegions: * const VkBufferCopy2KHR,
}
impl VkCopyBufferInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyImageInfo2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: * const VkImageCopy2KHR,
}
impl VkCopyImageInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBlitImageInfo2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: * const VkImageBlit2KHR,
	filter: VkFilter,
}
impl VkBlitImageInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyBufferToImageInfo2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcBuffer: VkBuffer,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: * const VkBufferImageCopy2KHR,
}
impl VkCopyBufferToImageInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyImageToBufferInfo2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstBuffer: VkBuffer,
	regionCount: u32,
	pRegions: * const VkBufferImageCopy2KHR,
}
impl VkCopyImageToBufferInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkResolveImageInfo2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: * const VkImageResolve2KHR,
}
impl VkResolveImageInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFragmentShadingRateKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	sampleCounts: VkSampleCountFlagBits,
	fragmentSize: VkExtent2D,
}
impl VkPhysicalDeviceFragmentShadingRateKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureBuildSizesInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	accelerationStructureSize: VkDeviceSize,
	updateScratchSize: VkDeviceSize,
	buildScratchSize: VkDeviceSize,
}
impl VkAccelerationStructureBuildSizesInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMutableDescriptorTypeListVALVE
{
	descriptorTypeCount: u32,
	pDescriptorTypes: * const VkDescriptorType,
}
impl VkMutableDescriptorTypeListVALVE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputBindingDescription2EXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	binding: u32,
	stride: u32,
	inputRate: VkVertexInputRate,
	divisor: u32,
}
impl VkVertexInputBindingDescription2EXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputAttributeDescription2EXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	location: u32,
	binding: u32,
	format: VkFormat,
	offset: u32,
}
impl VkVertexInputAttributeDescription2EXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageMemoryBarrier2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcStageMask: VkPipelineStageFlagBits2KHR,
	srcAccessMask: VkAccessFlagBits2KHR,
	dstStageMask: VkPipelineStageFlagBits2KHR,
	dstAccessMask: VkAccessFlagBits2KHR,
	oldLayout: VkImageLayout,
	newLayout: VkImageLayout,
	srcQueueFamilyIndex: u32,
	dstQueueFamilyIndex: u32,
	image: VkImage,
	subresourceRange: VkImageSubresourceRange,
}
impl VkImageMemoryBarrier2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferMemoryBarrier2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	srcStageMask: VkPipelineStageFlagBits2KHR,
	srcAccessMask: VkAccessFlagBits2KHR,
	dstStageMask: VkPipelineStageFlagBits2KHR,
	dstAccessMask: VkAccessFlagBits2KHR,
	srcQueueFamilyIndex: u32,
	dstQueueFamilyIndex: u32,
	buffer: VkBuffer,
	offset: VkDeviceSize,
	size: VkDeviceSize,
}
impl VkBufferMemoryBarrier2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDependencyInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	dependencyFlags: VkDependencyFlagBits,
	memoryBarrierCount: u32,
	pMemoryBarriers: * const VkMemoryBarrier2KHR,
	bufferMemoryBarrierCount: u32,
	pBufferMemoryBarriers: * const VkBufferMemoryBarrier2KHR,
	imageMemoryBarrierCount: u32,
	pImageMemoryBarriers: * const VkImageMemoryBarrier2KHR,
}
impl VkDependencyInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreSubmitInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	semaphore: VkSemaphore,
	value: u64,
	stageMask: VkPipelineStageFlagBits2KHR,
	deviceIndex: u32,
}
impl VkSemaphoreSubmitInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferSubmitInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	commandBuffer: VkCommandBuffer,
	deviceMask: u32,
}
impl VkCommandBufferSubmitInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubmitInfo2KHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkSubmitFlagBitsKHR,
	waitSemaphoreInfoCount: u32,
	pWaitSemaphoreInfos: * const VkSemaphoreSubmitInfoKHR,
	commandBufferInfoCount: u32,
	pCommandBufferInfos: * const VkCommandBufferSubmitInfoKHR,
	signalSemaphoreInfoCount: u32,
	pSignalSemaphoreInfos: * const VkSemaphoreSubmitInfoKHR,
}
impl VkSubmitInfo2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCheckpointData2NV
{
	sType: VkStructureType,
	pNext: * const c_void,
	stage: VkPipelineStageFlagBits2KHR,
	pCheckpointMarker: * const c_void,
}
impl VkCheckpointData2NV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceVideoFormatInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	imageUsage: VkImageUsageFlagBits,
	pVideoProfiles: * const VkVideoProfilesKHR,
}
impl VkPhysicalDeviceVideoFormatInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoFormatPropertiesKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	format: VkFormat,
}
impl VkVideoFormatPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoCapabilitiesKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	capabilityFlags: VkVideoCapabilityFlagBitsKHR,
	minBitstreamBufferOffsetAlignment: VkDeviceSize,
	minBitstreamBufferSizeAlignment: VkDeviceSize,
	videoPictureExtentGranularity: VkExtent2D,
	minExtent: VkExtent2D,
	maxExtent: VkExtent2D,
	maxReferencePicturesSlotsCount: u32,
	maxReferencePicturesActiveCount: u32,
}
impl VkVideoCapabilitiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoGetMemoryPropertiesKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	memoryBindIndex: u32,
	pMemoryRequirements: * const VkMemoryRequirements2,
}
impl VkVideoGetMemoryPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_GET_MEMORY_PROPERTIES_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoBindMemoryKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	memoryBindIndex: u32,
	memory: VkDeviceMemory,
	memoryOffset: VkDeviceSize,
	memorySize: VkDeviceSize,
}
impl VkVideoBindMemoryKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_BIND_MEMORY_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoPictureResourceKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	codedOffset: VkOffset2D,
	codedExtent: VkExtent2D,
	baseArrayLayer: u32,
	imageViewBinding: VkImageView,
}
impl VkVideoPictureResourceKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoReferenceSlotKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	slotIndex: i8,
	pPictureResource: * const VkVideoPictureResourceKHR,
}
impl VkVideoReferenceSlotKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoDecodeInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkVideoDecodeFlagBitsKHR,
	codedOffset: VkOffset2D,
	codedExtent: VkExtent2D,
	srcBuffer: VkBuffer,
	srcBufferOffset: VkDeviceSize,
	srcBufferRange: VkDeviceSize,
	dstPictureResource: VkVideoPictureResourceKHR,
	pSetupReferenceSlot: * const VkVideoReferenceSlotKHR,
	referenceSlotCount: u32,
	pReferenceSlots: * const VkVideoReferenceSlotKHR,
}
impl VkVideoDecodeInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoSessionCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	queueFamilyIndex: u32,
	flags: VkVideoSessionCreateFlagBitsKHR,
	pVideoProfile: * const VkVideoProfileKHR,
	pictureFormat: VkFormat,
	maxCodedExtent: VkExtent2D,
	referencePicturesFormat: VkFormat,
	maxReferencePicturesSlotsCount: u32,
	maxReferencePicturesActiveCount: u32,
}
impl VkVideoSessionCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoSessionParametersCreateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	videoSessionParametersTemplate: VkVideoSessionParametersKHR,
	videoSession: VkVideoSessionKHR,
}
impl VkVideoSessionParametersCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoSessionParametersUpdateInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	updateSequenceCount: u32,
}
impl VkVideoSessionParametersUpdateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoBeginCodingInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkVideoBeginCodingFlagBitsKHR,
	codecQualityPreset: VkVideoCodingQualityPresetFlagBitsKHR,
	videoSession: VkVideoSessionKHR,
	videoSessionParameters: VkVideoSessionParametersKHR,
	referenceSlotCount: u32,
	pReferenceSlots: * const VkVideoReferenceSlotKHR,
}
impl VkVideoBeginCodingInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoEndCodingInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkVideoEndCodingFlagBitsKHR,
}
impl VkVideoEndCodingInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoCodingControlInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkVideoCodingControlFlagBitsKHR,
}
impl VkVideoCodingControlInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoEncodeInfoKHR
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkVideoEncodeFlagBitsKHR,
	qualityLevel: u32,
	codedExtent: VkExtent2D,
	dstBitstreamBuffer: VkBuffer,
	dstBitstreamBufferOffset: VkDeviceSize,
	dstBitstreamBufferMaxRange: VkDeviceSize,
	srcPictureResource: VkVideoPictureResourceKHR,
	pSetupReferenceSlot: * const VkVideoReferenceSlotKHR,
	referenceSlotCount: u32,
	pReferenceSlots: * const VkVideoReferenceSlotKHR,
}
impl VkVideoEncodeInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_INFO_KHR;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoEncodeH264DpbSlotInfoEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	slotIndex: i8,
	pStdPictureInfo: * const StdVideoEncodeH264PictureInfo,
}
impl VkVideoEncodeH264DpbSlotInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVideoEncodeH264NaluSliceEXT
{
	sType: VkStructureType,
	pNext: * const c_void,
	pSliceHeaderStd: * const StdVideoEncodeH264SliceHeader,
	mbCount: u32,
	refFinalList0EntryCount: u8,
	pRefFinalList0Entries: * const VkVideoEncodeH264DpbSlotInfoEXT,
	refFinalList1EntryCount: u8,
	pRefFinalList1Entries: * const VkVideoEncodeH264DpbSlotInfoEXT,
	precedingNaluBytes: u32,
	minQp: u8,
	maxQp: u8,
}
impl VkVideoEncodeH264NaluSliceEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_NALU_SLICE_EXT;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCuModuleCreateInfoNVX
{
	sType: VkStructureType,
	pNext: * const c_void,
	dataSize: size_t,
	pData: * const c_void,
}
impl VkCuModuleCreateInfoNVX
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCuFunctionCreateInfoNVX
{
	sType: VkStructureType,
	pNext: * const c_void,
	module: VkCuModuleNVX,
	pName: * const c_char,
}
impl VkCuFunctionCreateInfoNVX
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCuLaunchInfoNVX
{
	sType: VkStructureType,
	pNext: * const c_void,
	function: VkCuFunctionNVX,
	gridDimX: u32,
	gridDimY: u32,
	gridDimZ: u32,
	blockDimX: u32,
	blockDimY: u32,
	blockDimZ: u32,
	sharedMemBytes: u32,
	paramCount: size_t,
	pParams: * const * const c_void,
	extraCount: size_t,
	pExtras: * const * const c_void,
}
impl VkCuLaunchInfoNVX
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX;

        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSRTDataNV
{
	sx: f32,
	a: f32,
	b: f32,
	pvx: f32,
	sy: f32,
	c: f32,
	pvy: f32,
	sz: f32,
	pvz: f32,
	qx: f32,
	qy: f32,
	qz: f32,
	qw: f32,
	tx: f32,
	ty: f32,
	tz: f32,
}
impl VkSRTDataNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureSRTMotionInstanceNV
{
	transformT0: VkSRTDataNV,
	transformT1: VkSRTDataNV,
	instanceCustomIndex: u32,
	mask: u32,
	instanceShaderBindingTableRecordOffset: u32,
	flags: VkGeometryInstanceFlagBitsKHR,
	accelerationStructureReference: u64,
}
impl VkAccelerationStructureSRTMotionInstanceNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureMatrixMotionInstanceNV
{
	transformT0: VkTransformMatrixKHR,
	transformT1: VkTransformMatrixKHR,
	instanceCustomIndex: u32,
	mask: u32,
	instanceShaderBindingTableRecordOffset: u32,
	flags: VkGeometryInstanceFlagBitsKHR,
	accelerationStructureReference: u64,
}
impl VkAccelerationStructureMatrixMotionInstanceNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureMotionInstanceNV
{
	type: VkAccelerationStructureMotionInstanceTypeNV,
	flags: VkAccelerationStructureMotionInstanceFlagBitsNV,
	data: VkAccelerationStructureMotionInstanceDataNV,
}
impl VkAccelerationStructureMotionInstanceNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryGetRemoteAddressInfoNV
{
	sType: VkStructureType,
	pNext: * const c_void,
	memory: VkDeviceMemory,
	handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl VkMemoryGetRemoteAddressInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV;

        return s;
    }
}

