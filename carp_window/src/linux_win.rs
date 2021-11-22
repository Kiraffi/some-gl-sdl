
use std::{os::raw::*, ptr::null};

pub type Atom = c_ulong;
pub type XID = c_ulong;
pub type Window = XID;
pub type Colormap = XID;
pub type KeySym = XID;
pub enum EMPTYTYPE {}
pub type XPrivate = *mut EMPTYTYPE;
pub type XDisplay = *mut EMPTYTYPE;
pub type XPointer = *mut c_char;
pub type _XrmHashBucketRec = *mut EMPTYTYPE;
pub type XExtData = *mut EMPTYTYPE;
pub type ScreenFormat = *mut EMPTYTYPE;
pub type Visual = *mut EMPTYTYPE;
pub type Depth = *mut EMPTYTYPE;

pub type GC = *mut EMPTYTYPE;
pub type XComposeStatus = *mut EMPTYTYPE;



#[link(name = "X11")]
#[link(name = "GL")]
extern "system" {
    pub fn XInitThreads() -> c_int;
    pub fn XrmInitialize();
    pub fn XOpenDisplay(_: *const c_char) -> *mut Display;
    pub fn XResourceManagerString(_ :Display) -> *mut c_char;
    pub fn XInternAtom(_: *mut Display, _: *const c_char, _: c_int) -> Atom;


    pub fn XScreenOfDisplay(display: *mut Display, display_index: c_int) -> *mut Screen;
    pub fn XCreateSimpleWindow(display: *mut Display, window: Window, x: c_int, y: c_int, 
        width: c_uint, height: c_uint, border_width: c_uint, bord: c_ulong, bg: c_ulong) -> Window;
    
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





    pub fn XLookupString(key_event: *mut XEvent, buffer: *mut c_char, bytes: c_int, 
        keysym: *mut KeySym, comp: *const c_void /* *mut XComposeStatus*/) -> c_int;


    pub fn XSetWMProtocols(display: *mut Display, window: Window, protocols: *mut Atom, count: c_int) -> c_int;
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct Display {
    pub ext_data: *mut XExtData,
    pub private1: XPrivate,
    pub fd: c_int,
    pub private2: c_int,
    pub proto_major_version: c_int,
    pub proto_minor_version: c_int,
    pub vendor: *mut c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(_: *mut XDisplay) -> XID>,
    pub byte_order: c_int,
    pub bitmap_unit: c_int,
    pub bitmap_pad: c_int,
    pub bitmap_bit_order: c_int,
    pub nformats: c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: c_int,
    pub release: c_int,
    pub private9: XPrivate,
    pub private10: XPrivate,
    pub qlen: c_int,
    pub last_request_read: c_ulong,
    pub request: c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(_: *mut XDisplay) -> c_int>,
    pub display_name: *mut c_char,
    pub default_screen: c_int,
    pub nscreens: c_int,
    pub screens: *mut Screen,
    pub motion_buffer: c_ulong,
    pub private16: c_ulong,
    pub min_keycode: c_int,
    pub max_keycode: c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: c_int,
    pub xdefaults: *mut c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XEvent
{
    pub pad: [c_long; 24],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut XDisplay,
    pub root: Window,
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
    pub ndepths: c_int,
    pub depths: *mut Depth,
    pub root_depth: c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: c_ulong,
    pub black_pixel: c_ulong,
    pub max_maps: c_int,
    pub min_maps: c_int,
    pub backing_store: c_int,
    pub save_unders: c_int,
    pub root_input_mask: c_long,
}

pub const SubstructureNotifyMask: c_long = 1 << 19; 
pub const StructureNotifyMask: c_long = 1 << 17;
pub const NoEventMask: c_long = 0x0;
pub const KeyPressMask: c_long = 0x1;
pub const KeyReleaseMask: c_long = 0x2;
pub const PointerMotionMask: c_long = 0x40;
pub const PointerMotionHintMask: c_long = 0x80;
pub const Button1MotionMask: c_long = 0x100;
pub const Button2MotionMask: c_long = 0x200;
pub const Button3MotionMask: c_long = 0x400;
pub const Button4MotionMask: c_long = 0x800;
pub const Button5MotionMask: c_long = 0x1000;
pub const ButtonMotionMask: c_long =  0x2000;
pub const KeymapStateMask: c_long = 0x4000;


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
/*
    extern int XDefaultScreen(
        Display*		/* display */
    );

extern int XNextEvent(
    Display*		/* display */,
    XEvent*		/* event_return */
);
    extern int XCloseDisplay(
        Display*		/* display */
    );

extern Window XCreateSimpleWindow(
    Display*		/* display */,
    Window		/* parent */,
    int			/* x */,
    int			/* y */,
    unsigned int	/* width */,
    unsigned int	/* height */,
    unsigned int	/* border_width */,
    unsigned long	/* border */,
    unsigned long	/* background */
);
    extern int XClearWindow(
        Display*		/* display */,
        Window		/* w */
    );
extern int XFree(
    void*		/* data */
);

extern int XMapRaised(
    Display*		/* display */,
    Window		/* w */
);


extern int XDestroyWindow(
    Display*		/* display */,
    Window		/* w */
);


extern Window XRootWindow(
            Display*		/* display */,
            int			/* screen_number */
        );



            extern unsigned long XBlackPixel(
        Display*		/* display */,
        int			/* screen_number */
    );
    extern unsigned long XWhitePixel(
        Display*		/* display */,
        int			/* screen_number */
    );










extern int XLookupString(
    XKeyEvent*		/* event_struct */,
    char*		/* buffer_return */,
    int			/* bytes_buffer */,
    KeySym*		/* keysym_return */,
    XComposeStatus*	/* status_in_out */
);

*/
unsafe fn create_window() -> bool
{
    // Open the display
	let display = XOpenDisplay(null());
	if display.is_null() 
    {
		println!("Couldnt open display!");
		return false;
	}
	let screen_id = XDefaultScreen(display); //(*display).default_screen; // DefaultScreenOfDisplay(display);
    println!("default {}", screen_id);
	let screen = XScreenOfDisplay(display, screen_id); // _) DefaultScreen(display);
    if screen.is_null()
    {
        println!("Failed to get screen of display");
        return false;
    }

    let root = XRootWindow(display, screen_id);
    if root == 0
    {
        println!("Root is zero");
        return false;
    }

    println!("root {}", root);
    let black_pixel = XBlackPixel(display, screen_id);
    let white_pixel = XWhitePixel(display, screen_id);

    let window = XCreateSimpleWindow(display, root, 0, 0, 640, 480, 2, black_pixel, white_pixel);
    println!("Window: {}", window);

    //XSelectInput(display, window, KeyPressMask | KeyReleaseMask | KeymapStateMask | StructureNotifyMask | SubstructureNotifyMask);
    XSelectInput(display, window, KeyPressMask | StructureNotifyMask | SubstructureNotifyMask);

    XStoreName(display, window, b"Named Window\0".as_ptr() as _);
    
    
    let mut atom_delete_window: Atom  = XInternAtom(display, b"WM_DELETE_WINDOW\0".as_ptr() as _, false as _);
    XSetWMProtocols(display, window, &mut atom_delete_window, 1);
    
    //Show window
    println!("Clear: {}", XClearWindow(display, window));
    println!("xmap raised: {}", XMapRaised(display, window));

    let mut running = true;
	// Enter message loop
	while running
    {
        let mut ev: XEvent = std::mem::zeroed();
		println!("event: {}", XNextEvent(display, &mut ev));

        println!("type: {}", ev.pad[0]);
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
                //if (ev.xclient.data.l[0] == atomWmDeleteWindow) 
                if ev.pad[7] == atom_delete_window as i64
                {
                    println!("Atom delete window!");
                    running = false;
                }
            },
            DestroyNotify =>
            {
                running = false;
                
            },
            _ => {}
        };
	}

    println!("destroy window: {}", XDestroyWindow(display, window));
    //println!("xfree: {}", XFree(screen as _));
    println!("xclose display: {}", XCloseDisplay(display));


/*
	// Open the window
	window = XCreateSimpleWindow(display, RootWindowOfScreen(screen), 0, 0, 320, 200, 1, BlackPixel(display, screenId), WhitePixel(display, screenId));

    // Show the window
	XClearWindow(display, window);
	XMapRaised(display, window);
	
	// Enter message loop
	while (true) {
		XNextEvent(display, &ev);
		// Do something
	}

	// Cleanup
	XDestroyWindow(display, window);
	XFree(screen);
	XCloseDisplay(display);
    */
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