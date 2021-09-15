

pub struct RenderPassParams
{

}

struct RenderHandle
{
    handle_index: u64,
    handle_type: u32,
    handle_generation: u32
}

impl RenderHandle 
{
    fn is_valid(&self) -> bool { self.handle_index != 0 }
    fn create_handle() -> RenderHandle 
    { 
        RenderHandle{ handle_index: 0, handle_type: 0, handle_generation: 0 }
    }
}


pub enum MemoryType
{
    NORMALIZED8,
    NORMALIZED16,
    UINT8,
    UINT16,
    UINT32,
    INT8,
    INT16,
    INT32,
    FLOAT16,
    FLOAT32,
}

#[repr(u32)]
pub enum TextureUsage
{
    Color = 1,
    Depth = 2,
    GraphicsWrite = 4,
    ComputeWrite = 8,

    SampleGraphics = 16,
    SampleCompute = 32,
    Persistent = 64,
}

pub struct TextureInfo
{
    handle: RenderHandle,
    texture_memory_type: MemoryType,
    width: u32,
    height: u32,

    r_bits: u8,
    g_bits: u8,
    b_bits: u8,
    a_bits: u8,
}
impl TextureInfo 
{
    pub fn is_valid(&self) -> bool { self.handle.is_valid() }
}

pub struct RenderTarget
{
    texture: TextureInfo,
    usage: TextureUsage,
}

pub struct RenderPass
{
    handle: RenderHandle,
    targets: Vec<u64>,
    shader_handle: u64,

    binds: Vec<(u32, u64)>,
}

impl RenderPass
{
    pub fn new() -> Self
    {
        Self { handle: RenderHandle::create_handle(), targets: Vec::new(), shader_handle: 0, binds: Vec::new() }
    }

    pub fn is_valid(&self) -> bool { self.handle.is_valid() }
}


pub trait Reneder
{
    fn create_render_pass(&mut self);
    fn load_shader(&mut self);


    fn get_render_pass(&self, handle: u64) -> RenderPass;
}