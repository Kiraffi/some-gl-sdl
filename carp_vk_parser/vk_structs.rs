pub struct VkBaseOutStructure
{
	sType: VkStructureType
	pNext: VkBaseOutStructure
}
impl VkBaseOutStructure
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkBaseInStructure
{
	sType: VkStructureType
	pNext: VkBaseInStructure
}
impl VkBaseInStructure
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkOffset2D
{
	x: int32_t
	y: int32_t
}
impl VkOffset2D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkOffset3D
{
	x: int32_t
	y: int32_t
	z: int32_t
}
impl VkOffset3D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkExtent2D
{
	width: uint32_t
	height: uint32_t
}
impl VkExtent2D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkExtent3D
{
	width: uint32_t
	height: uint32_t
	depth: uint32_t
}
impl VkExtent3D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkViewport
{
	x: float
	y: float
	width: float
	height: float
	minDepth: float
	maxDepth: float
}
impl VkViewport
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkRect2D
{
	offset: VkOffset2D
	extent: VkExtent2D
}
impl VkRect2D
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkClearRect
{
	rect: VkRect2D
	baseArrayLayer: uint32_t
	layerCount: uint32_t
}
impl VkClearRect
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkComponentMapping
{
	r: VkComponentSwizzle
	g: VkComponentSwizzle
	b: VkComponentSwizzle
	a: VkComponentSwizzle
}
impl VkComponentMapping
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPhysicalDeviceProperties
{
	apiVersion: uint32_t
	driverVersion: uint32_t
	vendorID: uint32_t
	deviceID: uint32_t
	deviceType: VkPhysicalDeviceType
	deviceName: char
	pipelineCacheUUID: uint8_t
	limits: VkPhysicalDeviceLimits
	sparseProperties: VkPhysicalDeviceSparseProperties
}
impl VkPhysicalDeviceProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkExtensionProperties
{
	extensionName: char
	specVersion: uint32_t
}
impl VkExtensionProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkLayerProperties
{
	layerName: char
	specVersion: uint32_t
	implementationVersion: uint32_t
	description: char
}
impl VkLayerProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkApplicationInfo
{
	sType: VkStructureType
	pNext: void
	pApplicationName: char
	applicationVersion: uint32_t
	pEngineName: char
	engineVersion: uint32_t
	apiVersion: uint32_t
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

pub struct VkAllocationCallbacks
{
	pUserData: void
	pfnAllocation: PFN_vkAllocationFunction
	pfnReallocation: PFN_vkReallocationFunction
	pfnFree: PFN_vkFreeFunction
	pfnInternalAllocation: PFN_vkInternalAllocationNotification
	pfnInternalFree: PFN_vkInternalFreeNotification
}
impl VkAllocationCallbacks
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDeviceQueueCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkDeviceQueueCreateFlags
	queueFamilyIndex: uint32_t
	queueCount: uint32_t
	pQueuePriorities: float
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

pub struct VkDeviceCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkDeviceCreateFlags
	queueCreateInfoCount: uint32_t
	pQueueCreateInfos: VkDeviceQueueCreateInfo
	enabledLayerCount: uint32_t
	ppEnabledLayerNames: char
	enabledExtensionCount: uint32_t
	ppEnabledExtensionNames: char
	pEnabledFeatures: VkPhysicalDeviceFeatures
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

pub struct VkInstanceCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkInstanceCreateFlags
	pApplicationInfo: VkApplicationInfo
	enabledLayerCount: uint32_t
	ppEnabledLayerNames: char
	enabledExtensionCount: uint32_t
	ppEnabledExtensionNames: char
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

pub struct VkQueueFamilyProperties
{
	queueFlags: VkQueueFlags
	queueCount: uint32_t
	timestampValidBits: uint32_t
	minImageTransferGranularity: VkExtent3D
}
impl VkQueueFamilyProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPhysicalDeviceMemoryProperties
{
	memoryTypeCount: uint32_t
	memoryTypes: VkMemoryType
	memoryHeapCount: uint32_t
	memoryHeaps: VkMemoryHeap
}
impl VkPhysicalDeviceMemoryProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkMemoryAllocateInfo
{
	sType: VkStructureType
	pNext: void
	allocationSize: VkDeviceSize
	memoryTypeIndex: uint32_t
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

pub struct VkMemoryRequirements
{
	size: VkDeviceSize
	alignment: VkDeviceSize
	memoryTypeBits: uint32_t
}
impl VkMemoryRequirements
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSparseImageFormatProperties
{
	aspectMask: VkImageAspectFlags
	imageGranularity: VkExtent3D
	flags: VkSparseImageFormatFlags
}
impl VkSparseImageFormatProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSparseImageMemoryRequirements
{
	formatProperties: VkSparseImageFormatProperties
	imageMipTailFirstLod: uint32_t
	imageMipTailSize: VkDeviceSize
	imageMipTailOffset: VkDeviceSize
	imageMipTailStride: VkDeviceSize
}
impl VkSparseImageMemoryRequirements
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkMemoryType
{
	propertyFlags: VkMemoryPropertyFlags
	heapIndex: uint32_t
}
impl VkMemoryType
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkMemoryHeap
{
	size: VkDeviceSize
	flags: VkMemoryHeapFlags
}
impl VkMemoryHeap
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkMappedMemoryRange
{
	sType: VkStructureType
	pNext: void
	memory: VkDeviceMemory
	offset: VkDeviceSize
	size: VkDeviceSize
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

pub struct VkFormatProperties
{
	linearTilingFeatures: VkFormatFeatureFlags
	optimalTilingFeatures: VkFormatFeatureFlags
	bufferFeatures: VkFormatFeatureFlags
}
impl VkFormatProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkImageFormatProperties
{
	maxExtent: VkExtent3D
	maxMipLevels: uint32_t
	maxArrayLayers: uint32_t
	sampleCounts: VkSampleCountFlags
	maxResourceSize: VkDeviceSize
}
impl VkImageFormatProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDescriptorBufferInfo
{
	buffer: VkBuffer
	offset: VkDeviceSize
	range: VkDeviceSize
}
impl VkDescriptorBufferInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDescriptorImageInfo
{
	sampler: VkSampler
	imageView: VkImageView
	imageLayout: VkImageLayout
}
impl VkDescriptorImageInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkWriteDescriptorSet
{
	sType: VkStructureType
	pNext: void
	dstSet: VkDescriptorSet
	dstBinding: uint32_t
	dstArrayElement: uint32_t
	descriptorCount: uint32_t
	descriptorType: VkDescriptorType
	pImageInfo: VkDescriptorImageInfo
	pBufferInfo: VkDescriptorBufferInfo
	pTexelBufferView: VkBufferView
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

pub struct VkCopyDescriptorSet
{
	sType: VkStructureType
	pNext: void
	srcSet: VkDescriptorSet
	srcBinding: uint32_t
	srcArrayElement: uint32_t
	dstSet: VkDescriptorSet
	dstBinding: uint32_t
	dstArrayElement: uint32_t
	descriptorCount: uint32_t
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

pub struct VkBufferCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkBufferCreateFlags
	size: VkDeviceSize
	usage: VkBufferUsageFlags
	sharingMode: VkSharingMode
	queueFamilyIndexCount: uint32_t
	pQueueFamilyIndices: uint32_t
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

pub struct VkBufferViewCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkBufferViewCreateFlags
	buffer: VkBuffer
	format: VkFormat
	offset: VkDeviceSize
	range: VkDeviceSize
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

pub struct VkImageSubresource
{
	aspectMask: VkImageAspectFlags
	mipLevel: uint32_t
	arrayLayer: uint32_t
}
impl VkImageSubresource
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkImageSubresourceLayers
{
	aspectMask: VkImageAspectFlags
	mipLevel: uint32_t
	baseArrayLayer: uint32_t
	layerCount: uint32_t
}
impl VkImageSubresourceLayers
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkImageSubresourceRange
{
	aspectMask: VkImageAspectFlags
	baseMipLevel: uint32_t
	levelCount: uint32_t
	baseArrayLayer: uint32_t
	layerCount: uint32_t
}
impl VkImageSubresourceRange
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkMemoryBarrier
{
	sType: VkStructureType
	pNext: void
	srcAccessMask: VkAccessFlags
	dstAccessMask: VkAccessFlags
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

pub struct VkBufferMemoryBarrier
{
	sType: VkStructureType
	pNext: void
	srcAccessMask: VkAccessFlags
	dstAccessMask: VkAccessFlags
	srcQueueFamilyIndex: uint32_t
	dstQueueFamilyIndex: uint32_t
	buffer: VkBuffer
	offset: VkDeviceSize
	size: VkDeviceSize
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

pub struct VkImageMemoryBarrier
{
	sType: VkStructureType
	pNext: void
	srcAccessMask: VkAccessFlags
	dstAccessMask: VkAccessFlags
	oldLayout: VkImageLayout
	newLayout: VkImageLayout
	srcQueueFamilyIndex: uint32_t
	dstQueueFamilyIndex: uint32_t
	image: VkImage
	subresourceRange: VkImageSubresourceRange
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

pub struct VkImageCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkImageCreateFlags
	imageType: VkImageType
	format: VkFormat
	extent: VkExtent3D
	mipLevels: uint32_t
	arrayLayers: uint32_t
	samples: VkSampleCountFlagBits
	tiling: VkImageTiling
	usage: VkImageUsageFlags
	sharingMode: VkSharingMode
	queueFamilyIndexCount: uint32_t
	pQueueFamilyIndices: uint32_t
	initialLayout: VkImageLayout
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

pub struct VkSubresourceLayout
{
	offset: VkDeviceSize
	size: VkDeviceSize
	rowPitch: VkDeviceSize
	arrayPitch: VkDeviceSize
	depthPitch: VkDeviceSize
}
impl VkSubresourceLayout
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkImageViewCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkImageViewCreateFlags
	image: VkImage
	viewType: VkImageViewType
	format: VkFormat
	components: VkComponentMapping
	subresourceRange: VkImageSubresourceRange
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

pub struct VkBufferCopy
{
	srcOffset: VkDeviceSize
	dstOffset: VkDeviceSize
	size: VkDeviceSize
}
impl VkBufferCopy
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSparseMemoryBind
{
	resourceOffset: VkDeviceSize
	size: VkDeviceSize
	memory: VkDeviceMemory
	memoryOffset: VkDeviceSize
	flags: VkSparseMemoryBindFlags
}
impl VkSparseMemoryBind
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSparseImageMemoryBind
{
	subresource: VkImageSubresource
	offset: VkOffset3D
	extent: VkExtent3D
	memory: VkDeviceMemory
	memoryOffset: VkDeviceSize
	flags: VkSparseMemoryBindFlags
}
impl VkSparseImageMemoryBind
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSparseBufferMemoryBindInfo
{
	buffer: VkBuffer
	bindCount: uint32_t
	pBinds: VkSparseMemoryBind
}
impl VkSparseBufferMemoryBindInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSparseImageOpaqueMemoryBindInfo
{
	image: VkImage
	bindCount: uint32_t
	pBinds: VkSparseMemoryBind
}
impl VkSparseImageOpaqueMemoryBindInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSparseImageMemoryBindInfo
{
	image: VkImage
	bindCount: uint32_t
	pBinds: VkSparseImageMemoryBind
}
impl VkSparseImageMemoryBindInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkBindSparseInfo
{
	sType: VkStructureType
	pNext: void
	waitSemaphoreCount: uint32_t
	pWaitSemaphores: VkSemaphore
	bufferBindCount: uint32_t
	pBufferBinds: VkSparseBufferMemoryBindInfo
	imageOpaqueBindCount: uint32_t
	pImageOpaqueBinds: VkSparseImageOpaqueMemoryBindInfo
	imageBindCount: uint32_t
	pImageBinds: VkSparseImageMemoryBindInfo
	signalSemaphoreCount: uint32_t
	pSignalSemaphores: VkSemaphore
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

pub struct VkImageCopy
{
	srcSubresource: VkImageSubresourceLayers
	srcOffset: VkOffset3D
	dstSubresource: VkImageSubresourceLayers
	dstOffset: VkOffset3D
	extent: VkExtent3D
}
impl VkImageCopy
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkImageBlit
{
	srcSubresource: VkImageSubresourceLayers
	srcOffsets: VkOffset3D
	dstSubresource: VkImageSubresourceLayers
	dstOffsets: VkOffset3D
}
impl VkImageBlit
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkBufferImageCopy
{
	bufferOffset: VkDeviceSize
	bufferRowLength: uint32_t
	bufferImageHeight: uint32_t
	imageSubresource: VkImageSubresourceLayers
	imageOffset: VkOffset3D
	imageExtent: VkExtent3D
}
impl VkBufferImageCopy
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkImageResolve
{
	srcSubresource: VkImageSubresourceLayers
	srcOffset: VkOffset3D
	dstSubresource: VkImageSubresourceLayers
	dstOffset: VkOffset3D
	extent: VkExtent3D
}
impl VkImageResolve
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkShaderModuleCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkShaderModuleCreateFlags
	codeSize: size_t
	pCode: uint32_t
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

pub struct VkDescriptorSetLayoutBinding
{
	binding: uint32_t
	descriptorType: VkDescriptorType
	descriptorCount: uint32_t
	stageFlags: VkShaderStageFlags
	pImmutableSamplers: VkSampler
}
impl VkDescriptorSetLayoutBinding
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDescriptorSetLayoutCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkDescriptorSetLayoutCreateFlags
	bindingCount: uint32_t
	pBindings: VkDescriptorSetLayoutBinding
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

pub struct VkDescriptorPoolSize
{
	type: VkDescriptorType
	descriptorCount: uint32_t
}
impl VkDescriptorPoolSize
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDescriptorPoolCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkDescriptorPoolCreateFlags
	maxSets: uint32_t
	poolSizeCount: uint32_t
	pPoolSizes: VkDescriptorPoolSize
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

pub struct VkDescriptorSetAllocateInfo
{
	sType: VkStructureType
	pNext: void
	descriptorPool: VkDescriptorPool
	descriptorSetCount: uint32_t
	pSetLayouts: VkDescriptorSetLayout
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

pub struct VkSpecializationMapEntry
{
	constantID: uint32_t
	offset: uint32_t
	size: size_t
}
impl VkSpecializationMapEntry
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSpecializationInfo
{
	mapEntryCount: uint32_t
	pMapEntries: VkSpecializationMapEntry
	dataSize: size_t
	pData: void
}
impl VkSpecializationInfo
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineShaderStageCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineShaderStageCreateFlags
	stage: VkShaderStageFlagBits
	module: VkShaderModule
	pName: char
	pSpecializationInfo: VkSpecializationInfo
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

pub struct VkComputePipelineCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineCreateFlags
	stage: VkPipelineShaderStageCreateInfo
	layout: VkPipelineLayout
	basePipelineHandle: VkPipeline
	basePipelineIndex: int32_t
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

pub struct VkVertexInputBindingDescription
{
	binding: uint32_t
	stride: uint32_t
	inputRate: VkVertexInputRate
}
impl VkVertexInputBindingDescription
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkVertexInputAttributeDescription
{
	location: uint32_t
	binding: uint32_t
	format: VkFormat
	offset: uint32_t
}
impl VkVertexInputAttributeDescription
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineVertexInputStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineVertexInputStateCreateFlags
	vertexBindingDescriptionCount: uint32_t
	pVertexBindingDescriptions: VkVertexInputBindingDescription
	vertexAttributeDescriptionCount: uint32_t
	pVertexAttributeDescriptions: VkVertexInputAttributeDescription
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

pub struct VkPipelineInputAssemblyStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineInputAssemblyStateCreateFlags
	topology: VkPrimitiveTopology
	primitiveRestartEnable: VkBool32
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

pub struct VkPipelineTessellationStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineTessellationStateCreateFlags
	patchControlPoints: uint32_t
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

pub struct VkPipelineViewportStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineViewportStateCreateFlags
	viewportCount: uint32_t
	pViewports: VkViewport
	scissorCount: uint32_t
	pScissors: VkRect2D
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

pub struct VkPipelineRasterizationStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineRasterizationStateCreateFlags
	depthClampEnable: VkBool32
	rasterizerDiscardEnable: VkBool32
	polygonMode: VkPolygonMode
	cullMode: VkCullModeFlags
	frontFace: VkFrontFace
	depthBiasEnable: VkBool32
	depthBiasConstantFactor: float
	depthBiasClamp: float
	depthBiasSlopeFactor: float
	lineWidth: float
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

pub struct VkPipelineMultisampleStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineMultisampleStateCreateFlags
	rasterizationSamples: VkSampleCountFlagBits
	sampleShadingEnable: VkBool32
	minSampleShading: float
	pSampleMask: VkSampleMask
	alphaToCoverageEnable: VkBool32
	alphaToOneEnable: VkBool32
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

pub struct VkPipelineColorBlendAttachmentState
{
	blendEnable: VkBool32
	srcColorBlendFactor: VkBlendFactor
	dstColorBlendFactor: VkBlendFactor
	colorBlendOp: VkBlendOp
	srcAlphaBlendFactor: VkBlendFactor
	dstAlphaBlendFactor: VkBlendFactor
	alphaBlendOp: VkBlendOp
	colorWriteMask: VkColorComponentFlags
}
impl VkPipelineColorBlendAttachmentState
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineColorBlendStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineColorBlendStateCreateFlags
	logicOpEnable: VkBool32
	logicOp: VkLogicOp
	attachmentCount: uint32_t
	pAttachments: VkPipelineColorBlendAttachmentState
	blendConstants: float
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

pub struct VkPipelineDynamicStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineDynamicStateCreateFlags
	dynamicStateCount: uint32_t
	pDynamicStates: VkDynamicState
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

pub struct VkStencilOpState
{
	failOp: VkStencilOp
	passOp: VkStencilOp
	depthFailOp: VkStencilOp
	compareOp: VkCompareOp
	compareMask: uint32_t
	writeMask: uint32_t
	reference: uint32_t
}
impl VkStencilOpState
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineDepthStencilStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineDepthStencilStateCreateFlags
	depthTestEnable: VkBool32
	depthWriteEnable: VkBool32
	depthCompareOp: VkCompareOp
	depthBoundsTestEnable: VkBool32
	stencilTestEnable: VkBool32
	front: VkStencilOpState
	back: VkStencilOpState
	minDepthBounds: float
	maxDepthBounds: float
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

pub struct VkGraphicsPipelineCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineCreateFlags
	stageCount: uint32_t
	pStages: VkPipelineShaderStageCreateInfo
	pVertexInputState: VkPipelineVertexInputStateCreateInfo
	pInputAssemblyState: VkPipelineInputAssemblyStateCreateInfo
	pTessellationState: VkPipelineTessellationStateCreateInfo
	pViewportState: VkPipelineViewportStateCreateInfo
	pRasterizationState: VkPipelineRasterizationStateCreateInfo
	pMultisampleState: VkPipelineMultisampleStateCreateInfo
	pDepthStencilState: VkPipelineDepthStencilStateCreateInfo
	pColorBlendState: VkPipelineColorBlendStateCreateInfo
	pDynamicState: VkPipelineDynamicStateCreateInfo
	layout: VkPipelineLayout
	renderPass: VkRenderPass
	subpass: uint32_t
	basePipelineHandle: VkPipeline
	basePipelineIndex: int32_t
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

pub struct VkPipelineCacheCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineCacheCreateFlags
	initialDataSize: size_t
	pInitialData: void
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

pub struct VkPipelineCacheHeaderVersionOne
{
	headerSize: uint32_t
	headerVersion: VkPipelineCacheHeaderVersion
	vendorID: uint32_t
	deviceID: uint32_t
	pipelineCacheUUID: uint8_t
}
impl VkPipelineCacheHeaderVersionOne
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPushConstantRange
{
	stageFlags: VkShaderStageFlags
	offset: uint32_t
	size: uint32_t
}
impl VkPushConstantRange
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineLayoutCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineLayoutCreateFlags
	setLayoutCount: uint32_t
	pSetLayouts: VkDescriptorSetLayout
	pushConstantRangeCount: uint32_t
	pPushConstantRanges: VkPushConstantRange
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

pub struct VkSamplerCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkSamplerCreateFlags
	magFilter: VkFilter
	minFilter: VkFilter
	mipmapMode: VkSamplerMipmapMode
	addressModeU: VkSamplerAddressMode
	addressModeV: VkSamplerAddressMode
	addressModeW: VkSamplerAddressMode
	mipLodBias: float
	anisotropyEnable: VkBool32
	maxAnisotropy: float
	compareEnable: VkBool32
	compareOp: VkCompareOp
	minLod: float
	maxLod: float
	borderColor: VkBorderColor
	unnormalizedCoordinates: VkBool32
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

pub struct VkCommandPoolCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkCommandPoolCreateFlags
	queueFamilyIndex: uint32_t
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

pub struct VkCommandBufferAllocateInfo
{
	sType: VkStructureType
	pNext: void
	commandPool: VkCommandPool
	level: VkCommandBufferLevel
	commandBufferCount: uint32_t
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

pub struct VkCommandBufferInheritanceInfo
{
	sType: VkStructureType
	pNext: void
	renderPass: VkRenderPass
	subpass: uint32_t
	framebuffer: VkFramebuffer
	occlusionQueryEnable: VkBool32
	queryFlags: VkQueryControlFlags
	pipelineStatistics: VkQueryPipelineStatisticFlags
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

pub struct VkCommandBufferBeginInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkCommandBufferUsageFlags
	pInheritanceInfo: VkCommandBufferInheritanceInfo
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

pub struct VkRenderPassBeginInfo
{
	sType: VkStructureType
	pNext: void
	renderPass: VkRenderPass
	framebuffer: VkFramebuffer
	renderArea: VkRect2D
	clearValueCount: uint32_t
	pClearValues: VkClearValue
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

pub struct VkClearDepthStencilValue
{
	depth: float
	stencil: uint32_t
}
impl VkClearDepthStencilValue
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkClearAttachment
{
	aspectMask: VkImageAspectFlags
	colorAttachment: uint32_t
	clearValue: VkClearValue
}
impl VkClearAttachment
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkAttachmentDescription
{
	flags: VkAttachmentDescriptionFlags
	format: VkFormat
	samples: VkSampleCountFlagBits
	loadOp: VkAttachmentLoadOp
	storeOp: VkAttachmentStoreOp
	stencilLoadOp: VkAttachmentLoadOp
	stencilStoreOp: VkAttachmentStoreOp
	initialLayout: VkImageLayout
	finalLayout: VkImageLayout
}
impl VkAttachmentDescription
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkAttachmentReference
{
	attachment: uint32_t
	layout: VkImageLayout
}
impl VkAttachmentReference
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSubpassDescription
{
	flags: VkSubpassDescriptionFlags
	pipelineBindPoint: VkPipelineBindPoint
	inputAttachmentCount: uint32_t
	pInputAttachments: VkAttachmentReference
	colorAttachmentCount: uint32_t
	pColorAttachments: VkAttachmentReference
	pResolveAttachments: VkAttachmentReference
	pDepthStencilAttachment: VkAttachmentReference
	preserveAttachmentCount: uint32_t
	pPreserveAttachments: uint32_t
}
impl VkSubpassDescription
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSubpassDependency
{
	srcSubpass: uint32_t
	dstSubpass: uint32_t
	srcStageMask: VkPipelineStageFlags
	dstStageMask: VkPipelineStageFlags
	srcAccessMask: VkAccessFlags
	dstAccessMask: VkAccessFlags
	dependencyFlags: VkDependencyFlags
}
impl VkSubpassDependency
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkRenderPassCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkRenderPassCreateFlags
	attachmentCount: uint32_t
	pAttachments: VkAttachmentDescription
	subpassCount: uint32_t
	pSubpasses: VkSubpassDescription
	dependencyCount: uint32_t
	pDependencies: VkSubpassDependency
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

pub struct VkEventCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkEventCreateFlags
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

pub struct VkFenceCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkFenceCreateFlags
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

pub struct VkPhysicalDeviceFeatures
{
	robustBufferAccess: VkBool32
	fullDrawIndexUint32: VkBool32
	imageCubeArray: VkBool32
	independentBlend: VkBool32
	geometryShader: VkBool32
	tessellationShader: VkBool32
	sampleRateShading: VkBool32
	dualSrcBlend: VkBool32
	logicOp: VkBool32
	multiDrawIndirect: VkBool32
	drawIndirectFirstInstance: VkBool32
	depthClamp: VkBool32
	depthBiasClamp: VkBool32
	fillModeNonSolid: VkBool32
	depthBounds: VkBool32
	wideLines: VkBool32
	largePoints: VkBool32
	alphaToOne: VkBool32
	multiViewport: VkBool32
	samplerAnisotropy: VkBool32
	textureCompressionETC2: VkBool32
	textureCompressionASTC_LDR: VkBool32
	textureCompressionBC: VkBool32
	occlusionQueryPrecise: VkBool32
	pipelineStatisticsQuery: VkBool32
	vertexPipelineStoresAndAtomics: VkBool32
	fragmentStoresAndAtomics: VkBool32
	shaderTessellationAndGeometryPointSize: VkBool32
	shaderImageGatherExtended: VkBool32
	shaderStorageImageExtendedFormats: VkBool32
	shaderStorageImageMultisample: VkBool32
	shaderStorageImageReadWithoutFormat: VkBool32
	shaderStorageImageWriteWithoutFormat: VkBool32
	shaderUniformBufferArrayDynamicIndexing: VkBool32
	shaderSampledImageArrayDynamicIndexing: VkBool32
	shaderStorageBufferArrayDynamicIndexing: VkBool32
	shaderStorageImageArrayDynamicIndexing: VkBool32
	shaderClipDistance: VkBool32
	shaderCullDistance: VkBool32
	shaderFloat64: VkBool32
	shaderInt64: VkBool32
	shaderInt16: VkBool32
	shaderResourceResidency: VkBool32
	shaderResourceMinLod: VkBool32
	sparseBinding: VkBool32
	sparseResidencyBuffer: VkBool32
	sparseResidencyImage2D: VkBool32
	sparseResidencyImage3D: VkBool32
	sparseResidency2Samples: VkBool32
	sparseResidency4Samples: VkBool32
	sparseResidency8Samples: VkBool32
	sparseResidency16Samples: VkBool32
	sparseResidencyAliased: VkBool32
	variableMultisampleRate: VkBool32
	inheritedQueries: VkBool32
}
impl VkPhysicalDeviceFeatures
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPhysicalDeviceSparseProperties
{
	residencyStandard2DBlockShape: VkBool32
	residencyStandard2DMultisampleBlockShape: VkBool32
	residencyStandard3DBlockShape: VkBool32
	residencyAlignedMipSize: VkBool32
	residencyNonResidentStrict: VkBool32
}
impl VkPhysicalDeviceSparseProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPhysicalDeviceLimits
{
	maxImageDimension1D: uint32_t
	maxImageDimension2D: uint32_t
	maxImageDimension3D: uint32_t
	maxImageDimensionCube: uint32_t
	maxImageArrayLayers: uint32_t
	maxTexelBufferElements: uint32_t
	maxUniformBufferRange: uint32_t
	maxStorageBufferRange: uint32_t
	maxPushConstantsSize: uint32_t
	maxMemoryAllocationCount: uint32_t
	maxSamplerAllocationCount: uint32_t
	bufferImageGranularity: VkDeviceSize
	sparseAddressSpaceSize: VkDeviceSize
	maxBoundDescriptorSets: uint32_t
	maxPerStageDescriptorSamplers: uint32_t
	maxPerStageDescriptorUniformBuffers: uint32_t
	maxPerStageDescriptorStorageBuffers: uint32_t
	maxPerStageDescriptorSampledImages: uint32_t
	maxPerStageDescriptorStorageImages: uint32_t
	maxPerStageDescriptorInputAttachments: uint32_t
	maxPerStageResources: uint32_t
	maxDescriptorSetSamplers: uint32_t
	maxDescriptorSetUniformBuffers: uint32_t
	maxDescriptorSetUniformBuffersDynamic: uint32_t
	maxDescriptorSetStorageBuffers: uint32_t
	maxDescriptorSetStorageBuffersDynamic: uint32_t
	maxDescriptorSetSampledImages: uint32_t
	maxDescriptorSetStorageImages: uint32_t
	maxDescriptorSetInputAttachments: uint32_t
	maxVertexInputAttributes: uint32_t
	maxVertexInputBindings: uint32_t
	maxVertexInputAttributeOffset: uint32_t
	maxVertexInputBindingStride: uint32_t
	maxVertexOutputComponents: uint32_t
	maxTessellationGenerationLevel: uint32_t
	maxTessellationPatchSize: uint32_t
	maxTessellationControlPerVertexInputComponents: uint32_t
	maxTessellationControlPerVertexOutputComponents: uint32_t
	maxTessellationControlPerPatchOutputComponents: uint32_t
	maxTessellationControlTotalOutputComponents: uint32_t
	maxTessellationEvaluationInputComponents: uint32_t
	maxTessellationEvaluationOutputComponents: uint32_t
	maxGeometryShaderInvocations: uint32_t
	maxGeometryInputComponents: uint32_t
	maxGeometryOutputComponents: uint32_t
	maxGeometryOutputVertices: uint32_t
	maxGeometryTotalOutputComponents: uint32_t
	maxFragmentInputComponents: uint32_t
	maxFragmentOutputAttachments: uint32_t
	maxFragmentDualSrcAttachments: uint32_t
	maxFragmentCombinedOutputResources: uint32_t
	maxComputeSharedMemorySize: uint32_t
	maxComputeWorkGroupCount: uint32_t
	maxComputeWorkGroupInvocations: uint32_t
	maxComputeWorkGroupSize: uint32_t
	subPixelPrecisionBits: uint32_t
	subTexelPrecisionBits: uint32_t
	mipmapPrecisionBits: uint32_t
	maxDrawIndexedIndexValue: uint32_t
	maxDrawIndirectCount: uint32_t
	maxSamplerLodBias: float
	maxSamplerAnisotropy: float
	maxViewports: uint32_t
	maxViewportDimensions: uint32_t
	viewportBoundsRange: float
	viewportSubPixelBits: uint32_t
	minMemoryMapAlignment: size_t
	minTexelBufferOffsetAlignment: VkDeviceSize
	minUniformBufferOffsetAlignment: VkDeviceSize
	minStorageBufferOffsetAlignment: VkDeviceSize
	minTexelOffset: int32_t
	maxTexelOffset: uint32_t
	minTexelGatherOffset: int32_t
	maxTexelGatherOffset: uint32_t
	minInterpolationOffset: float
	maxInterpolationOffset: float
	subPixelInterpolationOffsetBits: uint32_t
	maxFramebufferWidth: uint32_t
	maxFramebufferHeight: uint32_t
	maxFramebufferLayers: uint32_t
	framebufferColorSampleCounts: VkSampleCountFlags
	framebufferDepthSampleCounts: VkSampleCountFlags
	framebufferStencilSampleCounts: VkSampleCountFlags
	framebufferNoAttachmentsSampleCounts: VkSampleCountFlags
	maxColorAttachments: uint32_t
	sampledImageColorSampleCounts: VkSampleCountFlags
	sampledImageIntegerSampleCounts: VkSampleCountFlags
	sampledImageDepthSampleCounts: VkSampleCountFlags
	sampledImageStencilSampleCounts: VkSampleCountFlags
	storageImageSampleCounts: VkSampleCountFlags
	maxSampleMaskWords: uint32_t
	timestampComputeAndGraphics: VkBool32
	timestampPeriod: float
	maxClipDistances: uint32_t
	maxCullDistances: uint32_t
	maxCombinedClipAndCullDistances: uint32_t
	discreteQueuePriorities: uint32_t
	pointSizeRange: float
	lineWidthRange: float
	pointSizeGranularity: float
	lineWidthGranularity: float
	strictLines: VkBool32
	standardSampleLocations: VkBool32
	optimalBufferCopyOffsetAlignment: VkDeviceSize
	optimalBufferCopyRowPitchAlignment: VkDeviceSize
	nonCoherentAtomSize: VkDeviceSize
}
impl VkPhysicalDeviceLimits
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSemaphoreCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkSemaphoreCreateFlags
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

pub struct VkQueryPoolCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkQueryPoolCreateFlags
	queryType: VkQueryType
	queryCount: uint32_t
	pipelineStatistics: VkQueryPipelineStatisticFlags
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

pub struct VkFramebufferCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkFramebufferCreateFlags
	renderPass: VkRenderPass
	attachmentCount: uint32_t
	pAttachments: VkImageView
	width: uint32_t
	height: uint32_t
	layers: uint32_t
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

pub struct VkDrawIndirectCommand
{
	vertexCount: uint32_t
	instanceCount: uint32_t
	firstVertex: uint32_t
	firstInstance: uint32_t
}
impl VkDrawIndirectCommand
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDrawIndexedIndirectCommand
{
	indexCount: uint32_t
	instanceCount: uint32_t
	firstIndex: uint32_t
	vertexOffset: int32_t
	firstInstance: uint32_t
}
impl VkDrawIndexedIndirectCommand
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDispatchIndirectCommand
{
	x: uint32_t
	y: uint32_t
	z: uint32_t
}
impl VkDispatchIndirectCommand
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkMultiDrawInfoEXT
{
	firstVertex: uint32_t
	vertexCount: uint32_t
}
impl VkMultiDrawInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkMultiDrawIndexedInfoEXT
{
	firstIndex: uint32_t
	indexCount: uint32_t
	vertexOffset: int32_t
}
impl VkMultiDrawIndexedInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSubmitInfo
{
	sType: VkStructureType
	pNext: void
	waitSemaphoreCount: uint32_t
	pWaitSemaphores: VkSemaphore
	pWaitDstStageMask: VkPipelineStageFlags
	commandBufferCount: uint32_t
	pCommandBuffers: VkCommandBuffer
	signalSemaphoreCount: uint32_t
	pSignalSemaphores: VkSemaphore
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

pub struct VkDisplayPropertiesKHR
{
	display: VkDisplayKHR
	displayName: char
	physicalDimensions: VkExtent2D
	physicalResolution: VkExtent2D
	supportedTransforms: VkSurfaceTransformFlagsKHR
	planeReorderPossible: VkBool32
	persistentContent: VkBool32
}
impl VkDisplayPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDisplayPlanePropertiesKHR
{
	currentDisplay: VkDisplayKHR
	currentStackIndex: uint32_t
}
impl VkDisplayPlanePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDisplayModeParametersKHR
{
	visibleRegion: VkExtent2D
	refreshRate: uint32_t
}
impl VkDisplayModeParametersKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDisplayModePropertiesKHR
{
	displayMode: VkDisplayModeKHR
	parameters: VkDisplayModeParametersKHR
}
impl VkDisplayModePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDisplayModeCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkDisplayModeCreateFlagsKHR
	parameters: VkDisplayModeParametersKHR
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

pub struct VkDisplayPlaneCapabilitiesKHR
{
	supportedAlpha: VkDisplayPlaneAlphaFlagsKHR
	minSrcPosition: VkOffset2D
	maxSrcPosition: VkOffset2D
	minSrcExtent: VkExtent2D
	maxSrcExtent: VkExtent2D
	minDstPosition: VkOffset2D
	maxDstPosition: VkOffset2D
	minDstExtent: VkExtent2D
	maxDstExtent: VkExtent2D
}
impl VkDisplayPlaneCapabilitiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDisplaySurfaceCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkDisplaySurfaceCreateFlagsKHR
	displayMode: VkDisplayModeKHR
	planeIndex: uint32_t
	planeStackIndex: uint32_t
	transform: VkSurfaceTransformFlagBitsKHR
	globalAlpha: float
	alphaMode: VkDisplayPlaneAlphaFlagBitsKHR
	imageExtent: VkExtent2D
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

pub struct VkDisplayPresentInfoKHR
{
	sType: VkStructureType
	pNext: void
	srcRect: VkRect2D
	dstRect: VkRect2D
	persistent: VkBool32
}
impl VkDisplayPresentInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR;

        return s;
    }
}

pub struct VkSurfaceCapabilitiesKHR
{
	minImageCount: uint32_t
	maxImageCount: uint32_t
	currentExtent: VkExtent2D
	minImageExtent: VkExtent2D
	maxImageExtent: VkExtent2D
	maxImageArrayLayers: uint32_t
	supportedTransforms: VkSurfaceTransformFlagsKHR
	currentTransform: VkSurfaceTransformFlagBitsKHR
	supportedCompositeAlpha: VkCompositeAlphaFlagsKHR
	supportedUsageFlags: VkImageUsageFlags
}
impl VkSurfaceCapabilitiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkAndroidSurfaceCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkAndroidSurfaceCreateFlagsKHR
	window: ANativeWindow
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

pub struct VkViSurfaceCreateInfoNN
{
	sType: VkStructureType
	pNext: void
	flags: VkViSurfaceCreateFlagsNN
	window: void
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

pub struct VkWaylandSurfaceCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkWaylandSurfaceCreateFlagsKHR
	display: wl_display
	surface: wl_surface
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

pub struct VkWin32SurfaceCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkWin32SurfaceCreateFlagsKHR
	hinstance: HINSTANCE
	hwnd: HWND
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

pub struct VkXlibSurfaceCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkXlibSurfaceCreateFlagsKHR
	dpy: Display
	window: Window
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

pub struct VkXcbSurfaceCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkXcbSurfaceCreateFlagsKHR
	connection: xcb_connection_t
	window: xcb_window_t
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

pub struct VkDirectFBSurfaceCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkDirectFBSurfaceCreateFlagsEXT
	dfb: IDirectFB
	surface: IDirectFBSurface
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

pub struct VkImagePipeSurfaceCreateInfoFUCHSIA
{
	sType: VkStructureType
	pNext: void
	flags: VkImagePipeSurfaceCreateFlagsFUCHSIA
	imagePipeHandle: zx_handle_t
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

pub struct VkStreamDescriptorSurfaceCreateInfoGGP
{
	sType: VkStructureType
	pNext: void
	flags: VkStreamDescriptorSurfaceCreateFlagsGGP
	streamDescriptor: GgpStreamDescriptor
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

pub struct VkScreenSurfaceCreateInfoQNX
{
	sType: VkStructureType
	pNext: void
	flags: VkScreenSurfaceCreateFlagsQNX
	context: _screen_context
	window: _screen_window
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

pub struct VkSurfaceFormatKHR
{
	format: VkFormat
	colorSpace: VkColorSpaceKHR
}
impl VkSurfaceFormatKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSwapchainCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkSwapchainCreateFlagsKHR
	surface: VkSurfaceKHR
	minImageCount: uint32_t
	imageFormat: VkFormat
	imageColorSpace: VkColorSpaceKHR
	imageExtent: VkExtent2D
	imageArrayLayers: uint32_t
	imageUsage: VkImageUsageFlags
	imageSharingMode: VkSharingMode
	queueFamilyIndexCount: uint32_t
	pQueueFamilyIndices: uint32_t
	preTransform: VkSurfaceTransformFlagBitsKHR
	compositeAlpha: VkCompositeAlphaFlagBitsKHR
	presentMode: VkPresentModeKHR
	clipped: VkBool32
	oldSwapchain: VkSwapchainKHR
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

pub struct VkPresentInfoKHR
{
	sType: VkStructureType
	pNext: void
	waitSemaphoreCount: uint32_t
	pWaitSemaphores: VkSemaphore
	swapchainCount: uint32_t
	pSwapchains: VkSwapchainKHR
	pImageIndices: uint32_t
	pResults: VkResult
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

pub struct VkDebugReportCallbackCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkDebugReportFlagsEXT
	pfnCallback: PFN_vkDebugReportCallbackEXT
	pUserData: void
}
impl VkDebugReportCallbackCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkValidationFlagsEXT
{
	sType: VkStructureType
	pNext: void
	disabledValidationCheckCount: uint32_t
	pDisabledValidationChecks: VkValidationCheckEXT
}
impl VkValidationFlagsEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT;

        return s;
    }
}

pub struct VkValidationFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	enabledValidationFeatureCount: uint32_t
	pEnabledValidationFeatures: VkValidationFeatureEnableEXT
	disabledValidationFeatureCount: uint32_t
	pDisabledValidationFeatures: VkValidationFeatureDisableEXT
}
impl VkValidationFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT;

        return s;
    }
}

pub struct VkPipelineRasterizationStateRasterizationOrderAMD
{
	sType: VkStructureType
	pNext: void
	rasterizationOrder: VkRasterizationOrderAMD
}
impl VkPipelineRasterizationStateRasterizationOrderAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD;

        return s;
    }
}

pub struct VkDebugMarkerObjectNameInfoEXT
{
	sType: VkStructureType
	pNext: void
	objectType: VkDebugReportObjectTypeEXT
	object: uint64_t
	pObjectName: char
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

pub struct VkDebugMarkerObjectTagInfoEXT
{
	sType: VkStructureType
	pNext: void
	objectType: VkDebugReportObjectTypeEXT
	object: uint64_t
	tagName: uint64_t
	tagSize: size_t
	pTag: void
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

pub struct VkDebugMarkerMarkerInfoEXT
{
	sType: VkStructureType
	pNext: void
	pMarkerName: char
	color: float
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

pub struct VkDedicatedAllocationImageCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	dedicatedAllocation: VkBool32
}
impl VkDedicatedAllocationImageCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkDedicatedAllocationBufferCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	dedicatedAllocation: VkBool32
}
impl VkDedicatedAllocationBufferCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkDedicatedAllocationMemoryAllocateInfoNV
{
	sType: VkStructureType
	pNext: void
	image: VkImage
	buffer: VkBuffer
}
impl VkDedicatedAllocationMemoryAllocateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV;

        return s;
    }
}

pub struct VkExternalImageFormatPropertiesNV
{
	imageFormatProperties: VkImageFormatProperties
	externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV
	exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV
	compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV
}
impl VkExternalImageFormatPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkExternalMemoryImageCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	handleTypes: VkExternalMemoryHandleTypeFlagsNV
}
impl VkExternalMemoryImageCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkExportMemoryAllocateInfoNV
{
	sType: VkStructureType
	pNext: void
	handleTypes: VkExternalMemoryHandleTypeFlagsNV
}
impl VkExportMemoryAllocateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV;

        return s;
    }
}

pub struct VkImportMemoryWin32HandleInfoNV
{
	sType: VkStructureType
	pNext: void
	handleType: VkExternalMemoryHandleTypeFlagsNV
	handle: HANDLE
}
impl VkImportMemoryWin32HandleInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV;

        return s;
    }
}

pub struct VkExportMemoryWin32HandleInfoNV
{
	sType: VkStructureType
	pNext: void
	pAttributes: SECURITY_ATTRIBUTES
	dwAccess: DWORD
}
impl VkExportMemoryWin32HandleInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV;

        return s;
    }
}

pub struct VkWin32KeyedMutexAcquireReleaseInfoNV
{
	sType: VkStructureType
	pNext: void
	acquireCount: uint32_t
	pAcquireSyncs: VkDeviceMemory
	pAcquireKeys: uint64_t
	pAcquireTimeoutMilliseconds: uint32_t
	releaseCount: uint32_t
	pReleaseSyncs: VkDeviceMemory
	pReleaseKeys: uint64_t
}
impl VkWin32KeyedMutexAcquireReleaseInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV
{
	sType: VkStructureType
	pNext: void
	deviceGeneratedCommands: VkBool32
}
impl VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV;

        return s;
    }
}

pub struct VkDevicePrivateDataCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	privateDataSlotRequestCount: uint32_t
}
impl VkDevicePrivateDataCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPrivateDataSlotCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkPrivateDataSlotCreateFlagsEXT
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

pub struct VkPhysicalDevicePrivateDataFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	privateData: VkBool32
}
impl VkPhysicalDevicePrivateDataFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV
{
	sType: VkStructureType
	pNext: void
	maxGraphicsShaderGroupCount: uint32_t
	maxIndirectSequenceCount: uint32_t
	maxIndirectCommandsTokenCount: uint32_t
	maxIndirectCommandsStreamCount: uint32_t
	maxIndirectCommandsTokenOffset: uint32_t
	maxIndirectCommandsStreamStride: uint32_t
	minSequencesCountBufferOffsetAlignment: uint32_t
	minSequencesIndexBufferOffsetAlignment: uint32_t
	minIndirectCommandsBufferOffsetAlignment: uint32_t
}
impl VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceMultiDrawPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	maxMultiDrawCount: uint32_t
}
impl VkPhysicalDeviceMultiDrawPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkGraphicsShaderGroupCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	stageCount: uint32_t
	pStages: VkPipelineShaderStageCreateInfo
	pVertexInputState: VkPipelineVertexInputStateCreateInfo
	pTessellationState: VkPipelineTessellationStateCreateInfo
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

pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	groupCount: uint32_t
	pGroups: VkGraphicsShaderGroupCreateInfoNV
	pipelineCount: uint32_t
	pPipelines: VkPipeline
}
impl VkGraphicsPipelineShaderGroupsCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkBindShaderGroupIndirectCommandNV
{
	groupIndex: uint32_t
}
impl VkBindShaderGroupIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkBindIndexBufferIndirectCommandNV
{
	bufferAddress: VkDeviceAddress
	size: uint32_t
	indexType: VkIndexType
}
impl VkBindIndexBufferIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkBindVertexBufferIndirectCommandNV
{
	bufferAddress: VkDeviceAddress
	size: uint32_t
	stride: uint32_t
}
impl VkBindVertexBufferIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSetStateFlagsIndirectCommandNV
{
	data: uint32_t
}
impl VkSetStateFlagsIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkIndirectCommandsStreamNV
{
	buffer: VkBuffer
	offset: VkDeviceSize
}
impl VkIndirectCommandsStreamNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkIndirectCommandsLayoutTokenNV
{
	sType: VkStructureType
	pNext: void
	tokenType: VkIndirectCommandsTokenTypeNV
	stream: uint32_t
	offset: uint32_t
	vertexBindingUnit: uint32_t
	vertexDynamicStride: VkBool32
	pushconstantPipelineLayout: VkPipelineLayout
	pushconstantShaderStageFlags: VkShaderStageFlags
	pushconstantOffset: uint32_t
	pushconstantSize: uint32_t
	indirectStateFlags: VkIndirectStateFlagsNV
	indexTypeCount: uint32_t
	pIndexTypes: VkIndexType
	pIndexTypeValues: uint32_t
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

pub struct VkIndirectCommandsLayoutCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	flags: VkIndirectCommandsLayoutUsageFlagsNV
	pipelineBindPoint: VkPipelineBindPoint
	tokenCount: uint32_t
	pTokens: VkIndirectCommandsLayoutTokenNV
	streamCount: uint32_t
	pStreamStrides: uint32_t
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

pub struct VkGeneratedCommandsInfoNV
{
	sType: VkStructureType
	pNext: void
	pipelineBindPoint: VkPipelineBindPoint
	pipeline: VkPipeline
	indirectCommandsLayout: VkIndirectCommandsLayoutNV
	streamCount: uint32_t
	pStreams: VkIndirectCommandsStreamNV
	sequencesCount: uint32_t
	preprocessBuffer: VkBuffer
	preprocessOffset: VkDeviceSize
	preprocessSize: VkDeviceSize
	sequencesCountBuffer: VkBuffer
	sequencesCountOffset: VkDeviceSize
	sequencesIndexBuffer: VkBuffer
	sequencesIndexOffset: VkDeviceSize
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

pub struct VkGeneratedCommandsMemoryRequirementsInfoNV
{
	sType: VkStructureType
	pNext: void
	pipelineBindPoint: VkPipelineBindPoint
	pipeline: VkPipeline
	indirectCommandsLayout: VkIndirectCommandsLayoutNV
	maxSequencesCount: uint32_t
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

pub struct VkPhysicalDeviceFeatures2
{
	sType: VkStructureType
	pNext: void
	features: VkPhysicalDeviceFeatures
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

pub struct VkPhysicalDeviceProperties2
{
	sType: VkStructureType
	pNext: void
	properties: VkPhysicalDeviceProperties
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

pub struct VkFormatProperties2
{
	sType: VkStructureType
	pNext: void
	formatProperties: VkFormatProperties
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

pub struct VkImageFormatProperties2
{
	sType: VkStructureType
	pNext: void
	imageFormatProperties: VkImageFormatProperties
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

pub struct VkPhysicalDeviceImageFormatInfo2
{
	sType: VkStructureType
	pNext: void
	format: VkFormat
	type: VkImageType
	tiling: VkImageTiling
	usage: VkImageUsageFlags
	flags: VkImageCreateFlags
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

pub struct VkQueueFamilyProperties2
{
	sType: VkStructureType
	pNext: void
	queueFamilyProperties: VkQueueFamilyProperties
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

pub struct VkPhysicalDeviceMemoryProperties2
{
	sType: VkStructureType
	pNext: void
	memoryProperties: VkPhysicalDeviceMemoryProperties
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

pub struct VkSparseImageFormatProperties2
{
	sType: VkStructureType
	pNext: void
	properties: VkSparseImageFormatProperties
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

pub struct VkPhysicalDeviceSparseImageFormatInfo2
{
	sType: VkStructureType
	pNext: void
	format: VkFormat
	type: VkImageType
	samples: VkSampleCountFlagBits
	usage: VkImageUsageFlags
	tiling: VkImageTiling
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

pub struct VkPhysicalDevicePushDescriptorPropertiesKHR
{
	sType: VkStructureType
	pNext: void
	maxPushDescriptors: uint32_t
}
impl VkPhysicalDevicePushDescriptorPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR;

        return s;
    }
}

pub struct VkConformanceVersion
{
	major: uint8_t
	minor: uint8_t
	subminor: uint8_t
	patch: uint8_t
}
impl VkConformanceVersion
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

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

pub struct VkPhysicalDeviceDriverProperties
{
	sType: VkStructureType
	pNext: void
	driverID: VkDriverId
	driverName: char
	driverInfo: char
	conformanceVersion: VkConformanceVersion
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

pub struct VkPresentRegionsKHR
{
	sType: VkStructureType
	pNext: void
	swapchainCount: uint32_t
	pRegions: VkPresentRegionKHR
}
impl VkPresentRegionsKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR;

        return s;
    }
}

pub struct VkPresentRegionKHR
{
	rectangleCount: uint32_t
	pRectangles: VkRectLayerKHR
}
impl VkPresentRegionKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkRectLayerKHR
{
	offset: VkOffset2D
	extent: VkExtent2D
	layer: uint32_t
}
impl VkRectLayerKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPhysicalDeviceVariablePointersFeatures
{
	sType: VkStructureType
	pNext: void
	variablePointersStorageBuffer: VkBool32
	variablePointers: VkBool32
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

pub struct VkExternalMemoryProperties
{
	externalMemoryFeatures: VkExternalMemoryFeatureFlags
	exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags
	compatibleHandleTypes: VkExternalMemoryHandleTypeFlags
}
impl VkExternalMemoryProperties
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

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

pub struct VkPhysicalDeviceExternalImageFormatInfo
{
	sType: VkStructureType
	pNext: void
	handleType: VkExternalMemoryHandleTypeFlagBits
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

pub struct VkExternalImageFormatProperties
{
	sType: VkStructureType
	pNext: void
	externalMemoryProperties: VkExternalMemoryProperties
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

pub struct VkPhysicalDeviceExternalBufferInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkBufferCreateFlags
	usage: VkBufferUsageFlags
	handleType: VkExternalMemoryHandleTypeFlagBits
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

pub struct VkExternalBufferProperties
{
	sType: VkStructureType
	pNext: void
	externalMemoryProperties: VkExternalMemoryProperties
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

pub struct VkPhysicalDeviceIDProperties
{
	sType: VkStructureType
	pNext: void
	deviceUUID: uint8_t
	driverUUID: uint8_t
	deviceLUID: uint8_t
	deviceNodeMask: uint32_t
	deviceLUIDValid: VkBool32
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

pub struct VkExternalMemoryImageCreateInfo
{
	sType: VkStructureType
	pNext: void
	handleTypes: VkExternalMemoryHandleTypeFlags
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

pub struct VkExternalMemoryBufferCreateInfo
{
	sType: VkStructureType
	pNext: void
	handleTypes: VkExternalMemoryHandleTypeFlags
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

pub struct VkExportMemoryAllocateInfo
{
	sType: VkStructureType
	pNext: void
	handleTypes: VkExternalMemoryHandleTypeFlags
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

pub struct VkImportMemoryWin32HandleInfoKHR
{
	sType: VkStructureType
	pNext: void
	handleType: VkExternalMemoryHandleTypeFlagBits
	handle: HANDLE
	name: LPCWSTR
}
impl VkImportMemoryWin32HandleInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR;

        return s;
    }
}

pub struct VkExportMemoryWin32HandleInfoKHR
{
	sType: VkStructureType
	pNext: void
	pAttributes: SECURITY_ATTRIBUTES
	dwAccess: DWORD
	name: LPCWSTR
}
impl VkExportMemoryWin32HandleInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR;

        return s;
    }
}

pub struct VkImportMemoryZirconHandleInfoFUCHSIA
{
	sType: VkStructureType
	pNext: void
	handleType: VkExternalMemoryHandleTypeFlagBits
	handle: zx_handle_t
}
impl VkImportMemoryZirconHandleInfoFUCHSIA
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA;

        return s;
    }
}

pub struct VkMemoryZirconHandlePropertiesFUCHSIA
{
	sType: VkStructureType
	pNext: void
	memoryTypeBits: uint32_t
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

pub struct VkMemoryGetZirconHandleInfoFUCHSIA
{
	sType: VkStructureType
	pNext: void
	memory: VkDeviceMemory
	handleType: VkExternalMemoryHandleTypeFlagBits
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

pub struct VkMemoryWin32HandlePropertiesKHR
{
	sType: VkStructureType
	pNext: void
	memoryTypeBits: uint32_t
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

pub struct VkMemoryGetWin32HandleInfoKHR
{
	sType: VkStructureType
	pNext: void
	memory: VkDeviceMemory
	handleType: VkExternalMemoryHandleTypeFlagBits
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

pub struct VkImportMemoryFdInfoKHR
{
	sType: VkStructureType
	pNext: void
	handleType: VkExternalMemoryHandleTypeFlagBits
	fd: int
}
impl VkImportMemoryFdInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR;

        return s;
    }
}

pub struct VkMemoryFdPropertiesKHR
{
	sType: VkStructureType
	pNext: void
	memoryTypeBits: uint32_t
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

pub struct VkMemoryGetFdInfoKHR
{
	sType: VkStructureType
	pNext: void
	memory: VkDeviceMemory
	handleType: VkExternalMemoryHandleTypeFlagBits
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

pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR
{
	sType: VkStructureType
	pNext: void
	acquireCount: uint32_t
	pAcquireSyncs: VkDeviceMemory
	pAcquireKeys: uint64_t
	pAcquireTimeouts: uint32_t
	releaseCount: uint32_t
	pReleaseSyncs: VkDeviceMemory
	pReleaseKeys: uint64_t
}
impl VkWin32KeyedMutexAcquireReleaseInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceExternalSemaphoreInfo
{
	sType: VkStructureType
	pNext: void
	handleType: VkExternalSemaphoreHandleTypeFlagBits
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

pub struct VkExternalSemaphoreProperties
{
	sType: VkStructureType
	pNext: void
	exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags
	compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags
	externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags
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

pub struct VkExportSemaphoreCreateInfo
{
	sType: VkStructureType
	pNext: void
	handleTypes: VkExternalSemaphoreHandleTypeFlags
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

pub struct VkImportSemaphoreWin32HandleInfoKHR
{
	sType: VkStructureType
	pNext: void
	semaphore: VkSemaphore
	flags: VkSemaphoreImportFlags
	handleType: VkExternalSemaphoreHandleTypeFlagBits
	handle: HANDLE
	name: LPCWSTR
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

pub struct VkExportSemaphoreWin32HandleInfoKHR
{
	sType: VkStructureType
	pNext: void
	pAttributes: SECURITY_ATTRIBUTES
	dwAccess: DWORD
	name: LPCWSTR
}
impl VkExportSemaphoreWin32HandleInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR;

        return s;
    }
}

pub struct VkD3D12FenceSubmitInfoKHR
{
	sType: VkStructureType
	pNext: void
	waitSemaphoreValuesCount: uint32_t
	pWaitSemaphoreValues: uint64_t
	signalSemaphoreValuesCount: uint32_t
	pSignalSemaphoreValues: uint64_t
}
impl VkD3D12FenceSubmitInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR;

        return s;
    }
}

pub struct VkSemaphoreGetWin32HandleInfoKHR
{
	sType: VkStructureType
	pNext: void
	semaphore: VkSemaphore
	handleType: VkExternalSemaphoreHandleTypeFlagBits
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

pub struct VkImportSemaphoreFdInfoKHR
{
	sType: VkStructureType
	pNext: void
	semaphore: VkSemaphore
	flags: VkSemaphoreImportFlags
	handleType: VkExternalSemaphoreHandleTypeFlagBits
	fd: int
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

pub struct VkSemaphoreGetFdInfoKHR
{
	sType: VkStructureType
	pNext: void
	semaphore: VkSemaphore
	handleType: VkExternalSemaphoreHandleTypeFlagBits
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

pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA
{
	sType: VkStructureType
	pNext: void
	semaphore: VkSemaphore
	flags: VkSemaphoreImportFlags
	handleType: VkExternalSemaphoreHandleTypeFlagBits
	zirconHandle: zx_handle_t
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

pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA
{
	sType: VkStructureType
	pNext: void
	semaphore: VkSemaphore
	handleType: VkExternalSemaphoreHandleTypeFlagBits
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

pub struct VkPhysicalDeviceExternalFenceInfo
{
	sType: VkStructureType
	pNext: void
	handleType: VkExternalFenceHandleTypeFlagBits
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

pub struct VkExternalFenceProperties
{
	sType: VkStructureType
	pNext: void
	exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags
	compatibleHandleTypes: VkExternalFenceHandleTypeFlags
	externalFenceFeatures: VkExternalFenceFeatureFlags
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

pub struct VkExportFenceCreateInfo
{
	sType: VkStructureType
	pNext: void
	handleTypes: VkExternalFenceHandleTypeFlags
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

pub struct VkImportFenceWin32HandleInfoKHR
{
	sType: VkStructureType
	pNext: void
	fence: VkFence
	flags: VkFenceImportFlags
	handleType: VkExternalFenceHandleTypeFlagBits
	handle: HANDLE
	name: LPCWSTR
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

pub struct VkExportFenceWin32HandleInfoKHR
{
	sType: VkStructureType
	pNext: void
	pAttributes: SECURITY_ATTRIBUTES
	dwAccess: DWORD
	name: LPCWSTR
}
impl VkExportFenceWin32HandleInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR;

        return s;
    }
}

pub struct VkFenceGetWin32HandleInfoKHR
{
	sType: VkStructureType
	pNext: void
	fence: VkFence
	handleType: VkExternalFenceHandleTypeFlagBits
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

pub struct VkImportFenceFdInfoKHR
{
	sType: VkStructureType
	pNext: void
	fence: VkFence
	flags: VkFenceImportFlags
	handleType: VkExternalFenceHandleTypeFlagBits
	fd: int
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

pub struct VkFenceGetFdInfoKHR
{
	sType: VkStructureType
	pNext: void
	fence: VkFence
	handleType: VkExternalFenceHandleTypeFlagBits
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

pub struct VkPhysicalDeviceMultiviewFeatures
{
	sType: VkStructureType
	pNext: void
	multiview: VkBool32
	multiviewGeometryShader: VkBool32
	multiviewTessellationShader: VkBool32
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

pub struct VkPhysicalDeviceMultiviewProperties
{
	sType: VkStructureType
	pNext: void
	maxMultiviewViewCount: uint32_t
	maxMultiviewInstanceIndex: uint32_t
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

pub struct VkRenderPassMultiviewCreateInfo
{
	sType: VkStructureType
	pNext: void
	subpassCount: uint32_t
	pViewMasks: uint32_t
	dependencyCount: uint32_t
	pViewOffsets: int32_t
	correlationMaskCount: uint32_t
	pCorrelationMasks: uint32_t
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

pub struct VkSurfaceCapabilities2EXT
{
	sType: VkStructureType
	pNext: void
	minImageCount: uint32_t
	maxImageCount: uint32_t
	currentExtent: VkExtent2D
	minImageExtent: VkExtent2D
	maxImageExtent: VkExtent2D
	maxImageArrayLayers: uint32_t
	supportedTransforms: VkSurfaceTransformFlagsKHR
	currentTransform: VkSurfaceTransformFlagBitsKHR
	supportedCompositeAlpha: VkCompositeAlphaFlagsKHR
	supportedUsageFlags: VkImageUsageFlags
	supportedSurfaceCounters: VkSurfaceCounterFlagsEXT
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

pub struct VkDisplayPowerInfoEXT
{
	sType: VkStructureType
	pNext: void
	powerState: VkDisplayPowerStateEXT
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

pub struct VkDeviceEventInfoEXT
{
	sType: VkStructureType
	pNext: void
	deviceEvent: VkDeviceEventTypeEXT
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

pub struct VkDisplayEventInfoEXT
{
	sType: VkStructureType
	pNext: void
	displayEvent: VkDisplayEventTypeEXT
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

pub struct VkSwapchainCounterCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	surfaceCounters: VkSurfaceCounterFlagsEXT
}
impl VkSwapchainCounterCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceGroupProperties
{
	sType: VkStructureType
	pNext: void
	physicalDeviceCount: uint32_t
	physicalDevices: VkPhysicalDevice
	subsetAllocation: VkBool32
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

pub struct VkMemoryAllocateFlagsInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkMemoryAllocateFlags
	deviceMask: uint32_t
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

pub struct VkBindBufferMemoryInfo
{
	sType: VkStructureType
	pNext: void
	buffer: VkBuffer
	memory: VkDeviceMemory
	memoryOffset: VkDeviceSize
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

pub struct VkBindBufferMemoryDeviceGroupInfo
{
	sType: VkStructureType
	pNext: void
	deviceIndexCount: uint32_t
	pDeviceIndices: uint32_t
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

pub struct VkBindImageMemoryInfo
{
	sType: VkStructureType
	pNext: void
	image: VkImage
	memory: VkDeviceMemory
	memoryOffset: VkDeviceSize
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

pub struct VkBindImageMemoryDeviceGroupInfo
{
	sType: VkStructureType
	pNext: void
	deviceIndexCount: uint32_t
	pDeviceIndices: uint32_t
	splitInstanceBindRegionCount: uint32_t
	pSplitInstanceBindRegions: VkRect2D
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

pub struct VkDeviceGroupRenderPassBeginInfo
{
	sType: VkStructureType
	pNext: void
	deviceMask: uint32_t
	deviceRenderAreaCount: uint32_t
	pDeviceRenderAreas: VkRect2D
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

pub struct VkDeviceGroupCommandBufferBeginInfo
{
	sType: VkStructureType
	pNext: void
	deviceMask: uint32_t
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

pub struct VkDeviceGroupSubmitInfo
{
	sType: VkStructureType
	pNext: void
	waitSemaphoreCount: uint32_t
	pWaitSemaphoreDeviceIndices: uint32_t
	commandBufferCount: uint32_t
	pCommandBufferDeviceMasks: uint32_t
	signalSemaphoreCount: uint32_t
	pSignalSemaphoreDeviceIndices: uint32_t
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

pub struct VkDeviceGroupBindSparseInfo
{
	sType: VkStructureType
	pNext: void
	resourceDeviceIndex: uint32_t
	memoryDeviceIndex: uint32_t
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

pub struct VkDeviceGroupPresentCapabilitiesKHR
{
	sType: VkStructureType
	pNext: void
	presentMask: uint32_t
	modes: VkDeviceGroupPresentModeFlagsKHR
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

pub struct VkImageSwapchainCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	swapchain: VkSwapchainKHR
}
impl VkImageSwapchainCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR;

        return s;
    }
}

pub struct VkBindImageMemorySwapchainInfoKHR
{
	sType: VkStructureType
	pNext: void
	swapchain: VkSwapchainKHR
	imageIndex: uint32_t
}
impl VkBindImageMemorySwapchainInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR;

        return s;
    }
}

pub struct VkAcquireNextImageInfoKHR
{
	sType: VkStructureType
	pNext: void
	swapchain: VkSwapchainKHR
	timeout: uint64_t
	semaphore: VkSemaphore
	fence: VkFence
	deviceMask: uint32_t
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

pub struct VkDeviceGroupPresentInfoKHR
{
	sType: VkStructureType
	pNext: void
	swapchainCount: uint32_t
	pDeviceMasks: uint32_t
	mode: VkDeviceGroupPresentModeFlagBitsKHR
}
impl VkDeviceGroupPresentInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR;

        return s;
    }
}

pub struct VkDeviceGroupDeviceCreateInfo
{
	sType: VkStructureType
	pNext: void
	physicalDeviceCount: uint32_t
	pPhysicalDevices: VkPhysicalDevice
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

pub struct VkDeviceGroupSwapchainCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	modes: VkDeviceGroupPresentModeFlagsKHR
}
impl VkDeviceGroupSwapchainCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR;

        return s;
    }
}

pub struct VkDescriptorUpdateTemplateEntry
{
	dstBinding: uint32_t
	dstArrayElement: uint32_t
	descriptorCount: uint32_t
	descriptorType: VkDescriptorType
	offset: size_t
	stride: size_t
}
impl VkDescriptorUpdateTemplateEntry
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

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

pub struct VkDescriptorUpdateTemplateCreateInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkDescriptorUpdateTemplateCreateFlags
	descriptorUpdateEntryCount: uint32_t
	pDescriptorUpdateEntries: VkDescriptorUpdateTemplateEntry
	templateType: VkDescriptorUpdateTemplateType
	descriptorSetLayout: VkDescriptorSetLayout
	pipelineBindPoint: VkPipelineBindPoint
	pipelineLayout: VkPipelineLayout
	set: uint32_t
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

pub struct VkXYColorEXT
{
	x: float
	y: float
}
impl VkXYColorEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPhysicalDevicePresentIdFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	presentId: VkBool32
}
impl VkPhysicalDevicePresentIdFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR;

        return s;
    }
}

pub struct VkPresentIdKHR
{
	sType: VkStructureType
	pNext: void
	swapchainCount: uint32_t
	pPresentIds: uint64_t
}
impl VkPresentIdKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PRESENT_ID_KHR;

        return s;
    }
}

pub struct VkPhysicalDevicePresentWaitFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	presentWait: VkBool32
}
impl VkPhysicalDevicePresentWaitFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR;

        return s;
    }
}

pub struct VkHdrMetadataEXT
{
	sType: VkStructureType
	pNext: void
	displayPrimaryRed: VkXYColorEXT
	displayPrimaryGreen: VkXYColorEXT
	displayPrimaryBlue: VkXYColorEXT
	whitePoint: VkXYColorEXT
	maxLuminance: float
	minLuminance: float
	maxContentLightLevel: float
	maxFrameAverageLightLevel: float
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

pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD
{
	sType: VkStructureType
	pNext: void
	localDimmingSupport: VkBool32
}
impl VkDisplayNativeHdrSurfaceCapabilitiesAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD;

        return s;
    }
}

pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD
{
	sType: VkStructureType
	pNext: void
	localDimmingEnable: VkBool32
}
impl VkSwapchainDisplayNativeHdrCreateInfoAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD;

        return s;
    }
}

pub struct VkRefreshCycleDurationGOOGLE
{
	refreshDuration: uint64_t
}
impl VkRefreshCycleDurationGOOGLE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPastPresentationTimingGOOGLE
{
	presentID: uint32_t
	desiredPresentTime: uint64_t
	actualPresentTime: uint64_t
	earliestPresentTime: uint64_t
	presentMargin: uint64_t
}
impl VkPastPresentationTimingGOOGLE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPresentTimesInfoGOOGLE
{
	sType: VkStructureType
	pNext: void
	swapchainCount: uint32_t
	pTimes: VkPresentTimeGOOGLE
}
impl VkPresentTimesInfoGOOGLE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE;

        return s;
    }
}

pub struct VkPresentTimeGOOGLE
{
	presentID: uint32_t
	desiredPresentTime: uint64_t
}
impl VkPresentTimeGOOGLE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkIOSSurfaceCreateInfoMVK
{
	sType: VkStructureType
	pNext: void
	flags: VkIOSSurfaceCreateFlagsMVK
	pView: void
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

pub struct VkMacOSSurfaceCreateInfoMVK
{
	sType: VkStructureType
	pNext: void
	flags: VkMacOSSurfaceCreateFlagsMVK
	pView: void
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

pub struct VkMetalSurfaceCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkMetalSurfaceCreateFlagsEXT
	pLayer: CAMetalLayer
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

pub struct VkViewportWScalingNV
{
	xcoeff: float
	ycoeff: float
}
impl VkViewportWScalingNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineViewportWScalingStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	viewportWScalingEnable: VkBool32
	viewportCount: uint32_t
	pViewportWScalings: VkViewportWScalingNV
}
impl VkPipelineViewportWScalingStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkViewportSwizzleNV
{
	x: VkViewportCoordinateSwizzleNV
	y: VkViewportCoordinateSwizzleNV
	z: VkViewportCoordinateSwizzleNV
	w: VkViewportCoordinateSwizzleNV
}
impl VkViewportSwizzleNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineViewportSwizzleStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineViewportSwizzleStateCreateFlagsNV
	viewportCount: uint32_t
	pViewportSwizzles: VkViewportSwizzleNV
}
impl VkPipelineViewportSwizzleStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT
{
	sType: VkStructureType
	pNext: void
	maxDiscardRectangles: uint32_t
}
impl VkPhysicalDeviceDiscardRectanglePropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPipelineDiscardRectangleStateCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineDiscardRectangleStateCreateFlagsEXT
	discardRectangleMode: VkDiscardRectangleModeEXT
	discardRectangleCount: uint32_t
	pDiscardRectangles: VkRect2D
}
impl VkPipelineDiscardRectangleStateCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
{
	sType: VkStructureType
	pNext: void
	perViewPositionAllComponents: VkBool32
}
impl VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX;

        return s;
    }
}

pub struct VkInputAttachmentAspectReference
{
	subpass: uint32_t
	inputAttachmentIndex: uint32_t
	aspectMask: VkImageAspectFlags
}
impl VkInputAttachmentAspectReference
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

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

pub struct VkRenderPassInputAttachmentAspectCreateInfo
{
	sType: VkStructureType
	pNext: void
	aspectReferenceCount: uint32_t
	pAspectReferences: VkInputAttachmentAspectReference
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

pub struct VkPhysicalDeviceSurfaceInfo2KHR
{
	sType: VkStructureType
	pNext: void
	surface: VkSurfaceKHR
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

pub struct VkSurfaceCapabilities2KHR
{
	sType: VkStructureType
	pNext: void
	surfaceCapabilities: VkSurfaceCapabilitiesKHR
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

pub struct VkSurfaceFormat2KHR
{
	sType: VkStructureType
	pNext: void
	surfaceFormat: VkSurfaceFormatKHR
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

pub struct VkDisplayProperties2KHR
{
	sType: VkStructureType
	pNext: void
	displayProperties: VkDisplayPropertiesKHR
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

pub struct VkDisplayPlaneProperties2KHR
{
	sType: VkStructureType
	pNext: void
	displayPlaneProperties: VkDisplayPlanePropertiesKHR
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

pub struct VkDisplayModeProperties2KHR
{
	sType: VkStructureType
	pNext: void
	displayModeProperties: VkDisplayModePropertiesKHR
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

pub struct VkDisplayPlaneInfo2KHR
{
	sType: VkStructureType
	pNext: void
	mode: VkDisplayModeKHR
	planeIndex: uint32_t
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

pub struct VkDisplayPlaneCapabilities2KHR
{
	sType: VkStructureType
	pNext: void
	capabilities: VkDisplayPlaneCapabilitiesKHR
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

pub struct VkSharedPresentSurfaceCapabilitiesKHR
{
	sType: VkStructureType
	pNext: void
	sharedPresentSupportedUsageFlags: VkImageUsageFlags
}
impl VkSharedPresentSurfaceCapabilitiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR;

        return s;
    }
}

pub struct VkPhysicalDevice16BitStorageFeatures
{
	sType: VkStructureType
	pNext: void
	storageBuffer16BitAccess: VkBool32
	uniformAndStorageBuffer16BitAccess: VkBool32
	storagePushConstant16: VkBool32
	storageInputOutput16: VkBool32
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

pub struct VkPhysicalDeviceSubgroupProperties
{
	sType: VkStructureType
	pNext: void
	subgroupSize: uint32_t
	supportedStages: VkShaderStageFlags
	supportedOperations: VkSubgroupFeatureFlags
	quadOperationsInAllStages: VkBool32
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

pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures
{
	sType: VkStructureType
	pNext: void
	shaderSubgroupExtendedTypes: VkBool32
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

pub struct VkBufferMemoryRequirementsInfo2
{
	sType: VkStructureType
	pNext: void
	buffer: VkBuffer
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

pub struct VkImageMemoryRequirementsInfo2
{
	sType: VkStructureType
	pNext: void
	image: VkImage
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

pub struct VkImageSparseMemoryRequirementsInfo2
{
	sType: VkStructureType
	pNext: void
	image: VkImage
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

pub struct VkMemoryRequirements2
{
	sType: VkStructureType
	pNext: void
	memoryRequirements: VkMemoryRequirements
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

pub struct VkSparseImageMemoryRequirements2
{
	sType: VkStructureType
	pNext: void
	memoryRequirements: VkSparseImageMemoryRequirements
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

pub struct VkPhysicalDevicePointClippingProperties
{
	sType: VkStructureType
	pNext: void
	pointClippingBehavior: VkPointClippingBehavior
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

pub struct VkMemoryDedicatedRequirements
{
	sType: VkStructureType
	pNext: void
	prefersDedicatedAllocation: VkBool32
	requiresDedicatedAllocation: VkBool32
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

pub struct VkMemoryDedicatedAllocateInfo
{
	sType: VkStructureType
	pNext: void
	image: VkImage
	buffer: VkBuffer
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

pub struct VkImageViewUsageCreateInfo
{
	sType: VkStructureType
	pNext: void
	usage: VkImageUsageFlags
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

pub struct VkPipelineTessellationDomainOriginStateCreateInfo
{
	sType: VkStructureType
	pNext: void
	domainOrigin: VkTessellationDomainOrigin
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

pub struct VkSamplerYcbcrConversionInfo
{
	sType: VkStructureType
	pNext: void
	conversion: VkSamplerYcbcrConversion
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

pub struct VkSamplerYcbcrConversionCreateInfo
{
	sType: VkStructureType
	pNext: void
	format: VkFormat
	ycbcrModel: VkSamplerYcbcrModelConversion
	ycbcrRange: VkSamplerYcbcrRange
	components: VkComponentMapping
	xChromaOffset: VkChromaLocation
	yChromaOffset: VkChromaLocation
	chromaFilter: VkFilter
	forceExplicitReconstruction: VkBool32
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

pub struct VkBindImagePlaneMemoryInfo
{
	sType: VkStructureType
	pNext: void
	planeAspect: VkImageAspectFlagBits
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

pub struct VkImagePlaneMemoryRequirementsInfo
{
	sType: VkStructureType
	pNext: void
	planeAspect: VkImageAspectFlagBits
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

pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures
{
	sType: VkStructureType
	pNext: void
	samplerYcbcrConversion: VkBool32
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

pub struct VkSamplerYcbcrConversionImageFormatProperties
{
	sType: VkStructureType
	pNext: void
	combinedImageSamplerDescriptorCount: uint32_t
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

pub struct VkTextureLODGatherFormatPropertiesAMD
{
	sType: VkStructureType
	pNext: void
	supportsTextureGatherLODBiasAMD: VkBool32
}
impl VkTextureLODGatherFormatPropertiesAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD;

        return s;
    }
}

pub struct VkConditionalRenderingBeginInfoEXT
{
	sType: VkStructureType
	pNext: void
	buffer: VkBuffer
	offset: VkDeviceSize
	flags: VkConditionalRenderingFlagsEXT
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

pub struct VkProtectedSubmitInfo
{
	sType: VkStructureType
	pNext: void
	protectedSubmit: VkBool32
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

pub struct VkPhysicalDeviceProtectedMemoryFeatures
{
	sType: VkStructureType
	pNext: void
	protectedMemory: VkBool32
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

pub struct VkPhysicalDeviceProtectedMemoryProperties
{
	sType: VkStructureType
	pNext: void
	protectedNoFault: VkBool32
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

pub struct VkDeviceQueueInfo2
{
	sType: VkStructureType
	pNext: void
	flags: VkDeviceQueueCreateFlags
	queueFamilyIndex: uint32_t
	queueIndex: uint32_t
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

pub struct VkPipelineCoverageToColorStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineCoverageToColorStateCreateFlagsNV
	coverageToColorEnable: VkBool32
	coverageToColorLocation: uint32_t
}
impl VkPipelineCoverageToColorStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties
{
	sType: VkStructureType
	pNext: void
	filterMinmaxSingleComponentFormats: VkBool32
	filterMinmaxImageComponentMapping: VkBool32
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

pub struct VkSampleLocationEXT
{
	x: float
	y: float
}
impl VkSampleLocationEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSampleLocationsInfoEXT
{
	sType: VkStructureType
	pNext: void
	sampleLocationsPerPixel: VkSampleCountFlagBits
	sampleLocationGridSize: VkExtent2D
	sampleLocationsCount: uint32_t
	pSampleLocations: VkSampleLocationEXT
}
impl VkSampleLocationsInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT;

        return s;
    }
}

pub struct VkAttachmentSampleLocationsEXT
{
	attachmentIndex: uint32_t
	sampleLocationsInfo: VkSampleLocationsInfoEXT
}
impl VkAttachmentSampleLocationsEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkSubpassSampleLocationsEXT
{
	subpassIndex: uint32_t
	sampleLocationsInfo: VkSampleLocationsInfoEXT
}
impl VkSubpassSampleLocationsEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkRenderPassSampleLocationsBeginInfoEXT
{
	sType: VkStructureType
	pNext: void
	attachmentInitialSampleLocationsCount: uint32_t
	pAttachmentInitialSampleLocations: VkAttachmentSampleLocationsEXT
	postSubpassSampleLocationsCount: uint32_t
	pPostSubpassSampleLocations: VkSubpassSampleLocationsEXT
}
impl VkRenderPassSampleLocationsBeginInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT;

        return s;
    }
}

pub struct VkPipelineSampleLocationsStateCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	sampleLocationsEnable: VkBool32
	sampleLocationsInfo: VkSampleLocationsInfoEXT
}
impl VkPipelineSampleLocationsStateCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	sampleLocationSampleCounts: VkSampleCountFlags
	maxSampleLocationGridSize: VkExtent2D
	sampleLocationCoordinateRange: float
	sampleLocationSubPixelBits: uint32_t
	variableSampleLocations: VkBool32
}
impl VkPhysicalDeviceSampleLocationsPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkMultisamplePropertiesEXT
{
	sType: VkStructureType
	pNext: void
	maxSampleLocationGridSize: VkExtent2D
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

pub struct VkSamplerReductionModeCreateInfo
{
	sType: VkStructureType
	pNext: void
	reductionMode: VkSamplerReductionMode
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

pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	advancedBlendCoherentOperations: VkBool32
}
impl VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceMultiDrawFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	multiDraw: VkBool32
}
impl VkPhysicalDeviceMultiDrawFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	advancedBlendMaxColorAttachments: uint32_t
	advancedBlendIndependentBlend: VkBool32
	advancedBlendNonPremultipliedSrcColor: VkBool32
	advancedBlendNonPremultipliedDstColor: VkBool32
	advancedBlendCorrelatedOverlap: VkBool32
	advancedBlendAllOperations: VkBool32
}
impl VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	srcPremultiplied: VkBool32
	dstPremultiplied: VkBool32
	blendOverlap: VkBlendOverlapEXT
}
impl VkPipelineColorBlendAdvancedStateCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceInlineUniformBlockFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	inlineUniformBlock: VkBool32
	descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32
}
impl VkPhysicalDeviceInlineUniformBlockFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceInlineUniformBlockPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	maxInlineUniformBlockSize: uint32_t
	maxPerStageDescriptorInlineUniformBlocks: uint32_t
	maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: uint32_t
	maxDescriptorSetInlineUniformBlocks: uint32_t
	maxDescriptorSetUpdateAfterBindInlineUniformBlocks: uint32_t
}
impl VkPhysicalDeviceInlineUniformBlockPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkWriteDescriptorSetInlineUniformBlockEXT
{
	sType: VkStructureType
	pNext: void
	dataSize: uint32_t
	pData: void
}
impl VkWriteDescriptorSetInlineUniformBlockEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT;

        return s;
    }
}

pub struct VkDescriptorPoolInlineUniformBlockCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	maxInlineUniformBlockBindings: uint32_t
}
impl VkDescriptorPoolInlineUniformBlockCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPipelineCoverageModulationStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineCoverageModulationStateCreateFlagsNV
	coverageModulationMode: VkCoverageModulationModeNV
	coverageModulationTableEnable: VkBool32
	coverageModulationTableCount: uint32_t
	pCoverageModulationTable: float
}
impl VkPipelineCoverageModulationStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkImageFormatListCreateInfo
{
	sType: VkStructureType
	pNext: void
	viewFormatCount: uint32_t
	pViewFormats: VkFormat
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

pub struct VkValidationCacheCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkValidationCacheCreateFlagsEXT
	initialDataSize: size_t
	pInitialData: void
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

pub struct VkShaderModuleValidationCacheCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	validationCache: VkValidationCacheEXT
}
impl VkShaderModuleValidationCacheCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceMaintenance3Properties
{
	sType: VkStructureType
	pNext: void
	maxPerSetDescriptors: uint32_t
	maxMemoryAllocationSize: VkDeviceSize
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

pub struct VkDescriptorSetLayoutSupport
{
	sType: VkStructureType
	pNext: void
	supported: VkBool32
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

pub struct VkPhysicalDeviceShaderDrawParametersFeatures
{
	sType: VkStructureType
	pNext: void
	shaderDrawParameters: VkBool32
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

pub struct VkPhysicalDeviceShaderFloat16Int8Features
{
	sType: VkStructureType
	pNext: void
	shaderFloat16: VkBool32
	shaderInt8: VkBool32
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

pub struct VkPhysicalDeviceFloatControlsProperties
{
	sType: VkStructureType
	pNext: void
	denormBehaviorIndependence: VkShaderFloatControlsIndependence
	roundingModeIndependence: VkShaderFloatControlsIndependence
	shaderSignedZeroInfNanPreserveFloat16: VkBool32
	shaderSignedZeroInfNanPreserveFloat32: VkBool32
	shaderSignedZeroInfNanPreserveFloat64: VkBool32
	shaderDenormPreserveFloat16: VkBool32
	shaderDenormPreserveFloat32: VkBool32
	shaderDenormPreserveFloat64: VkBool32
	shaderDenormFlushToZeroFloat16: VkBool32
	shaderDenormFlushToZeroFloat32: VkBool32
	shaderDenormFlushToZeroFloat64: VkBool32
	shaderRoundingModeRTEFloat16: VkBool32
	shaderRoundingModeRTEFloat32: VkBool32
	shaderRoundingModeRTEFloat64: VkBool32
	shaderRoundingModeRTZFloat16: VkBool32
	shaderRoundingModeRTZFloat32: VkBool32
	shaderRoundingModeRTZFloat64: VkBool32
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

pub struct VkPhysicalDeviceHostQueryResetFeatures
{
	sType: VkStructureType
	pNext: void
	hostQueryReset: VkBool32
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

pub struct VkNativeBufferUsage2ANDROID
{
	consumer: uint64_t
	producer: uint64_t
}
impl VkNativeBufferUsage2ANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkNativeBufferANDROID
{
	sType: VkStructureType
	pNext: void
	handle: void
	stride: int
	format: int
	usage: int
	usage2: VkNativeBufferUsage2ANDROID
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

pub struct VkSwapchainImageCreateInfoANDROID
{
	sType: VkStructureType
	pNext: void
	usage: VkSwapchainImageUsageFlagsANDROID
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

pub struct VkPhysicalDevicePresentationPropertiesANDROID
{
	sType: VkStructureType
	pNext: void
	sharedImage: VkBool32
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

pub struct VkShaderResourceUsageAMD
{
	numUsedVgprs: uint32_t
	numUsedSgprs: uint32_t
	ldsSizePerLocalWorkGroup: uint32_t
	ldsUsageSizeInBytes: size_t
	scratchMemUsageInBytes: size_t
}
impl VkShaderResourceUsageAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkShaderStatisticsInfoAMD
{
	shaderStageMask: VkShaderStageFlags
	resourceUsage: VkShaderResourceUsageAMD
	numPhysicalVgprs: uint32_t
	numPhysicalSgprs: uint32_t
	numAvailableVgprs: uint32_t
	numAvailableSgprs: uint32_t
	computeWorkGroupSize: uint32_t
}
impl VkShaderStatisticsInfoAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	globalPriority: VkQueueGlobalPriorityEXT
}
impl VkDeviceQueueGlobalPriorityCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	globalPriorityQuery: VkBool32
}
impl VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT;

        return s;
    }
}

pub struct VkQueueFamilyGlobalPriorityPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	priorityCount: uint32_t
	priorities: VkQueueGlobalPriorityEXT
}
impl VkQueueFamilyGlobalPriorityPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkDebugUtilsObjectNameInfoEXT
{
	sType: VkStructureType
	pNext: void
	objectType: VkObjectType
	objectHandle: uint64_t
	pObjectName: char
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

pub struct VkDebugUtilsObjectTagInfoEXT
{
	sType: VkStructureType
	pNext: void
	objectType: VkObjectType
	objectHandle: uint64_t
	tagName: uint64_t
	tagSize: size_t
	pTag: void
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

pub struct VkDebugUtilsLabelEXT
{
	sType: VkStructureType
	pNext: void
	pLabelName: char
	color: float
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

pub struct VkDebugUtilsMessengerCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkDebugUtilsMessengerCreateFlagsEXT
	messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT
	messageType: VkDebugUtilsMessageTypeFlagsEXT
	pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT
	pUserData: void
}
impl VkDebugUtilsMessengerCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkDebugUtilsMessengerCallbackDataEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkDebugUtilsMessengerCallbackDataFlagsEXT
	pMessageIdName: char
	messageIdNumber: int32_t
	pMessage: char
	queueLabelCount: uint32_t
	pQueueLabels: VkDebugUtilsLabelEXT
	cmdBufLabelCount: uint32_t
	pCmdBufLabels: VkDebugUtilsLabelEXT
	objectCount: uint32_t
	pObjects: VkDebugUtilsObjectNameInfoEXT
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

pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	deviceMemoryReport: VkBool32
}
impl VkPhysicalDeviceDeviceMemoryReportFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT;

        return s;
    }
}

pub struct VkDeviceDeviceMemoryReportCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkDeviceMemoryReportFlagsEXT
	pfnUserCallback: PFN_vkDeviceMemoryReportCallbackEXT
	pUserData: void
}
impl VkDeviceDeviceMemoryReportCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkDeviceMemoryReportCallbackDataEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkDeviceMemoryReportFlagsEXT
	type: VkDeviceMemoryReportEventTypeEXT
	memoryObjectId: uint64_t
	size: VkDeviceSize
	objectType: VkObjectType
	objectHandle: uint64_t
	heapIndex: uint32_t
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

pub struct VkImportMemoryHostPointerInfoEXT
{
	sType: VkStructureType
	pNext: void
	handleType: VkExternalMemoryHandleTypeFlagBits
	pHostPointer: void
}
impl VkImportMemoryHostPointerInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT;

        return s;
    }
}

pub struct VkMemoryHostPointerPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	memoryTypeBits: uint32_t
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

pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	minImportedHostPointerAlignment: VkDeviceSize
}
impl VkPhysicalDeviceExternalMemoryHostPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	primitiveOverestimationSize: float
	maxExtraPrimitiveOverestimationSize: float
	extraPrimitiveOverestimationSizeGranularity: float
	primitiveUnderestimation: VkBool32
	conservativePointAndLineRasterization: VkBool32
	degenerateTrianglesRasterized: VkBool32
	degenerateLinesRasterized: VkBool32
	fullyCoveredFragmentShaderInputVariable: VkBool32
	conservativeRasterizationPostDepthCoverage: VkBool32
}
impl VkPhysicalDeviceConservativeRasterizationPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkCalibratedTimestampInfoEXT
{
	sType: VkStructureType
	pNext: void
	timeDomain: VkTimeDomainEXT
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

pub struct VkPhysicalDeviceShaderCorePropertiesAMD
{
	sType: VkStructureType
	pNext: void
	shaderEngineCount: uint32_t
	shaderArraysPerEngineCount: uint32_t
	computeUnitsPerShaderArray: uint32_t
	simdPerComputeUnit: uint32_t
	wavefrontsPerSimd: uint32_t
	wavefrontSize: uint32_t
	sgprsPerSimd: uint32_t
	minSgprAllocation: uint32_t
	maxSgprAllocation: uint32_t
	sgprAllocationGranularity: uint32_t
	vgprsPerSimd: uint32_t
	minVgprAllocation: uint32_t
	maxVgprAllocation: uint32_t
	vgprAllocationGranularity: uint32_t
}
impl VkPhysicalDeviceShaderCorePropertiesAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD;

        return s;
    }
}

pub struct VkPhysicalDeviceShaderCoreProperties2AMD
{
	sType: VkStructureType
	pNext: void
	shaderCoreFeatures: VkShaderCorePropertiesFlagsAMD
	activeComputeUnitCount: uint32_t
}
impl VkPhysicalDeviceShaderCoreProperties2AMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD;

        return s;
    }
}

pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT
	conservativeRasterizationMode: VkConservativeRasterizationModeEXT
	extraPrimitiveOverestimationSize: float
}
impl VkPipelineRasterizationConservativeStateCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceDescriptorIndexingFeatures
{
	sType: VkStructureType
	pNext: void
	shaderInputAttachmentArrayDynamicIndexing: VkBool32
	shaderUniformTexelBufferArrayDynamicIndexing: VkBool32
	shaderStorageTexelBufferArrayDynamicIndexing: VkBool32
	shaderUniformBufferArrayNonUniformIndexing: VkBool32
	shaderSampledImageArrayNonUniformIndexing: VkBool32
	shaderStorageBufferArrayNonUniformIndexing: VkBool32
	shaderStorageImageArrayNonUniformIndexing: VkBool32
	shaderInputAttachmentArrayNonUniformIndexing: VkBool32
	shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32
	shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32
	descriptorBindingUniformBufferUpdateAfterBind: VkBool32
	descriptorBindingSampledImageUpdateAfterBind: VkBool32
	descriptorBindingStorageImageUpdateAfterBind: VkBool32
	descriptorBindingStorageBufferUpdateAfterBind: VkBool32
	descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32
	descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32
	descriptorBindingUpdateUnusedWhilePending: VkBool32
	descriptorBindingPartiallyBound: VkBool32
	descriptorBindingVariableDescriptorCount: VkBool32
	runtimeDescriptorArray: VkBool32
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

pub struct VkPhysicalDeviceDescriptorIndexingProperties
{
	sType: VkStructureType
	pNext: void
	maxUpdateAfterBindDescriptorsInAllPools: uint32_t
	shaderUniformBufferArrayNonUniformIndexingNative: VkBool32
	shaderSampledImageArrayNonUniformIndexingNative: VkBool32
	shaderStorageBufferArrayNonUniformIndexingNative: VkBool32
	shaderStorageImageArrayNonUniformIndexingNative: VkBool32
	shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32
	robustBufferAccessUpdateAfterBind: VkBool32
	quadDivergentImplicitLod: VkBool32
	maxPerStageDescriptorUpdateAfterBindSamplers: uint32_t
	maxPerStageDescriptorUpdateAfterBindUniformBuffers: uint32_t
	maxPerStageDescriptorUpdateAfterBindStorageBuffers: uint32_t
	maxPerStageDescriptorUpdateAfterBindSampledImages: uint32_t
	maxPerStageDescriptorUpdateAfterBindStorageImages: uint32_t
	maxPerStageDescriptorUpdateAfterBindInputAttachments: uint32_t
	maxPerStageUpdateAfterBindResources: uint32_t
	maxDescriptorSetUpdateAfterBindSamplers: uint32_t
	maxDescriptorSetUpdateAfterBindUniformBuffers: uint32_t
	maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: uint32_t
	maxDescriptorSetUpdateAfterBindStorageBuffers: uint32_t
	maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: uint32_t
	maxDescriptorSetUpdateAfterBindSampledImages: uint32_t
	maxDescriptorSetUpdateAfterBindStorageImages: uint32_t
	maxDescriptorSetUpdateAfterBindInputAttachments: uint32_t
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

pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo
{
	sType: VkStructureType
	pNext: void
	bindingCount: uint32_t
	pBindingFlags: VkDescriptorBindingFlags
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

pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo
{
	sType: VkStructureType
	pNext: void
	descriptorSetCount: uint32_t
	pDescriptorCounts: uint32_t
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

pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport
{
	sType: VkStructureType
	pNext: void
	maxVariableDescriptorCount: uint32_t
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

pub struct VkAttachmentDescription2
{
	sType: VkStructureType
	pNext: void
	flags: VkAttachmentDescriptionFlags
	format: VkFormat
	samples: VkSampleCountFlagBits
	loadOp: VkAttachmentLoadOp
	storeOp: VkAttachmentStoreOp
	stencilLoadOp: VkAttachmentLoadOp
	stencilStoreOp: VkAttachmentStoreOp
	initialLayout: VkImageLayout
	finalLayout: VkImageLayout
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

pub struct VkAttachmentReference2
{
	sType: VkStructureType
	pNext: void
	attachment: uint32_t
	layout: VkImageLayout
	aspectMask: VkImageAspectFlags
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

pub struct VkSubpassDescription2
{
	sType: VkStructureType
	pNext: void
	flags: VkSubpassDescriptionFlags
	pipelineBindPoint: VkPipelineBindPoint
	viewMask: uint32_t
	inputAttachmentCount: uint32_t
	pInputAttachments: VkAttachmentReference2
	colorAttachmentCount: uint32_t
	pColorAttachments: VkAttachmentReference2
	pResolveAttachments: VkAttachmentReference2
	pDepthStencilAttachment: VkAttachmentReference2
	preserveAttachmentCount: uint32_t
	pPreserveAttachments: uint32_t
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

pub struct VkSubpassDependency2
{
	sType: VkStructureType
	pNext: void
	srcSubpass: uint32_t
	dstSubpass: uint32_t
	srcStageMask: VkPipelineStageFlags
	dstStageMask: VkPipelineStageFlags
	srcAccessMask: VkAccessFlags
	dstAccessMask: VkAccessFlags
	dependencyFlags: VkDependencyFlags
	viewOffset: int32_t
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

pub struct VkRenderPassCreateInfo2
{
	sType: VkStructureType
	pNext: void
	flags: VkRenderPassCreateFlags
	attachmentCount: uint32_t
	pAttachments: VkAttachmentDescription2
	subpassCount: uint32_t
	pSubpasses: VkSubpassDescription2
	dependencyCount: uint32_t
	pDependencies: VkSubpassDependency2
	correlatedViewMaskCount: uint32_t
	pCorrelatedViewMasks: uint32_t
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

pub struct VkSubpassBeginInfo
{
	sType: VkStructureType
	pNext: void
	contents: VkSubpassContents
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

pub struct VkSubpassEndInfo
{
	sType: VkStructureType
	pNext: void
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

pub struct VkPhysicalDeviceTimelineSemaphoreFeatures
{
	sType: VkStructureType
	pNext: void
	timelineSemaphore: VkBool32
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

pub struct VkPhysicalDeviceTimelineSemaphoreProperties
{
	sType: VkStructureType
	pNext: void
	maxTimelineSemaphoreValueDifference: uint64_t
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

pub struct VkSemaphoreTypeCreateInfo
{
	sType: VkStructureType
	pNext: void
	semaphoreType: VkSemaphoreType
	initialValue: uint64_t
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

pub struct VkTimelineSemaphoreSubmitInfo
{
	sType: VkStructureType
	pNext: void
	waitSemaphoreValueCount: uint32_t
	pWaitSemaphoreValues: uint64_t
	signalSemaphoreValueCount: uint32_t
	pSignalSemaphoreValues: uint64_t
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

pub struct VkSemaphoreWaitInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkSemaphoreWaitFlags
	semaphoreCount: uint32_t
	pSemaphores: VkSemaphore
	pValues: uint64_t
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

pub struct VkSemaphoreSignalInfo
{
	sType: VkStructureType
	pNext: void
	semaphore: VkSemaphore
	value: uint64_t
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

pub struct VkVertexInputBindingDivisorDescriptionEXT
{
	binding: uint32_t
	divisor: uint32_t
}
impl VkVertexInputBindingDivisorDescriptionEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	vertexBindingDivisorCount: uint32_t
	pVertexBindingDivisors: VkVertexInputBindingDivisorDescriptionEXT
}
impl VkPipelineVertexInputDivisorStateCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	maxVertexAttribDivisor: uint32_t
}
impl VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	pciDomain: uint32_t
	pciBus: uint32_t
	pciDevice: uint32_t
	pciFunction: uint32_t
}
impl VkPhysicalDevicePCIBusInfoPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkImportAndroidHardwareBufferInfoANDROID
{
	sType: VkStructureType
	pNext: void
	buffer: AHardwareBuffer
}
impl VkImportAndroidHardwareBufferInfoANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID;

        return s;
    }
}

pub struct VkAndroidHardwareBufferUsageANDROID
{
	sType: VkStructureType
	pNext: void
	androidHardwareBufferUsage: uint64_t
}
impl VkAndroidHardwareBufferUsageANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID;

        return s;
    }
}

pub struct VkAndroidHardwareBufferPropertiesANDROID
{
	sType: VkStructureType
	pNext: void
	allocationSize: VkDeviceSize
	memoryTypeBits: uint32_t
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

pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID
{
	sType: VkStructureType
	pNext: void
	memory: VkDeviceMemory
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

pub struct VkAndroidHardwareBufferFormatPropertiesANDROID
{
	sType: VkStructureType
	pNext: void
	format: VkFormat
	externalFormat: uint64_t
	formatFeatures: VkFormatFeatureFlags
	samplerYcbcrConversionComponents: VkComponentMapping
	suggestedYcbcrModel: VkSamplerYcbcrModelConversion
	suggestedYcbcrRange: VkSamplerYcbcrRange
	suggestedXChromaOffset: VkChromaLocation
	suggestedYChromaOffset: VkChromaLocation
}
impl VkAndroidHardwareBufferFormatPropertiesANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID;

        return s;
    }
}

pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT
{
	sType: VkStructureType
	pNext: void
	conditionalRenderingEnable: VkBool32
}
impl VkCommandBufferInheritanceConditionalRenderingInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT;

        return s;
    }
}

pub struct VkExternalFormatANDROID
{
	sType: VkStructureType
	pNext: void
	externalFormat: uint64_t
}
impl VkExternalFormatANDROID
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID;

        return s;
    }
}

pub struct VkPhysicalDevice8BitStorageFeatures
{
	sType: VkStructureType
	pNext: void
	storageBuffer8BitAccess: VkBool32
	uniformAndStorageBuffer8BitAccess: VkBool32
	storagePushConstant8: VkBool32
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

pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	conditionalRendering: VkBool32
	inheritedConditionalRendering: VkBool32
}
impl VkPhysicalDeviceConditionalRenderingFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceVulkanMemoryModelFeatures
{
	sType: VkStructureType
	pNext: void
	vulkanMemoryModel: VkBool32
	vulkanMemoryModelDeviceScope: VkBool32
	vulkanMemoryModelAvailabilityVisibilityChains: VkBool32
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

pub struct VkPhysicalDeviceShaderAtomicInt64Features
{
	sType: VkStructureType
	pNext: void
	shaderBufferInt64Atomics: VkBool32
	shaderSharedInt64Atomics: VkBool32
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

pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	shaderBufferFloat32Atomics: VkBool32
	shaderBufferFloat32AtomicAdd: VkBool32
	shaderBufferFloat64Atomics: VkBool32
	shaderBufferFloat64AtomicAdd: VkBool32
	shaderSharedFloat32Atomics: VkBool32
	shaderSharedFloat32AtomicAdd: VkBool32
	shaderSharedFloat64Atomics: VkBool32
	shaderSharedFloat64AtomicAdd: VkBool32
	shaderImageFloat32Atomics: VkBool32
	shaderImageFloat32AtomicAdd: VkBool32
	sparseImageFloat32Atomics: VkBool32
	sparseImageFloat32AtomicAdd: VkBool32
}
impl VkPhysicalDeviceShaderAtomicFloatFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT
{
	sType: VkStructureType
	pNext: void
	shaderBufferFloat16Atomics: VkBool32
	shaderBufferFloat16AtomicAdd: VkBool32
	shaderBufferFloat16AtomicMinMax: VkBool32
	shaderBufferFloat32AtomicMinMax: VkBool32
	shaderBufferFloat64AtomicMinMax: VkBool32
	shaderSharedFloat16Atomics: VkBool32
	shaderSharedFloat16AtomicAdd: VkBool32
	shaderSharedFloat16AtomicMinMax: VkBool32
	shaderSharedFloat32AtomicMinMax: VkBool32
	shaderSharedFloat64AtomicMinMax: VkBool32
	shaderImageFloat32AtomicMinMax: VkBool32
	sparseImageFloat32AtomicMinMax: VkBool32
}
impl VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	vertexAttributeInstanceRateDivisor: VkBool32
	vertexAttributeInstanceRateZeroDivisor: VkBool32
}
impl VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT;

        return s;
    }
}

pub struct VkQueueFamilyCheckpointPropertiesNV
{
	sType: VkStructureType
	pNext: void
	checkpointExecutionStageMask: VkPipelineStageFlags
}
impl VkQueueFamilyCheckpointPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV;

        return s;
    }
}

pub struct VkCheckpointDataNV
{
	sType: VkStructureType
	pNext: void
	stage: VkPipelineStageFlagBits
	pCheckpointMarker: void
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

pub struct VkPhysicalDeviceDepthStencilResolveProperties
{
	sType: VkStructureType
	pNext: void
	supportedDepthResolveModes: VkResolveModeFlags
	supportedStencilResolveModes: VkResolveModeFlags
	independentResolveNone: VkBool32
	independentResolve: VkBool32
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

pub struct VkSubpassDescriptionDepthStencilResolve
{
	sType: VkStructureType
	pNext: void
	depthResolveMode: VkResolveModeFlagBits
	stencilResolveMode: VkResolveModeFlagBits
	pDepthStencilResolveAttachment: VkAttachmentReference2
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

pub struct VkImageViewASTCDecodeModeEXT
{
	sType: VkStructureType
	pNext: void
	decodeMode: VkFormat
}
impl VkImageViewASTCDecodeModeEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	decodeModeSharedExponent: VkBool32
}
impl VkPhysicalDeviceASTCDecodeFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	transformFeedback: VkBool32
	geometryStreams: VkBool32
}
impl VkPhysicalDeviceTransformFeedbackFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	maxTransformFeedbackStreams: uint32_t
	maxTransformFeedbackBuffers: uint32_t
	maxTransformFeedbackBufferSize: VkDeviceSize
	maxTransformFeedbackStreamDataSize: uint32_t
	maxTransformFeedbackBufferDataSize: uint32_t
	maxTransformFeedbackBufferDataStride: uint32_t
	transformFeedbackQueries: VkBool32
	transformFeedbackStreamsLinesTriangles: VkBool32
	transformFeedbackRasterizationStreamSelect: VkBool32
	transformFeedbackDraw: VkBool32
}
impl VkPhysicalDeviceTransformFeedbackPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPipelineRasterizationStateStreamCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineRasterizationStateStreamCreateFlagsEXT
	rasterizationStream: uint32_t
}
impl VkPipelineRasterizationStateStreamCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV
{
	sType: VkStructureType
	pNext: void
	representativeFragmentTest: VkBool32
}
impl VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV;

        return s;
    }
}

pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	representativeFragmentTestEnable: VkBool32
}
impl VkPipelineRepresentativeFragmentTestStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV
{
	sType: VkStructureType
	pNext: void
	exclusiveScissor: VkBool32
}
impl VkPhysicalDeviceExclusiveScissorFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV;

        return s;
    }
}

pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	exclusiveScissorCount: uint32_t
	pExclusiveScissors: VkRect2D
}
impl VkPipelineViewportExclusiveScissorStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV
{
	sType: VkStructureType
	pNext: void
	cornerSampledImage: VkBool32
}
impl VkPhysicalDeviceCornerSampledImageFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV
{
	sType: VkStructureType
	pNext: void
	computeDerivativeGroupQuads: VkBool32
	computeDerivativeGroupLinear: VkBool32
}
impl VkPhysicalDeviceComputeShaderDerivativesFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV
{
	sType: VkStructureType
	pNext: void
	fragmentShaderBarycentric: VkBool32
}
impl VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV
{
	sType: VkStructureType
	pNext: void
	imageFootprint: VkBool32
}
impl VkPhysicalDeviceShaderImageFootprintFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV
{
	sType: VkStructureType
	pNext: void
	dedicatedAllocationImageAliasing: VkBool32
}
impl VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV;

        return s;
    }
}

pub struct VkShadingRatePaletteNV
{
	shadingRatePaletteEntryCount: uint32_t
	pShadingRatePaletteEntries: VkShadingRatePaletteEntryNV
}
impl VkShadingRatePaletteNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	shadingRateImageEnable: VkBool32
	viewportCount: uint32_t
	pShadingRatePalettes: VkShadingRatePaletteNV
}
impl VkPipelineViewportShadingRateImageStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceShadingRateImageFeaturesNV
{
	sType: VkStructureType
	pNext: void
	shadingRateImage: VkBool32
	shadingRateCoarseSampleOrder: VkBool32
}
impl VkPhysicalDeviceShadingRateImageFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceShadingRateImagePropertiesNV
{
	sType: VkStructureType
	pNext: void
	shadingRateTexelSize: VkExtent2D
	shadingRatePaletteSize: uint32_t
	shadingRateMaxCoarseSamples: uint32_t
}
impl VkPhysicalDeviceShadingRateImagePropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI
{
	sType: VkStructureType
	pNext: void
	invocationMask: VkBool32
}
impl VkPhysicalDeviceInvocationMaskFeaturesHUAWEI
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI;

        return s;
    }
}

pub struct VkCoarseSampleLocationNV
{
	pixelX: uint32_t
	pixelY: uint32_t
	sample: uint32_t
}
impl VkCoarseSampleLocationNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkCoarseSampleOrderCustomNV
{
	shadingRate: VkShadingRatePaletteEntryNV
	sampleCount: uint32_t
	sampleLocationCount: uint32_t
	pSampleLocations: VkCoarseSampleLocationNV
}
impl VkCoarseSampleOrderCustomNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	sampleOrderType: VkCoarseSampleOrderTypeNV
	customSampleOrderCount: uint32_t
	pCustomSampleOrders: VkCoarseSampleOrderCustomNV
}
impl VkPipelineViewportCoarseSampleOrderStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceMeshShaderFeaturesNV
{
	sType: VkStructureType
	pNext: void
	taskShader: VkBool32
	meshShader: VkBool32
}
impl VkPhysicalDeviceMeshShaderFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceMeshShaderPropertiesNV
{
	sType: VkStructureType
	pNext: void
	maxDrawMeshTasksCount: uint32_t
	maxTaskWorkGroupInvocations: uint32_t
	maxTaskWorkGroupSize: uint32_t
	maxTaskTotalMemorySize: uint32_t
	maxTaskOutputCount: uint32_t
	maxMeshWorkGroupInvocations: uint32_t
	maxMeshWorkGroupSize: uint32_t
	maxMeshTotalMemorySize: uint32_t
	maxMeshOutputVertices: uint32_t
	maxMeshOutputPrimitives: uint32_t
	maxMeshMultiviewViewCount: uint32_t
	meshOutputPerVertexGranularity: uint32_t
	meshOutputPerPrimitiveGranularity: uint32_t
}
impl VkPhysicalDeviceMeshShaderPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV;

        return s;
    }
}

pub struct VkDrawMeshTasksIndirectCommandNV
{
	taskCount: uint32_t
	firstTask: uint32_t
}
impl VkDrawMeshTasksIndirectCommandNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkRayTracingShaderGroupCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	type: VkRayTracingShaderGroupTypeKHR
	generalShader: uint32_t
	closestHitShader: uint32_t
	anyHitShader: uint32_t
	intersectionShader: uint32_t
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

pub struct VkRayTracingShaderGroupCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	type: VkRayTracingShaderGroupTypeKHR
	generalShader: uint32_t
	closestHitShader: uint32_t
	anyHitShader: uint32_t
	intersectionShader: uint32_t
	pShaderGroupCaptureReplayHandle: void
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

pub struct VkRayTracingPipelineCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineCreateFlags
	stageCount: uint32_t
	pStages: VkPipelineShaderStageCreateInfo
	groupCount: uint32_t
	pGroups: VkRayTracingShaderGroupCreateInfoNV
	maxRecursionDepth: uint32_t
	layout: VkPipelineLayout
	basePipelineHandle: VkPipeline
	basePipelineIndex: int32_t
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

pub struct VkRayTracingPipelineCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineCreateFlags
	stageCount: uint32_t
	pStages: VkPipelineShaderStageCreateInfo
	groupCount: uint32_t
	pGroups: VkRayTracingShaderGroupCreateInfoKHR
	maxPipelineRayRecursionDepth: uint32_t
	pLibraryInfo: VkPipelineLibraryCreateInfoKHR
	pLibraryInterface: VkRayTracingPipelineInterfaceCreateInfoKHR
	pDynamicState: VkPipelineDynamicStateCreateInfo
	layout: VkPipelineLayout
	basePipelineHandle: VkPipeline
	basePipelineIndex: int32_t
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

pub struct VkGeometryTrianglesNV
{
	sType: VkStructureType
	pNext: void
	vertexData: VkBuffer
	vertexOffset: VkDeviceSize
	vertexCount: uint32_t
	vertexStride: VkDeviceSize
	vertexFormat: VkFormat
	indexData: VkBuffer
	indexOffset: VkDeviceSize
	indexCount: uint32_t
	indexType: VkIndexType
	transformData: VkBuffer
	transformOffset: VkDeviceSize
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

pub struct VkGeometryAABBNV
{
	sType: VkStructureType
	pNext: void
	aabbData: VkBuffer
	numAABBs: uint32_t
	stride: uint32_t
	offset: VkDeviceSize
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

pub struct VkGeometryDataNV
{
	triangles: VkGeometryTrianglesNV
	aabbs: VkGeometryAABBNV
}
impl VkGeometryDataNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkGeometryNV
{
	sType: VkStructureType
	pNext: void
	geometryType: VkGeometryTypeKHR
	geometry: VkGeometryDataNV
	flags: VkGeometryFlagsKHR
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

pub struct VkAccelerationStructureInfoNV
{
	sType: VkStructureType
	pNext: void
	type: VkAccelerationStructureTypeNV
	flags: VkBuildAccelerationStructureFlagsNV
	instanceCount: uint32_t
	geometryCount: uint32_t
	pGeometries: VkGeometryNV
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

pub struct VkAccelerationStructureCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	compactedSize: VkDeviceSize
	info: VkAccelerationStructureInfoNV
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

pub struct VkBindAccelerationStructureMemoryInfoNV
{
	sType: VkStructureType
	pNext: void
	accelerationStructure: VkAccelerationStructureNV
	memory: VkDeviceMemory
	memoryOffset: VkDeviceSize
	deviceIndexCount: uint32_t
	pDeviceIndices: uint32_t
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

pub struct VkWriteDescriptorSetAccelerationStructureKHR
{
	sType: VkStructureType
	pNext: void
	accelerationStructureCount: uint32_t
	pAccelerationStructures: VkAccelerationStructureKHR
}
impl VkWriteDescriptorSetAccelerationStructureKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR;

        return s;
    }
}

pub struct VkWriteDescriptorSetAccelerationStructureNV
{
	sType: VkStructureType
	pNext: void
	accelerationStructureCount: uint32_t
	pAccelerationStructures: VkAccelerationStructureNV
}
impl VkWriteDescriptorSetAccelerationStructureNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV;

        return s;
    }
}

pub struct VkAccelerationStructureMemoryRequirementsInfoNV
{
	sType: VkStructureType
	pNext: void
	type: VkAccelerationStructureMemoryRequirementsTypeNV
	accelerationStructure: VkAccelerationStructureNV
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

pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	accelerationStructure: VkBool32
	accelerationStructureCaptureReplay: VkBool32
	accelerationStructureIndirectBuild: VkBool32
	accelerationStructureHostCommands: VkBool32
	descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32
}
impl VkPhysicalDeviceAccelerationStructureFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	rayTracingPipeline: VkBool32
	rayTracingPipelineShaderGroupHandleCaptureReplay: VkBool32
	rayTracingPipelineShaderGroupHandleCaptureReplayMixed: VkBool32
	rayTracingPipelineTraceRaysIndirect: VkBool32
	rayTraversalPrimitiveCulling: VkBool32
}
impl VkPhysicalDeviceRayTracingPipelineFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceRayQueryFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	rayQuery: VkBool32
}
impl VkPhysicalDeviceRayQueryFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR
{
	sType: VkStructureType
	pNext: void
	maxGeometryCount: uint64_t
	maxInstanceCount: uint64_t
	maxPrimitiveCount: uint64_t
	maxPerStageDescriptorAccelerationStructures: uint32_t
	maxPerStageDescriptorUpdateAfterBindAccelerationStructures: uint32_t
	maxDescriptorSetAccelerationStructures: uint32_t
	maxDescriptorSetUpdateAfterBindAccelerationStructures: uint32_t
	minAccelerationStructureScratchOffsetAlignment: uint32_t
}
impl VkPhysicalDeviceAccelerationStructurePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR
{
	sType: VkStructureType
	pNext: void
	shaderGroupHandleSize: uint32_t
	maxRayRecursionDepth: uint32_t
	maxShaderGroupStride: uint32_t
	shaderGroupBaseAlignment: uint32_t
	shaderGroupHandleCaptureReplaySize: uint32_t
	maxRayDispatchInvocationCount: uint32_t
	shaderGroupHandleAlignment: uint32_t
	maxRayHitAttributeSize: uint32_t
}
impl VkPhysicalDeviceRayTracingPipelinePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceRayTracingPropertiesNV
{
	sType: VkStructureType
	pNext: void
	shaderGroupHandleSize: uint32_t
	maxRecursionDepth: uint32_t
	maxShaderGroupStride: uint32_t
	shaderGroupBaseAlignment: uint32_t
	maxGeometryCount: uint64_t
	maxInstanceCount: uint64_t
	maxTriangleCount: uint64_t
	maxDescriptorSetAccelerationStructures: uint32_t
}
impl VkPhysicalDeviceRayTracingPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV;

        return s;
    }
}

pub struct VkStridedDeviceAddressRegionKHR
{
	deviceAddress: VkDeviceAddress
	stride: VkDeviceSize
	size: VkDeviceSize
}
impl VkStridedDeviceAddressRegionKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkTraceRaysIndirectCommandKHR
{
	width: uint32_t
	height: uint32_t
	depth: uint32_t
}
impl VkTraceRaysIndirectCommandKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkDrmFormatModifierPropertiesListEXT
{
	sType: VkStructureType
	pNext: void
	drmFormatModifierCount: uint32_t
	pDrmFormatModifierProperties: VkDrmFormatModifierPropertiesEXT
}
impl VkDrmFormatModifierPropertiesListEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT;

        return s;
    }
}

pub struct VkDrmFormatModifierPropertiesEXT
{
	drmFormatModifier: uint64_t
	drmFormatModifierPlaneCount: uint32_t
	drmFormatModifierTilingFeatures: VkFormatFeatureFlags
}
impl VkDrmFormatModifierPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT
{
	sType: VkStructureType
	pNext: void
	drmFormatModifier: uint64_t
	sharingMode: VkSharingMode
	queueFamilyIndexCount: uint32_t
	pQueueFamilyIndices: uint32_t
}
impl VkPhysicalDeviceImageDrmFormatModifierInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT;

        return s;
    }
}

pub struct VkImageDrmFormatModifierListCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	drmFormatModifierCount: uint32_t
	pDrmFormatModifiers: uint64_t
}
impl VkImageDrmFormatModifierListCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	drmFormatModifier: uint64_t
	drmFormatModifierPlaneCount: uint32_t
	pPlaneLayouts: VkSubresourceLayout
}
impl VkImageDrmFormatModifierExplicitCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkImageDrmFormatModifierPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	drmFormatModifier: uint64_t
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

pub struct VkImageStencilUsageCreateInfo
{
	sType: VkStructureType
	pNext: void
	stencilUsage: VkImageUsageFlags
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

pub struct VkDeviceMemoryOverallocationCreateInfoAMD
{
	sType: VkStructureType
	pNext: void
	overallocationBehavior: VkMemoryOverallocationBehaviorAMD
}
impl VkDeviceMemoryOverallocationCreateInfoAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	fragmentDensityMap: VkBool32
	fragmentDensityMapDynamic: VkBool32
	fragmentDensityMapNonSubsampledImages: VkBool32
}
impl VkPhysicalDeviceFragmentDensityMapFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT
{
	sType: VkStructureType
	pNext: void
	fragmentDensityMapDeferred: VkBool32
}
impl VkPhysicalDeviceFragmentDensityMap2FeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	minFragmentDensityTexelSize: VkExtent2D
	maxFragmentDensityTexelSize: VkExtent2D
	fragmentDensityInvocations: VkBool32
}
impl VkPhysicalDeviceFragmentDensityMapPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT
{
	sType: VkStructureType
	pNext: void
	subsampledLoads: VkBool32
	subsampledCoarseReconstructionEarlyAccess: VkBool32
	maxSubsampledArrayLayers: uint32_t
	maxDescriptorSetSubsampledSamplers: uint32_t
}
impl VkPhysicalDeviceFragmentDensityMap2PropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkRenderPassFragmentDensityMapCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	fragmentDensityMapAttachment: VkAttachmentReference
}
impl VkRenderPassFragmentDensityMapCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceScalarBlockLayoutFeatures
{
	sType: VkStructureType
	pNext: void
	scalarBlockLayout: VkBool32
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

pub struct VkSurfaceProtectedCapabilitiesKHR
{
	sType: VkStructureType
	pNext: void
	supportsProtected: VkBool32
}
impl VkSurfaceProtectedCapabilitiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures
{
	sType: VkStructureType
	pNext: void
	uniformBufferStandardLayout: VkBool32
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

pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	depthClipEnable: VkBool32
}
impl VkPhysicalDeviceDepthClipEnableFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT;

        return s;
    }
}

pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineRasterizationDepthClipStateCreateFlagsEXT
	depthClipEnable: VkBool32
}
impl VkPipelineRasterizationDepthClipStateCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	heapBudget: VkDeviceSize
	heapUsage: VkDeviceSize
}
impl VkPhysicalDeviceMemoryBudgetPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	memoryPriority: VkBool32
}
impl VkPhysicalDeviceMemoryPriorityFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT;

        return s;
    }
}

pub struct VkMemoryPriorityAllocateInfoEXT
{
	sType: VkStructureType
	pNext: void
	priority: float
}
impl VkMemoryPriorityAllocateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	pageableDeviceLocalMemory: VkBool32
}
impl VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceBufferDeviceAddressFeatures
{
	sType: VkStructureType
	pNext: void
	bufferDeviceAddress: VkBool32
	bufferDeviceAddressCaptureReplay: VkBool32
	bufferDeviceAddressMultiDevice: VkBool32
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

pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	bufferDeviceAddress: VkBool32
	bufferDeviceAddressCaptureReplay: VkBool32
	bufferDeviceAddressMultiDevice: VkBool32
}
impl VkPhysicalDeviceBufferDeviceAddressFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;

        return s;
    }
}

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

pub struct VkBufferDeviceAddressInfo
{
	sType: VkStructureType
	pNext: void
	buffer: VkBuffer
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

pub struct VkBufferOpaqueCaptureAddressCreateInfo
{
	sType: VkStructureType
	pNext: void
	opaqueCaptureAddress: uint64_t
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

pub struct VkBufferDeviceAddressCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	deviceAddress: VkDeviceAddress
}
impl VkBufferDeviceAddressCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT
{
	sType: VkStructureType
	pNext: void
	imageViewType: VkImageViewType
}
impl VkPhysicalDeviceImageViewImageFormatInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT;

        return s;
    }
}

pub struct VkFilterCubicImageViewImageFormatPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	filterCubic: VkBool32
	filterCubicMinmax: VkBool32
}
impl VkFilterCubicImageViewImageFormatPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceImagelessFramebufferFeatures
{
	sType: VkStructureType
	pNext: void
	imagelessFramebuffer: VkBool32
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

pub struct VkFramebufferAttachmentsCreateInfo
{
	sType: VkStructureType
	pNext: void
	attachmentImageInfoCount: uint32_t
	pAttachmentImageInfos: VkFramebufferAttachmentImageInfo
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

pub struct VkFramebufferAttachmentImageInfo
{
	sType: VkStructureType
	pNext: void
	flags: VkImageCreateFlags
	usage: VkImageUsageFlags
	width: uint32_t
	height: uint32_t
	layerCount: uint32_t
	viewFormatCount: uint32_t
	pViewFormats: VkFormat
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

pub struct VkRenderPassAttachmentBeginInfo
{
	sType: VkStructureType
	pNext: void
	attachmentCount: uint32_t
	pAttachments: VkImageView
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

pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	textureCompressionASTC_HDR: VkBool32
}
impl VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV
{
	sType: VkStructureType
	pNext: void
	cooperativeMatrix: VkBool32
	cooperativeMatrixRobustBufferAccess: VkBool32
}
impl VkPhysicalDeviceCooperativeMatrixFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV
{
	sType: VkStructureType
	pNext: void
	cooperativeMatrixSupportedStages: VkShaderStageFlags
}
impl VkPhysicalDeviceCooperativeMatrixPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV;

        return s;
    }
}

pub struct VkCooperativeMatrixPropertiesNV
{
	sType: VkStructureType
	pNext: void
	MSize: uint32_t
	NSize: uint32_t
	KSize: uint32_t
	AType: VkComponentTypeNV
	BType: VkComponentTypeNV
	CType: VkComponentTypeNV
	DType: VkComponentTypeNV
	scope: VkScopeNV
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

pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	ycbcrImageArrays: VkBool32
}
impl VkPhysicalDeviceYcbcrImageArraysFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT;

        return s;
    }
}

pub struct VkImageViewHandleInfoNVX
{
	sType: VkStructureType
	pNext: void
	imageView: VkImageView
	descriptorType: VkDescriptorType
	sampler: VkSampler
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

pub struct VkImageViewAddressPropertiesNVX
{
	sType: VkStructureType
	pNext: void
	deviceAddress: VkDeviceAddress
	size: VkDeviceSize
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

pub struct VkPresentFrameTokenGGP
{
	sType: VkStructureType
	pNext: void
	frameToken: GgpFrameToken
}
impl VkPresentFrameTokenGGP
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP;

        return s;
    }
}

pub struct VkPipelineCreationFeedbackEXT
{
	flags: VkPipelineCreationFeedbackFlagsEXT
	duration: uint64_t
}
impl VkPipelineCreationFeedbackEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkPipelineCreationFeedbackCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	pPipelineCreationFeedback: VkPipelineCreationFeedbackEXT
	pipelineStageCreationFeedbackCount: uint32_t
	pPipelineStageCreationFeedbacks: VkPipelineCreationFeedbackEXT
}
impl VkPipelineCreationFeedbackCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkSurfaceFullScreenExclusiveInfoEXT
{
	sType: VkStructureType
	pNext: void
	fullScreenExclusive: VkFullScreenExclusiveEXT
}
impl VkSurfaceFullScreenExclusiveInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT;

        return s;
    }
}

pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT
{
	sType: VkStructureType
	pNext: void
	hmonitor: HMONITOR
}
impl VkSurfaceFullScreenExclusiveWin32InfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT;

        return s;
    }
}

pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT
{
	sType: VkStructureType
	pNext: void
	fullScreenExclusiveSupported: VkBool32
}
impl VkSurfaceCapabilitiesFullScreenExclusiveEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT;

        return s;
    }
}

pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	performanceCounterQueryPools: VkBool32
	performanceCounterMultipleQueryPools: VkBool32
}
impl VkPhysicalDevicePerformanceQueryFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR
{
	sType: VkStructureType
	pNext: void
	allowCommandBufferQueryCopies: VkBool32
}
impl VkPhysicalDevicePerformanceQueryPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR;

        return s;
    }
}

pub struct VkPerformanceCounterKHR
{
	sType: VkStructureType
	pNext: void
	unit: VkPerformanceCounterUnitKHR
	scope: VkPerformanceCounterScopeKHR
	storage: VkPerformanceCounterStorageKHR
	uuid: uint8_t
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

pub struct VkPerformanceCounterDescriptionKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkPerformanceCounterDescriptionFlagsKHR
	name: char
	category: char
	description: char
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

pub struct VkQueryPoolPerformanceCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	queueFamilyIndex: uint32_t
	counterIndexCount: uint32_t
	pCounterIndices: uint32_t
}
impl VkQueryPoolPerformanceCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR;

        return s;
    }
}

pub struct VkAcquireProfilingLockInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkAcquireProfilingLockFlagsKHR
	timeout: uint64_t
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

pub struct VkPerformanceQuerySubmitInfoKHR
{
	sType: VkStructureType
	pNext: void
	counterPassIndex: uint32_t
}
impl VkPerformanceQuerySubmitInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR;

        return s;
    }
}

pub struct VkHeadlessSurfaceCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkHeadlessSurfaceCreateFlagsEXT
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

pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV
{
	sType: VkStructureType
	pNext: void
	coverageReductionMode: VkBool32
}
impl VkPhysicalDeviceCoverageReductionModeFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV;

        return s;
    }
}

pub struct VkPipelineCoverageReductionStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	flags: VkPipelineCoverageReductionStateCreateFlagsNV
	coverageReductionMode: VkCoverageReductionModeNV
}
impl VkPipelineCoverageReductionStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkFramebufferMixedSamplesCombinationNV
{
	sType: VkStructureType
	pNext: void
	coverageReductionMode: VkCoverageReductionModeNV
	rasterizationSamples: VkSampleCountFlagBits
	depthStencilSamples: VkSampleCountFlags
	colorSamples: VkSampleCountFlags
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

pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL
{
	sType: VkStructureType
	pNext: void
	shaderIntegerFunctions2: VkBool32
}
impl VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL;

        return s;
    }
}

pub struct VkPerformanceValueINTEL
{
	type: VkPerformanceValueTypeINTEL
	data: VkPerformanceValueDataINTEL
}
impl VkPerformanceValueINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkInitializePerformanceApiInfoINTEL
{
	sType: VkStructureType
	pNext: void
	pUserData: void
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

pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL
{
	sType: VkStructureType
	pNext: void
	performanceCountersSampling: VkQueryPoolSamplingModeINTEL
}
impl VkQueryPoolPerformanceQueryCreateInfoINTEL
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL;

        return s;
    }
}

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

pub struct VkPerformanceMarkerInfoINTEL
{
	sType: VkStructureType
	pNext: void
	marker: uint64_t
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

pub struct VkPerformanceStreamMarkerInfoINTEL
{
	sType: VkStructureType
	pNext: void
	marker: uint32_t
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

pub struct VkPerformanceOverrideInfoINTEL
{
	sType: VkStructureType
	pNext: void
	type: VkPerformanceOverrideTypeINTEL
	enable: VkBool32
	parameter: uint64_t
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

pub struct VkPerformanceConfigurationAcquireInfoINTEL
{
	sType: VkStructureType
	pNext: void
	type: VkPerformanceConfigurationTypeINTEL
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

pub struct VkPhysicalDeviceShaderClockFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	shaderSubgroupClock: VkBool32
	shaderDeviceClock: VkBool32
}
impl VkPhysicalDeviceShaderClockFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT
{
	sType: VkStructureType
	pNext: void
	indexTypeUint8: VkBool32
}
impl VkPhysicalDeviceIndexTypeUint8FeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV
{
	sType: VkStructureType
	pNext: void
	shaderSMCount: uint32_t
	shaderWarpsPerSM: uint32_t
}
impl VkPhysicalDeviceShaderSMBuiltinsPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV
{
	sType: VkStructureType
	pNext: void
	shaderSMBuiltins: VkBool32
}
impl VkPhysicalDeviceShaderSMBuiltinsFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	fragmentShaderSampleInterlock: VkBool32
	fragmentShaderPixelInterlock: VkBool32
	fragmentShaderShadingRateInterlock: VkBool32
}
impl VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures
{
	sType: VkStructureType
	pNext: void
	separateDepthStencilLayouts: VkBool32
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

pub struct VkAttachmentReferenceStencilLayout
{
	sType: VkStructureType
	pNext: void
	stencilLayout: VkImageLayout
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

pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	primitiveTopologyListRestart: VkBool32
	primitiveTopologyPatchListRestart: VkBool32
}
impl VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT;

        return s;
    }
}

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

pub struct VkAttachmentDescriptionStencilLayout
{
	sType: VkStructureType
	pNext: void
	stencilInitialLayout: VkImageLayout
	stencilFinalLayout: VkImageLayout
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

pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	pipelineExecutableInfo: VkBool32
}
impl VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR;

        return s;
    }
}

pub struct VkPipelineInfoKHR
{
	sType: VkStructureType
	pNext: void
	pipeline: VkPipeline
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

pub struct VkPipelineExecutablePropertiesKHR
{
	sType: VkStructureType
	pNext: void
	stages: VkShaderStageFlags
	name: char
	description: char
	subgroupSize: uint32_t
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

pub struct VkPipelineExecutableInfoKHR
{
	sType: VkStructureType
	pNext: void
	pipeline: VkPipeline
	executableIndex: uint32_t
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

pub struct VkPipelineExecutableStatisticKHR
{
	sType: VkStructureType
	pNext: void
	name: char
	description: char
	format: VkPipelineExecutableStatisticFormatKHR
	value: VkPipelineExecutableStatisticValueKHR
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

pub struct VkPipelineExecutableInternalRepresentationKHR
{
	sType: VkStructureType
	pNext: void
	name: char
	description: char
	isText: VkBool32
	dataSize: size_t
	pData: void
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

pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	shaderDemoteToHelperInvocation: VkBool32
}
impl VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	texelBufferAlignment: VkBool32
}
impl VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	storageTexelBufferOffsetAlignmentBytes: VkDeviceSize
	storageTexelBufferOffsetSingleTexelAlignment: VkBool32
	uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize
	uniformTexelBufferOffsetSingleTexelAlignment: VkBool32
}
impl VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceSubgroupSizeControlFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	subgroupSizeControl: VkBool32
	computeFullSubgroups: VkBool32
}
impl VkPhysicalDeviceSubgroupSizeControlFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceSubgroupSizeControlPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	minSubgroupSize: uint32_t
	maxSubgroupSize: uint32_t
	maxComputeWorkgroupSubgroups: uint32_t
	requiredSubgroupSizeStages: VkShaderStageFlags
}
impl VkPhysicalDeviceSubgroupSizeControlPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	requiredSubgroupSize: uint32_t
}
impl VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkSubpassShadingPipelineCreateInfoHUAWEI
{
	sType: VkStructureType
	pNext: void
	renderPass: VkRenderPass
	subpass: uint32_t
}
impl VkSubpassShadingPipelineCreateInfoHUAWEI
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI;

        return s;
    }
}

pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI
{
	sType: VkStructureType
	pNext: void
	maxSubpassShadingWorkgroupSizeAspectRatio: uint32_t
}
impl VkPhysicalDeviceSubpassShadingPropertiesHUAWEI
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI;

        return s;
    }
}

pub struct VkMemoryOpaqueCaptureAddressAllocateInfo
{
	sType: VkStructureType
	pNext: void
	opaqueCaptureAddress: uint64_t
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

pub struct VkDeviceMemoryOpaqueCaptureAddressInfo
{
	sType: VkStructureType
	pNext: void
	memory: VkDeviceMemory
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

pub struct VkPhysicalDeviceLineRasterizationFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	rectangularLines: VkBool32
	bresenhamLines: VkBool32
	smoothLines: VkBool32
	stippledRectangularLines: VkBool32
	stippledBresenhamLines: VkBool32
	stippledSmoothLines: VkBool32
}
impl VkPhysicalDeviceLineRasterizationFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceLineRasterizationPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	lineSubPixelPrecisionBits: uint32_t
}
impl VkPhysicalDeviceLineRasterizationPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPipelineRasterizationLineStateCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	lineRasterizationMode: VkLineRasterizationModeEXT
	stippledLineEnable: VkBool32
	lineStippleFactor: uint32_t
	lineStipplePattern: uint16_t
}
impl VkPipelineRasterizationLineStateCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	pipelineCreationCacheControl: VkBool32
}
impl VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceVulkan11Features
{
	sType: VkStructureType
	pNext: void
	storageBuffer16BitAccess: VkBool32
	uniformAndStorageBuffer16BitAccess: VkBool32
	storagePushConstant16: VkBool32
	storageInputOutput16: VkBool32
	multiview: VkBool32
	multiviewGeometryShader: VkBool32
	multiviewTessellationShader: VkBool32
	variablePointersStorageBuffer: VkBool32
	variablePointers: VkBool32
	protectedMemory: VkBool32
	samplerYcbcrConversion: VkBool32
	shaderDrawParameters: VkBool32
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

pub struct VkPhysicalDeviceVulkan11Properties
{
	sType: VkStructureType
	pNext: void
	deviceUUID: uint8_t
	driverUUID: uint8_t
	deviceLUID: uint8_t
	deviceNodeMask: uint32_t
	deviceLUIDValid: VkBool32
	subgroupSize: uint32_t
	subgroupSupportedStages: VkShaderStageFlags
	subgroupSupportedOperations: VkSubgroupFeatureFlags
	subgroupQuadOperationsInAllStages: VkBool32
	pointClippingBehavior: VkPointClippingBehavior
	maxMultiviewViewCount: uint32_t
	maxMultiviewInstanceIndex: uint32_t
	protectedNoFault: VkBool32
	maxPerSetDescriptors: uint32_t
	maxMemoryAllocationSize: VkDeviceSize
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

pub struct VkPhysicalDeviceVulkan12Features
{
	sType: VkStructureType
	pNext: void
	samplerMirrorClampToEdge: VkBool32
	drawIndirectCount: VkBool32
	storageBuffer8BitAccess: VkBool32
	uniformAndStorageBuffer8BitAccess: VkBool32
	storagePushConstant8: VkBool32
	shaderBufferInt64Atomics: VkBool32
	shaderSharedInt64Atomics: VkBool32
	shaderFloat16: VkBool32
	shaderInt8: VkBool32
	descriptorIndexing: VkBool32
	shaderInputAttachmentArrayDynamicIndexing: VkBool32
	shaderUniformTexelBufferArrayDynamicIndexing: VkBool32
	shaderStorageTexelBufferArrayDynamicIndexing: VkBool32
	shaderUniformBufferArrayNonUniformIndexing: VkBool32
	shaderSampledImageArrayNonUniformIndexing: VkBool32
	shaderStorageBufferArrayNonUniformIndexing: VkBool32
	shaderStorageImageArrayNonUniformIndexing: VkBool32
	shaderInputAttachmentArrayNonUniformIndexing: VkBool32
	shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32
	shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32
	descriptorBindingUniformBufferUpdateAfterBind: VkBool32
	descriptorBindingSampledImageUpdateAfterBind: VkBool32
	descriptorBindingStorageImageUpdateAfterBind: VkBool32
	descriptorBindingStorageBufferUpdateAfterBind: VkBool32
	descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32
	descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32
	descriptorBindingUpdateUnusedWhilePending: VkBool32
	descriptorBindingPartiallyBound: VkBool32
	descriptorBindingVariableDescriptorCount: VkBool32
	runtimeDescriptorArray: VkBool32
	samplerFilterMinmax: VkBool32
	scalarBlockLayout: VkBool32
	imagelessFramebuffer: VkBool32
	uniformBufferStandardLayout: VkBool32
	shaderSubgroupExtendedTypes: VkBool32
	separateDepthStencilLayouts: VkBool32
	hostQueryReset: VkBool32
	timelineSemaphore: VkBool32
	bufferDeviceAddress: VkBool32
	bufferDeviceAddressCaptureReplay: VkBool32
	bufferDeviceAddressMultiDevice: VkBool32
	vulkanMemoryModel: VkBool32
	vulkanMemoryModelDeviceScope: VkBool32
	vulkanMemoryModelAvailabilityVisibilityChains: VkBool32
	shaderOutputViewportIndex: VkBool32
	shaderOutputLayer: VkBool32
	subgroupBroadcastDynamicId: VkBool32
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

pub struct VkPhysicalDeviceVulkan12Properties
{
	sType: VkStructureType
	pNext: void
	driverID: VkDriverId
	driverName: char
	driverInfo: char
	conformanceVersion: VkConformanceVersion
	denormBehaviorIndependence: VkShaderFloatControlsIndependence
	roundingModeIndependence: VkShaderFloatControlsIndependence
	shaderSignedZeroInfNanPreserveFloat16: VkBool32
	shaderSignedZeroInfNanPreserveFloat32: VkBool32
	shaderSignedZeroInfNanPreserveFloat64: VkBool32
	shaderDenormPreserveFloat16: VkBool32
	shaderDenormPreserveFloat32: VkBool32
	shaderDenormPreserveFloat64: VkBool32
	shaderDenormFlushToZeroFloat16: VkBool32
	shaderDenormFlushToZeroFloat32: VkBool32
	shaderDenormFlushToZeroFloat64: VkBool32
	shaderRoundingModeRTEFloat16: VkBool32
	shaderRoundingModeRTEFloat32: VkBool32
	shaderRoundingModeRTEFloat64: VkBool32
	shaderRoundingModeRTZFloat16: VkBool32
	shaderRoundingModeRTZFloat32: VkBool32
	shaderRoundingModeRTZFloat64: VkBool32
	maxUpdateAfterBindDescriptorsInAllPools: uint32_t
	shaderUniformBufferArrayNonUniformIndexingNative: VkBool32
	shaderSampledImageArrayNonUniformIndexingNative: VkBool32
	shaderStorageBufferArrayNonUniformIndexingNative: VkBool32
	shaderStorageImageArrayNonUniformIndexingNative: VkBool32
	shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32
	robustBufferAccessUpdateAfterBind: VkBool32
	quadDivergentImplicitLod: VkBool32
	maxPerStageDescriptorUpdateAfterBindSamplers: uint32_t
	maxPerStageDescriptorUpdateAfterBindUniformBuffers: uint32_t
	maxPerStageDescriptorUpdateAfterBindStorageBuffers: uint32_t
	maxPerStageDescriptorUpdateAfterBindSampledImages: uint32_t
	maxPerStageDescriptorUpdateAfterBindStorageImages: uint32_t
	maxPerStageDescriptorUpdateAfterBindInputAttachments: uint32_t
	maxPerStageUpdateAfterBindResources: uint32_t
	maxDescriptorSetUpdateAfterBindSamplers: uint32_t
	maxDescriptorSetUpdateAfterBindUniformBuffers: uint32_t
	maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: uint32_t
	maxDescriptorSetUpdateAfterBindStorageBuffers: uint32_t
	maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: uint32_t
	maxDescriptorSetUpdateAfterBindSampledImages: uint32_t
	maxDescriptorSetUpdateAfterBindStorageImages: uint32_t
	maxDescriptorSetUpdateAfterBindInputAttachments: uint32_t
	supportedDepthResolveModes: VkResolveModeFlags
	supportedStencilResolveModes: VkResolveModeFlags
	independentResolveNone: VkBool32
	independentResolve: VkBool32
	filterMinmaxSingleComponentFormats: VkBool32
	filterMinmaxImageComponentMapping: VkBool32
	maxTimelineSemaphoreValueDifference: uint64_t
	framebufferIntegerColorSampleCounts: VkSampleCountFlags
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

pub struct VkPipelineCompilerControlCreateInfoAMD
{
	sType: VkStructureType
	pNext: void
	compilerControlFlags: VkPipelineCompilerControlFlagsAMD
}
impl VkPipelineCompilerControlCreateInfoAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD;

        return s;
    }
}

pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD
{
	sType: VkStructureType
	pNext: void
	deviceCoherentMemory: VkBool32
}
impl VkPhysicalDeviceCoherentMemoryFeaturesAMD
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD;

        return s;
    }
}

pub struct VkPhysicalDeviceToolPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	name: char
	version: char
	purposes: VkToolPurposeFlagsEXT
	description: char
	layer: char
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

pub struct VkSamplerCustomBorderColorCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	customBorderColor: VkClearColorValue
	format: VkFormat
}
impl VkSamplerCustomBorderColorCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	maxCustomBorderColorSamplers: uint32_t
}
impl VkPhysicalDeviceCustomBorderColorPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	customBorderColors: VkBool32
	customBorderColorWithoutFormat: VkBool32
}
impl VkPhysicalDeviceCustomBorderColorFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT;

        return s;
    }
}

pub struct VkAccelerationStructureGeometryTrianglesDataKHR
{
	sType: VkStructureType
	pNext: void
	vertexFormat: VkFormat
	vertexData: VkDeviceOrHostAddressConstKHR
	vertexStride: VkDeviceSize
	maxVertex: uint32_t
	indexType: VkIndexType
	indexData: VkDeviceOrHostAddressConstKHR
	transformData: VkDeviceOrHostAddressConstKHR
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

pub struct VkAccelerationStructureGeometryAabbsDataKHR
{
	sType: VkStructureType
	pNext: void
	data: VkDeviceOrHostAddressConstKHR
	stride: VkDeviceSize
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

pub struct VkAccelerationStructureGeometryInstancesDataKHR
{
	sType: VkStructureType
	pNext: void
	arrayOfPointers: VkBool32
	data: VkDeviceOrHostAddressConstKHR
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

pub struct VkAccelerationStructureGeometryKHR
{
	sType: VkStructureType
	pNext: void
	geometryType: VkGeometryTypeKHR
	geometry: VkAccelerationStructureGeometryDataKHR
	flags: VkGeometryFlagsKHR
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

pub struct VkAccelerationStructureBuildGeometryInfoKHR
{
	sType: VkStructureType
	pNext: void
	type: VkAccelerationStructureTypeKHR
	flags: VkBuildAccelerationStructureFlagsKHR
	mode: VkBuildAccelerationStructureModeKHR
	srcAccelerationStructure: VkAccelerationStructureKHR
	dstAccelerationStructure: VkAccelerationStructureKHR
	geometryCount: uint32_t
	pGeometries: VkAccelerationStructureGeometryKHR
	ppGeometries: VkAccelerationStructureGeometryKHR
	scratchData: VkDeviceOrHostAddressKHR
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

pub struct VkAccelerationStructureBuildRangeInfoKHR
{
	primitiveCount: uint32_t
	primitiveOffset: uint32_t
	firstVertex: uint32_t
	transformOffset: uint32_t
}
impl VkAccelerationStructureBuildRangeInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkAccelerationStructureCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	createFlags: VkAccelerationStructureCreateFlagsKHR
	buffer: VkBuffer
	offset: VkDeviceSize
	size: VkDeviceSize
	type: VkAccelerationStructureTypeKHR
	deviceAddress: VkDeviceAddress
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

pub struct VkAabbPositionsKHR
{
	minX: float
	minY: float
	minZ: float
	maxX: float
	maxY: float
	maxZ: float
}
impl VkAabbPositionsKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

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

pub struct VkTransformMatrixKHR
{
	matrix: float
}
impl VkTransformMatrixKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

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

pub struct VkAccelerationStructureInstanceKHR
{
	transform: VkTransformMatrixKHR
	instanceCustomIndex: uint32_t
	mask: uint32_t
	instanceShaderBindingTableRecordOffset: uint32_t
	flags: VkGeometryInstanceFlagsKHR
	accelerationStructureReference: uint64_t
}
impl VkAccelerationStructureInstanceKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

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

pub struct VkAccelerationStructureDeviceAddressInfoKHR
{
	sType: VkStructureType
	pNext: void
	accelerationStructure: VkAccelerationStructureKHR
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

pub struct VkAccelerationStructureVersionInfoKHR
{
	sType: VkStructureType
	pNext: void
	pVersionData: uint8_t
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

pub struct VkCopyAccelerationStructureInfoKHR
{
	sType: VkStructureType
	pNext: void
	src: VkAccelerationStructureKHR
	dst: VkAccelerationStructureKHR
	mode: VkCopyAccelerationStructureModeKHR
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

pub struct VkCopyAccelerationStructureToMemoryInfoKHR
{
	sType: VkStructureType
	pNext: void
	src: VkAccelerationStructureKHR
	dst: VkDeviceOrHostAddressKHR
	mode: VkCopyAccelerationStructureModeKHR
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

pub struct VkCopyMemoryToAccelerationStructureInfoKHR
{
	sType: VkStructureType
	pNext: void
	src: VkDeviceOrHostAddressConstKHR
	dst: VkAccelerationStructureKHR
	mode: VkCopyAccelerationStructureModeKHR
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

pub struct VkRayTracingPipelineInterfaceCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	maxPipelineRayPayloadSize: uint32_t
	maxPipelineRayHitAttributeSize: uint32_t
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

pub struct VkPipelineLibraryCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	libraryCount: uint32_t
	pLibraries: VkPipeline
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

pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	extendedDynamicState: VkBool32
}
impl VkPhysicalDeviceExtendedDynamicStateFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT
{
	sType: VkStructureType
	pNext: void
	extendedDynamicState2: VkBool32
	extendedDynamicState2LogicOp: VkBool32
	extendedDynamicState2PatchControlPoints: VkBool32
}
impl VkPhysicalDeviceExtendedDynamicState2FeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT;

        return s;
    }
}

pub struct VkRenderPassTransformBeginInfoQCOM
{
	sType: VkStructureType
	pNext: void
	transform: VkSurfaceTransformFlagBitsKHR
}
impl VkRenderPassTransformBeginInfoQCOM
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM;

        return s;
    }
}

pub struct VkCopyCommandTransformInfoQCOM
{
	sType: VkStructureType
	pNext: void
	transform: VkSurfaceTransformFlagBitsKHR
}
impl VkCopyCommandTransformInfoQCOM
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM;

        return s;
    }
}

pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM
{
	sType: VkStructureType
	pNext: void
	transform: VkSurfaceTransformFlagBitsKHR
	renderArea: VkRect2D
}
impl VkCommandBufferInheritanceRenderPassTransformInfoQCOM
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM;

        return s;
    }
}

pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV
{
	sType: VkStructureType
	pNext: void
	diagnosticsConfig: VkBool32
}
impl VkPhysicalDeviceDiagnosticsConfigFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV;

        return s;
    }
}

pub struct VkDeviceDiagnosticsConfigCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	flags: VkDeviceDiagnosticsConfigFlagsNV
}
impl VkDeviceDiagnosticsConfigCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	shaderZeroInitializeWorkgroupMemory: VkBool32
}
impl VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	shaderSubgroupUniformControlFlow: VkBool32
}
impl VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceRobustness2FeaturesEXT
{
	sType: VkStructureType
	pNext: void
	robustBufferAccess2: VkBool32
	robustImageAccess2: VkBool32
	nullDescriptor: VkBool32
}
impl VkPhysicalDeviceRobustness2FeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceRobustness2PropertiesEXT
{
	sType: VkStructureType
	pNext: void
	robustStorageBufferAccessSizeAlignment: VkDeviceSize
	robustUniformBufferAccessSizeAlignment: VkDeviceSize
}
impl VkPhysicalDeviceRobustness2PropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceImageRobustnessFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	robustImageAccess: VkBool32
}
impl VkPhysicalDeviceImageRobustnessFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	workgroupMemoryExplicitLayout: VkBool32
	workgroupMemoryExplicitLayoutScalarBlockLayout: VkBool32
	workgroupMemoryExplicitLayout8BitAccess: VkBool32
	workgroupMemoryExplicitLayout16BitAccess: VkBool32
}
impl VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDevicePortabilitySubsetFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	constantAlphaColorBlendFactors: VkBool32
	events: VkBool32
	imageViewFormatReinterpretation: VkBool32
	imageViewFormatSwizzle: VkBool32
	imageView2DOn3DImage: VkBool32
	multisampleArrayImage: VkBool32
	mutableComparisonSamplers: VkBool32
	pointPolygons: VkBool32
	samplerMipLodBias: VkBool32
	separateStencilMaskRef: VkBool32
	shaderSampleRateInterpolationFunctions: VkBool32
	tessellationIsolines: VkBool32
	tessellationPointMode: VkBool32
	triangleFans: VkBool32
	vertexAttributeAccessBeyondStride: VkBool32
}
impl VkPhysicalDevicePortabilitySubsetFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDevicePortabilitySubsetPropertiesKHR
{
	sType: VkStructureType
	pNext: void
	minVertexInputBindingStrideAlignment: uint32_t
}
impl VkPhysicalDevicePortabilitySubsetPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR;

        return s;
    }
}

pub struct VkPhysicalDevice4444FormatsFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	formatA4R4G4B4: VkBool32
	formatA4B4G4R4: VkBool32
}
impl VkPhysicalDevice4444FormatsFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI
{
	sType: VkStructureType
	pNext: void
	subpassShading: VkBool32
}
impl VkPhysicalDeviceSubpassShadingFeaturesHUAWEI
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI;

        return s;
    }
}

pub struct VkBufferCopy2KHR
{
	sType: VkStructureType
	pNext: void
	srcOffset: VkDeviceSize
	dstOffset: VkDeviceSize
	size: VkDeviceSize
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

pub struct VkImageCopy2KHR
{
	sType: VkStructureType
	pNext: void
	srcSubresource: VkImageSubresourceLayers
	srcOffset: VkOffset3D
	dstSubresource: VkImageSubresourceLayers
	dstOffset: VkOffset3D
	extent: VkExtent3D
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

pub struct VkImageBlit2KHR
{
	sType: VkStructureType
	pNext: void
	srcSubresource: VkImageSubresourceLayers
	srcOffsets: VkOffset3D
	dstSubresource: VkImageSubresourceLayers
	dstOffsets: VkOffset3D
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

pub struct VkBufferImageCopy2KHR
{
	sType: VkStructureType
	pNext: void
	bufferOffset: VkDeviceSize
	bufferRowLength: uint32_t
	bufferImageHeight: uint32_t
	imageSubresource: VkImageSubresourceLayers
	imageOffset: VkOffset3D
	imageExtent: VkExtent3D
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

pub struct VkImageResolve2KHR
{
	sType: VkStructureType
	pNext: void
	srcSubresource: VkImageSubresourceLayers
	srcOffset: VkOffset3D
	dstSubresource: VkImageSubresourceLayers
	dstOffset: VkOffset3D
	extent: VkExtent3D
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

pub struct VkCopyBufferInfo2KHR
{
	sType: VkStructureType
	pNext: void
	srcBuffer: VkBuffer
	dstBuffer: VkBuffer
	regionCount: uint32_t
	pRegions: VkBufferCopy2KHR
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

pub struct VkCopyImageInfo2KHR
{
	sType: VkStructureType
	pNext: void
	srcImage: VkImage
	srcImageLayout: VkImageLayout
	dstImage: VkImage
	dstImageLayout: VkImageLayout
	regionCount: uint32_t
	pRegions: VkImageCopy2KHR
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

pub struct VkBlitImageInfo2KHR
{
	sType: VkStructureType
	pNext: void
	srcImage: VkImage
	srcImageLayout: VkImageLayout
	dstImage: VkImage
	dstImageLayout: VkImageLayout
	regionCount: uint32_t
	pRegions: VkImageBlit2KHR
	filter: VkFilter
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

pub struct VkCopyBufferToImageInfo2KHR
{
	sType: VkStructureType
	pNext: void
	srcBuffer: VkBuffer
	dstImage: VkImage
	dstImageLayout: VkImageLayout
	regionCount: uint32_t
	pRegions: VkBufferImageCopy2KHR
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

pub struct VkCopyImageToBufferInfo2KHR
{
	sType: VkStructureType
	pNext: void
	srcImage: VkImage
	srcImageLayout: VkImageLayout
	dstBuffer: VkBuffer
	regionCount: uint32_t
	pRegions: VkBufferImageCopy2KHR
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

pub struct VkResolveImageInfo2KHR
{
	sType: VkStructureType
	pNext: void
	srcImage: VkImage
	srcImageLayout: VkImageLayout
	dstImage: VkImage
	dstImageLayout: VkImageLayout
	regionCount: uint32_t
	pRegions: VkImageResolve2KHR
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

pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT
{
	sType: VkStructureType
	pNext: void
	shaderImageInt64Atomics: VkBool32
	sparseImageInt64Atomics: VkBool32
}
impl VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT;

        return s;
    }
}

pub struct VkFragmentShadingRateAttachmentInfoKHR
{
	sType: VkStructureType
	pNext: void
	pFragmentShadingRateAttachment: VkAttachmentReference2
	shadingRateAttachmentTexelSize: VkExtent2D
}
impl VkFragmentShadingRateAttachmentInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR;

        return s;
    }
}

pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	fragmentSize: VkExtent2D
	combinerOps: VkFragmentShadingRateCombinerOpKHR
}
impl VkPipelineFragmentShadingRateStateCreateInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	pipelineFragmentShadingRate: VkBool32
	primitiveFragmentShadingRate: VkBool32
	attachmentFragmentShadingRate: VkBool32
}
impl VkPhysicalDeviceFragmentShadingRateFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR
{
	sType: VkStructureType
	pNext: void
	minFragmentShadingRateAttachmentTexelSize: VkExtent2D
	maxFragmentShadingRateAttachmentTexelSize: VkExtent2D
	maxFragmentShadingRateAttachmentTexelSizeAspectRatio: uint32_t
	primitiveFragmentShadingRateWithMultipleViewports: VkBool32
	layeredShadingRateAttachments: VkBool32
	fragmentShadingRateNonTrivialCombinerOps: VkBool32
	maxFragmentSize: VkExtent2D
	maxFragmentSizeAspectRatio: uint32_t
	maxFragmentShadingRateCoverageSamples: uint32_t
	maxFragmentShadingRateRasterizationSamples: VkSampleCountFlagBits
	fragmentShadingRateWithShaderDepthStencilWrites: VkBool32
	fragmentShadingRateWithSampleMask: VkBool32
	fragmentShadingRateWithShaderSampleMask: VkBool32
	fragmentShadingRateWithConservativeRasterization: VkBool32
	fragmentShadingRateWithFragmentShaderInterlock: VkBool32
	fragmentShadingRateWithCustomSampleLocations: VkBool32
	fragmentShadingRateStrictMultiplyCombiner: VkBool32
}
impl VkPhysicalDeviceFragmentShadingRatePropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentShadingRateKHR
{
	sType: VkStructureType
	pNext: void
	sampleCounts: VkSampleCountFlags
	fragmentSize: VkExtent2D
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

pub struct VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	shaderTerminateInvocation: VkBool32
}
impl VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV
{
	sType: VkStructureType
	pNext: void
	fragmentShadingRateEnums: VkBool32
	supersampleFragmentShadingRates: VkBool32
	noInvocationFragmentShadingRates: VkBool32
}
impl VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV
{
	sType: VkStructureType
	pNext: void
	maxFragmentShadingRateInvocationCount: VkSampleCountFlagBits
}
impl VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV;

        return s;
    }
}

pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV
{
	sType: VkStructureType
	pNext: void
	shadingRateType: VkFragmentShadingRateTypeNV
	shadingRate: VkFragmentShadingRateNV
	combinerOps: VkFragmentShadingRateCombinerOpKHR
}
impl VkPipelineFragmentShadingRateEnumStateCreateInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV;

        return s;
    }
}

pub struct VkAccelerationStructureBuildSizesInfoKHR
{
	sType: VkStructureType
	pNext: void
	accelerationStructureSize: VkDeviceSize
	updateScratchSize: VkDeviceSize
	buildScratchSize: VkDeviceSize
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

pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE
{
	sType: VkStructureType
	pNext: void
	mutableDescriptorType: VkBool32
}
impl VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE;

        return s;
    }
}

pub struct VkMutableDescriptorTypeListVALVE
{
	descriptorTypeCount: uint32_t
	pDescriptorTypes: VkDescriptorType
}
impl VkMutableDescriptorTypeListVALVE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkMutableDescriptorTypeCreateInfoVALVE
{
	sType: VkStructureType
	pNext: void
	mutableDescriptorTypeListCount: uint32_t
	pMutableDescriptorTypeLists: VkMutableDescriptorTypeListVALVE
}
impl VkMutableDescriptorTypeCreateInfoVALVE
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE;

        return s;
    }
}

pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	vertexInputDynamicState: VkBool32
}
impl VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV
{
	sType: VkStructureType
	pNext: void
	externalMemoryRDMA: VkBool32
}
impl VkPhysicalDeviceExternalMemoryRDMAFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV;

        return s;
    }
}

pub struct VkVertexInputBindingDescription2EXT
{
	sType: VkStructureType
	pNext: void
	binding: uint32_t
	stride: uint32_t
	inputRate: VkVertexInputRate
	divisor: uint32_t
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

pub struct VkVertexInputAttributeDescription2EXT
{
	sType: VkStructureType
	pNext: void
	location: uint32_t
	binding: uint32_t
	format: VkFormat
	offset: uint32_t
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

pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	colorWriteEnable: VkBool32
}
impl VkPhysicalDeviceColorWriteEnableFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT;

        return s;
    }
}

pub struct VkPipelineColorWriteCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	attachmentCount: uint32_t
	pColorWriteEnables: VkBool32
}
impl VkPipelineColorWriteCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkMemoryBarrier2KHR
{
	sType: VkStructureType
	pNext: void
	srcStageMask: VkPipelineStageFlags2KHR
	srcAccessMask: VkAccessFlags2KHR
	dstStageMask: VkPipelineStageFlags2KHR
	dstAccessMask: VkAccessFlags2KHR
}
impl VkMemoryBarrier2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR;

        return s;
    }
}

pub struct VkImageMemoryBarrier2KHR
{
	sType: VkStructureType
	pNext: void
	srcStageMask: VkPipelineStageFlags2KHR
	srcAccessMask: VkAccessFlags2KHR
	dstStageMask: VkPipelineStageFlags2KHR
	dstAccessMask: VkAccessFlags2KHR
	oldLayout: VkImageLayout
	newLayout: VkImageLayout
	srcQueueFamilyIndex: uint32_t
	dstQueueFamilyIndex: uint32_t
	image: VkImage
	subresourceRange: VkImageSubresourceRange
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

pub struct VkBufferMemoryBarrier2KHR
{
	sType: VkStructureType
	pNext: void
	srcStageMask: VkPipelineStageFlags2KHR
	srcAccessMask: VkAccessFlags2KHR
	dstStageMask: VkPipelineStageFlags2KHR
	dstAccessMask: VkAccessFlags2KHR
	srcQueueFamilyIndex: uint32_t
	dstQueueFamilyIndex: uint32_t
	buffer: VkBuffer
	offset: VkDeviceSize
	size: VkDeviceSize
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

pub struct VkDependencyInfoKHR
{
	sType: VkStructureType
	pNext: void
	dependencyFlags: VkDependencyFlags
	memoryBarrierCount: uint32_t
	pMemoryBarriers: VkMemoryBarrier2KHR
	bufferMemoryBarrierCount: uint32_t
	pBufferMemoryBarriers: VkBufferMemoryBarrier2KHR
	imageMemoryBarrierCount: uint32_t
	pImageMemoryBarriers: VkImageMemoryBarrier2KHR
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

pub struct VkSemaphoreSubmitInfoKHR
{
	sType: VkStructureType
	pNext: void
	semaphore: VkSemaphore
	value: uint64_t
	stageMask: VkPipelineStageFlags2KHR
	deviceIndex: uint32_t
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

pub struct VkCommandBufferSubmitInfoKHR
{
	sType: VkStructureType
	pNext: void
	commandBuffer: VkCommandBuffer
	deviceMask: uint32_t
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

pub struct VkSubmitInfo2KHR
{
	sType: VkStructureType
	pNext: void
	flags: VkSubmitFlagsKHR
	waitSemaphoreInfoCount: uint32_t
	pWaitSemaphoreInfos: VkSemaphoreSubmitInfoKHR
	commandBufferInfoCount: uint32_t
	pCommandBufferInfos: VkCommandBufferSubmitInfoKHR
	signalSemaphoreInfoCount: uint32_t
	pSignalSemaphoreInfos: VkSemaphoreSubmitInfoKHR
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

pub struct VkQueueFamilyCheckpointProperties2NV
{
	sType: VkStructureType
	pNext: void
	checkpointExecutionStageMask: VkPipelineStageFlags2KHR
}
impl VkQueueFamilyCheckpointProperties2NV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV;

        return s;
    }
}

pub struct VkCheckpointData2NV
{
	sType: VkStructureType
	pNext: void
	stage: VkPipelineStageFlags2KHR
	pCheckpointMarker: void
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

pub struct VkPhysicalDeviceSynchronization2FeaturesKHR
{
	sType: VkStructureType
	pNext: void
	synchronization2: VkBool32
}
impl VkPhysicalDeviceSynchronization2FeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR;

        return s;
    }
}

pub struct VkVideoQueueFamilyProperties2KHR
{
	sType: VkStructureType
	pNext: void
	videoCodecOperations: VkVideoCodecOperationFlagsKHR
}
impl VkVideoQueueFamilyProperties2KHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR;

        return s;
    }
}

pub struct VkVideoProfilesKHR
{
	sType: VkStructureType
	pNext: void
	profileCount: uint32_t
	pProfiles: VkVideoProfileKHR
}
impl VkVideoProfilesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_PROFILES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceVideoFormatInfoKHR
{
	sType: VkStructureType
	pNext: void
	imageUsage: VkImageUsageFlags
	pVideoProfiles: VkVideoProfilesKHR
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

pub struct VkVideoFormatPropertiesKHR
{
	sType: VkStructureType
	pNext: void
	format: VkFormat
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

pub struct VkVideoProfileKHR
{
	sType: VkStructureType
	pNext: void
	videoCodecOperation: VkVideoCodecOperationFlagBitsKHR
	chromaSubsampling: VkVideoChromaSubsamplingFlagsKHR
	lumaBitDepth: VkVideoComponentBitDepthFlagsKHR
	chromaBitDepth: VkVideoComponentBitDepthFlagsKHR
}
impl VkVideoProfileKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_PROFILE_KHR;

        return s;
    }
}

pub struct VkVideoCapabilitiesKHR
{
	sType: VkStructureType
	pNext: void
	capabilityFlags: VkVideoCapabilityFlagsKHR
	minBitstreamBufferOffsetAlignment: VkDeviceSize
	minBitstreamBufferSizeAlignment: VkDeviceSize
	videoPictureExtentGranularity: VkExtent2D
	minExtent: VkExtent2D
	maxExtent: VkExtent2D
	maxReferencePicturesSlotsCount: uint32_t
	maxReferencePicturesActiveCount: uint32_t
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

pub struct VkVideoGetMemoryPropertiesKHR
{
	sType: VkStructureType
	pNext: void
	memoryBindIndex: uint32_t
	pMemoryRequirements: VkMemoryRequirements2
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

pub struct VkVideoBindMemoryKHR
{
	sType: VkStructureType
	pNext: void
	memoryBindIndex: uint32_t
	memory: VkDeviceMemory
	memoryOffset: VkDeviceSize
	memorySize: VkDeviceSize
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

pub struct VkVideoPictureResourceKHR
{
	sType: VkStructureType
	pNext: void
	codedOffset: VkOffset2D
	codedExtent: VkExtent2D
	baseArrayLayer: uint32_t
	imageViewBinding: VkImageView
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

pub struct VkVideoReferenceSlotKHR
{
	sType: VkStructureType
	pNext: void
	slotIndex: int8_t
	pPictureResource: VkVideoPictureResourceKHR
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

pub struct VkVideoDecodeInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoDecodeFlagsKHR
	codedOffset: VkOffset2D
	codedExtent: VkExtent2D
	srcBuffer: VkBuffer
	srcBufferOffset: VkDeviceSize
	srcBufferRange: VkDeviceSize
	dstPictureResource: VkVideoPictureResourceKHR
	pSetupReferenceSlot: VkVideoReferenceSlotKHR
	referenceSlotCount: uint32_t
	pReferenceSlots: VkVideoReferenceSlotKHR
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

pub struct VkVideoDecodeH264ProfileEXT
{
	sType: VkStructureType
	pNext: void
	stdProfileIdc: StdVideoH264ProfileIdc
	pictureLayout: VkVideoDecodeH264PictureLayoutFlagsEXT
}
impl VkVideoDecodeH264ProfileEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PROFILE_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH264CapabilitiesEXT
{
	sType: VkStructureType
	pNext: void
	maxLevel: uint32_t
	fieldOffsetGranularity: VkOffset2D
	stdExtensionVersion: VkExtensionProperties
}
impl VkVideoDecodeH264CapabilitiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_CAPABILITIES_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH264SessionCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoDecodeH264CreateFlagsEXT
	pStdExtensionVersion: VkExtensionProperties
}
impl VkVideoDecodeH264SessionCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH264SessionParametersAddInfoEXT
{
	sType: VkStructureType
	pNext: void
	spsStdCount: uint32_t
	pSpsStd: StdVideoH264SequenceParameterSet
	ppsStdCount: uint32_t
	pPpsStd: StdVideoH264PictureParameterSet
}
impl VkVideoDecodeH264SessionParametersAddInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH264SessionParametersCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	maxSpsStdCount: uint32_t
	maxPpsStdCount: uint32_t
	pParametersAddInfo: VkVideoDecodeH264SessionParametersAddInfoEXT
}
impl VkVideoDecodeH264SessionParametersCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH264PictureInfoEXT
{
	sType: VkStructureType
	pNext: void
	pStdPictureInfo: StdVideoDecodeH264PictureInfo
	slicesCount: uint32_t
	pSlicesDataOffsets: uint32_t
}
impl VkVideoDecodeH264PictureInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_PICTURE_INFO_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH264DpbSlotInfoEXT
{
	sType: VkStructureType
	pNext: void
	pStdReferenceInfo: StdVideoDecodeH264ReferenceInfo
}
impl VkVideoDecodeH264DpbSlotInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH264MvcEXT
{
	sType: VkStructureType
	pNext: void
	pStdMvc: StdVideoDecodeH264Mvc
}
impl VkVideoDecodeH264MvcEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H264_MVC_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH265ProfileEXT
{
	sType: VkStructureType
	pNext: void
	stdProfileIdc: StdVideoH265ProfileIdc
}
impl VkVideoDecodeH265ProfileEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PROFILE_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH265CapabilitiesEXT
{
	sType: VkStructureType
	pNext: void
	maxLevel: uint32_t
	stdExtensionVersion: VkExtensionProperties
}
impl VkVideoDecodeH265CapabilitiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_CAPABILITIES_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH265SessionCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoDecodeH265CreateFlagsEXT
	pStdExtensionVersion: VkExtensionProperties
}
impl VkVideoDecodeH265SessionCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH265SessionParametersAddInfoEXT
{
	sType: VkStructureType
	pNext: void
	spsStdCount: uint32_t
	pSpsStd: StdVideoH265SequenceParameterSet
	ppsStdCount: uint32_t
	pPpsStd: StdVideoH265PictureParameterSet
}
impl VkVideoDecodeH265SessionParametersAddInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH265SessionParametersCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	maxSpsStdCount: uint32_t
	maxPpsStdCount: uint32_t
	pParametersAddInfo: VkVideoDecodeH265SessionParametersAddInfoEXT
}
impl VkVideoDecodeH265SessionParametersCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH265PictureInfoEXT
{
	sType: VkStructureType
	pNext: void
	pStdPictureInfo: StdVideoDecodeH265PictureInfo
	slicesCount: uint32_t
	pSlicesDataOffsets: uint32_t
}
impl VkVideoDecodeH265PictureInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_PICTURE_INFO_EXT;

        return s;
    }
}

pub struct VkVideoDecodeH265DpbSlotInfoEXT
{
	sType: VkStructureType
	pNext: void
	pStdReferenceInfo: StdVideoDecodeH265ReferenceInfo
}
impl VkVideoDecodeH265DpbSlotInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT;

        return s;
    }
}

pub struct VkVideoSessionCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	queueFamilyIndex: uint32_t
	flags: VkVideoSessionCreateFlagsKHR
	pVideoProfile: VkVideoProfileKHR
	pictureFormat: VkFormat
	maxCodedExtent: VkExtent2D
	referencePicturesFormat: VkFormat
	maxReferencePicturesSlotsCount: uint32_t
	maxReferencePicturesActiveCount: uint32_t
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

pub struct VkVideoSessionParametersCreateInfoKHR
{
	sType: VkStructureType
	pNext: void
	videoSessionParametersTemplate: VkVideoSessionParametersKHR
	videoSession: VkVideoSessionKHR
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

pub struct VkVideoSessionParametersUpdateInfoKHR
{
	sType: VkStructureType
	pNext: void
	updateSequenceCount: uint32_t
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

pub struct VkVideoBeginCodingInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoBeginCodingFlagsKHR
	codecQualityPreset: VkVideoCodingQualityPresetFlagsKHR
	videoSession: VkVideoSessionKHR
	videoSessionParameters: VkVideoSessionParametersKHR
	referenceSlotCount: uint32_t
	pReferenceSlots: VkVideoReferenceSlotKHR
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

pub struct VkVideoEndCodingInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoEndCodingFlagsKHR
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

pub struct VkVideoCodingControlInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoCodingControlFlagsKHR
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

pub struct VkVideoEncodeInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoEncodeFlagsKHR
	qualityLevel: uint32_t
	codedExtent: VkExtent2D
	dstBitstreamBuffer: VkBuffer
	dstBitstreamBufferOffset: VkDeviceSize
	dstBitstreamBufferMaxRange: VkDeviceSize
	srcPictureResource: VkVideoPictureResourceKHR
	pSetupReferenceSlot: VkVideoReferenceSlotKHR
	referenceSlotCount: uint32_t
	pReferenceSlots: VkVideoReferenceSlotKHR
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

pub struct VkVideoEncodeRateControlInfoKHR
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoEncodeRateControlFlagsKHR
	rateControlMode: VkVideoEncodeRateControlModeFlagBitsKHR
	averageBitrate: uint32_t
	peakToAverageBitrateRatio: uint16_t
	frameRateNumerator: uint16_t
	frameRateDenominator: uint16_t
	virtualBufferSizeInMs: uint32_t
}
impl VkVideoEncodeRateControlInfoKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_RATE_CONTROL_INFO_KHR;

        return s;
    }
}

pub struct VkVideoEncodeH264CapabilitiesEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoEncodeH264CapabilityFlagsEXT
	inputModeFlags: VkVideoEncodeH264InputModeFlagsEXT
	outputModeFlags: VkVideoEncodeH264OutputModeFlagsEXT
	minPictureSizeInMbs: VkExtent2D
	maxPictureSizeInMbs: VkExtent2D
	inputImageDataAlignment: VkExtent2D
	maxNumL0ReferenceForP: uint8_t
	maxNumL0ReferenceForB: uint8_t
	maxNumL1Reference: uint8_t
	qualityLevelCount: uint8_t
	stdExtensionVersion: VkExtensionProperties
}
impl VkVideoEncodeH264CapabilitiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_CAPABILITIES_EXT;

        return s;
    }
}

pub struct VkVideoEncodeH264SessionCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	flags: VkVideoEncodeH264CreateFlagsEXT
	maxPictureSizeInMbs: VkExtent2D
	pStdExtensionVersion: VkExtensionProperties
}
impl VkVideoEncodeH264SessionCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkVideoEncodeH264SessionParametersAddInfoEXT
{
	sType: VkStructureType
	pNext: void
	spsStdCount: uint32_t
	pSpsStd: StdVideoH264SequenceParameterSet
	ppsStdCount: uint32_t
	pPpsStd: StdVideoH264PictureParameterSet
}
impl VkVideoEncodeH264SessionParametersAddInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT;

        return s;
    }
}

pub struct VkVideoEncodeH264SessionParametersCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	maxSpsStdCount: uint32_t
	maxPpsStdCount: uint32_t
	pParametersAddInfo: VkVideoEncodeH264SessionParametersAddInfoEXT
}
impl VkVideoEncodeH264SessionParametersCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkVideoEncodeH264DpbSlotInfoEXT
{
	sType: VkStructureType
	pNext: void
	slotIndex: int8_t
	pStdPictureInfo: StdVideoEncodeH264PictureInfo
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

pub struct VkVideoEncodeH264VclFrameInfoEXT
{
	sType: VkStructureType
	pNext: void
	refDefaultFinalList0EntryCount: uint8_t
	pRefDefaultFinalList0Entries: VkVideoEncodeH264DpbSlotInfoEXT
	refDefaultFinalList1EntryCount: uint8_t
	pRefDefaultFinalList1Entries: VkVideoEncodeH264DpbSlotInfoEXT
	naluSliceEntryCount: uint32_t
	pNaluSliceEntries: VkVideoEncodeH264NaluSliceEXT
	pCurrentPictureInfo: VkVideoEncodeH264DpbSlotInfoEXT
}
impl VkVideoEncodeH264VclFrameInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT;

        return s;
    }
}

pub struct VkVideoEncodeH264EmitPictureParametersEXT
{
	sType: VkStructureType
	pNext: void
	spsId: uint8_t
	emitSpsEnable: VkBool32
	ppsIdEntryCount: uint32_t
	ppsIdEntries: uint8_t
}
impl VkVideoEncodeH264EmitPictureParametersEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT;

        return s;
    }
}

pub struct VkVideoEncodeH264ProfileEXT
{
	sType: VkStructureType
	pNext: void
	stdProfileIdc: StdVideoH264ProfileIdc
}
impl VkVideoEncodeH264ProfileEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_VIDEO_ENCODE_H264_PROFILE_EXT;

        return s;
    }
}

pub struct VkVideoEncodeH264NaluSliceEXT
{
	sType: VkStructureType
	pNext: void
	pSliceHeaderStd: StdVideoEncodeH264SliceHeader
	mbCount: uint32_t
	refFinalList0EntryCount: uint8_t
	pRefFinalList0Entries: VkVideoEncodeH264DpbSlotInfoEXT
	refFinalList1EntryCount: uint8_t
	pRefFinalList1Entries: VkVideoEncodeH264DpbSlotInfoEXT
	precedingNaluBytes: uint32_t
	minQp: uint8_t
	maxQp: uint8_t
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

pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV
{
	sType: VkStructureType
	pNext: void
	inheritedViewportScissor2D: VkBool32
}
impl VkPhysicalDeviceInheritedViewportScissorFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV;

        return s;
    }
}

pub struct VkCommandBufferInheritanceViewportScissorInfoNV
{
	sType: VkStructureType
	pNext: void
	viewportScissor2D: VkBool32
	viewportDepthCount: uint32_t
	pViewportDepths: VkViewport
}
impl VkCommandBufferInheritanceViewportScissorInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV;

        return s;
    }
}

pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	ycbcr2plane444Formats: VkBool32
}
impl VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT
{
	sType: VkStructureType
	pNext: void
	provokingVertexLast: VkBool32
	transformFeedbackPreservesProvokingVertex: VkBool32
}
impl VkPhysicalDeviceProvokingVertexFeaturesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	provokingVertexModePerPipeline: VkBool32
	transformFeedbackPreservesTriangleFanProvokingVertex: VkBool32
}
impl VkPhysicalDeviceProvokingVertexPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT
{
	sType: VkStructureType
	pNext: void
	provokingVertexMode: VkProvokingVertexModeEXT
}
impl VkPipelineRasterizationProvokingVertexStateCreateInfoEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT;

        return s;
    }
}

pub struct VkCuModuleCreateInfoNVX
{
	sType: VkStructureType
	pNext: void
	dataSize: size_t
	pData: void
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

pub struct VkCuFunctionCreateInfoNVX
{
	sType: VkStructureType
	pNext: void
	module: VkCuModuleNVX
	pName: char
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

pub struct VkCuLaunchInfoNVX
{
	sType: VkStructureType
	pNext: void
	function: VkCuFunctionNVX
	gridDimX: uint32_t
	gridDimY: uint32_t
	gridDimZ: uint32_t
	blockDimX: uint32_t
	blockDimY: uint32_t
	blockDimZ: uint32_t
	sharedMemBytes: uint32_t
	paramCount: size_t
	pParams: void
	extraCount: size_t
	pExtras: void
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

pub struct VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR
{
	sType: VkStructureType
	pNext: void
	shaderIntegerDotProduct: VkBool32
}
impl VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR
{
	sType: VkStructureType
	pNext: void
	integerDotProduct8BitUnsignedAccelerated: VkBool32
	integerDotProduct8BitSignedAccelerated: VkBool32
	integerDotProduct8BitMixedSignednessAccelerated: VkBool32
	integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32
	integerDotProduct4x8BitPackedSignedAccelerated: VkBool32
	integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32
	integerDotProduct16BitUnsignedAccelerated: VkBool32
	integerDotProduct16BitSignedAccelerated: VkBool32
	integerDotProduct16BitMixedSignednessAccelerated: VkBool32
	integerDotProduct32BitUnsignedAccelerated: VkBool32
	integerDotProduct32BitSignedAccelerated: VkBool32
	integerDotProduct32BitMixedSignednessAccelerated: VkBool32
	integerDotProduct64BitUnsignedAccelerated: VkBool32
	integerDotProduct64BitSignedAccelerated: VkBool32
	integerDotProduct64BitMixedSignednessAccelerated: VkBool32
	integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32
	integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32
	integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32
	integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32
	integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32
	integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32
}
impl VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR;

        return s;
    }
}

pub struct VkPhysicalDeviceDrmPropertiesEXT
{
	sType: VkStructureType
	pNext: void
	hasPrimary: VkBool32
	hasRender: VkBool32
	primaryMajor: int64_t
	primaryMinor: int64_t
	renderMajor: int64_t
	renderMinor: int64_t
}
impl VkPhysicalDeviceDrmPropertiesEXT
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT;

        return s;
    }
}

pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV
{
	sType: VkStructureType
	pNext: void
	rayTracingMotionBlur: VkBool32
	rayTracingMotionBlurPipelineTraceRaysIndirect: VkBool32
}
impl VkPhysicalDeviceRayTracingMotionBlurFeaturesNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV;

        return s;
    }
}

pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV
{
	sType: VkStructureType
	pNext: void
	vertexData: VkDeviceOrHostAddressConstKHR
}
impl VkAccelerationStructureGeometryMotionTrianglesDataNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV;

        return s;
    }
}

pub struct VkAccelerationStructureMotionInfoNV
{
	sType: VkStructureType
	pNext: void
	maxInstances: uint32_t
	flags: VkAccelerationStructureMotionInfoFlagsNV
}
impl VkAccelerationStructureMotionInfoNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        s.sType = VkStructureType::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV;

        return s;
    }
}

pub struct VkSRTDataNV
{
	sx: float
	a: float
	b: float
	pvx: float
	sy: float
	c: float
	pvy: float
	sz: float
	pvz: float
	qx: float
	qy: float
	qz: float
	qw: float
	tx: float
	ty: float
	tz: float
}
impl VkSRTDataNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkAccelerationStructureSRTMotionInstanceNV
{
	transformT0: VkSRTDataNV
	transformT1: VkSRTDataNV
	instanceCustomIndex: uint32_t
	mask: uint32_t
	instanceShaderBindingTableRecordOffset: uint32_t
	flags: VkGeometryInstanceFlagsKHR
	accelerationStructureReference: uint64_t
}
impl VkAccelerationStructureSRTMotionInstanceNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkAccelerationStructureMatrixMotionInstanceNV
{
	transformT0: VkTransformMatrixKHR
	transformT1: VkTransformMatrixKHR
	instanceCustomIndex: uint32_t
	mask: uint32_t
	instanceShaderBindingTableRecordOffset: uint32_t
	flags: VkGeometryInstanceFlagsKHR
	accelerationStructureReference: uint64_t
}
impl VkAccelerationStructureMatrixMotionInstanceNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkAccelerationStructureMotionInstanceNV
{
	type: VkAccelerationStructureMotionInstanceTypeNV
	flags: VkAccelerationStructureMotionInstanceFlagsNV
	data: VkAccelerationStructureMotionInstanceDataNV
}
impl VkAccelerationStructureMotionInstanceNV
{
    fn new() -> Self
    {
        let mut s: Self = unsafe { mem::zeroed() };
        
        return s;
    }
}

pub struct VkMemoryGetRemoteAddressInfoNV
{
	sType: VkStructureType
	pNext: void
	memory: VkDeviceMemory
	handleType: VkExternalMemoryHandleTypeFlagBits
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

