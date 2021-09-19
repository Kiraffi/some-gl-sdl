#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::os::raw::*;
pub mod fn_test;
use fn_test::*;

type SDL_GLContext = *const c_void;

fn get_usize_from_keycode(keycode: u32) -> usize
{
	let mut val:u32 = keycode; 
	if (val & 0x40000000) == 0x40000000 
	{
		val =  (val & 0xffffu32) + 128u32;
	}
	
	return val as usize;
}


pub enum empty_enum_type {}
pub type HINSTANCE = *mut empty_enum_type;
pub type FARPROC = HINSTANCE;
pub type HMODULE = HINSTANCE;
pub type LPCSTR = *const std::os::raw::c_char;
pub type BOOL = std::os::raw::c_int;

extern "system" 
{
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
    pub fn LoadLibraryA(lpFileName: LPCSTR) -> HMODULE;
    pub fn FreeLibrary(hLibModule: HMODULE) -> BOOL;
}


pub fn sdl_proc_address(procname: &str) -> *const () 
{
	match std::ffi::CString::new(procname) 
    {
		Ok(procname) => unsafe 
        {
			SDL_GL_GetProcAddress(procname.as_ptr() as *const std::os::raw::c_char) as *const ()
		},
		// string contains a nul byte - it won't match anything.
		Err(_) => std::ptr::null(),
	}
}

fn test_sdl() -> Result<(), &'static str>
{
    unsafe
    {
        if SDL_Init(SDL_INIT_VIDEO) < 0
        {
            return Err("failure to init sdl!");
        }

        let width = 640i32;
        let height  = 480i32;
       
       SDL_GL_SetAttribute( SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 4 );
       SDL_GL_SetAttribute( SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 6 );
        // 0x1FFF0000 windowpos undefined
       //Create window
       let window_flags: i32 = (SDL_WindowFlags::SDL_WINDOW_OPENGL as i32) | (SDL_WindowFlags::SDL_WINDOW_SHOWN as i32);
       let gWindow = SDL_CreateWindow( b"SDL Tutorial\0".as_ptr() as *const i8, 
       0x1FFF0000, 0x1FFF0000, width, height, window_flags as u32 );
       if gWindow == core::ptr::null_mut()
       {
           println!( "Window could not be created! SDL Error: {:?}", SDL_GetError() );
           return Err("Failed to create window");
       }




       //Create context
       let gContext = SDL_GL_CreateContext( gWindow );
       if gContext == core::ptr::null()
       {
        println!( "OpenGL context could not be created! SDL Error: {:?}", SDL_GetError() );
           return Err("Failed to create opengl context!");
       }

       //Use Vsync
        if SDL_GL_SetSwapInterval( 1 ) < 0
        {
            println!( "Warning: Unable to set VSync! SDL Error: {:?}", SDL_GetError() );
        }


        if !gl::load_with(&|s| sdl_proc_address(s) as *const _)
        {
            return Err("failed to load opengl functions!");
        }
		//gl::load_gl_funcs(_opengl32, _wglGetProcAddress.unwrap());
		//gl::load_gl_funcs(_opengl32, _wglGetProcAddress.unwrap());
		//gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
		//gl::glDebugMessageCallback(Some(gl_callback), std::ptr::null());
		//gl::glDebugMessageControl(gl::GL_DONT_CARE, gl::GL_DONT_CARE, gl::GL_DONT_CARE, 0,
		//							std::ptr::null(), gl::GL_TRUE as u8);
		gl::glEnable(gl::GL_DEBUG_OUTPUT);
	

        let version;
        match
        {
            let data = std::ffi::CStr::from_ptr(gl::glGetString(gl::GL_VERSION) as *const _)
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
                return Err("Failed to read version data from gl!");
            }
        }

        println!("OpenGL version {}", version);
            
        let mut font_system: render_systems::fontsystem::FontSystem = render_systems::fontsystem::FontSystem::init()?;
        let frame_data_buffer: render_gl::ShaderBuffer = render_gl::ShaderBuffer::new(
            gl::GL_UNIFORM_BUFFER,
            65536
        );
    
        let mut vao: gl::GLuint = 0;
        {
            gl::glGenVertexArrays(1, &mut vao);
        }
        /*
        //Initialize OpenGL
        if( !initGL() )
        {
            printf( "Unable to initialize OpenGL!\n" );
            success = false;
        }


        */  
        let letter_size = 16f32;

        let mut quit = false;
       
        let colors = my_core::get_u32_agbr_color(1.0, 1.0, 1.0, 1.0);

        //While application is running
        while !quit
        {

            quit = !update();
            let s: String = "High Score: ".to_string();
    		font_system.draw_string(&s, (width - 300) as f32, 5.0f32, letter_size as f32, letter_size as f32, colors);
		
            let tmp = render_gl::CommonShaderFrameDate::new(width, height);
			frame_data_buffer.write_data(0, std::mem::size_of::<render_gl::CommonShaderFrameDate>(), &tmp as *const _ as *const gl::GLvoid);

            gl::glBindVertexArray(vao);
			frame_data_buffer.bind(0);

			font_system.update(width as f32, height as f32);
			font_system.draw();
            //println!("Events done");
            
            //Render quad
            //render();
            
            //Update screen
            SDL_GL_SwapWindow( gWindow );
            let one_milli = std::time::Duration::from_millis(1);
            std::thread::sleep(one_milli);
        }
        // SDL_FreeSurface?
        SDL_DestroyWindow( gWindow );

        SDL_Quit();
    }

    Ok(())
} 


fn update() -> bool

{
            //SDL_PumpEvents();
            let mut polling: c_int = 1;
            let mut sdl_event = SDL_Event{ sdl_type: SDL_EventType::SDL_FIRSTEVENT, sdl_timestamp: 0, _padding: [0; 56] };
            while polling > 0
            {
                unsafe
                {
                    polling = SDL_PollEvent(&mut sdl_event);
                }
                if sdl_event.sdl_type != SDL_EventType::SDL_FIRSTEVENT
                {
                    println!("sdl event: {}", sdl_event.sdl_type as u32);
                    match sdl_event.sdl_type as SDL_EventType
                    {
                        SDL_EventType::SDL_QUIT => {println!("quitting"); return false},
                        SDL_EventType::SDL_KEYDOWN => 
                        {
                            unsafe 
                            {

                            }
                        },
/*


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


*/

























                _ => ()
            }
        }
    }

    return true;
}


fn main() {
    match test_sdl()
    {
        Ok(_) => (),
        Err(s) => println!("Failure sdl: {:?}", &s)
    }

}
