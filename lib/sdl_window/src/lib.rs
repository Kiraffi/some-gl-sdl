//use gl;
//use std::ffi::CStr;

use core::MyTimer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
//use sdl2::sys::Keycode;




#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum MyKey {
    Backspace = Keycode::Backspace as i32,
    Tab = Keycode::Tab as i32,
    Return = Keycode::Return as i32,
    Escape = Keycode::Escape as i32,
    Space = Keycode::Space as i32,
    Exclaim = Keycode::Exclaim as i32,
    Quotedbl = Keycode::Quotedbl as i32,
    Hash = Keycode::Hash as i32,
    Dollar = Keycode::Dollar as i32,
    Percent = Keycode::Percent as i32,
    Ampersand = Keycode::Ampersand as i32,
    Quote = Keycode::Quote as i32,
    LeftParen = Keycode::LeftParen as i32,
    RightParen = Keycode::RightParen as i32,
    Asterisk = Keycode::Asterisk as i32,
    Plus = Keycode::Plus as i32,
    Comma = Keycode::Comma as i32,
    Minus = Keycode::Minus as i32,
    Period = Keycode::Period as i32,
    Slash = Keycode::Slash as i32,
    Num0 = Keycode::Num0 as i32,
    Num1 = Keycode::Num1 as i32,
    Num2 = Keycode::Num2 as i32,
    Num3 = Keycode::Num3 as i32,
    Num4 = Keycode::Num4 as i32,
    Num5 = Keycode::Num5 as i32,
    Num6 = Keycode::Num6 as i32,
    Num7 = Keycode::Num7 as i32,
    Num8 = Keycode::Num8 as i32,
    Num9 = Keycode::Num9 as i32,
    Colon = Keycode::Colon as i32,
    Semicolon = Keycode::Semicolon as i32,
    Less = Keycode::Less as i32,
    Equals = Keycode::Equals as i32,
    Greater = Keycode::Greater as i32,
    Question = Keycode::Question as i32,
    At = Keycode::At as i32,
    LeftBracket = Keycode::LeftBracket as i32,
    Backslash = Keycode::Backslash as i32,
    RightBracket = Keycode::RightBracket as i32,
    Caret = Keycode::Caret as i32,
    Underscore = Keycode::Underscore as i32,
    Backquote = Keycode::Backquote as i32,
    A = Keycode::A as i32,
    B = Keycode::B as i32,
    C = Keycode::C as i32,
    D = Keycode::D as i32,
    E = Keycode::E as i32,
    F = Keycode::F as i32,
    G = Keycode::G as i32,
    H = Keycode::H as i32,
    I = Keycode::I as i32,
    J = Keycode::J as i32,
    K = Keycode::K as i32,
    L = Keycode::L as i32,
    M = Keycode::M as i32,
    N = Keycode::N as i32,
    O = Keycode::O as i32,
    P = Keycode::P as i32,
    Q = Keycode::Q as i32,
    R = Keycode::R as i32,
    S = Keycode::S as i32,
    T = Keycode::T as i32,
    U = Keycode::U as i32,
    V = Keycode::V as i32,
    W = Keycode::W as i32,
    X = Keycode::X as i32,
    Y = Keycode::Y as i32,
    Z = Keycode::Z as i32,
    Delete = Keycode::Delete as i32,
    CapsLock = Keycode::CapsLock as i32,
    F1 = Keycode::F1 as i32,
    F2 = Keycode::F2 as i32,
    F3 = Keycode::F3 as i32,
    F4 = Keycode::F4 as i32,
    F5 = Keycode::F5 as i32,
    F6 = Keycode::F6 as i32,
    F7 = Keycode::F7 as i32,
    F8 = Keycode::F8 as i32,
    F9 = Keycode::F9 as i32,
    F10 = Keycode::F10 as i32,
    F11 = Keycode::F11 as i32,
    F12 = Keycode::F12 as i32,
    PrintScreen = Keycode::PrintScreen as i32,
    ScrollLock = Keycode::ScrollLock as i32,
    Pause = Keycode::Pause as i32,
    Insert = Keycode::Insert as i32,
    Home = Keycode::Home as i32,
    PageUp = Keycode::PageUp as i32,
    End = Keycode::End as i32,
    PageDown = Keycode::PageDown as i32,
    Right = Keycode::Right as i32,
    Left = Keycode::Left as i32,
    Down = Keycode::Down as i32,
    Up = Keycode::Up as i32,
    NumLockClear = Keycode::NumLockClear as i32,
    KpDivide = Keycode::KpDivide as i32,
    KpMultiply = Keycode::KpMultiply as i32,
    KpMinus = Keycode::KpMinus as i32,
    KpPlus = Keycode::KpPlus as i32,
    KpEnter = Keycode::KpEnter as i32,
    Kp1 = Keycode::Kp1 as i32,
    Kp2 = Keycode::Kp2 as i32,
    Kp3 = Keycode::Kp3 as i32,
    Kp4 = Keycode::Kp4 as i32,
    Kp5 = Keycode::Kp5 as i32,
    Kp6 = Keycode::Kp6 as i32,
    Kp7 = Keycode::Kp7 as i32,
    Kp8 = Keycode::Kp8 as i32,
    Kp9 = Keycode::Kp9 as i32,
    Kp0 = Keycode::Kp0 as i32,
    KpPeriod = Keycode::KpPeriod as i32,
    Application = Keycode::Application as i32,
    Power = Keycode::Power as i32,
    KpEquals = Keycode::KpEquals as i32,
    F13 = Keycode::F13 as i32,
    F14 = Keycode::F14 as i32,
    F15 = Keycode::F15 as i32,
    F16 = Keycode::F16 as i32,
    F17 = Keycode::F17 as i32,
    F18 = Keycode::F18 as i32,
    F19 = Keycode::F19 as i32,
    F20 = Keycode::F20 as i32,
    F21 = Keycode::F21 as i32,
    F22 = Keycode::F22 as i32,
    F23 = Keycode::F23 as i32,
    F24 = Keycode::F24 as i32,
    Execute = Keycode::Execute as i32,
    Help = Keycode::Help as i32,
    Menu = Keycode::Menu as i32,
    Select = Keycode::Select as i32,
    Stop = Keycode::Stop as i32,
    Again = Keycode::Again as i32,
    Undo = Keycode::Undo as i32,
    Cut = Keycode::Cut as i32,
    Copy = Keycode::Copy as i32,
    Paste = Keycode::Paste as i32,
    Find = Keycode::Find as i32,
    Mute = Keycode::Mute as i32,
    VolumeUp = Keycode::VolumeUp as i32,
    VolumeDown = Keycode::VolumeDown as i32,
    KpComma = Keycode::KpComma as i32,
    KpEqualsAS400 = Keycode::KpEqualsAS400 as i32,
    AltErase = Keycode::AltErase as i32,
    Sysreq = Keycode::Sysreq as i32,
    Cancel = Keycode::Cancel as i32,
    Clear = Keycode::Clear as i32,
    Prior = Keycode::Prior as i32,
    Return2 = Keycode::Return2 as i32,
    Separator = Keycode::Separator as i32,
    Out = Keycode::Out as i32,
    Oper = Keycode::Oper as i32,
    ClearAgain = Keycode::ClearAgain as i32,
    CrSel = Keycode::CrSel as i32,
    ExSel = Keycode::ExSel as i32,
    Kp00 = Keycode::Kp00 as i32,
    Kp000 = Keycode::Kp000 as i32,
    ThousandsSeparator = Keycode::ThousandsSeparator as i32,
    DecimalSeparator = Keycode::DecimalSeparator as i32,
    CurrencyUnit = Keycode::CurrencyUnit as i32,
    CurrencySubUnit = Keycode::CurrencySubUnit as i32,
    KpLeftParen = Keycode::KpLeftParen as i32,
    KpRightParen = Keycode::KpRightParen as i32,
    KpLeftBrace = Keycode::KpLeftBrace as i32,
    KpRightBrace = Keycode::KpRightBrace as i32,
    KpTab = Keycode::KpTab as i32,
    KpBackspace = Keycode::KpBackspace as i32,
    KpA = Keycode::KpA as i32,
    KpB = Keycode::KpB as i32,
    KpC = Keycode::KpC as i32,
    KpD = Keycode::KpD as i32,
    KpE = Keycode::KpE as i32,
    KpF = Keycode::KpF as i32,
    KpXor = Keycode::KpXor as i32,
    KpPower = Keycode::KpPower as i32,
    KpPercent = Keycode::KpPercent as i32,
    KpLess = Keycode::KpLess as i32,
    KpGreater = Keycode::KpGreater as i32,
    KpAmpersand = Keycode::KpAmpersand as i32,
    KpDblAmpersand = Keycode::KpDblAmpersand as i32,
    KpVerticalBar = Keycode::KpVerticalBar as i32,
    KpDblVerticalBar = Keycode::KpDblVerticalBar as i32,
    KpColon = Keycode::KpColon as i32,
    KpHash = Keycode::KpHash as i32,
    KpSpace = Keycode::KpSpace as i32,
    KpAt = Keycode::KpAt as i32,
    KpExclam = Keycode::KpExclam as i32,
    KpMemStore = Keycode::KpMemStore as i32,
    KpMemRecall = Keycode::KpMemRecall as i32,
    KpMemClear = Keycode::KpMemClear as i32,
    KpMemAdd = Keycode::KpMemAdd as i32,
    KpMemSubtract = Keycode::KpMemSubtract as i32,
    KpMemMultiply = Keycode::KpMemMultiply as i32,
    KpMemDivide = Keycode::KpMemDivide as i32,
    KpPlusMinus = Keycode::KpPlusMinus as i32,
    KpClear = Keycode::KpClear as i32,
    KpClearEntry = Keycode::KpClearEntry as i32,
    KpBinary = Keycode::KpBinary as i32,
    KpOctal = Keycode::KpOctal as i32,
    KpDecimal = Keycode::KpDecimal as i32,
    KpHexadecimal = Keycode::KpHexadecimal as i32,
    LCtrl = Keycode::LCtrl as i32,
    LShift = Keycode::LShift as i32,
    LAlt = Keycode::LAlt as i32,
    LGui = Keycode::LGui as i32,
    RCtrl = Keycode::RCtrl as i32,
    RShift = Keycode::RShift as i32,
    RAlt = Keycode::RAlt as i32,
    RGui = Keycode::RGui as i32,
    Mode = Keycode::Mode as i32,
    AudioNext = Keycode::AudioNext as i32,
    AudioPrev = Keycode::AudioPrev as i32,
    AudioStop = Keycode::AudioStop as i32,
    AudioPlay = Keycode::AudioPlay as i32,
    AudioMute = Keycode::AudioMute as i32,
    MediaSelect = Keycode::MediaSelect as i32,
    Www = Keycode::Www as i32,
    Mail = Keycode::Mail as i32,
    Calculator = Keycode::Calculator as i32,
    Computer = Keycode::Computer as i32,
    AcSearch = Keycode::AcSearch as i32,
    AcHome = Keycode::AcHome as i32,
    AcBack = Keycode::AcBack as i32,
    AcForward = Keycode::AcForward as i32,
    AcStop = Keycode::AcStop as i32,
    AcRefresh = Keycode::AcRefresh as i32,
    AcBookmarks = Keycode::AcBookmarks as i32,
    BrightnessDown = Keycode::BrightnessDown as i32,
    BrightnessUp = Keycode::BrightnessUp as i32,
    DisplaySwitch = Keycode::DisplaySwitch as i32,
    KbdIllumToggle = Keycode::KbdIllumToggle as i32,
    KbdIllumDown = Keycode::KbdIllumDown as i32,
    KbdIllumUp = Keycode::KbdIllumUp as i32,
    Eject = Keycode::Eject as i32,
    Sleep = Keycode::Sleep as i32,
}



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
	pub window_width: i32,
	pub window_height: i32,
	pub vsync: bool,

	pub timer: MyTimer,



	pub key_downs: [u8; 512],
	pub key_downs_previous: [u8; 512],
	pub key_half_count: [u8; 512],

	pub mouse_x: i32,
	pub mouse_y: i32,
	pub mouse_b: i32,

	pub quit: bool,
	pub resized: bool,


	pub _sdl: sdl2::Sdl,
	pub video: sdl2::VideoSubsystem,
	sdl_timer: sdl2::TimerSubsystem,
	_window: sdl2::video::Window,
	event_pump: sdl2::EventPump,

	//gl: *const std::os::raw::c_void,
	pub _gl_context: sdl2::video::GLContext,

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
		let mut t = Self{ window_width, window_height, vsync: vsync,
			timer: MyTimer{ now_stamp: sdl_timer.performance_counter(),
				 last_stamp: sdl_timer.performance_counter(), 
				 perf_freq: sdl_timer.performance_frequency() as f64, 
				 dt: 0.0f64 
			},
			_sdl: sdl, video, sdl_timer, _window : window, event_pump, _gl_context, 
			key_downs: [0; 512], key_downs_previous: [0; 512], key_half_count: [0; 512],
			mouse_x: 0, mouse_y: 0, mouse_b: 0,
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

	pub fn was_pressed(&self, key_code: MyKey) -> bool
	{
		let index = get_usize_from_mykey(key_code);
		return index < 512 && ((self.key_downs[index] == 1u8 && self.key_downs_previous[index] == 0u8 ) || self.key_half_count[index] >= 2u8);
	}

	pub fn was_released(&self, key_code: MyKey) -> bool
	{
		let index = get_usize_from_mykey(key_code);
		return index < 512 && ((self.key_downs[index] == 0u8 && self.key_downs_previous[index] == 1u8 ) || self.key_half_count[index] >= 2u8);
	}

	pub fn is_down(&self, key_code: MyKey)  -> bool
	{
		let index = get_usize_from_mykey(key_code);
		return index < 512 && self.key_downs[index] == 1u8;
	}

	pub fn swap_buffer(&mut self)
	{
		self._window.gl_swap_window();
	}

	pub fn set_window_title(&mut self, title:&String )
	{
		self._window.set_title(&title).unwrap();
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
							self.resized = self.window_width != width || self.window_height != height;
							self.window_width = width;
							self.window_height = height;
						},

						_ => {}
					}
				},


				Event::MouseButtonDown { mouse_btn, x, y, .. } =>
				{
					self.mouse_x = x;
					self.mouse_y = self.window_height as i32 - y;
					if mouse_btn == MouseButton::Left
					{
						self.mouse_b |= 1;
					}
					else if mouse_btn == MouseButton::Right
					{
						self.mouse_b |= 2;
					}
				},
				Event::MouseButtonUp { mouse_btn, x, y, .. } =>
				{
					self.mouse_x = x;
					self.mouse_y = self.window_height as i32 - y;
					if mouse_btn == MouseButton::Left
					{
						self.mouse_b &= !1;
					}
					else if mouse_btn == MouseButton::Right
					{
						self.mouse_b &= !2;
					}
				},
				Event::MouseMotion { x, y, .. } =>
				{
					self.mouse_x = x;
					self.mouse_y = self.window_height - y;
				},

				_ => {}
			}
		}
	}
}
