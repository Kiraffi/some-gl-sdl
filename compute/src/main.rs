
extern crate gl;
extern crate core;

//use render_gl::*;

use std::ffi::CString;

pub struct ShaderData
{
	_roll_x: f32,
	_roll_y: f32,
	_screen_size_x: i32,
	_screen_size_y: i32
}



fn run(app: &mut sdl_window::App) -> Result<(), String>
{
	render_gl::init_gl(app.window_width, app.window_height)?;

	let vert_shader = render_gl::Shader::from_vert_source(
		&CString::new(include_str!("fullscreen_quad.vert")).unwrap(), &"fullscreen_quad.vert".to_string()
	)?;

	let frag_shader = render_gl::Shader::from_frag_source(
		&CString::new(include_str!("fullscreen_quad.frag")).unwrap(), &"fullscreen_quad.frag".to_string()
	)?;


	let shader_program = render_gl::Program::from_shaders(
		&[vert_shader, frag_shader]
	).unwrap();

	shader_program.set_used();


	let comp_shader = render_gl::Shader::from_comp_source(
		&CString::new(include_str!("compute.comp")).unwrap(), &"compute.comp".to_string()
	)?;

	let compute_shader_program = render_gl::Program::from_shaders(
		&[comp_shader]
	).unwrap();

	shader_program.set_used();


	let tex = render_gl::Texture::new_texture(app.window_width, app.window_height, gl::TEXTURE_2D, gl::RGBA8);

	let mut shader_data: Vec<ShaderData> = vec![ShaderData{	_roll_x: 0.0f32,
		_roll_y: 0.0f32,
		_screen_size_x: app.window_width,
		_screen_size_y: app.window_height
	}];

	let ssbo: render_gl::ShaderBuffer = render_gl::ShaderBuffer::new_with_data(
//		gl::SHADER_STORAGE_BUFFER,
		gl::UNIFORM_BUFFER,
		std::mem::size_of::<ShaderData>(),
		shader_data.as_ptr() as *const gl::types::GLvoid
	);

	let mut vao: gl::types::GLuint = 0;
	unsafe
	{
		gl::GenVertexArrays(1, &mut vao);
	}

	let mut roll = 0.0f32;

	while !app.quit
	{
		app.update();
		if app.resized
		{
			render_gl::resize(app.window_width, app.window_height);
			app.resized = false;
		}

		roll += app.timer.dt as f32;

		shader_data[0]._roll_x = roll;
		shader_data[0]._screen_size_x = app.window_width;
		shader_data[0]._screen_size_y = app.window_height;

		ssbo.write_data(0, ssbo.get_size(), shader_data.as_ptr() as *const gl::types::GLvoid);


		shader_program.set_used();
		unsafe
		{
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
			gl::DepthFunc(gl::LESS);
			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::ALWAYS);

			compute_shader_program.set_used();
			ssbo.bind(0);

			gl::BindImageTexture(0, tex.get_handle(), 0, gl::FALSE, 0, gl::WRITE_ONLY, tex.get_pixel_type());
			gl::DispatchCompute(((app.window_width + 7) / 8) as u32, ((app.window_height + 7) / 8) as u32, 1);
			//gl::MemoryBarrier(gl::TEXTURE_FETCH_BARRIER_BIT);
			// prevent sampling before all writing to image are done
			gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
			shader_program.set_used();

			gl::BindVertexArray(vao);


			gl::BindTexture(gl::TEXTURE_2D, tex.get_handle());
			gl::DrawArrays(
				gl::TRIANGLES, // mode
				0, // starting index in the enabled arrays
				6 // number of indices to be rendered
			);

		}
		app.swap_buffer();
		::std::thread::sleep(std::time::Duration::from_millis(1));
	}
	return Ok(());
}


fn main()
{
	{
		let mut app;
		match sdl_window::App::init(800, 600, "Compute test", true)
		{
			Ok(v) =>
			{
				app = v;
				//match app.run(&"new_font.dat".to_string())
				match run(&mut app)
				{
					Ok(_) =>
					{

					}
					Err(f) =>
					{
						println!("Runtime error: {}", f);
						//panic!(f);
					}
				}
			}
			Err(e) =>
			{
				println!("Error: {}", e);
				//panic!(e);
			}
		}
	}
}
