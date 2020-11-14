extern crate sdl2;
extern crate gl;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::ffi::CString;


pub mod render_gl;


fn main()
{
	println!("Hello, world!");

	let _sdl = sdl2::init().unwrap();
	let _video = _sdl.video().unwrap();
	let _window = _video.window("Game", 800, 600)
		.resizable()
		.opengl()
		.build()
		.unwrap();


	let _gl_attr = _video.gl_attr();

	_gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	_gl_attr.set_context_version(4, 5);

	let mut _event_pump = _sdl.event_pump().unwrap();

	let _gl_context = _window.gl_create_context().unwrap();
	let _gl = gl::load_with(|s| _video.gl_get_proc_address(s) as *const std::os::raw::c_void);

	unsafe
	{
		gl::Viewport(0, 0, 800, 600); // set viewport
		gl::ClearColor(0.3, 0.3, 0.5, 1.0);
	}



	let vert_shader = render_gl::Shader::from_vert_source(
		&CString::new(include_str!("triangle.vert")).unwrap()
	).unwrap();

	let frag_shader = render_gl::Shader::from_frag_source(
		&CString::new(include_str!("triangle.frag")).unwrap()
	).unwrap();


	let shader_program = render_gl::Program::from_shaders(
		&[vert_shader, frag_shader]
	).unwrap();

	shader_program.set_used();

	let vertices: Vec<f32> = vec![
		// positions	  // colors
		0.5, -0.5, 0.0,   1.0, 0.0, 0.0,   // bottom right
		-0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
		0.0,  0.5, 0.0,   0.0, 0.0, 1.0	// top
	];



	let mut vbo: gl::types::GLuint = 0;
	unsafe {
		gl::GenBuffers(1, &mut vbo);
	}
	unsafe {
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER, // target
			(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
			vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
			gl::STATIC_DRAW, // usage
		);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
	}


	let mut vao: gl::types::GLuint = 0;
	unsafe {
		gl::GenVertexArrays(1, &mut vao);
	}

	unsafe {
		gl::BindVertexArray(vao);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
		gl::VertexAttribPointer(
			0, // index of the generic vertex attribute ("layout (location = 0)")
			3, // the number of components per generic vertex attribute
			gl::FLOAT, // data type
			gl::FALSE, // normalized (int-to-float conversion)
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
			std::ptr::null() // offset of the first component
		);
		gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
		gl::VertexAttribPointer(
			1, // index of the generic vertex attribute ("layout (location = 0)")
			3, // the number of components per generic vertex attribute
			gl::FLOAT, // data type
			gl::FALSE, // normalized (int-to-float conversion)
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
			(3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
		);

		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);
	}




	'running: loop
	{
		for event in _event_pump.poll_iter()
		{
			match event
			{
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
		}
		unsafe
		{
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		shader_program.set_used();
		unsafe {
			gl::BindVertexArray(vao);
			gl::DrawArrays(
				gl::TRIANGLES, // mode
				0, // starting index in the enabled arrays
				3 // number of indices to be rendered
			);
		}
		_window.gl_swap_window();
	}
}





