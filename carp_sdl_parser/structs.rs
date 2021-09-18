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
