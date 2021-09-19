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