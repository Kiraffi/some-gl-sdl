use std::ffi::{CString, CStr};


extern "system" fn gl_callback(msg_source: gl::GLenum, msg_type: gl::GLenum,
    _id: gl::GLuint, severity: gl::GLenum, _length: gl::GLsizei,
    message: *const gl::GLchar, _user_param: *mut std::os::raw::c_void)
{
    let message = unsafe
    {
        String::from_utf8(CStr::from_ptr(message).to_bytes().to_vec()).unwrap()
    };


    match severity
    {
        gl::GL_DEBUG_SEVERITY_NOTIFICATION =>
        {
    //        println!("Info: {}", message);
        },

        gl::GL_DEBUG_SEVERITY_LOW |
            gl::GL_DEBUG_SEVERITY_MEDIUM |
            gl::GL_DEBUG_SEVERITY_HIGH =>
        {
            println!("Message: {}", message);
        }
        _=>
        {
            return;
        }
    };

    match msg_source
    {
        gl::GL_DEBUG_SOURCE_API => {},
        gl::GL_DEBUG_SOURCE_WINDOW_SYSTEM => {},
        gl::GL_DEBUG_SOURCE_SHADER_COMPILER => {},
        gl::GL_DEBUG_SOURCE_THIRD_PARTY => {},
        gl::GL_DEBUG_SOURCE_APPLICATION => {},
        gl::GL_DEBUG_SOURCE_OTHER => {},
        _ => return
    };

    match msg_type
    {
        gl::GL_DEBUG_TYPE_ERROR => {},
        gl::GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR => {},
        gl::GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR => {},
        gl::GL_DEBUG_TYPE_PORTABILITY => {},
        gl::GL_DEBUG_TYPE_PERFORMANCE => {},
        gl::GL_DEBUG_TYPE_MARKER => {},
        gl::GL_DEBUG_TYPE_PUSH_GROUP => {},
        gl::GL_DEBUG_TYPE_POP_GROUP => {},
        gl::GL_DEBUG_TYPE_OTHER => {},
        _ => return
    };

}

pub fn init_gl<F>(window_state: &window_state::WindowState, loadfn: F) -> Result<(), String> where F: Fn(&'static str) -> *const std::os::raw::c_void
{
    unsafe
    {
        let _gl = gl::load_with(&|s| loadfn(s));
        //gl::load_gl_funcs(_opengl32, _wglGetProcAddress.unwrap());
        //gl::load_gl_funcs(_opengl32, _wglGetProcAddress.unwrap());
        //gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::glDebugMessageCallback(Some(gl_callback), std::ptr::null());
        gl::glDebugMessageControl(gl::GL_DONT_CARE, gl::GL_DONT_CARE, gl::GL_DONT_CARE, 0,
                                    std::ptr::null(), gl::GL_TRUE as u8);
        gl::glEnable(gl::GL_DEBUG_OUTPUT);
    }

    let version;
    match unsafe
    {
        let data = CStr::from_ptr(gl::glGetString(gl::GL_VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data)
    }
    {
        Ok(v) =>
        {
            version = v;
        }
        Err(e) =>
        {
            println!("Error: {}", e);
            return Err("Failed to read version data from gl!".to_string());
        }
    }

    println!("OpenGL version {}", version);


    unsafe
    {
        resize(window_state.window_width, window_state.window_height);
        gl::glClearColor(0.2, 0.3, 0.5, 1.0);
        gl::glClearDepth(1.0);
        // Swapping up and down just messes things up like in renderdoc....
        gl::glClipControl(gl::GL_UPPER_LEFT, gl::GL_ZERO_TO_ONE);
        //gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
    }
    return Ok(());
}

pub fn resize(window_width : i32, window_height: i32)
{
    unsafe
    {
        gl::glViewport(0, 0, window_width, window_height);
    }
}

pub fn update(window_state: &mut window_state::WindowState)
{
    if window_state.resized
    {
        resize(window_state.window_width, window_state.window_height);
        window_state.resized = false;
    }
}


pub struct CommonShaderFrameDate
{
    pub screen_size_x: f32,
    pub screen_size_y: f32,
    pub padding_1: f32,
    pub padding_2: f32,
}

impl CommonShaderFrameDate
{
    pub fn new(width: i32, height: i32) -> CommonShaderFrameDate
    {
        CommonShaderFrameDate{ screen_size_x: width as f32, screen_size_y: height as f32, padding_1: 0.0f32, padding_2: 0.0f32 }
    }
}

pub struct Program
{
    id: gl::GLuint,
}

impl Program
{
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String>
    {
        let program_id = unsafe { gl::glCreateProgram() };

        for shader in shaders
        {
            unsafe { gl::glAttachShader(program_id, shader.id()); }
        }

        unsafe { gl::glLinkProgram(program_id); }

        let mut success: gl::GLint = 1;
        unsafe
        {
            gl::glGetProgramiv(program_id, gl::GL_LINK_STATUS, &mut success);
        }

        if success == 0
        {
            let mut len: gl::GLint = 0;
            unsafe
            {
                gl::glGetProgramiv(program_id, gl::GL_INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe
            {
                gl::glGetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders
        {
            unsafe { gl::glDetachShader(program_id, shader.id()); }
        }

        Ok(Program { id: program_id })
    }

    pub fn id(&self) -> gl::GLuint
    {
        self.id
    }

    pub fn set_used(&self)
    {
        unsafe
        {
            gl::glUseProgram(self.id);
        }
    }



}

impl Drop for Program
{
    fn drop(&mut self)
    {
        unsafe
        {
            gl::glDeleteProgram(self.id);
        }
    }
}

pub struct Shader
{
    id: gl::GLuint,
}

impl Shader
{
    pub fn from_source(source: &CStr, kind: gl::GLenum ) -> Result<Shader, String>
    {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr, name: &String) -> Result<Shader, String>
    {
        match Shader::from_source(source, gl::GL_VERTEX_SHADER)
        {
            Ok(k) =>
            {
                return Ok(k);
            }
            Err(e) =>
            {
                println!("Failed to compile vertex shader: {}", name);
                return Err(e);
            }
        }
    }

    pub fn from_frag_source(source: &CStr, name: &String) -> Result<Shader, String>
    {
        match Shader::from_source(source, gl::GL_FRAGMENT_SHADER)
        {
            Ok(k) =>
            {
                return Ok(k);
            }
            Err(e) =>
            {
                println!("Failed to compile fragment shader: {}", name);
                return Err(e);
            }
        }
    }

    pub fn from_comp_source(source: &CStr, name: &String) -> Result<Shader, String>
    {
        match Shader::from_source(source, gl::GL_COMPUTE_SHADER)
        {
            Ok(k) =>
            {
                return Ok(k);
            }
            Err(e) =>
            {
                println!("Failed to compile compute shader: {}", name);
                return Err(e);
            }
        }
    }


    pub fn id(&self) -> gl::GLuint
    {
        self.id
    }
}

impl Drop for Shader
{
    fn drop(&mut self)
    {
        unsafe
        {
            gl::glDeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::GLenum )
    -> Result<gl::GLuint, String>
{
    let id = unsafe { gl::glCreateShader(kind) };
    unsafe
    {
        gl::glShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::glCompileShader(id);
    }

    let mut success: gl::GLint = 1;
    unsafe
    {
        gl::glGetShaderiv(id, gl::GL_COMPILE_STATUS, &mut success);
    }

    if success == 0
    {
        let mut len: gl::GLint = 0;
        unsafe
        {
            gl::glGetShaderiv(id, gl::GL_INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe
        {
            gl::glGetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize)
    -> CString
{
    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(len + 1, b' ');
    buffer[len] = b'\0';
    unsafe { CString::from_raw(buffer.as_ptr() as _) }
}


pub struct Texture
{
    pub handle: gl::GLuint,
    pub width: i32,
    pub height: i32,
    pub texture_type: gl::GLenum,
    pub pixel_type: gl::GLenum
}

impl Texture
{
    fn generate_handle(width: i32, height: i32, texture_type: gl::GLenum, pixel_type: gl::GLenum) -> gl::GLuint
    {
        let mut handle: gl::GLuint = 0;
        unsafe
        {
            gl::glCreateTextures(texture_type, 1, &mut handle);
            gl::glTextureStorage2D(handle, 1, pixel_type, width, height);
            //gl::TextureSubImage2D(handle, 0, 0, 0, width, height, gl::BGRA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::GLvoid);

            //gl::GenTextures(1, &mut handle);
            //gl::BindTexture(texture_type, handle);
            //gl::TexStorage2D(texture_type, 1, pixel_type, width, height);

            //gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, texture_width, texture_height, gl::BGRA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::GLvoid);
        }
        return handle;
    }
    // texture width, height, and type such as GL_TEXTURE_2D (gl::TEXTURE2D), pixel_type such as gl::RGBA8
    pub fn new_texture(width: i32, height: i32, texture_type: gl::GLenum, pixel_type: gl::GLenum) -> Self
    {
        let handle = Texture::generate_handle(width, height, texture_type, pixel_type);
        return Self { handle, width, height, texture_type, pixel_type };
    }

    pub fn resize(&mut self, width: i32, height: i32)
    {
        self.delete_texture();
        self.handle = Texture::generate_handle(width, height, self.texture_type, self.pixel_type);
        self.width = width;
        self.height = height;
    }

    pub fn delete_texture(&mut self)
    {
        if self.handle != 0
        {
            unsafe
            {
                gl::glDeleteTextures(1, &self.handle);
            }
            self.handle = 0;
        }
    }
    pub fn get_handle(&self) -> gl::GLuint
    {
        return self.handle;
    }

    pub fn get_pixel_type(&self) -> gl::GLenum
    {
        return self.pixel_type;
    }
}

impl Drop for Texture
{
    fn drop(&mut self)
    {
        self.delete_texture();
    }
}


pub struct ShaderBuffer
{
    handle: gl::GLuint,
    buffer_type: gl::GLenum,
    size: usize
}

impl ShaderBuffer
{
    pub fn new_with_data(buffer_type: gl::GLenum, size: usize, data_ptr: *const gl::GLvoid) -> Self
    {
        let mut tmp_handle: gl::GLuint = 0;
        unsafe
        {
            gl::glCreateBuffers(1, &mut tmp_handle);
            gl::glNamedBufferData(tmp_handle, size as gl::GLsizeiptr, data_ptr, gl::GL_DYNAMIC_COPY);

            //gl::GenBuffers(1, &mut tmp_handle);
            //gl::BindBuffer(buffer_type, tmp_handle);
            //gl::BufferData(
            //    buffer_type,
            //    size as gl::GLsizeiptr,
            //    data_ptr,
            //    gl::DYNAMIC_COPY, // usage
            //);
            //gl::BindBuffer(buffer_type, 0);

        }

        Self
        {
            handle: tmp_handle, buffer_type, size
        }
    }

    pub fn new(buffer_type: gl::GLenum, size: usize) -> Self
    {
        Self::new_with_data(buffer_type, size, 0 as *const gl::GLvoid)
    }
    pub fn bind(&self, slot :u32)
    {
        unsafe
        {
            gl::glBindBufferBase(self.buffer_type, slot, self.handle);
        }
    }

    pub fn write_data(&self, offset_in_bytes: usize, size: usize, ptr: *const gl::GLvoid)
    {
        unsafe
        {
            gl::glNamedBufferSubData(self.handle, offset_in_bytes as gl::GLintptr, size as gl::GLintptr, ptr);
        }
    }

    pub fn get_size(&self) -> usize
    {
        return self.size;
    }
}

impl Drop for ShaderBuffer
{
    fn drop(&mut self)
    {
        if self.handle != 0
        {
            unsafe
            {
                gl::glDeleteBuffers(1, &self.handle);
            }
            self.handle = 0;
        }
    }
}
