extern crate sdl2;
extern crate gl;

extern crate render_gl;
extern crate core;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::mouse::MouseButton;

use std::ffi::CString;

pub struct ShaderData
{
	_roll_x: f32,
	_roll_y: f32,
	_screen_size_x: i32,
	_screen_size_y: i32
}



fn resize_window(width:i32, height:i32, tex: &mut render_gl::Texture)
{
	println!("Resized: {}: {}", width, height);
	unsafe
	{
		gl::Viewport(0, 0, width, height);
	}
	tex.resize(width, height);
}

fn run(app: &mut core::App) -> Result<(), String>
{

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


	let mut tex = render_gl::Texture::new_texture(app.window_width, app.window_height, gl::TEXTURE_2D, gl::RGBA8);

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

	let mut now_stamp: u64 = app.sdl_timer.performance_counter();
	let mut last_stamp: u64;
	let perf_freq: f64 = app.sdl_timer.performance_frequency() as f64;
	let mut _dt: f32;

	let mut roll = 0.0f32;

	loop
	{

		last_stamp = now_stamp;
		now_stamp = app.sdl_timer.performance_counter();
		_dt = ((now_stamp - last_stamp) as f64 * 1000.0f64 / perf_freq ) as f32;
		roll += _dt;

		for event in app.event_pump.poll_iter()
		{
			match event
			{
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
				{
					return Ok(());
				},
/*
				Event::KeyDown { keycode, keymod, .. } =>
				{
					match keycode
					{
						Some(keyocde_value) =>
						{

						},
						None => {}
					}


				},

				Event::MouseButtonDown { mouse_btn, x, y, .. } =>
				{

				},
				Event::MouseButtonUp { mouse_btn, x, y, .. } =>
				{

				},
				Event::MouseMotion { x, y, .. } =>
				{

				},
*/
				Event::Window {win_event, ..  } =>
				{
					match win_event
					{
						sdl2::event::WindowEvent::Resized( width, height ) =>
						{
							app.window_width = width;
							app.window_height = height;
							resize_window(width, height, &mut tex);
						},

						_ => {}
					}
				},
				_ => {}
			}
		}

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
		::std::thread::sleep(std::time::Duration::from_millis(1));
		app.window.gl_swap_window();
	}
	//return Ok(());
}


fn main()
{
	{
		let mut app;
		match core::App::init(800, 600, "Compute test", true)
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
