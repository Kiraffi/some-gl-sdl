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
