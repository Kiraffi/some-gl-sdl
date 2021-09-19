pub struct VkBaseOutStructure
{
    sType: VkStructureType;
    pNext: VkBaseOutStructure;
}
pub struct VkBaseInStructure
{
    sType: VkStructureType;
    pNext: VkBaseInStructure;
}
pub struct VkOffset2D
{
    x: int32_t;
    y: int32_t;
}
pub struct VkOffset3D
{
    x: int32_t;
    y: int32_t;
    z: int32_t;
}
pub struct VkExtent2D
{
    width: uint32_t;
    height: uint32_t;
}
pub struct VkExtent3D
{
    width: uint32_t;
    height: uint32_t;
    depth: uint32_t;
}
pub struct VkViewport
{
    x: float;
    y: float;
    width: float;
    height: float;
    minDepth: float;
    maxDepth: float;
}
pub struct VkRect2D
{
    offset: VkOffset2D;
    extent: VkExtent2D;
}
pub struct VkClearRect
{
    rect: VkRect2D;
    baseArrayLayer: uint32_t;
    layerCount: uint32_t;
}
pub struct VkComponentMapping
{
    r: VkComponentSwizzle;
    g: VkComponentSwizzle;
    b: VkComponentSwizzle;
    a: VkComponentSwizzle;
}
pub struct VkPhysicalDeviceProperties
{
    apiVersion: uint32_t;
    driverVersion: uint32_t;
    vendorID: uint32_t;
    deviceID: uint32_t;
    deviceType: VkPhysicalDeviceType;
    deviceName: char;
    pipelineCacheUUID: uint8_t;
    limits: VkPhysicalDeviceLimits;
    sparseProperties: VkPhysicalDeviceSparseProperties;
}
pub struct VkExtensionProperties
{
    extensionName: char;
    specVersion: uint32_t;
}
pub struct VkLayerProperties
{
    layerName: char;
    specVersion: uint32_t;
    implementationVersion: uint32_t;
    description: char;
}
pub struct VkApplicationInfo
{
    sType: VkStructureType;
    pNext: void;
    pApplicationName: char;
    applicationVersion: uint32_t;
    pEngineName: char;
    engineVersion: uint32_t;
    apiVersion: uint32_t;
}
pub struct VkAllocationCallbacks
{
    pUserData: void;
    pfnAllocation: PFN_vkAllocationFunction;
    pfnReallocation: PFN_vkReallocationFunction;
    pfnFree: PFN_vkFreeFunction;
    pfnInternalAllocation: PFN_vkInternalAllocationNotification;
    pfnInternalFree: PFN_vkInternalFreeNotification;
}
pub struct VkDeviceQueueCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDeviceQueueCreateFlags;
    queueFamilyIndex: uint32_t;
    queueCount: uint32_t;
    pQueuePriorities: float;
}
pub struct VkDeviceCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDeviceCreateFlags;
    queueCreateInfoCount: uint32_t;
    pQueueCreateInfos: VkDeviceQueueCreateInfo;
    enabledLayerCount: uint32_t;
    ppEnabledLayerNames: char;
    enabledExtensionCount: uint32_t;
    ppEnabledExtensionNames: char;
    pEnabledFeatures: VkPhysicalDeviceFeatures;
}
pub struct VkInstanceCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkInstanceCreateFlags;
    pApplicationInfo: VkApplicationInfo;
    enabledLayerCount: uint32_t;
    ppEnabledLayerNames: char;
    enabledExtensionCount: uint32_t;
    ppEnabledExtensionNames: char;
}
pub struct VkQueueFamilyProperties
{
    queueFlags: VkQueueFlags;
    queueCount: uint32_t;
    timestampValidBits: uint32_t;
    minImageTransferGranularity: VkExtent3D;
}
pub struct VkPhysicalDeviceMemoryProperties
{
    memoryTypeCount: uint32_t;
    memoryTypes: VkMemoryType;
    memoryHeapCount: uint32_t;
    memoryHeaps: VkMemoryHeap;
}
pub struct VkMemoryAllocateInfo
{
    sType: VkStructureType;
    pNext: void;
    allocationSize: VkDeviceSize;
    memoryTypeIndex: uint32_t;
}
pub struct VkMemoryRequirements
{
    size: VkDeviceSize;
    alignment: VkDeviceSize;
    memoryTypeBits: uint32_t;
}
pub struct VkSparseImageFormatProperties
{
    aspectMask: VkImageAspectFlags;
    imageGranularity: VkExtent3D;
    flags: VkSparseImageFormatFlags;
}
pub struct VkSparseImageMemoryRequirements
{
    formatProperties: VkSparseImageFormatProperties;
    imageMipTailFirstLod: uint32_t;
    imageMipTailSize: VkDeviceSize;
    imageMipTailOffset: VkDeviceSize;
    imageMipTailStride: VkDeviceSize;
}
pub struct VkMemoryType
{
    propertyFlags: VkMemoryPropertyFlags;
    heapIndex: uint32_t;
}
pub struct VkMemoryHeap
{
    size: VkDeviceSize;
    flags: VkMemoryHeapFlags;
}
pub struct VkMappedMemoryRange
{
    sType: VkStructureType;
    pNext: void;
    memory: VkDeviceMemory;
    offset: VkDeviceSize;
    size: VkDeviceSize;
}
pub struct VkFormatProperties
{
    linearTilingFeatures: VkFormatFeatureFlags;
    optimalTilingFeatures: VkFormatFeatureFlags;
    bufferFeatures: VkFormatFeatureFlags;
}
pub struct VkImageFormatProperties
{
    maxExtent: VkExtent3D;
    maxMipLevels: uint32_t;
    maxArrayLayers: uint32_t;
    sampleCounts: VkSampleCountFlags;
    maxResourceSize: VkDeviceSize;
}
pub struct VkDescriptorBufferInfo
{
    buffer: VkBuffer;
    offset: VkDeviceSize;
    range: VkDeviceSize;
}
pub struct VkDescriptorImageInfo
{
    sampler: VkSampler;
    imageView: VkImageView;
    imageLayout: VkImageLayout;
}
pub struct VkWriteDescriptorSet
{
    sType: VkStructureType;
    pNext: void;
    dstSet: VkDescriptorSet;
    dstBinding: uint32_t;
    dstArrayElement: uint32_t;
    descriptorCount: uint32_t;
    descriptorType: VkDescriptorType;
    pImageInfo: VkDescriptorImageInfo;
    pBufferInfo: VkDescriptorBufferInfo;
    pTexelBufferView: VkBufferView;
}
pub struct VkCopyDescriptorSet
{
    sType: VkStructureType;
    pNext: void;
    srcSet: VkDescriptorSet;
    srcBinding: uint32_t;
    srcArrayElement: uint32_t;
    dstSet: VkDescriptorSet;
    dstBinding: uint32_t;
    dstArrayElement: uint32_t;
    descriptorCount: uint32_t;
}
pub struct VkBufferCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkBufferCreateFlags;
    size: VkDeviceSize;
    usage: VkBufferUsageFlags;
    sharingMode: VkSharingMode;
    queueFamilyIndexCount: uint32_t;
    pQueueFamilyIndices: uint32_t;
}
pub struct VkBufferViewCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkBufferViewCreateFlags;
    buffer: VkBuffer;
    format: VkFormat;
    offset: VkDeviceSize;
    range: VkDeviceSize;
}
pub struct VkImageSubresource
{
    aspectMask: VkImageAspectFlags;
    mipLevel: uint32_t;
    arrayLayer: uint32_t;
}
pub struct VkImageSubresourceLayers
{
    aspectMask: VkImageAspectFlags;
    mipLevel: uint32_t;
    baseArrayLayer: uint32_t;
    layerCount: uint32_t;
}
pub struct VkImageSubresourceRange
{
    aspectMask: VkImageAspectFlags;
    baseMipLevel: uint32_t;
    levelCount: uint32_t;
    baseArrayLayer: uint32_t;
    layerCount: uint32_t;
}
pub struct VkMemoryBarrier
{
    sType: VkStructureType;
    pNext: void;
    srcAccessMask: VkAccessFlags;
    dstAccessMask: VkAccessFlags;
}
pub struct VkBufferMemoryBarrier
{
    sType: VkStructureType;
    pNext: void;
    srcAccessMask: VkAccessFlags;
    dstAccessMask: VkAccessFlags;
    srcQueueFamilyIndex: uint32_t;
    dstQueueFamilyIndex: uint32_t;
    buffer: VkBuffer;
    offset: VkDeviceSize;
    size: VkDeviceSize;
}
pub struct VkImageMemoryBarrier
{
    sType: VkStructureType;
    pNext: void;
    srcAccessMask: VkAccessFlags;
    dstAccessMask: VkAccessFlags;
    oldLayout: VkImageLayout;
    newLayout: VkImageLayout;
    srcQueueFamilyIndex: uint32_t;
    dstQueueFamilyIndex: uint32_t;
    image: VkImage;
    subresourceRange: VkImageSubresourceRange;
}
pub struct VkImageCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkImageCreateFlags;
    imageType: VkImageType;
    format: VkFormat;
    extent: VkExtent3D;
    mipLevels: uint32_t;
    arrayLayers: uint32_t;
    samples: VkSampleCountFlagBits;
    tiling: VkImageTiling;
    usage: VkImageUsageFlags;
    sharingMode: VkSharingMode;
    queueFamilyIndexCount: uint32_t;
    pQueueFamilyIndices: uint32_t;
    initialLayout: VkImageLayout;
}
pub struct VkSubresourceLayout
{
    offset: VkDeviceSize;
    size: VkDeviceSize;
    rowPitch: VkDeviceSize;
    arrayPitch: VkDeviceSize;
    depthPitch: VkDeviceSize;
}
pub struct VkImageViewCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkImageViewCreateFlags;
    image: VkImage;
    viewType: VkImageViewType;
    format: VkFormat;
    components: VkComponentMapping;
    subresourceRange: VkImageSubresourceRange;
}
pub struct VkBufferCopy
{
    srcOffset: VkDeviceSize;
    dstOffset: VkDeviceSize;
    size: VkDeviceSize;
}
pub struct VkSparseMemoryBind
{
    resourceOffset: VkDeviceSize;
    size: VkDeviceSize;
    memory: VkDeviceMemory;
    memoryOffset: VkDeviceSize;
    flags: VkSparseMemoryBindFlags;
}
pub struct VkSparseImageMemoryBind
{
    subresource: VkImageSubresource;
    offset: VkOffset3D;
    extent: VkExtent3D;
    memory: VkDeviceMemory;
    memoryOffset: VkDeviceSize;
    flags: VkSparseMemoryBindFlags;
}
pub struct VkSparseBufferMemoryBindInfo
{
    buffer: VkBuffer;
    bindCount: uint32_t;
    pBinds: VkSparseMemoryBind;
}
pub struct VkSparseImageOpaqueMemoryBindInfo
{
    image: VkImage;
    bindCount: uint32_t;
    pBinds: VkSparseMemoryBind;
}
pub struct VkSparseImageMemoryBindInfo
{
    image: VkImage;
    bindCount: uint32_t;
    pBinds: VkSparseImageMemoryBind;
}
pub struct VkBindSparseInfo
{
    sType: VkStructureType;
    pNext: void;
    waitSemaphoreCount: uint32_t;
    pWaitSemaphores: VkSemaphore;
    bufferBindCount: uint32_t;
    pBufferBinds: VkSparseBufferMemoryBindInfo;
    imageOpaqueBindCount: uint32_t;
    pImageOpaqueBinds: VkSparseImageOpaqueMemoryBindInfo;
    imageBindCount: uint32_t;
    pImageBinds: VkSparseImageMemoryBindInfo;
    signalSemaphoreCount: uint32_t;
    pSignalSemaphores: VkSemaphore;
}
pub struct VkImageCopy
{
    srcSubresource: VkImageSubresourceLayers;
    srcOffset: VkOffset3D;
    dstSubresource: VkImageSubresourceLayers;
    dstOffset: VkOffset3D;
    extent: VkExtent3D;
}
pub struct VkImageBlit
{
    srcSubresource: VkImageSubresourceLayers;
    srcOffsets: VkOffset3D;
    dstSubresource: VkImageSubresourceLayers;
    dstOffsets: VkOffset3D;
}
pub struct VkBufferImageCopy
{
    bufferOffset: VkDeviceSize;
    bufferRowLength: uint32_t;
    bufferImageHeight: uint32_t;
    imageSubresource: VkImageSubresourceLayers;
    imageOffset: VkOffset3D;
    imageExtent: VkExtent3D;
}
pub struct VkImageResolve
{
    srcSubresource: VkImageSubresourceLayers;
    srcOffset: VkOffset3D;
    dstSubresource: VkImageSubresourceLayers;
    dstOffset: VkOffset3D;
    extent: VkExtent3D;
}
pub struct VkShaderModuleCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkShaderModuleCreateFlags;
    codeSize: size_t;
    pCode: uint32_t;
}
pub struct VkDescriptorSetLayoutBinding
{
    binding: uint32_t;
    descriptorType: VkDescriptorType;
    descriptorCount: uint32_t;
    stageFlags: VkShaderStageFlags;
    pImmutableSamplers: VkSampler;
}
pub struct VkDescriptorSetLayoutCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDescriptorSetLayoutCreateFlags;
    bindingCount: uint32_t;
    pBindings: VkDescriptorSetLayoutBinding;
}
pub struct VkDescriptorPoolSize
{
    type: VkDescriptorType;
    descriptorCount: uint32_t;
}
pub struct VkDescriptorPoolCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDescriptorPoolCreateFlags;
    maxSets: uint32_t;
    poolSizeCount: uint32_t;
    pPoolSizes: VkDescriptorPoolSize;
}
pub struct VkDescriptorSetAllocateInfo
{
    sType: VkStructureType;
    pNext: void;
    descriptorPool: VkDescriptorPool;
    descriptorSetCount: uint32_t;
    pSetLayouts: VkDescriptorSetLayout;
}
pub struct VkSpecializationMapEntry
{
    constantID: uint32_t;
    offset: uint32_t;
    size: size_t;
}
pub struct VkSpecializationInfo
{
    mapEntryCount: uint32_t;
    pMapEntries: VkSpecializationMapEntry;
    dataSize: size_t;
    pData: void;
}
pub struct VkPipelineShaderStageCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineShaderStageCreateFlags;
    stage: VkShaderStageFlagBits;
    module: VkShaderModule;
    pName: char;
    pSpecializationInfo: VkSpecializationInfo;
}
pub struct VkComputePipelineCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineCreateFlags;
    stage: VkPipelineShaderStageCreateInfo;
    layout: VkPipelineLayout;
    basePipelineHandle: VkPipeline;
    basePipelineIndex: int32_t;
}
pub struct VkVertexInputBindingDescription
{
    binding: uint32_t;
    stride: uint32_t;
    inputRate: VkVertexInputRate;
}
pub struct VkVertexInputAttributeDescription
{
    location: uint32_t;
    binding: uint32_t;
    format: VkFormat;
    offset: uint32_t;
}
pub struct VkPipelineVertexInputStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineVertexInputStateCreateFlags;
    vertexBindingDescriptionCount: uint32_t;
    pVertexBindingDescriptions: VkVertexInputBindingDescription;
    vertexAttributeDescriptionCount: uint32_t;
    pVertexAttributeDescriptions: VkVertexInputAttributeDescription;
}
pub struct VkPipelineInputAssemblyStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineInputAssemblyStateCreateFlags;
    topology: VkPrimitiveTopology;
    primitiveRestartEnable: VkBool32;
}
pub struct VkPipelineTessellationStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineTessellationStateCreateFlags;
    patchControlPoints: uint32_t;
}
pub struct VkPipelineViewportStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineViewportStateCreateFlags;
    viewportCount: uint32_t;
    pViewports: VkViewport;
    scissorCount: uint32_t;
    pScissors: VkRect2D;
}
pub struct VkPipelineRasterizationStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineRasterizationStateCreateFlags;
    depthClampEnable: VkBool32;
    rasterizerDiscardEnable: VkBool32;
    polygonMode: VkPolygonMode;
    cullMode: VkCullModeFlags;
    frontFace: VkFrontFace;
    depthBiasEnable: VkBool32;
    depthBiasConstantFactor: float;
    depthBiasClamp: float;
    depthBiasSlopeFactor: float;
    lineWidth: float;
}
pub struct VkPipelineMultisampleStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineMultisampleStateCreateFlags;
    rasterizationSamples: VkSampleCountFlagBits;
    sampleShadingEnable: VkBool32;
    minSampleShading: float;
    pSampleMask: VkSampleMask;
    alphaToCoverageEnable: VkBool32;
    alphaToOneEnable: VkBool32;
}
pub struct VkPipelineColorBlendAttachmentState
{
    blendEnable: VkBool32;
    srcColorBlendFactor: VkBlendFactor;
    dstColorBlendFactor: VkBlendFactor;
    colorBlendOp: VkBlendOp;
    srcAlphaBlendFactor: VkBlendFactor;
    dstAlphaBlendFactor: VkBlendFactor;
    alphaBlendOp: VkBlendOp;
    colorWriteMask: VkColorComponentFlags;
}
pub struct VkPipelineColorBlendStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineColorBlendStateCreateFlags;
    logicOpEnable: VkBool32;
    logicOp: VkLogicOp;
    attachmentCount: uint32_t;
    pAttachments: VkPipelineColorBlendAttachmentState;
    blendConstants: float;
}
pub struct VkPipelineDynamicStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineDynamicStateCreateFlags;
    dynamicStateCount: uint32_t;
    pDynamicStates: VkDynamicState;
}
pub struct VkStencilOpState
{
    failOp: VkStencilOp;
    passOp: VkStencilOp;
    depthFailOp: VkStencilOp;
    compareOp: VkCompareOp;
    compareMask: uint32_t;
    writeMask: uint32_t;
    reference: uint32_t;
}
pub struct VkPipelineDepthStencilStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineDepthStencilStateCreateFlags;
    depthTestEnable: VkBool32;
    depthWriteEnable: VkBool32;
    depthCompareOp: VkCompareOp;
    depthBoundsTestEnable: VkBool32;
    stencilTestEnable: VkBool32;
    front: VkStencilOpState;
    back: VkStencilOpState;
    minDepthBounds: float;
    maxDepthBounds: float;
}
pub struct VkGraphicsPipelineCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineCreateFlags;
    stageCount: uint32_t;
    pStages: VkPipelineShaderStageCreateInfo;
    pVertexInputState: VkPipelineVertexInputStateCreateInfo;
    pInputAssemblyState: VkPipelineInputAssemblyStateCreateInfo;
    pTessellationState: VkPipelineTessellationStateCreateInfo;
    pViewportState: VkPipelineViewportStateCreateInfo;
    pRasterizationState: VkPipelineRasterizationStateCreateInfo;
    pMultisampleState: VkPipelineMultisampleStateCreateInfo;
    pDepthStencilState: VkPipelineDepthStencilStateCreateInfo;
    pColorBlendState: VkPipelineColorBlendStateCreateInfo;
    pDynamicState: VkPipelineDynamicStateCreateInfo;
    layout: VkPipelineLayout;
    renderPass: VkRenderPass;
    subpass: uint32_t;
    basePipelineHandle: VkPipeline;
    basePipelineIndex: int32_t;
}
pub struct VkPipelineCacheCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineCacheCreateFlags;
    initialDataSize: size_t;
    pInitialData: void;
}
pub struct VkPipelineCacheHeaderVersionOne
{
    headerSize: uint32_t;
    headerVersion: VkPipelineCacheHeaderVersion;
    vendorID: uint32_t;
    deviceID: uint32_t;
    pipelineCacheUUID: uint8_t;
}
pub struct VkPushConstantRange
{
    stageFlags: VkShaderStageFlags;
    offset: uint32_t;
    size: uint32_t;
}
pub struct VkPipelineLayoutCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineLayoutCreateFlags;
    setLayoutCount: uint32_t;
    pSetLayouts: VkDescriptorSetLayout;
    pushConstantRangeCount: uint32_t;
    pPushConstantRanges: VkPushConstantRange;
}
pub struct VkSamplerCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkSamplerCreateFlags;
    magFilter: VkFilter;
    minFilter: VkFilter;
    mipmapMode: VkSamplerMipmapMode;
    addressModeU: VkSamplerAddressMode;
    addressModeV: VkSamplerAddressMode;
    addressModeW: VkSamplerAddressMode;
    mipLodBias: float;
    anisotropyEnable: VkBool32;
    maxAnisotropy: float;
    compareEnable: VkBool32;
    compareOp: VkCompareOp;
    minLod: float;
    maxLod: float;
    borderColor: VkBorderColor;
    unnormalizedCoordinates: VkBool32;
}
pub struct VkCommandPoolCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkCommandPoolCreateFlags;
    queueFamilyIndex: uint32_t;
}
pub struct VkCommandBufferAllocateInfo
{
    sType: VkStructureType;
    pNext: void;
    commandPool: VkCommandPool;
    level: VkCommandBufferLevel;
    commandBufferCount: uint32_t;
}
pub struct VkCommandBufferInheritanceInfo
{
    sType: VkStructureType;
    pNext: void;
    renderPass: VkRenderPass;
    subpass: uint32_t;
    framebuffer: VkFramebuffer;
    occlusionQueryEnable: VkBool32;
    queryFlags: VkQueryControlFlags;
    pipelineStatistics: VkQueryPipelineStatisticFlags;
}
pub struct VkCommandBufferBeginInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkCommandBufferUsageFlags;
    pInheritanceInfo: VkCommandBufferInheritanceInfo;
}
pub struct VkRenderPassBeginInfo
{
    sType: VkStructureType;
    pNext: void;
    renderPass: VkRenderPass;
    framebuffer: VkFramebuffer;
    renderArea: VkRect2D;
    clearValueCount: uint32_t;
    pClearValues: VkClearValue;
}
pub struct VkClearDepthStencilValue
{
    depth: float;
    stencil: uint32_t;
}
pub struct VkClearAttachment
{
    aspectMask: VkImageAspectFlags;
    colorAttachment: uint32_t;
    clearValue: VkClearValue;
}
pub struct VkAttachmentDescription
{
    flags: VkAttachmentDescriptionFlags;
    format: VkFormat;
    samples: VkSampleCountFlagBits;
    loadOp: VkAttachmentLoadOp;
    storeOp: VkAttachmentStoreOp;
    stencilLoadOp: VkAttachmentLoadOp;
    stencilStoreOp: VkAttachmentStoreOp;
    initialLayout: VkImageLayout;
    finalLayout: VkImageLayout;
}
pub struct VkAttachmentReference
{
    attachment: uint32_t;
    layout: VkImageLayout;
}
pub struct VkSubpassDescription
{
    flags: VkSubpassDescriptionFlags;
    pipelineBindPoint: VkPipelineBindPoint;
    inputAttachmentCount: uint32_t;
    pInputAttachments: VkAttachmentReference;
    colorAttachmentCount: uint32_t;
    pColorAttachments: VkAttachmentReference;
    pResolveAttachments: VkAttachmentReference;
    pDepthStencilAttachment: VkAttachmentReference;
    preserveAttachmentCount: uint32_t;
    pPreserveAttachments: uint32_t;
}
pub struct VkSubpassDependency
{
    srcSubpass: uint32_t;
    dstSubpass: uint32_t;
    srcStageMask: VkPipelineStageFlags;
    dstStageMask: VkPipelineStageFlags;
    srcAccessMask: VkAccessFlags;
    dstAccessMask: VkAccessFlags;
    dependencyFlags: VkDependencyFlags;
}
pub struct VkRenderPassCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkRenderPassCreateFlags;
    attachmentCount: uint32_t;
    pAttachments: VkAttachmentDescription;
    subpassCount: uint32_t;
    pSubpasses: VkSubpassDescription;
    dependencyCount: uint32_t;
    pDependencies: VkSubpassDependency;
}
pub struct VkEventCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkEventCreateFlags;
}
pub struct VkFenceCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkFenceCreateFlags;
}
pub struct VkPhysicalDeviceFeatures
{
    robustBufferAccess: VkBool32;
    fullDrawIndexUint32: VkBool32;
    imageCubeArray: VkBool32;
    independentBlend: VkBool32;
    geometryShader: VkBool32;
    tessellationShader: VkBool32;
    sampleRateShading: VkBool32;
    dualSrcBlend: VkBool32;
    logicOp: VkBool32;
    multiDrawIndirect: VkBool32;
    drawIndirectFirstInstance: VkBool32;
    depthClamp: VkBool32;
    depthBiasClamp: VkBool32;
    fillModeNonSolid: VkBool32;
    depthBounds: VkBool32;
    wideLines: VkBool32;
    largePoints: VkBool32;
    alphaToOne: VkBool32;
    multiViewport: VkBool32;
    samplerAnisotropy: VkBool32;
    textureCompressionETC2: VkBool32;
    textureCompressionASTC_LDR: VkBool32;
    textureCompressionBC: VkBool32;
    occlusionQueryPrecise: VkBool32;
    pipelineStatisticsQuery: VkBool32;
    vertexPipelineStoresAndAtomics: VkBool32;
    fragmentStoresAndAtomics: VkBool32;
    shaderTessellationAndGeometryPointSize: VkBool32;
    shaderImageGatherExtended: VkBool32;
    shaderStorageImageExtendedFormats: VkBool32;
    shaderStorageImageMultisample: VkBool32;
    shaderStorageImageReadWithoutFormat: VkBool32;
    shaderStorageImageWriteWithoutFormat: VkBool32;
    shaderUniformBufferArrayDynamicIndexing: VkBool32;
    shaderSampledImageArrayDynamicIndexing: VkBool32;
    shaderStorageBufferArrayDynamicIndexing: VkBool32;
    shaderStorageImageArrayDynamicIndexing: VkBool32;
    shaderClipDistance: VkBool32;
    shaderCullDistance: VkBool32;
    shaderFloat64: VkBool32;
    shaderInt64: VkBool32;
    shaderInt16: VkBool32;
    shaderResourceResidency: VkBool32;
    shaderResourceMinLod: VkBool32;
    sparseBinding: VkBool32;
    sparseResidencyBuffer: VkBool32;
    sparseResidencyImage2D: VkBool32;
    sparseResidencyImage3D: VkBool32;
    sparseResidency2Samples: VkBool32;
    sparseResidency4Samples: VkBool32;
    sparseResidency8Samples: VkBool32;
    sparseResidency16Samples: VkBool32;
    sparseResidencyAliased: VkBool32;
    variableMultisampleRate: VkBool32;
    inheritedQueries: VkBool32;
}
pub struct VkPhysicalDeviceSparseProperties
{
    residencyStandard2DBlockShape: VkBool32;
    residencyStandard2DMultisampleBlockShape: VkBool32;
    residencyStandard3DBlockShape: VkBool32;
    residencyAlignedMipSize: VkBool32;
    residencyNonResidentStrict: VkBool32;
}
pub struct VkPhysicalDeviceLimits
{
    maxImageDimension1D: uint32_t;
    maxImageDimension2D: uint32_t;
    maxImageDimension3D: uint32_t;
    maxImageDimensionCube: uint32_t;
    maxImageArrayLayers: uint32_t;
    maxTexelBufferElements: uint32_t;
    maxUniformBufferRange: uint32_t;
    maxStorageBufferRange: uint32_t;
    maxPushConstantsSize: uint32_t;
    maxMemoryAllocationCount: uint32_t;
    maxSamplerAllocationCount: uint32_t;
    bufferImageGranularity: VkDeviceSize;
    sparseAddressSpaceSize: VkDeviceSize;
    maxBoundDescriptorSets: uint32_t;
    maxPerStageDescriptorSamplers: uint32_t;
    maxPerStageDescriptorUniformBuffers: uint32_t;
    maxPerStageDescriptorStorageBuffers: uint32_t;
    maxPerStageDescriptorSampledImages: uint32_t;
    maxPerStageDescriptorStorageImages: uint32_t;
    maxPerStageDescriptorInputAttachments: uint32_t;
    maxPerStageResources: uint32_t;
    maxDescriptorSetSamplers: uint32_t;
    maxDescriptorSetUniformBuffers: uint32_t;
    maxDescriptorSetUniformBuffersDynamic: uint32_t;
    maxDescriptorSetStorageBuffers: uint32_t;
    maxDescriptorSetStorageBuffersDynamic: uint32_t;
    maxDescriptorSetSampledImages: uint32_t;
    maxDescriptorSetStorageImages: uint32_t;
    maxDescriptorSetInputAttachments: uint32_t;
    maxVertexInputAttributes: uint32_t;
    maxVertexInputBindings: uint32_t;
    maxVertexInputAttributeOffset: uint32_t;
    maxVertexInputBindingStride: uint32_t;
    maxVertexOutputComponents: uint32_t;
    maxTessellationGenerationLevel: uint32_t;
    maxTessellationPatchSize: uint32_t;
    maxTessellationControlPerVertexInputComponents: uint32_t;
    maxTessellationControlPerVertexOutputComponents: uint32_t;
    maxTessellationControlPerPatchOutputComponents: uint32_t;
    maxTessellationControlTotalOutputComponents: uint32_t;
    maxTessellationEvaluationInputComponents: uint32_t;
    maxTessellationEvaluationOutputComponents: uint32_t;
    maxGeometryShaderInvocations: uint32_t;
    maxGeometryInputComponents: uint32_t;
    maxGeometryOutputComponents: uint32_t;
    maxGeometryOutputVertices: uint32_t;
    maxGeometryTotalOutputComponents: uint32_t;
    maxFragmentInputComponents: uint32_t;
    maxFragmentOutputAttachments: uint32_t;
    maxFragmentDualSrcAttachments: uint32_t;
    maxFragmentCombinedOutputResources: uint32_t;
    maxComputeSharedMemorySize: uint32_t;
    maxComputeWorkGroupCount: uint32_t;
    maxComputeWorkGroupInvocations: uint32_t;
    maxComputeWorkGroupSize: uint32_t;
    subPixelPrecisionBits: uint32_t;
    subTexelPrecisionBits: uint32_t;
    mipmapPrecisionBits: uint32_t;
    maxDrawIndexedIndexValue: uint32_t;
    maxDrawIndirectCount: uint32_t;
    maxSamplerLodBias: float;
    maxSamplerAnisotropy: float;
    maxViewports: uint32_t;
    maxViewportDimensions: uint32_t;
    viewportBoundsRange: float;
    viewportSubPixelBits: uint32_t;
    minMemoryMapAlignment: size_t;
    minTexelBufferOffsetAlignment: VkDeviceSize;
    minUniformBufferOffsetAlignment: VkDeviceSize;
    minStorageBufferOffsetAlignment: VkDeviceSize;
    minTexelOffset: int32_t;
    maxTexelOffset: uint32_t;
    minTexelGatherOffset: int32_t;
    maxTexelGatherOffset: uint32_t;
    minInterpolationOffset: float;
    maxInterpolationOffset: float;
    subPixelInterpolationOffsetBits: uint32_t;
    maxFramebufferWidth: uint32_t;
    maxFramebufferHeight: uint32_t;
    maxFramebufferLayers: uint32_t;
    framebufferColorSampleCounts: VkSampleCountFlags;
    framebufferDepthSampleCounts: VkSampleCountFlags;
    framebufferStencilSampleCounts: VkSampleCountFlags;
    framebufferNoAttachmentsSampleCounts: VkSampleCountFlags;
    maxColorAttachments: uint32_t;
    sampledImageColorSampleCounts: VkSampleCountFlags;
    sampledImageIntegerSampleCounts: VkSampleCountFlags;
    sampledImageDepthSampleCounts: VkSampleCountFlags;
    sampledImageStencilSampleCounts: VkSampleCountFlags;
    storageImageSampleCounts: VkSampleCountFlags;
    maxSampleMaskWords: uint32_t;
    timestampComputeAndGraphics: VkBool32;
    timestampPeriod: float;
    maxClipDistances: uint32_t;
    maxCullDistances: uint32_t;
    maxCombinedClipAndCullDistances: uint32_t;
    discreteQueuePriorities: uint32_t;
    pointSizeRange: float;
    lineWidthRange: float;
    pointSizeGranularity: float;
    lineWidthGranularity: float;
    strictLines: VkBool32;
    standardSampleLocations: VkBool32;
    optimalBufferCopyOffsetAlignment: VkDeviceSize;
    optimalBufferCopyRowPitchAlignment: VkDeviceSize;
    nonCoherentAtomSize: VkDeviceSize;
}
pub struct VkSemaphoreCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkSemaphoreCreateFlags;
}
pub struct VkQueryPoolCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkQueryPoolCreateFlags;
    queryType: VkQueryType;
    queryCount: uint32_t;
    pipelineStatistics: VkQueryPipelineStatisticFlags;
}
pub struct VkFramebufferCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkFramebufferCreateFlags;
    renderPass: VkRenderPass;
    attachmentCount: uint32_t;
    pAttachments: VkImageView;
    width: uint32_t;
    height: uint32_t;
    layers: uint32_t;
}
pub struct VkDrawIndirectCommand
{
    vertexCount: uint32_t;
    instanceCount: uint32_t;
    firstVertex: uint32_t;
    firstInstance: uint32_t;
}
pub struct VkDrawIndexedIndirectCommand
{
    indexCount: uint32_t;
    instanceCount: uint32_t;
    firstIndex: uint32_t;
    vertexOffset: int32_t;
    firstInstance: uint32_t;
}
pub struct VkDispatchIndirectCommand
{
    x: uint32_t;
    y: uint32_t;
    z: uint32_t;
}
pub struct VkMultiDrawInfoEXT
{
    firstVertex: uint32_t;
    vertexCount: uint32_t;
}
pub struct VkMultiDrawIndexedInfoEXT
{
    firstIndex: uint32_t;
    indexCount: uint32_t;
    vertexOffset: int32_t;
}
pub struct VkSubmitInfo
{
    sType: VkStructureType;
    pNext: void;
    waitSemaphoreCount: uint32_t;
    pWaitSemaphores: VkSemaphore;
    pWaitDstStageMask: VkPipelineStageFlags;
    commandBufferCount: uint32_t;
    pCommandBuffers: VkCommandBuffer;
    signalSemaphoreCount: uint32_t;
    pSignalSemaphores: VkSemaphore;
}
pub struct VkDisplayPropertiesKHR
{
    display: VkDisplayKHR;
    displayName: char;
    physicalDimensions: VkExtent2D;
    physicalResolution: VkExtent2D;
    supportedTransforms: VkSurfaceTransformFlagsKHR;
    planeReorderPossible: VkBool32;
    persistentContent: VkBool32;
}
pub struct VkDisplayPlanePropertiesKHR
{
    currentDisplay: VkDisplayKHR;
    currentStackIndex: uint32_t;
}
pub struct VkDisplayModeParametersKHR
{
    visibleRegion: VkExtent2D;
    refreshRate: uint32_t;
}
pub struct VkDisplayModePropertiesKHR
{
    displayMode: VkDisplayModeKHR;
    parameters: VkDisplayModeParametersKHR;
}
pub struct VkDisplayModeCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDisplayModeCreateFlagsKHR;
    parameters: VkDisplayModeParametersKHR;
}
pub struct VkDisplayPlaneCapabilitiesKHR
{
    supportedAlpha: VkDisplayPlaneAlphaFlagsKHR;
    minSrcPosition: VkOffset2D;
    maxSrcPosition: VkOffset2D;
    minSrcExtent: VkExtent2D;
    maxSrcExtent: VkExtent2D;
    minDstPosition: VkOffset2D;
    maxDstPosition: VkOffset2D;
    minDstExtent: VkExtent2D;
    maxDstExtent: VkExtent2D;
}
pub struct VkDisplaySurfaceCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDisplaySurfaceCreateFlagsKHR;
    displayMode: VkDisplayModeKHR;
    planeIndex: uint32_t;
    planeStackIndex: uint32_t;
    transform: VkSurfaceTransformFlagBitsKHR;
    globalAlpha: float;
    alphaMode: VkDisplayPlaneAlphaFlagBitsKHR;
    imageExtent: VkExtent2D;
}
pub struct VkDisplayPresentInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    srcRect: VkRect2D;
    dstRect: VkRect2D;
    persistent: VkBool32;
}
pub struct VkSurfaceCapabilitiesKHR
{
    minImageCount: uint32_t;
    maxImageCount: uint32_t;
    currentExtent: VkExtent2D;
    minImageExtent: VkExtent2D;
    maxImageExtent: VkExtent2D;
    maxImageArrayLayers: uint32_t;
    supportedTransforms: VkSurfaceTransformFlagsKHR;
    currentTransform: VkSurfaceTransformFlagBitsKHR;
    supportedCompositeAlpha: VkCompositeAlphaFlagsKHR;
    supportedUsageFlags: VkImageUsageFlags;
}
pub struct VkAndroidSurfaceCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkAndroidSurfaceCreateFlagsKHR;
    window: ANativeWindow;
}
pub struct VkViSurfaceCreateInfoNN
{
    sType: VkStructureType;
    pNext: void;
    flags: VkViSurfaceCreateFlagsNN;
    window: void;
}
pub struct VkWaylandSurfaceCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkWaylandSurfaceCreateFlagsKHR;
    display: wl_display;
    surface: wl_surface;
}
pub struct VkWin32SurfaceCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkWin32SurfaceCreateFlagsKHR;
    hinstance: HINSTANCE;
    hwnd: HWND;
}
pub struct VkXlibSurfaceCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkXlibSurfaceCreateFlagsKHR;
    dpy: Display;
    window: Window;
}
pub struct VkXcbSurfaceCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkXcbSurfaceCreateFlagsKHR;
    connection: xcb_connection_t;
    window: xcb_window_t;
}
pub struct VkDirectFBSurfaceCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDirectFBSurfaceCreateFlagsEXT;
    dfb: IDirectFB;
    surface: IDirectFBSurface;
}
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA
{
    sType: VkStructureType;
    pNext: void;
    flags: VkImagePipeSurfaceCreateFlagsFUCHSIA;
    imagePipeHandle: zx_handle_t;
}
pub struct VkStreamDescriptorSurfaceCreateInfoGGP
{
    sType: VkStructureType;
    pNext: void;
    flags: VkStreamDescriptorSurfaceCreateFlagsGGP;
    streamDescriptor: GgpStreamDescriptor;
}
pub struct VkScreenSurfaceCreateInfoQNX
{
    sType: VkStructureType;
    pNext: void;
    flags: VkScreenSurfaceCreateFlagsQNX;
    context: _screen_context;
    window: _screen_window;
}
pub struct VkSurfaceFormatKHR
{
    format: VkFormat;
    colorSpace: VkColorSpaceKHR;
}
pub struct VkSwapchainCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkSwapchainCreateFlagsKHR;
    surface: VkSurfaceKHR;
    minImageCount: uint32_t;
    imageFormat: VkFormat;
    imageColorSpace: VkColorSpaceKHR;
    imageExtent: VkExtent2D;
    imageArrayLayers: uint32_t;
    imageUsage: VkImageUsageFlags;
    imageSharingMode: VkSharingMode;
    queueFamilyIndexCount: uint32_t;
    pQueueFamilyIndices: uint32_t;
    preTransform: VkSurfaceTransformFlagBitsKHR;
    compositeAlpha: VkCompositeAlphaFlagBitsKHR;
    presentMode: VkPresentModeKHR;
    clipped: VkBool32;
    oldSwapchain: VkSwapchainKHR;
}
pub struct VkPresentInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    waitSemaphoreCount: uint32_t;
    pWaitSemaphores: VkSemaphore;
    swapchainCount: uint32_t;
    pSwapchains: VkSwapchainKHR;
    pImageIndices: uint32_t;
    pResults: VkResult;
}
pub struct VkDebugReportCallbackCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDebugReportFlagsEXT;
    pfnCallback: PFN_vkDebugReportCallbackEXT;
    pUserData: void;
}
pub struct VkValidationFlagsEXT
{
    sType: VkStructureType;
    pNext: void;
    disabledValidationCheckCount: uint32_t;
    pDisabledValidationChecks: VkValidationCheckEXT;
}
pub struct VkValidationFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    enabledValidationFeatureCount: uint32_t;
    pEnabledValidationFeatures: VkValidationFeatureEnableEXT;
    disabledValidationFeatureCount: uint32_t;
    pDisabledValidationFeatures: VkValidationFeatureDisableEXT;
}
pub struct VkPipelineRasterizationStateRasterizationOrderAMD
{
    sType: VkStructureType;
    pNext: void;
    rasterizationOrder: VkRasterizationOrderAMD;
}
pub struct VkDebugMarkerObjectNameInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    objectType: VkDebugReportObjectTypeEXT;
    object: uint64_t;
    pObjectName: char;
}
pub struct VkDebugMarkerObjectTagInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    objectType: VkDebugReportObjectTypeEXT;
    object: uint64_t;
    tagName: uint64_t;
    tagSize: size_t;
    pTag: void;
}
pub struct VkDebugMarkerMarkerInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    pMarkerName: char;
    color: float;
}
pub struct VkDedicatedAllocationImageCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    dedicatedAllocation: VkBool32;
}
pub struct VkDedicatedAllocationBufferCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    dedicatedAllocation: VkBool32;
}
pub struct VkDedicatedAllocationMemoryAllocateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    image: VkImage;
    buffer: VkBuffer;
}
pub struct VkExternalImageFormatPropertiesNV
{
    imageFormatProperties: VkImageFormatProperties;
    externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV;
    exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV;
    compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV;
}
pub struct VkExternalMemoryImageCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    handleTypes: VkExternalMemoryHandleTypeFlagsNV;
}
pub struct VkExportMemoryAllocateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    handleTypes: VkExternalMemoryHandleTypeFlagsNV;
}
pub struct VkImportMemoryWin32HandleInfoNV
{
    sType: VkStructureType;
    pNext: void;
    handleType: VkExternalMemoryHandleTypeFlagsNV;
    handle: HANDLE;
}
pub struct VkExportMemoryWin32HandleInfoNV
{
    sType: VkStructureType;
    pNext: void;
    pAttributes: SECURITY_ATTRIBUTES;
    dwAccess: DWORD;
}
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV
{
    sType: VkStructureType;
    pNext: void;
    acquireCount: uint32_t;
    pAcquireSyncs: VkDeviceMemory;
    pAcquireKeys: uint64_t;
    pAcquireTimeoutMilliseconds: uint32_t;
    releaseCount: uint32_t;
    pReleaseSyncs: VkDeviceMemory;
    pReleaseKeys: uint64_t;
}
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    deviceGeneratedCommands: VkBool32;
}
pub struct VkDevicePrivateDataCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    privateDataSlotRequestCount: uint32_t;
}
pub struct VkPrivateDataSlotCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPrivateDataSlotCreateFlagsEXT;
}
pub struct VkPhysicalDevicePrivateDataFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    privateData: VkBool32;
}
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV
{
    sType: VkStructureType;
    pNext: void;
    maxGraphicsShaderGroupCount: uint32_t;
    maxIndirectSequenceCount: uint32_t;
    maxIndirectCommandsTokenCount: uint32_t;
    maxIndirectCommandsStreamCount: uint32_t;
    maxIndirectCommandsTokenOffset: uint32_t;
    maxIndirectCommandsStreamStride: uint32_t;
    minSequencesCountBufferOffsetAlignment: uint32_t;
    minSequencesIndexBufferOffsetAlignment: uint32_t;
    minIndirectCommandsBufferOffsetAlignment: uint32_t;
}
pub struct VkPhysicalDeviceMultiDrawPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    maxMultiDrawCount: uint32_t;
}
pub struct VkGraphicsShaderGroupCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    stageCount: uint32_t;
    pStages: VkPipelineShaderStageCreateInfo;
    pVertexInputState: VkPipelineVertexInputStateCreateInfo;
    pTessellationState: VkPipelineTessellationStateCreateInfo;
}
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    groupCount: uint32_t;
    pGroups: VkGraphicsShaderGroupCreateInfoNV;
    pipelineCount: uint32_t;
    pPipelines: VkPipeline;
}
pub struct VkBindShaderGroupIndirectCommandNV
{
    groupIndex: uint32_t;
}
pub struct VkBindIndexBufferIndirectCommandNV
{
    bufferAddress: VkDeviceAddress;
    size: uint32_t;
    indexType: VkIndexType;
}
pub struct VkBindVertexBufferIndirectCommandNV
{
    bufferAddress: VkDeviceAddress;
    size: uint32_t;
    stride: uint32_t;
}
pub struct VkSetStateFlagsIndirectCommandNV
{
    data: uint32_t;
}
pub struct VkIndirectCommandsStreamNV
{
    buffer: VkBuffer;
    offset: VkDeviceSize;
}
pub struct VkIndirectCommandsLayoutTokenNV
{
    sType: VkStructureType;
    pNext: void;
    tokenType: VkIndirectCommandsTokenTypeNV;
    stream: uint32_t;
    offset: uint32_t;
    vertexBindingUnit: uint32_t;
    vertexDynamicStride: VkBool32;
    pushconstantPipelineLayout: VkPipelineLayout;
    pushconstantShaderStageFlags: VkShaderStageFlags;
    pushconstantOffset: uint32_t;
    pushconstantSize: uint32_t;
    indirectStateFlags: VkIndirectStateFlagsNV;
    indexTypeCount: uint32_t;
    pIndexTypes: VkIndexType;
    pIndexTypeValues: uint32_t;
}
pub struct VkIndirectCommandsLayoutCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    flags: VkIndirectCommandsLayoutUsageFlagsNV;
    pipelineBindPoint: VkPipelineBindPoint;
    tokenCount: uint32_t;
    pTokens: VkIndirectCommandsLayoutTokenNV;
    streamCount: uint32_t;
    pStreamStrides: uint32_t;
}
pub struct VkGeneratedCommandsInfoNV
{
    sType: VkStructureType;
    pNext: void;
    pipelineBindPoint: VkPipelineBindPoint;
    pipeline: VkPipeline;
    indirectCommandsLayout: VkIndirectCommandsLayoutNV;
    streamCount: uint32_t;
    pStreams: VkIndirectCommandsStreamNV;
    sequencesCount: uint32_t;
    preprocessBuffer: VkBuffer;
    preprocessOffset: VkDeviceSize;
    preprocessSize: VkDeviceSize;
    sequencesCountBuffer: VkBuffer;
    sequencesCountOffset: VkDeviceSize;
    sequencesIndexBuffer: VkBuffer;
    sequencesIndexOffset: VkDeviceSize;
}
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV
{
    sType: VkStructureType;
    pNext: void;
    pipelineBindPoint: VkPipelineBindPoint;
    pipeline: VkPipeline;
    indirectCommandsLayout: VkIndirectCommandsLayoutNV;
    maxSequencesCount: uint32_t;
}
pub struct VkPhysicalDeviceFeatures2
{
    sType: VkStructureType;
    pNext: void;
    features: VkPhysicalDeviceFeatures;
}
pub struct VkPhysicalDeviceFeatures2KHR
{
}
pub struct VkPhysicalDeviceProperties2
{
    sType: VkStructureType;
    pNext: void;
    properties: VkPhysicalDeviceProperties;
}
pub struct VkPhysicalDeviceProperties2KHR
{
}
pub struct VkFormatProperties2
{
    sType: VkStructureType;
    pNext: void;
    formatProperties: VkFormatProperties;
}
pub struct VkFormatProperties2KHR
{
}
pub struct VkImageFormatProperties2
{
    sType: VkStructureType;
    pNext: void;
    imageFormatProperties: VkImageFormatProperties;
}
pub struct VkImageFormatProperties2KHR
{
}
pub struct VkPhysicalDeviceImageFormatInfo2
{
    sType: VkStructureType;
    pNext: void;
    format: VkFormat;
    type: VkImageType;
    tiling: VkImageTiling;
    usage: VkImageUsageFlags;
    flags: VkImageCreateFlags;
}
pub struct VkPhysicalDeviceImageFormatInfo2KHR
{
}
pub struct VkQueueFamilyProperties2
{
    sType: VkStructureType;
    pNext: void;
    queueFamilyProperties: VkQueueFamilyProperties;
}
pub struct VkQueueFamilyProperties2KHR
{
}
pub struct VkPhysicalDeviceMemoryProperties2
{
    sType: VkStructureType;
    pNext: void;
    memoryProperties: VkPhysicalDeviceMemoryProperties;
}
pub struct VkPhysicalDeviceMemoryProperties2KHR
{
}
pub struct VkSparseImageFormatProperties2
{
    sType: VkStructureType;
    pNext: void;
    properties: VkSparseImageFormatProperties;
}
pub struct VkSparseImageFormatProperties2KHR
{
}
pub struct VkPhysicalDeviceSparseImageFormatInfo2
{
    sType: VkStructureType;
    pNext: void;
    format: VkFormat;
    type: VkImageType;
    samples: VkSampleCountFlagBits;
    usage: VkImageUsageFlags;
    tiling: VkImageTiling;
}
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR
{
}
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    maxPushDescriptors: uint32_t;
}
pub struct VkConformanceVersion
{
    major: uint8_t;
    minor: uint8_t;
    subminor: uint8_t;
    patch: uint8_t;
}
pub struct VkConformanceVersionKHR
{
}
pub struct VkPhysicalDeviceDriverProperties
{
    sType: VkStructureType;
    pNext: void;
    driverID: VkDriverId;
    driverName: char;
    driverInfo: char;
    conformanceVersion: VkConformanceVersion;
}
pub struct VkPhysicalDeviceDriverPropertiesKHR
{
}
pub struct VkPresentRegionsKHR
{
    sType: VkStructureType;
    pNext: void;
    swapchainCount: uint32_t;
    pRegions: VkPresentRegionKHR;
}
pub struct VkPresentRegionKHR
{
    rectangleCount: uint32_t;
    pRectangles: VkRectLayerKHR;
}
pub struct VkRectLayerKHR
{
    offset: VkOffset2D;
    extent: VkExtent2D;
    layer: uint32_t;
}
pub struct VkPhysicalDeviceVariablePointersFeatures
{
    sType: VkStructureType;
    pNext: void;
    variablePointersStorageBuffer: VkBool32;
    variablePointers: VkBool32;
}
pub struct VkPhysicalDeviceVariablePointersFeaturesKHR
{
}
pub struct VkPhysicalDeviceVariablePointerFeaturesKHR
{
}
pub struct VkPhysicalDeviceVariablePointerFeatures
{
}
pub struct VkExternalMemoryProperties
{
    externalMemoryFeatures: VkExternalMemoryFeatureFlags;
    exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags;
    compatibleHandleTypes: VkExternalMemoryHandleTypeFlags;
}
pub struct VkExternalMemoryPropertiesKHR
{
}
pub struct VkPhysicalDeviceExternalImageFormatInfo
{
    sType: VkStructureType;
    pNext: void;
    handleType: VkExternalMemoryHandleTypeFlagBits;
}
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR
{
}
pub struct VkExternalImageFormatProperties
{
    sType: VkStructureType;
    pNext: void;
    externalMemoryProperties: VkExternalMemoryProperties;
}
pub struct VkExternalImageFormatPropertiesKHR
{
}
pub struct VkPhysicalDeviceExternalBufferInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkBufferCreateFlags;
    usage: VkBufferUsageFlags;
    handleType: VkExternalMemoryHandleTypeFlagBits;
}
pub struct VkPhysicalDeviceExternalBufferInfoKHR
{
}
pub struct VkExternalBufferProperties
{
    sType: VkStructureType;
    pNext: void;
    externalMemoryProperties: VkExternalMemoryProperties;
}
pub struct VkExternalBufferPropertiesKHR
{
}
pub struct VkPhysicalDeviceIDProperties
{
    sType: VkStructureType;
    pNext: void;
    deviceUUID: uint8_t;
    driverUUID: uint8_t;
    deviceLUID: uint8_t;
    deviceNodeMask: uint32_t;
    deviceLUIDValid: VkBool32;
}
pub struct VkPhysicalDeviceIDPropertiesKHR
{
}
pub struct VkExternalMemoryImageCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    handleTypes: VkExternalMemoryHandleTypeFlags;
}
pub struct VkExternalMemoryImageCreateInfoKHR
{
}
pub struct VkExternalMemoryBufferCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    handleTypes: VkExternalMemoryHandleTypeFlags;
}
pub struct VkExternalMemoryBufferCreateInfoKHR
{
}
pub struct VkExportMemoryAllocateInfo
{
    sType: VkStructureType;
    pNext: void;
    handleTypes: VkExternalMemoryHandleTypeFlags;
}
pub struct VkExportMemoryAllocateInfoKHR
{
}
pub struct VkImportMemoryWin32HandleInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    handleType: VkExternalMemoryHandleTypeFlagBits;
    handle: HANDLE;
    name: LPCWSTR;
}
pub struct VkExportMemoryWin32HandleInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    pAttributes: SECURITY_ATTRIBUTES;
    dwAccess: DWORD;
    name: LPCWSTR;
}
pub struct VkImportMemoryZirconHandleInfoFUCHSIA
{
    sType: VkStructureType;
    pNext: void;
    handleType: VkExternalMemoryHandleTypeFlagBits;
    handle: zx_handle_t;
}
pub struct VkMemoryZirconHandlePropertiesFUCHSIA
{
    sType: VkStructureType;
    pNext: void;
    memoryTypeBits: uint32_t;
}
pub struct VkMemoryGetZirconHandleInfoFUCHSIA
{
    sType: VkStructureType;
    pNext: void;
    memory: VkDeviceMemory;
    handleType: VkExternalMemoryHandleTypeFlagBits;
}
pub struct VkMemoryWin32HandlePropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    memoryTypeBits: uint32_t;
}
pub struct VkMemoryGetWin32HandleInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    memory: VkDeviceMemory;
    handleType: VkExternalMemoryHandleTypeFlagBits;
}
pub struct VkImportMemoryFdInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    handleType: VkExternalMemoryHandleTypeFlagBits;
    fd: int;
}
pub struct VkMemoryFdPropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    memoryTypeBits: uint32_t;
}
pub struct VkMemoryGetFdInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    memory: VkDeviceMemory;
    handleType: VkExternalMemoryHandleTypeFlagBits;
}
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    acquireCount: uint32_t;
    pAcquireSyncs: VkDeviceMemory;
    pAcquireKeys: uint64_t;
    pAcquireTimeouts: uint32_t;
    releaseCount: uint32_t;
    pReleaseSyncs: VkDeviceMemory;
    pReleaseKeys: uint64_t;
}
pub struct VkPhysicalDeviceExternalSemaphoreInfo
{
    sType: VkStructureType;
    pNext: void;
    handleType: VkExternalSemaphoreHandleTypeFlagBits;
}
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR
{
}
pub struct VkExternalSemaphoreProperties
{
    sType: VkStructureType;
    pNext: void;
    exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags;
    compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags;
    externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags;
}
pub struct VkExternalSemaphorePropertiesKHR
{
}
pub struct VkExportSemaphoreCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    handleTypes: VkExternalSemaphoreHandleTypeFlags;
}
pub struct VkExportSemaphoreCreateInfoKHR
{
}
pub struct VkImportSemaphoreWin32HandleInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    semaphore: VkSemaphore;
    flags: VkSemaphoreImportFlags;
    handleType: VkExternalSemaphoreHandleTypeFlagBits;
    handle: HANDLE;
    name: LPCWSTR;
}
pub struct VkExportSemaphoreWin32HandleInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    pAttributes: SECURITY_ATTRIBUTES;
    dwAccess: DWORD;
    name: LPCWSTR;
}
pub struct VkD3D12FenceSubmitInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    waitSemaphoreValuesCount: uint32_t;
    pWaitSemaphoreValues: uint64_t;
    signalSemaphoreValuesCount: uint32_t;
    pSignalSemaphoreValues: uint64_t;
}
pub struct VkSemaphoreGetWin32HandleInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    semaphore: VkSemaphore;
    handleType: VkExternalSemaphoreHandleTypeFlagBits;
}
pub struct VkImportSemaphoreFdInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    semaphore: VkSemaphore;
    flags: VkSemaphoreImportFlags;
    handleType: VkExternalSemaphoreHandleTypeFlagBits;
    fd: int;
}
pub struct VkSemaphoreGetFdInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    semaphore: VkSemaphore;
    handleType: VkExternalSemaphoreHandleTypeFlagBits;
}
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA
{
    sType: VkStructureType;
    pNext: void;
    semaphore: VkSemaphore;
    flags: VkSemaphoreImportFlags;
    handleType: VkExternalSemaphoreHandleTypeFlagBits;
    zirconHandle: zx_handle_t;
}
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA
{
    sType: VkStructureType;
    pNext: void;
    semaphore: VkSemaphore;
    handleType: VkExternalSemaphoreHandleTypeFlagBits;
}
pub struct VkPhysicalDeviceExternalFenceInfo
{
    sType: VkStructureType;
    pNext: void;
    handleType: VkExternalFenceHandleTypeFlagBits;
}
pub struct VkPhysicalDeviceExternalFenceInfoKHR
{
}
pub struct VkExternalFenceProperties
{
    sType: VkStructureType;
    pNext: void;
    exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags;
    compatibleHandleTypes: VkExternalFenceHandleTypeFlags;
    externalFenceFeatures: VkExternalFenceFeatureFlags;
}
pub struct VkExternalFencePropertiesKHR
{
}
pub struct VkExportFenceCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    handleTypes: VkExternalFenceHandleTypeFlags;
}
pub struct VkExportFenceCreateInfoKHR
{
}
pub struct VkImportFenceWin32HandleInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    fence: VkFence;
    flags: VkFenceImportFlags;
    handleType: VkExternalFenceHandleTypeFlagBits;
    handle: HANDLE;
    name: LPCWSTR;
}
pub struct VkExportFenceWin32HandleInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    pAttributes: SECURITY_ATTRIBUTES;
    dwAccess: DWORD;
    name: LPCWSTR;
}
pub struct VkFenceGetWin32HandleInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    fence: VkFence;
    handleType: VkExternalFenceHandleTypeFlagBits;
}
pub struct VkImportFenceFdInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    fence: VkFence;
    flags: VkFenceImportFlags;
    handleType: VkExternalFenceHandleTypeFlagBits;
    fd: int;
}
pub struct VkFenceGetFdInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    fence: VkFence;
    handleType: VkExternalFenceHandleTypeFlagBits;
}
pub struct VkPhysicalDeviceMultiviewFeatures
{
    sType: VkStructureType;
    pNext: void;
    multiview: VkBool32;
    multiviewGeometryShader: VkBool32;
    multiviewTessellationShader: VkBool32;
}
pub struct VkPhysicalDeviceMultiviewFeaturesKHR
{
}
pub struct VkPhysicalDeviceMultiviewProperties
{
    sType: VkStructureType;
    pNext: void;
    maxMultiviewViewCount: uint32_t;
    maxMultiviewInstanceIndex: uint32_t;
}
pub struct VkPhysicalDeviceMultiviewPropertiesKHR
{
}
pub struct VkRenderPassMultiviewCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    subpassCount: uint32_t;
    pViewMasks: uint32_t;
    dependencyCount: uint32_t;
    pViewOffsets: int32_t;
    correlationMaskCount: uint32_t;
    pCorrelationMasks: uint32_t;
}
pub struct VkRenderPassMultiviewCreateInfoKHR
{
}
pub struct VkSurfaceCapabilities2EXT
{
    sType: VkStructureType;
    pNext: void;
    minImageCount: uint32_t;
    maxImageCount: uint32_t;
    currentExtent: VkExtent2D;
    minImageExtent: VkExtent2D;
    maxImageExtent: VkExtent2D;
    maxImageArrayLayers: uint32_t;
    supportedTransforms: VkSurfaceTransformFlagsKHR;
    currentTransform: VkSurfaceTransformFlagBitsKHR;
    supportedCompositeAlpha: VkCompositeAlphaFlagsKHR;
    supportedUsageFlags: VkImageUsageFlags;
    supportedSurfaceCounters: VkSurfaceCounterFlagsEXT;
}
pub struct VkDisplayPowerInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    powerState: VkDisplayPowerStateEXT;
}
pub struct VkDeviceEventInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    deviceEvent: VkDeviceEventTypeEXT;
}
pub struct VkDisplayEventInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    displayEvent: VkDisplayEventTypeEXT;
}
pub struct VkSwapchainCounterCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    surfaceCounters: VkSurfaceCounterFlagsEXT;
}
pub struct VkPhysicalDeviceGroupProperties
{
    sType: VkStructureType;
    pNext: void;
    physicalDeviceCount: uint32_t;
    physicalDevices: VkPhysicalDevice;
    subsetAllocation: VkBool32;
}
pub struct VkPhysicalDeviceGroupPropertiesKHR
{
}
pub struct VkMemoryAllocateFlagsInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkMemoryAllocateFlags;
    deviceMask: uint32_t;
}
pub struct VkMemoryAllocateFlagsInfoKHR
{
}
pub struct VkBindBufferMemoryInfo
{
    sType: VkStructureType;
    pNext: void;
    buffer: VkBuffer;
    memory: VkDeviceMemory;
    memoryOffset: VkDeviceSize;
}
pub struct VkBindBufferMemoryInfoKHR
{
}
pub struct VkBindBufferMemoryDeviceGroupInfo
{
    sType: VkStructureType;
    pNext: void;
    deviceIndexCount: uint32_t;
    pDeviceIndices: uint32_t;
}
pub struct VkBindBufferMemoryDeviceGroupInfoKHR
{
}
pub struct VkBindImageMemoryInfo
{
    sType: VkStructureType;
    pNext: void;
    image: VkImage;
    memory: VkDeviceMemory;
    memoryOffset: VkDeviceSize;
}
pub struct VkBindImageMemoryInfoKHR
{
}
pub struct VkBindImageMemoryDeviceGroupInfo
{
    sType: VkStructureType;
    pNext: void;
    deviceIndexCount: uint32_t;
    pDeviceIndices: uint32_t;
    splitInstanceBindRegionCount: uint32_t;
    pSplitInstanceBindRegions: VkRect2D;
}
pub struct VkBindImageMemoryDeviceGroupInfoKHR
{
}
pub struct VkDeviceGroupRenderPassBeginInfo
{
    sType: VkStructureType;
    pNext: void;
    deviceMask: uint32_t;
    deviceRenderAreaCount: uint32_t;
    pDeviceRenderAreas: VkRect2D;
}
pub struct VkDeviceGroupRenderPassBeginInfoKHR
{
}
pub struct VkDeviceGroupCommandBufferBeginInfo
{
    sType: VkStructureType;
    pNext: void;
    deviceMask: uint32_t;
}
pub struct VkDeviceGroupCommandBufferBeginInfoKHR
{
}
pub struct VkDeviceGroupSubmitInfo
{
    sType: VkStructureType;
    pNext: void;
    waitSemaphoreCount: uint32_t;
    pWaitSemaphoreDeviceIndices: uint32_t;
    commandBufferCount: uint32_t;
    pCommandBufferDeviceMasks: uint32_t;
    signalSemaphoreCount: uint32_t;
    pSignalSemaphoreDeviceIndices: uint32_t;
}
pub struct VkDeviceGroupSubmitInfoKHR
{
}
pub struct VkDeviceGroupBindSparseInfo
{
    sType: VkStructureType;
    pNext: void;
    resourceDeviceIndex: uint32_t;
    memoryDeviceIndex: uint32_t;
}
pub struct VkDeviceGroupBindSparseInfoKHR
{
}
pub struct VkDeviceGroupPresentCapabilitiesKHR
{
    sType: VkStructureType;
    pNext: void;
    presentMask: uint32_t;
    modes: VkDeviceGroupPresentModeFlagsKHR;
}
pub struct VkImageSwapchainCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    swapchain: VkSwapchainKHR;
}
pub struct VkBindImageMemorySwapchainInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    swapchain: VkSwapchainKHR;
    imageIndex: uint32_t;
}
pub struct VkAcquireNextImageInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    swapchain: VkSwapchainKHR;
    timeout: uint64_t;
    semaphore: VkSemaphore;
    fence: VkFence;
    deviceMask: uint32_t;
}
pub struct VkDeviceGroupPresentInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    swapchainCount: uint32_t;
    pDeviceMasks: uint32_t;
    mode: VkDeviceGroupPresentModeFlagBitsKHR;
}
pub struct VkDeviceGroupDeviceCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    physicalDeviceCount: uint32_t;
    pPhysicalDevices: VkPhysicalDevice;
}
pub struct VkDeviceGroupDeviceCreateInfoKHR
{
}
pub struct VkDeviceGroupSwapchainCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    modes: VkDeviceGroupPresentModeFlagsKHR;
}
pub struct VkDescriptorUpdateTemplateEntry
{
    dstBinding: uint32_t;
    dstArrayElement: uint32_t;
    descriptorCount: uint32_t;
    descriptorType: VkDescriptorType;
    offset: size_t;
    stride: size_t;
}
pub struct VkDescriptorUpdateTemplateEntryKHR
{
}
pub struct VkDescriptorUpdateTemplateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDescriptorUpdateTemplateCreateFlags;
    descriptorUpdateEntryCount: uint32_t;
    pDescriptorUpdateEntries: VkDescriptorUpdateTemplateEntry;
    templateType: VkDescriptorUpdateTemplateType;
    descriptorSetLayout: VkDescriptorSetLayout;
    pipelineBindPoint: VkPipelineBindPoint;
    pipelineLayout: VkPipelineLayout;
    set: uint32_t;
}
pub struct VkDescriptorUpdateTemplateCreateInfoKHR
{
}
pub struct VkXYColorEXT
{
    x: float;
    y: float;
}
pub struct VkPhysicalDevicePresentIdFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    presentId: VkBool32;
}
pub struct VkPresentIdKHR
{
    sType: VkStructureType;
    pNext: void;
    swapchainCount: uint32_t;
    pPresentIds: uint64_t;
}
pub struct VkPhysicalDevicePresentWaitFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    presentWait: VkBool32;
}
pub struct VkHdrMetadataEXT
{
    sType: VkStructureType;
    pNext: void;
    displayPrimaryRed: VkXYColorEXT;
    displayPrimaryGreen: VkXYColorEXT;
    displayPrimaryBlue: VkXYColorEXT;
    whitePoint: VkXYColorEXT;
    maxLuminance: float;
    minLuminance: float;
    maxContentLightLevel: float;
    maxFrameAverageLightLevel: float;
}
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD
{
    sType: VkStructureType;
    pNext: void;
    localDimmingSupport: VkBool32;
}
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD
{
    sType: VkStructureType;
    pNext: void;
    localDimmingEnable: VkBool32;
}
pub struct VkRefreshCycleDurationGOOGLE
{
    refreshDuration: uint64_t;
}
pub struct VkPastPresentationTimingGOOGLE
{
    presentID: uint32_t;
    desiredPresentTime: uint64_t;
    actualPresentTime: uint64_t;
    earliestPresentTime: uint64_t;
    presentMargin: uint64_t;
}
pub struct VkPresentTimesInfoGOOGLE
{
    sType: VkStructureType;
    pNext: void;
    swapchainCount: uint32_t;
    pTimes: VkPresentTimeGOOGLE;
}
pub struct VkPresentTimeGOOGLE
{
    presentID: uint32_t;
    desiredPresentTime: uint64_t;
}
pub struct VkIOSSurfaceCreateInfoMVK
{
    sType: VkStructureType;
    pNext: void;
    flags: VkIOSSurfaceCreateFlagsMVK;
    pView: void;
}
pub struct VkMacOSSurfaceCreateInfoMVK
{
    sType: VkStructureType;
    pNext: void;
    flags: VkMacOSSurfaceCreateFlagsMVK;
    pView: void;
}
pub struct VkMetalSurfaceCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkMetalSurfaceCreateFlagsEXT;
    pLayer: CAMetalLayer;
}
pub struct VkViewportWScalingNV
{
    xcoeff: float;
    ycoeff: float;
}
pub struct VkPipelineViewportWScalingStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    viewportWScalingEnable: VkBool32;
    viewportCount: uint32_t;
    pViewportWScalings: VkViewportWScalingNV;
}
pub struct VkViewportSwizzleNV
{
    x: VkViewportCoordinateSwizzleNV;
    y: VkViewportCoordinateSwizzleNV;
    z: VkViewportCoordinateSwizzleNV;
    w: VkViewportCoordinateSwizzleNV;
}
pub struct VkPipelineViewportSwizzleStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineViewportSwizzleStateCreateFlagsNV;
    viewportCount: uint32_t;
    pViewportSwizzles: VkViewportSwizzleNV;
}
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    maxDiscardRectangles: uint32_t;
}
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineDiscardRectangleStateCreateFlagsEXT;
    discardRectangleMode: VkDiscardRectangleModeEXT;
    discardRectangleCount: uint32_t;
    pDiscardRectangles: VkRect2D;
}
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
{
    sType: VkStructureType;
    pNext: void;
    perViewPositionAllComponents: VkBool32;
}
pub struct VkInputAttachmentAspectReference
{
    subpass: uint32_t;
    inputAttachmentIndex: uint32_t;
    aspectMask: VkImageAspectFlags;
}
pub struct VkInputAttachmentAspectReferenceKHR
{
}
pub struct VkRenderPassInputAttachmentAspectCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    aspectReferenceCount: uint32_t;
    pAspectReferences: VkInputAttachmentAspectReference;
}
pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR
{
}
pub struct VkPhysicalDeviceSurfaceInfo2KHR
{
    sType: VkStructureType;
    pNext: void;
    surface: VkSurfaceKHR;
}
pub struct VkSurfaceCapabilities2KHR
{
    sType: VkStructureType;
    pNext: void;
    surfaceCapabilities: VkSurfaceCapabilitiesKHR;
}
pub struct VkSurfaceFormat2KHR
{
    sType: VkStructureType;
    pNext: void;
    surfaceFormat: VkSurfaceFormatKHR;
}
pub struct VkDisplayProperties2KHR
{
    sType: VkStructureType;
    pNext: void;
    displayProperties: VkDisplayPropertiesKHR;
}
pub struct VkDisplayPlaneProperties2KHR
{
    sType: VkStructureType;
    pNext: void;
    displayPlaneProperties: VkDisplayPlanePropertiesKHR;
}
pub struct VkDisplayModeProperties2KHR
{
    sType: VkStructureType;
    pNext: void;
    displayModeProperties: VkDisplayModePropertiesKHR;
}
pub struct VkDisplayPlaneInfo2KHR
{
    sType: VkStructureType;
    pNext: void;
    mode: VkDisplayModeKHR;
    planeIndex: uint32_t;
}
pub struct VkDisplayPlaneCapabilities2KHR
{
    sType: VkStructureType;
    pNext: void;
    capabilities: VkDisplayPlaneCapabilitiesKHR;
}
pub struct VkSharedPresentSurfaceCapabilitiesKHR
{
    sType: VkStructureType;
    pNext: void;
    sharedPresentSupportedUsageFlags: VkImageUsageFlags;
}
pub struct VkPhysicalDevice16BitStorageFeatures
{
    sType: VkStructureType;
    pNext: void;
    storageBuffer16BitAccess: VkBool32;
    uniformAndStorageBuffer16BitAccess: VkBool32;
    storagePushConstant16: VkBool32;
    storageInputOutput16: VkBool32;
}
pub struct VkPhysicalDevice16BitStorageFeaturesKHR
{
}
pub struct VkPhysicalDeviceSubgroupProperties
{
    sType: VkStructureType;
    pNext: void;
    subgroupSize: uint32_t;
    supportedStages: VkShaderStageFlags;
    supportedOperations: VkSubgroupFeatureFlags;
    quadOperationsInAllStages: VkBool32;
}
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures
{
    sType: VkStructureType;
    pNext: void;
    shaderSubgroupExtendedTypes: VkBool32;
}
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR
{
}
pub struct VkBufferMemoryRequirementsInfo2
{
    sType: VkStructureType;
    pNext: void;
    buffer: VkBuffer;
}
pub struct VkBufferMemoryRequirementsInfo2KHR
{
}
pub struct VkImageMemoryRequirementsInfo2
{
    sType: VkStructureType;
    pNext: void;
    image: VkImage;
}
pub struct VkImageMemoryRequirementsInfo2KHR
{
}
pub struct VkImageSparseMemoryRequirementsInfo2
{
    sType: VkStructureType;
    pNext: void;
    image: VkImage;
}
pub struct VkImageSparseMemoryRequirementsInfo2KHR
{
}
pub struct VkMemoryRequirements2
{
    sType: VkStructureType;
    pNext: void;
    memoryRequirements: VkMemoryRequirements;
}
pub struct VkMemoryRequirements2KHR
{
}
pub struct VkSparseImageMemoryRequirements2
{
    sType: VkStructureType;
    pNext: void;
    memoryRequirements: VkSparseImageMemoryRequirements;
}
pub struct VkSparseImageMemoryRequirements2KHR
{
}
pub struct VkPhysicalDevicePointClippingProperties
{
    sType: VkStructureType;
    pNext: void;
    pointClippingBehavior: VkPointClippingBehavior;
}
pub struct VkPhysicalDevicePointClippingPropertiesKHR
{
}
pub struct VkMemoryDedicatedRequirements
{
    sType: VkStructureType;
    pNext: void;
    prefersDedicatedAllocation: VkBool32;
    requiresDedicatedAllocation: VkBool32;
}
pub struct VkMemoryDedicatedRequirementsKHR
{
}
pub struct VkMemoryDedicatedAllocateInfo
{
    sType: VkStructureType;
    pNext: void;
    image: VkImage;
    buffer: VkBuffer;
}
pub struct VkMemoryDedicatedAllocateInfoKHR
{
}
pub struct VkImageViewUsageCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    usage: VkImageUsageFlags;
}
pub struct VkImageViewUsageCreateInfoKHR
{
}
pub struct VkPipelineTessellationDomainOriginStateCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    domainOrigin: VkTessellationDomainOrigin;
}
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR
{
}
pub struct VkSamplerYcbcrConversionInfo
{
    sType: VkStructureType;
    pNext: void;
    conversion: VkSamplerYcbcrConversion;
}
pub struct VkSamplerYcbcrConversionInfoKHR
{
}
pub struct VkSamplerYcbcrConversionCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    format: VkFormat;
    ycbcrModel: VkSamplerYcbcrModelConversion;
    ycbcrRange: VkSamplerYcbcrRange;
    components: VkComponentMapping;
    xChromaOffset: VkChromaLocation;
    yChromaOffset: VkChromaLocation;
    chromaFilter: VkFilter;
    forceExplicitReconstruction: VkBool32;
}
pub struct VkSamplerYcbcrConversionCreateInfoKHR
{
}
pub struct VkBindImagePlaneMemoryInfo
{
    sType: VkStructureType;
    pNext: void;
    planeAspect: VkImageAspectFlagBits;
}
pub struct VkBindImagePlaneMemoryInfoKHR
{
}
pub struct VkImagePlaneMemoryRequirementsInfo
{
    sType: VkStructureType;
    pNext: void;
    planeAspect: VkImageAspectFlagBits;
}
pub struct VkImagePlaneMemoryRequirementsInfoKHR
{
}
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures
{
    sType: VkStructureType;
    pNext: void;
    samplerYcbcrConversion: VkBool32;
}
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR
{
}
pub struct VkSamplerYcbcrConversionImageFormatProperties
{
    sType: VkStructureType;
    pNext: void;
    combinedImageSamplerDescriptorCount: uint32_t;
}
pub struct VkSamplerYcbcrConversionImageFormatPropertiesKHR
{
}
pub struct VkTextureLODGatherFormatPropertiesAMD
{
    sType: VkStructureType;
    pNext: void;
    supportsTextureGatherLODBiasAMD: VkBool32;
}
pub struct VkConditionalRenderingBeginInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    buffer: VkBuffer;
    offset: VkDeviceSize;
    flags: VkConditionalRenderingFlagsEXT;
}
pub struct VkProtectedSubmitInfo
{
    sType: VkStructureType;
    pNext: void;
    protectedSubmit: VkBool32;
}
pub struct VkPhysicalDeviceProtectedMemoryFeatures
{
    sType: VkStructureType;
    pNext: void;
    protectedMemory: VkBool32;
}
pub struct VkPhysicalDeviceProtectedMemoryProperties
{
    sType: VkStructureType;
    pNext: void;
    protectedNoFault: VkBool32;
}
pub struct VkDeviceQueueInfo2
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDeviceQueueCreateFlags;
    queueFamilyIndex: uint32_t;
    queueIndex: uint32_t;
}
pub struct VkPipelineCoverageToColorStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineCoverageToColorStateCreateFlagsNV;
    coverageToColorEnable: VkBool32;
    coverageToColorLocation: uint32_t;
}
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties
{
    sType: VkStructureType;
    pNext: void;
    filterMinmaxSingleComponentFormats: VkBool32;
    filterMinmaxImageComponentMapping: VkBool32;
}
pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT
{
}
pub struct VkSampleLocationEXT
{
    x: float;
    y: float;
}
pub struct VkSampleLocationsInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    sampleLocationsPerPixel: VkSampleCountFlagBits;
    sampleLocationGridSize: VkExtent2D;
    sampleLocationsCount: uint32_t;
    pSampleLocations: VkSampleLocationEXT;
}
pub struct VkAttachmentSampleLocationsEXT
{
    attachmentIndex: uint32_t;
    sampleLocationsInfo: VkSampleLocationsInfoEXT;
}
pub struct VkSubpassSampleLocationsEXT
{
    subpassIndex: uint32_t;
    sampleLocationsInfo: VkSampleLocationsInfoEXT;
}
pub struct VkRenderPassSampleLocationsBeginInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    attachmentInitialSampleLocationsCount: uint32_t;
    pAttachmentInitialSampleLocations: VkAttachmentSampleLocationsEXT;
    postSubpassSampleLocationsCount: uint32_t;
    pPostSubpassSampleLocations: VkSubpassSampleLocationsEXT;
}
pub struct VkPipelineSampleLocationsStateCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    sampleLocationsEnable: VkBool32;
    sampleLocationsInfo: VkSampleLocationsInfoEXT;
}
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    sampleLocationSampleCounts: VkSampleCountFlags;
    maxSampleLocationGridSize: VkExtent2D;
    sampleLocationCoordinateRange: float;
    sampleLocationSubPixelBits: uint32_t;
    variableSampleLocations: VkBool32;
}
pub struct VkMultisamplePropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    maxSampleLocationGridSize: VkExtent2D;
}
pub struct VkSamplerReductionModeCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    reductionMode: VkSamplerReductionMode;
}
pub struct VkSamplerReductionModeCreateInfoEXT
{
}
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    advancedBlendCoherentOperations: VkBool32;
}
pub struct VkPhysicalDeviceMultiDrawFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    multiDraw: VkBool32;
}
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    advancedBlendMaxColorAttachments: uint32_t;
    advancedBlendIndependentBlend: VkBool32;
    advancedBlendNonPremultipliedSrcColor: VkBool32;
    advancedBlendNonPremultipliedDstColor: VkBool32;
    advancedBlendCorrelatedOverlap: VkBool32;
    advancedBlendAllOperations: VkBool32;
}
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    srcPremultiplied: VkBool32;
    dstPremultiplied: VkBool32;
    blendOverlap: VkBlendOverlapEXT;
}
pub struct VkPhysicalDeviceInlineUniformBlockFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    inlineUniformBlock: VkBool32;
    descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32;
}
pub struct VkPhysicalDeviceInlineUniformBlockPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    maxInlineUniformBlockSize: uint32_t;
    maxPerStageDescriptorInlineUniformBlocks: uint32_t;
    maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: uint32_t;
    maxDescriptorSetInlineUniformBlocks: uint32_t;
    maxDescriptorSetUpdateAfterBindInlineUniformBlocks: uint32_t;
}
pub struct VkWriteDescriptorSetInlineUniformBlockEXT
{
    sType: VkStructureType;
    pNext: void;
    dataSize: uint32_t;
    pData: void;
}
pub struct VkDescriptorPoolInlineUniformBlockCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    maxInlineUniformBlockBindings: uint32_t;
}
pub struct VkPipelineCoverageModulationStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineCoverageModulationStateCreateFlagsNV;
    coverageModulationMode: VkCoverageModulationModeNV;
    coverageModulationTableEnable: VkBool32;
    coverageModulationTableCount: uint32_t;
    pCoverageModulationTable: float;
}
pub struct VkImageFormatListCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    viewFormatCount: uint32_t;
    pViewFormats: VkFormat;
}
pub struct VkImageFormatListCreateInfoKHR
{
}
pub struct VkValidationCacheCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkValidationCacheCreateFlagsEXT;
    initialDataSize: size_t;
    pInitialData: void;
}
pub struct VkShaderModuleValidationCacheCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    validationCache: VkValidationCacheEXT;
}
pub struct VkPhysicalDeviceMaintenance3Properties
{
    sType: VkStructureType;
    pNext: void;
    maxPerSetDescriptors: uint32_t;
    maxMemoryAllocationSize: VkDeviceSize;
}
pub struct VkPhysicalDeviceMaintenance3PropertiesKHR
{
}
pub struct VkDescriptorSetLayoutSupport
{
    sType: VkStructureType;
    pNext: void;
    supported: VkBool32;
}
pub struct VkDescriptorSetLayoutSupportKHR
{
}
pub struct VkPhysicalDeviceShaderDrawParametersFeatures
{
    sType: VkStructureType;
    pNext: void;
    shaderDrawParameters: VkBool32;
}
pub struct VkPhysicalDeviceShaderDrawParameterFeatures
{
}
pub struct VkPhysicalDeviceShaderFloat16Int8Features
{
    sType: VkStructureType;
    pNext: void;
    shaderFloat16: VkBool32;
    shaderInt8: VkBool32;
}
pub struct VkPhysicalDeviceShaderFloat16Int8FeaturesKHR
{
}
pub struct VkPhysicalDeviceFloat16Int8FeaturesKHR
{
}
pub struct VkPhysicalDeviceFloatControlsProperties
{
    sType: VkStructureType;
    pNext: void;
    denormBehaviorIndependence: VkShaderFloatControlsIndependence;
    roundingModeIndependence: VkShaderFloatControlsIndependence;
    shaderSignedZeroInfNanPreserveFloat16: VkBool32;
    shaderSignedZeroInfNanPreserveFloat32: VkBool32;
    shaderSignedZeroInfNanPreserveFloat64: VkBool32;
    shaderDenormPreserveFloat16: VkBool32;
    shaderDenormPreserveFloat32: VkBool32;
    shaderDenormPreserveFloat64: VkBool32;
    shaderDenormFlushToZeroFloat16: VkBool32;
    shaderDenormFlushToZeroFloat32: VkBool32;
    shaderDenormFlushToZeroFloat64: VkBool32;
    shaderRoundingModeRTEFloat16: VkBool32;
    shaderRoundingModeRTEFloat32: VkBool32;
    shaderRoundingModeRTEFloat64: VkBool32;
    shaderRoundingModeRTZFloat16: VkBool32;
    shaderRoundingModeRTZFloat32: VkBool32;
    shaderRoundingModeRTZFloat64: VkBool32;
}
pub struct VkPhysicalDeviceFloatControlsPropertiesKHR
{
}
pub struct VkPhysicalDeviceHostQueryResetFeatures
{
    sType: VkStructureType;
    pNext: void;
    hostQueryReset: VkBool32;
}
pub struct VkPhysicalDeviceHostQueryResetFeaturesEXT
{
}
pub struct VkNativeBufferUsage2ANDROID
{
    consumer: uint64_t;
    producer: uint64_t;
}
pub struct VkNativeBufferANDROID
{
    sType: VkStructureType;
    pNext: void;
    handle: void;
    stride: int;
    format: int;
    usage: int;
    usage2: VkNativeBufferUsage2ANDROID;
}
pub struct VkSwapchainImageCreateInfoANDROID
{
    sType: VkStructureType;
    pNext: void;
    usage: VkSwapchainImageUsageFlagsANDROID;
}
pub struct VkPhysicalDevicePresentationPropertiesANDROID
{
    sType: VkStructureType;
    pNext: void;
    sharedImage: VkBool32;
}
pub struct VkShaderResourceUsageAMD
{
    numUsedVgprs: uint32_t;
    numUsedSgprs: uint32_t;
    ldsSizePerLocalWorkGroup: uint32_t;
    ldsUsageSizeInBytes: size_t;
    scratchMemUsageInBytes: size_t;
}
pub struct VkShaderStatisticsInfoAMD
{
    shaderStageMask: VkShaderStageFlags;
    resourceUsage: VkShaderResourceUsageAMD;
    numPhysicalVgprs: uint32_t;
    numPhysicalSgprs: uint32_t;
    numAvailableVgprs: uint32_t;
    numAvailableSgprs: uint32_t;
    computeWorkGroupSize: uint32_t;
}
pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    globalPriority: VkQueueGlobalPriorityEXT;
}
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    globalPriorityQuery: VkBool32;
}
pub struct VkQueueFamilyGlobalPriorityPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    priorityCount: uint32_t;
    priorities: VkQueueGlobalPriorityEXT;
}
pub struct VkDebugUtilsObjectNameInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    objectType: VkObjectType;
    objectHandle: uint64_t;
    pObjectName: char;
}
pub struct VkDebugUtilsObjectTagInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    objectType: VkObjectType;
    objectHandle: uint64_t;
    tagName: uint64_t;
    tagSize: size_t;
    pTag: void;
}
pub struct VkDebugUtilsLabelEXT
{
    sType: VkStructureType;
    pNext: void;
    pLabelName: char;
    color: float;
}
pub struct VkDebugUtilsMessengerCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDebugUtilsMessengerCreateFlagsEXT;
    messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT;
    messageType: VkDebugUtilsMessageTypeFlagsEXT;
    pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT;
    pUserData: void;
}
pub struct VkDebugUtilsMessengerCallbackDataEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDebugUtilsMessengerCallbackDataFlagsEXT;
    pMessageIdName: char;
    messageIdNumber: int32_t;
    pMessage: char;
    queueLabelCount: uint32_t;
    pQueueLabels: VkDebugUtilsLabelEXT;
    cmdBufLabelCount: uint32_t;
    pCmdBufLabels: VkDebugUtilsLabelEXT;
    objectCount: uint32_t;
    pObjects: VkDebugUtilsObjectNameInfoEXT;
}
pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    deviceMemoryReport: VkBool32;
}
pub struct VkDeviceDeviceMemoryReportCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDeviceMemoryReportFlagsEXT;
    pfnUserCallback: PFN_vkDeviceMemoryReportCallbackEXT;
    pUserData: void;
}
pub struct VkDeviceMemoryReportCallbackDataEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDeviceMemoryReportFlagsEXT;
    type: VkDeviceMemoryReportEventTypeEXT;
    memoryObjectId: uint64_t;
    size: VkDeviceSize;
    objectType: VkObjectType;
    objectHandle: uint64_t;
    heapIndex: uint32_t;
}
pub struct VkImportMemoryHostPointerInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    handleType: VkExternalMemoryHandleTypeFlagBits;
    pHostPointer: void;
}
pub struct VkMemoryHostPointerPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    memoryTypeBits: uint32_t;
}
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    minImportedHostPointerAlignment: VkDeviceSize;
}
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    primitiveOverestimationSize: float;
    maxExtraPrimitiveOverestimationSize: float;
    extraPrimitiveOverestimationSizeGranularity: float;
    primitiveUnderestimation: VkBool32;
    conservativePointAndLineRasterization: VkBool32;
    degenerateTrianglesRasterized: VkBool32;
    degenerateLinesRasterized: VkBool32;
    fullyCoveredFragmentShaderInputVariable: VkBool32;
    conservativeRasterizationPostDepthCoverage: VkBool32;
}
pub struct VkCalibratedTimestampInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    timeDomain: VkTimeDomainEXT;
}
pub struct VkPhysicalDeviceShaderCorePropertiesAMD
{
    sType: VkStructureType;
    pNext: void;
    shaderEngineCount: uint32_t;
    shaderArraysPerEngineCount: uint32_t;
    computeUnitsPerShaderArray: uint32_t;
    simdPerComputeUnit: uint32_t;
    wavefrontsPerSimd: uint32_t;
    wavefrontSize: uint32_t;
    sgprsPerSimd: uint32_t;
    minSgprAllocation: uint32_t;
    maxSgprAllocation: uint32_t;
    sgprAllocationGranularity: uint32_t;
    vgprsPerSimd: uint32_t;
    minVgprAllocation: uint32_t;
    maxVgprAllocation: uint32_t;
    vgprAllocationGranularity: uint32_t;
}
pub struct VkPhysicalDeviceShaderCoreProperties2AMD
{
    sType: VkStructureType;
    pNext: void;
    shaderCoreFeatures: VkShaderCorePropertiesFlagsAMD;
    activeComputeUnitCount: uint32_t;
}
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT;
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT;
    extraPrimitiveOverestimationSize: float;
}
pub struct VkPhysicalDeviceDescriptorIndexingFeatures
{
    sType: VkStructureType;
    pNext: void;
    shaderInputAttachmentArrayDynamicIndexing: VkBool32;
    shaderUniformTexelBufferArrayDynamicIndexing: VkBool32;
    shaderStorageTexelBufferArrayDynamicIndexing: VkBool32;
    shaderUniformBufferArrayNonUniformIndexing: VkBool32;
    shaderSampledImageArrayNonUniformIndexing: VkBool32;
    shaderStorageBufferArrayNonUniformIndexing: VkBool32;
    shaderStorageImageArrayNonUniformIndexing: VkBool32;
    shaderInputAttachmentArrayNonUniformIndexing: VkBool32;
    shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32;
    shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32;
    descriptorBindingUniformBufferUpdateAfterBind: VkBool32;
    descriptorBindingSampledImageUpdateAfterBind: VkBool32;
    descriptorBindingStorageImageUpdateAfterBind: VkBool32;
    descriptorBindingStorageBufferUpdateAfterBind: VkBool32;
    descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32;
    descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32;
    descriptorBindingUpdateUnusedWhilePending: VkBool32;
    descriptorBindingPartiallyBound: VkBool32;
    descriptorBindingVariableDescriptorCount: VkBool32;
    runtimeDescriptorArray: VkBool32;
}
pub struct VkPhysicalDeviceDescriptorIndexingFeaturesEXT
{
}
pub struct VkPhysicalDeviceDescriptorIndexingProperties
{
    sType: VkStructureType;
    pNext: void;
    maxUpdateAfterBindDescriptorsInAllPools: uint32_t;
    shaderUniformBufferArrayNonUniformIndexingNative: VkBool32;
    shaderSampledImageArrayNonUniformIndexingNative: VkBool32;
    shaderStorageBufferArrayNonUniformIndexingNative: VkBool32;
    shaderStorageImageArrayNonUniformIndexingNative: VkBool32;
    shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32;
    robustBufferAccessUpdateAfterBind: VkBool32;
    quadDivergentImplicitLod: VkBool32;
    maxPerStageDescriptorUpdateAfterBindSamplers: uint32_t;
    maxPerStageDescriptorUpdateAfterBindUniformBuffers: uint32_t;
    maxPerStageDescriptorUpdateAfterBindStorageBuffers: uint32_t;
    maxPerStageDescriptorUpdateAfterBindSampledImages: uint32_t;
    maxPerStageDescriptorUpdateAfterBindStorageImages: uint32_t;
    maxPerStageDescriptorUpdateAfterBindInputAttachments: uint32_t;
    maxPerStageUpdateAfterBindResources: uint32_t;
    maxDescriptorSetUpdateAfterBindSamplers: uint32_t;
    maxDescriptorSetUpdateAfterBindUniformBuffers: uint32_t;
    maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: uint32_t;
    maxDescriptorSetUpdateAfterBindStorageBuffers: uint32_t;
    maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: uint32_t;
    maxDescriptorSetUpdateAfterBindSampledImages: uint32_t;
    maxDescriptorSetUpdateAfterBindStorageImages: uint32_t;
    maxDescriptorSetUpdateAfterBindInputAttachments: uint32_t;
}
pub struct VkPhysicalDeviceDescriptorIndexingPropertiesEXT
{
}
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    bindingCount: uint32_t;
    pBindingFlags: VkDescriptorBindingFlags;
}
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfoEXT
{
}
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo
{
    sType: VkStructureType;
    pNext: void;
    descriptorSetCount: uint32_t;
    pDescriptorCounts: uint32_t;
}
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfoEXT
{
}
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport
{
    sType: VkStructureType;
    pNext: void;
    maxVariableDescriptorCount: uint32_t;
}
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupportEXT
{
}
pub struct VkAttachmentDescription2
{
    sType: VkStructureType;
    pNext: void;
    flags: VkAttachmentDescriptionFlags;
    format: VkFormat;
    samples: VkSampleCountFlagBits;
    loadOp: VkAttachmentLoadOp;
    storeOp: VkAttachmentStoreOp;
    stencilLoadOp: VkAttachmentLoadOp;
    stencilStoreOp: VkAttachmentStoreOp;
    initialLayout: VkImageLayout;
    finalLayout: VkImageLayout;
}
pub struct VkAttachmentDescription2KHR
{
}
pub struct VkAttachmentReference2
{
    sType: VkStructureType;
    pNext: void;
    attachment: uint32_t;
    layout: VkImageLayout;
    aspectMask: VkImageAspectFlags;
}
pub struct VkAttachmentReference2KHR
{
}
pub struct VkSubpassDescription2
{
    sType: VkStructureType;
    pNext: void;
    flags: VkSubpassDescriptionFlags;
    pipelineBindPoint: VkPipelineBindPoint;
    viewMask: uint32_t;
    inputAttachmentCount: uint32_t;
    pInputAttachments: VkAttachmentReference2;
    colorAttachmentCount: uint32_t;
    pColorAttachments: VkAttachmentReference2;
    pResolveAttachments: VkAttachmentReference2;
    pDepthStencilAttachment: VkAttachmentReference2;
    preserveAttachmentCount: uint32_t;
    pPreserveAttachments: uint32_t;
}
pub struct VkSubpassDescription2KHR
{
}
pub struct VkSubpassDependency2
{
    sType: VkStructureType;
    pNext: void;
    srcSubpass: uint32_t;
    dstSubpass: uint32_t;
    srcStageMask: VkPipelineStageFlags;
    dstStageMask: VkPipelineStageFlags;
    srcAccessMask: VkAccessFlags;
    dstAccessMask: VkAccessFlags;
    dependencyFlags: VkDependencyFlags;
    viewOffset: int32_t;
}
pub struct VkSubpassDependency2KHR
{
}
pub struct VkRenderPassCreateInfo2
{
    sType: VkStructureType;
    pNext: void;
    flags: VkRenderPassCreateFlags;
    attachmentCount: uint32_t;
    pAttachments: VkAttachmentDescription2;
    subpassCount: uint32_t;
    pSubpasses: VkSubpassDescription2;
    dependencyCount: uint32_t;
    pDependencies: VkSubpassDependency2;
    correlatedViewMaskCount: uint32_t;
    pCorrelatedViewMasks: uint32_t;
}
pub struct VkRenderPassCreateInfo2KHR
{
}
pub struct VkSubpassBeginInfo
{
    sType: VkStructureType;
    pNext: void;
    contents: VkSubpassContents;
}
pub struct VkSubpassBeginInfoKHR
{
}
pub struct VkSubpassEndInfo
{
    sType: VkStructureType;
    pNext: void;
}
pub struct VkSubpassEndInfoKHR
{
}
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures
{
    sType: VkStructureType;
    pNext: void;
    timelineSemaphore: VkBool32;
}
pub struct VkPhysicalDeviceTimelineSemaphoreFeaturesKHR
{
}
pub struct VkPhysicalDeviceTimelineSemaphoreProperties
{
    sType: VkStructureType;
    pNext: void;
    maxTimelineSemaphoreValueDifference: uint64_t;
}
pub struct VkPhysicalDeviceTimelineSemaphorePropertiesKHR
{
}
pub struct VkSemaphoreTypeCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    semaphoreType: VkSemaphoreType;
    initialValue: uint64_t;
}
pub struct VkSemaphoreTypeCreateInfoKHR
{
}
pub struct VkTimelineSemaphoreSubmitInfo
{
    sType: VkStructureType;
    pNext: void;
    waitSemaphoreValueCount: uint32_t;
    pWaitSemaphoreValues: uint64_t;
    signalSemaphoreValueCount: uint32_t;
    pSignalSemaphoreValues: uint64_t;
}
pub struct VkTimelineSemaphoreSubmitInfoKHR
{
}
pub struct VkSemaphoreWaitInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkSemaphoreWaitFlags;
    semaphoreCount: uint32_t;
    pSemaphores: VkSemaphore;
    pValues: uint64_t;
}
pub struct VkSemaphoreWaitInfoKHR
{
}
pub struct VkSemaphoreSignalInfo
{
    sType: VkStructureType;
    pNext: void;
    semaphore: VkSemaphore;
    value: uint64_t;
}
pub struct VkSemaphoreSignalInfoKHR
{
}
pub struct VkVertexInputBindingDivisorDescriptionEXT
{
    binding: uint32_t;
    divisor: uint32_t;
}
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    vertexBindingDivisorCount: uint32_t;
    pVertexBindingDivisors: VkVertexInputBindingDivisorDescriptionEXT;
}
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    maxVertexAttribDivisor: uint32_t;
}
pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    pciDomain: uint32_t;
    pciBus: uint32_t;
    pciDevice: uint32_t;
    pciFunction: uint32_t;
}
pub struct VkImportAndroidHardwareBufferInfoANDROID
{
    sType: VkStructureType;
    pNext: void;
    buffer: AHardwareBuffer;
}
pub struct VkAndroidHardwareBufferUsageANDROID
{
    sType: VkStructureType;
    pNext: void;
    androidHardwareBufferUsage: uint64_t;
}
pub struct VkAndroidHardwareBufferPropertiesANDROID
{
    sType: VkStructureType;
    pNext: void;
    allocationSize: VkDeviceSize;
    memoryTypeBits: uint32_t;
}
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID
{
    sType: VkStructureType;
    pNext: void;
    memory: VkDeviceMemory;
}
pub struct VkAndroidHardwareBufferFormatPropertiesANDROID
{
    sType: VkStructureType;
    pNext: void;
    format: VkFormat;
    externalFormat: uint64_t;
    formatFeatures: VkFormatFeatureFlags;
    samplerYcbcrConversionComponents: VkComponentMapping;
    suggestedYcbcrModel: VkSamplerYcbcrModelConversion;
    suggestedYcbcrRange: VkSamplerYcbcrRange;
    suggestedXChromaOffset: VkChromaLocation;
    suggestedYChromaOffset: VkChromaLocation;
}
pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    conditionalRenderingEnable: VkBool32;
}
pub struct VkExternalFormatANDROID
{
    sType: VkStructureType;
    pNext: void;
    externalFormat: uint64_t;
}
pub struct VkPhysicalDevice8BitStorageFeatures
{
    sType: VkStructureType;
    pNext: void;
    storageBuffer8BitAccess: VkBool32;
    uniformAndStorageBuffer8BitAccess: VkBool32;
    storagePushConstant8: VkBool32;
}
pub struct VkPhysicalDevice8BitStorageFeaturesKHR
{
}
pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    conditionalRendering: VkBool32;
    inheritedConditionalRendering: VkBool32;
}
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures
{
    sType: VkStructureType;
    pNext: void;
    vulkanMemoryModel: VkBool32;
    vulkanMemoryModelDeviceScope: VkBool32;
    vulkanMemoryModelAvailabilityVisibilityChains: VkBool32;
}
pub struct VkPhysicalDeviceVulkanMemoryModelFeaturesKHR
{
}
pub struct VkPhysicalDeviceShaderAtomicInt64Features
{
    sType: VkStructureType;
    pNext: void;
    shaderBufferInt64Atomics: VkBool32;
    shaderSharedInt64Atomics: VkBool32;
}
pub struct VkPhysicalDeviceShaderAtomicInt64FeaturesKHR
{
}
pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    shaderBufferFloat32Atomics: VkBool32;
    shaderBufferFloat32AtomicAdd: VkBool32;
    shaderBufferFloat64Atomics: VkBool32;
    shaderBufferFloat64AtomicAdd: VkBool32;
    shaderSharedFloat32Atomics: VkBool32;
    shaderSharedFloat32AtomicAdd: VkBool32;
    shaderSharedFloat64Atomics: VkBool32;
    shaderSharedFloat64AtomicAdd: VkBool32;
    shaderImageFloat32Atomics: VkBool32;
    shaderImageFloat32AtomicAdd: VkBool32;
    sparseImageFloat32Atomics: VkBool32;
    sparseImageFloat32AtomicAdd: VkBool32;
}
pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    shaderBufferFloat16Atomics: VkBool32;
    shaderBufferFloat16AtomicAdd: VkBool32;
    shaderBufferFloat16AtomicMinMax: VkBool32;
    shaderBufferFloat32AtomicMinMax: VkBool32;
    shaderBufferFloat64AtomicMinMax: VkBool32;
    shaderSharedFloat16Atomics: VkBool32;
    shaderSharedFloat16AtomicAdd: VkBool32;
    shaderSharedFloat16AtomicMinMax: VkBool32;
    shaderSharedFloat32AtomicMinMax: VkBool32;
    shaderSharedFloat64AtomicMinMax: VkBool32;
    shaderImageFloat32AtomicMinMax: VkBool32;
    sparseImageFloat32AtomicMinMax: VkBool32;
}
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    vertexAttributeInstanceRateDivisor: VkBool32;
    vertexAttributeInstanceRateZeroDivisor: VkBool32;
}
pub struct VkQueueFamilyCheckpointPropertiesNV
{
    sType: VkStructureType;
    pNext: void;
    checkpointExecutionStageMask: VkPipelineStageFlags;
}
pub struct VkCheckpointDataNV
{
    sType: VkStructureType;
    pNext: void;
    stage: VkPipelineStageFlagBits;
    pCheckpointMarker: void;
}
pub struct VkPhysicalDeviceDepthStencilResolveProperties
{
    sType: VkStructureType;
    pNext: void;
    supportedDepthResolveModes: VkResolveModeFlags;
    supportedStencilResolveModes: VkResolveModeFlags;
    independentResolveNone: VkBool32;
    independentResolve: VkBool32;
}
pub struct VkPhysicalDeviceDepthStencilResolvePropertiesKHR
{
}
pub struct VkSubpassDescriptionDepthStencilResolve
{
    sType: VkStructureType;
    pNext: void;
    depthResolveMode: VkResolveModeFlagBits;
    stencilResolveMode: VkResolveModeFlagBits;
    pDepthStencilResolveAttachment: VkAttachmentReference2;
}
pub struct VkSubpassDescriptionDepthStencilResolveKHR
{
}
pub struct VkImageViewASTCDecodeModeEXT
{
    sType: VkStructureType;
    pNext: void;
    decodeMode: VkFormat;
}
pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    decodeModeSharedExponent: VkBool32;
}
pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    transformFeedback: VkBool32;
    geometryStreams: VkBool32;
}
pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    maxTransformFeedbackStreams: uint32_t;
    maxTransformFeedbackBuffers: uint32_t;
    maxTransformFeedbackBufferSize: VkDeviceSize;
    maxTransformFeedbackStreamDataSize: uint32_t;
    maxTransformFeedbackBufferDataSize: uint32_t;
    maxTransformFeedbackBufferDataStride: uint32_t;
    transformFeedbackQueries: VkBool32;
    transformFeedbackStreamsLinesTriangles: VkBool32;
    transformFeedbackRasterizationStreamSelect: VkBool32;
    transformFeedbackDraw: VkBool32;
}
pub struct VkPipelineRasterizationStateStreamCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineRasterizationStateStreamCreateFlagsEXT;
    rasterizationStream: uint32_t;
}
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    representativeFragmentTest: VkBool32;
}
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    representativeFragmentTestEnable: VkBool32;
}
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    exclusiveScissor: VkBool32;
}
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    exclusiveScissorCount: uint32_t;
    pExclusiveScissors: VkRect2D;
}
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    cornerSampledImage: VkBool32;
}
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    computeDerivativeGroupQuads: VkBool32;
    computeDerivativeGroupLinear: VkBool32;
}
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    fragmentShaderBarycentric: VkBool32;
}
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    imageFootprint: VkBool32;
}
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    dedicatedAllocationImageAliasing: VkBool32;
}
pub struct VkShadingRatePaletteNV
{
    shadingRatePaletteEntryCount: uint32_t;
    pShadingRatePaletteEntries: VkShadingRatePaletteEntryNV;
}
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    shadingRateImageEnable: VkBool32;
    viewportCount: uint32_t;
    pShadingRatePalettes: VkShadingRatePaletteNV;
}
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    shadingRateImage: VkBool32;
    shadingRateCoarseSampleOrder: VkBool32;
}
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV
{
    sType: VkStructureType;
    pNext: void;
    shadingRateTexelSize: VkExtent2D;
    shadingRatePaletteSize: uint32_t;
    shadingRateMaxCoarseSamples: uint32_t;
}
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI
{
    sType: VkStructureType;
    pNext: void;
    invocationMask: VkBool32;
}
pub struct VkCoarseSampleLocationNV
{
    pixelX: uint32_t;
    pixelY: uint32_t;
    sample: uint32_t;
}
pub struct VkCoarseSampleOrderCustomNV
{
    shadingRate: VkShadingRatePaletteEntryNV;
    sampleCount: uint32_t;
    sampleLocationCount: uint32_t;
    pSampleLocations: VkCoarseSampleLocationNV;
}
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    sampleOrderType: VkCoarseSampleOrderTypeNV;
    customSampleOrderCount: uint32_t;
    pCustomSampleOrders: VkCoarseSampleOrderCustomNV;
}
pub struct VkPhysicalDeviceMeshShaderFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    taskShader: VkBool32;
    meshShader: VkBool32;
}
pub struct VkPhysicalDeviceMeshShaderPropertiesNV
{
    sType: VkStructureType;
    pNext: void;
    maxDrawMeshTasksCount: uint32_t;
    maxTaskWorkGroupInvocations: uint32_t;
    maxTaskWorkGroupSize: uint32_t;
    maxTaskTotalMemorySize: uint32_t;
    maxTaskOutputCount: uint32_t;
    maxMeshWorkGroupInvocations: uint32_t;
    maxMeshWorkGroupSize: uint32_t;
    maxMeshTotalMemorySize: uint32_t;
    maxMeshOutputVertices: uint32_t;
    maxMeshOutputPrimitives: uint32_t;
    maxMeshMultiviewViewCount: uint32_t;
    meshOutputPerVertexGranularity: uint32_t;
    meshOutputPerPrimitiveGranularity: uint32_t;
}
pub struct VkDrawMeshTasksIndirectCommandNV
{
    taskCount: uint32_t;
    firstTask: uint32_t;
}
pub struct VkRayTracingShaderGroupCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    type: VkRayTracingShaderGroupTypeKHR;
    generalShader: uint32_t;
    closestHitShader: uint32_t;
    anyHitShader: uint32_t;
    intersectionShader: uint32_t;
}
pub struct VkRayTracingShaderGroupCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    type: VkRayTracingShaderGroupTypeKHR;
    generalShader: uint32_t;
    closestHitShader: uint32_t;
    anyHitShader: uint32_t;
    intersectionShader: uint32_t;
    pShaderGroupCaptureReplayHandle: void;
}
pub struct VkRayTracingPipelineCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineCreateFlags;
    stageCount: uint32_t;
    pStages: VkPipelineShaderStageCreateInfo;
    groupCount: uint32_t;
    pGroups: VkRayTracingShaderGroupCreateInfoNV;
    maxRecursionDepth: uint32_t;
    layout: VkPipelineLayout;
    basePipelineHandle: VkPipeline;
    basePipelineIndex: int32_t;
}
pub struct VkRayTracingPipelineCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineCreateFlags;
    stageCount: uint32_t;
    pStages: VkPipelineShaderStageCreateInfo;
    groupCount: uint32_t;
    pGroups: VkRayTracingShaderGroupCreateInfoKHR;
    maxPipelineRayRecursionDepth: uint32_t;
    pLibraryInfo: VkPipelineLibraryCreateInfoKHR;
    pLibraryInterface: VkRayTracingPipelineInterfaceCreateInfoKHR;
    pDynamicState: VkPipelineDynamicStateCreateInfo;
    layout: VkPipelineLayout;
    basePipelineHandle: VkPipeline;
    basePipelineIndex: int32_t;
}
pub struct VkGeometryTrianglesNV
{
    sType: VkStructureType;
    pNext: void;
    vertexData: VkBuffer;
    vertexOffset: VkDeviceSize;
    vertexCount: uint32_t;
    vertexStride: VkDeviceSize;
    vertexFormat: VkFormat;
    indexData: VkBuffer;
    indexOffset: VkDeviceSize;
    indexCount: uint32_t;
    indexType: VkIndexType;
    transformData: VkBuffer;
    transformOffset: VkDeviceSize;
}
pub struct VkGeometryAABBNV
{
    sType: VkStructureType;
    pNext: void;
    aabbData: VkBuffer;
    numAABBs: uint32_t;
    stride: uint32_t;
    offset: VkDeviceSize;
}
pub struct VkGeometryDataNV
{
    triangles: VkGeometryTrianglesNV;
    aabbs: VkGeometryAABBNV;
}
pub struct VkGeometryNV
{
    sType: VkStructureType;
    pNext: void;
    geometryType: VkGeometryTypeKHR;
    geometry: VkGeometryDataNV;
    flags: VkGeometryFlagsKHR;
}
pub struct VkAccelerationStructureInfoNV
{
    sType: VkStructureType;
    pNext: void;
    type: VkAccelerationStructureTypeNV;
    flags: VkBuildAccelerationStructureFlagsNV;
    instanceCount: uint32_t;
    geometryCount: uint32_t;
    pGeometries: VkGeometryNV;
}
pub struct VkAccelerationStructureCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    compactedSize: VkDeviceSize;
    info: VkAccelerationStructureInfoNV;
}
pub struct VkBindAccelerationStructureMemoryInfoNV
{
    sType: VkStructureType;
    pNext: void;
    accelerationStructure: VkAccelerationStructureNV;
    memory: VkDeviceMemory;
    memoryOffset: VkDeviceSize;
    deviceIndexCount: uint32_t;
    pDeviceIndices: uint32_t;
}
pub struct VkWriteDescriptorSetAccelerationStructureKHR
{
    sType: VkStructureType;
    pNext: void;
    accelerationStructureCount: uint32_t;
    pAccelerationStructures: VkAccelerationStructureKHR;
}
pub struct VkWriteDescriptorSetAccelerationStructureNV
{
    sType: VkStructureType;
    pNext: void;
    accelerationStructureCount: uint32_t;
    pAccelerationStructures: VkAccelerationStructureNV;
}
pub struct VkAccelerationStructureMemoryRequirementsInfoNV
{
    sType: VkStructureType;
    pNext: void;
    type: VkAccelerationStructureMemoryRequirementsTypeNV;
    accelerationStructure: VkAccelerationStructureNV;
}
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    accelerationStructure: VkBool32;
    accelerationStructureCaptureReplay: VkBool32;
    accelerationStructureIndirectBuild: VkBool32;
    accelerationStructureHostCommands: VkBool32;
    descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32;
}
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    rayTracingPipeline: VkBool32;
    rayTracingPipelineShaderGroupHandleCaptureReplay: VkBool32;
    rayTracingPipelineShaderGroupHandleCaptureReplayMixed: VkBool32;
    rayTracingPipelineTraceRaysIndirect: VkBool32;
    rayTraversalPrimitiveCulling: VkBool32;
}
pub struct VkPhysicalDeviceRayQueryFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    rayQuery: VkBool32;
}
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    maxGeometryCount: uint64_t;
    maxInstanceCount: uint64_t;
    maxPrimitiveCount: uint64_t;
    maxPerStageDescriptorAccelerationStructures: uint32_t;
    maxPerStageDescriptorUpdateAfterBindAccelerationStructures: uint32_t;
    maxDescriptorSetAccelerationStructures: uint32_t;
    maxDescriptorSetUpdateAfterBindAccelerationStructures: uint32_t;
    minAccelerationStructureScratchOffsetAlignment: uint32_t;
}
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    shaderGroupHandleSize: uint32_t;
    maxRayRecursionDepth: uint32_t;
    maxShaderGroupStride: uint32_t;
    shaderGroupBaseAlignment: uint32_t;
    shaderGroupHandleCaptureReplaySize: uint32_t;
    maxRayDispatchInvocationCount: uint32_t;
    shaderGroupHandleAlignment: uint32_t;
    maxRayHitAttributeSize: uint32_t;
}
pub struct VkPhysicalDeviceRayTracingPropertiesNV
{
    sType: VkStructureType;
    pNext: void;
    shaderGroupHandleSize: uint32_t;
    maxRecursionDepth: uint32_t;
    maxShaderGroupStride: uint32_t;
    shaderGroupBaseAlignment: uint32_t;
    maxGeometryCount: uint64_t;
    maxInstanceCount: uint64_t;
    maxTriangleCount: uint64_t;
    maxDescriptorSetAccelerationStructures: uint32_t;
}
pub struct VkStridedDeviceAddressRegionKHR
{
    deviceAddress: VkDeviceAddress;
    stride: VkDeviceSize;
    size: VkDeviceSize;
}
pub struct VkTraceRaysIndirectCommandKHR
{
    width: uint32_t;
    height: uint32_t;
    depth: uint32_t;
}
pub struct VkDrmFormatModifierPropertiesListEXT
{
    sType: VkStructureType;
    pNext: void;
    drmFormatModifierCount: uint32_t;
    pDrmFormatModifierProperties: VkDrmFormatModifierPropertiesEXT;
}
pub struct VkDrmFormatModifierPropertiesEXT
{
    drmFormatModifier: uint64_t;
    drmFormatModifierPlaneCount: uint32_t;
    drmFormatModifierTilingFeatures: VkFormatFeatureFlags;
}
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    drmFormatModifier: uint64_t;
    sharingMode: VkSharingMode;
    queueFamilyIndexCount: uint32_t;
    pQueueFamilyIndices: uint32_t;
}
pub struct VkImageDrmFormatModifierListCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    drmFormatModifierCount: uint32_t;
    pDrmFormatModifiers: uint64_t;
}
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    drmFormatModifier: uint64_t;
    drmFormatModifierPlaneCount: uint32_t;
    pPlaneLayouts: VkSubresourceLayout;
}
pub struct VkImageDrmFormatModifierPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    drmFormatModifier: uint64_t;
}
pub struct VkImageStencilUsageCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    stencilUsage: VkImageUsageFlags;
}
pub struct VkImageStencilUsageCreateInfoEXT
{
}
pub struct VkDeviceMemoryOverallocationCreateInfoAMD
{
    sType: VkStructureType;
    pNext: void;
    overallocationBehavior: VkMemoryOverallocationBehaviorAMD;
}
pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    fragmentDensityMap: VkBool32;
    fragmentDensityMapDynamic: VkBool32;
    fragmentDensityMapNonSubsampledImages: VkBool32;
}
pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    fragmentDensityMapDeferred: VkBool32;
}
pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    minFragmentDensityTexelSize: VkExtent2D;
    maxFragmentDensityTexelSize: VkExtent2D;
    fragmentDensityInvocations: VkBool32;
}
pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    subsampledLoads: VkBool32;
    subsampledCoarseReconstructionEarlyAccess: VkBool32;
    maxSubsampledArrayLayers: uint32_t;
    maxDescriptorSetSubsampledSamplers: uint32_t;
}
pub struct VkRenderPassFragmentDensityMapCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    fragmentDensityMapAttachment: VkAttachmentReference;
}
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures
{
    sType: VkStructureType;
    pNext: void;
    scalarBlockLayout: VkBool32;
}
pub struct VkPhysicalDeviceScalarBlockLayoutFeaturesEXT
{
}
pub struct VkSurfaceProtectedCapabilitiesKHR
{
    sType: VkStructureType;
    pNext: void;
    supportsProtected: VkBool32;
}
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures
{
    sType: VkStructureType;
    pNext: void;
    uniformBufferStandardLayout: VkBool32;
}
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR
{
}
pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    depthClipEnable: VkBool32;
}
pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineRasterizationDepthClipStateCreateFlagsEXT;
    depthClipEnable: VkBool32;
}
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    heapBudget: VkDeviceSize;
    heapUsage: VkDeviceSize;
}
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    memoryPriority: VkBool32;
}
pub struct VkMemoryPriorityAllocateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    priority: float;
}
pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    pageableDeviceLocalMemory: VkBool32;
}
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures
{
    sType: VkStructureType;
    pNext: void;
    bufferDeviceAddress: VkBool32;
    bufferDeviceAddressCaptureReplay: VkBool32;
    bufferDeviceAddressMultiDevice: VkBool32;
}
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesKHR
{
}
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    bufferDeviceAddress: VkBool32;
    bufferDeviceAddressCaptureReplay: VkBool32;
    bufferDeviceAddressMultiDevice: VkBool32;
}
pub struct VkPhysicalDeviceBufferAddressFeaturesEXT
{
}
pub struct VkBufferDeviceAddressInfo
{
    sType: VkStructureType;
    pNext: void;
    buffer: VkBuffer;
}
pub struct VkBufferDeviceAddressInfoKHR
{
}
pub struct VkBufferDeviceAddressInfoEXT
{
}
pub struct VkBufferOpaqueCaptureAddressCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    opaqueCaptureAddress: uint64_t;
}
pub struct VkBufferOpaqueCaptureAddressCreateInfoKHR
{
}
pub struct VkBufferDeviceAddressCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    deviceAddress: VkDeviceAddress;
}
pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    imageViewType: VkImageViewType;
}
pub struct VkFilterCubicImageViewImageFormatPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    filterCubic: VkBool32;
    filterCubicMinmax: VkBool32;
}
pub struct VkPhysicalDeviceImagelessFramebufferFeatures
{
    sType: VkStructureType;
    pNext: void;
    imagelessFramebuffer: VkBool32;
}
pub struct VkPhysicalDeviceImagelessFramebufferFeaturesKHR
{
}
pub struct VkFramebufferAttachmentsCreateInfo
{
    sType: VkStructureType;
    pNext: void;
    attachmentImageInfoCount: uint32_t;
    pAttachmentImageInfos: VkFramebufferAttachmentImageInfo;
}
pub struct VkFramebufferAttachmentsCreateInfoKHR
{
}
pub struct VkFramebufferAttachmentImageInfo
{
    sType: VkStructureType;
    pNext: void;
    flags: VkImageCreateFlags;
    usage: VkImageUsageFlags;
    width: uint32_t;
    height: uint32_t;
    layerCount: uint32_t;
    viewFormatCount: uint32_t;
    pViewFormats: VkFormat;
}
pub struct VkFramebufferAttachmentImageInfoKHR
{
}
pub struct VkRenderPassAttachmentBeginInfo
{
    sType: VkStructureType;
    pNext: void;
    attachmentCount: uint32_t;
    pAttachments: VkImageView;
}
pub struct VkRenderPassAttachmentBeginInfoKHR
{
}
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    textureCompressionASTC_HDR: VkBool32;
}
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    cooperativeMatrix: VkBool32;
    cooperativeMatrixRobustBufferAccess: VkBool32;
}
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV
{
    sType: VkStructureType;
    pNext: void;
    cooperativeMatrixSupportedStages: VkShaderStageFlags;
}
pub struct VkCooperativeMatrixPropertiesNV
{
    sType: VkStructureType;
    pNext: void;
    MSize: uint32_t;
    NSize: uint32_t;
    KSize: uint32_t;
    AType: VkComponentTypeNV;
    BType: VkComponentTypeNV;
    CType: VkComponentTypeNV;
    DType: VkComponentTypeNV;
    scope: VkScopeNV;
}
pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    ycbcrImageArrays: VkBool32;
}
pub struct VkImageViewHandleInfoNVX
{
    sType: VkStructureType;
    pNext: void;
    imageView: VkImageView;
    descriptorType: VkDescriptorType;
    sampler: VkSampler;
}
pub struct VkImageViewAddressPropertiesNVX
{
    sType: VkStructureType;
    pNext: void;
    deviceAddress: VkDeviceAddress;
    size: VkDeviceSize;
}
pub struct VkPresentFrameTokenGGP
{
    sType: VkStructureType;
    pNext: void;
    frameToken: GgpFrameToken;
}
pub struct VkPipelineCreationFeedbackEXT
{
    flags: VkPipelineCreationFeedbackFlagsEXT;
    duration: uint64_t;
}
pub struct VkPipelineCreationFeedbackCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    pPipelineCreationFeedback: VkPipelineCreationFeedbackEXT;
    pipelineStageCreationFeedbackCount: uint32_t;
    pPipelineStageCreationFeedbacks: VkPipelineCreationFeedbackEXT;
}
pub struct VkSurfaceFullScreenExclusiveInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    fullScreenExclusive: VkFullScreenExclusiveEXT;
}
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT
{
    sType: VkStructureType;
    pNext: void;
    hmonitor: HMONITOR;
}
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT
{
    sType: VkStructureType;
    pNext: void;
    fullScreenExclusiveSupported: VkBool32;
}
pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    performanceCounterQueryPools: VkBool32;
    performanceCounterMultipleQueryPools: VkBool32;
}
pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    allowCommandBufferQueryCopies: VkBool32;
}
pub struct VkPerformanceCounterKHR
{
    sType: VkStructureType;
    pNext: void;
    unit: VkPerformanceCounterUnitKHR;
    scope: VkPerformanceCounterScopeKHR;
    storage: VkPerformanceCounterStorageKHR;
    uuid: uint8_t;
}
pub struct VkPerformanceCounterDescriptionKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPerformanceCounterDescriptionFlagsKHR;
    name: char;
    category: char;
    description: char;
}
pub struct VkQueryPoolPerformanceCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    queueFamilyIndex: uint32_t;
    counterIndexCount: uint32_t;
    pCounterIndices: uint32_t;
}
pub struct VkAcquireProfilingLockInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkAcquireProfilingLockFlagsKHR;
    timeout: uint64_t;
}
pub struct VkPerformanceQuerySubmitInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    counterPassIndex: uint32_t;
}
pub struct VkHeadlessSurfaceCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkHeadlessSurfaceCreateFlagsEXT;
}
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    coverageReductionMode: VkBool32;
}
pub struct VkPipelineCoverageReductionStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    flags: VkPipelineCoverageReductionStateCreateFlagsNV;
    coverageReductionMode: VkCoverageReductionModeNV;
}
pub struct VkFramebufferMixedSamplesCombinationNV
{
    sType: VkStructureType;
    pNext: void;
    coverageReductionMode: VkCoverageReductionModeNV;
    rasterizationSamples: VkSampleCountFlagBits;
    depthStencilSamples: VkSampleCountFlags;
    colorSamples: VkSampleCountFlags;
}
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL
{
    sType: VkStructureType;
    pNext: void;
    shaderIntegerFunctions2: VkBool32;
}
pub struct VkPerformanceValueINTEL
{
    type: VkPerformanceValueTypeINTEL;
    data: VkPerformanceValueDataINTEL;
}
pub struct VkInitializePerformanceApiInfoINTEL
{
    sType: VkStructureType;
    pNext: void;
    pUserData: void;
}
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL
{
    sType: VkStructureType;
    pNext: void;
    performanceCountersSampling: VkQueryPoolSamplingModeINTEL;
}
pub struct VkQueryPoolCreateInfoINTEL
{
}
pub struct VkPerformanceMarkerInfoINTEL
{
    sType: VkStructureType;
    pNext: void;
    marker: uint64_t;
}
pub struct VkPerformanceStreamMarkerInfoINTEL
{
    sType: VkStructureType;
    pNext: void;
    marker: uint32_t;
}
pub struct VkPerformanceOverrideInfoINTEL
{
    sType: VkStructureType;
    pNext: void;
    type: VkPerformanceOverrideTypeINTEL;
    enable: VkBool32;
    parameter: uint64_t;
}
pub struct VkPerformanceConfigurationAcquireInfoINTEL
{
    sType: VkStructureType;
    pNext: void;
    type: VkPerformanceConfigurationTypeINTEL;
}
pub struct VkPhysicalDeviceShaderClockFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    shaderSubgroupClock: VkBool32;
    shaderDeviceClock: VkBool32;
}
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    indexTypeUint8: VkBool32;
}
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV
{
    sType: VkStructureType;
    pNext: void;
    shaderSMCount: uint32_t;
    shaderWarpsPerSM: uint32_t;
}
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    shaderSMBuiltins: VkBool32;
}
pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    fragmentShaderSampleInterlock: VkBool32;
    fragmentShaderPixelInterlock: VkBool32;
    fragmentShaderShadingRateInterlock: VkBool32;
}
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures
{
    sType: VkStructureType;
    pNext: void;
    separateDepthStencilLayouts: VkBool32;
}
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR
{
}
pub struct VkAttachmentReferenceStencilLayout
{
    sType: VkStructureType;
    pNext: void;
    stencilLayout: VkImageLayout;
}
pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    primitiveTopologyListRestart: VkBool32;
    primitiveTopologyPatchListRestart: VkBool32;
}
pub struct VkAttachmentReferenceStencilLayoutKHR
{
}
pub struct VkAttachmentDescriptionStencilLayout
{
    sType: VkStructureType;
    pNext: void;
    stencilInitialLayout: VkImageLayout;
    stencilFinalLayout: VkImageLayout;
}
pub struct VkAttachmentDescriptionStencilLayoutKHR
{
}
pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    pipelineExecutableInfo: VkBool32;
}
pub struct VkPipelineInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    pipeline: VkPipeline;
}
pub struct VkPipelineExecutablePropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    stages: VkShaderStageFlags;
    name: char;
    description: char;
    subgroupSize: uint32_t;
}
pub struct VkPipelineExecutableInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    pipeline: VkPipeline;
    executableIndex: uint32_t;
}
pub struct VkPipelineExecutableStatisticKHR
{
    sType: VkStructureType;
    pNext: void;
    name: char;
    description: char;
    format: VkPipelineExecutableStatisticFormatKHR;
    value: VkPipelineExecutableStatisticValueKHR;
}
pub struct VkPipelineExecutableInternalRepresentationKHR
{
    sType: VkStructureType;
    pNext: void;
    name: char;
    description: char;
    isText: VkBool32;
    dataSize: size_t;
    pData: void;
}
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    shaderDemoteToHelperInvocation: VkBool32;
}
pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    texelBufferAlignment: VkBool32;
}
pub struct VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    storageTexelBufferOffsetAlignmentBytes: VkDeviceSize;
    storageTexelBufferOffsetSingleTexelAlignment: VkBool32;
    uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize;
    uniformTexelBufferOffsetSingleTexelAlignment: VkBool32;
}
pub struct VkPhysicalDeviceSubgroupSizeControlFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    subgroupSizeControl: VkBool32;
    computeFullSubgroups: VkBool32;
}
pub struct VkPhysicalDeviceSubgroupSizeControlPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    minSubgroupSize: uint32_t;
    maxSubgroupSize: uint32_t;
    maxComputeWorkgroupSubgroups: uint32_t;
    requiredSubgroupSizeStages: VkShaderStageFlags;
}
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    requiredSubgroupSize: uint32_t;
}
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI
{
    sType: VkStructureType;
    pNext: void;
    renderPass: VkRenderPass;
    subpass: uint32_t;
}
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI
{
    sType: VkStructureType;
    pNext: void;
    maxSubpassShadingWorkgroupSizeAspectRatio: uint32_t;
}
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo
{
    sType: VkStructureType;
    pNext: void;
    opaqueCaptureAddress: uint64_t;
}
pub struct VkMemoryOpaqueCaptureAddressAllocateInfoKHR
{
}
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo
{
    sType: VkStructureType;
    pNext: void;
    memory: VkDeviceMemory;
}
pub struct VkDeviceMemoryOpaqueCaptureAddressInfoKHR
{
}
pub struct VkPhysicalDeviceLineRasterizationFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    rectangularLines: VkBool32;
    bresenhamLines: VkBool32;
    smoothLines: VkBool32;
    stippledRectangularLines: VkBool32;
    stippledBresenhamLines: VkBool32;
    stippledSmoothLines: VkBool32;
}
pub struct VkPhysicalDeviceLineRasterizationPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    lineSubPixelPrecisionBits: uint32_t;
}
pub struct VkPipelineRasterizationLineStateCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    lineRasterizationMode: VkLineRasterizationModeEXT;
    stippledLineEnable: VkBool32;
    lineStippleFactor: uint32_t;
    lineStipplePattern: uint16_t;
}
pub struct VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    pipelineCreationCacheControl: VkBool32;
}
pub struct VkPhysicalDeviceVulkan11Features
{
    sType: VkStructureType;
    pNext: void;
    storageBuffer16BitAccess: VkBool32;
    uniformAndStorageBuffer16BitAccess: VkBool32;
    storagePushConstant16: VkBool32;
    storageInputOutput16: VkBool32;
    multiview: VkBool32;
    multiviewGeometryShader: VkBool32;
    multiviewTessellationShader: VkBool32;
    variablePointersStorageBuffer: VkBool32;
    variablePointers: VkBool32;
    protectedMemory: VkBool32;
    samplerYcbcrConversion: VkBool32;
    shaderDrawParameters: VkBool32;
}
pub struct VkPhysicalDeviceVulkan11Properties
{
    sType: VkStructureType;
    pNext: void;
    deviceUUID: uint8_t;
    driverUUID: uint8_t;
    deviceLUID: uint8_t;
    deviceNodeMask: uint32_t;
    deviceLUIDValid: VkBool32;
    subgroupSize: uint32_t;
    subgroupSupportedStages: VkShaderStageFlags;
    subgroupSupportedOperations: VkSubgroupFeatureFlags;
    subgroupQuadOperationsInAllStages: VkBool32;
    pointClippingBehavior: VkPointClippingBehavior;
    maxMultiviewViewCount: uint32_t;
    maxMultiviewInstanceIndex: uint32_t;
    protectedNoFault: VkBool32;
    maxPerSetDescriptors: uint32_t;
    maxMemoryAllocationSize: VkDeviceSize;
}
pub struct VkPhysicalDeviceVulkan12Features
{
    sType: VkStructureType;
    pNext: void;
    samplerMirrorClampToEdge: VkBool32;
    drawIndirectCount: VkBool32;
    storageBuffer8BitAccess: VkBool32;
    uniformAndStorageBuffer8BitAccess: VkBool32;
    storagePushConstant8: VkBool32;
    shaderBufferInt64Atomics: VkBool32;
    shaderSharedInt64Atomics: VkBool32;
    shaderFloat16: VkBool32;
    shaderInt8: VkBool32;
    descriptorIndexing: VkBool32;
    shaderInputAttachmentArrayDynamicIndexing: VkBool32;
    shaderUniformTexelBufferArrayDynamicIndexing: VkBool32;
    shaderStorageTexelBufferArrayDynamicIndexing: VkBool32;
    shaderUniformBufferArrayNonUniformIndexing: VkBool32;
    shaderSampledImageArrayNonUniformIndexing: VkBool32;
    shaderStorageBufferArrayNonUniformIndexing: VkBool32;
    shaderStorageImageArrayNonUniformIndexing: VkBool32;
    shaderInputAttachmentArrayNonUniformIndexing: VkBool32;
    shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32;
    shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32;
    descriptorBindingUniformBufferUpdateAfterBind: VkBool32;
    descriptorBindingSampledImageUpdateAfterBind: VkBool32;
    descriptorBindingStorageImageUpdateAfterBind: VkBool32;
    descriptorBindingStorageBufferUpdateAfterBind: VkBool32;
    descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32;
    descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32;
    descriptorBindingUpdateUnusedWhilePending: VkBool32;
    descriptorBindingPartiallyBound: VkBool32;
    descriptorBindingVariableDescriptorCount: VkBool32;
    runtimeDescriptorArray: VkBool32;
    samplerFilterMinmax: VkBool32;
    scalarBlockLayout: VkBool32;
    imagelessFramebuffer: VkBool32;
    uniformBufferStandardLayout: VkBool32;
    shaderSubgroupExtendedTypes: VkBool32;
    separateDepthStencilLayouts: VkBool32;
    hostQueryReset: VkBool32;
    timelineSemaphore: VkBool32;
    bufferDeviceAddress: VkBool32;
    bufferDeviceAddressCaptureReplay: VkBool32;
    bufferDeviceAddressMultiDevice: VkBool32;
    vulkanMemoryModel: VkBool32;
    vulkanMemoryModelDeviceScope: VkBool32;
    vulkanMemoryModelAvailabilityVisibilityChains: VkBool32;
    shaderOutputViewportIndex: VkBool32;
    shaderOutputLayer: VkBool32;
    subgroupBroadcastDynamicId: VkBool32;
}
pub struct VkPhysicalDeviceVulkan12Properties
{
    sType: VkStructureType;
    pNext: void;
    driverID: VkDriverId;
    driverName: char;
    driverInfo: char;
    conformanceVersion: VkConformanceVersion;
    denormBehaviorIndependence: VkShaderFloatControlsIndependence;
    roundingModeIndependence: VkShaderFloatControlsIndependence;
    shaderSignedZeroInfNanPreserveFloat16: VkBool32;
    shaderSignedZeroInfNanPreserveFloat32: VkBool32;
    shaderSignedZeroInfNanPreserveFloat64: VkBool32;
    shaderDenormPreserveFloat16: VkBool32;
    shaderDenormPreserveFloat32: VkBool32;
    shaderDenormPreserveFloat64: VkBool32;
    shaderDenormFlushToZeroFloat16: VkBool32;
    shaderDenormFlushToZeroFloat32: VkBool32;
    shaderDenormFlushToZeroFloat64: VkBool32;
    shaderRoundingModeRTEFloat16: VkBool32;
    shaderRoundingModeRTEFloat32: VkBool32;
    shaderRoundingModeRTEFloat64: VkBool32;
    shaderRoundingModeRTZFloat16: VkBool32;
    shaderRoundingModeRTZFloat32: VkBool32;
    shaderRoundingModeRTZFloat64: VkBool32;
    maxUpdateAfterBindDescriptorsInAllPools: uint32_t;
    shaderUniformBufferArrayNonUniformIndexingNative: VkBool32;
    shaderSampledImageArrayNonUniformIndexingNative: VkBool32;
    shaderStorageBufferArrayNonUniformIndexingNative: VkBool32;
    shaderStorageImageArrayNonUniformIndexingNative: VkBool32;
    shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32;
    robustBufferAccessUpdateAfterBind: VkBool32;
    quadDivergentImplicitLod: VkBool32;
    maxPerStageDescriptorUpdateAfterBindSamplers: uint32_t;
    maxPerStageDescriptorUpdateAfterBindUniformBuffers: uint32_t;
    maxPerStageDescriptorUpdateAfterBindStorageBuffers: uint32_t;
    maxPerStageDescriptorUpdateAfterBindSampledImages: uint32_t;
    maxPerStageDescriptorUpdateAfterBindStorageImages: uint32_t;
    maxPerStageDescriptorUpdateAfterBindInputAttachments: uint32_t;
    maxPerStageUpdateAfterBindResources: uint32_t;
    maxDescriptorSetUpdateAfterBindSamplers: uint32_t;
    maxDescriptorSetUpdateAfterBindUniformBuffers: uint32_t;
    maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: uint32_t;
    maxDescriptorSetUpdateAfterBindStorageBuffers: uint32_t;
    maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: uint32_t;
    maxDescriptorSetUpdateAfterBindSampledImages: uint32_t;
    maxDescriptorSetUpdateAfterBindStorageImages: uint32_t;
    maxDescriptorSetUpdateAfterBindInputAttachments: uint32_t;
    supportedDepthResolveModes: VkResolveModeFlags;
    supportedStencilResolveModes: VkResolveModeFlags;
    independentResolveNone: VkBool32;
    independentResolve: VkBool32;
    filterMinmaxSingleComponentFormats: VkBool32;
    filterMinmaxImageComponentMapping: VkBool32;
    maxTimelineSemaphoreValueDifference: uint64_t;
    framebufferIntegerColorSampleCounts: VkSampleCountFlags;
}
pub struct VkPipelineCompilerControlCreateInfoAMD
{
    sType: VkStructureType;
    pNext: void;
    compilerControlFlags: VkPipelineCompilerControlFlagsAMD;
}
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD
{
    sType: VkStructureType;
    pNext: void;
    deviceCoherentMemory: VkBool32;
}
pub struct VkPhysicalDeviceToolPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    name: char;
    version: char;
    purposes: VkToolPurposeFlagsEXT;
    description: char;
    layer: char;
}
pub struct VkSamplerCustomBorderColorCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    customBorderColor: VkClearColorValue;
    format: VkFormat;
}
pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    maxCustomBorderColorSamplers: uint32_t;
}
pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    customBorderColors: VkBool32;
    customBorderColorWithoutFormat: VkBool32;
}
pub struct VkAccelerationStructureGeometryTrianglesDataKHR
{
    sType: VkStructureType;
    pNext: void;
    vertexFormat: VkFormat;
    vertexData: VkDeviceOrHostAddressConstKHR;
    vertexStride: VkDeviceSize;
    maxVertex: uint32_t;
    indexType: VkIndexType;
    indexData: VkDeviceOrHostAddressConstKHR;
    transformData: VkDeviceOrHostAddressConstKHR;
}
pub struct VkAccelerationStructureGeometryAabbsDataKHR
{
    sType: VkStructureType;
    pNext: void;
    data: VkDeviceOrHostAddressConstKHR;
    stride: VkDeviceSize;
}
pub struct VkAccelerationStructureGeometryInstancesDataKHR
{
    sType: VkStructureType;
    pNext: void;
    arrayOfPointers: VkBool32;
    data: VkDeviceOrHostAddressConstKHR;
}
pub struct VkAccelerationStructureGeometryKHR
{
    sType: VkStructureType;
    pNext: void;
    geometryType: VkGeometryTypeKHR;
    geometry: VkAccelerationStructureGeometryDataKHR;
    flags: VkGeometryFlagsKHR;
}
pub struct VkAccelerationStructureBuildGeometryInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    type: VkAccelerationStructureTypeKHR;
    flags: VkBuildAccelerationStructureFlagsKHR;
    mode: VkBuildAccelerationStructureModeKHR;
    srcAccelerationStructure: VkAccelerationStructureKHR;
    dstAccelerationStructure: VkAccelerationStructureKHR;
    geometryCount: uint32_t;
    pGeometries: VkAccelerationStructureGeometryKHR;
    ppGeometries: VkAccelerationStructureGeometryKHR;
    scratchData: VkDeviceOrHostAddressKHR;
}
pub struct VkAccelerationStructureBuildRangeInfoKHR
{
    primitiveCount: uint32_t;
    primitiveOffset: uint32_t;
    firstVertex: uint32_t;
    transformOffset: uint32_t;
}
pub struct VkAccelerationStructureCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    createFlags: VkAccelerationStructureCreateFlagsKHR;
    buffer: VkBuffer;
    offset: VkDeviceSize;
    size: VkDeviceSize;
    type: VkAccelerationStructureTypeKHR;
    deviceAddress: VkDeviceAddress;
}
pub struct VkAabbPositionsKHR
{
    minX: float;
    minY: float;
    minZ: float;
    maxX: float;
    maxY: float;
    maxZ: float;
}
pub struct VkAabbPositionsNV
{
}
pub struct VkTransformMatrixKHR
{
    matrix: float;
}
pub struct VkTransformMatrixNV
{
}
pub struct VkAccelerationStructureInstanceKHR
{
    transform: VkTransformMatrixKHR;
    instanceCustomIndex: uint32_t;
    mask: uint32_t;
    instanceShaderBindingTableRecordOffset: uint32_t;
    flags: VkGeometryInstanceFlagsKHR;
    accelerationStructureReference: uint64_t;
}
pub struct VkAccelerationStructureInstanceNV
{
}
pub struct VkAccelerationStructureDeviceAddressInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    accelerationStructure: VkAccelerationStructureKHR;
}
pub struct VkAccelerationStructureVersionInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    pVersionData: uint8_t;
}
pub struct VkCopyAccelerationStructureInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    src: VkAccelerationStructureKHR;
    dst: VkAccelerationStructureKHR;
    mode: VkCopyAccelerationStructureModeKHR;
}
pub struct VkCopyAccelerationStructureToMemoryInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    src: VkAccelerationStructureKHR;
    dst: VkDeviceOrHostAddressKHR;
    mode: VkCopyAccelerationStructureModeKHR;
}
pub struct VkCopyMemoryToAccelerationStructureInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    src: VkDeviceOrHostAddressConstKHR;
    dst: VkAccelerationStructureKHR;
    mode: VkCopyAccelerationStructureModeKHR;
}
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    maxPipelineRayPayloadSize: uint32_t;
    maxPipelineRayHitAttributeSize: uint32_t;
}
pub struct VkPipelineLibraryCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    libraryCount: uint32_t;
    pLibraries: VkPipeline;
}
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    extendedDynamicState: VkBool32;
}
pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    extendedDynamicState2: VkBool32;
    extendedDynamicState2LogicOp: VkBool32;
    extendedDynamicState2PatchControlPoints: VkBool32;
}
pub struct VkRenderPassTransformBeginInfoQCOM
{
    sType: VkStructureType;
    pNext: void;
    transform: VkSurfaceTransformFlagBitsKHR;
}
pub struct VkCopyCommandTransformInfoQCOM
{
    sType: VkStructureType;
    pNext: void;
    transform: VkSurfaceTransformFlagBitsKHR;
}
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM
{
    sType: VkStructureType;
    pNext: void;
    transform: VkSurfaceTransformFlagBitsKHR;
    renderArea: VkRect2D;
}
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    diagnosticsConfig: VkBool32;
}
pub struct VkDeviceDiagnosticsConfigCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    flags: VkDeviceDiagnosticsConfigFlagsNV;
}
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    shaderZeroInitializeWorkgroupMemory: VkBool32;
}
pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    shaderSubgroupUniformControlFlow: VkBool32;
}
pub struct VkPhysicalDeviceRobustness2FeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    robustBufferAccess2: VkBool32;
    robustImageAccess2: VkBool32;
    nullDescriptor: VkBool32;
}
pub struct VkPhysicalDeviceRobustness2PropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    robustStorageBufferAccessSizeAlignment: VkDeviceSize;
    robustUniformBufferAccessSizeAlignment: VkDeviceSize;
}
pub struct VkPhysicalDeviceImageRobustnessFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    robustImageAccess: VkBool32;
}
pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    workgroupMemoryExplicitLayout: VkBool32;
    workgroupMemoryExplicitLayoutScalarBlockLayout: VkBool32;
    workgroupMemoryExplicitLayout8BitAccess: VkBool32;
    workgroupMemoryExplicitLayout16BitAccess: VkBool32;
}
pub struct VkPhysicalDevicePortabilitySubsetFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    constantAlphaColorBlendFactors: VkBool32;
    events: VkBool32;
    imageViewFormatReinterpretation: VkBool32;
    imageViewFormatSwizzle: VkBool32;
    imageView2DOn3DImage: VkBool32;
    multisampleArrayImage: VkBool32;
    mutableComparisonSamplers: VkBool32;
    pointPolygons: VkBool32;
    samplerMipLodBias: VkBool32;
    separateStencilMaskRef: VkBool32;
    shaderSampleRateInterpolationFunctions: VkBool32;
    tessellationIsolines: VkBool32;
    tessellationPointMode: VkBool32;
    triangleFans: VkBool32;
    vertexAttributeAccessBeyondStride: VkBool32;
}
pub struct VkPhysicalDevicePortabilitySubsetPropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    minVertexInputBindingStrideAlignment: uint32_t;
}
pub struct VkPhysicalDevice4444FormatsFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    formatA4R4G4B4: VkBool32;
    formatA4B4G4R4: VkBool32;
}
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI
{
    sType: VkStructureType;
    pNext: void;
    subpassShading: VkBool32;
}
pub struct VkBufferCopy2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcOffset: VkDeviceSize;
    dstOffset: VkDeviceSize;
    size: VkDeviceSize;
}
pub struct VkImageCopy2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcSubresource: VkImageSubresourceLayers;
    srcOffset: VkOffset3D;
    dstSubresource: VkImageSubresourceLayers;
    dstOffset: VkOffset3D;
    extent: VkExtent3D;
}
pub struct VkImageBlit2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcSubresource: VkImageSubresourceLayers;
    srcOffsets: VkOffset3D;
    dstSubresource: VkImageSubresourceLayers;
    dstOffsets: VkOffset3D;
}
pub struct VkBufferImageCopy2KHR
{
    sType: VkStructureType;
    pNext: void;
    bufferOffset: VkDeviceSize;
    bufferRowLength: uint32_t;
    bufferImageHeight: uint32_t;
    imageSubresource: VkImageSubresourceLayers;
    imageOffset: VkOffset3D;
    imageExtent: VkExtent3D;
}
pub struct VkImageResolve2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcSubresource: VkImageSubresourceLayers;
    srcOffset: VkOffset3D;
    dstSubresource: VkImageSubresourceLayers;
    dstOffset: VkOffset3D;
    extent: VkExtent3D;
}
pub struct VkCopyBufferInfo2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcBuffer: VkBuffer;
    dstBuffer: VkBuffer;
    regionCount: uint32_t;
    pRegions: VkBufferCopy2KHR;
}
pub struct VkCopyImageInfo2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcImage: VkImage;
    srcImageLayout: VkImageLayout;
    dstImage: VkImage;
    dstImageLayout: VkImageLayout;
    regionCount: uint32_t;
    pRegions: VkImageCopy2KHR;
}
pub struct VkBlitImageInfo2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcImage: VkImage;
    srcImageLayout: VkImageLayout;
    dstImage: VkImage;
    dstImageLayout: VkImageLayout;
    regionCount: uint32_t;
    pRegions: VkImageBlit2KHR;
    filter: VkFilter;
}
pub struct VkCopyBufferToImageInfo2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcBuffer: VkBuffer;
    dstImage: VkImage;
    dstImageLayout: VkImageLayout;
    regionCount: uint32_t;
    pRegions: VkBufferImageCopy2KHR;
}
pub struct VkCopyImageToBufferInfo2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcImage: VkImage;
    srcImageLayout: VkImageLayout;
    dstBuffer: VkBuffer;
    regionCount: uint32_t;
    pRegions: VkBufferImageCopy2KHR;
}
pub struct VkResolveImageInfo2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcImage: VkImage;
    srcImageLayout: VkImageLayout;
    dstImage: VkImage;
    dstImageLayout: VkImageLayout;
    regionCount: uint32_t;
    pRegions: VkImageResolve2KHR;
}
pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    shaderImageInt64Atomics: VkBool32;
    sparseImageInt64Atomics: VkBool32;
}
pub struct VkFragmentShadingRateAttachmentInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    pFragmentShadingRateAttachment: VkAttachmentReference2;
    shadingRateAttachmentTexelSize: VkExtent2D;
}
pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    fragmentSize: VkExtent2D;
    combinerOps: VkFragmentShadingRateCombinerOpKHR;
}
pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    pipelineFragmentShadingRate: VkBool32;
    primitiveFragmentShadingRate: VkBool32;
    attachmentFragmentShadingRate: VkBool32;
}
pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    minFragmentShadingRateAttachmentTexelSize: VkExtent2D;
    maxFragmentShadingRateAttachmentTexelSize: VkExtent2D;
    maxFragmentShadingRateAttachmentTexelSizeAspectRatio: uint32_t;
    primitiveFragmentShadingRateWithMultipleViewports: VkBool32;
    layeredShadingRateAttachments: VkBool32;
    fragmentShadingRateNonTrivialCombinerOps: VkBool32;
    maxFragmentSize: VkExtent2D;
    maxFragmentSizeAspectRatio: uint32_t;
    maxFragmentShadingRateCoverageSamples: uint32_t;
    maxFragmentShadingRateRasterizationSamples: VkSampleCountFlagBits;
    fragmentShadingRateWithShaderDepthStencilWrites: VkBool32;
    fragmentShadingRateWithSampleMask: VkBool32;
    fragmentShadingRateWithShaderSampleMask: VkBool32;
    fragmentShadingRateWithConservativeRasterization: VkBool32;
    fragmentShadingRateWithFragmentShaderInterlock: VkBool32;
    fragmentShadingRateWithCustomSampleLocations: VkBool32;
    fragmentShadingRateStrictMultiplyCombiner: VkBool32;
}
pub struct VkPhysicalDeviceFragmentShadingRateKHR
{
    sType: VkStructureType;
    pNext: void;
    sampleCounts: VkSampleCountFlags;
    fragmentSize: VkExtent2D;
}
pub struct VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    shaderTerminateInvocation: VkBool32;
}
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    fragmentShadingRateEnums: VkBool32;
    supersampleFragmentShadingRates: VkBool32;
    noInvocationFragmentShadingRates: VkBool32;
}
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV
{
    sType: VkStructureType;
    pNext: void;
    maxFragmentShadingRateInvocationCount: VkSampleCountFlagBits;
}
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV
{
    sType: VkStructureType;
    pNext: void;
    shadingRateType: VkFragmentShadingRateTypeNV;
    shadingRate: VkFragmentShadingRateNV;
    combinerOps: VkFragmentShadingRateCombinerOpKHR;
}
pub struct VkAccelerationStructureBuildSizesInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    accelerationStructureSize: VkDeviceSize;
    updateScratchSize: VkDeviceSize;
    buildScratchSize: VkDeviceSize;
}
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE
{
    sType: VkStructureType;
    pNext: void;
    mutableDescriptorType: VkBool32;
}
pub struct VkMutableDescriptorTypeListVALVE
{
    descriptorTypeCount: uint32_t;
    pDescriptorTypes: VkDescriptorType;
}
pub struct VkMutableDescriptorTypeCreateInfoVALVE
{
    sType: VkStructureType;
    pNext: void;
    mutableDescriptorTypeListCount: uint32_t;
    pMutableDescriptorTypeLists: VkMutableDescriptorTypeListVALVE;
}
pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    vertexInputDynamicState: VkBool32;
}
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    externalMemoryRDMA: VkBool32;
}
pub struct VkVertexInputBindingDescription2EXT
{
    sType: VkStructureType;
    pNext: void;
    binding: uint32_t;
    stride: uint32_t;
    inputRate: VkVertexInputRate;
    divisor: uint32_t;
}
pub struct VkVertexInputAttributeDescription2EXT
{
    sType: VkStructureType;
    pNext: void;
    location: uint32_t;
    binding: uint32_t;
    format: VkFormat;
    offset: uint32_t;
}
pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    colorWriteEnable: VkBool32;
}
pub struct VkPipelineColorWriteCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    attachmentCount: uint32_t;
    pColorWriteEnables: VkBool32;
}
pub struct VkMemoryBarrier2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcStageMask: VkPipelineStageFlags2KHR;
    srcAccessMask: VkAccessFlags2KHR;
    dstStageMask: VkPipelineStageFlags2KHR;
    dstAccessMask: VkAccessFlags2KHR;
}
pub struct VkImageMemoryBarrier2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcStageMask: VkPipelineStageFlags2KHR;
    srcAccessMask: VkAccessFlags2KHR;
    dstStageMask: VkPipelineStageFlags2KHR;
    dstAccessMask: VkAccessFlags2KHR;
    oldLayout: VkImageLayout;
    newLayout: VkImageLayout;
    srcQueueFamilyIndex: uint32_t;
    dstQueueFamilyIndex: uint32_t;
    image: VkImage;
    subresourceRange: VkImageSubresourceRange;
}
pub struct VkBufferMemoryBarrier2KHR
{
    sType: VkStructureType;
    pNext: void;
    srcStageMask: VkPipelineStageFlags2KHR;
    srcAccessMask: VkAccessFlags2KHR;
    dstStageMask: VkPipelineStageFlags2KHR;
    dstAccessMask: VkAccessFlags2KHR;
    srcQueueFamilyIndex: uint32_t;
    dstQueueFamilyIndex: uint32_t;
    buffer: VkBuffer;
    offset: VkDeviceSize;
    size: VkDeviceSize;
}
pub struct VkDependencyInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    dependencyFlags: VkDependencyFlags;
    memoryBarrierCount: uint32_t;
    pMemoryBarriers: VkMemoryBarrier2KHR;
    bufferMemoryBarrierCount: uint32_t;
    pBufferMemoryBarriers: VkBufferMemoryBarrier2KHR;
    imageMemoryBarrierCount: uint32_t;
    pImageMemoryBarriers: VkImageMemoryBarrier2KHR;
}
pub struct VkSemaphoreSubmitInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    semaphore: VkSemaphore;
    value: uint64_t;
    stageMask: VkPipelineStageFlags2KHR;
    deviceIndex: uint32_t;
}
pub struct VkCommandBufferSubmitInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    commandBuffer: VkCommandBuffer;
    deviceMask: uint32_t;
}
pub struct VkSubmitInfo2KHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkSubmitFlagsKHR;
    waitSemaphoreInfoCount: uint32_t;
    pWaitSemaphoreInfos: VkSemaphoreSubmitInfoKHR;
    commandBufferInfoCount: uint32_t;
    pCommandBufferInfos: VkCommandBufferSubmitInfoKHR;
    signalSemaphoreInfoCount: uint32_t;
    pSignalSemaphoreInfos: VkSemaphoreSubmitInfoKHR;
}
pub struct VkQueueFamilyCheckpointProperties2NV
{
    sType: VkStructureType;
    pNext: void;
    checkpointExecutionStageMask: VkPipelineStageFlags2KHR;
}
pub struct VkCheckpointData2NV
{
    sType: VkStructureType;
    pNext: void;
    stage: VkPipelineStageFlags2KHR;
    pCheckpointMarker: void;
}
pub struct VkPhysicalDeviceSynchronization2FeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    synchronization2: VkBool32;
}
pub struct VkVideoQueueFamilyProperties2KHR
{
    sType: VkStructureType;
    pNext: void;
    videoCodecOperations: VkVideoCodecOperationFlagsKHR;
}
pub struct VkVideoProfilesKHR
{
    sType: VkStructureType;
    pNext: void;
    profileCount: uint32_t;
    pProfiles: VkVideoProfileKHR;
}
pub struct VkPhysicalDeviceVideoFormatInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    imageUsage: VkImageUsageFlags;
    pVideoProfiles: VkVideoProfilesKHR;
}
pub struct VkVideoFormatPropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    format: VkFormat;
}
pub struct VkVideoProfileKHR
{
    sType: VkStructureType;
    pNext: void;
    videoCodecOperation: VkVideoCodecOperationFlagBitsKHR;
    chromaSubsampling: VkVideoChromaSubsamplingFlagsKHR;
    lumaBitDepth: VkVideoComponentBitDepthFlagsKHR;
    chromaBitDepth: VkVideoComponentBitDepthFlagsKHR;
}
pub struct VkVideoCapabilitiesKHR
{
    sType: VkStructureType;
    pNext: void;
    capabilityFlags: VkVideoCapabilityFlagsKHR;
    minBitstreamBufferOffsetAlignment: VkDeviceSize;
    minBitstreamBufferSizeAlignment: VkDeviceSize;
    videoPictureExtentGranularity: VkExtent2D;
    minExtent: VkExtent2D;
    maxExtent: VkExtent2D;
    maxReferencePicturesSlotsCount: uint32_t;
    maxReferencePicturesActiveCount: uint32_t;
}
pub struct VkVideoGetMemoryPropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    memoryBindIndex: uint32_t;
    pMemoryRequirements: VkMemoryRequirements2;
}
pub struct VkVideoBindMemoryKHR
{
    sType: VkStructureType;
    pNext: void;
    memoryBindIndex: uint32_t;
    memory: VkDeviceMemory;
    memoryOffset: VkDeviceSize;
    memorySize: VkDeviceSize;
}
pub struct VkVideoPictureResourceKHR
{
    sType: VkStructureType;
    pNext: void;
    codedOffset: VkOffset2D;
    codedExtent: VkExtent2D;
    baseArrayLayer: uint32_t;
    imageViewBinding: VkImageView;
}
pub struct VkVideoReferenceSlotKHR
{
    sType: VkStructureType;
    pNext: void;
    slotIndex: int8_t;
    pPictureResource: VkVideoPictureResourceKHR;
}
pub struct VkVideoDecodeInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoDecodeFlagsKHR;
    codedOffset: VkOffset2D;
    codedExtent: VkExtent2D;
    srcBuffer: VkBuffer;
    srcBufferOffset: VkDeviceSize;
    srcBufferRange: VkDeviceSize;
    dstPictureResource: VkVideoPictureResourceKHR;
    pSetupReferenceSlot: VkVideoReferenceSlotKHR;
    referenceSlotCount: uint32_t;
    pReferenceSlots: VkVideoReferenceSlotKHR;
}
pub struct VkVideoDecodeH264ProfileEXT
{
    sType: VkStructureType;
    pNext: void;
    stdProfileIdc: StdVideoH264ProfileIdc;
    pictureLayout: VkVideoDecodeH264PictureLayoutFlagsEXT;
}
pub struct VkVideoDecodeH264CapabilitiesEXT
{
    sType: VkStructureType;
    pNext: void;
    maxLevel: uint32_t;
    fieldOffsetGranularity: VkOffset2D;
    stdExtensionVersion: VkExtensionProperties;
}
pub struct VkVideoDecodeH264SessionCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoDecodeH264CreateFlagsEXT;
    pStdExtensionVersion: VkExtensionProperties;
}
pub struct VkVideoDecodeH264SessionParametersAddInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    spsStdCount: uint32_t;
    pSpsStd: StdVideoH264SequenceParameterSet;
    ppsStdCount: uint32_t;
    pPpsStd: StdVideoH264PictureParameterSet;
}
pub struct VkVideoDecodeH264SessionParametersCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    maxSpsStdCount: uint32_t;
    maxPpsStdCount: uint32_t;
    pParametersAddInfo: VkVideoDecodeH264SessionParametersAddInfoEXT;
}
pub struct VkVideoDecodeH264PictureInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    pStdPictureInfo: StdVideoDecodeH264PictureInfo;
    slicesCount: uint32_t;
    pSlicesDataOffsets: uint32_t;
}
pub struct VkVideoDecodeH264DpbSlotInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    pStdReferenceInfo: StdVideoDecodeH264ReferenceInfo;
}
pub struct VkVideoDecodeH264MvcEXT
{
    sType: VkStructureType;
    pNext: void;
    pStdMvc: StdVideoDecodeH264Mvc;
}
pub struct VkVideoDecodeH265ProfileEXT
{
    sType: VkStructureType;
    pNext: void;
    stdProfileIdc: StdVideoH265ProfileIdc;
}
pub struct VkVideoDecodeH265CapabilitiesEXT
{
    sType: VkStructureType;
    pNext: void;
    maxLevel: uint32_t;
    stdExtensionVersion: VkExtensionProperties;
}
pub struct VkVideoDecodeH265SessionCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoDecodeH265CreateFlagsEXT;
    pStdExtensionVersion: VkExtensionProperties;
}
pub struct VkVideoDecodeH265SessionParametersAddInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    spsStdCount: uint32_t;
    pSpsStd: StdVideoH265SequenceParameterSet;
    ppsStdCount: uint32_t;
    pPpsStd: StdVideoH265PictureParameterSet;
}
pub struct VkVideoDecodeH265SessionParametersCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    maxSpsStdCount: uint32_t;
    maxPpsStdCount: uint32_t;
    pParametersAddInfo: VkVideoDecodeH265SessionParametersAddInfoEXT;
}
pub struct VkVideoDecodeH265PictureInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    pStdPictureInfo: StdVideoDecodeH265PictureInfo;
    slicesCount: uint32_t;
    pSlicesDataOffsets: uint32_t;
}
pub struct VkVideoDecodeH265DpbSlotInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    pStdReferenceInfo: StdVideoDecodeH265ReferenceInfo;
}
pub struct VkVideoSessionCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    queueFamilyIndex: uint32_t;
    flags: VkVideoSessionCreateFlagsKHR;
    pVideoProfile: VkVideoProfileKHR;
    pictureFormat: VkFormat;
    maxCodedExtent: VkExtent2D;
    referencePicturesFormat: VkFormat;
    maxReferencePicturesSlotsCount: uint32_t;
    maxReferencePicturesActiveCount: uint32_t;
}
pub struct VkVideoSessionParametersCreateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    videoSessionParametersTemplate: VkVideoSessionParametersKHR;
    videoSession: VkVideoSessionKHR;
}
pub struct VkVideoSessionParametersUpdateInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    updateSequenceCount: uint32_t;
}
pub struct VkVideoBeginCodingInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoBeginCodingFlagsKHR;
    codecQualityPreset: VkVideoCodingQualityPresetFlagsKHR;
    videoSession: VkVideoSessionKHR;
    videoSessionParameters: VkVideoSessionParametersKHR;
    referenceSlotCount: uint32_t;
    pReferenceSlots: VkVideoReferenceSlotKHR;
}
pub struct VkVideoEndCodingInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoEndCodingFlagsKHR;
}
pub struct VkVideoCodingControlInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoCodingControlFlagsKHR;
}
pub struct VkVideoEncodeInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoEncodeFlagsKHR;
    qualityLevel: uint32_t;
    codedExtent: VkExtent2D;
    dstBitstreamBuffer: VkBuffer;
    dstBitstreamBufferOffset: VkDeviceSize;
    dstBitstreamBufferMaxRange: VkDeviceSize;
    srcPictureResource: VkVideoPictureResourceKHR;
    pSetupReferenceSlot: VkVideoReferenceSlotKHR;
    referenceSlotCount: uint32_t;
    pReferenceSlots: VkVideoReferenceSlotKHR;
}
pub struct VkVideoEncodeRateControlInfoKHR
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoEncodeRateControlFlagsKHR;
    rateControlMode: VkVideoEncodeRateControlModeFlagBitsKHR;
    averageBitrate: uint32_t;
    peakToAverageBitrateRatio: uint16_t;
    frameRateNumerator: uint16_t;
    frameRateDenominator: uint16_t;
    virtualBufferSizeInMs: uint32_t;
}
pub struct VkVideoEncodeH264CapabilitiesEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoEncodeH264CapabilityFlagsEXT;
    inputModeFlags: VkVideoEncodeH264InputModeFlagsEXT;
    outputModeFlags: VkVideoEncodeH264OutputModeFlagsEXT;
    minPictureSizeInMbs: VkExtent2D;
    maxPictureSizeInMbs: VkExtent2D;
    inputImageDataAlignment: VkExtent2D;
    maxNumL0ReferenceForP: uint8_t;
    maxNumL0ReferenceForB: uint8_t;
    maxNumL1Reference: uint8_t;
    qualityLevelCount: uint8_t;
    stdExtensionVersion: VkExtensionProperties;
}
pub struct VkVideoEncodeH264SessionCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    flags: VkVideoEncodeH264CreateFlagsEXT;
    maxPictureSizeInMbs: VkExtent2D;
    pStdExtensionVersion: VkExtensionProperties;
}
pub struct VkVideoEncodeH264SessionParametersAddInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    spsStdCount: uint32_t;
    pSpsStd: StdVideoH264SequenceParameterSet;
    ppsStdCount: uint32_t;
    pPpsStd: StdVideoH264PictureParameterSet;
}
pub struct VkVideoEncodeH264SessionParametersCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    maxSpsStdCount: uint32_t;
    maxPpsStdCount: uint32_t;
    pParametersAddInfo: VkVideoEncodeH264SessionParametersAddInfoEXT;
}
pub struct VkVideoEncodeH264DpbSlotInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    slotIndex: int8_t;
    pStdPictureInfo: StdVideoEncodeH264PictureInfo;
}
pub struct VkVideoEncodeH264VclFrameInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    refDefaultFinalList0EntryCount: uint8_t;
    pRefDefaultFinalList0Entries: VkVideoEncodeH264DpbSlotInfoEXT;
    refDefaultFinalList1EntryCount: uint8_t;
    pRefDefaultFinalList1Entries: VkVideoEncodeH264DpbSlotInfoEXT;
    naluSliceEntryCount: uint32_t;
    pNaluSliceEntries: VkVideoEncodeH264NaluSliceEXT;
    pCurrentPictureInfo: VkVideoEncodeH264DpbSlotInfoEXT;
}
pub struct VkVideoEncodeH264EmitPictureParametersEXT
{
    sType: VkStructureType;
    pNext: void;
    spsId: uint8_t;
    emitSpsEnable: VkBool32;
    ppsIdEntryCount: uint32_t;
    ppsIdEntries: uint8_t;
}
pub struct VkVideoEncodeH264ProfileEXT
{
    sType: VkStructureType;
    pNext: void;
    stdProfileIdc: StdVideoH264ProfileIdc;
}
pub struct VkVideoEncodeH264NaluSliceEXT
{
    sType: VkStructureType;
    pNext: void;
    pSliceHeaderStd: StdVideoEncodeH264SliceHeader;
    mbCount: uint32_t;
    refFinalList0EntryCount: uint8_t;
    pRefFinalList0Entries: VkVideoEncodeH264DpbSlotInfoEXT;
    refFinalList1EntryCount: uint8_t;
    pRefFinalList1Entries: VkVideoEncodeH264DpbSlotInfoEXT;
    precedingNaluBytes: uint32_t;
    minQp: uint8_t;
    maxQp: uint8_t;
}
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    inheritedViewportScissor2D: VkBool32;
}
pub struct VkCommandBufferInheritanceViewportScissorInfoNV
{
    sType: VkStructureType;
    pNext: void;
    viewportScissor2D: VkBool32;
    viewportDepthCount: uint32_t;
    pViewportDepths: VkViewport;
}
pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    ycbcr2plane444Formats: VkBool32;
}
pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT
{
    sType: VkStructureType;
    pNext: void;
    provokingVertexLast: VkBool32;
    transformFeedbackPreservesProvokingVertex: VkBool32;
}
pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    provokingVertexModePerPipeline: VkBool32;
    transformFeedbackPreservesTriangleFanProvokingVertex: VkBool32;
}
pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT
{
    sType: VkStructureType;
    pNext: void;
    provokingVertexMode: VkProvokingVertexModeEXT;
}
pub struct VkCuModuleCreateInfoNVX
{
    sType: VkStructureType;
    pNext: void;
    dataSize: size_t;
    pData: void;
}
pub struct VkCuFunctionCreateInfoNVX
{
    sType: VkStructureType;
    pNext: void;
    module: VkCuModuleNVX;
    pName: char;
}
pub struct VkCuLaunchInfoNVX
{
    sType: VkStructureType;
    pNext: void;
    function: VkCuFunctionNVX;
    gridDimX: uint32_t;
    gridDimY: uint32_t;
    gridDimZ: uint32_t;
    blockDimX: uint32_t;
    blockDimY: uint32_t;
    blockDimZ: uint32_t;
    sharedMemBytes: uint32_t;
    paramCount: size_t;
    pParams: void;
    extraCount: size_t;
    pExtras: void;
}
pub struct VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR
{
    sType: VkStructureType;
    pNext: void;
    shaderIntegerDotProduct: VkBool32;
}
pub struct VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR
{
    sType: VkStructureType;
    pNext: void;
    integerDotProduct8BitUnsignedAccelerated: VkBool32;
    integerDotProduct8BitSignedAccelerated: VkBool32;
    integerDotProduct8BitMixedSignednessAccelerated: VkBool32;
    integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32;
    integerDotProduct4x8BitPackedSignedAccelerated: VkBool32;
    integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32;
    integerDotProduct16BitUnsignedAccelerated: VkBool32;
    integerDotProduct16BitSignedAccelerated: VkBool32;
    integerDotProduct16BitMixedSignednessAccelerated: VkBool32;
    integerDotProduct32BitUnsignedAccelerated: VkBool32;
    integerDotProduct32BitSignedAccelerated: VkBool32;
    integerDotProduct32BitMixedSignednessAccelerated: VkBool32;
    integerDotProduct64BitUnsignedAccelerated: VkBool32;
    integerDotProduct64BitSignedAccelerated: VkBool32;
    integerDotProduct64BitMixedSignednessAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32;
    integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32;
}
pub struct VkPhysicalDeviceDrmPropertiesEXT
{
    sType: VkStructureType;
    pNext: void;
    hasPrimary: VkBool32;
    hasRender: VkBool32;
    primaryMajor: int64_t;
    primaryMinor: int64_t;
    renderMajor: int64_t;
    renderMinor: int64_t;
}
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV
{
    sType: VkStructureType;
    pNext: void;
    rayTracingMotionBlur: VkBool32;
    rayTracingMotionBlurPipelineTraceRaysIndirect: VkBool32;
}
pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV
{
    sType: VkStructureType;
    pNext: void;
    vertexData: VkDeviceOrHostAddressConstKHR;
}
pub struct VkAccelerationStructureMotionInfoNV
{
    sType: VkStructureType;
    pNext: void;
    maxInstances: uint32_t;
    flags: VkAccelerationStructureMotionInfoFlagsNV;
}
pub struct VkSRTDataNV
{
    sx: float;
    a: float;
    b: float;
    pvx: float;
    sy: float;
    c: float;
    pvy: float;
    sz: float;
    pvz: float;
    qx: float;
    qy: float;
    qz: float;
    qw: float;
    tx: float;
    ty: float;
    tz: float;
}
pub struct VkAccelerationStructureSRTMotionInstanceNV
{
    transformT0: VkSRTDataNV;
    transformT1: VkSRTDataNV;
    instanceCustomIndex: uint32_t;
    mask: uint32_t;
    instanceShaderBindingTableRecordOffset: uint32_t;
    flags: VkGeometryInstanceFlagsKHR;
    accelerationStructureReference: uint64_t;
}
pub struct VkAccelerationStructureMatrixMotionInstanceNV
{
    transformT0: VkTransformMatrixKHR;
    transformT1: VkTransformMatrixKHR;
    instanceCustomIndex: uint32_t;
    mask: uint32_t;
    instanceShaderBindingTableRecordOffset: uint32_t;
    flags: VkGeometryInstanceFlagsKHR;
    accelerationStructureReference: uint64_t;
}
pub struct VkAccelerationStructureMotionInstanceNV
{
    type: VkAccelerationStructureMotionInstanceTypeNV;
    flags: VkAccelerationStructureMotionInstanceFlagsNV;
    data: VkAccelerationStructureMotionInstanceDataNV;
}
pub struct VkMemoryGetRemoteAddressInfoNV
{
    sType: VkStructureType;
    pNext: void;
    memory: VkDeviceMemory;
    handleType: VkExternalMemoryHandleTypeFlagBits;
}
