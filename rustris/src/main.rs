extern crate sdl2;
extern crate gl;

extern crate render_gl;
extern crate core;
extern crate render_systems;

use sdl2::keyboard::Keycode;

use std::ffi::CString;
use render_systems::fontsystem::{FontSystem};

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

fn run(app: &mut core::App) -> Result<(), String>
{
	let mut font_system: FontSystem = FontSystem::init()?;

	// use paired number
	let box_size = 40;
	let shader_program;
	{
		let vert_shader = render_gl::Shader::from_vert_source(
			&CString::new(include_str!("triangle.vert")).unwrap(), &"triangle.vert".to_string()
		)?;
		let frag_shader = render_gl::Shader::from_frag_source(
			&CString::new(include_str!("triangle.frag")).unwrap(), &"triangle.frag".to_string()
		)?;

		shader_program = render_gl::Program::from_shaders(
			&[vert_shader, frag_shader]
		).unwrap();
	}

	shader_program.set_used();

	let mut queries1: [gl::types::GLuint; 4] = [0; 4];
	let mut queries2: [gl::types::GLuint; 4] = [0; 4];
	unsafe 
	{
		gl::GenQueries(4, &mut queries1[0]);
		gl::GenQueries(4, &mut queries2[0]);
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

		let mut start_x: f32 =  (app.window_width - (board.size_x * box_size) ) as f32 / 2.0f32;
		let mut start_y: f32 =  (app.window_height - (board.size_y * box_size) ) as f32 / 2.0f32;

		start_x = start_x + box_size as f32 / 2.0f32;
		start_y = start_y + box_size as f32 / 2.0f32;

		if ((start_x * 2.0f32) as i32) % 2i32 == 0i32 { start_x += 0.5f32; }
		if ((start_y * 2.0f32) as i32) % 2i32 == 0i32 { start_y += 0.5f32; }

		for y in 0..board.size_y
		{
			for x in 0..board.size_x
			{
				let pos_x = start_x + (x * box_size) as f32;
				let pos_y = start_y + (y * box_size) as f32;

				shader_data.push(ShaderData{_pos_x: pos_x, _pos_y: pos_y, _col: col, _size: box_size as f32});
			}
		}
	}

	
 
	let ssbo: render_gl::ShaderBuffer = render_gl::ShaderBuffer::new_with_data(
		//gl::SHADER_STORAGE_BUFFER,
		gl::UNIFORM_BUFFER,
		shader_data.len() * std::mem::size_of::<ShaderData>(),
		shader_data.as_ptr() as *const gl::types::GLvoid
	);

	let mut vao: gl::types::GLuint = 0;
	unsafe
	{
		gl::GenVertexArrays(1, &mut vao);
	}

	let mut rng_: Randomm = Randomm{ rng_seed: app.timer.now_stamp, rng_inc: 0xa5dfa5dfu64 };
	let mut state = GameState{ player: BlockPiece
			{ pos_x: 3, pos_y: 20, block_type: (rng_.get_next() % 7) as u8, rotation: 0},
			score: 0, high_score: 0, lines: 0, last_row_down: app.timer.now_stamp, rng: rng_ };

	let mut frame_count :u64 = 0u64;
	let letter_size = 16;

	let mut durations = [0.0f64 ; 20];

	while !app.quit
	{
		let current_frame_index : usize = (frame_count % 4) as usize;
		let previous_frame_index : usize = ((frame_count + 1) % 4) as usize;

		app.update();
		font_system.update(app.window_width as f32, app.window_height as f32);

		let mut s: String = "Score: ".to_string();
		s += &state.score.to_string();
		font_system.draw_string(&s, 5.0f32, 5.0f32, letter_size as f32, letter_size as f32, colors[8]);

		let mut s: String = "Lines: ".to_string();
		s += &state.lines.to_string();
		font_system.draw_string(&s, 5.0f32, (5 + letter_size) as f32, letter_size as f32, letter_size as f32, colors[8]);

		let mut s: String = "High Score: ".to_string();
		s += &state.high_score.to_string();
		font_system.draw_string(&s, (app.window_width - 300) as f32, 5.0f32, letter_size as f32, letter_size as f32, colors[8]);
		

		if app.was_pressed(Keycode::A) || app.was_pressed(Keycode::Left)
		{
			let mut tmp = state.player.clone();
			tmp.pos_x -= 1;
			if !board.check_hit(&tmp)
			{
				state.player.pos_x -= 1;
			}
		}

		if app.was_pressed(Keycode::D) || app.was_pressed(Keycode::Right)
		{
			let mut tmp = state.player.clone();
			tmp.pos_x += 1;
			if !board.check_hit(&tmp)
			{
				state.player.pos_x += 1;
			}
		}
		
		if app.was_pressed(Keycode::W) || app.was_pressed(Keycode::Up)
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
		}

		if app.was_pressed(Keycode::S) || app.was_pressed(Keycode::Down)
		{
			while row_down(&mut state, &mut board, app.timer.now_stamp) {}
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

		if (app.timer.now_stamp - state.last_row_down) as f64 * 1000.0f64 / app.timer.perf_freq > 100.0f64
		{
			row_down(&mut state, &mut board, app.timer.now_stamp);
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




		shader_program.set_used();
		unsafe
		{
			gl::QueryCounter(queries1[current_frame_index], gl::TIMESTAMP);
			
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
			gl::DepthFunc(gl::LESS);
			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::ALWAYS);

			gl::Uniform4f(0, 0.0f32, box_size as f32, app.window_width as f32, app.window_height as f32);
			gl::Disable(gl::BLEND);


			
			gl::BindVertexArray(vao);

			ssbo.bind(2);

			gl::DrawArrays(
				gl::TRIANGLES, // mode
				0, // starting index in the enabled arrays
				6 * shader_data.len() as i32 // number of indices to be rendered
			);

			font_system.draw();

			gl::QueryCounter(queries2[current_frame_index], gl::TIMESTAMP);

		}
		::std::thread::sleep(std::time::Duration::from_micros(1000));
		app.window.gl_swap_window();

		 0.0f32;
		if frame_count >= 4
		{
			unsafe
			{
				let mut done = 0;
				let mut done2 = 0;
				while done == 0 && done2 == 0
				{
					if done == 0
					{
						gl::GetQueryObjectiv(queries1[previous_frame_index], gl::QUERY_RESULT_AVAILABLE, &mut done);
					}
					if done2 == 0
					{
						gl::GetQueryObjectiv(queries2[previous_frame_index], gl::QUERY_RESULT_AVAILABLE, &mut done2);
					}
				}
				
				let mut start_time: gl::types::GLuint64 = 0;
				let mut end_time: gl::types::GLuint64 = 0;
				gl::GetQueryObjectui64v(queries1[previous_frame_index], gl::QUERY_RESULT, &mut start_time);
				gl::GetQueryObjectui64v(queries2[previous_frame_index], gl::QUERY_RESULT, &mut end_time);
				
				let duration: f64 = ((end_time - start_time) as f64 / 10000000.0) as f64;

				durations[ (frame_count % durations.len() as u64) as usize ] = duration;
			}
		}

		let mut duration = 0.0f64;
		for dur in durations
		{
			duration += dur;
		}
		duration /= durations.len() as f64;
		//println!("x: {}, y: {}", pos_x, pos_y);
		//println!("Frame duration: {}", _dt);
		let title: String = format!("Gpu duration: {:.3}ms, fps: {:.3}", duration * 1000.0f64, 1.0f64 / duration);
		app.window.set_title(&title).unwrap();
		frame_count = frame_count + 1u64;
	}
	return Ok(());
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
