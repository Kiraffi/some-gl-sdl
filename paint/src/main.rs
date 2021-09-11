extern crate sdl2;
extern crate gl;

extern crate render_gl;
extern crate core;
extern crate sdl_window;

use sdl_window::*;

use std::ffi::CString;

use std::intrinsics::transmute;
use std::io::Write;
use std::io::Read;

pub struct ShaderData
{
	_pos_x: f32,
	_pos_y: f32,
	_col: u32,
	_size: f32
}

fn load_file(name: &String) -> Result<Vec<u8>, String>
{
	let mut file;
	match std::fs::File::open(name)
	{
		Ok(x) =>
		{
			file = x;
		},
		Err(e) =>
		{
			println!("Failed to open file: {}, error: {}", name, e);
			return Err("Failed to open file!".to_string());
		}
	}

	let mut contents: Vec<u8> = Vec::new();
	match file.read_to_end(&mut contents)
	{
		Ok(_s) => { println!("read: {}", _s);},
		Err(e) =>
		{
			println!("Failed to read file: {}, error: {}", name, e);
			return Err("Failed to read file!".to_string());
		}
	}
	return Ok(contents);
}


fn save_file(name: &String, letters: &Vec<u8>) -> Result<(), String>
{
	let mut file;
	match std::fs::File::create(name)
	{
		Ok(x) => { file = x; }
		Err(e) =>
		{
			println!("Failed to create file: {}, error: {}", name, e);
			return Err("Failed to create file".to_string());
		}
	}
	match file.write_all(&letters)
	{
		Ok(()) => {}
		Err(e) =>
		{
			println!("Failed to write to file: {}, error: {}", name, e);
			return Err("Failed to write to file".to_string());
		}
	}
	return Ok(());
}

fn run(app: &mut sdl_window::App, file_name: &String) -> Result<(), String>
{
	render_gl::init_gl(&app.window_state)?;

	let box_size = 30;

	let vert_shader = render_gl::Shader::from_vert_source(
		&CString::new(include_str!("triangle.vert")).unwrap(), &"triangle.vert".to_string()
	)?;

	let frag_shader = render_gl::Shader::from_frag_source(
		&CString::new(include_str!("triangle.frag")).unwrap(), &"triangle.frag".to_string()
	)?;


	let shader_program = render_gl::Program::from_shaders(
		&[vert_shader, frag_shader]
	).unwrap();

	shader_program.set_used();

	let colors: [u32;2] = [
		core::get_u32_agbr_color(0.0, 0.0, 0.0, 1.0),
		core::get_u32_agbr_color(1.0, 1.0, 1.0, 1.0),
	];

	let board_size_x = 8;
	let board_size_y = 12;

	/*
	let mut board: Vec<u8> = Vec::new();
	for _ in 0.. (board_size_x * board_size_y)
	{
		board.push(0);
	}
	*/
	let start_x: f32 = (-(board_size_x as f32) / 2.0f32 + 0.5f32) * box_size as f32;
	let start_y: f32 = (-(board_size_y as f32) / 2.0f32 + 0.5f32) * box_size as f32;

	let start_x_px = ((app.window_state.window_width - board_size_x * box_size) / 2) as i32;
	let start_y_px = ((app.window_state.window_height - board_size_y * box_size) / 2) as i32;
	let end_x_px = start_x_px + (board_size_x * box_size) as i32;
	let end_y_px = start_y_px + (board_size_y * box_size) as i32;

	let mut letters: Vec<u8> = Vec::new();
	{
		for __letter_count in 32..128
		{
			for _letter_height in 0..board_size_y
			{
				letters.push(0);
			}
		}
	}
	let mut letter_index = ('a' as u8) - 32;

	let mut shader_data: Vec<ShaderData> = Vec::new();
	{

		let col = colors[0];
		for y in 0..board_size_y
		{
			for x in 0..board_size_x
			{
				let pos_x = start_x + (x * box_size) as f32;
				let pos_y = start_y + (y * box_size) as f32;

				shader_data.push(ShaderData{_pos_x: pos_x, _pos_y: pos_y, _col: col, _size: box_size as f32});
			}
		}

		let mut letter_row = 0;
		let mut letter_col = 0;
		for _ in 0.. 128
		{
			if letter_col == 16
			{
				letter_row += 1;
				letter_col = 0;
			}
			for y in 0..board_size_y
			{
				for x in 0..board_size_x
				{
					let pos_x = -(app.window_state.window_width / 2) + 10 + letter_col * (board_size_x + 1) + x;
					let pos_y = -(app.window_state.window_height / 2) + 10 + letter_row * (board_size_y + 1) + y;

					shader_data.push(ShaderData{_pos_x: pos_x as f32, _pos_y: pos_y as f32, _col: col, _size: 1.0f32});
				}
			}
			letter_col += 1;
		}
	}

	match load_file(file_name)
	{
		Ok(x)=>
		{
			letters = x;
		},
		// should this be handled some way differently than just saying error happened?
		Err(_e) =>
		{
		}
	}


	let ssbo: render_gl::ShaderBuffer = render_gl::ShaderBuffer::new_with_data(
		gl::SHADER_STORAGE_BUFFER,
//			gl::UNIFORM_BUFFER,
		shader_data.len() * std::mem::size_of::<ShaderData>(),
		shader_data.as_ptr() as *const gl::types::GLvoid
	);

	let mut vao: gl::types::GLuint = 0;
	unsafe
	{
		gl::GenVertexArrays(1, &mut vao);
	}

	while !app.window_state.quit
	{

		app.update();
		render_gl::update(&mut app.window_state);

		let ctrl_down = app.is_down(MyKey::LCtrl) || app.is_down(MyKey::RCtrl);
		if app.was_pressed(MyKey::S) && ctrl_down
		{
			save_file(file_name, &letters)?;
		}

		else if app.was_pressed(MyKey::L) && ctrl_down
		{
			match load_file(file_name)
			{
				Ok(x)=>
				{
					letters = x;
				},
				// should this be handled some way differently than just saying error happened?
				Err(_e) =>
				{
				}
			}
		}
		else 
		{
			for x in 32i32..128i32 
			{
				let key: MyKey = unsafe { transmute(x) };
				if app.was_pressed(key)
				{
					letter_index = (x - 32) as u8;
				}
			}
		}
		if app.window_state.mouse_b != 0
		{
			if app.window_state.mouse_x >= start_x_px && app.window_state.mouse_y >= start_y_px
				&& app.window_state.mouse_x < end_x_px && app.window_state.mouse_y < end_y_px
			{
				let p_x = app.window_state.mouse_x - start_x_px;
				let p_y = app.window_state.mouse_y - start_y_px;

				let x_b = p_x / (box_size as i32);
				let y_b = p_y / (box_size as i32);

				let index = x_b + y_b * (board_size_x as i32);
				let byte_index: u8 = 1 << x_b as u8;
				let letters_index: usize = (y_b + (letter_index as i32) * board_size_y) as usize;
				if app.window_state.mouse_b == 1 && index >= 0 && index < (board_size_x * board_size_y) as i32
				{
					letters[letters_index] |= byte_index;
				}
				else if app.window_state.mouse_b == 2 && index >= 0 && index < (board_size_x * board_size_y) as i32
				{
					letters[letters_index] &= !byte_index;
				}
			}
		}

		// Write all the tiles into color from background.
		for y in 0..board_size_y
		{
			for x in 0..board_size_x
			{
				let index = (y + (letter_index as i32) * board_size_y) as usize;
				let shader_index = (x + y * board_size_x) as usize;

				shader_data[shader_index]._col = colors[((letters[index] >> x) & 1) as usize];
			}
		}

		for letter_c in 0..128 - 32
		{
			for y in 0..board_size_y
			{
				for x in 0..board_size_x
				{
					let shader_index = (x + y * board_size_x + (letter_c + 1) * board_size_x * board_size_y) as usize;
					let index = (y + letter_c * board_size_y) as usize;

					shader_data[shader_index]._col = colors[((letters[index] >> x) & 1) as usize];
				}
			}
		}

		ssbo.write_data(0, ssbo.get_size(), shader_data.as_ptr() as *const gl::types::GLvoid);




		shader_program.set_used();
		unsafe
		{
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
			gl::DepthFunc(gl::LESS);
			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::ALWAYS);

			gl::Uniform4f(0, 0.0f32, box_size as f32, app.window_state.window_width as f32, app.window_state.window_height as f32);

			gl::BindVertexArray(vao);

			ssbo.bind(2);

			gl::DrawArrays(
				gl::TRIANGLES, // mode
				0, // starting index in the enabled arrays
				6 * shader_data.len() as i32 // number of indices to be rendered
			);

			//gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
		}
		app.swap_buffer();
		::std::thread::sleep(std::time::Duration::from_millis(1));
		//println!("x: {}, y: {}", pos_x, pos_y);
		//println!("Frame duration: {}", _dt);
	}
	return Ok(());
}


fn main()
{
	use std::env;

	println!("Hello, world!");
	let filename: String;
	let args: Vec<_> = env::args().collect();
	
	for i in 1 .. args.len()
	{
		println!("{}: {}", i, args[i as usize]);
		
	}
	if args.len() < 2
	{

		println!("Using default filename, new_font.dat!");
		filename = "new_font.dat".to_string();
	}
	else
	{
		filename = args[1].clone();
	}
	

	{
		let mut app;
		match sdl_window::App::init(800, 600, "Paint", true)
		{
			Ok(v) =>
			{
				app = v;
				//match app.run(&"new_font.dat".to_string())
				match run(&mut app, &filename)
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
