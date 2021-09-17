use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use sdl_window_state::MyKey;

fn get_usize_from_mykey(keycode: MyKey) -> usize
{
	let mut val:u32 = unsafe 
	{ 
		std::mem::transmute::<MyKey, u32>(keycode)
	};
	if (val & 0x40000000) == 0x40000000 
	{
		val =  (val & 0xffffu32) + 128u32;
	}
	
	return val as usize;
}

fn get_usize_from_keycode(keycode: Keycode) -> usize
{
	let mut val:u32 = unsafe 
	{ 
		std::mem::transmute::<Keycode, u32>(keycode)
	};
	if (val & 0x40000000) == 0x40000000 
	{
		val =  (val & 0xffffu32) + 128u32;
	}
	
	return val as usize;
}


pub struct App
{
	pub window_state: sdl_window_state::SdlWindowState,

	_sdl: sdl2::Sdl,
	video: sdl2::VideoSubsystem,
	sdl_timer: sdl2::TimerSubsystem,
	window: sdl2::video::Window,
	event_pump: sdl2::EventPump,

	//gl: *const std::os::raw::c_void,
	_gl_context: sdl2::video::GLContext,

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
		gl_attr.set_context_version(4, 6);

		gl_attr.set_context_flags().debug().set();
		gl_attr.set_framebuffer_srgb_compatible(true);

		let _gl_context = window.gl_create_context()?;

		/*

		let _gl = gl::load_with(&|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);


		unsafe
		{
			//gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
			gl::DebugMessageCallback(Some(gl_callback), std::ptr::null());
			gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0,
										std::ptr::null(), gl::TRUE);
			gl::Enable(gl::DEBUG_OUTPUT);
		}

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
			gl::Viewport(0, 0, window_width, window_height);
			gl::ClearColor(0.2, 0.3, 0.5, 1.0);
			gl::ClearDepth(1.0);
			// Swapping up and down just messes things up like in renderdoc....
			//gl::ClipControl(gl::UPPER_LEFT, gl::ZERO_TO_ONE);
			gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
		}
*/
		let event_pump = sdl.event_pump()?;
		let mut t = Self{ window_state: sdl_window_state::SdlWindowState { window_width, window_height, vsync: vsync,
			timer: my_core::MyTimer{ now_stamp: sdl_timer.performance_counter(),
				 last_stamp: sdl_timer.performance_counter(), 
				 perf_freq: sdl_timer.performance_frequency() as f64, 
				 dt: 0.0f64 
			},
			key_downs: [0; 512], key_downs_previous: [0; 512], key_half_count: [0; 512],
			mouse_x: 0, mouse_y: 0, mouse_b: 0,
			quit: false, resized: false, },

			_sdl: sdl, video, sdl_timer, window, event_pump, _gl_context, 
		};
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

		self.window_state.vsync = enable_vsync;
		return Ok(());
	}

	pub fn was_pressed(&self, key_code: MyKey) -> bool
	{
		let index = get_usize_from_mykey(key_code);
		return index < 512 && ((self.window_state.key_downs[index] == 1u8 && self.window_state.key_downs_previous[index] == 0u8 ) ||
			self.window_state.key_half_count[index] >= 2u8);
	}

	pub fn was_released(&self, key_code: MyKey) -> bool
	{
		let index = get_usize_from_mykey(key_code);
		return index < 512 && ((self.window_state.key_downs[index] == 0u8 && self.window_state.key_downs_previous[index] == 1u8 ) ||
			self.window_state.key_half_count[index] >= 2u8);
	}

	pub fn is_down(&self, key_code: MyKey)  -> bool
	{
		let index = get_usize_from_mykey(key_code);
		return index < 512 && self.window_state.key_downs[index] == 1u8;
	}

	pub fn swap_buffer(&mut self)
	{
		self.window.gl_swap_window();
	}

	pub fn set_window_title(&mut self, title:&String )
	{
		self.window.set_title(&title).unwrap();
	}

	pub fn update(&mut self)
	{
		let ref mut timer = self.window_state.timer;
		timer.last_stamp = timer.now_stamp;
		timer.now_stamp = self.sdl_timer.performance_counter();
		timer.dt = (timer.now_stamp - timer.last_stamp) as f64 * 1000.0f64 / timer.perf_freq;

		self.window_state.key_half_count = [0; 512];
		self.window_state.key_downs_previous = self.window_state.key_downs;

		for event in self.event_pump.poll_iter()
		{
			match event
			{
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
				{
					self.window_state.quit = true;
				},
				Event::KeyDown { keycode, .. } =>
				{
					match keycode
					{
						Some(x) =>
						{
							let index = get_usize_from_keycode(x);
							if index < 512
							{
								self.window_state.key_half_count[ index ] += 1u8;
								self.window_state.key_downs[ index ] = 1u8;
							}
							println!("index pressed: {}", index);
						},
						_ => {}
					}
				},
				Event::KeyUp { keycode, .. } =>
				{
					match keycode
					{
						Some(x) =>
						{
							let index = get_usize_from_keycode(x);
							if index < 512
							{
								self.window_state.key_half_count[ index ] += 1u8;
								self.window_state.key_downs[ index ] = 0u8;
							}
							//println!("index relased: {}", index);
						},
						_ => {}
					}
				},

				Event::Window {win_event, ..  } =>
				{
					match win_event
					{
						sdl2::event::WindowEvent::Resized( width, height ) =>
						{
							self.window_state.resized = self.window_state.window_width != width || self.window_state.window_height != height;
							self.window_state.window_width = width;
							self.window_state.window_height = height;
						},

						_ => {}
					}
				},


				Event::MouseButtonDown { mouse_btn, x, y, .. } =>
				{
					self.window_state.mouse_x = x;
					self.window_state.mouse_y = self.window_state.window_height as i32 - y;
					if mouse_btn == MouseButton::Left
					{
						self.window_state.mouse_b |= 1;
					}
					else if mouse_btn == MouseButton::Right
					{
						self.window_state.mouse_b |= 2;
					}
				},
				Event::MouseButtonUp { mouse_btn, x, y, .. } =>
				{
					self.window_state.mouse_x = x;
					self.window_state.mouse_y = self.window_state.window_height as i32 - y;
					if mouse_btn == MouseButton::Left
					{
						self.window_state.mouse_b &= !1;
					}
					else if mouse_btn == MouseButton::Right
					{
						self.window_state.mouse_b &= !2;
					}
				},
				Event::MouseMotion { x, y, .. } =>
				{
					self.window_state.mouse_x = x;
					self.window_state.mouse_y = self.window_state.window_height - y;
				},

				_ => {}
			}
		}
	}
}
