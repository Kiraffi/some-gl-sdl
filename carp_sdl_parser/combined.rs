#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
//Enums 
//File: include/SDL.h
//File: include/SDL_video.h
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
//File: include/SDL_mouse.h
//File: include/SDL_main.h
//File: include/SDL_keyboard.h
//File: include/SDL_joystick.h
//File: include/SDL_scancode.h

//Structs 
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Event
{
    sdl_type: u32,
    sdl_timestamp: u32,
    _padding: [u8; 56]
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Window { _something: [u8; 0] }

//File: include/SDL.h
//File: include/SDL_video.h
//File: include/SDL_events.h
//File: include/SDL_mouse.h
//File: include/SDL_main.h
//File: include/SDL_keyboard.h
//File: include/SDL_joystick.h
//File: include/SDL_scancode.h

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

}
