extern crate gl;

extern crate render_gl;
extern crate core;
extern crate render_systems;
//extern crate sdl_window;

use sdl_window_state::MyKey;

use std::ffi::CString;
use render_systems::fontsystem::FontSystem;
use render_gl::CommonShaderFrameDate;
use core::RandomPCG;

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
	rng: RandomPCG
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

const BLOCKS: [Block; 7] = [
	Block{ blocks: [1,1,1,1, 0,0,0,0]},
	Block{ blocks: [0,1,1,1, 0,0,0,1]},
	Block{ blocks: [0,1,1,1, 0,1,0,0]},

	Block{ blocks: [0,1,1,1, 0,0,1,0]},
	Block{ blocks: [0,1,1,0, 0,1,1,0]},
	Block{ blocks: [0,1,1,0, 0,0,1,1]},
	Block{ blocks: [0,0,1,1, 0,1,1,0]}
];




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


const TRIANGLE_VERT: &'static str = include_str!("triangle.vert");
const TRIANGLE_FRAG: &'static str = include_str!("triangle.frag");


fn include_file(s: &'static str, name: &'static str) -> Result<CString, String>
{
	let res = match CString::new(s)
	{
		Ok(v) => v,
		Err(_) => return Err(format!("Failed to include file: {}", name).to_string())
	};
	return Ok(res);
}

fn run() -> Result<(), String>
{
	let mut app = sdl_window::App::init(800, 900, "Rustris", false)?;

	//let _gl = gl::load_with(&|s| app.video.gl_get_proc_address(s) as *const std::os::raw::c_void);
	render_gl::init_gl(&app.window_state)?;

	let mut font_system: FontSystem = FontSystem::init()?;
//	let frag_shader = render_gl::Shader::from_frag_source(
//		&CString::new(include_str!("triangle.frag")).unwrap(), &"triangle.frag".to_string()

	// use paired number
	let box_size = 40;
	let shader_program;
	{
		let vert_shader = render_gl::Shader::from_vert_source(
			&include_file(TRIANGLE_VERT, &"trangle.vert")?, &"triangle.vert".to_string()
		)?;
		let frag_shader = render_gl::Shader::from_frag_source(
			&include_file(TRIANGLE_FRAG,"triangle.frag")?, &"triangle.frag".to_string()
		)?;

		shader_program = render_gl::Program::from_shaders(
			&[vert_shader, frag_shader]
		).unwrap();
	}

	shader_program.set_used();

	let mut queries1: [gl::GLuint; 4] = [0; 4];
	let mut queries2: [gl::GLuint; 4] = [0; 4];
	let mut queries3: [gl::GLuint; 4] = [0; 4];
	unsafe 
	{
		gl::glGenQueries(4, &mut queries1[0]);
		gl::glGenQueries(4, &mut queries2[0]);
		gl::glGenQueries(4, &mut queries3[0]);
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
		for _ in 0..1024
		{
			shader_data.push(ShaderData{_pos_x: 0.0f32, _pos_y: 0.0f32, _col: 0u32, _size: 0.0f32});
		}


		let col = colors[0];

		let mut start_x: f32 =  (app.window_state.window_width - (board.size_x * box_size) ) as f32 / 2.0f32;
		let mut start_y: f32 =  (app.window_state.window_height - (board.size_y * box_size) ) as f32 / 2.0f32;

		start_x = start_x + box_size as f32 / 2.0f32;
		start_y = start_y + box_size as f32 / 2.0f32;

		if ((start_x * 2.0f32) as i32) % 2i32 == 0i32 { start_x += 0.5f32; }
		if ((start_y * 2.0f32) as i32) % 2i32 == 0i32 { start_y += 0.5f32; }
		let mut index = 0usize;

		for y in 0..board.size_y
		{
			for x in 0..board.size_x
			{
				let pos_x = start_x + (x * box_size) as f32;
				let pos_y = start_y + (y * box_size) as f32;

				shader_data[index] = ShaderData{_pos_x: pos_x, _pos_y: pos_y, _col: col, _size: box_size as f32};
				index += 1usize;
			}
		}
	}

	
 
	let ssbo: render_gl::ShaderBuffer = render_gl::ShaderBuffer::new_with_data(
		//gl::GL_SHADER_STORAGE_BUFFER,
		gl::GL_UNIFORM_BUFFER,
		shader_data.len() * std::mem::size_of::<ShaderData>(),
		shader_data.as_ptr() as *const gl::GLvoid
	);

	let frame_data_buffer: render_gl::ShaderBuffer = render_gl::ShaderBuffer::new(
		gl::GL_UNIFORM_BUFFER,
		65536
	);

	let mut vao: gl::GLuint = 0;
	unsafe
	{
		gl::glGenVertexArrays(1, &mut vao);
	}

	let mut rng_: RandomPCG = RandomPCG::new(app.window_state.timer.now_stamp);
	let mut state = GameState{ player: BlockPiece
			{ pos_x: 3, pos_y: 20, block_type: (rng_.get_next() % 7) as u8, rotation: 0},
			score: 0, high_score: 0, lines: 0, last_row_down: app.window_state.timer.now_stamp, rng: rng_ };

	let mut frame_count :u64 = 0u64;
	let letter_size = 16;

	let mut durations1 = [0.0f64 ; 20];
	let mut durations2 = [0.0f64 ; 20];

	while !app.window_state.quit
	{
		let current_frame_index : usize = (frame_count % 4) as usize;
		let previous_frame_index : usize = ((frame_count + 1) % 4) as usize;

		let mut s: String = "Score: ".to_string();
		s += &state.score.to_string();
		font_system.draw_string(&s, 5.0f32, 5.0f32, letter_size as f32, letter_size as f32, colors[8]);

		let mut s: String = "Lines: ".to_string();
		s += &state.lines.to_string();
		font_system.draw_string(&s, 5.0f32, (5 + letter_size) as f32, letter_size as f32, letter_size as f32, colors[8]);

		let mut s: String = "High Score: ".to_string();
		s += &state.high_score.to_string();
		font_system.draw_string(&s, (app.window_state.window_width - 300) as f32, 5.0f32, letter_size as f32, letter_size as f32, colors[8]);
		
		app.update();
		render_gl::update(&mut app.window_state);

		if app.was_pressed(MyKey::A) || app.was_pressed(MyKey::Left)
		{
			let mut tmp = state.player.clone();
			tmp.pos_x -= 1;
			if !board.check_hit(&tmp)
			{
				state.player.pos_x -= 1;
			}
		}

		if app.was_pressed(MyKey::D) || app.was_pressed(MyKey::Right)
		{
			let mut tmp = state.player.clone();
			tmp.pos_x += 1;
			if !board.check_hit(&tmp)
			{
				state.player.pos_x += 1;
			}
		}
		
		if app.was_pressed(MyKey::W) || app.was_pressed(MyKey::Up)
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

		if app.was_pressed(MyKey::S) || app.was_pressed(MyKey::Down)
		{
			while row_down(&mut state, &mut board, app.window_state.timer.now_stamp) {}
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

		if (app.window_state.timer.now_stamp - state.last_row_down) as f64 * 1000.0f64 / app.window_state.timer.perf_freq > 100.0f64
		{
			row_down(&mut state, &mut board, app.window_state.timer.now_stamp);
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


		unsafe
		{ 
			gl::glQueryCounter(queries1[current_frame_index], gl::GL_TIMESTAMP);
			font_system.update(app.window_state.window_width as f32, app.window_state.window_height as f32);

			let tmp = CommonShaderFrameDate::new(app.window_state.window_width, app.window_state.window_height);

			frame_data_buffer.write_data(0, std::mem::size_of::<CommonShaderFrameDate>(), &tmp as *const _ as *const gl::GLvoid);
			ssbo.write_data(0, shader_data.len() * std::mem::size_of::<ShaderData>(), shader_data.as_ptr() as *const gl::GLvoid);
			shader_program.set_used();
			
			gl::glClear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT );
			gl::glDepthFunc(gl::GL_LESS);
			gl::glEnable(gl::GL_DEPTH_TEST);
			gl::glDepthFunc(gl::GL_ALWAYS);
			gl::glDisable(gl::GL_BLEND);

			
			gl::glBindVertexArray(vao);

			frame_data_buffer.bind(0);
			ssbo.bind(2);

			gl::glDrawArrays(
				gl::GL_TRIANGLES, // mode
				0, // starting index in the enabled arrays
				6 * shader_data.len() as i32 // number of indices to be rendered
			);
			for _ in 0..1
			{
				font_system.draw();
			}

			gl::glQueryCounter(queries2[current_frame_index], gl::GL_TIMESTAMP);
			app.swap_buffer();
			gl::glQueryCounter(queries3[current_frame_index], gl::GL_TIMESTAMP);
		}
		::std::thread::sleep(std::time::Duration::from_micros(1000));

		if frame_count >= 4
		{
			unsafe
			{
				let mut done = 0;
				let mut done2 = 0;
				let mut done3 = 0;
				while done == 0 && done2 == 0
				{
					if done == 0
					{
						gl::glGetQueryObjectiv(queries1[previous_frame_index], gl::GL_QUERY_RESULT_AVAILABLE, &mut done);
					}
					if done2 == 0
					{
						gl::glGetQueryObjectiv(queries2[previous_frame_index], gl::GL_QUERY_RESULT_AVAILABLE, &mut done2);
					}
					if done3 == 0
					{
						gl::glGetQueryObjectiv(queries3[previous_frame_index], gl::GL_QUERY_RESULT_AVAILABLE, &mut done3);
					}
				}
				
				let mut start_time: gl::GLuint64 = 0;
				let mut end_time1: gl::GLuint64 = 0;
				let mut end_time2: gl::GLuint64 = 0;
				gl::glGetQueryObjectui64v(queries1[previous_frame_index], gl::GL_QUERY_RESULT, &mut start_time);
				gl::glGetQueryObjectui64v(queries2[previous_frame_index], gl::GL_QUERY_RESULT, &mut end_time1);
				gl::glGetQueryObjectui64v(queries3[previous_frame_index], gl::GL_QUERY_RESULT, &mut end_time2);
				
				let duration1: f64 = ((end_time1 - start_time) as f64 / 1000000000.0) as f64;
				let duration2: f64 = ((end_time2 - start_time) as f64 / 1000000000.0) as f64;

				durations1[ (frame_count % durations1.len() as u64) as usize ] = duration1;
				durations2[ (frame_count % durations2.len() as u64) as usize ] = duration2;
			}
		}

		let mut duration1 = 0.0f64;
		let mut duration2 = 0.0f64;
		for dur in durations1
		{
			duration1 += dur;
		}
		for dur in durations2
		{
			duration2 += dur;
		}
		duration1 /= durations1.len() as f64;
		duration2 /= durations2.len() as f64;
		//println!("x: {}, y: {}", pos_x, pos_y);
		//println!("Frame duration: {}", _dt);
		let title: String = format!("Gpu duration: {:.3}ms, fps: {:.3} vs whole {:.3}, fps: {:.3}", 
			duration1 * 1000.0f64, 1.0f64 / duration1,
			duration2 * 1000.0f64, 1.0f64 / duration2);
		app.set_window_title(&title);
		frame_count = frame_count + 1u64;
	}
	return Ok(());
}

fn main()
{
	match run()
	{
		Ok(_) =>
		{
		}
		Err(e) =>
		{
			println!("Error: {}", e);
			//panic!(e);
		}
	}
}
