extern crate sdl2;
extern crate gl;

extern crate render_gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::ffi::CString;
use std::ffi::CStr;

//pub mod render_gl;

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

fn clamp(value: f32, min: f32, max: f32) ->f32
{
	let mut v = if value > min { value } else { min };
	v = if v < max { v } else { max };
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
					let (mut x_pos, mut y_pos) = get_rotated(x, y, piece.rotation);
					x_pos += piece.pos_x;
					y_pos += piece.pos_y;
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
					let (mut x_pos, mut y_pos) = get_rotated(x, y, piece.rotation);
					x_pos += piece.pos_x;
					y_pos += piece.pos_y;
					if x_pos < self.size_x && y_pos < self.size_y + 4 && x_pos >= 0 && y_pos >= 0
					{
						self.board[(x_pos + y_pos * self.size_x) as usize] = piece.block_type + 1;
					}
				}
			}
		}
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




pub struct ShaderData
{
	_pos_x: f32,
	_pos_y: f32,
	_col: u32,
	_size: f32
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
		board.add_piece(&state.player);
		state.player.pos_x = 3;
		state.player.pos_y = 20;
		state.player.block_type = (state.player.block_type + 1) % 7;

		let mut rows = 0;
		for y in 0..board.size_y
		{
			while board.check_row(y)
			{
				rows += 1;
				board.remove_row(y);
			}
		}
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
		println!("Score: {}", state.score);
		return false;
	}
	else
	{
		state.player.pos_y -= 1;
		return true;
	}
}


struct App 
{
	
	window_width: i32,
	window_height: i32,
	vsync: bool,

	_sdl: sdl2::Sdl,
	video: sdl2::VideoSubsystem,
	sdl_timer: sdl2::TimerSubsystem,
	window: sdl2::video::Window,
	event_pump: sdl2::EventPump,

	//gl: *const std::os::raw::c_void,
	_gl_context: sdl2::video::GLContext
}

impl App
{
	pub fn init(window_width: i32, window_height: i32, window_name: &str, vsync: bool) -> Result<Self, String>
	{
		/*
		if width == 1
		{
			return Err("failed to initialize".to_string());
		}
*/
		let sdl: sdl2::Sdl  = sdl2::init().unwrap();
		let video: sdl2::VideoSubsystem = sdl.video().unwrap();
		let sdl_timer: sdl2::TimerSubsystem = sdl.timer().unwrap();
		let window;
		match video.window(window_name, window_width as u32, window_height as u32)
		.resizable()
		.opengl()
		.build()
		{
			Ok(v) => 
			{
				window = v; 
			} 
			Err(e) => 
			{ 
				println!("Error: {}", e); 
				return Err("Failed to build window!".to_string()); 
			}
		}
	
		let gl_attr = video.gl_attr();
	
		gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		gl_attr.set_context_version(4, 5);
	
		gl_attr.set_context_flags().debug().set();
		
	
		let _gl_context = window.gl_create_context()?;
		let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);
	
		
	
		let version;
		match unsafe
		{
			let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
				.to_bytes()
				.to_vec();
			String::from_utf8(data)
		}
		{
			Ok(v) => 
			{
				version = v; 
			} 
			Err(e) => 
			{ 
				println!("Error: {}", e); 
				return Err("Failed to read version data from gl!".to_string()); 
			}
		}
	
		println!("OpenGL version {}", version);
	
	
		unsafe
		{
			gl::Viewport(0, 0, window_width as i32, window_height as i32);
			gl::ClearColor(0.2, 0.3, 0.5, 1.0);
			gl::ClearDepth(1.0);
			// Swapping up and down just messes things up like in renderdoc....
			//gl::ClipControl(gl::UPPER_LEFT, gl::ZERO_TO_ONE);
			gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
		}
	

		let event_pump = sdl.event_pump()?;

		let mut t = Self{ window_width, window_height, vsync: vsync, 
			_sdl: sdl, video, sdl_timer, window, event_pump, _gl_context };
		
		t.enable_vsync(vsync)?;
	
		return Ok(t);
	}

	pub fn enable_vsync(&mut self, enable_vsync: bool) -> Result<(), String>
	{
		if enable_vsync
		{
			self.video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync)?;
		}
		else
		{
			self.video.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate)?;
		}

		self.vsync = enable_vsync;
		return Ok(());
	}

	pub fn run(&mut self) -> Result<(), String>
	{
		let box_size = 40;

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

		let colors: [u32; 8] = [
			// Background color
			get_u32_agbr_color(0.0, 0.0, 0.0, 1.0),
			
			// Block colors
			get_u32_agbr_color(1.0, 0.0, 0.0, 1.0),
			get_u32_agbr_color(0.0, 1.0, 0.0, 1.0),
			get_u32_agbr_color(0.0, 0.0, 1.0, 1.0),
			get_u32_agbr_color(1.0, 0.0, 1.0, 1.0),
			get_u32_agbr_color(0.0, 1.0, 1.0, 1.0),
			get_u32_agbr_color(1.0, 1.0, 0.0, 1.0),
			get_u32_agbr_color(1.0, 0.6, 1.0, 1.0),
		];


		let mut board: Board = Board::new(10, 20);


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

		let mut now_stamp: u64 = self.sdl_timer.performance_counter();
		let mut last_stamp: u64;
		let perf_freq: f64 = self.sdl_timer.performance_frequency() as f64;
		let mut _dt: f32;





		let mut state = GameState{ player: BlockPiece{pos_x: 3, pos_y: 20, block_type: 0, rotation: 0}, score: 0, last_row_down: now_stamp };

		loop
		{

			last_stamp = now_stamp;
			now_stamp = self.sdl_timer.performance_counter();
			_dt = ((now_stamp - last_stamp) as f64 * 1000.0f64 / perf_freq ) as f32;

			for event in self.event_pump.poll_iter()
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
						state.player.rotation = (state.player.rotation + 1) % 4;
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
								self.window_width = width;
								self.window_height = height;
								println!("Resized: {}: {}", self.window_width, self.window_height);
								unsafe
								{
									gl::Viewport(0, 0, self.window_width, self.window_height);
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
				gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
				gl::DepthFunc(gl::LESS);
				gl::Enable(gl::DEPTH_TEST);
				gl::DepthFunc(gl::ALWAYS);

				gl::Uniform4f(0, 0.0f32, box_size as f32, self.window_width as f32, self.window_height as f32);

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
			self.window.gl_swap_window();
			//println!("x: {}, y: {}", pos_x, pos_y);
			//println!("Frame duration: {}", _dt);
		}
	}

	//return Ok(());
}

fn main()
{
	println!("Hello, world!");

	let mut app;
	match App::init(800, 900, "Rustris", true)
	{
		Ok(v) => 
		{
			app = v;
			match app.run()
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
