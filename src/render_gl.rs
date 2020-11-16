use gl;
use std;
use std::ffi::{CString, CStr};

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

	pub fn from_vert_source(source: &CStr) -> Result<Shader, String>
	{
		Shader::from_source(source, gl::VERTEX_SHADER)
	}

	pub fn from_frag_source(source: &CStr) -> Result<Shader, String>
	{
		Shader::from_source(source, gl::FRAGMENT_SHADER)
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
			gl::GenBuffers(1, &mut tmp_handle);
			gl::BindBuffer(buffer_type, tmp_handle);
			gl::BufferData(
				buffer_type,
				size as gl::types::GLsizeiptr,
				data_ptr,
				gl::DYNAMIC_COPY, // usage
			);

			gl::BindBuffer(buffer_type, 0);
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
