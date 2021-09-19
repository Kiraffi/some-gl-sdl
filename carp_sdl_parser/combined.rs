 //Defines 
//File: include/SDL.h
pub const SDL_INIT_TIMER: u32 = 0x00000001 as u32;
pub const SDL_INIT_VIDEO: u32 = 0x00000020 as u32;
pub const SDL_INIT_EVENTS: u32 = 0x00004000 as u32;
//File: include/SDL_video.h
//File: include/SDL_events.h
//File: include/SDL_mouse.h
//File: include/SDL_main.h
//File: include/SDL_keyboard.h
//File: include/SDL_joystick.h
//File: include/SDL_scancode.h
//File: include/SDL_keycode.h
//File: include/SDL_error.h

//Enums 
//File: include/SDL.h
//File: include/SDL_video.h
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SDL_WindowFlags
{
	SDL_WINDOW_FULLSCREEN = 0x00000001,
	SDL_WINDOW_OPENGL = 0x00000002,
	SDL_WINDOW_SHOWN = 0x00000004,
	SDL_WINDOW_HIDDEN = 0x00000008,
	SDL_WINDOW_BORDERLESS = 0x00000010,
	SDL_WINDOW_RESIZABLE = 0x00000020,
	SDL_WINDOW_MINIMIZED = 0x00000040,
	SDL_WINDOW_MAXIMIZED = 0x00000080,
	SDL_WINDOW_MOUSE_GRABBED = 0x00000100,
	SDL_WINDOW_INPUT_FOCUS = 0x00000200,
	SDL_WINDOW_MOUSE_FOCUS = 0x00000400,
	SDL_WINDOW_FOREIGN = 0x00000800,
	SDL_WINDOW_ALLOW_HIGHDPI = 0x00002000,
	SDL_WINDOW_MOUSE_CAPTURE    = 0x00004000,
	SDL_WINDOW_ALWAYS_ON_TOP    = 0x00008000,
	SDL_WINDOW_SKIP_TASKBAR     = 0x00010000,
	SDL_WINDOW_UTILITY          = 0x00020000,
	SDL_WINDOW_TOOLTIP          = 0x00040000,
	SDL_WINDOW_POPUP_MENU       = 0x00080000,
	SDL_WINDOW_KEYBOARD_GRABBED = 0x00100000,
	SDL_WINDOW_VULKAN           = 0x10000000,
	SDL_WINDOW_METAL            = 0x20000000,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SDL_GLattr
{
	SDL_GL_RED_SIZE,
	SDL_GL_GREEN_SIZE,
	SDL_GL_BLUE_SIZE,
	SDL_GL_ALPHA_SIZE,
	SDL_GL_BUFFER_SIZE,
	SDL_GL_DOUBLEBUFFER,
	SDL_GL_DEPTH_SIZE,
	SDL_GL_STENCIL_SIZE,
	SDL_GL_ACCUM_RED_SIZE,
	SDL_GL_ACCUM_GREEN_SIZE,
	SDL_GL_ACCUM_BLUE_SIZE,
	SDL_GL_ACCUM_ALPHA_SIZE,
	SDL_GL_STEREO,
	SDL_GL_MULTISAMPLEBUFFERS,
	SDL_GL_MULTISAMPLESAMPLES,
	SDL_GL_ACCELERATED_VISUAL,
	SDL_GL_RETAINED_BACKING,
	SDL_GL_CONTEXT_MAJOR_VERSION,
	SDL_GL_CONTEXT_MINOR_VERSION,
	SDL_GL_CONTEXT_EGL,
	SDL_GL_CONTEXT_FLAGS,
	SDL_GL_CONTEXT_PROFILE_MASK,
	SDL_GL_SHARE_WITH_CURRENT_CONTEXT,
	SDL_GL_FRAMEBUFFER_SRGB_CAPABLE,
	SDL_GL_CONTEXT_RELEASE_BEHAVIOR,
	SDL_GL_CONTEXT_RESET_NOTIFICATION,
	SDL_GL_CONTEXT_NO_ERROR,
}

//File: include/SDL_events.h
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SDL_EventType
{
	SDL_FIRSTEVENT     = 0,
	SDL_QUIT           = 0x100,
	SDL_APP_TERMINATING,
	SDL_APP_LOWMEMORY,
	SDL_APP_WILLENTERBACKGROUND,
	SDL_APP_DIDENTERBACKGROUND,
	SDL_APP_WILLENTERFOREGROUND,
	SDL_APP_DIDENTERFOREGROUND,
	SDL_LOCALECHANGED,
	SDL_DISPLAYEVENT   = 0x150,
	SDL_WINDOWEVENT    = 0x200,
	SDL_SYSWMEVENT,
	SDL_KEYDOWN        = 0x300,
	SDL_KEYUP,
	SDL_TEXTEDITING,
	SDL_TEXTINPUT,
	SDL_KEYMAPCHANGED,
	SDL_MOUSEMOTION    = 0x400,
	SDL_MOUSEBUTTONDOWN,
	SDL_MOUSEBUTTONUP,
	SDL_MOUSEWHEEL,
	SDL_JOYAXISMOTION  = 0x600,
	SDL_JOYBALLMOTION,
	SDL_JOYHATMOTION,
	SDL_JOYBUTTONDOWN,
	SDL_JOYBUTTONUP,
	SDL_JOYDEVICEADDED,
	SDL_JOYDEVICEREMOVED,
	SDL_CONTROLLERAXISMOTION  = 0x650,
	SDL_CONTROLLERBUTTONDOWN,
	SDL_CONTROLLERBUTTONUP,
	SDL_CONTROLLERDEVICEADDED,
	SDL_CONTROLLERDEVICEREMOVED,
	SDL_CONTROLLERDEVICEREMAPPED,
	SDL_CONTROLLERTOUCHPADDOWN,
	SDL_CONTROLLERTOUCHPADMOTION,
	SDL_CONTROLLERTOUCHPADUP,
	SDL_CONTROLLERSENSORUPDATE,
	SDL_FINGERDOWN      = 0x700,
	SDL_FINGERUP,
	SDL_FINGERMOTION,
	SDL_DOLLARGESTURE   = 0x800,
	SDL_DOLLARRECORD,
	SDL_MULTIGESTURE,
	SDL_CLIPBOARDUPDATE = 0x900,
	SDL_DROPFILE        = 0x1000,
	SDL_DROPTEXT,
	SDL_DROPBEGIN,
	SDL_DROPCOMPLETE,
	SDL_AUDIODEVICEADDED = 0x1100,
	SDL_AUDIODEVICEREMOVED,
	SDL_SENSORUPDATE = 0x1200,
	SDL_RENDER_TARGETS_RESET = 0x2000,
	SDL_RENDER_DEVICE_RESET,
	SDL_USEREVENT    = 0x8000,
	SDL_LASTEVENT    = 0xFFFF,
}

//File: include/SDL_mouse.h
//File: include/SDL_main.h
//File: include/SDL_keyboard.h
//File: include/SDL_joystick.h
//File: include/SDL_scancode.h
//File: include/SDL_keycode.h
//File: include/SDL_error.h

//Structs 
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Event
{
    pub sdl_type: SDL_EventType,
    pub sdl_timestamp: u32,
    pub _padding: [u8; 56]
}

//File: include/SDL.h
//File: include/SDL_video.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Window
{
	 pub _something: [u8; 0]
}

//File: include/SDL_events.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_KeyboardEvent
{
	 pub type_name: u32,
	 pub timestamp: u32,
	 pub windowID: u32,
	 pub state: u8,
	 pub repeat: u8,
	 pub padding2: u8,
	 pub padding3: u8,
	 pub keysym: SDL_Keysym,
}

//File: include/SDL_mouse.h
//File: include/SDL_main.h
//File: include/SDL_keyboard.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Keysym
{
	 pub scancode: SDL_Scancode,
	 pub sym: SDL_Keycode,
	 pub mod_name: u16,
	 pub unused: u32,
}

//File: include/SDL_joystick.h
//File: include/SDL_scancode.h
//File: include/SDL_keycode.h
//File: include/SDL_error.h

//Classes 
extern "C"
{
	//File: include/SDL.h
	pub fn SDL_Init(flags: u32, 	) -> i32;
	pub fn SDL_InitSubSystem(flags: u32, 	) -> i32;
	pub fn SDL_QuitSubSystem(flags: u32, 	) -> ();
	pub fn SDL_WasInit(flags: u32, 	) -> u32;
	pub fn SDL_Quit(	) -> ();

	//File: include/SDL_video.h
	pub fn SDL_CreateWindow(title: *const c_char, x: i32, y: i32, w: i32, h: i32, flags: u32, 	) -> *mut SDL_Window;
	pub fn SDL_DestroyWindow(window: *mut SDL_Window, 	) -> ();
	pub fn SDL_GL_GetProcAddress(proc: *const c_char, 	) -> *mut ();
	pub fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: i32, 	) -> i32;
	pub fn SDL_GL_CreateContext(window: *mut SDL_Window, 	) -> SDL_GLContext;
	pub fn SDL_GL_SetSwapInterval(interval: i32, 	) -> i32;
	pub fn SDL_GL_GetSwapInterval(	) -> i32;
	pub fn SDL_GL_SwapWindow(window: *mut SDL_Window, 	) -> ();

	//File: include/SDL_events.h
	pub fn SDL_PumpEvents(	) -> ();
	pub fn SDL_PollEvent(event: *mut SDL_Event, 	) -> i32;

	//File: include/SDL_mouse.h

	//File: include/SDL_main.h

	//File: include/SDL_keyboard.h

	//File: include/SDL_joystick.h

	//File: include/SDL_scancode.h

	//File: include/SDL_keycode.h

	//File: include/SDL_error.h
	pub fn SDL_GetError(	) -> *const c_char;

}
