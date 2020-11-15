extern crate sdl2;
extern crate gl;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::ffi::CString;
use std::ffi::CStr;

pub mod render_gl;

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
	last_row_down: u64,
}
pub struct Board
{
	size_x: i32,
	size_y: i32,
	board: Vec<u8>
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
	pub fn new(size_x: i32, size_y: i32) -> Self
	{
		let mut b: Vec<u8> = Vec::new();
		for _x in 0..size_x * (size_y + 4)
		{
			b.push(0);
		}

		Self { size_x, size_y, board: b }
	}

	pub fn check_hit(&self, piece: &BlockPiece) -> bool
	{
		for y in 0..2i32
		{
			for x in 0..4i32
			{
				if BLOCKS[piece.block_type as usize].blocks[(x + y * 4) as usize] == 1
				{
					let x_pos = x + piece.pos_x;
					let y_pos = y + piece.pos_y;
					if x_pos >= self.size_x || y_pos >= self.size_y + 4 || x_pos < 0 || y_pos < 0
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

	pub fn add_piece(&mut self, piece: &BlockPiece)
	{
		for y in 0..2i32
		{
			for x in 0..4i32
			{
				if BLOCKS[piece.block_type as usize].blocks[(x + y * 4) as usize] == 1
				{
					let x_pos = x + piece.pos_x;
					let y_pos = y + piece.pos_y;
					if x_pos < self.size_x && y_pos < self.size_y + 4 && x_pos >= 0 && y_pos >= 0
					{
						self.board[(x_pos + y_pos * self.size_x) as usize] = piece.block_type + 1;
					}
				}
			}
		}
	}
}




pub struct ShaderData
{
	_pos_x: f32,
	_pos_y: f32,
	_col: u32,
	_size: f32
}

impl ShaderData
{
	pub fn new(_pos_x: f32, _pos_y: f32, _col: u32, _size: f32) -> Self { Self { _pos_x, _pos_y, _col, _size } }
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
}



fn clamp(value: f32, min: f32, max: f32) ->f32
{
	let mut v = value.max(min);
	v = v.min(max);
	return v;
}

fn get_u32_agbr_color(r: f32, g: f32, b: f32, a: f32) -> u32
{
	let r = clamp(r, 0.0f32, 1.0f32);
	let g = clamp(g, 0.0f32, 1.0f32);
	let b = clamp(b, 0.0f32, 1.0f32);
	let a = clamp(a, 0.0f32, 1.0f32);

	let mut v = 0u32;
	v += (r * 255.0f32) as u32;
	v += ((g * 255.0f32) as u32) << 8u32;
	v += ((b * 255.0f32) as u32) << 16u32;
	v += ((a * 255.0f32) as u32) << 24u32;

	return v;
}

fn row_down(state: &mut GameState, board: &mut Board, now_stamp : u64) -> bool
{
	let mut tmp = state.player.clone();
	tmp.pos_y -= 1;
	state.last_row_down = now_stamp;
	if board.check_hit(&tmp)
	{
		board.add_piece(&state.player);
		state.player.pos_x = 3;
		state.player.pos_y = 20;
		state.player.block_type = (state.player.block_type + 1) % 7;
		return false;
	}
	else
	{
		state.player.pos_y -= 1;
		return true;
	}
}

fn main()
{
	println!("Hello, world!");
	let mut window_width: i32 = 800;
	let mut window_height: i32 = 600;
	let enable_vsync: bool = true;

	let box_size = 40;

	let mut board: Board = Board::new(10, 20);

	let board_back_ground_color = get_u32_agbr_color(0.0f32, 0.0f32, 0.0f32, 1.0f32);

	let _sdl: sdl2::Sdl  = sdl2::init().unwrap();
	let _video: sdl2::VideoSubsystem = _sdl.video().unwrap();
	let _sdl_timer: sdl2::TimerSubsystem = _sdl.timer().unwrap();
	let _window = _video.window("Rustris", window_width as u32, window_height as u32)
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
		gl::ClearColor(0.2, 0.3, 0.5, 1.0);
		gl::ClearDepth(1.0);
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

	// Fill board for shader
	let mut shader_data: Vec<ShaderData> = Vec::new();
	{
		let col = board_back_ground_color;
		for y in 0..board.size_y
		{
			for x in 0..board.size_x
			{
				let x_pos = (x as f32 - (board.size_x) as f32 / 2.0f32 - 0.5f32) * box_size as f32;
				let y_pos = (y as f32 - (board.size_y) as f32 / 2.0f32 - 0.5f32) * box_size as f32;
				shader_data.push(ShaderData::new(x_pos, y_pos, col, box_size as f32 ));
			}
		}
	}

	let ssbo: ShaderBuffer = ShaderBuffer::new_with_data(
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

	let mut now_stamp: u64 = _sdl_timer.performance_counter();
	let mut last_stamp: u64;
	let perf_freq: f64 = _sdl_timer.performance_frequency() as f64;
	let mut _dt: f32;


	let colors: Vec<u32> = vec![
		board_back_ground_color,
		get_u32_agbr_color(1.0, 0.0, 0.0, 1.0),
		get_u32_agbr_color(0.0, 1.0, 0.0, 1.0),
		get_u32_agbr_color(0.0, 0.0, 1.0, 1.0),
		get_u32_agbr_color(1.0, 0.0, 1.0, 1.0),
		get_u32_agbr_color(0.0, 1.0, 1.0, 1.0),
		get_u32_agbr_color(1.0, 1.0, 0.0, 1.0),
		get_u32_agbr_color(1.0, 0.6, 1.0, 1.0),
	];




	let mut state = GameState{ player: BlockPiece{pos_x: 3, pos_y: 20, block_type: 0, rotation: 0}, score: 0, last_row_down: now_stamp };

	'running: loop
	{

		last_stamp = now_stamp;
		now_stamp = _sdl_timer.performance_counter();
		_dt = ((now_stamp - last_stamp) as f64 * 1000.0f64 / perf_freq ) as f32;

		for event in _event_pump.poll_iter()
		{
			match event
			{
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
				{
					break 'running
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
					state.player.pos_y += 1;
					state.player.pos_y = state.player.pos_y.min(board.size_y - 1);
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


		// Write all the tiles into color from background.
		for y in 0..board.size_y
		{
			for x in 0..board.size_x
			{
				let index = (y * board.size_x + x) as usize;
				shader_data[index]._col = colors[board.board[index] as usize];
			}
		}

		if (now_stamp - state.last_row_down) as f64 * 1000.0f64 / perf_freq > 500.0f64
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
					if x + state.player.pos_x < board.size_x && y + state.player.pos_y < board.size_y
					{
						shader_data[((state.player.pos_y + y) * board.size_x + (state.player.pos_x + x)) as usize]._col = colors[(state.player.block_type + 1) as usize];
					}
				}
			}
		}
		ssbo.write_data(0, ssbo.size, shader_data.as_ptr() as *const gl::types::GLvoid);




		shader_program.set_used();
		unsafe
		{
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
			gl::DepthFunc(gl::LESS);
			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::ALWAYS);

			gl::Uniform4f(0, 0.0f32, box_size as f32, window_width as f32, window_height as f32);

			gl::BindVertexArray(vao);

			ssbo.bind(2);

			gl::DrawArrays(
				gl::TRIANGLES, // mode
				0, // starting index in the enabled arrays
				6 * shader_data.len() as i32 // number of indices to be rendered
			);

			//gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
		}
		::std::thread::sleep(std::time::Duration::from_millis(1));
		_window.gl_swap_window();
		//println!("x: {}, y: {}", pos_x, pos_y);
		//println!("Frame duration: {}", _dt);
	}
}
