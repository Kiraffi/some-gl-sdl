pub enum VkImageLayout
{
    VK_IMAGE_LAYOUT_UNDEFINED: i32 = 0,
    VK_IMAGE_LAYOUT_GENERAL: i32 = 1,
    VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: i32 = 2,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: i32 = 3,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: i32 = 4,
    VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: i32 = 5,
    VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: i32 = 6,
    VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: i32 = 7,
    VK_IMAGE_LAYOUT_PREINITIALIZED: i32 = 8,
}

pub enum VkAttachmentLoadOp
{
    VK_ATTACHMENT_LOAD_OP_LOAD: i32 = 0,
    VK_ATTACHMENT_LOAD_OP_CLEAR: i32 = 1,
    VK_ATTACHMENT_LOAD_OP_DONT_CARE: i32 = 2,
}

pub enum VkAttachmentStoreOp
{
    VK_ATTACHMENT_STORE_OP_STORE: i32 = 0,
    VK_ATTACHMENT_STORE_OP_DONT_CARE: i32 = 1,
}

pub enum VkImageType
{
    VK_IMAGE_TYPE_1D: i32 = 0,
    VK_IMAGE_TYPE_2D: i32 = 1,
    VK_IMAGE_TYPE_3D: i32 = 2,
}

pub enum VkImageTiling
{
    VK_IMAGE_TILING_OPTIMAL: i32 = 0,
    VK_IMAGE_TILING_LINEAR: i32 = 1,
}

pub enum VkImageViewType
{
    VK_IMAGE_VIEW_TYPE_1D: i32 = 0,
    VK_IMAGE_VIEW_TYPE_2D: i32 = 1,
    VK_IMAGE_VIEW_TYPE_3D: i32 = 2,
    VK_IMAGE_VIEW_TYPE_CUBE: i32 = 3,
    VK_IMAGE_VIEW_TYPE_1D_ARRAY: i32 = 4,
    VK_IMAGE_VIEW_TYPE_2D_ARRAY: i32 = 5,
    VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: i32 = 6,
}

pub enum VkCommandBufferLevel
{
    VK_COMMAND_BUFFER_LEVEL_PRIMARY: i32 = 0,
    VK_COMMAND_BUFFER_LEVEL_SECONDARY: i32 = 1,
}

pub enum VkComponentSwizzle
{
    VK_COMPONENT_SWIZZLE_IDENTITY: i32 = 0,
    VK_COMPONENT_SWIZZLE_ZERO: i32 = 1,
    VK_COMPONENT_SWIZZLE_ONE: i32 = 2,
    VK_COMPONENT_SWIZZLE_R: i32 = 3,
    VK_COMPONENT_SWIZZLE_G: i32 = 4,
    VK_COMPONENT_SWIZZLE_B: i32 = 5,
    VK_COMPONENT_SWIZZLE_A: i32 = 6,
}

pub enum VkDescriptorType
{
    VK_DESCRIPTOR_TYPE_SAMPLER: i32 = 0,
    VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: i32 = 1,
    VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: i32 = 2,
    VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: i32 = 3,
    VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: i32 = 4,
    VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: i32 = 5,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: i32 = 6,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: i32 = 7,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: i32 = 8,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: i32 = 9,
    VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: i32 = 10,
}

pub enum VkQueryType
{
    VK_QUERY_TYPE_OCCLUSION: i32 = 0,
    VK_QUERY_TYPE_PIPELINE_STATISTICS: i32 = 1,
    VK_QUERY_TYPE_TIMESTAMP: i32 = 2,
}

pub enum VkBorderColor
{
    VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: i32 = 0,
    VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: i32 = 1,
    VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: i32 = 2,
    VK_BORDER_COLOR_INT_OPAQUE_BLACK: i32 = 3,
    VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: i32 = 4,
    VK_BORDER_COLOR_INT_OPAQUE_WHITE: i32 = 5,
}

pub enum VkPipelineBindPoint
{
    VK_PIPELINE_BIND_POINT_GRAPHICS: i32 = 0,
    VK_PIPELINE_BIND_POINT_COMPUTE: i32 = 1,
}

pub enum VkPipelineCacheHeaderVersion
{
    VK_PIPELINE_CACHE_HEADER_VERSION_ONE: i32 = 1,
}

pub enum VkPrimitiveTopology
{
    VK_PRIMITIVE_TOPOLOGY_POINT_LIST: i32 = 0,
    VK_PRIMITIVE_TOPOLOGY_LINE_LIST: i32 = 1,
    VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: i32 = 2,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: i32 = 3,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: i32 = 4,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: i32 = 5,
    VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: i32 = 6,
    VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: i32 = 7,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: i32 = 8,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: i32 = 9,
    VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: i32 = 10,
}

pub enum VkSharingMode
{
    VK_SHARING_MODE_EXCLUSIVE: i32 = 0,
    VK_SHARING_MODE_CONCURRENT: i32 = 1,
}

pub enum VkIndexType
{
    VK_INDEX_TYPE_UINT16: i32 = 0,
    VK_INDEX_TYPE_UINT32: i32 = 1,
}

pub enum VkFilter
{
    VK_FILTER_NEAREST: i32 = 0,
    VK_FILTER_LINEAR: i32 = 1,
}

pub enum VkSamplerMipmapMode
{
    VK_SAMPLER_MIPMAP_MODE_NEAREST: i32 = 0,
    VK_SAMPLER_MIPMAP_MODE_LINEAR: i32 = 1,
}

pub enum VkSamplerAddressMode
{
    VK_SAMPLER_ADDRESS_MODE_REPEAT: i32 = 0,
    VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: i32 = 1,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: i32 = 2,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: i32 = 3,
}

pub enum VkCompareOp
{
    VK_COMPARE_OP_NEVER: i32 = 0,
    VK_COMPARE_OP_LESS: i32 = 1,
    VK_COMPARE_OP_EQUAL: i32 = 2,
    VK_COMPARE_OP_LESS_OR_EQUAL: i32 = 3,
    VK_COMPARE_OP_GREATER: i32 = 4,
    VK_COMPARE_OP_NOT_EQUAL: i32 = 5,
    VK_COMPARE_OP_GREATER_OR_EQUAL: i32 = 6,
    VK_COMPARE_OP_ALWAYS: i32 = 7,
}

pub enum VkPolygonMode
{
    VK_POLYGON_MODE_FILL: i32 = 0,
    VK_POLYGON_MODE_LINE: i32 = 1,
    VK_POLYGON_MODE_POINT: i32 = 2,
}

pub enum VkFrontFace
{
    VK_FRONT_FACE_COUNTER_CLOCKWISE: i32 = 0,
    VK_FRONT_FACE_CLOCKWISE: i32 = 1,
}

pub enum VkBlendFactor
{
    VK_BLEND_FACTOR_ZERO: i32 = 0,
    VK_BLEND_FACTOR_ONE: i32 = 1,
    VK_BLEND_FACTOR_SRC_COLOR: i32 = 2,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: i32 = 3,
    VK_BLEND_FACTOR_DST_COLOR: i32 = 4,
    VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: i32 = 5,
    VK_BLEND_FACTOR_SRC_ALPHA: i32 = 6,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: i32 = 7,
    VK_BLEND_FACTOR_DST_ALPHA: i32 = 8,
    VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: i32 = 9,
    VK_BLEND_FACTOR_CONSTANT_COLOR: i32 = 10,
    VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: i32 = 11,
    VK_BLEND_FACTOR_CONSTANT_ALPHA: i32 = 12,
    VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: i32 = 13,
    VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: i32 = 14,
    VK_BLEND_FACTOR_SRC1_COLOR: i32 = 15,
    VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: i32 = 16,
    VK_BLEND_FACTOR_SRC1_ALPHA: i32 = 17,
    VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: i32 = 18,
}

pub enum VkBlendOp
{
    VK_BLEND_OP_ADD: i32 = 0,
    VK_BLEND_OP_SUBTRACT: i32 = 1,
    VK_BLEND_OP_REVERSE_SUBTRACT: i32 = 2,
    VK_BLEND_OP_MIN: i32 = 3,
    VK_BLEND_OP_MAX: i32 = 4,
}

pub enum VkStencilOp
{
    VK_STENCIL_OP_KEEP: i32 = 0,
    VK_STENCIL_OP_ZERO: i32 = 1,
    VK_STENCIL_OP_REPLACE: i32 = 2,
    VK_STENCIL_OP_INCREMENT_AND_CLAMP: i32 = 3,
    VK_STENCIL_OP_DECREMENT_AND_CLAMP: i32 = 4,
    VK_STENCIL_OP_INVERT: i32 = 5,
    VK_STENCIL_OP_INCREMENT_AND_WRAP: i32 = 6,
    VK_STENCIL_OP_DECREMENT_AND_WRAP: i32 = 7,
}

pub enum VkLogicOp
{
    VK_LOGIC_OP_CLEAR: i32 = 0,
    VK_LOGIC_OP_AND: i32 = 1,
    VK_LOGIC_OP_AND_REVERSE: i32 = 2,
    VK_LOGIC_OP_COPY: i32 = 3,
    VK_LOGIC_OP_AND_INVERTED: i32 = 4,
    VK_LOGIC_OP_NO_OP: i32 = 5,
    VK_LOGIC_OP_XOR: i32 = 6,
    VK_LOGIC_OP_OR: i32 = 7,
    VK_LOGIC_OP_NOR: i32 = 8,
    VK_LOGIC_OP_EQUIVALENT: i32 = 9,
    VK_LOGIC_OP_INVERT: i32 = 10,
    VK_LOGIC_OP_OR_REVERSE: i32 = 11,
    VK_LOGIC_OP_COPY_INVERTED: i32 = 12,
    VK_LOGIC_OP_OR_INVERTED: i32 = 13,
    VK_LOGIC_OP_NAND: i32 = 14,
    VK_LOGIC_OP_SET: i32 = 15,
}

pub enum VkInternalAllocationType
{
    VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: i32 = 0,
}

pub enum VkSystemAllocationScope
{
    VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: i32 = 0,
    VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: i32 = 1,
    VK_SYSTEM_ALLOCATION_SCOPE_CACHE: i32 = 2,
    VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: i32 = 3,
    VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: i32 = 4,
}

pub enum VkPhysicalDeviceType
{
    VK_PHYSICAL_DEVICE_TYPE_OTHER: i32 = 0,
    VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: i32 = 1,
    VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: i32 = 2,
    VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: i32 = 3,
    VK_PHYSICAL_DEVICE_TYPE_CPU: i32 = 4,
}

pub enum VkVertexInputRate
{
    VK_VERTEX_INPUT_RATE_VERTEX: i32 = 0,
    VK_VERTEX_INPUT_RATE_INSTANCE: i32 = 1,
}

pub enum VkFormat
{
    VK_FORMAT_UNDEFINED: i32 = 0,
    VK_FORMAT_R4G4_UNORM_PACK8: i32 = 1,
    VK_FORMAT_R4G4B4A4_UNORM_PACK16: i32 = 2,
    VK_FORMAT_B4G4R4A4_UNORM_PACK16: i32 = 3,
    VK_FORMAT_R5G6B5_UNORM_PACK16: i32 = 4,
    VK_FORMAT_B5G6R5_UNORM_PACK16: i32 = 5,
    VK_FORMAT_R5G5B5A1_UNORM_PACK16: i32 = 6,
    VK_FORMAT_B5G5R5A1_UNORM_PACK16: i32 = 7,
    VK_FORMAT_A1R5G5B5_UNORM_PACK16: i32 = 8,
    VK_FORMAT_R8_UNORM: i32 = 9,
    VK_FORMAT_R8_SNORM: i32 = 10,
    VK_FORMAT_R8_USCALED: i32 = 11,
    VK_FORMAT_R8_SSCALED: i32 = 12,
    VK_FORMAT_R8_UINT: i32 = 13,
    VK_FORMAT_R8_SINT: i32 = 14,
    VK_FORMAT_R8_SRGB: i32 = 15,
    VK_FORMAT_R8G8_UNORM: i32 = 16,
    VK_FORMAT_R8G8_SNORM: i32 = 17,
    VK_FORMAT_R8G8_USCALED: i32 = 18,
    VK_FORMAT_R8G8_SSCALED: i32 = 19,
    VK_FORMAT_R8G8_UINT: i32 = 20,
    VK_FORMAT_R8G8_SINT: i32 = 21,
    VK_FORMAT_R8G8_SRGB: i32 = 22,
    VK_FORMAT_R8G8B8_UNORM: i32 = 23,
    VK_FORMAT_R8G8B8_SNORM: i32 = 24,
    VK_FORMAT_R8G8B8_USCALED: i32 = 25,
    VK_FORMAT_R8G8B8_SSCALED: i32 = 26,
    VK_FORMAT_R8G8B8_UINT: i32 = 27,
    VK_FORMAT_R8G8B8_SINT: i32 = 28,
    VK_FORMAT_R8G8B8_SRGB: i32 = 29,
    VK_FORMAT_B8G8R8_UNORM: i32 = 30,
    VK_FORMAT_B8G8R8_SNORM: i32 = 31,
    VK_FORMAT_B8G8R8_USCALED: i32 = 32,
    VK_FORMAT_B8G8R8_SSCALED: i32 = 33,
    VK_FORMAT_B8G8R8_UINT: i32 = 34,
    VK_FORMAT_B8G8R8_SINT: i32 = 35,
    VK_FORMAT_B8G8R8_SRGB: i32 = 36,
    VK_FORMAT_R8G8B8A8_UNORM: i32 = 37,
    VK_FORMAT_R8G8B8A8_SNORM: i32 = 38,
    VK_FORMAT_R8G8B8A8_USCALED: i32 = 39,
    VK_FORMAT_R8G8B8A8_SSCALED: i32 = 40,
    VK_FORMAT_R8G8B8A8_UINT: i32 = 41,
    VK_FORMAT_R8G8B8A8_SINT: i32 = 42,
    VK_FORMAT_R8G8B8A8_SRGB: i32 = 43,
    VK_FORMAT_B8G8R8A8_UNORM: i32 = 44,
    VK_FORMAT_B8G8R8A8_SNORM: i32 = 45,
    VK_FORMAT_B8G8R8A8_USCALED: i32 = 46,
    VK_FORMAT_B8G8R8A8_SSCALED: i32 = 47,
    VK_FORMAT_B8G8R8A8_UINT: i32 = 48,
    VK_FORMAT_B8G8R8A8_SINT: i32 = 49,
    VK_FORMAT_B8G8R8A8_SRGB: i32 = 50,
    VK_FORMAT_A8B8G8R8_UNORM_PACK32: i32 = 51,
    VK_FORMAT_A8B8G8R8_SNORM_PACK32: i32 = 52,
    VK_FORMAT_A8B8G8R8_USCALED_PACK32: i32 = 53,
    VK_FORMAT_A8B8G8R8_SSCALED_PACK32: i32 = 54,
    VK_FORMAT_A8B8G8R8_UINT_PACK32: i32 = 55,
    VK_FORMAT_A8B8G8R8_SINT_PACK32: i32 = 56,
    VK_FORMAT_A8B8G8R8_SRGB_PACK32: i32 = 57,
    VK_FORMAT_A2R10G10B10_UNORM_PACK32: i32 = 58,
    VK_FORMAT_A2R10G10B10_SNORM_PACK32: i32 = 59,
    VK_FORMAT_A2R10G10B10_USCALED_PACK32: i32 = 60,
    VK_FORMAT_A2R10G10B10_SSCALED_PACK32: i32 = 61,
    VK_FORMAT_A2R10G10B10_UINT_PACK32: i32 = 62,
    VK_FORMAT_A2R10G10B10_SINT_PACK32: i32 = 63,
    VK_FORMAT_A2B10G10R10_UNORM_PACK32: i32 = 64,
    VK_FORMAT_A2B10G10R10_SNORM_PACK32: i32 = 65,
    VK_FORMAT_A2B10G10R10_USCALED_PACK32: i32 = 66,
    VK_FORMAT_A2B10G10R10_SSCALED_PACK32: i32 = 67,
    VK_FORMAT_A2B10G10R10_UINT_PACK32: i32 = 68,
    VK_FORMAT_A2B10G10R10_SINT_PACK32: i32 = 69,
    VK_FORMAT_R16_UNORM: i32 = 70,
    VK_FORMAT_R16_SNORM: i32 = 71,
    VK_FORMAT_R16_USCALED: i32 = 72,
    VK_FORMAT_R16_SSCALED: i32 = 73,
    VK_FORMAT_R16_UINT: i32 = 74,
    VK_FORMAT_R16_SINT: i32 = 75,
    VK_FORMAT_R16_SFLOAT: i32 = 76,
    VK_FORMAT_R16G16_UNORM: i32 = 77,
    VK_FORMAT_R16G16_SNORM: i32 = 78,
    VK_FORMAT_R16G16_USCALED: i32 = 79,
    VK_FORMAT_R16G16_SSCALED: i32 = 80,
    VK_FORMAT_R16G16_UINT: i32 = 81,
    VK_FORMAT_R16G16_SINT: i32 = 82,
    VK_FORMAT_R16G16_SFLOAT: i32 = 83,
    VK_FORMAT_R16G16B16_UNORM: i32 = 84,
    VK_FORMAT_R16G16B16_SNORM: i32 = 85,
    VK_FORMAT_R16G16B16_USCALED: i32 = 86,
    VK_FORMAT_R16G16B16_SSCALED: i32 = 87,
    VK_FORMAT_R16G16B16_UINT: i32 = 88,
    VK_FORMAT_R16G16B16_SINT: i32 = 89,
    VK_FORMAT_R16G16B16_SFLOAT: i32 = 90,
    VK_FORMAT_R16G16B16A16_UNORM: i32 = 91,
    VK_FORMAT_R16G16B16A16_SNORM: i32 = 92,
    VK_FORMAT_R16G16B16A16_USCALED: i32 = 93,
    VK_FORMAT_R16G16B16A16_SSCALED: i32 = 94,
    VK_FORMAT_R16G16B16A16_UINT: i32 = 95,
    VK_FORMAT_R16G16B16A16_SINT: i32 = 96,
    VK_FORMAT_R16G16B16A16_SFLOAT: i32 = 97,
    VK_FORMAT_R32_UINT: i32 = 98,
    VK_FORMAT_R32_SINT: i32 = 99,
    VK_FORMAT_R32_SFLOAT: i32 = 100,
    VK_FORMAT_R32G32_UINT: i32 = 101,
    VK_FORMAT_R32G32_SINT: i32 = 102,
    VK_FORMAT_R32G32_SFLOAT: i32 = 103,
    VK_FORMAT_R32G32B32_UINT: i32 = 104,
    VK_FORMAT_R32G32B32_SINT: i32 = 105,
    VK_FORMAT_R32G32B32_SFLOAT: i32 = 106,
    VK_FORMAT_R32G32B32A32_UINT: i32 = 107,
    VK_FORMAT_R32G32B32A32_SINT: i32 = 108,
    VK_FORMAT_R32G32B32A32_SFLOAT: i32 = 109,
    VK_FORMAT_R64_UINT: i32 = 110,
    VK_FORMAT_R64_SINT: i32 = 111,
    VK_FORMAT_R64_SFLOAT: i32 = 112,
    VK_FORMAT_R64G64_UINT: i32 = 113,
    VK_FORMAT_R64G64_SINT: i32 = 114,
    VK_FORMAT_R64G64_SFLOAT: i32 = 115,
    VK_FORMAT_R64G64B64_UINT: i32 = 116,
    VK_FORMAT_R64G64B64_SINT: i32 = 117,
    VK_FORMAT_R64G64B64_SFLOAT: i32 = 118,
    VK_FORMAT_R64G64B64A64_UINT: i32 = 119,
    VK_FORMAT_R64G64B64A64_SINT: i32 = 120,
    VK_FORMAT_R64G64B64A64_SFLOAT: i32 = 121,
    VK_FORMAT_B10G11R11_UFLOAT_PACK32: i32 = 122,
    VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: i32 = 123,
    VK_FORMAT_D16_UNORM: i32 = 124,
    VK_FORMAT_X8_D24_UNORM_PACK32: i32 = 125,
    VK_FORMAT_D32_SFLOAT: i32 = 126,
    VK_FORMAT_S8_UINT: i32 = 127,
    VK_FORMAT_D16_UNORM_S8_UINT: i32 = 128,
    VK_FORMAT_D24_UNORM_S8_UINT: i32 = 129,
    VK_FORMAT_D32_SFLOAT_S8_UINT: i32 = 130,
    VK_FORMAT_BC1_RGB_UNORM_BLOCK: i32 = 131,
    VK_FORMAT_BC1_RGB_SRGB_BLOCK: i32 = 132,
    VK_FORMAT_BC1_RGBA_UNORM_BLOCK: i32 = 133,
    VK_FORMAT_BC1_RGBA_SRGB_BLOCK: i32 = 134,
    VK_FORMAT_BC2_UNORM_BLOCK: i32 = 135,
    VK_FORMAT_BC2_SRGB_BLOCK: i32 = 136,
    VK_FORMAT_BC3_UNORM_BLOCK: i32 = 137,
    VK_FORMAT_BC3_SRGB_BLOCK: i32 = 138,
    VK_FORMAT_BC4_UNORM_BLOCK: i32 = 139,
    VK_FORMAT_BC4_SNORM_BLOCK: i32 = 140,
    VK_FORMAT_BC5_UNORM_BLOCK: i32 = 141,
    VK_FORMAT_BC5_SNORM_BLOCK: i32 = 142,
    VK_FORMAT_BC6H_UFLOAT_BLOCK: i32 = 143,
    VK_FORMAT_BC6H_SFLOAT_BLOCK: i32 = 144,
    VK_FORMAT_BC7_UNORM_BLOCK: i32 = 145,
    VK_FORMAT_BC7_SRGB_BLOCK: i32 = 146,
    VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: i32 = 147,
    VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: i32 = 148,
    VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: i32 = 149,
    VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: i32 = 150,
    VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: i32 = 151,
    VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: i32 = 152,
    VK_FORMAT_EAC_R11_UNORM_BLOCK: i32 = 153,
    VK_FORMAT_EAC_R11_SNORM_BLOCK: i32 = 154,
    VK_FORMAT_EAC_R11G11_UNORM_BLOCK: i32 = 155,
    VK_FORMAT_EAC_R11G11_SNORM_BLOCK: i32 = 156,
    VK_FORMAT_ASTC_4x4_UNORM_BLOCK: i32 = 157,
    VK_FORMAT_ASTC_4x4_SRGB_BLOCK: i32 = 158,
    VK_FORMAT_ASTC_5x4_UNORM_BLOCK: i32 = 159,
    VK_FORMAT_ASTC_5x4_SRGB_BLOCK: i32 = 160,
    VK_FORMAT_ASTC_5x5_UNORM_BLOCK: i32 = 161,
    VK_FORMAT_ASTC_5x5_SRGB_BLOCK: i32 = 162,
    VK_FORMAT_ASTC_6x5_UNORM_BLOCK: i32 = 163,
    VK_FORMAT_ASTC_6x5_SRGB_BLOCK: i32 = 164,
    VK_FORMAT_ASTC_6x6_UNORM_BLOCK: i32 = 165,
    VK_FORMAT_ASTC_6x6_SRGB_BLOCK: i32 = 166,
    VK_FORMAT_ASTC_8x5_UNORM_BLOCK: i32 = 167,
    VK_FORMAT_ASTC_8x5_SRGB_BLOCK: i32 = 168,
    VK_FORMAT_ASTC_8x6_UNORM_BLOCK: i32 = 169,
    VK_FORMAT_ASTC_8x6_SRGB_BLOCK: i32 = 170,
    VK_FORMAT_ASTC_8x8_UNORM_BLOCK: i32 = 171,
    VK_FORMAT_ASTC_8x8_SRGB_BLOCK: i32 = 172,
    VK_FORMAT_ASTC_10x5_UNORM_BLOCK: i32 = 173,
    VK_FORMAT_ASTC_10x5_SRGB_BLOCK: i32 = 174,
    VK_FORMAT_ASTC_10x6_UNORM_BLOCK: i32 = 175,
    VK_FORMAT_ASTC_10x6_SRGB_BLOCK: i32 = 176,
    VK_FORMAT_ASTC_10x8_UNORM_BLOCK: i32 = 177,
    VK_FORMAT_ASTC_10x8_SRGB_BLOCK: i32 = 178,
    VK_FORMAT_ASTC_10x10_UNORM_BLOCK: i32 = 179,
    VK_FORMAT_ASTC_10x10_SRGB_BLOCK: i32 = 180,
    VK_FORMAT_ASTC_12x10_UNORM_BLOCK: i32 = 181,
    VK_FORMAT_ASTC_12x10_SRGB_BLOCK: i32 = 182,
    VK_FORMAT_ASTC_12x12_UNORM_BLOCK: i32 = 183,
    VK_FORMAT_ASTC_12x12_SRGB_BLOCK: i32 = 184,
}

pub enum VkStructureType
{
    VK_STRUCTURE_TYPE_APPLICATION_INFO: i32 = 0,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: i32 = 1,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: i32 = 2,
    VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: i32 = 3,
    VK_STRUCTURE_TYPE_SUBMIT_INFO: i32 = 4,
    VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: i32 = 5,
    VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: i32 = 6,
    VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: i32 = 7,
    VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: i32 = 8,
    VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: i32 = 9,
    VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: i32 = 10,
    VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: i32 = 11,
    VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: i32 = 12,
    VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: i32 = 13,
    VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: i32 = 14,
    VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: i32 = 15,
    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: i32 = 16,
    VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: i32 = 17,
    VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: i32 = 18,
    VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: i32 = 19,
    VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: i32 = 20,
    VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: i32 = 21,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: i32 = 22,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: i32 = 23,
    VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: i32 = 24,
    VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: i32 = 25,
    VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: i32 = 26,
    VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: i32 = 27,
    VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: i32 = 28,
    VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: i32 = 29,
    VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: i32 = 30,
    VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: i32 = 31,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: i32 = 32,
    VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: i32 = 33,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: i32 = 34,
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: i32 = 35,
    VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: i32 = 36,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: i32 = 37,
    VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: i32 = 38,
    VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: i32 = 39,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: i32 = 40,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: i32 = 41,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: i32 = 42,
    VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: i32 = 43,
    VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: i32 = 44,
    VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: i32 = 45,
    VK_STRUCTURE_TYPE_MEMORY_BARRIER: i32 = 46,
    VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: i32 = 47,
    VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: i32 = 48,
}

pub enum VkSubpassContents
{
    VK_SUBPASS_CONTENTS_INLINE: i32 = 0,
    VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: i32 = 1,
}

pub enum VkResult
{
    VK_SUCCESS: i32 = 0,
    VK_NOT_READY: i32 = 1,
    VK_TIMEOUT: i32 = 2,
    VK_EVENT_SET: i32 = 3,
    VK_EVENT_RESET: i32 = 4,
    VK_INCOMPLETE: i32 = 5,
    VK_ERROR_OUT_OF_HOST_MEMORY: i32 = -1,
    VK_ERROR_OUT_OF_DEVICE_MEMORY: i32 = -2,
    VK_ERROR_INITIALIZATION_FAILED: i32 = -3,
    VK_ERROR_DEVICE_LOST: i32 = -4,
    VK_ERROR_MEMORY_MAP_FAILED: i32 = -5,
    VK_ERROR_LAYER_NOT_PRESENT: i32 = -6,
    VK_ERROR_EXTENSION_NOT_PRESENT: i32 = -7,
    VK_ERROR_FEATURE_NOT_PRESENT: i32 = -8,
    VK_ERROR_INCOMPATIBLE_DRIVER: i32 = -9,
    VK_ERROR_TOO_MANY_OBJECTS: i32 = -10,
    VK_ERROR_FORMAT_NOT_SUPPORTED: i32 = -11,
    VK_ERROR_FRAGMENTED_POOL: i32 = -12,
    VK_ERROR_UNKNOWN: i32 = -13,
}

pub enum VkDynamicState
{
    VK_DYNAMIC_STATE_VIEWPORT: i32 = 0,
    VK_DYNAMIC_STATE_SCISSOR: i32 = 1,
    VK_DYNAMIC_STATE_LINE_WIDTH: i32 = 2,
    VK_DYNAMIC_STATE_DEPTH_BIAS: i32 = 3,
    VK_DYNAMIC_STATE_BLEND_CONSTANTS: i32 = 4,
    VK_DYNAMIC_STATE_DEPTH_BOUNDS: i32 = 5,
    VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: i32 = 6,
    VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: i32 = 7,
    VK_DYNAMIC_STATE_STENCIL_REFERENCE: i32 = 8,
}

pub enum VkDescriptorUpdateTemplateType
{
    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET: i32 = 0,
}

pub enum VkObjectType
{
    VK_OBJECT_TYPE_UNKNOWN: i32 = 0,
    VK_OBJECT_TYPE_INSTANCE: i32 = 1,
    VK_OBJECT_TYPE_PHYSICAL_DEVICE: i32 = 2,
    VK_OBJECT_TYPE_DEVICE: i32 = 3,
    VK_OBJECT_TYPE_QUEUE: i32 = 4,
    VK_OBJECT_TYPE_SEMAPHORE: i32 = 5,
    VK_OBJECT_TYPE_COMMAND_BUFFER: i32 = 6,
    VK_OBJECT_TYPE_FENCE: i32 = 7,
    VK_OBJECT_TYPE_DEVICE_MEMORY: i32 = 8,
    VK_OBJECT_TYPE_BUFFER: i32 = 9,
    VK_OBJECT_TYPE_IMAGE: i32 = 10,
    VK_OBJECT_TYPE_EVENT: i32 = 11,
    VK_OBJECT_TYPE_QUERY_POOL: i32 = 12,
    VK_OBJECT_TYPE_BUFFER_VIEW: i32 = 13,
    VK_OBJECT_TYPE_IMAGE_VIEW: i32 = 14,
    VK_OBJECT_TYPE_SHADER_MODULE: i32 = 15,
    VK_OBJECT_TYPE_PIPELINE_CACHE: i32 = 16,
    VK_OBJECT_TYPE_PIPELINE_LAYOUT: i32 = 17,
    VK_OBJECT_TYPE_RENDER_PASS: i32 = 18,
    VK_OBJECT_TYPE_PIPELINE: i32 = 19,
    VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: i32 = 20,
    VK_OBJECT_TYPE_SAMPLER: i32 = 21,
    VK_OBJECT_TYPE_DESCRIPTOR_POOL: i32 = 22,
    VK_OBJECT_TYPE_DESCRIPTOR_SET: i32 = 23,
    VK_OBJECT_TYPE_FRAMEBUFFER: i32 = 24,
    VK_OBJECT_TYPE_COMMAND_POOL: i32 = 25,
}

pub enum VkSemaphoreType
{
    VK_SEMAPHORE_TYPE_BINARY: i32 = 0,
    VK_SEMAPHORE_TYPE_TIMELINE: i32 = 1,
}

pub enum VkPresentModeKHR
{
    VK_PRESENT_MODE_IMMEDIATE_KHR: i32 = 0,
    VK_PRESENT_MODE_MAILBOX_KHR: i32 = 1,
    VK_PRESENT_MODE_FIFO_KHR: i32 = 2,
    VK_PRESENT_MODE_FIFO_RELAXED_KHR: i32 = 3,
}

pub enum VkColorSpaceKHR
{
    VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: i32 = 0,
}

pub enum VkTimeDomainEXT
{
    VK_TIME_DOMAIN_DEVICE_EXT: i32 = 0,
    VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT: i32 = 1,
    VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT: i32 = 2,
    VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT: i32 = 3,
}

pub enum VkDebugReportObjectTypeEXT
{
    VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: i32 = 0,
    VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: i32 = 1,
    VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: i32 = 2,
    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: i32 = 3,
    VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: i32 = 4,
    VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: i32 = 5,
    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: i32 = 6,
    VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: i32 = 7,
    VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: i32 = 8,
    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: i32 = 9,
    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: i32 = 10,
    VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: i32 = 11,
    VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: i32 = 12,
    VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: i32 = 13,
    VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: i32 = 14,
    VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: i32 = 15,
    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: i32 = 16,
    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: i32 = 17,
    VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: i32 = 18,
    VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: i32 = 19,
    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: i32 = 20,
    VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: i32 = 21,
    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: i32 = 22,
    VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: i32 = 23,
    VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: i32 = 24,
    VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: i32 = 25,
    VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: i32 = 26,
    VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: i32 = 27,
    VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: i32 = 28,
    VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: i32 = 29,
    VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: i32 = 30,
    VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT: i32 = 33,
}

pub enum VkDeviceMemoryReportEventTypeEXT
{
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT: i32 = 0,
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT: i32 = 1,
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT: i32 = 2,
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT: i32 = 3,
    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT: i32 = 4,
}

pub enum VkRasterizationOrderAMD
{
    VK_RASTERIZATION_ORDER_STRICT_AMD: i32 = 0,
    VK_RASTERIZATION_ORDER_RELAXED_AMD: i32 = 1,
}

pub enum VkValidationCheckEXT
{
    VK_VALIDATION_CHECK_ALL_EXT: i32 = 0,
    VK_VALIDATION_CHECK_SHADERS_EXT: i32 = 1,
}

pub enum VkValidationFeatureEnableEXT
{
    VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT: i32 = 0,
    VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: i32 = 1,
    VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT: i32 = 2,
    VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT: i32 = 3,
    VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT: i32 = 4,
}

pub enum VkValidationFeatureDisableEXT
{
    VK_VALIDATION_FEATURE_DISABLE_ALL_EXT: i32 = 0,
    VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT: i32 = 1,
    VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT: i32 = 2,
    VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT: i32 = 3,
    VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT: i32 = 4,
    VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT: i32 = 5,
    VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT: i32 = 6,
    VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT: i32 = 7,
}

pub enum VkIndirectCommandsTokenTypeNV
{
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV: i32 = 0,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV: i32 = 1,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV: i32 = 2,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV: i32 = 3,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV: i32 = 4,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV: i32 = 5,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV: i32 = 6,
    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV: i32 = 7,
}

pub enum VkDisplayPowerStateEXT
{
    VK_DISPLAY_POWER_STATE_OFF_EXT: i32 = 0,
    VK_DISPLAY_POWER_STATE_SUSPEND_EXT: i32 = 1,
    VK_DISPLAY_POWER_STATE_ON_EXT: i32 = 2,
}

pub enum VkDeviceEventTypeEXT
{
    VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: i32 = 0,
}

pub enum VkDisplayEventTypeEXT
{
    VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: i32 = 0,
}

pub enum VkViewportCoordinateSwizzleNV
{
    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: i32 = 0,
    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: i32 = 1,
    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: i32 = 2,
    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: i32 = 3,
    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: i32 = 4,
    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: i32 = 5,
    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: i32 = 6,
    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: i32 = 7,
}

pub enum VkDiscardRectangleModeEXT
{
    VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: i32 = 0,
    VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: i32 = 1,
}

pub enum VkPointClippingBehavior
{
    VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: i32 = 0,
    VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: i32 = 1,
}

pub enum VkSamplerReductionMode
{
    VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: i32 = 0,
    VK_SAMPLER_REDUCTION_MODE_MIN: i32 = 1,
    VK_SAMPLER_REDUCTION_MODE_MAX: i32 = 2,
}

pub enum VkTessellationDomainOrigin
{
    VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: i32 = 0,
    VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: i32 = 1,
}

pub enum VkSamplerYcbcrModelConversion
{
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: i32 = 0,
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: i32 = 1,
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: i32 = 2,
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: i32 = 3,
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: i32 = 4,
}

pub enum VkSamplerYcbcrRange
{
    VK_SAMPLER_YCBCR_RANGE_ITU_FULL: i32 = 0,
    VK_SAMPLER_YCBCR_RANGE_ITU_NARROW: i32 = 1,
}

pub enum VkChromaLocation
{
    VK_CHROMA_LOCATION_COSITED_EVEN: i32 = 0,
    VK_CHROMA_LOCATION_MIDPOINT: i32 = 1,
}

pub enum VkBlendOverlapEXT
{
    VK_BLEND_OVERLAP_UNCORRELATED_EXT: i32 = 0,
    VK_BLEND_OVERLAP_DISJOINT_EXT: i32 = 1,
    VK_BLEND_OVERLAP_CONJOINT_EXT: i32 = 2,
}

pub enum VkCoverageModulationModeNV
{
    VK_COVERAGE_MODULATION_MODE_NONE_NV: i32 = 0,
    VK_COVERAGE_MODULATION_MODE_RGB_NV: i32 = 1,
    VK_COVERAGE_MODULATION_MODE_ALPHA_NV: i32 = 2,
    VK_COVERAGE_MODULATION_MODE_RGBA_NV: i32 = 3,
}

pub enum VkCoverageReductionModeNV
{
    VK_COVERAGE_REDUCTION_MODE_MERGE_NV: i32 = 0,
    VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV: i32 = 1,
}

pub enum VkValidationCacheHeaderVersionEXT
{
    VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: i32 = 1,
}

pub enum VkShaderInfoTypeAMD
{
    VK_SHADER_INFO_TYPE_STATISTICS_AMD: i32 = 0,
    VK_SHADER_INFO_TYPE_BINARY_AMD: i32 = 1,
    VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD: i32 = 2,
}

pub enum VkQueueGlobalPriorityEXT
{
    VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT: i32 = 128,
    VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT: i32 = 256,
    VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT: i32 = 512,
    VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT: i32 = 1024,
}

pub enum VkConservativeRasterizationModeEXT
{
    VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: i32 = 0,
    VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: i32 = 1,
    VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: i32 = 2,
}

pub enum VkVendorId
{
    VK_VENDOR_ID_VIV: i32 = 0x10001,
    VK_VENDOR_ID_VSI: i32 = 0x10002,
    VK_VENDOR_ID_KAZAN: i32 = 0x10003,
    VK_VENDOR_ID_CODEPLAY: i32 = 0x10004,
    VK_VENDOR_ID_MESA: i32 = 0x10005,
    VK_VENDOR_ID_POCL: i32 = 0x10006,
}

pub enum VkDriverId
{
    VK_DRIVER_ID_AMD_PROPRIETARY: i32 = 1,
    VK_DRIVER_ID_AMD_OPEN_SOURCE: i32 = 2,
    VK_DRIVER_ID_MESA_RADV: i32 = 3,
    VK_DRIVER_ID_NVIDIA_PROPRIETARY: i32 = 4,
    VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: i32 = 5,
    VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA: i32 = 6,
    VK_DRIVER_ID_IMAGINATION_PROPRIETARY: i32 = 7,
    VK_DRIVER_ID_QUALCOMM_PROPRIETARY: i32 = 8,
    VK_DRIVER_ID_ARM_PROPRIETARY: i32 = 9,
    VK_DRIVER_ID_GOOGLE_SWIFTSHADER: i32 = 10,
    VK_DRIVER_ID_GGP_PROPRIETARY: i32 = 11,
    VK_DRIVER_ID_BROADCOM_PROPRIETARY: i32 = 12,
    VK_DRIVER_ID_MESA_LLVMPIPE: i32 = 13,
    VK_DRIVER_ID_MOLTENVK: i32 = 14,
    VK_DRIVER_ID_COREAVI_PROPRIETARY: i32 = 15,
    VK_DRIVER_ID_JUICE_PROPRIETARY: i32 = 16,
    VK_DRIVER_ID_VERISILICON_PROPRIETARY: i32 = 17,
}

pub enum VkShadingRatePaletteEntryNV
{
    VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV: i32 = 0,
    VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV: i32 = 1,
    VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV: i32 = 2,
    VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV: i32 = 3,
    VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV: i32 = 4,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV: i32 = 5,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV: i32 = 6,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV: i32 = 7,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV: i32 = 8,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV: i32 = 9,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV: i32 = 10,
    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV: i32 = 11,
}

pub enum VkCoarseSampleOrderTypeNV
{
    VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV: i32 = 0,
    VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV: i32 = 1,
    VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV: i32 = 2,
    VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV: i32 = 3,
}

pub enum VkCopyAccelerationStructureModeKHR
{
    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR: i32 = 0,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR: i32 = 1,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR: i32 = 2,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR: i32 = 3,
}

pub enum VkBuildAccelerationStructureModeKHR
{
    VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR: i32 = 0,
    VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR: i32 = 1,
}

pub enum VkAccelerationStructureTypeKHR
{
    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR: i32 = 0,
    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR: i32 = 1,
    VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR: i32 = 2,
}

pub enum VkGeometryTypeKHR
{
    VK_GEOMETRY_TYPE_TRIANGLES_KHR: i32 = 0,
    VK_GEOMETRY_TYPE_AABBS_KHR: i32 = 1,
    VK_GEOMETRY_TYPE_INSTANCES_KHR: i32 = 2,
}

pub enum VkAccelerationStructureMemoryRequirementsTypeNV
{
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV: i32 = 0,
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV: i32 = 1,
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV: i32 = 2,
}

pub enum VkAccelerationStructureBuildTypeKHR
{
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR: i32 = 0,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR: i32 = 1,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR: i32 = 2,
}

pub enum VkRayTracingShaderGroupTypeKHR
{
    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR: i32 = 0,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR: i32 = 1,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR: i32 = 2,
}

pub enum VkAccelerationStructureCompatibilityKHR
{
    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR: i32 = 0,
    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR: i32 = 1,
}

pub enum VkShaderGroupShaderKHR
{
    VK_SHADER_GROUP_SHADER_GENERAL_KHR: i32 = 0,
    VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR: i32 = 1,
    VK_SHADER_GROUP_SHADER_ANY_HIT_KHR: i32 = 2,
    VK_SHADER_GROUP_SHADER_INTERSECTION_KHR: i32 = 3,
}

pub enum VkMemoryOverallocationBehaviorAMD
{
    VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD: i32 = 0,
    VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD: i32 = 1,
    VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD: i32 = 2,
}

pub enum VkScopeNV
{
    VK_SCOPE_DEVICE_NV: i32 = 1,
    VK_SCOPE_WORKGROUP_NV: i32 = 2,
    VK_SCOPE_SUBGROUP_NV: i32 = 3,
    VK_SCOPE_QUEUE_FAMILY_NV: i32 = 5,
}

pub enum VkComponentTypeNV
{
    VK_COMPONENT_TYPE_FLOAT16_NV: i32 = 0,
    VK_COMPONENT_TYPE_FLOAT32_NV: i32 = 1,
    VK_COMPONENT_TYPE_FLOAT64_NV: i32 = 2,
    VK_COMPONENT_TYPE_SINT8_NV: i32 = 3,
    VK_COMPONENT_TYPE_SINT16_NV: i32 = 4,
    VK_COMPONENT_TYPE_SINT32_NV: i32 = 5,
    VK_COMPONENT_TYPE_SINT64_NV: i32 = 6,
    VK_COMPONENT_TYPE_UINT8_NV: i32 = 7,
    VK_COMPONENT_TYPE_UINT16_NV: i32 = 8,
    VK_COMPONENT_TYPE_UINT32_NV: i32 = 9,
    VK_COMPONENT_TYPE_UINT64_NV: i32 = 10,
}

pub enum VkFullScreenExclusiveEXT
{
    VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: i32 = 0,
    VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: i32 = 1,
    VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: i32 = 2,
    VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: i32 = 3,
}

pub enum VkPerformanceCounterScopeKHR
{
    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR: i32 = 0,
    VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR: i32 = 1,
    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR: i32 = 2,
}

pub enum VkPerformanceCounterUnitKHR
{
    VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR: i32 = 0,
    VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR: i32 = 1,
    VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR: i32 = 2,
    VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR: i32 = 3,
    VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR: i32 = 4,
    VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR: i32 = 5,
    VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR: i32 = 6,
    VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR: i32 = 7,
    VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR: i32 = 8,
    VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR: i32 = 9,
    VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR: i32 = 10,
}

pub enum VkPerformanceCounterStorageKHR
{
    VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR: i32 = 0,
    VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR: i32 = 1,
    VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR: i32 = 2,
    VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR: i32 = 3,
    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR: i32 = 4,
    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR: i32 = 5,
}

pub enum VkPerformanceConfigurationTypeINTEL
{
    VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: i32 = 0,
}

pub enum VkQueryPoolSamplingModeINTEL
{
    VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL: i32 = 0,
}

pub enum VkPerformanceOverrideTypeINTEL
{
    VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL: i32 = 0,
    VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL: i32 = 1,
}

pub enum VkPerformanceParameterTypeINTEL
{
    VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL: i32 = 0,
    VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL: i32 = 1,
}

pub enum VkPerformanceValueTypeINTEL
{
    VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL: i32 = 0,
    VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL: i32 = 1,
    VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL: i32 = 2,
    VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL: i32 = 3,
    VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL: i32 = 4,
}

pub enum VkShaderFloatControlsIndependence
{
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: i32 = 0,
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: i32 = 1,
    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: i32 = 2,
}

pub enum VkPipelineExecutableStatisticFormatKHR
{
    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR: i32 = 0,
    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR: i32 = 1,
    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR: i32 = 2,
    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR: i32 = 3,
}

pub enum VkLineRasterizationModeEXT
{
    VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT: i32 = 0,
    VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT: i32 = 1,
    VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT: i32 = 2,
    VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT: i32 = 3,
}

pub enum VkFragmentShadingRateCombinerOpKHR
{
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR: i32 = 0,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR: i32 = 1,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR: i32 = 2,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR: i32 = 3,
    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR: i32 = 4,
}

pub enum VkFragmentShadingRateNV
{
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: i32 = 0,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: i32 = 1,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: i32 = 4,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: i32 = 5,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: i32 = 6,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: i32 = 9,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: i32 = 10,
    VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: i32 = 11,
    VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: i32 = 12,
    VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: i32 = 13,
    VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: i32 = 14,
    VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV: i32 = 15,
}

pub enum VkFragmentShadingRateTypeNV
{
    VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV: i32 = 0,
    VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV: i32 = 1,
}

pub enum VkProvokingVertexModeEXT
{
    VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT: i32 = 0,
    VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT: i32 = 1,
}

pub enum VkAccelerationStructureMotionInstanceTypeNV
{
    VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV: i32 = 0,
    VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV: i32 = 1,
    VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV: i32 = 2,
}

pub enum VkQueryResultStatusKHR
{
    VK_QUERY_RESULT_STATUS_ERROR_KHR: i32 = -1,
    VK_QUERY_RESULT_STATUS_NOT_READY_KHR: i32 = 0,
    VK_QUERY_RESULT_STATUS_COMPLETE_KHR: i32 = 1,
}

