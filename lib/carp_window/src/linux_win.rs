#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(dead_code)]

use std::{ffi::CString, os::raw::*, ptr::null};


#[repr(C)]
pub struct App
{
    display: *mut Display,
    window: Window,
    colormap: Colormap,
    gl_context: GLXContext,
    atom_delete_window: Atom,

    pub width: i32,
    pub height: i32,
    pub running: bool,
    pub resized: bool,
}


impl Drop for App
{
    fn drop(&mut self)
    {
        if self.display.is_null()
        {
            return;
        }
        unsafe
        {
            if self.colormap != 0 as Colormap
            {
                XFreeColormap(self.display, self.colormap);
                self.colormap = 0 as Colormap;
            }
            if self.window != 0 as Window
            {
                println!("destroy window: {}", XDestroyWindow(self.display, self.window));
                self.window = 0 as Window;
            }
            if self.gl_context != 0 as GLXContext
            {
                println!("destroying context");
                glXDestroyContext(self.display, self.gl_context);
                self.gl_context = 0 as GLXContext;
            }
            println!("xclose display: {}", XCloseDisplay(self.display));
        }
    }
}

impl App
{
    pub fn new() -> App
    {

        return  App {
            display: 0 as *mut Display,
            window: 0 as Window,
            colormap: 0 as Colormap,
            gl_context: 0 as GLXContext,
            atom_delete_window: 0 as Atom,

            width: 0,
            height: 0,
            running: true,
            resized: false,
        }
    }

    pub unsafe fn load_fn(&self, proc: &'static str) -> *const c_void
    {
        let proc_cstr = CString::new(proc).unwrap();
        let proc_ptr = proc_cstr.as_ptr();

        let proc = glXGetProcAddress(proc_ptr as _);
        return proc;
    }


    pub unsafe fn swap_buffers(&mut self)
    {
        glXSwapBuffers(self.display, self.window);
    }
    pub unsafe fn set_window_title(&mut self, title: *const c_char)
    {
        // Set window title
        XStoreName(self.display, self.window, title);

    }

    pub unsafe fn update(&mut self)
    {
        while XPending(self.display) > 0 && self.running
        {
            let mut ev: XEvent = std::mem::zeroed();
            let _event_out = XNextEvent(self.display, &mut ev);
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
                        XStoreName(self.display, self.window, b"Named Window2\0".as_ptr() as _);
                    }
                    if keysym == XK_Escape
                    {
                        println!("esc pressed: {}", keysym);
                        self.running = false;
                    }
                },
                ClientMessage =>
                {
                    // 64 bit
                    if ev.pad[7] == self.atom_delete_window as i64
                    {
                        println!("Atom delete window!");
                        self.running = false;
                    }
                },
                Expose=>
                {
                    // 64 bit
                    let width_height = ev.pad[6];
                    let width = (width_height & 0xffff_ffff) as i32;
                    let height = ((width_height >> 32) & 0xffff_ffff) as i32;

                    println!("width: {}, height {}", width, height);
                    self.width = width;
                    self.height = height;
                    self.resized = true;
                },

                DestroyNotify =>
                {
                    self.running = false;
                },
                _ => {}
            };


        }
    }


    pub unsafe fn create_window(&mut self, width: i32, height: i32, title: *const c_char) -> bool
    {
        self.width = width;
        self.height = height;

        // Open the display
        self.display = XOpenDisplay(null());

        if self.display.is_null()
        {
            println!("Couldnt open display!");
            return false;
        }
        let screen_id = XDefaultScreen(self.display);
        let screen = XScreenOfDisplay(self.display, screen_id);
        if screen.is_null()
        {
            println!("Failed to get screen of display");
            return false;
        }


        let mut major_glx_version: GLint = 0;
        let mut minor_glx_version: GLint = 0;

        let query_glx  = glXQueryVersion(self.display, &mut major_glx_version, &mut minor_glx_version);
        if query_glx == 0 || major_glx_version < 1 || (major_glx_version == 1 && minor_glx_version < 4)
        {
            print!("Need to have glx 1.4 at least.");
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
        let fb_configs = glXChooseFBConfig(self.display, screen_id, attribs.as_ptr(), &mut fbcount);
        if fb_configs.is_null()
        {
            println!("Failed to retrieve framebuffer.");
            return false;
        }
        println!("Found {} matching framebuffers.", fbcount);

        let mut best_fbc =  -1;
        let mut best_num_samp = -1;

        for i in 0..fbcount
        {
            let fb_conf = *fb_configs.offset(i as isize);
            let visual_info_tmp = glXGetVisualFromFBConfig( self.display, fb_conf );

            if !visual_info_tmp.is_null()
            {
                let mut samp_buf = 0;
                let mut samples = 0;
                let mut srgb = 0;

                glXGetFBConfigAttrib( self.display, fb_conf, GLX_SAMPLE_BUFFERS, &mut samp_buf );
                glXGetFBConfigAttrib( self.display, fb_conf, GLX_SAMPLES, &mut samples  );
                glXGetFBConfigAttrib( self.display, fb_conf, GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB, &mut srgb);

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
        let visual_info = glXGetVisualFromFBConfig( self.display, bestFbc );

        if visual_info.is_null()
        {
            println!("Could not create correct visual_info window.");
            return false;
        }

        if (*visual_info).visual.is_null()
        {
            println!("Could not create correct visual_info, visual window.");
            XFree(visual_info as _);
            return false;
        }

        if screen_id != (*visual_info).screen
        {
            println!("screenId({}) does not match visual->screen({})", screen_id, (*visual_info).screen);
            XFree(visual_info as _);
            return false;
        }

        let root = XRootWindow(self.display, screen_id);
        self.colormap = XCreateColormap(self.display, root, (*visual_info).visual, AllocNone);
        let black_pixel = XBlackPixel(self.display, screen_id);
        let white_pixel = XWhitePixel(self.display, screen_id);

        // Open the window
        let mut windowAttribs:XSetWindowAttributes = std::mem::zeroed();
        windowAttribs.border_pixel = black_pixel;
        windowAttribs.background_pixel = white_pixel;
        windowAttribs.colormap = self.colormap;
        windowAttribs.event_mask = KeyPressMask | KeyReleaseMask | KeymapStateMask
            | StructureNotifyMask | SubstructureNotifyMask | ExposureMask;

        self.window = XCreateWindow(self.display, root, 0, 0, width as u32, height as u32,
         0, (*visual_info).depth, InputOutput, (*visual_info).visual,
         CWBackPixel | CWColormap | CWBorderPixel | CWEventMask, &windowAttribs);

    /*
         // Create GLX OpenGL old context
        let gl_context = glXCreateContext(self.display, visual_info, 0 as _, true);
    */
        // does this need to be saved to release?
        XFree(visual_info as _);


        let context_attribs: [c_int; 10] = [
            GLX_CONTEXT_MAJOR_VERSION_ARB, 4,
            GLX_CONTEXT_MINOR_VERSION_ARB, 5,
            GLX_CONTEXT_PROFILE_MASK_ARB, GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
            GLX_CONTEXT_FLAGS_ARB, GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
            0, 0
        ];
        //let gl_context = glXCreateNewContext( self.display, bestFbc, GLX_RGBA_TYPE,
         //    0 as _, true );
        let temp_fn = glXGetProcAddress(b"glXCreateContextAttribsARB\x00".as_ptr());
        if temp_fn.is_null()
        {
            print!("Couldnt fine create context attrib arb");
            return false;
        }
        let attrbCreate : fn(_: *mut Display, _: GLXFBConfig, _: GLXContext, _: c_int, _: *const c_int) -> GLXContext =
            std::mem::transmute(temp_fn);
        self.gl_context = attrbCreate( self.display, bestFbc,  0 as _, true as _, context_attribs.as_ptr() );


        // sync
        XSync(self.display, false);

        println!("make current: {}", glXMakeCurrent(self.display, self.window, self.gl_context));

        self.set_window_title(title);

        // Needed for handling pressing the cross button for exit
        self.atom_delete_window  = XInternAtom(self.display, b"WM_DELETE_WINDOW\0".as_ptr() as _, false as _);
        XSetWMProtocols(self.display, self.window, &mut self.atom_delete_window, 1);


        //Show window
        println!("Clear: {}", XClearWindow(self.display, self.window));
        println!("xmap raised: {}", XMapRaised(self.display, self.window));

        return true;
    }
}






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
    pub fn dlsym(lib_module_handle: *const EMPTYTYPE, proc_name: *const i8) -> *const EMPTYTYPE;

    fn XInitThreads() -> c_int;
    fn XrmInitialize();

    fn XOpenDisplay(_: *const c_char) -> *mut Display;
    fn XResourceManagerString(_ :Display) -> *mut c_char;
    fn XInternAtom(_: *mut Display, _: *const c_char, _: c_int) -> Atom;

    fn XScreenOfDisplay(display: *mut Display, display_index: c_int) -> *mut Screen;
    fn XCreateSimpleWindow(display: *mut Display, window: Window, x: c_int, y: c_int,
        width: c_uint, height: c_uint, border_width: c_uint, bord: c_ulong, bg: c_ulong) -> Window;
    fn XCreateWindow(display: *mut Display, window: Window, x: c_int, y: c_int,
        width: c_uint, height: c_uint, border_width: c_uint, depth: c_int,
        cls: c_uint, visual: Visual, value_mask: c_ulong, win_attr: *const XSetWindowAttributes) -> Window;

    fn XPending(display: *mut Display) -> c_int;

    fn XRootWindow(display: *mut Display, screen_id: c_int) -> Window;
    fn XBlackPixel(display: *mut Display, screen_id: c_int) -> c_ulong;
    fn XWhitePixel(display: *mut Display, screen_id: c_int) -> c_ulong;

    fn XClearWindow(display: *mut Display, window: Window) -> c_int;
    fn XMapRaised(display: *mut Display, window: Window) -> c_int;
    fn XDestroyWindow(display: *mut Display, window: Window) -> c_int;

    fn XFree(data: *mut c_void) -> c_int;
    fn XCloseDisplay(display: *mut Display) -> c_int;
    fn XNextEvent(display: *mut Display, xevent: *mut XEvent) -> c_int;

    fn XDefaultScreen(display: *mut Display) -> c_int;
    fn XStoreName(display: *mut Display, window: Window, name: *const c_char) -> c_int;

    fn XSelectInput(display: *mut Display, window: Window, event_mask: c_long) -> c_int;


    fn XFreeColormap(display: *mut Display, colormap: Colormap) -> c_int;

    fn XLookupString(key_event: *mut XEvent, buffer: *mut c_char, bytes: c_int,
        keysym: *mut KeySym, comp: *const c_void /* *mut XComposeStatus*/) -> c_int;


    fn XSetWMProtocols(display: *mut Display, window: Window, protocols: *mut Atom, count: c_int) -> c_int;




    fn XDisplayHeight(display: *mut Display, screen_id: c_int) -> c_int;
    fn XDisplayHeightMM(display: *mut Display, screen_id: c_int) -> c_int;
    fn XDisplayPlanes(display: *mut Display, screen_id: c_int) -> c_int;
    fn XDisplayWidth(display: *mut Display, screen_id: c_int) -> c_int;
    fn XDisplayWidthMM(display: *mut Display, screen_id: c_int) -> c_int;


    fn XSync(display: *mut Display, sync: bool) -> c_int;

    fn XSetWindowColormap(display: *mut Display, window: Window, colormap: Colormap) -> c_int;
    fn XCreateColormap(display: *mut Display, window: Window, visual: Visual, alloc: c_int) -> Colormap;

    // GL lib
    fn glXQueryVersion(display: *mut Display, major_version: &mut GLint, minor_version: &mut GLint) -> GLint;
    fn glXChooseVisual(display: *mut Display, screen_id: c_int, attrib_list: *const GLint) -> *mut XVisualInfo;
    fn glXCreateContext(display: *mut Display, visual_info: *const XVisualInfo, share_list: GLXContext, direct: bool) -> GLXContext;
    fn glXDestroyContext(display: *mut Display, context: GLXContext);

    fn glXMakeCurrent(display: *mut Display, drawable: GLXDrawable, context: GLXContext) -> bool;
    fn glXSwapBuffers(display: *mut Display, drawable: GLXDrawable);
    fn glXChooseFBConfig(display: *mut Display, screen_id: c_int, attrib_list: *const GLint, items: &mut c_int) -> *const GLXFBConfig;
    fn glXGetVisualFromFBConfig(display: *mut Display, glx_fb_config: GLXFBConfig) -> *const XVisualInfo;

    fn glXGetFBConfigAttrib(display: *mut Display, glx_fb_config: GLXFBConfig, attr: c_int, value: &mut c_int) -> c_int;

    fn glXCreateNewContext(display: *mut Display, glx_fb_config: GLXFBConfig,
        render_type: c_int, share_list: GLXContext, direct: bool) -> GLXContext;

    fn glXGetProcAddress(procname: *const c_uchar) -> *const c_void;

}

#[repr(C)]
struct XVisualInfo
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


#[repr(C)]
struct XEvent
{
    pub pad: [c_long; 24],
}



#[repr(C)]
struct XSetWindowAttributes
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



const CWBackPixmap: c_ulong = 1 << 0;
const CWBackPixel: c_ulong = 1 << 1;
const CWBorderPixmap: c_ulong = 1 << 2;
const CWBorderPixel: c_ulong = 1 << 3;
const CWBitGravity: c_ulong = 1 << 4;
const CWWinGravity: c_ulong = 1<< 5;
const CWBackingStore: c_ulong = 1<< 6;
const CWBackingPlanes: c_ulong = 1 << 7;
const CWBackingPixel: c_ulong = 1 << 8;
const CWOverrideRedirect: c_ulong = 1 <<9;
const CWSaveUnder: c_ulong = 1 << 10;
const CWEventMask: c_ulong = 1 << 11;
const CWDontPropagate: c_ulong = 1 <<12;
const CWColormap: c_ulong = 1 << 13;
const CWCursor: c_ulong = 1 << 14;


const CWX: c_ulong = 1 << 0;
const CWY: c_ulong = 1 << 1;
const CWWidth: c_ulong = 1 << 2;
const CWHeight: c_ulong = 1 << 3;
const CWBorderWidth: c_ulong = 1 << 4;
const CWSibling: c_ulong = 1 << 5;
const CWStackMode: c_ulong = 1 << 6;

const InputOutput: c_uint = 1;



const NoEventMask: c_long = 0x0;
const KeyPressMask: c_long = 0x1;
const KeyReleaseMask: c_long = 0x2;
const ButtonPressMask: c_long = 0x4;
const ButtonReleaseMask: c_long = 0x8;
const EnterWindowMask: c_long = 0x10;
const LeaveWindowMask: c_long = 0x20;
const PointerMotionMask: c_long = 0x40;
const PointerMotionHintMask: c_long = 0x80;
const Button1MotionMask: c_long = 0x100;
const Button2MotionMask: c_long = 0x200;
const Button3MotionMask: c_long = 0x400;
const Button4MotionMask: c_long = 0x800;
const Button5MotionMask: c_long = 0x1000;
const ButtonMotionMask: c_long =  0x2000;
const KeymapStateMask: c_long = 0x4000;
const ExposureMask: c_long = 0x8000;
const VisibilityChangeMask: c_long = 0x1_0000;
const StructureNotifyMask: c_long = 0x2_0000;
const ResizeRedirectMask: c_long = 0x4_0000;
const SubstructureNotifyMask: c_long = 0x8_0000;
const SubstructureRedirectMask: c_long = 0x10_0000;
const FocusChangeMask: c_long = 0x20_0000;
const PropertyChangeMask: c_long = 0x40_0000;
const ColormapChangeMask: c_long = 0x80_0000;
const OwnerGrabButtonMask: c_long = 0x100_0000;

// Events
const KeyPress: c_int = 2;
const KeyRelease: c_int = 3;
const ButtonPress: c_int = 4;
const ButtonRelease: c_int = 5;
const MotionNotify: c_int = 6;
const EnterNotify: c_int = 7;
const LeaveNotify: c_int = 8;
const FocusIn: c_int = 9;
const FocusOut: c_int = 10;
const KeymapNotify: c_int = 11;
const Expose: c_int = 12;
const GraphicsExpose: c_int = 13;
const NoExpose: c_int = 14;
const VisibilityNotify: c_int = 15;
const CreateNotify: c_int = 16;
const DestroyNotify: c_int = 17;
const UnmapNotify: c_int = 18;
const MapNotify: c_int = 19;
const MapRequest: c_int = 20;
const ReparentNotify: c_int = 21;
const ConfigureNotify: c_int = 22;
const ConfigureRequest: c_int = 23;
const GravityNotify: c_int = 24;
const ResizeRequest: c_int = 25;
const CirculateNotify: c_int = 26;
const CirculateRequest: c_int = 27;
const PropertyNotify: c_int = 28;
const SelectionClear: c_int = 29;
const SelectionRequest: c_int = 30;
const SelectionNotify: c_int = 31;
const ColormapNotify: c_int = 32;
const ClientMessage: c_int = 33;
const MappingNotify: c_int = 34;
const GenericEvent: c_int = 35;
const LASTEvent: c_int = 36;    /* must be bigger than any event # */



const XK_Escape: KeySym = 0xff1b;


const AllocNone: c_int = 0;


const GLX_RGBA: GLint = 4;
const GLX_DOUBLEBUFFER: GLint = 5;



const GLX_AUX_BUFFERS: GLint = 7;
const GLX_RED_SIZE: GLint = 8;
const GLX_GREEN_SIZE: GLint = 9;
const GLX_BLUE_SIZE: GLint = 10;
const GLX_ALPHA_SIZE: GLint = 11;
const GLX_DEPTH_SIZE: GLint = 12;
const GLX_STENCIL_SIZE: GLint = 13;
const GLX_ACCUM_RED_SIZE: GLint = 14;
const GLX_ACCUM_GREEN_SIZE: GLint = 15;
const GLX_ACCUM_BLUE_SIZE: GLint = 16;
const GLX_ACCUM_ALPHA_SIZE: GLint = 17;



const GLX_X_VISUAL_TYPE: GLint = 0x22;
const GLX_DRAWABLE_TYPE: GLint = 0x8010;
const GLX_RENDER_TYPE: GLint = 0x8011;
const GLX_X_RENDERABLE: GLint = 0x8012;
const GLX_FBCONFIG_ID: GLint = 0x8013;

const GLX_RGBA_TYPE: GLint = 0x8014;
const GLX_RGBA_BIT: GLint = 0x00000001;
const GLX_WINDOW_BIT: GLint = 0x00000001;
const GLX_TRUE_COLOR: GLint = 0x8002;
const GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB: GLint = 0x20B2;
// 1.4 glx
const GLX_SAMPLE_BUFFERS: GLint = 0x186a0;
const GLX_SAMPLES: GLint = 0x186a1;




const GLX_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
const GLX_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
const GLX_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x1;
const GLX_CONTEXT_FLAGS_ARB: c_int = 0x2094;
const GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: c_int = 0x2;


