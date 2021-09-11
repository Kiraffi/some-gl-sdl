use std::ffi::{CString, CStr};


extern "system" fn gl_callback(msg_source: gl::types::GLenum, msg_type: gl::types::GLenum,
	_id: gl::types::GLuint, severity: gl::types::GLenum, _length: gl::types::GLsizei,
	message: *const gl::types::GLchar, _user_param: *mut std::os::raw::c_void)
{
	let message = unsafe
	{
		String::from_utf8(CStr::from_ptr(message).to_bytes().to_vec()).unwrap()
	};


	match severity
	{
		gl::DEBUG_SEVERITY_NOTIFICATION =>
		{
	//		println!("Info: {}", message);
		},

		gl::DEBUG_SEVERITY_LOW |
			gl::DEBUG_SEVERITY_MEDIUM |
			gl::DEBUG_SEVERITY_HIGH =>
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
		gl::DEBUG_SOURCE_API => {},
		gl::DEBUG_SOURCE_WINDOW_SYSTEM => {},
		gl::DEBUG_SOURCE_SHADER_COMPILER => {},
		gl::DEBUG_SOURCE_THIRD_PARTY => {},
		gl::DEBUG_SOURCE_APPLICATION => {},
		gl::DEBUG_SOURCE_OTHER => {},
		_ => return
	};

	match msg_type
	{
		gl::DEBUG_TYPE_ERROR => {},
		gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => {},
		gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => {},
		gl::DEBUG_TYPE_PORTABILITY => {},
		gl::DEBUG_TYPE_PERFORMANCE => {},
		gl::DEBUG_TYPE_MARKER => {},
		gl::DEBUG_TYPE_PUSH_GROUP => {},
		gl::DEBUG_TYPE_POP_GROUP => {},
		gl::DEBUG_TYPE_OTHER => {},
		_ => return
	};

}
extern "C" {
    #[doc = "  \\brief Get the address of an OpenGL function."]
    fn SDL_GL_GetProcAddress(proc_: *const libc::c_char) -> *mut libc::c_void;
}

#[doc(alias = "SDL_GL_GetProcAddress")]
pub fn gl_get_proc_address2(procname: &str) -> *const () {
	match CString::new(procname) {
		Ok(procname) => unsafe {
			SDL_GL_GetProcAddress(procname.as_ptr() as *const libc::c_char) as *const ()
		},
		// string contains a nul byte - it won't match anything.
		Err(_) => std::ptr::null(),
	}
}



pub fn init_gl(window_state: &sdl_window_state::SdlWindowState) -> Result<(), String>
{


	unsafe
	{
		let _gl = gl::load_with(&|s| gl_get_proc_address2(s) as *const std::os::raw::c_void);

		//gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
		gl::DebugMessageCallback(Some(gl_callback), std::ptr::null());
		gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0,
									std::ptr::null(), gl::TRUE);
		gl::Enable(gl::DEBUG_OUTPUT);
	}

	let version;
	match unsafe
	{
		let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
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
		gl::ClearColor(0.2, 0.3, 0.5, 1.0);
		gl::ClearDepth(1.0);
		// Swapping up and down just messes things up like in renderdoc....
		//gl::ClipControl(gl::UPPER_LEFT, gl::ZERO_TO_ONE);
		gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
	}
	return Ok(());
}

pub fn resize(window_width : i32, window_height: i32)
{
	unsafe
	{
		gl::Viewport(0, 0, window_width, window_height);
	}
}

pub fn update(window_state: &mut sdl_window_state::SdlWindowState)
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
	id: gl::types::GLuint,
}

impl Program
{
	pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String>
	{
		let program_id = unsafe { gl::CreateProgram() };

		for shader in shaders
		{
			unsafe { gl::AttachShader(program_id, shader.id()); }
		}

		unsafe { gl::LinkProgram(program_id); }

		let mut success: gl::types::GLint = 1;
		unsafe
		{
			gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
		}

		if success == 0
		{
			let mut len: gl::types::GLint = 0;
			unsafe
			{
				gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
			}

			let error = create_whitespace_cstring_with_len(len as usize);

			unsafe
			{
				gl::GetProgramInfoLog(
					program_id,
					len,
					std::ptr::null_mut(),
					error.as_ptr() as *mut gl::types::GLchar
				);
			}

			return Err(error.to_string_lossy().into_owned());
		}

		for shader in shaders
		{
			unsafe { gl::DetachShader(program_id, shader.id()); }
		}

		Ok(Program { id: program_id })
	}

	pub fn id(&self) -> gl::types::GLuint
	{
		self.id
	}

	pub fn set_used(&self)
	{
		unsafe
		{
			gl::UseProgram(self.id);
		}
	}



}

impl Drop for Program
{
	fn drop(&mut self)
	{
		unsafe
		{
			gl::DeleteProgram(self.id);
		}
	}
}

pub struct Shader
{
	id: gl::types::GLuint,
}

impl Shader
{
	pub fn from_source(source: &CStr, kind: gl::types::GLenum ) -> Result<Shader, String>
	{
		let id = shader_from_source(source, kind)?;
		Ok(Shader { id })
	}

	pub fn from_vert_source(source: &CStr, name: &String) -> Result<Shader, String>
	{
		match Shader::from_source(source, gl::VERTEX_SHADER)
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
		match Shader::from_source(source, gl::FRAGMENT_SHADER)
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
		match Shader::from_source(source, gl::COMPUTE_SHADER)
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


	pub fn id(&self) -> gl::types::GLuint
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
			gl::DeleteShader(self.id);
		}
	}
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum )
	-> Result<gl::types::GLuint, String>
{
	let id = unsafe { gl::CreateShader(kind) };
	unsafe
	{
		gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
		gl::CompileShader(id);
	}

	let mut success: gl::types::GLint = 1;
	unsafe
	{
		gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
	}

	if success == 0
	{
		let mut len: gl::types::GLint = 0;
		unsafe
		{
			gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
		}

		let error = create_whitespace_cstring_with_len(len as usize);

		unsafe
		{
			gl::GetShaderInfoLog(
				id,
				len,
				std::ptr::null_mut(),
				error.as_ptr() as *mut gl::types::GLchar
			);
		}

		return Err(error.to_string_lossy().into_owned());
	}

	Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize)
	-> CString
{
	// allocate buffer of correct size
	let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
	// fill it with len spaces
	buffer.extend([b' '].iter().cycle().take(len));
	// convert buffer to CString
	unsafe { CString::from_vec_unchecked(buffer) }
}


pub struct Texture
{
	pub handle: gl::types::GLuint,
	pub width: i32,
	pub height: i32,
	pub texture_type: gl::types::GLenum,
	pub pixel_type: gl::types::GLenum
}

impl Texture
{
	fn generate_handle(width: i32, height: i32, texture_type: gl::types::GLenum, pixel_type: gl::types::GLenum) -> gl::types::GLuint
	{
		let mut handle: gl::types::GLuint = 0;
		unsafe
		{
			gl::CreateTextures(texture_type, 1, &mut handle);
			gl::TextureStorage2D(handle, 1, pixel_type, width, height);
			//gl::TextureSubImage2D(handle, 0, 0, 0, width, height, gl::BGRA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::types::GLvoid);

			//gl::GenTextures(1, &mut handle);
			//gl::BindTexture(texture_type, handle);
			//gl::TexStorage2D(texture_type, 1, pixel_type, width, height);
			
			//gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, texture_width, texture_height, gl::BGRA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::types::GLvoid);
		}
		return handle;
	}
	// texture width, height, and type such as GL_TEXTURE_2D (gl::TEXTURE2D), pixel_type such as gl::RGBA8
	pub fn new_texture(width: i32, height: i32, texture_type: gl::types::GLenum, pixel_type: gl::types::GLenum) -> Self
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
				gl::DeleteTextures(1, &self.handle);
			}
			self.handle = 0;
		}
	}
	pub fn get_handle(&self) -> gl::types::GLuint
	{
		return self.handle;
	}

	pub fn get_pixel_type(&self) -> gl::types::GLenum
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
	handle: gl::types::GLuint,
	buffer_type: gl::types::GLenum,
	size: usize
}

impl ShaderBuffer
{
	pub fn new_with_data(buffer_type: gl::types::GLenum, size: usize, data_ptr: *const gl::types::GLvoid) -> Self
	{
		let mut tmp_handle: gl::types::GLuint = 0;
		unsafe
		{
			gl::CreateBuffers(1, &mut tmp_handle);
			gl::NamedBufferData(tmp_handle, size as gl::types::GLsizeiptr, data_ptr, gl::DYNAMIC_COPY);

			//gl::GenBuffers(1, &mut tmp_handle);
			//gl::BindBuffer(buffer_type, tmp_handle);
			//gl::BufferData(
			//	buffer_type,
			//	size as gl::types::GLsizeiptr,
			//	data_ptr,
			//	gl::DYNAMIC_COPY, // usage
			//);
			//gl::BindBuffer(buffer_type, 0);

		}

		Self
		{
			handle: tmp_handle, buffer_type, size
		}
	}

	pub fn new(buffer_type: gl::types::GLenum, size: usize) -> Self
	{
		Self::new_with_data(buffer_type, size, 0 as *const gl::types::GLvoid)
	}
	pub fn bind(&self, slot :u32)
	{
		unsafe
		{
			gl::BindBufferBase(self.buffer_type, slot, self.handle);
		}
	}

	pub fn write_data(&self, offset_in_bytes: usize, size: usize, ptr: *const gl::types::GLvoid)
	{
		unsafe
		{
			gl::NamedBufferSubData(self.handle, offset_in_bytes as gl::types::GLintptr, size as gl::types::GLintptr, ptr);
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
				gl::DeleteBuffers(1, &self.handle);
			}
			self.handle = 0;
		}
	}
}
