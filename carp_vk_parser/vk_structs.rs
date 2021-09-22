#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBaseOutStructure
{
	sType: VkStructureType,
	pNext: * mut VkBaseOutStructure,
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
	pUserData: * mut c_void,
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
	codeSize: usize,
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
	type_type: VkDescriptorType,
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
	size: usize,
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
	dataSize: usize,
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
	initialDataSize: usize,
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
	minMemoryMapAlignment: usize,
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
pub struct VkPhysicalDeviceFeatures2
{
	sType: VkStructureType,
	pNext: * mut c_void,
	features: VkPhysicalDeviceFeatures,
}
impl VkPhysicalDeviceFeatures2
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkFormatProperties2
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkImageFormatProperties2
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkPhysicalDeviceImageFormatInfo2
{
	sType: VkStructureType,
	pNext: * const c_void,
	format: VkFormat,
	type_type: VkImageType,
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
pub struct VkQueueFamilyProperties2
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkPhysicalDeviceMemoryProperties2
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkSparseImageFormatProperties2
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkPhysicalDeviceSparseImageFormatInfo2
{
	sType: VkStructureType,
	pNext: * const c_void,
	format: VkFormat,
	type_type: VkImageType,
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
pub struct VkPhysicalDeviceDriverProperties
{
	sType: VkStructureType,
	pNext: * mut c_void,
	driverID: VkDriverId,
	driverName: c_char,
	driverInfo: c_char,
	conformanceVersion: VkConformanceVersion,
}
impl VkPhysicalDeviceDriverProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	variablePointersStorageBuffer: VkBool32,
	variablePointers: VkBool32,
}
impl VkPhysicalDeviceVariablePointersFeatures
{
    fn new() -> Self
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
pub struct VkPhysicalDeviceExternalImageFormatInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	handleType: VkExternalMemoryHandleTypeFlagBits,
}
impl VkPhysicalDeviceExternalImageFormatInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	externalMemoryProperties: VkExternalMemoryProperties,
}
impl VkExternalImageFormatProperties
{
    fn new() -> Self
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
pub struct VkExternalBufferProperties
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkPhysicalDeviceIDProperties
{
	sType: VkStructureType,
	pNext: * mut c_void,
	deviceUUID: u8,
	driverUUID: u8,
	deviceLUID: u8,
	deviceNodeMask: u32,
	deviceLUIDValid: VkBool32,
}
impl VkPhysicalDeviceIDProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	handleTypes: VkExternalMemoryHandleTypeFlagBits,
}
impl VkExternalMemoryImageCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	handleTypes: VkExternalMemoryHandleTypeFlagBits,
}
impl VkExternalMemoryBufferCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	handleTypes: VkExternalMemoryHandleTypeFlagBits,
}
impl VkExportMemoryAllocateInfo
{
    fn new() -> Self
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
pub struct VkExternalSemaphoreProperties
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkExportSemaphoreCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	handleTypes: VkExternalSemaphoreHandleTypeFlagBits,
}
impl VkExportSemaphoreCreateInfo
{
    fn new() -> Self
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
pub struct VkExternalFenceProperties
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkExportFenceCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	handleTypes: VkExternalFenceHandleTypeFlagBits,
}
impl VkExportFenceCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	multiview: VkBool32,
	multiviewGeometryShader: VkBool32,
	multiviewTessellationShader: VkBool32,
}
impl VkPhysicalDeviceMultiviewFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	maxMultiviewViewCount: u32,
	maxMultiviewInstanceIndex: u32,
}
impl VkPhysicalDeviceMultiviewProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	subpassCount: u32,
	pViewMasks: * const u32,
	dependencyCount: u32,
	pViewOffsets: * const i32,
	correlationMaskCount: u32,
	pCorrelationMasks: * const u32,
}
impl VkRenderPassMultiviewCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkMemoryAllocateFlagsInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	flags: VkMemoryAllocateFlagBits,
	deviceMask: u32,
}
impl VkMemoryAllocateFlagsInfo
{
    fn new() -> Self
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
pub struct VkBindBufferMemoryDeviceGroupInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	deviceIndexCount: u32,
	pDeviceIndices: * const u32,
}
impl VkBindBufferMemoryDeviceGroupInfo
{
    fn new() -> Self
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
pub struct VkBindImageMemoryDeviceGroupInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	deviceIndexCount: u32,
	pDeviceIndices: * const u32,
	splitInstanceBindRegionCount: u32,
	pSplitInstanceBindRegions: * const VkRect2D,
}
impl VkBindImageMemoryDeviceGroupInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	deviceMask: u32,
	deviceRenderAreaCount: u32,
	pDeviceRenderAreas: * const VkRect2D,
}
impl VkDeviceGroupRenderPassBeginInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	deviceMask: u32,
}
impl VkDeviceGroupCommandBufferBeginInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	waitSemaphoreCount: u32,
	pWaitSemaphoreDeviceIndices: * const u32,
	commandBufferCount: u32,
	pCommandBufferDeviceMasks: * const u32,
	signalSemaphoreCount: u32,
	pSignalSemaphoreDeviceIndices: * const u32,
}
impl VkDeviceGroupSubmitInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	resourceDeviceIndex: u32,
	memoryDeviceIndex: u32,
}
impl VkDeviceGroupBindSparseInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	physicalDeviceCount: u32,
	pPhysicalDevices: * const VkPhysicalDevice,
}
impl VkDeviceGroupDeviceCreateInfo
{
    fn new() -> Self
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
	dstBinding: u32,
	dstArrayElement: u32,
	descriptorCount: u32,
	descriptorType: VkDescriptorType,
	offset: usize,
	stride: usize,
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
pub struct VkRenderPassInputAttachmentAspectCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	aspectReferenceCount: u32,
	pAspectReferences: * const VkInputAttachmentAspectReference,
}
impl VkRenderPassInputAttachmentAspectCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	storageBuffer16BitAccess: VkBool32,
	uniformAndStorageBuffer16BitAccess: VkBool32,
	storagePushConstant16: VkBool32,
	storageInputOutput16: VkBool32,
}
impl VkPhysicalDevice16BitStorageFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	subgroupSize: u32,
	supportedStages: VkShaderStageFlagBits,
	supportedOperations: VkSubgroupFeatureFlagBits,
	quadOperationsInAllStages: VkBool32,
}
impl VkPhysicalDeviceSubgroupProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	shaderSubgroupExtendedTypes: VkBool32,
}
impl VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures
{
    fn new() -> Self
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
pub struct VkMemoryRequirements2
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkSparseImageMemoryRequirements2
{
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkPhysicalDevicePointClippingProperties
{
	sType: VkStructureType,
	pNext: * mut c_void,
	pointClippingBehavior: VkPointClippingBehavior,
}
impl VkPhysicalDevicePointClippingProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	prefersDedicatedAllocation: VkBool32,
	requiresDedicatedAllocation: VkBool32,
}
impl VkMemoryDedicatedRequirements
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	image: VkImage,
	buffer: VkBuffer,
}
impl VkMemoryDedicatedAllocateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	usage: VkImageUsageFlagBits,
}
impl VkImageViewUsageCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	domainOrigin: VkTessellationDomainOrigin,
}
impl VkPipelineTessellationDomainOriginStateCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	conversion: VkSamplerYcbcrConversion,
}
impl VkSamplerYcbcrConversionInfo
{
    fn new() -> Self
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
pub struct VkBindImagePlaneMemoryInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	planeAspect: VkImageAspectFlagBits,
}
impl VkBindImagePlaneMemoryInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	planeAspect: VkImageAspectFlagBits,
}
impl VkImagePlaneMemoryRequirementsInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	samplerYcbcrConversion: VkBool32,
}
impl VkPhysicalDeviceSamplerYcbcrConversionFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	combinedImageSamplerDescriptorCount: u32,
}
impl VkSamplerYcbcrConversionImageFormatProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	protectedSubmit: VkBool32,
}
impl VkProtectedSubmitInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	protectedMemory: VkBool32,
}
impl VkPhysicalDeviceProtectedMemoryFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	protectedNoFault: VkBool32,
}
impl VkPhysicalDeviceProtectedMemoryProperties
{
    fn new() -> Self
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
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties
{
	sType: VkStructureType,
	pNext: * mut c_void,
	filterMinmaxSingleComponentFormats: VkBool32,
	filterMinmaxImageComponentMapping: VkBool32,
}
impl VkPhysicalDeviceSamplerFilterMinmaxProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	reductionMode: VkSamplerReductionMode,
}
impl VkSamplerReductionModeCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	viewFormatCount: u32,
	pViewFormats: * const VkFormat,
}
impl VkImageFormatListCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	maxPerSetDescriptors: u32,
	maxMemoryAllocationSize: VkDeviceSize,
}
impl VkPhysicalDeviceMaintenance3Properties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
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
pub struct VkPhysicalDeviceShaderDrawParametersFeatures
{
	sType: VkStructureType,
	pNext: * mut c_void,
	shaderDrawParameters: VkBool32,
}
impl VkPhysicalDeviceShaderDrawParametersFeatures
{
    fn new() -> Self
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
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features
{
	sType: VkStructureType,
	pNext: * mut c_void,
	shaderFloat16: VkBool32,
	shaderInt8: VkBool32,
}
impl VkPhysicalDeviceShaderFloat16Int8Features
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	denormBehaviorIndependence: VkShaderFloatControlsIndependence,
	roundingModeIndependence: VkShaderFloatControlsIndependence,
	shaderSignedZeroInfNanPreserveFloat16: VkBool32,
	shaderSignedZeroInfNanPreserveFloat32: VkBool32,
	shaderSignedZeroInfNanPreserveFloat64: VkBool32,
	shaderDenormPreserveFloat16: VkBool32,
	shaderDenormPreserveFloat32: VkBool32,
	shaderDenormPreserveFloat64: VkBool32,
	shaderDenormFlushToZeroFloat16: VkBool32,
	shaderDenormFlushToZeroFloat32: VkBool32,
	shaderDenormFlushToZeroFloat64: VkBool32,
	shaderRoundingModeRTEFloat16: VkBool32,
	shaderRoundingModeRTEFloat32: VkBool32,
	shaderRoundingModeRTEFloat64: VkBool32,
	shaderRoundingModeRTZFloat16: VkBool32,
	shaderRoundingModeRTZFloat32: VkBool32,
	shaderRoundingModeRTZFloat64: VkBool32,
}
impl VkPhysicalDeviceFloatControlsProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	hostQueryReset: VkBool32,
}
impl VkPhysicalDeviceHostQueryResetFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	shaderInputAttachmentArrayDynamicIndexing: VkBool32,
	shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
	shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
	shaderUniformBufferArrayNonUniformIndexing: VkBool32,
	shaderSampledImageArrayNonUniformIndexing: VkBool32,
	shaderStorageBufferArrayNonUniformIndexing: VkBool32,
	shaderStorageImageArrayNonUniformIndexing: VkBool32,
	shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
	shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
	shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
	descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
	descriptorBindingSampledImageUpdateAfterBind: VkBool32,
	descriptorBindingStorageImageUpdateAfterBind: VkBool32,
	descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
	descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
	descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
	descriptorBindingUpdateUnusedWhilePending: VkBool32,
	descriptorBindingPartiallyBound: VkBool32,
	descriptorBindingVariableDescriptorCount: VkBool32,
	runtimeDescriptorArray: VkBool32,
}
impl VkPhysicalDeviceDescriptorIndexingFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	maxUpdateAfterBindDescriptorsInAllPools: u32,
	shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
	shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
	shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
	shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
	shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
	robustBufferAccessUpdateAfterBind: VkBool32,
	quadDivergentImplicitLod: VkBool32,
	maxPerStageDescriptorUpdateAfterBindSamplers: u32,
	maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
	maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
	maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
	maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
	maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
	maxPerStageUpdateAfterBindResources: u32,
	maxDescriptorSetUpdateAfterBindSamplers: u32,
	maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
	maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
	maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
	maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
	maxDescriptorSetUpdateAfterBindSampledImages: u32,
	maxDescriptorSetUpdateAfterBindStorageImages: u32,
	maxDescriptorSetUpdateAfterBindInputAttachments: u32,
}
impl VkPhysicalDeviceDescriptorIndexingProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	bindingCount: u32,
	pBindingFlags: * const VkDescriptorBindingFlagBits,
}
impl VkDescriptorSetLayoutBindingFlagsCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	descriptorSetCount: u32,
	pDescriptorCounts: * const u32,
}
impl VkDescriptorSetVariableDescriptorCountAllocateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	maxVariableDescriptorCount: u32,
}
impl VkDescriptorSetVariableDescriptorCountLayoutSupport
{
    fn new() -> Self
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
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures
{
	sType: VkStructureType,
	pNext: * mut c_void,
	timelineSemaphore: VkBool32,
}
impl VkPhysicalDeviceTimelineSemaphoreFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	maxTimelineSemaphoreValueDifference: u64,
}
impl VkPhysicalDeviceTimelineSemaphoreProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	semaphoreType: VkSemaphoreType,
	initialValue: u64,
}
impl VkSemaphoreTypeCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	waitSemaphoreValueCount: u32,
	pWaitSemaphoreValues: * const u64,
	signalSemaphoreValueCount: u32,
	pSignalSemaphoreValues: * const u64,
}
impl VkTimelineSemaphoreSubmitInfo
{
    fn new() -> Self
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
pub struct VkPhysicalDevice8BitStorageFeatures
{
	sType: VkStructureType,
	pNext: * mut c_void,
	storageBuffer8BitAccess: VkBool32,
	uniformAndStorageBuffer8BitAccess: VkBool32,
	storagePushConstant8: VkBool32,
}
impl VkPhysicalDevice8BitStorageFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	vulkanMemoryModel: VkBool32,
	vulkanMemoryModelDeviceScope: VkBool32,
	vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
}
impl VkPhysicalDeviceVulkanMemoryModelFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	shaderBufferInt64Atomics: VkBool32,
	shaderSharedInt64Atomics: VkBool32,
}
impl VkPhysicalDeviceShaderAtomicInt64Features
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	supportedDepthResolveModes: VkResolveModeFlagBits,
	supportedStencilResolveModes: VkResolveModeFlagBits,
	independentResolveNone: VkBool32,
	independentResolve: VkBool32,
}
impl VkPhysicalDeviceDepthStencilResolveProperties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	depthResolveMode: VkResolveModeFlagBits,
	stencilResolveMode: VkResolveModeFlagBits,
	pDepthStencilResolveAttachment: * const VkAttachmentReference2,
}
impl VkSubpassDescriptionDepthStencilResolve
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	stencilUsage: VkImageUsageFlagBits,
}
impl VkImageStencilUsageCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	scalarBlockLayout: VkBool32,
}
impl VkPhysicalDeviceScalarBlockLayoutFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	uniformBufferStandardLayout: VkBool32,
}
impl VkPhysicalDeviceUniformBufferStandardLayoutFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	bufferDeviceAddress: VkBool32,
	bufferDeviceAddressCaptureReplay: VkBool32,
	bufferDeviceAddressMultiDevice: VkBool32,
}
impl VkPhysicalDeviceBufferDeviceAddressFeatures
{
    fn new() -> Self
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
pub struct VkBufferOpaqueCaptureAddressCreateInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	opaqueCaptureAddress: u64,
}
impl VkBufferOpaqueCaptureAddressCreateInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	imagelessFramebuffer: VkBool32,
}
impl VkPhysicalDeviceImagelessFramebufferFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	attachmentImageInfoCount: u32,
	pAttachmentImageInfos: * const VkFramebufferAttachmentImageInfo,
}
impl VkFramebufferAttachmentsCreateInfo
{
    fn new() -> Self
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
pub struct VkRenderPassAttachmentBeginInfo
{
	sType: VkStructureType,
	pNext: * const c_void,
	attachmentCount: u32,
	pAttachments: * const VkImageView,
}
impl VkRenderPassAttachmentBeginInfo
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	separateDepthStencilLayouts: VkBool32,
}
impl VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	stencilLayout: VkImageLayout,
}
impl VkAttachmentReferenceStencilLayout
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	stencilInitialLayout: VkImageLayout,
	stencilFinalLayout: VkImageLayout,
}
impl VkAttachmentDescriptionStencilLayout
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * const c_void,
	opaqueCaptureAddress: u64,
}
impl VkMemoryOpaqueCaptureAddressAllocateInfo
{
    fn new() -> Self
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
pub struct VkPhysicalDeviceVulkan11Features
{
	sType: VkStructureType,
	pNext: * mut c_void,
	storageBuffer16BitAccess: VkBool32,
	uniformAndStorageBuffer16BitAccess: VkBool32,
	storagePushConstant16: VkBool32,
	storageInputOutput16: VkBool32,
	multiview: VkBool32,
	multiviewGeometryShader: VkBool32,
	multiviewTessellationShader: VkBool32,
	variablePointersStorageBuffer: VkBool32,
	variablePointers: VkBool32,
	protectedMemory: VkBool32,
	samplerYcbcrConversion: VkBool32,
	shaderDrawParameters: VkBool32,
}
impl VkPhysicalDeviceVulkan11Features
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	deviceUUID: u8,
	driverUUID: u8,
	deviceLUID: u8,
	deviceNodeMask: u32,
	deviceLUIDValid: VkBool32,
	subgroupSize: u32,
	subgroupSupportedStages: VkShaderStageFlagBits,
	subgroupSupportedOperations: VkSubgroupFeatureFlagBits,
	subgroupQuadOperationsInAllStages: VkBool32,
	pointClippingBehavior: VkPointClippingBehavior,
	maxMultiviewViewCount: u32,
	maxMultiviewInstanceIndex: u32,
	protectedNoFault: VkBool32,
	maxPerSetDescriptors: u32,
	maxMemoryAllocationSize: VkDeviceSize,
}
impl VkPhysicalDeviceVulkan11Properties
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	samplerMirrorClampToEdge: VkBool32,
	drawIndirectCount: VkBool32,
	storageBuffer8BitAccess: VkBool32,
	uniformAndStorageBuffer8BitAccess: VkBool32,
	storagePushConstant8: VkBool32,
	shaderBufferInt64Atomics: VkBool32,
	shaderSharedInt64Atomics: VkBool32,
	shaderFloat16: VkBool32,
	shaderInt8: VkBool32,
	descriptorIndexing: VkBool32,
	shaderInputAttachmentArrayDynamicIndexing: VkBool32,
	shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
	shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
	shaderUniformBufferArrayNonUniformIndexing: VkBool32,
	shaderSampledImageArrayNonUniformIndexing: VkBool32,
	shaderStorageBufferArrayNonUniformIndexing: VkBool32,
	shaderStorageImageArrayNonUniformIndexing: VkBool32,
	shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
	shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
	shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
	descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
	descriptorBindingSampledImageUpdateAfterBind: VkBool32,
	descriptorBindingStorageImageUpdateAfterBind: VkBool32,
	descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
	descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
	descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
	descriptorBindingUpdateUnusedWhilePending: VkBool32,
	descriptorBindingPartiallyBound: VkBool32,
	descriptorBindingVariableDescriptorCount: VkBool32,
	runtimeDescriptorArray: VkBool32,
	samplerFilterMinmax: VkBool32,
	scalarBlockLayout: VkBool32,
	imagelessFramebuffer: VkBool32,
	uniformBufferStandardLayout: VkBool32,
	shaderSubgroupExtendedTypes: VkBool32,
	separateDepthStencilLayouts: VkBool32,
	hostQueryReset: VkBool32,
	timelineSemaphore: VkBool32,
	bufferDeviceAddress: VkBool32,
	bufferDeviceAddressCaptureReplay: VkBool32,
	bufferDeviceAddressMultiDevice: VkBool32,
	vulkanMemoryModel: VkBool32,
	vulkanMemoryModelDeviceScope: VkBool32,
	vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
	shaderOutputViewportIndex: VkBool32,
	shaderOutputLayer: VkBool32,
	subgroupBroadcastDynamicId: VkBool32,
}
impl VkPhysicalDeviceVulkan12Features
{
    fn new() -> Self
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
	sType: VkStructureType,
	pNext: * mut c_void,
	driverID: VkDriverId,
	driverName: c_char,
	driverInfo: c_char,
	conformanceVersion: VkConformanceVersion,
	denormBehaviorIndependence: VkShaderFloatControlsIndependence,
	roundingModeIndependence: VkShaderFloatControlsIndependence,
	shaderSignedZeroInfNanPreserveFloat16: VkBool32,
	shaderSignedZeroInfNanPreserveFloat32: VkBool32,
	shaderSignedZeroInfNanPreserveFloat64: VkBool32,
	shaderDenormPreserveFloat16: VkBool32,
	shaderDenormPreserveFloat32: VkBool32,
	shaderDenormPreserveFloat64: VkBool32,
	shaderDenormFlushToZeroFloat16: VkBool32,
	shaderDenormFlushToZeroFloat32: VkBool32,
	shaderDenormFlushToZeroFloat64: VkBool32,
	shaderRoundingModeRTEFloat16: VkBool32,
	shaderRoundingModeRTEFloat32: VkBool32,
	shaderRoundingModeRTEFloat64: VkBool32,
	shaderRoundingModeRTZFloat16: VkBool32,
	shaderRoundingModeRTZFloat32: VkBool32,
	shaderRoundingModeRTZFloat64: VkBool32,
	maxUpdateAfterBindDescriptorsInAllPools: u32,
	shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
	shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
	shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
	shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
	shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
	robustBufferAccessUpdateAfterBind: VkBool32,
	quadDivergentImplicitLod: VkBool32,
	maxPerStageDescriptorUpdateAfterBindSamplers: u32,
	maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
	maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
	maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
	maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
	maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
	maxPerStageUpdateAfterBindResources: u32,
	maxDescriptorSetUpdateAfterBindSamplers: u32,
	maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
	maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
	maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
	maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
	maxDescriptorSetUpdateAfterBindSampledImages: u32,
	maxDescriptorSetUpdateAfterBindStorageImages: u32,
	maxDescriptorSetUpdateAfterBindInputAttachments: u32,
	supportedDepthResolveModes: VkResolveModeFlagBits,
	supportedStencilResolveModes: VkResolveModeFlagBits,
	independentResolveNone: VkBool32,
	independentResolve: VkBool32,
	filterMinmaxSingleComponentFormats: VkBool32,
	filterMinmaxImageComponentMapping: VkBool32,
	maxTimelineSemaphoreValueDifference: u64,
	framebufferIntegerColorSampleCounts: VkSampleCountFlagBits,
}
impl VkPhysicalDeviceVulkan12Properties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES;

        return s;
    }
}

