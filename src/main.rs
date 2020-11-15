extern crate sdl2;
extern crate gl;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::ffi::CString;
use std::ffi::CStr;

pub mod render_gl;

pub struct ShaderData
{
	_pos1: f32,
	_pos2: f32,
	_col: f32,
	_size: f32
}

impl ShaderData
{
	pub fn new(_pos1: f32, _pos2: f32, _col: f32, _size: f32) -> Self { Self { _pos1, _pos2, _col, _size } }
}


fn main()
{
	println!("Hello, world!");
	let mut window_width: i32 = 800;
	let mut window_height: i32 = 600;
	let enable_vsync: bool = true;

	let box_size = 20;


	let _sdl: sdl2::Sdl  = sdl2::init().unwrap();
	let _video: sdl2::VideoSubsystem = _sdl.video().unwrap();
	let _sdl_timer: sdl2::TimerSubsystem = _sdl.timer().unwrap();
	let _window = _video.window("Game", window_width as u32, window_height as u32)
		.resizable()
		.opengl()
		.build()
		.unwrap();

	let gl_attr = _video.gl_attr();

	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(4, 5);

	let mut _event_pump = _sdl.event_pump().unwrap();

	let _gl_context = _window.gl_create_context().unwrap();
	let _gl = gl::load_with(|s| _video.gl_get_proc_address(s) as *const std::os::raw::c_void);


	let version = unsafe
	{
		let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
			.to_bytes()
			.to_vec();
		String::from_utf8(data).unwrap()
	};

	println!("OpenGL version {}", version);


	unsafe
	{
		gl::Viewport(0, 0, window_width, window_height); // set viewport
		gl::ClearColor(0.0, 0.0, 0.0, 1.0);
		// Swapping up and down just messes things up like in renderdoc....
		//gl::ClipControl(gl::UPPER_LEFT, gl::ZERO_TO_ONE);
		gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
	}

	if enable_vsync
	{
		_video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync).unwrap();
	}
	else
	{
		_video.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate).unwrap();
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
/*
	let vertices: Vec<f32> = vec![
		// positions	  // colors
		-0.5, -0.5, 1.0,  1.0, 0.0, 0.0, // bottom right
		 0.5, -0.5, 1.0,  0.0, 1.0, 0.0, // bottom left
		 0.5,  0.5, 1.0,  0.0, 0.0, 1.0, // top
		-0.5,  0.5, 1.0,  0.0, 0.0, 1.0  // top
	];

	let indices: Vec<gl::types::GLuint> = vec![
		0, 1, 2,
		2, 3, 0
	];
*/
	let mut shader_data: Vec<ShaderData> = Vec::new();

	for y in 0..25
	{
		for x in 0..35
		{
			shader_data.push(ShaderData::new((x * box_size) as f32, (y * box_size) as f32, 0.0f32, box_size as f32 ));
		}
	}
/*
	let mut vbo: gl::types::GLuint = 0;
	unsafe
	{
		gl::GenBuffers(1, &mut vbo);
	}
	unsafe
	{
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER, // target
			(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
			vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
			gl::STATIC_DRAW, // usage
		);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
	}
	let mut indexbo: gl::types::GLuint = 0;
	unsafe
	{
		gl::GenBuffers(1, &mut indexbo);
	}
	unsafe
	{
		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indexbo);
		gl::BufferData(
			gl::ELEMENT_ARRAY_BUFFER, // target
			(indices.len() * std::mem::size_of::<gl::types::GLuint>()) as gl::types::GLsizeiptr, // size of data in bytes
			indices.as_ptr() as *const gl::types::GLvoid, // pointer to data
			gl::STATIC_DRAW, // usage
		);
		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); // unbind the buffer
	}
*/
	let mut ssbo: gl::types::GLuint = 0;

	unsafe
	{
		gl::GenBuffers(1, &mut ssbo);
		//let memsize = std::mem::size_of::<ShaderData>() * shader_data.len();
		gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
		gl::BufferData(
			gl::SHADER_STORAGE_BUFFER, // target
			(shader_data.len() * std::mem::size_of::<ShaderData>()) as gl::types::GLsizeiptr, // size of data in bytes
			shader_data.as_ptr() as *const gl::types::GLvoid, // pointer to data
			gl::DYNAMIC_COPY, // usage
		);

		//gl::BufferData(gl::SHADER_STORAGE_BUFFER, memsize as gl::types::GLsizeiptr, shader_data.as_ptr() as *const gl::types::GLvoid, gl::DYNAMIC_COPY);
		gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
	}

	let mut vao: gl::types::GLuint = 0;
	unsafe
	{
		gl::GenVertexArrays(1, &mut vao);
	}
/*
	unsafe
	{
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
*/
	let mut pos: f32 = 0.0f32;
	let mut dir: f32 = 1.0f32;
	let mut spd: f32 = 1.0f32;

	let mut now_stamp: u64 = _sdl_timer.performance_counter();
	let mut last_stamp: u64;
	let perf_freq: f64 = _sdl_timer.performance_frequency() as f64;
	let mut dt: f32;
	'running: loop
	{
		last_stamp = now_stamp;
		now_stamp = _sdl_timer.performance_counter();
		dt = ((now_stamp - last_stamp) as f64 * 1000.0f64 / perf_freq ) as f32;

		for event in _event_pump.poll_iter()
		{
			match event
			{
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
				{
					break 'running
				},
				Event::KeyDown { keycode: Some(Keycode::Q), .. } =>
				{
					println!("Q pressed\n");
				},
				Event::KeyUp { keycode: Some(Keycode::Q), .. } =>
				{
					println!("Q released\n");
				},
				Event::Window {win_event, ..  } =>
				{
					match win_event
					{
						sdl2::event::WindowEvent::Resized( width, height ) =>
						{
							window_width = width;
							window_height = height;
							println!("Resized: {}: {}", window_width, window_height);
							unsafe
							{
								gl::Viewport(0, 0, window_width, window_height); // set viewport
							}

						},

						_ => {}
					}
				},


				_ => {}
			}
		}
		unsafe
		{
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
			gl::DepthFunc(gl::LESS);
			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::ALWAYS);
		}

		shader_program.set_used();
		unsafe
		{
			let tmp_pos: f32 = pos / (window_width as f32);
			gl::Uniform4f(0, tmp_pos, box_size as f32, window_width as f32, window_height as f32);

			gl::BindVertexArray(vao);
			//gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indexbo);


			//gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, block_index, ssbo);
			gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 2, ssbo);
			/*
			gl::BindBufferRange(gl::SHADER_STORAGE_BUFFER,
				2,
				ssbo,
				0 as gl::types::GLintptr,
				(shader_data.len() * std::mem::size_of::<ShaderData>()) as gl::types::GLsizeiptr);

				//offset: types::GLintptr, size: types::GLsizeiptr

			*/
			gl::DrawArrays(
				gl::TRIANGLES, // mode
				0, // starting index in the enabled arrays
				6 * shader_data.len() as i32 // number of indices to be rendered
			);

			//gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
		}
		::std::thread::sleep(std::time::Duration::from_millis(1));
		_window.gl_swap_window();

		pos += dir * spd;
		if pos > 100.0f32
		{
			dir = -1.0f32;

		}
		else if pos < -100.0f32
		{
			dir = 1.0f32;
		}
		spd = 20.0f32;

		println!("Frame duration: {}", dt);
	}
}
