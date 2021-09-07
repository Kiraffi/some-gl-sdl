use gl;
use std::ffi::CStr;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

pub struct MyTimer
{
	pub now_stamp: u64,
	pub last_stamp: u64,
	pub perf_freq: f64,
	pub dt: f64,
}
pub struct App
{

	pub window_width: i32,
	pub window_height: i32,
	pub vsync: bool,

	pub timer: MyTimer,

	pub _sdl: sdl2::Sdl,
	pub video: sdl2::VideoSubsystem,
	pub sdl_timer: sdl2::TimerSubsystem,
	pub window: sdl2::video::Window,
	pub event_pump: sdl2::EventPump,

	//gl: *const std::os::raw::c_void,
	pub _gl_context: sdl2::video::GLContext,


	pub key_downs: [u8; 512],
	pub key_downs_previous: [u8; 512],
	pub key_half_count: [u8; 512],

	pub quit: bool,
	pub resized: bool,
}

extern "system" fn gl_callback(msg_source: gl::types::GLenum, msg_type: gl::types::GLenum,
	_id: gl::types::GLuint, severity: gl::types::GLenum, _length: gl::types::GLsizei,
	message: *const gl::types::GLchar, _user_param: *mut std::os::raw::c_void)
{
	let message = unsafe
	{
		String::from_utf8(CStr::from_ptr(message).to_bytes().to_vec()).unwrap()
	};


	match severity
	{
		gl::DEBUG_SEVERITY_NOTIFICATION =>
		{
	//		println!("Info: {}", message);
		},

		gl::DEBUG_SEVERITY_LOW |
			gl::DEBUG_SEVERITY_MEDIUM |
			gl::DEBUG_SEVERITY_HIGH =>
		{
			println!("Message: {}", message);
		}
		_=>
		{
			return;
		}
	};

	match msg_source
	{
		gl::DEBUG_SOURCE_API => {},
		gl::DEBUG_SOURCE_WINDOW_SYSTEM => {},
		gl::DEBUG_SOURCE_SHADER_COMPILER => {},
		gl::DEBUG_SOURCE_THIRD_PARTY => {},
		gl::DEBUG_SOURCE_APPLICATION => {},
		gl::DEBUG_SOURCE_OTHER => {},
		_ => return
	};

	match msg_type
	{
		gl::DEBUG_TYPE_ERROR => {},
		gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => {},
		gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => {},
		gl::DEBUG_TYPE_PORTABILITY => {},
		gl::DEBUG_TYPE_PERFORMANCE => {},
		gl::DEBUG_TYPE_MARKER => {},
		gl::DEBUG_TYPE_PUSH_GROUP => {},
		gl::DEBUG_TYPE_POP_GROUP => {},
		gl::DEBUG_TYPE_OTHER => {},
		_ => return
	};

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

		let event_pump = sdl.event_pump()?;
		let mut t = Self{ window_width, window_height, vsync: vsync,
			timer: MyTimer{ now_stamp: sdl_timer.performance_counter(),
				 last_stamp: sdl_timer.performance_counter(), 
				 perf_freq: sdl_timer.performance_frequency() as f64, 
				 dt: 0.0f64 
			},
			_sdl: sdl, video, sdl_timer, window, event_pump, _gl_context, 
			key_downs: [0; 512], key_downs_previous: [0; 512], key_half_count: [0; 512],
			quit: false, resized: false };

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

	pub fn was_pressed(&self, key_code: Keycode) -> bool
	{
		let index = get_usize_from_keycode(key_code);
		return index < 512 && ((self.key_downs[index] == 1u8 && self.key_downs_previous[index] == 0u8 ) || self.key_half_count[index] >= 2u8);
	}

	pub fn was_released(&self, key_code: Keycode) -> bool
	{
		let index = get_usize_from_keycode(key_code);
		return index < 512 && ((self.key_downs[index] == 0u8 && self.key_downs_previous[index] == 1u8 ) || self.key_half_count[index] >= 2u8);
	}

	pub fn is_down(&self, key_code: Keycode)  -> bool
	{
		let index = get_usize_from_keycode(key_code);
		return index < 512 && self.key_downs[index] == 1u8;
	}

	pub fn update(&mut self)
	{
		self.resized = false;
		self.timer.last_stamp = self.timer.now_stamp;
		self.timer.now_stamp = self.sdl_timer.performance_counter();
		self.timer.dt = (self.timer.now_stamp - self.timer.last_stamp) as f64 * 1000.0f64 / self.timer.perf_freq;

		self.key_half_count = [0; 512];
		self.key_downs_previous = self.key_downs;

		for event in self.event_pump.poll_iter()
		{
			match event
			{
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
				{
					self.quit = true;
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
								self.key_half_count[ index ] += 1u8;
								self.key_downs[ index ] = 1u8;
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
								self.key_half_count[ index ] += 1u8;
								self.key_downs[ index ] = 0u8;
							}
							println!("index relased: {}", index);
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
							self.window_width = width;
							self.window_height = height;
							println!("Resized: {}: {}", self.window_width, self.window_height);
							unsafe
							{
								gl::Viewport(0, 0, self.window_width, self.window_height);
							}
							self.resized = true;
						},

						_ => {}
					}
				},
				_ => {}
			}
		}
	}
}


pub fn clamp(value: f32, min: f32, max: f32) ->f32
{
	let mut v = if value > min { value } else { min };
	v = if v < max { v } else { max };
	return v;
}

pub fn get_u32_agbr_color(r: f32, g: f32, b: f32, a: f32) -> u32
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
