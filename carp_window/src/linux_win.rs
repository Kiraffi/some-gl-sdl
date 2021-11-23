#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::{os::raw::*, ptr::null};

pub type Atom = c_ulong;
pub type XID = c_ulong;
pub type Window = XID;
pub type Colormap = XID;
pub type Pixmap = XID;
pub type KeySym = XID;
pub type Cursor = XID;
pub type GLXDrawable = XID;


pub enum EMPTYTYPE {}
pub type XDisplay = *mut EMPTYTYPE;
pub type Visual = *mut EMPTYTYPE;

pub type GLXContext = *mut EMPTYTYPE;
pub type GLXFBConfig = *mut EMPTYTYPE;


pub enum Display {}
pub enum Screen {}

pub type VisualID = c_ulong;
pub type GLint = c_int;


#[link(name = "X11")]
#[link(name = "GL")]
extern "system"
{
    pub fn XInitThreads() -> c_int;
    pub fn XrmInitialize();

    pub fn XOpenDisplay(_: *const c_char) -> *mut Display;
    pub fn XResourceManagerString(_ :Display) -> *mut c_char;
    pub fn XInternAtom(_: *mut Display, _: *const c_char, _: c_int) -> Atom;

    pub fn XScreenOfDisplay(display: *mut Display, display_index: c_int) -> *mut Screen;
    pub fn XCreateSimpleWindow(display: *mut Display, window: Window, x: c_int, y: c_int,
        width: c_uint, height: c_uint, border_width: c_uint, bord: c_ulong, bg: c_ulong) -> Window;
    pub fn XCreateWindow(display: *mut Display, window: Window, x: c_int, y: c_int,
        width: c_uint, height: c_uint, border_width: c_uint, depth: c_int,
        cls: c_uint, visual: Visual, value_mask: c_ulong, win_attr: *const XSetWindowAttributes) -> Window;


    pub fn XRootWindow(display: *mut Display, screen_id: c_int) -> Window;
    pub fn XBlackPixel(display: *mut Display, screen_id: c_int) -> c_ulong;
    pub fn XWhitePixel(display: *mut Display, screen_id: c_int) -> c_ulong;

    pub fn XClearWindow(display: *mut Display, window: Window) -> c_int;
    pub fn XMapRaised(display: *mut Display, window: Window) -> c_int;
    pub fn XDestroyWindow(display: *mut Display, window: Window) -> c_int;

    pub fn XFree(data: *mut c_void) -> c_int;
    pub fn XCloseDisplay(display: *mut Display) -> c_int;
    pub fn XNextEvent(display: *mut Display, xevent: *mut XEvent) -> c_int;

    pub fn XDefaultScreen(display: *mut Display) -> c_int;
    pub fn XStoreName(display: *mut Display, window: Window, name: *const c_char) -> c_int;

    pub fn XSelectInput(display: *mut Display, window: Window, event_mask: c_long) -> c_int;


    pub fn XFreeColormap(display: *mut Display, colormap: Colormap) -> c_int;

    pub fn XLookupString(key_event: *mut XEvent, buffer: *mut c_char, bytes: c_int,
        keysym: *mut KeySym, comp: *const c_void /* *mut XComposeStatus*/) -> c_int;


    pub fn XSetWMProtocols(display: *mut Display, window: Window, protocols: *mut Atom, count: c_int) -> c_int;




    pub fn XDisplayHeight(display: *mut Display, screen_id: c_int) -> c_int;
    pub fn XDisplayHeightMM(display: *mut Display, screen_id: c_int) -> c_int;
    pub fn XDisplayPlanes(display: *mut Display, screen_id: c_int) -> c_int;
    pub fn XDisplayWidth(display: *mut Display, screen_id: c_int) -> c_int;
    pub fn XDisplayWidthMM(display: *mut Display, screen_id: c_int) -> c_int;



    pub fn XSetWindowColormap(display: *mut Display, window: Window, colormap: Colormap) -> c_int;
    pub fn XCreateColormap(display: *mut Display, window: Window, visual: Visual, alloc: c_int) -> Colormap;

    // GL lib
    pub fn glXQueryVersion(display: *mut Display, major_version: &mut GLint, minor_version: &mut GLint) -> GLint;
    pub fn glXChooseVisual(display: *mut Display, screen_id: c_int, attrib_list: *const GLint) -> *mut XVisualInfo;
    pub fn glXCreateContext(display: *mut Display, visual_info: *const XVisualInfo, share_list: GLXContext, direct: bool) -> GLXContext;
    pub fn glXDestroyContext(display: *mut Display, context: GLXContext);

    pub fn glXMakeCurrent(display: *mut Display, drawable: GLXDrawable, context: GLXContext) -> bool;
    pub fn glXSwapBuffers(display: *mut Display, drawable: GLXDrawable);
    pub fn glXChooseFBConfig(display: *mut Display, screen_id: c_int, attrib_list: *const GLint, items: &mut c_int) -> *const GLXFBConfig;
    pub fn glXGetVisualFromFBConfig(display: *mut Display, glx_fb_config: GLXFBConfig) -> *const XVisualInfo;

    pub fn glXGetFBConfigAttrib(display: *mut Display, glx_fb_config: GLXFBConfig, attr: c_int, value: &mut c_int) -> c_int;

    pub fn glXCreateNewContext(display: *mut Display, glx_fb_config: GLXFBConfig,
        render_type: c_int, share_list: GLXContext, direct: bool) -> GLXContext;

    pub fn glXGetProcAddress(procname: *const GLubyte) -> *mut c_void;

}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisualInfo
{
    pub visual: Visual,
    pub visualid: VisualID,
    pub screen: c_int,
    pub depth: c_int,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub colormap_size: c_int,
    pub bits_per_rgb: c_int,
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct XEvent
{
    pub pad: [c_long; 24],
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes
{
    pub background_pixmap: Pixmap,
    pub background_pixel: c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: c_ulong,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: c_int,
    pub event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}



pub const CWBackPixmap: c_ulong = 1 << 0;
pub const CWBackPixel: c_ulong = 1 << 1;
pub const CWBorderPixmap: c_ulong = 1 << 2;
pub const CWBorderPixel: c_ulong = 1 << 3;
pub const CWBitGravity: c_ulong = 1 << 4;
pub const CWWinGravity: c_ulong = 1<< 5;
pub const CWBackingStore: c_ulong = 1<< 6;
pub const CWBackingPlanes: c_ulong = 1 << 7;
pub const CWBackingPixel: c_ulong = 1 << 8;
pub const CWOverrideRedirect: c_ulong = 1 <<9;
pub const CWSaveUnder: c_ulong = 1 << 10;
pub const CWEventMask: c_ulong = 1 << 11;
pub const CWDontPropagate: c_ulong = 1 <<12;
pub const CWColormap: c_ulong = 1 << 13;
pub const CWCursor: c_ulong = 1 << 14;


pub const CWX: c_ulong = 1 << 0;
pub const CWY: c_ulong = 1 << 1;
pub const CWWidth: c_ulong = 1 << 2;
pub const CWHeight: c_ulong = 1 << 3;
pub const CWBorderWidth: c_ulong = 1 << 4;
pub const CWSibling: c_ulong = 1 << 5;
pub const CWStackMode: c_ulong = 1 << 6;

pub const InputOutput: c_uint = 1;



pub const NoEventMask: c_long = 0x0;
pub const KeyPressMask: c_long = 0x1;
pub const KeyReleaseMask: c_long = 0x2;
pub const ButtonPressMask: c_long = 0x4;
pub const ButtonReleaseMask: c_long = 0x8;
pub const EnterWindowMask: c_long = 0x10;
pub const LeaveWindowMask: c_long = 0x20;
pub const PointerMotionMask: c_long = 0x40;
pub const PointerMotionHintMask: c_long = 0x80;
pub const Button1MotionMask: c_long = 0x100;
pub const Button2MotionMask: c_long = 0x200;
pub const Button3MotionMask: c_long = 0x400;
pub const Button4MotionMask: c_long = 0x800;
pub const Button5MotionMask: c_long = 0x1000;
pub const ButtonMotionMask: c_long =  0x2000;
pub const KeymapStateMask: c_long = 0x4000;
pub const ExposureMask: c_long = 0x8000;
pub const VisibilityChangeMask: c_long = 0x1_0000;
pub const StructureNotifyMask: c_long = 0x2_0000;
pub const ResizeRedirectMask: c_long = 0x4_0000;
pub const SubstructureNotifyMask: c_long = 0x8_0000;
pub const SubstructureRedirectMask: c_long = 0x10_0000;
pub const FocusChangeMask: c_long = 0x20_0000;
pub const PropertyChangeMask: c_long = 0x40_0000;
pub const ColormapChangeMask: c_long = 0x80_0000;
pub const OwnerGrabButtonMask: c_long = 0x100_0000;

// Events
pub const KeyPress: c_int = 2;
pub const KeyRelease: c_int = 3;
pub const ButtonPress: c_int = 4;
pub const ButtonRelease: c_int = 5;
pub const MotionNotify: c_int = 6;
pub const EnterNotify: c_int = 7;
pub const LeaveNotify: c_int = 8;
pub const FocusIn: c_int = 9;
pub const FocusOut: c_int = 10;
pub const KeymapNotify: c_int = 11;
pub const Expose: c_int = 12;
pub const GraphicsExpose: c_int = 13;
pub const NoExpose: c_int = 14;
pub const VisibilityNotify: c_int = 15;
pub const CreateNotify: c_int = 16;
pub const DestroyNotify: c_int = 17;
pub const UnmapNotify: c_int = 18;
pub const MapNotify: c_int = 19;
pub const MapRequest: c_int = 20;
pub const ReparentNotify: c_int = 21;
pub const ConfigureNotify: c_int = 22;
pub const ConfigureRequest: c_int = 23;
pub const GravityNotify: c_int = 24;
pub const ResizeRequest: c_int = 25;
pub const CirculateNotify: c_int = 26;
pub const CirculateRequest: c_int = 27;
pub const PropertyNotify: c_int = 28;
pub const SelectionClear: c_int = 29;
pub const SelectionRequest: c_int = 30;
pub const SelectionNotify: c_int = 31;
pub const ColormapNotify: c_int = 32;
pub const ClientMessage: c_int = 33;
pub const MappingNotify: c_int = 34;
pub const GenericEvent: c_int = 35;
pub const LASTEvent: c_int = 36;	/* must be bigger than any event # */



pub const XK_Escape: KeySym = 0xff1b;


pub const AllocNone: c_int = 0;


pub const GLX_RGBA: GLint = 4;
pub const GLX_DOUBLEBUFFER: GLint = 5;



pub const GLX_AUX_BUFFERS: GLint = 7;
pub const GLX_RED_SIZE: GLint = 8;
pub const GLX_GREEN_SIZE: GLint = 9;
pub const GLX_BLUE_SIZE: GLint = 10;
pub const GLX_ALPHA_SIZE: GLint = 11;
pub const GLX_DEPTH_SIZE: GLint = 12;
pub const GLX_STENCIL_SIZE: GLint = 13;
pub const GLX_ACCUM_RED_SIZE: GLint = 14;
pub const GLX_ACCUM_GREEN_SIZE: GLint = 15;
pub const GLX_ACCUM_BLUE_SIZE: GLint = 16;
pub const GLX_ACCUM_ALPHA_SIZE: GLint = 17;



pub const GLX_X_VISUAL_TYPE: GLint = 0x22;
pub const GLX_DRAWABLE_TYPE: GLint = 0x8010;
pub const GLX_RENDER_TYPE: GLint = 0x8011;
pub const GLX_X_RENDERABLE: GLint = 0x8012;
pub const GLX_FBCONFIG_ID: GLint = 0x8013;

pub const GLX_RGBA_TYPE: GLint = 0x8014;
pub const GLX_RGBA_BIT: GLint = 0x00000001;
pub const GLX_WINDOW_BIT: GLint = 0x00000001;
pub const GLX_TRUE_COLOR: GLint = 0x8002;
pub const GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB: GLint = 0x20B2;
// 1.4 glx
pub const GLX_SAMPLE_BUFFERS: GLint = 0x186a0;
pub const GLX_SAMPLES: GLint = 0x186a1;


pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLvoid = c_void;
pub type GLbyte = c_schar;
pub type GLshort = c_short;
pub type GLubyte = c_uchar;
pub type GLushort = c_ushort;
pub type GLuint = c_uint;
pub type GLsizei = c_int;
pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;


extern "C" {
    pub fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
}
extern "C" {
    pub fn glBegin(mode: GLenum);
}
extern "C" {
    pub fn glEnd();
}
extern "C" {
    pub fn glVertex2d(x: GLdouble, y: GLdouble);
}
extern "C" {
    pub fn glVertex2f(x: GLfloat, y: GLfloat);
}
extern "C" {
    pub fn glVertex2i(x: GLint, y: GLint);
}
extern "C" {
    pub fn glVertex2s(x: GLshort, y: GLshort);
}
extern "C" {
    pub fn glVertex3d(x: GLdouble, y: GLdouble, z: GLdouble);
}
extern "C" {
    pub fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
}
extern "C" {
    pub fn glColor3f(red: GLfloat, green: GLfloat, blue: GLfloat);
}
extern "C" {
    pub fn glClear(mask: GLbitfield);
}
extern "C" {
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}

pub const GL_TRIANGLES: u32 = 4;
pub const GL_COLOR_BUFFER_BIT: u32 = 16384;

pub const GLX_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
pub const GLX_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
pub const GLX_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
pub const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x1;
pub const GLX_CONTEXT_FLAGS_ARB: c_int = 0x2094;
pub const GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: c_int = 0x2;

unsafe fn create_window() -> bool
{
    // Open the display
    let display = XOpenDisplay(null());
    if display.is_null()
    {
    	println!("Couldnt open display!");
    	return false;
    }
    let screen_id = XDefaultScreen(display);
    let screen = XScreenOfDisplay(display, screen_id);
    if screen.is_null()
    {
        println!("Failed to get screen of display");
        return false;
    }


    let mut major_glx_version: GLint = 0;
    let mut minor_glx_version: GLint = 0;

    let query_glx  = glXQueryVersion(display, &mut major_glx_version, &mut minor_glx_version);
    if query_glx == 0 || major_glx_version < 1 || (major_glx_version == 1 && minor_glx_version < 4)
    {
        print!("Need to have glx 1.4 at least.");
        XCloseDisplay(display);
        return false;
    }
    println!("Query glx result: {}, major: {}, minor: {}", query_glx, major_glx_version, minor_glx_version);

    let attribs: [GLint; 24] = [
        GLX_X_RENDERABLE    , 1,
		GLX_DRAWABLE_TYPE   , GLX_WINDOW_BIT,
		GLX_RENDER_TYPE     , GLX_RGBA_BIT,
		GLX_X_VISUAL_TYPE   , GLX_TRUE_COLOR,
		GLX_RED_SIZE        , 8,
		GLX_GREEN_SIZE      , 8,
		GLX_BLUE_SIZE       , 8,
		GLX_ALPHA_SIZE      , 8,
		GLX_DEPTH_SIZE      , 24,
		GLX_STENCIL_SIZE    , 8,
		GLX_DOUBLEBUFFER    , 1,
        0,0
    ];

    let mut fbcount = 0;
    let fb_configs = glXChooseFBConfig(display, screen_id, attribs.as_ptr(), &mut fbcount);
    if fb_configs.is_null()
    {
        println!("Failed to retrieve framebuffer.");
        XCloseDisplay(display);
        return false;
    }
    println!("Found {} matching framebuffers.", fbcount);

    let mut best_fbc =  -1;
    let mut best_num_samp = -1;

    for i in 0..fbcount
    {
        let fb_conf = *fb_configs.offset(i as isize);
        let visual_info_tmp = glXGetVisualFromFBConfig( display, fb_conf );

        if !visual_info_tmp.is_null()
        {
            let mut samp_buf = 0;
            let mut samples = 0;
            let mut srgb = 0;

            glXGetFBConfigAttrib( display, fb_conf, GLX_SAMPLE_BUFFERS, &mut samp_buf );
            glXGetFBConfigAttrib( display, fb_conf, GLX_SAMPLES, &mut samples  );
            glXGetFBConfigAttrib( display, fb_conf, GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB, &mut srgb);

            if best_fbc < 0 || (samp_buf > 0 && samples > best_num_samp && srgb > 0)
            {
                best_fbc = i;
                best_num_samp = samples;
            }
		}
		XFree( visual_info_tmp as _ );
	}
	println!("Best visual info index: {}", best_fbc);
	let bestFbc = *fb_configs.offset(best_fbc as isize);
	XFree( fb_configs as _ ); // Make sure to free this!

/*
    /* non modern opengl */
    let attribs: [GLint; 18] = [
        GLX_RGBA,
        GLX_DOUBLEBUFFER,
        GLX_DEPTH_SIZE,     24,
        GLX_STENCIL_SIZE,   8,
        GLX_RED_SIZE,       8,
        GLX_GREEN_SIZE,     8,
        GLX_BLUE_SIZE,      8,
        GLX_SAMPLE_BUFFERS, 0,
        GLX_SAMPLES,        0,
        0,0
    ];
    let visual_info = glXChooseVisual(display, screen_id, attribs.as_ptr());
*/
    let visual_info = glXGetVisualFromFBConfig( display, bestFbc );

    if visual_info.is_null()
    {
        println!("Could not create correct visual_info window.");
        XCloseDisplay(display);
        return false;
    }

    if (*visual_info).visual.is_null()
    {
        println!("Could not create correct visual_info, visual window.");
        XCloseDisplay(display);
        return false;

    }

    if screen_id != (*visual_info).screen
    {
        println!("screenId({}) does not match visual->screen({})", screen_id, (*visual_info).screen);
        XCloseDisplay(display);
        return false;
    }

    let root = XRootWindow(display, screen_id);
    let colormap = XCreateColormap(display, root, (*visual_info).visual, AllocNone);
    let black_pixel = XBlackPixel(display, screen_id);
    let white_pixel = XWhitePixel(display, screen_id);

	// Open the window
	let mut windowAttribs:XSetWindowAttributes = std::mem::zeroed();
    windowAttribs.border_pixel = black_pixel;
    windowAttribs.background_pixel = white_pixel;
    windowAttribs.colormap = colormap;
    windowAttribs.event_mask = KeyPressMask | KeyReleaseMask | KeymapStateMask
        | StructureNotifyMask | SubstructureNotifyMask | ExposureMask;

    let window = XCreateWindow(display, root, 0, 0, 640, 480,
     0, (*visual_info).depth, InputOutput, (*visual_info).visual,
     CWBackPixel | CWColormap | CWBorderPixel | CWEventMask, &windowAttribs);

/*
     // Create GLX OpenGL old context
    let gl_context = glXCreateContext(display, visual_info, 0 as _, true);
*/

    let context_attribs: [c_int; 10] = [
        GLX_CONTEXT_MAJOR_VERSION_ARB, 4,
        GLX_CONTEXT_MINOR_VERSION_ARB, 5,
        GLX_CONTEXT_PROFILE_MASK_ARB, GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
        GLX_CONTEXT_FLAGS_ARB, GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
        0, 0
    ];
    //let gl_context = glXCreateNewContext( display, bestFbc, GLX_RGBA_TYPE,
     //    0 as _, true );
    let temp_fn = glXGetProcAddress(b"glXCreateContextAttribsARB\x00".as_ptr());
    if temp_fn.is_null()
    {
        print!("Couldnt fine create context attrib arb");
        XCloseDisplay(display);
        return false;
    }
    let attrbCreate : fn(_: *mut Display, _: GLXFBConfig, _: GLXContext, _: c_int, _: *const c_int) -> GLXContext =
        std::mem::transmute(temp_fn);
    let gl_context = attrbCreate( display, bestFbc,  0 as _, true as _, context_attribs.as_ptr() );




    println!("make current: {}", glXMakeCurrent(display, window, gl_context));

    // Set window title
    XStoreName(display, window, b"Named Window\0".as_ptr() as _);

    // Needed for handling pressing the cross button for exit
    let mut atom_delete_window: Atom  = XInternAtom(display, b"WM_DELETE_WINDOW\0".as_ptr() as _, false as _);
    XSetWMProtocols(display, window, &mut atom_delete_window, 1);



    //Show window
    println!("Clear: {}", XClearWindow(display, window));
    println!("xmap raised: {}", XMapRaised(display, window));

    let mut running = true;
    while running
    {
        let mut ev: XEvent = std::mem::zeroed();
        let _event_out = XNextEvent(display, &mut ev);
        //println!("event: {}", _event_out);
        //println!("type: {}", ev.pad[0]);
        let event_type = (ev.pad[0] & 0xffff) as c_int;
        match event_type
        {
            KeyPress =>
            {
                println!("key press");
                let mut keysym = 0 as KeySym;
                let mut buffer: [c_char; 25] = [0 as c_char; 25];
                let len = XLookupString(&mut ev, buffer.as_mut_ptr(), 25, &mut keysym, null());
                if len > 0
                {
                    println!("button pressed: {}", keysym);
                    XStoreName(display, window, b"Named Window2\0".as_ptr() as _);
                }
                if keysym == XK_Escape
                {
                    println!("esc pressed: {}", keysym);
                    running = false;
                }
            },
            ClientMessage =>
            {
                // 64 bit
                if ev.pad[7] == atom_delete_window as i64
                {
                    println!("Atom delete window!");
                    running = false;
                }
            },
            Expose=>
            {
                // 64 bit
                let width_height = ev.pad[6];
                let width = (width_height & 0xffff_ffff) as i32;
                let height = ((width_height >> 32) & 0xffff_ffff) as i32;

                println!("width: {}, height {}", width, height);
                glViewport(0, 0, width, height);

            },

            DestroyNotify =>
            {
                running = false;

            },
            _ => {}
        };

        // Present frame
        glClearColor(0.3f32, 0.7f32, 0.3f32, 1.0f32);
        		// OpenGL Rendering
		glClear(GL_COLOR_BUFFER_BIT);

		glBegin(GL_TRIANGLES);
			glColor3f(  1.0f32,  0.0f32, 0.0f32);
			glVertex3f( 0.0f32, -1.0f32, 0.0f32);
			glColor3f(  0.0f32,  1.0f32, 0.0f32);
			glVertex3f(-1.0f32,  1.0f32, 0.0f32);
			glColor3f(  0.0f32,  0.0f32, 1.0f32);
			glVertex3f( 1.0f32,  1.0f32, 0.0f32);
		glEnd();

        glXSwapBuffers(display, window);
    }

    glXDestroyContext(display, gl_context);

    XFree(visual_info as _);
    XFreeColormap(display, colormap);
    println!("destroy window: {}", XDestroyWindow(display, window));
    //println!("xfree: {}", XFree(screen as _));
    println!("xclose display: {}", XCloseDisplay(display));

    return true;
}

pub fn linux_main()
{
    println!("linux main window!");

    unsafe
    {
        println!("Create window success: {}", create_window());
    }
}