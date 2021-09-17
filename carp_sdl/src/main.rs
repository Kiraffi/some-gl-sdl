#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::os::raw::*;


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







#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Event 
{
    sdl_type: u32, 
    _unused: [u8; 52] 
}





pub const  SDL_RELEASED:u32 = 0;
pub const SDL_PRESSED:u32 = 1;

pub const SDL_FIRSTEVENT: u32 = 0x000;

/* Application events */
pub const SDL_QUIT: u32                    = 0x100;
pub const SDL_APP_TERMINATING: u32         = 0x101;
pub const SDL_APP_LOWMEMORY: u32           = 0x102;
pub const SDL_APP_WILLENTERBACKGROUND: u32 = 0x103;
pub const SDL_APP_DIDENTERBACKGROUND: u32  = 0x104;
pub const SDL_APP_WILLENTERFOREGROUND: u32 = 0x105;
pub const SDL_APP_DIDENTERFOREGROUND: u32  = 0x106;
pub const SDL_LOCALECHANGED: u32           = 0x107;

/* Display events */
pub const SDL_DISPLAYEVENT: u32 = 0x150;

/* Window events */
pub const SDL_WINDOWEVENT: u32 = 0x200;
pub const SDL_SYSWMEVENT: u32  = 0x201;

/* Keyboard events */
pub const SDL_KEYDOWN: u32       = 0x300;
pub const SDL_KEYUP: u32         = 0x301;
pub const SDL_TEXTEDITING: u32   = 0x302;
pub const SDL_TEXTINPUT: u32     = 0x303;
pub const SDL_KEYMAPCHANGED: u32 = 0x304;

/* Mouse events */
pub const SDL_MOUSEMOTION: u32     = 0x400;
pub const SDL_MOUSEBUTTONDOWN: u32 = 0x401;
pub const SDL_MOUSEBUTTONUP: u32   = 0x402;
pub const SDL_MOUSEWHEEL: u32      = 0x403;

/* Joystick events */
pub const SDL_JOYAXISMOTION: u32     = 0x600;
pub const SDL_JOYBALLMOTION: u32     = 0x601;
pub const SDL_JOYHATMOTION: u32      = 0x602;
pub const SDL_JOYBUTTONDOWN: u32     = 0x603;
pub const SDL_JOYBUTTONUP: u32       = 0x604;
pub const SDL_JOYDEVICEADDED: u32    = 0x605;
pub const SDL_JOYDEVICEREMOVED: u32  = 0x606;

/* Game controller events */
pub const SDL_CONTROLLERAXISMOTION: u32     = 0x650;
pub const SDL_CONTROLLERBUTTONDOWN: u32     = 0x651;
pub const SDL_CONTROLLERBUTTONUP: u32       = 0x652;
pub const SDL_CONTROLLERDEVICEADDED: u32    = 0x653;
pub const SDL_CONTROLLERDEVICEREMOVED: u32  = 0x654;
pub const SDL_CONTROLLERDEVICEREMAPPED: u32 = 0x655;
pub const SDL_CONTROLLERTOUCHPADDOWN: u32   = 0x656;
pub const SDL_CONTROLLERTOUCHPADMOTION: u32 = 0x657;
pub const SDL_CONTROLLERTOUCHPADUP: u32     = 0x658;
pub const SDL_CONTROLLERSENSORUPDATE: u32   = 0x659;

/* Touch events */
pub const SDL_FINGERDOWN: u32   = 0x700;
pub const SDL_FINGERUP: u32     = 0x701;
pub const SDL_FINGERMOTION: u32 = 0x702;

/* Gesture events */
pub const SDL_DOLLARGESTURE: u32 = 0x800;
pub const SDL_DOLLARRECORD: u32  = 0x801;
pub const SDL_MULTIGESTURE: u32  = 0x802;

/* Clipboard events */
pub const SDL_CLIPBOARDUPDATE: u32 = 0x900;

/* Drag and drop events */
pub const SDL_DROPFILE: u32     = 0x1000;
pub const SDL_DROPTEXT: u32     = 0x1001;
pub const SDL_DROPBEGIN: u32    = 0x1002;
pub const SDL_DROPCOMPLETE: u32 = 0x1003;

/* Audio hotplug events */
pub const SDL_AUDIODEVICEADDED: u32   = 0x1100;
pub const SDL_AUDIODEVICEREMOVED: u32 = 0x1101;

/* Sensor events */
pub const SDL_SENSORUPDATE: u32 = 0x1200;

/* Render events */
pub const SDL_RENDER_TARGETS_RESET: u32 = 0x2000;
pub const SDL_RENDER_DEVICE_RESET: u32  = 0x2001;

pub const SDL_USEREVENT: u32 = 0x8000;

pub const SDL_LASTEVENT: u32 = 0xFFFF;






#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Window { _unused: [u8; 0] }


extern "C" 
{
    pub fn SDL_Init(flags: u32) -> c_int;
    pub fn SDL_InitSubSystem(flags: u32) -> c_int;
    pub fn SDL_QuitSubSystem(flags: u32);
    pub fn SDL_WasInit(flags: u32) -> u32;
    pub fn SDL_Quit();


    pub fn SDL_CreateWindow(title: *const c_char, x: c_int, y: c_int, w: c_int, h: c_int, flags: u32) -> *mut SDL_Window;
    pub fn SDL_DestroyWindow(window : *mut SDL_Window);
    pub fn SDL_GL_CreateContext(window: *mut SDL_Window) -> SDL_GLContext;
    pub fn SDL_GetError() -> *const c_char;
//
    pub fn SDL_GL_SetAttribute(attr: u32 , value: c_int) -> c_int;
    pub fn SDL_GL_GetSwapInterval() -> c_int;
    pub fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int;


    pub fn SDL_PollEvent(event: *mut SDL_Event) -> c_int;
    pub fn SDL_GL_SwapWindow(window: *mut SDL_Window );
    pub fn SDL_PumpEvents();

    fn SDL_GL_GetProcAddress(proc_: *const std::os::raw::c_char) -> *mut std::os::raw::c_void;
}



pub const SDL_GL_RED_SIZE: u32                   = 0;
pub const SDL_GL_GREEN_SIZE: u32                 = 1;
pub const SDL_GL_BLUE_SIZE: u32                  = 2;
pub const SDL_GL_ALPHA_SIZE: u32                 = 3;
pub const SDL_GL_BUFFER_SIZE: u32                = 4;
pub const SDL_GL_DOUBLEBUFFER: u32               = 5;
pub const SDL_GL_DEPTH_SIZE: u32                 = 6;
pub const SDL_GL_STENCIL_SIZE: u32               = 7;
pub const SDL_GL_ACCUM_RED_SIZE: u32             = 8;
pub const SDL_GL_ACCUM_GREEN_SIZE: u32           = 9;
pub const SDL_GL_ACCUM_BLUE_SIZE: u32            = 10;
pub const SDL_GL_ACCUM_ALPHA_SIZE: u32           = 11;
pub const SDL_GL_STEREO: u32                     = 12;
pub const SDL_GL_MULTISAMPLEBUFFERS: u32         = 13;
pub const SDL_GL_MULTISAMPLESAMPLES: u32         = 14;
pub const SDL_GL_ACCELERATED_VISUAL: u32         = 15;
pub const SDL_GL_RETAINED_BACKING: u32           = 16;
pub const SDL_GL_CONTEXT_MAJOR_VERSION: u32      = 17;
pub const SDL_GL_CONTEXT_MINOR_VERSION: u32      = 18;
pub const SDL_GL_CONTEXT_EGL: u32                = 19;
pub const SDL_GL_CONTEXT_FLAGS: u32              = 20;
pub const SDL_GL_CONTEXT_PROFILE_MASK: u32       = 21;
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: u32 = 22;
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: u32   = 23;
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: u32   = 24;
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: u32 = 25;
pub const SDL_GL_CONTEXT_NO_ERROR: u32           = 26; 


type SDL_GLContext = *const c_void;

pub const SDL_INIT_TIMER: u32 = 1;
pub const SDL_INIT_AUDIO: u32 = 16;
pub const SDL_INIT_VIDEO: u32 = 32;
pub const SDL_INIT_JOYSTICK: u32 = 512;
pub const SDL_INIT_HAPTIC: u32 = 4096;
pub const SDL_INIT_GAMECONTROLLER: u32 = 8192;
pub const SDL_INIT_EVENTS: u32 = 16384;
pub const SDL_INIT_SENSOR: u32 = 32768;
pub const SDL_INIT_NOPARACHUTE: u32 = 1048576;
pub const SDL_INIT_EVERYTHING: u32 = 62001;

pub const SDL_WINDOWPOS_UNDEFINED_MASK: u32 = 0x1FFF0000u32;
pub const fn SDL_WINDOWPOS_UNDEFINED_DISPLAY(X: u32) -> u32 { SDL_WINDOWPOS_UNDEFINED_MASK|(X) }
pub const SDL_WINDOWPOS_UNDEFINED: u32 = SDL_WINDOWPOS_UNDEFINED_DISPLAY(0);
pub fn SDL_WINDOWPOS_ISUNDEFINED(X: u32) -> bool { ((X)&0xFFFF0000) == SDL_WINDOWPOS_UNDEFINED_MASK }



//typedef enum{
pub const SDL_WINDOW_FULLSCREEN: u32 = 0x00000001;         /**< fullscreen window */
pub const SDL_WINDOW_OPENGL: u32 = 0x00000002;             /**< window usable with OpenGL context */
pub const SDL_WINDOW_SHOWN: u32 = 0x00000004;              /**< window is visible */
pub const SDL_WINDOW_HIDDEN: u32 = 0x00000008;             /**< window is not visible */
pub const SDL_WINDOW_BORDERLESS: u32 = 0x00000010;         /**< no window decoration */
pub const SDL_WINDOW_RESIZABLE: u32 = 0x00000020;          /**< window can be resized */
pub const SDL_WINDOW_MINIMIZED: u32 = 0x00000040;          /**< window is minimized */
pub const SDL_WINDOW_MAXIMIZED: u32 = 0x00000080;          /**< window is maximized */
pub const SDL_WINDOW_MOUSE_GRABBED: u32 = 0x00000100;      /**< window has grabbed mouse input */
pub const SDL_WINDOW_INPUT_FOCUS: u32 = 0x00000200;        /**< window has input focus */
pub const SDL_WINDOW_MOUSE_FOCUS: u32 = 0x00000400;        /**< window has mouse focus */
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: u32 = SDL_WINDOW_FULLSCREEN + 0x00001000;
pub const SDL_WINDOW_FOREIGN: u32 = 0x00000800;            /**< window not created by SDL */
pub const SDL_WINDOW_ALLOW_HIGHDPI: u32 = 0x00002000;      /**< window should be created in high-DPI mode if supported.
                                                 On macOS NSHighResolutionCapable must be set true in the
                                                 application's Info.plist for this to have any effect. */
pub const SDL_WINDOW_MOUSE_CAPTURE: u32    = 0x00004000;   /**< window has mouse captured (unrelated to MOUSE_GRABBED) */
pub const SDL_WINDOW_ALWAYS_ON_TOP: u32    = 0x00008000;   /**< window should always be above others */
pub const SDL_WINDOW_SKIP_TASKBAR: u32     = 0x00010000;   /**< window should not be added to the taskbar */
pub const SDL_WINDOW_UTILITY: u32          = 0x00020000;   /**< window should be treated as a utility window */
pub const SDL_WINDOW_TOOLTIP: u32          = 0x00040000;   /**< window should be treated as a tooltip */
pub const SDL_WINDOW_POPUP_MENU: u32       = 0x00080000;   /**< window should be treated as a popup menu */
pub const SDL_WINDOW_KEYBOARD_GRABBED: u32 = 0x00100000;   /**< window has grabbed keyboard input */
pub const SDL_WINDOW_VULKAN: u32           = 0x10000000;   /**< window usable for Vulkan surface */
pub const SDL_WINDOW_METAL: u32            = 0x20000000;   /**< window usable for Metal view */
pub const SDL_WINDOW_INPUT_GRABBED: u32 = SDL_WINDOW_MOUSE_GRABBED; /**< equivalent to SDL_WINDOW_MOUSE_GRABBED for compatibility */
//} SDL_WindowFlags;



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
       
       SDL_GL_SetAttribute( SDL_GL_CONTEXT_MAJOR_VERSION, 4 );
       SDL_GL_SetAttribute( SDL_GL_CONTEXT_MINOR_VERSION, 6 );

       //Create window
       let gWindow = SDL_CreateWindow( b"SDL Tutorial\0".as_ptr() as *const i8, 
        SDL_WINDOWPOS_UNDEFINED as i32, SDL_WINDOWPOS_UNDEFINED as i32, width, height, SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN );
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
            //SDL_PumpEvents();
            let mut polling: c_int = 1;
            let mut sdl_event = SDL_Event{ sdl_type: 0, _unused: [0; 52] };
            while polling > 0 && !quit
            {
                polling = SDL_PollEvent(&mut sdl_event);
                if sdl_event.sdl_type != 0
                {
                   match sdl_event.sdl_type
                   {
                        SDL_QUIT => quit = true,
                       _ => ()
                   }
                }
            }
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


fn main() {
    match test_sdl()
    {
        Ok(_) => (),
        Err(s) => println!("Failure sdl: {:?}", &s)
    }

}
