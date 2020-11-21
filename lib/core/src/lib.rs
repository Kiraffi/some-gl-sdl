use gl;
use std;
use std::ffi::CStr;

pub struct App
{

	pub window_width: i32,
	pub window_height: i32,
	vsync: bool,

	pub _sdl: sdl2::Sdl,
	pub video: sdl2::VideoSubsystem,
	pub sdl_timer: sdl2::TimerSubsystem,
	pub window: sdl2::video::Window,
	pub event_pump: sdl2::EventPump,

	//gl: *const std::os::raw::c_void,
	pub _gl_context: sdl2::video::GLContext
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
}