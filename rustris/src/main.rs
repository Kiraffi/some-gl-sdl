extern crate sdl2;
extern crate gl;

extern crate render_gl;
extern crate core;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::ffi::CString;

//pub mod render_gl;

struct Randomm
{
	rng_seed: u64,
	rng_inc: u64
}

pub struct Block
{
	blocks: [u8; 8],
}

#[derive(Clone,Copy)]
pub struct BlockPiece
{
	pos_x: i32,
	pos_y: i32,
	block_type: u8,
	rotation: u8
}

pub struct GameState
{
	player: BlockPiece,
	score: u32,
	high_score: u32,
	lines: u32,
	last_row_down: u64,
	rng: Randomm
}
pub struct Board
{
	size_x: i32,
	size_y: i32,
	extra_top: i32,
	board: Vec<u8>
}


pub struct ShaderData
{
	_pos_x: f32,
	_pos_y: f32,
	_col: u32,
	_size: f32
}


pub struct LetterData
{
	_pos_x: f32,
	_pos_y: f32,
	_col: u32,
	_size: f32,

	_uv_x: f32,
	_uv_y: f32,
	_tmp1: f32,
	_tmp2: f32
}

const BLOCKS: [Block; 7] = [
	Block{ blocks: [1,1,1,1, 0,0,0,0]},
	Block{ blocks: [0,1,1,1, 0,0,0,1]},
	Block{ blocks: [0,1,1,1, 0,1,0,0]},

	Block{ blocks: [0,1,1,1, 0,0,1,0]},
	Block{ blocks: [0,1,1,0, 0,1,1,0]},
	Block{ blocks: [0,1,1,0, 0,0,1,1]},
	Block{ blocks: [0,0,1,1, 0,1,1,0]}
];

impl Randomm
{
	// From https://www.pcg-random.org/download.html
	pub fn get_next(&mut self) -> u32
	{
		let old: u64 = self.rng_seed;
		self.rng_seed = self.rng_seed.wrapping_mul(636413622384679005u64);
		self.rng_seed = self.rng_seed.wrapping_add(self.rng_inc | 1u64);
		let xor_shifted: u32  = (((old >> 18u64) ^ old) >> 27u64) as u32;
		let rot: u32 = (old >> 59u64) as u32;
		let result: u32 = (xor_shifted >> rot) | (xor_shifted << ((!rot) & 31));
		return result;
	}
}



impl Board
{
	pub fn new(size_x: i32, size_y: i32, extra: i32) -> Self
	{
		let mut b: Vec<u8> = Vec::new();
		for _x in 0..size_x * (size_y + extra)
		{
			b.push(0);
		}

		Self { size_x, size_y, extra_top: extra, board: b }
	}

	pub fn check_hit(&self, piece: &BlockPiece) -> bool
	{
		for y in 0..2i32
		{
			for x in 0..4i32
			{
				if BLOCKS[piece.block_type as usize].blocks[(x + y * 4) as usize] == 1
				{
					let (mut x_pos, mut y_pos) = get_rotated(x, y, piece.rotation);
					x_pos += piece.pos_x;
					y_pos += piece.pos_y;
					if x_pos >= self.size_x || y_pos >= self.size_y + self.extra_top || x_pos < 0 || y_pos < 0
					{
						return true;
					}
					if self.board[(x_pos + y_pos * self.size_x) as usize] != 0
					{
						return true;
					}
				}
			}
		}
		return false;
	}

	pub fn add_piece(&mut self, piece: &BlockPiece) -> bool
	{
		let mut game_over = false;
		for y in 0..2i32
		{
			for x in 0..4i32
			{
				if BLOCKS[piece.block_type as usize].blocks[(x + y * 4) as usize] == 1
				{
					let (mut x_pos, mut y_pos) = get_rotated(x, y, piece.rotation);
					x_pos += piece.pos_x;
					y_pos += piece.pos_y;
					if x_pos < self.size_x && y_pos < self.size_y + self.extra_top && x_pos >= 0 && y_pos >= 0
					{
						self.board[(x_pos + y_pos * self.size_x) as usize] = piece.block_type + 1;
					}
					if y_pos >= self.size_y
					{
						game_over = true;
					}
				}
			}
		}
		return game_over;
	}

	pub fn check_left_border(&mut self, piece: &BlockPiece) -> bool
	{
		for y in 0..2i32
		{
			for x in 0..4i32
			{
				if BLOCKS[piece.block_type as usize].blocks[(x + y * 4) as usize] == 1
				{
					let (mut x_pos, _) = get_rotated(x, y, piece.rotation);
					x_pos += piece.pos_x;
					if x_pos < 0
					{
						return true;
					}
				}
			}
		}
		return false;
	}

	pub fn check_right_border(&mut self, piece: &BlockPiece) -> bool
	{
		for y in 0..2i32
		{
			for x in 0..4i32
			{
				if BLOCKS[piece.block_type as usize].blocks[(x + y * 4) as usize] == 1
				{
					let (mut x_pos, _) = get_rotated(x, y, piece.rotation);
					x_pos += piece.pos_x;
					if x_pos >= self.size_x
					{
						return true;
					}
				}
			}
		}
		return false;
	}

	pub fn check_row(&self, row_number: i32) -> bool
	{
		if row_number < 0 || row_number >= self.size_y
		{
			return false;
		}

		for x in 0.. self.size_x
		{
			if self.board[(x + row_number * self.size_x) as usize] == 0
			{
				return false;
			}
		}
		return true;
	}

	pub fn clear_board(&mut self)
	{
		for i in 0..self.size_x * (self.size_y + self.extra_top)
		{
			self.board[i as usize] = 0;
		}
	}

	pub fn remove_row(&mut self, row_number: i32)
	{
		if row_number < 0 || row_number >= self.size_y
		{
			return;
		}

		for y in row_number..self.size_y - 1
		{
			for x in 0.. self.size_x
			{
				self.board[(x + y * self.size_x) as usize] = self.board[(x + (y + 1) * self.size_x) as usize];
			}
		}
		for x in 0.. self.size_x
		{
			self.board[(x + (self.size_y - 1) * self.size_x) as usize] = 0;
		}
	}
}

impl BlockPiece
{
	// Try to make the rotation of block good?
	pub fn set_rotation(&mut self, rotation: u8)
	{
		let rotation = rotation % 4;
		let mut min_x = 4;
		let mut max_x = 0;
		let mut curr_min_x = 4;
		let mut curr_max_x = 0;
		let mut min_y = 4;
		let mut curr_min_y = 4;
		for y in 0..2i32
		{
			for x in 0..4i32
			{
				if BLOCKS[self.block_type as usize].blocks[(x + y * 4) as usize] == 1
				{
					let (xx, yy) = get_rotated(x, y, rotation);
					if yy == min_y
					{
						min_x = min_x.min(xx);
						max_x = max_x.max(xx);
						min_y = min_y.min(yy);
					}

					else if yy < min_y
					{
						min_x = xx;
						max_x = xx;
						min_y = yy;
					}

					let (xx2, yy2) = get_rotated(x, y, self.rotation);

					if yy2 == curr_min_y
					{
						curr_min_x = curr_min_x.min(xx2);
						curr_max_x = curr_max_x.max(xx2);
						curr_min_y = curr_min_y.min(yy2);
					}

					else if yy2 < curr_min_y
					{
						curr_min_x = xx2;
						curr_max_x = xx2;
						curr_min_y = yy2;
					}

				}
			}
		}

		let diff = (max_x - min_x) / 2 - (curr_max_x - curr_min_x) / 2;
		let diff_y = min_y - curr_min_y;
		self.pos_x -= diff + min_x - curr_min_x;
		self.pos_y -= diff_y;

		self.rotation = rotation;
	}
}



fn get_rotated(x: i32, y: i32, rotation: u8) -> (i32, i32)
{
	let mut pos_x = x;
	let mut pos_y = y;
	if rotation == 1
	{
		pos_x = 1 - y;
		pos_y = x;
	}
	if rotation == 2
	{
		pos_x = 3 - x;
		pos_y = 1 - y;
	}
	if rotation == 3
	{
		pos_x = y;
		pos_y = 3 - x;
	}
	// Case 0 no changes.
	return (pos_x, pos_y);
}


fn row_down(state: &mut GameState, board: &mut Board, now_stamp : u64) -> bool
{
	let mut tmp = state.player.clone();
	tmp.pos_y -= 1;
	state.last_row_down = now_stamp;

	if board.check_hit(&tmp)
	{
		let game_over = board.add_piece(&state.player);

		state.player.pos_x = 3;
		state.player.pos_y = 20;

		state.player.block_type = (state.rng.get_next() % 7) as u8;

		let mut rows = 0;
		for y in 0..board.size_y
		{
			while board.check_row(y)
			{
				rows += 1;
				board.remove_row(y);
			}
		}
		state.lines += rows;
		if rows == 1
		{
			state.score += 1;
		}
		if rows == 2
		{
			state.score += 3;
		}
		if rows == 3
		{
			state.score += 6;
		}
		if rows == 4
		{
			state.score += 10;
		}

		if game_over
		{
			board.clear_board();
			state.high_score = state.high_score.max(state.score);
			state.score = 0;
			state.lines = 0;
		}
		return false;
	}
	else
	{
		state.player.pos_y -= 1;
		return true;
	}
}


fn add_to_array(app: &core::App, s: &String, pos_x: f32, pos_y: f32, sz: f32, col: u32, letters: &mut Vec<LetterData>)
{
	let mut px = pos_x - app.window_width as f32 / 2.0f32;
	let py = (app.window_height / 2) as f32 - pos_y;
	for x in 0..s.len()
	{
		let l: u8 = s.as_bytes()[x] - 32;
		let tmp_pos_x = l as f32;// / (128.0f32 - 32.0f32);
		letters.push(LetterData{_pos_x: px, _pos_y: py, _col: col, _size: sz,
			_uv_x: tmp_pos_x, _uv_y: 0.5f32, _tmp1: 0.0f32, _tmp2: 0.0f32});

		px += sz + 1.0f32;
	}
}

fn run(app: &mut core::App) -> Result<(), String>
{
	let box_size = 40;


	let shader_program;
	let shader_textured_program;
	{
		let vert_shader = render_gl::Shader::from_vert_source(
			&CString::new(include_str!("triangle.vert")).unwrap(), &"triangle.vert".to_string()
		)?;
		let frag_shader = render_gl::Shader::from_frag_source(
			&CString::new(include_str!("triangle.frag")).unwrap(), &"triangle.frag".to_string()
		)?;


		let textured_vert_shader = render_gl::Shader::from_vert_source(
			&CString::new(include_str!("textured_triangle.vert")).unwrap(), &"textured_triangle.vert".to_string()
		)?;

		let textured_frag_shader = render_gl::Shader::from_frag_source(
			&CString::new(include_str!("textured_triangle.frag")).unwrap(), &"textured_triangle.frag".to_string()
		)?;

		shader_program = render_gl::Program::from_shaders(
			&[vert_shader, frag_shader]
		).unwrap();

		shader_textured_program = render_gl::Program::from_shaders(
			&[textured_vert_shader, textured_frag_shader]
		).unwrap();
	}


	let tex: Vec<u8> = include_bytes!("../../new_font.dat").to_vec();

	let mut tex_handle: gl::types::GLuint = 0;
	{
		let mut font_tex: Vec<u8> = Vec::new();

		for y in 0..12
		{
			for l in 0 .. (128-32)
			{
				let val = tex[y + l * 12];
				for x in 0..8
				{
					let v = ((val >> x) & 1) * 255;
					font_tex.push(v);
					font_tex.push(v);
					font_tex.push(v);
					font_tex.push(v);
				}
			}
		}

		unsafe
		{
			let texture_width = 8*(128-32);
			let texture_height = 12;

			gl::GenTextures(1, &mut tex_handle);
			gl::BindTexture(gl::TEXTURE_2D, tex_handle);
			gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, texture_width, texture_height);
			gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, texture_width, texture_height, gl::BGRA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::types::GLvoid);

			//gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
			//gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
			//gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, texture_width, texture_height, 0, gl::BGRA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::types::GLvoid);

			//gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			//gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
			//gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8, texture_width, texture_height, 0, gl::RGBA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::types::GLvoid);
			// 0 mips, 0 baselevel

			//gl::GenerateMipmap(gl::TEXTURE_2D);

		}
	}

	shader_program.set_used();

	let mut queries1: gl::types::GLuint = 0;
	let mut queries2: gl::types::GLuint = 0;
	unsafe 
	{
		gl::GenQueries(1, &mut queries1);
		gl::GenQueries(1, &mut queries2);
	}


	let colors: [u32; 9] = [
		// Background color
		core::get_u32_agbr_color(0.0, 0.0, 0.0, 1.0),

		// Block colors
		core::get_u32_agbr_color(1.0, 0.0, 0.0, 1.0),
		core::get_u32_agbr_color(0.0, 1.0, 0.0, 1.0),
		core::get_u32_agbr_color(0.0, 0.0, 1.0, 1.0),
		core::get_u32_agbr_color(1.0, 0.0, 1.0, 1.0),
		core::get_u32_agbr_color(0.0, 1.0, 1.0, 1.0),
		core::get_u32_agbr_color(1.0, 1.0, 0.0, 1.0),
		core::get_u32_agbr_color(1.0, 0.6, 1.0, 1.0),

		// Font color
		core::get_u32_agbr_color(1.0, 1.6, 1.0, 1.0),
	];

	let mut board: Board = Board::new(10, 20, 4);

	// Fill board for shader
	let mut shader_data: Vec<ShaderData> = Vec::new();
	{
		let col = colors[0];
		for y in 0..board.size_y
		{
			for x in 0..board.size_x
			{
				let pos_x = (x as f32 - (board.size_x) as f32 / 2.0f32 - 0.5f32) * box_size as f32;
				let pos_y = (y as f32 - (board.size_y) as f32 / 2.0f32 - 0.5f32) * box_size as f32;

				shader_data.push(ShaderData{_pos_x: pos_x, _pos_y: pos_y, _col: col, _size: box_size as f32});
			}
		}
	}

	let letter_size = 20;
	let max_letters = 512;
	// Fill board for shader
	let mut letter_datas: Vec<LetterData> = Vec::new();
	for _x in 0..max_letters
	{
		letter_datas.push(LetterData{_pos_x: 0.0f32, _pos_y: 0.032, _col: 0, _size: 0.032,
			_uv_x: 0.032, _uv_y: 0.0f32, _tmp1: 0.0f32, _tmp2: 0.0f32});
	}



	let ssbo: render_gl::ShaderBuffer = render_gl::ShaderBuffer::new_with_data(
		//gl::SHADER_STORAGE_BUFFER,
		gl::UNIFORM_BUFFER,
		shader_data.len() * std::mem::size_of::<ShaderData>(),
		shader_data.as_ptr() as *const gl::types::GLvoid
	);

	let ssbo2: render_gl::ShaderBuffer = render_gl::ShaderBuffer::new_with_data(
		gl::SHADER_STORAGE_BUFFER,
		//gl::UNIFORM_BUFFER,
		letter_datas.len() * std::mem::size_of::<LetterData>(),
		letter_datas.as_ptr() as *const gl::types::GLvoid
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


	let mut rng_: Randomm = Randomm{ rng_seed: now_stamp, rng_inc: 0xa5dfa5dfu64 };
	let mut state = GameState{ player: BlockPiece
			{ pos_x: 3, pos_y: 20, block_type: (rng_.get_next() % 7) as u8, rotation: 0},
			score: 0, high_score: 0, lines: 0, last_row_down: now_stamp, rng: rng_ };

	loop
	{
		letter_datas.clear();
		//app.add_to_array(&"Tetris".to_string(), 30.0f32, 30.0f32, letter_size as f32, colors[8], &mut letter_datas);

		let mut s: String = "score: ".to_string();
		s += &state.score.to_string();
		add_to_array(&app, &s, 30.0f32, 30.0f32, letter_size as f32, colors[8], &mut letter_datas);

		let mut s: String = "lines: ".to_string();
		s += &state.lines.to_string();
		add_to_array(&app, &s, 30.0f32, 60.0f32, letter_size as f32, colors[8], &mut letter_datas);

		let mut s: String = "high score: ".to_string();
		s += &state.high_score.to_string();
		add_to_array(&app, &s, 330.0f32, 50.0f32, letter_size as f32, colors[8], &mut letter_datas);

		last_stamp = now_stamp;
		now_stamp = app.sdl_timer.performance_counter();
		_dt = ((now_stamp - last_stamp) as f64 * 1000.0f64 / perf_freq ) as f32;

		for event in app.event_pump.poll_iter()
		{
			match event
			{
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
				{
					return Ok(());
				},
				Event::KeyDown { keycode: Some(Keycode::A), .. } |
				Event::KeyDown { keycode: Some(Keycode::Left), .. } =>
				{
					let mut tmp = state.player.clone();
					tmp.pos_x -= 1;
					if !board.check_hit(&tmp)
					{
						state.player.pos_x -= 1;
					}
				},
				Event::KeyDown { keycode: Some(Keycode::F), .. } =>
				{
					state.player.block_type = (state.player.block_type + 1) % 7;
				},


				Event::KeyDown { keycode: Some(Keycode::D), .. } |
				Event::KeyDown { keycode: Some(Keycode::Right), .. } =>
				{
					let mut tmp = state.player.clone();
					tmp.pos_x += 1;
					if !board.check_hit(&tmp)
					{
						state.player.pos_x += 1;
					}
				},
				Event::KeyDown { keycode: Some(Keycode::W), .. } |
				Event::KeyDown { keycode: Some(Keycode::Up), .. } =>
				{
					state.player.set_rotation(state.player.rotation + 1);

					while board.check_left_border(&state.player)
					{
						state.player.pos_x += 1;
					}
					while board.check_right_border(&state.player)
					{
						state.player.pos_x -= 1;
					}
					while board.check_hit(&state.player)
					{
						state.player.pos_y += 1;
					}
				},

				Event::KeyDown { keycode: Some(Keycode::S), .. } |
				Event::KeyDown { keycode: Some(Keycode::Down), .. } =>
				{
					while row_down(&mut state, &mut board, now_stamp) {}
				},
				Event::Window {win_event, ..  } =>
				{
					match win_event
					{
						sdl2::event::WindowEvent::Resized( width, height ) =>
						{
							app.window_width = width;
							app.window_height = height;
							println!("Resized: {}: {}", app.window_width, app.window_height);
							unsafe
							{
								gl::Viewport(0, 0, app.window_width, app.window_height);
							}

						},

						_ => {}
					}
				},
				_ => {}
			}
		}


		// Write all the tiles into color from background.
		for y in 0..board.size_y
		{
			for x in 0..board.size_x
			{
				let index = (y * board.size_x + x) as usize;
				shader_data[index]._col = colors[board.board[index] as usize];
			}
		}

		if (now_stamp - state.last_row_down) as f64 * 1000.0f64 / perf_freq > 100.0f64
		{
			row_down(&mut state, &mut board, now_stamp);
		}


		// Draw moving piece.
		for y in 0i32..2i32
		{
			for x in 0i32..4i32
			{
				if BLOCKS[state.player.block_type as usize].blocks[(x + y * 4) as usize] == 1
				{
					let (pos_x, pos_y) = get_rotated(x, y, state.player.rotation);
					if pos_x + state.player.pos_x < board.size_x && pos_y + state.player.pos_y < board.size_y
					{
						shader_data[(state.player.pos_x + pos_x + (state.player.pos_y + pos_y) * board.size_x) as usize]._col
							= colors[(state.player.block_type + 1) as usize];
					}
				}
			}
		}
		ssbo.write_data(0, ssbo.get_size(), shader_data.as_ptr() as *const gl::types::GLvoid);
		ssbo2.write_data(0, ssbo2.get_size(), letter_datas.as_ptr() as *const gl::types::GLvoid);




		shader_program.set_used();
		unsafe
		{
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
			gl::DepthFunc(gl::LESS);
			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::ALWAYS);

			gl::Uniform4f(0, 0.0f32, box_size as f32, app.window_width as f32, app.window_height as f32);
			gl::Disable(gl::BLEND);

			gl::QueryCounter(queries1, gl::TIMESTAMP);
			gl::BindVertexArray(vao);

			ssbo.bind(2);
			ssbo2.bind(3);

			gl::DrawArrays(
				gl::TRIANGLES, // mode
				0, // starting index in the enabled arrays
				6 * shader_data.len() as i32 // number of indices to be rendered
			);

			shader_textured_program.set_used();
			gl::Uniform4f(0, 0.0f32, box_size as f32, app.window_width as f32, app.window_height as f32);

			gl::BindTexture(gl::TEXTURE_2D, tex_handle);
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);


			gl::DrawArrays(
				gl::TRIANGLES, // mode
				0, // starting index in the enabled arrays
				6 * letter_datas.len() as i32// number of indices to be rendered
			);
			//gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
			gl::QueryCounter(queries2, gl::TIMESTAMP);

		}
		::std::thread::sleep(std::time::Duration::from_millis(1));
		app.window.gl_swap_window();

		let duration: f32;
		unsafe
		{
			let mut done = 0;
			let mut done2 = 0;
			while done == 0
			{
				gl::GetQueryObjectiv(queries1, gl::QUERY_RESULT_AVAILABLE, &mut done);
			}

			while done2 == 0
			{
				gl::GetQueryObjectiv(queries2, gl::QUERY_RESULT_AVAILABLE, &mut done2);
			}

			let mut start_time: gl::types::GLuint64 = 0;
			let mut end_time: gl::types::GLuint64 = 0;
				gl::GetQueryObjectui64v(queries1, gl::QUERY_RESULT, &mut start_time);
			gl::GetQueryObjectui64v(queries2, gl::QUERY_RESULT, &mut end_time);
			
			duration = ((end_time - start_time) as f64 / 1000000.0) as f32;
		}
		//println!("x: {}, y: {}", pos_x, pos_y);
		//println!("Frame duration: {}", _dt);
		let title: String = format!("Gpu duration: {}", duration);
		app.window.set_title(&title).unwrap();
	}
	//return Ok(());
}


fn main()
{
	println!("Hello, world!");

	let mut app;
	match core::App::init(800, 900, "Rustris", true)
	{
		Ok(v) =>
		{
			app = v;
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
